
#![forbid(unsafe_code)]

//! Structured outcomes for the Zamani frontend parser.
//!
//! File: `src/frontend/parser/result.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Represent the outcome of parsing without conflating successful parsing,
//! successful recovery, incomplete input, and unrecoverable failure.
//!
//! # Ownership
//!
//! This module owns:
//! - the public, domain-neutral parse outcome status;
//! - the generic `ParseResult<T>` container;
//! - optional partial syntax and token-consumption information;
//! - retained parser errors and structured parser diagnostics;
//! - result transformations that preserve outcome metadata.
//!
//! This module does not own:
//! - grammar productions or parser-session state;
//! - lexical scanning or token construction;
//! - diagnostic formatting, collection policy, or recovery algorithms;
//! - AST definitions or AST validation;
//! - name resolution, type checking, effects, capabilities, or contracts;
//! - resource negotiation, IR lowering, scheduling, or target realization.
//!
//! # Integration contract
//!
//! - `error.rs` is authoritative for `ParseError` and `ParseErrorClass`.
//! - `diagnostics.rs` is authoritative for `ParserDiagnostic`.
//! - `limits.rs` is authoritative for parser resource-budget failures.
//! - `session.rs` should construct the result from its final parse state.
//! - `parser.rs` should report whether parsing completed, recovered, needs
//!   more input, or failed.
//! - `api.rs` should expose this result without replacing its status with a
//!   boolean or discarding diagnostics.
//! - Whole-program parsing should report trailing input as an error when
//!   `ParserConfig::require_eof` is enabled.
//! - Fragment parsers may report remaining tokens according to their own
//!   documented consumption contract.
//!
//! # Status invariants
//!
//! `Complete` means the selected grammar entry point completed without
//! parser recovery. Warnings may still be present.
//!
//! `Recovered` means parsing produced a usable result after one or more
//! recovery actions. Diagnostics describing those actions must be retained
//! whenever the configured diagnostic policy permits.
//!
//! `Incomplete` means more input may allow the selected grammar entry point
//! to complete. This is useful for editors, REPLs, and incremental tools.
//!
//! `Failed` means the entry point did not complete successfully. A partial
//! value may be retained for tooling, but must not be treated as a valid,
//! complete program.
//!
//! This type records the parser's outcome; it does not independently prove
//! that a caller has enforced these invariants. The parser/session layer is
//! responsible for assigning the correct status.
//!
//! # Scalability and resource safety
//!
//! This module imposes no language-level limits on declarations, operations,
//! qubits, devices, register sizes, tensor rank, processors, nodes, or other
//! computational dimensions. Collections grow according to available memory
//! and the policies of the parser that creates the result.
//!
//! `try_reserve`-based append methods allow callers to handle collection
//! capacity errors where the allocator reports them. They cannot guarantee
//! recovery from process-wide memory exhaustion, and they do not replace
//! parser budgets for untrusted input.
//!
//! No global mutable state is used. The same result contract serves classical,
//! quantum, HDL, hybrid, distributed, and future grammar extensions.

use core::fmt;
use std::collections::TryReserveError;
use std::error::Error;

use super::diagnostics::{ParserDiagnostic, ParserDiagnosticSeverity};
use super::error::{ParseError, ParseErrorClass};

/// Overall outcome of one parser entry point.
///
/// This is a syntax-processing status, not a semantic-validity or
/// target-compatibility verdict.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ParseStatus {
    /// The selected grammar entry point completed without recovery.
    Complete,

    /// A usable result was produced after syntax recovery.
    Recovered,

    /// More input may allow the selected grammar entry point to complete.
    Incomplete,

    /// The selected grammar entry point could not complete successfully.
    Failed,
}

impl ParseStatus {
    /// Whether the selected grammar entry point completed without recovery.
    #[must_use]
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
    }

    /// Whether the parser completed by recovering from syntax errors.
    #[must_use]
    pub const fn is_recovered(self) -> bool {
        matches!(self, Self::Recovered)
    }

    /// Whether the parser expects or may benefit from additional input.
    #[must_use]
    pub const fn is_incomplete(self) -> bool {
        matches!(self, Self::Incomplete)
    }

    /// Whether the selected grammar entry point failed to complete.
    #[must_use]
    pub const fn is_failed(self) -> bool {
        matches!(self, Self::Failed)
    }

    /// Whether the outcome contains a complete parse, with or without
    /// recovery.
    ///
    /// A recovered result is not equivalent to an error-free parse.
    #[must_use]
    pub const fn has_usable_result(self) -> bool {
        matches!(self, Self::Complete | Self::Recovered)
    }
}

impl fmt::Display for ParseStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Complete => "complete",
            Self::Recovered => "recovered",
            Self::Incomplete => "incomplete",
            Self::Failed => "failed",
        })
    }
}

/// Token-consumption information reported by a parser entry point.
///
/// `remaining_tokens` is optional because some entry points do not have a
/// meaningful finite token-stream boundary, or do not calculate the count.
/// Counts follow the entry point's documented convention, including whether
/// an EOF token is included.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TokenConsumption {
    consumed_tokens: usize,
    remaining_tokens: Option<usize>,
}

impl TokenConsumption {
    /// Create token-consumption information.
    #[must_use]
    pub const fn new(
        consumed_tokens: usize,
        remaining_tokens: Option<usize>,
    ) -> Self {
        Self {
            consumed_tokens,
            remaining_tokens,
        }
    }

    /// Number of tokens consumed by the entry point.
    #[must_use]
    pub const fn consumed_tokens(self) -> usize {
        self.consumed_tokens
    }

    /// Number of tokens remaining, if known.
    #[must_use]
    pub const fn remaining_tokens(self) -> Option<usize> {
        self.remaining_tokens
    }

    /// Whether the caller reports no remaining tokens.
    ///
    /// Returns `false` when the remaining-token count is unknown.
    #[must_use]
    pub const fn is_fully_consumed(self) -> bool {
        matches!(self.remaining_tokens, Some(0))
    }
}

/// Result of a parser entry point.
///
/// The value is optional so callers can distinguish failure without a
/// partially constructed syntax value from failure where partial syntax is
/// retained for editor tooling. A present value alone does not imply that
/// parsing completed successfully; inspect [`ParseResult::status`] first.
///
/// Parser errors preserve structured failure information. Diagnostics
/// provide the presentation/tooling view. When an error is added through
/// [`ParseResult::try_push_error`] or [`ParseResult::push_error`], a matching
/// diagnostic is also retained unless allocation fails through the fallible
/// method.
///
/// This type intentionally does not impose an AST type. It can carry the
/// repository's canonical AST or a grammar-specific intermediate value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseResult<T> {
    value: Option<T>,
    status: ParseStatus,
    errors: Vec<ParseError>,
    diagnostics: Vec<ParserDiagnostic>,
    consumption: Option<TokenConsumption>,
    diagnostics_truncated: bool,
}

impl<T> ParseResult<T> {
    /// Create a result with an optional value and an explicit parse status.
    ///
    /// The caller is responsible for selecting a status consistent with the
    /// parser's actual completion and recovery behavior.
    #[must_use]
    pub fn new(value: Option<T>, status: ParseStatus) -> Self {
        Self {
            value,
            status,
            errors: Vec::new(),
            diagnostics: Vec::new(),
            consumption: None,
            diagnostics_truncated: false,
        }
    }

    /// Create a result for a parse that completed without recovery.
    #[must_use]
    pub fn complete(value: T) -> Self {
        Self::new(Some(value), ParseStatus::Complete)
    }

    /// Create a result for a parse that completed after recovery.
    #[must_use]
    pub fn recovered(value: T) -> Self {
        Self::new(Some(value), ParseStatus::Recovered)
    }

    /// Create an incomplete result without a partial value.
    #[must_use]
    pub fn incomplete() -> Self {
        Self::new(None, ParseStatus::Incomplete)
    }

    /// Create an incomplete result retaining partial syntax.
    #[must_use]
    pub fn incomplete_with_partial(value: T) -> Self {
        Self::new(Some(value), ParseStatus::Incomplete)
    }

    /// Create a failed result without a partial value.
    #[must_use]
    pub fn failed() -> Self {
        Self::new(None, ParseStatus::Failed)
    }

    /// Create a failed result while retaining partial syntax for tooling.
    ///
    /// The partial value is not a successful complete parse.
    #[must_use]
    pub fn failed_with_partial(value: T) -> Self {
        Self::new(Some(value), ParseStatus::Failed)
    }

    /// Return the parse status.
    #[must_use]
    pub const fn status(&self) -> ParseStatus {
        self.status
    }

    /// Return the parsed or partial value, if present.
    #[must_use]
    pub const fn value(&self) -> Option<&T> {
        self.value.as_ref()
    }

    /// Return the parsed or partial value mutably, if present.
    ///
    /// Mutating the value does not change the recorded parse status.
    #[must_use]
    pub fn value_mut(&mut self) -> Option<&mut T> {
        self.value.as_mut()
    }

    /// Consume the result and return its value, if present.
    ///
    /// The caller must inspect the status first if a complete parse is
    /// required.
    #[must_use]
    pub fn into_value(self) -> Option<T> {
        self.value
    }

    /// Consume the result and return the value only when parsing completed
    /// without recovery.
    #[must_use]
    pub fn into_complete_value(self) -> Option<T> {
        if self.status.is_complete() {
            self.value
        } else {
            None
        }
    }

    /// Consume the result and return the value when parsing completed either
    /// cleanly or with recovery.
    ///
    /// This deliberately excludes incomplete and failed results.
    #[must_use]
    pub fn into_usable_value(self) -> Option<T> {
        if self.status.has_usable_result() {
            self.value
        } else {
            None
        }
    }

    /// Whether a value, including partial syntax, is retained.
    #[must_use]
    pub const fn has_value(&self) -> bool {
        self.value.is_some()
    }

    /// Whether the entry point completed without recovery.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.status.is_complete()
    }

    /// Whether the entry point completed after recovery.
    #[must_use]
    pub const fn is_recovered(&self) -> bool {
        self.status.is_recovered()
    }

    /// Whether more input may be needed.
    #[must_use]
    pub const fn is_incomplete(&self) -> bool {
        self.status.is_incomplete()
    }

    /// Whether the entry point failed to complete.
    #[must_use]
    pub const fn is_failed(&self) -> bool {
        self.status.is_failed()
    }

    /// Return whether the status represents a complete or recovered parse.
    #[must_use]
    pub const fn has_usable_result(&self) -> bool {
        self.status.has_usable_result()
    }

    /// Return retained parser errors in insertion order.
    #[must_use]
    pub fn errors(&self) -> &[ParseError] {
        &self.errors
    }

    /// Return retained structured diagnostics in reporting order.
    #[must_use]
    pub fn diagnostics(&self) -> &[ParserDiagnostic] {
        &self.diagnostics
    }

    /// Return token-consumption information, if recorded.
    #[must_use]
    pub const fn consumption(&self) -> Option<TokenConsumption> {
        self.consumption
    }

    /// Whether one or more diagnostics could not be retained.
    ///
    /// The parser or diagnostic collector must set this flag when its
    /// configured retention budget drops diagnostics.
    #[must_use]
    pub const fn diagnostics_truncated(&self) -> bool {
        self.diagnostics_truncated
    }

    /// Whether the result contains at least one parser error or error-level
    /// diagnostic.
    ///
    /// This is independent of status: for example, a recovered parse normally
    /// has errors, while an incomplete result may have no diagnostics yet.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
            || self
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.severity().is_error())
    }

    /// Whether any diagnostics have been retained.
    #[must_use]
    pub fn has_diagnostics(&self) -> bool {
        !self.diagnostics.is_empty()
    }

    /// Replace token-consumption information.
    #[must_use]
    pub fn with_consumption(mut self, consumption: TokenConsumption) -> Self {
        self.consumption = Some(consumption);
        self
    }

    /// Set whether diagnostic retention was truncated.
    ///
    /// This does not change the status or synthesize a diagnostic. The
    /// component enforcing the diagnostic policy is responsible for recording
    /// the appropriate warning/error when possible.
    #[must_use]
    pub const fn with_diagnostics_truncated(mut self, truncated: bool) -> Self {
        self.diagnostics_truncated = truncated;
        self
    }

    /// Add a structured diagnostic, returning a capacity error if reserving
    /// storage fails.
    ///
    /// The result is unchanged if reservation fails.
    pub fn try_push_diagnostic(
        &mut self,
        diagnostic: ParserDiagnostic,
    ) -> Result<(), TryReserveError> {
        self.diagnostics.try_reserve(1)?;
        self.diagnostics.push(diagnostic);
        Ok(())
    }

    /// Add a structured diagnostic.
    ///
    /// Use [`ParseResult::try_push_diagnostic`] when the caller needs to
    /// handle a reported allocation-capacity failure.
    pub fn push_diagnostic(&mut self, diagnostic: ParserDiagnostic) {
        self.diagnostics.push(diagnostic);
    }

    /// Add a parser error and its corresponding structured diagnostic.
    ///
    /// The method reserves capacity for both collections before changing
    /// either collection. If reservation fails, neither collection's length
    /// changes.
    pub fn try_push_error(
        &mut self,
        error: ParseError,
    ) -> Result<(), TryReserveError> {
        let diagnostic = ParserDiagnostic::from_error(&error);

        self.errors.try_reserve(1)?;
        self.diagnostics.try_reserve(1)?;

        self.errors.push(error);
        self.diagnostics.push(diagnostic);
        Ok(())
    }

    /// Add a parser error and its corresponding structured diagnostic.
    ///
    /// This uses ordinary `Vec` allocation behavior. Call
    /// [`ParseResult::try_push_error`] when the caller needs an explicit
    /// capacity-error path.
    pub fn push_error(&mut self, error: ParseError) {
        let diagnostic = ParserDiagnostic::from_error(&error);
        self.errors.push(error);
        self.diagnostics.push(diagnostic);
    }

    /// Mark the result as recovered.
    ///
    /// The caller must only use this after the parser has successfully
    /// resumed and produced a usable result.
    #[must_use]
    pub const fn mark_recovered(mut self) -> Self {
        self.status = ParseStatus::Recovered;
        self
    }

    /// Mark the result as incomplete.
    #[must_use]
    pub const fn mark_incomplete(mut self) -> Self {
        self.status = ParseStatus::Incomplete;
        self
    }

    /// Mark the result as failed.
    ///
    /// Any retained partial value remains available through `value()`, but
    /// must not be treated as a complete parse.
    #[must_use]
    pub const fn mark_failed(mut self) -> Self {
        self.status = ParseStatus::Failed;
        self
    }

    /// Transform the retained value while preserving errors, diagnostics,
    /// status, token-consumption information, and truncation metadata.
    #[must_use]
    pub fn map<U>(self, transform: impl FnOnce(T) -> U) -> ParseResult<U> {
        ParseResult {
            value: self.value.map(transform),
            status: self.status,
            errors: self.errors,
            diagnostics: self.diagnostics,
            consumption: self.consumption,
            diagnostics_truncated: self.diagnostics_truncated,
        }
    }

    /// Borrow the result's value while preserving its metadata by reference.
    #[must_use]
    pub fn as_ref(&self) -> ParseResult<&T> {
        ParseResult {
            value: self.value.as_ref(),
            status: self.status,
            errors: self.errors.clone(),
            diagnostics: self.diagnostics.clone(),
            consumption: self.consumption,
            diagnostics_truncated: self.diagnostics_truncated,
        }
    }

    /// Return an iterator over retained parser errors.
    pub fn errors_iter(&self) -> impl ExactSizeIterator<Item = &ParseError> {
        self.errors.iter()
    }

    /// Return an iterator over retained diagnostics.
    pub fn diagnostics_iter(
        &self,
    ) -> impl ExactSizeIterator<Item = &ParserDiagnostic> {
        self.diagnostics.iter()
    }
}

impl<T> Default for ParseResult<T> {
    /// The default result is failed and contains no value or diagnostics.
    fn default() -> Self {
        Self::failed()
    }
}

impl<T> fmt::Display for ParseResult<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "parse {}", self.status)?;

        if let Some(consumption) = self.consumption {
            write!(
                formatter,
                " (consumed {} token(s)",
                consumption.consumed_tokens()
            )?;

            if let Some(remaining) = consumption.remaining_tokens() {
                write!(formatter, ", {remaining} remaining")?;
            }

            formatter.write_str(")")?;
        }

        if !self.errors.is_empty() {
            write!(formatter, ", {} error(s)", self.errors.len())?;
        }

        if !self.diagnostics.is_empty() {
            write!(
                formatter,
                ", {} diagnostic(s)",
                self.diagnostics.len()
            )?;
        }

        if self.diagnostics_truncated {
            formatter.write_str(", diagnostics truncated")?;
        }

        Ok(())
    }
}

/// Convert a parse result into its status when the caller does not need
/// to retain the parsed value or diagnostic collections.
impl<T> From<ParseResult<T>> for ParseStatus {
    fn from(result: ParseResult<T>) -> Self {
        result.status
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_error() -> ParseError {
        ParseError::new(
            super::super::error::ParseErrorKind::InternalInvariant {
                message: "test invariant".to_owned(),
            },
            ParseErrorClass::Internal,
        )
    }

    #[test]
    fn complete_result_retains_value_and_status() {
        let result = ParseResult::complete(42_u32);

        assert_eq!(result.status(), ParseStatus::Complete);
        assert_eq!(result.value(), Some(&42));
        assert!(result.is_complete());
        assert!(result.has_usable_result());
        assert!(!result.has_errors());
    }

    #[test]
    fn recovered_result_is_not_clean_completion() {
        let result = ParseResult::recovered("partial".to_owned());

        assert!(result.is_recovered());
        assert!(!result.is_complete());
        assert!(result.has_usable_result());
        assert_eq!(result.into_complete_value(), None);
    }

    #[test]
    fn incomplete_result_can_retain_partial_value() {
        let result = ParseResult::incomplete_with_partial(vec![1, 2, 3]);

        assert!(result.is_incomplete());
        assert!(result.has_value());
        assert!(!result.has_usable_result());
        assert_eq!(result.value().map(Vec::len), Some(3));
    }

    #[test]
    fn failed_result_can_retain_partial_value_for_tooling() {
        let result = ParseResult::failed_with_partial("partial".to_owned());

        assert!(result.is_failed());
        assert!(result.has_value());
        assert_eq!(result.into_usable_value(), None);
    }

    #[test]
    fn token_consumption_preserves_unknown_remaining_count() {
        let consumption = TokenConsumption::new(17, None);

        assert_eq!(consumption.consumed_tokens(), 17);
        assert_eq!(consumption.remaining_tokens(), None);
        assert!(!consumption.is_fully_consumed());
    }

    #[test]
    fn zero_remaining_tokens_means_fully_consumed() {
        let consumption = TokenConsumption::new(9, Some(0));

        assert!(consumption.is_fully_consumed());
    }

    #[test]
    fn adding_error_retains_error_and_structured_diagnostic() {
        let mut result = ParseResult::<()>::failed();
        result.push_error(sample_error());

        assert_eq!(result.errors().len(), 1);
        assert_eq!(result.diagnostics().len(), 1);
        assert!(result.has_errors());
        assert_eq!(
            result.diagnostics()[0].code(),
            "parser.internal_invariant"
        );
    }

    #[test]
    fn diagnostic_only_error_is_detected() {
        let mut result = ParseResult::<()>::complete(());
        result.push_diagnostic(ParserDiagnostic::new(
            "parser.test_error",
            ParserDiagnosticSeverity::Error,
            "test diagnostic",
            None,
        ));

        assert!(result.has_errors());
        assert!(result.has_diagnostics());
    }

    #[test]
    fn warnings_do_not_make_has_errors_true() {
        let mut result = ParseResult::<()>::complete(());
        result.push_diagnostic(ParserDiagnostic::new(
            "parser.test_warning",
            ParserDiagnosticSeverity::Warning,
            "test warning",
            None,
        ));

        assert!(!result.has_errors());
        assert!(result.has_diagnostics());
    }

    #[test]
    fn mapping_preserves_result_metadata() {
        let result = ParseResult::recovered(5_u32)
            .with_consumption(TokenConsumption::new(5, Some(2)))
            .with_diagnostics_truncated(true);

        let mapped = result.map(|value| value.to_string());

        assert_eq!(mapped.status(), ParseStatus::Recovered);
        assert_eq!(mapped.value().map(String::as_str), Some("5"));
        assert_eq!(
            mapped.consumption(),
            Some(TokenConsumption::new(5, Some(2)))
        );
        assert!(mapped.diagnostics_truncated());
    }

    #[test]
    fn borrowing_preserves_status_and_metadata() {
        let result = ParseResult::complete(vec![1, 2])
            .with_consumption(TokenConsumption::new(2, Some(0)));

        let borrowed = result.as_ref();

        assert_eq!(borrowed.status(), ParseStatus::Complete);
        assert_eq!(borrowed.value().map(Vec::as_slice), Some(&[1, 2][..]));
        assert_eq!(
            borrowed.consumption(),
            Some(TokenConsumption::new(2, Some(0)))
        );
    }

    #[test]
    fn marking_failed_does_not_discard_partial_value() {
        let result = ParseResult::complete(123).mark_failed();

        assert!(result.is_failed());
        assert_eq!(result.value(), Some(&123));
    }

    #[test]
    fn default_result_is_failed() {
        let result = ParseResult::<()>::default();

        assert!(result.is_failed());
        assert!(!result.has_value());
        assert!(result.errors().is_empty());
        assert!(result.diagnostics().is_empty());
    }

    #[test]
    fn status_display_is_stable_and_machine_simple() {
        assert_eq!(ParseStatus::Complete.to_string(), "complete");
        assert_eq!(ParseStatus::Recovered.to_string(), "recovered");
        assert_eq!(ParseStatus::Incomplete.to_string(), "incomplete");
        assert_eq!(ParseStatus::Failed.to_string(), "failed");
    }
}
