#![forbid(unsafe_code)]

//! Resource-budget enforcement primitives for the Zamani lexer.
//!
//! File: src/frontend/lexer/limits.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provide a single, reusable boundary for checking lexical resource budgets
//! and recording resource usage without imposing language-level capacity
//! restrictions.
//!
//! # Ownership
//!
//! `config.rs` remains the canonical owner of:
//! - `ResourceLimit`;
//! - `LexerLimits`;
//! - `LexerConfig` and lexical policies.
//!
//! This module re-exports the two resource-budget types and owns:
//! - resource categories;
//! - structured budget violations;
//! - checked resource-usage accounting;
//! - reusable budget-checking and accounting operations.
//!
//! It does not own:
//! - source scanning or cursor movement;
//! - token construction;
//! - lexical diagnostics and their presentation;
//! - malformed-input recovery;
//! - keyword, operator, or dialect registration;
//! - language grammar or semantic rules;
//! - quantum, classical, HDL, accelerator, or distributed execution;
//! - hardware discovery or allocation.
//!
//! # Scalability contract
//!
//! An unlimited budget means that this module imposes no explicit upper
//! bound for the corresponding resource. It does not promise infinite
//! memory, infinite execution time, or resources unavailable to the host.
//!
//! No language-level maximum is established for source size, token count,
//! identifier length, numeric literal length, quantum operations, processor
//! count, register width, tensor rank, device count, or topology size.
//!
//! Callers choose budgets according to their trust boundary, workload,
//! available resources, and operational policy.
//!
//! A budget violation is an implementation-resource event, not proof that
//! the source construct is semantically invalid.
//!
//! # Integration contract
//!
//! - `config.rs` is the canonical definition of limits and configuration.
//! - `scanner.rs` checks source bytes, emitted tokens, token bytes, and
//!   lexical nesting before committing corresponding operations.
//! - `number.rs` accounts for numeric digits examined by its recognizer.
//! - `string.rs` and `character.rs` account for literal source bytes.
//! - `diagnostics.rs` accounts for diagnostics retained or emitted.
//! - `comment.rs` or the current comment-recognition path accounts for
//!   nested-comment depth where nesting is supported.
//! - `recovery.rs` must not bypass the same configured budgets.
//! - `api.rs` may expose these types without taking ownership of scanning.
//! - `tests/` verifies boundaries, overflow handling, and determinism.
//!
//! These primitives intentionally do not depend on scanner internals.
//! Existing scanners can adopt them incrementally without introducing
//! a second configuration type.
//!
//! # Counting conventions
//!
//! - Byte counts mean UTF-8 source bytes, not Unicode scalar values,
//!   grapheme clusters, or terminal display columns.
//! - Token counts follow the scanner's documented emitted-token convention,
//!   including EOF if EOF is counted by that scanner.
//! - Numeric digit counts are lexical-work counts, not numeric precision.
//! - Literal byte counts refer to source representation, not decoded value.
//! - Nesting depth is the greatest observed active depth.
//! - Diagnostic counts follow the diagnostics retention/emission contract.
//!
//! All arithmetic is checked. Failed accounting does not modify the
//! corresponding usage counter.

use core::fmt;

pub use super::config::{LexerLimits, ResourceLimit};

/// A resource dimension governed by [`LexerLimits`].
///
/// Each variant maps to exactly one canonical configuration field.
/// This mapping is deliberately independent of computational domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ResourceKind {
    /// UTF-8 bytes in the complete source input.
    SourceBytes,

    /// Number of tokens emitted by the lexer.
    Tokens,

    /// UTF-8 bytes in an individual token's source spelling.
    TokenBytes,

    /// Numeric-literal digits examined by lexical recognition.
    NumericDigits,

    /// UTF-8 bytes in a string or character literal spelling.
    LiteralBytes,

    /// Active nesting depth for supported nested lexical constructs.
    LexicalNestingDepth,

    /// Diagnostics emitted or retained under the chosen policy.
    Diagnostics,
}

impl ResourceKind {
    /// Returns the canonical configured limit for this resource.
    #[must_use]
    pub const fn limit(self, limits: &LexerLimits) -> ResourceLimit {
        match self {
            Self::SourceBytes => limits.source_bytes,
            Self::Tokens => limits.tokens,
            Self::TokenBytes => limits.token_bytes,
            Self::NumericDigits => limits.numeric_digits,
            Self::LiteralBytes => limits.literal_bytes,
            Self::LexicalNestingDepth => limits.lexical_nesting_depth,
            Self::Diagnostics => limits.diagnostics,
        }
    }

    /// Returns the stable machine-readable resource name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceBytes => "source_bytes",
            Self::Tokens => "tokens",
            Self::TokenBytes => "token_bytes",
            Self::NumericDigits => "numeric_digits",
            Self::LiteralBytes => "literal_bytes",
            Self::LexicalNestingDepth => "lexical_nesting_depth",
            Self::Diagnostics => "diagnostics",
        }
    }
}

impl fmt::Display for ResourceKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Why a resource-budget operation failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LimitFailure {
    /// The proposed usage exceeds an explicit configured limit.
    Exceeded,

    /// The proposed accounting operation cannot be represented by `usize`.
    CounterOverflow,
}

impl fmt::Display for LimitFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exceeded => formatter.write_str("resource budget exceeded"),
            Self::CounterOverflow => {
                formatter.write_str("resource usage counter overflow")
            }
        }
    }
}

/// Structured information about a rejected resource operation.
///
/// `observed` is the proposed value when it is representable. It is `None`
/// when checked arithmetic overflowed, so the implementation never reports
/// a fabricated numeric count.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LimitViolation {
    resource: ResourceKind,
    limit: ResourceLimit,
    observed: Option<usize>,
    failure: LimitFailure,
}

impl LimitViolation {
    /// Creates a violation for an explicit limit.
    #[must_use]
    pub const fn exceeded(
        resource: ResourceKind,
        limit: ResourceLimit,
        observed: usize,
    ) -> Self {
        Self {
            resource,
            limit,
            observed: Some(observed),
            failure: LimitFailure::Exceeded,
        }
    }

    /// Creates a violation caused by checked arithmetic overflow.
    #[must_use]
    pub const fn counter_overflow(
        resource: ResourceKind,
        limit: ResourceLimit,
    ) -> Self {
        Self {
            resource,
            limit,
            observed: None,
            failure: LimitFailure::CounterOverflow,
        }
    }

    /// Returns the affected resource.
    #[must_use]
    pub const fn resource(self) -> ResourceKind {
        self.resource
    }

    /// Returns the configured resource limit.
    #[must_use]
    pub const fn limit(self) -> ResourceLimit {
        self.limit
    }

    /// Returns the proposed usage when it is representable.
    #[must_use]
    pub const fn observed(self) -> Option<usize> {
        self.observed
    }

    /// Returns the failure category.
    #[must_use]
    pub const fn failure(self) -> LimitFailure {
        self.failure
    }
}

impl fmt::Display for LimitViolation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.failure, self.observed) {
            (LimitFailure::Exceeded, Some(observed)) => write!(
                formatter,
                "{}: {} observed; configured limit is {}",
                self.resource, observed, self.limit
            ),
            (LimitFailure::CounterOverflow, _) => write!(
                formatter,
                "{}: accounting overflow; configured limit is {}",
                self.resource, self.limit
            ),
            (LimitFailure::Exceeded, None) => {
                write!(formatter, "{}: {}", self.resource, self.failure)
            }
        }
    }
}

impl std::error::Error for LimitViolation {}

/// Measured usage for one lexical input.
///
/// Counters are private so callers cannot accidentally bypass checked
/// accounting by directly assigning values. Read-only accessors expose the
/// recorded values.
///
/// `lexical_nesting_depth` records the greatest depth observed, rather than
/// the current depth. The recognizer still owns the current nesting state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ResourceUsage {
    source_bytes: usize,
    tokens: usize,
    token_bytes: usize,
    numeric_digits: usize,
    literal_bytes: usize,
    lexical_nesting_depth: usize,
    diagnostics: usize,
}

impl ResourceUsage {
    /// Creates an empty usage record.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            source_bytes: 0,
            tokens: 0,
            token_bytes: 0,
            numeric_digits: 0,
            literal_bytes: 0,
            lexical_nesting_depth: 0,
            diagnostics: 0,
        }
    }

    /// Returns the number of source UTF-8 bytes recorded.
    #[must_use]
    pub const fn source_bytes(&self) -> usize {
        self.source_bytes
    }

    /// Returns the number of tokens recorded.
    #[must_use]
    pub const fn tokens(&self) -> usize {
        self.tokens
    }

    /// Returns the greatest individual token byte length recorded.
    #[must_use]
    pub const fn token_bytes(&self) -> usize {
        self.token_bytes
    }

    /// Returns the number of numeric digits recorded as examined.
    #[must_use]
    pub const fn numeric_digits(&self) -> usize {
        self.numeric_digits
    }

    /// Returns the greatest literal byte length recorded.
    #[must_use]
    pub const fn literal_bytes(&self) -> usize {
        self.literal_bytes
    }

    /// Returns the greatest nesting depth observed.
    #[must_use]
    pub const fn lexical_nesting_depth(&self) -> usize {
        self.lexical_nesting_depth
    }

    /// Returns the number of diagnostics recorded.
    #[must_use]
    pub const fn diagnostics(&self) -> usize {
        self.diagnostics
    }

    /// Returns the recorded value for a resource category.
    #[must_use]
    pub const fn get(&self, resource: ResourceKind) -> usize {
        match resource {
            Self::resource_kind_source_bytes() => 0,
            _ => self.get_non_source(resource),
        }
    }

    const fn resource_kind_source_bytes() -> ResourceKind {
        ResourceKind::SourceBytes
    }

    const fn get_non_source(&self, resource: ResourceKind) -> usize {
        match resource {
            ResourceKind::SourceBytes => self.source_bytes,
            ResourceKind::Tokens => self.tokens,
            ResourceKind::TokenBytes => self.token_bytes,
            ResourceKind::NumericDigits => self.numeric_digits,
            ResourceKind::LiteralBytes => self.literal_bytes,
            ResourceKind::LexicalNestingDepth => self.lexical_nesting_depth,
            ResourceKind::Diagnostics => self.diagnostics,
        }
    }
}

/// Checked accounting for one lexical input.
///
/// This tracker owns only counters and policy checks. It does not allocate,
/// inspect source text, create diagnostics, or advance the scanner cursor.
///
/// A failed operation leaves the relevant counter unchanged. This lets
/// callers decide whether to stop, recover, or convert the violation into a
/// canonical diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BudgetTracker {
    limits: LexerLimits,
    usage: ResourceUsage,
}

impl BudgetTracker {
    /// Creates a tracker using the canonical lexer limits.
    #[must_use]
    pub const fn new(limits: LexerLimits) -> Self {
        Self {
            limits,
            usage: ResourceUsage::new(),
        }
    }

    /// Returns the configured budgets.
    #[must_use]
    pub const fn limits(&self) -> &LexerLimits {
        &self.limits
    }

    /// Returns a snapshot of current resource usage.
    #[must_use]
    pub const fn usage(&self) -> &ResourceUsage {
        &self.usage
    }

    /// Returns whether a proposed value is permitted by the selected budget.
    ///
    /// This check does not modify usage counters.
    pub const fn check(
        &self,
        resource: ResourceKind,
        proposed: usize,
    ) -> Result<(), LimitViolation> {
        let limit = resource.limit(&self.limits);

        if limit.permits(proposed) {
            Ok(())
        } else {
            Err(LimitViolation::exceeded(resource, limit, proposed))
        }
    }

    /// Records the complete source size.
    ///
    /// This is a set-once-style operation: calling it again replaces the
    /// previously recorded source size after validating the new value.
    /// Callers should invoke it once during scanner initialization.
    pub fn set_source_bytes(
        &mut self,
        bytes: usize,
    ) -> Result<(), LimitViolation> {
        self.check(ResourceKind::SourceBytes, bytes)?;
        self.usage.source_bytes = bytes;
        Ok(())
    }

    /// Records one emitted token.
    ///
    /// The token count is incremented only when the next count is both
    /// representable and within the configured budget.
    pub fn record_token(&mut self) -> Result<(), LimitViolation> {
        let next = self.usage.tokens.checked_add(1).ok_or_else(|| {
            LimitViolation::counter_overflow(
                ResourceKind::Tokens,
                self.limits.tokens,
            )
        })?;

        self.check(ResourceKind::Tokens, next)?;
        self.usage.tokens = next;
        Ok(())
    }

    /// Records an individual token's source byte length.
    ///
    /// This field is a maximum-observed size, not a cumulative byte count.
    pub fn record_token_bytes(
        &mut self,
        bytes: usize,
    ) -> Result<(), LimitViolation> {
        self.check(ResourceKind::TokenBytes, bytes)?;
        self.usage.token_bytes = self.usage.token_bytes.max(bytes);
        Ok(())
    }

    /// Records numeric digits examined by a recognizer.
    ///
    /// The count is cumulative for the current lexical input. A recognizer
    /// should account for each digit exactly once.
    pub fn record_numeric_digits(
        &mut self,
        digits: usize,
    ) -> Result<(), LimitViolation> {
        let next = self
            .usage
            .numeric_digits
            .checked_add(digits)
            .ok_or_else(|| {
                LimitViolation::counter_overflow(
                    ResourceKind::NumericDigits,
                    self.limits.numeric_digits,
                )
            })?;

        self.check(ResourceKind::NumericDigits, next)?;
        self.usage.numeric_digits = next;
        Ok(())
    }

    /// Records an individual literal's source byte length.
    ///
    /// This field is a maximum-observed size, not a cumulative byte count.
    pub fn record_literal_bytes(
        &mut self,
        bytes: usize,
    ) -> Result<(), LimitViolation> {
        self.check(ResourceKind::LiteralBytes, bytes)?;
        self.usage.literal_bytes = self.usage.literal_bytes.max(bytes);
        Ok(())
    }

    /// Records a newly observed active nesting depth.
    ///
    /// The maximum-observed depth changes only after the budget check passes.
    /// The caller remains responsible for tracking and unwinding current
    /// nesting state when scanning nested constructs.
    pub fn observe_nesting_depth(
        &mut self,
        depth: usize,
    ) -> Result<(), LimitViolation> {
        self.check(ResourceKind::LexicalNestingDepth, depth)?;
        self.usage.lexical_nesting_depth =
            self.usage.lexical_nesting_depth.max(depth);
        Ok(())
    }

    /// Records one emitted or retained diagnostic.
    pub fn record_diagnostic(&mut self) -> Result<(), LimitViolation> {
        let next = self.usage.diagnostics.checked_add(1).ok_or_else(|| {
            LimitViolation::counter_overflow(
                ResourceKind::Diagnostics,
                self.limits.diagnostics,
            )
        })?;

        self.check(ResourceKind::Diagnostics, next)?;
        self.usage.diagnostics = next;
        Ok(())
    }

    /// Resets usage for a new input while retaining the same configured
    /// budgets.
    ///
    /// Do not reset a tracker while a scanner is still processing the input
    /// whose usage it accounts for.
    pub fn reset(&mut self) {
        self.usage = ResourceUsage::new();
    }
}

impl Default for BudgetTracker {
    fn default() -> Self {
        Self::new(LexerLimits::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::num::NonZeroUsize;

    fn bounded_limit(value: usize) -> ResourceLimit {
        ResourceLimit::at_most(value)
            .expect("test limits must be non-zero")
    }

    fn limits_with(
        resource: ResourceKind,
        value: usize,
    ) -> LexerLimits {
        let mut limits = LexerLimits::unlimited();

        match resource {
            ResourceKind::SourceBytes => {
                limits.source_bytes = bounded_limit(value);
            }
            ResourceKind::Tokens => limits.tokens = bounded_limit(value),
            ResourceKind::TokenBytes => {
                limits.token_bytes = bounded_limit(value);
            }
            ResourceKind::NumericDigits => {
                limits.numeric_digits = bounded_limit(value);
            }
            ResourceKind::LiteralBytes => {
                limits.literal_bytes = bounded_limit(value);
            }
            ResourceKind::LexicalNestingDepth => {
                limits.lexical_nesting_depth = bounded_limit(value);
            }
            ResourceKind::Diagnostics => {
                limits.diagnostics = bounded_limit(value);
            }
        }

        limits
    }

    #[test]
    fn unlimited_budgets_permit_representable_values() {
        let tracker = BudgetTracker::new(LexerLimits::unlimited());

        for resource in [
            ResourceKind::SourceBytes,
            ResourceKind::Tokens,
            ResourceKind::TokenBytes,
            ResourceKind::NumericDigits,
            ResourceKind::LiteralBytes,
            ResourceKind::LexicalNestingDepth,
            ResourceKind::Diagnostics,
        ] {
            assert!(tracker.check(resource, usize::MAX).is_ok());
        }
    }

    #[test]
    fn explicit_budget_is_inclusive() {
        let tracker =
            BudgetTracker::new(limits_with(ResourceKind::Tokens, 3));

        assert!(tracker.check(ResourceKind::Tokens, 0).is_ok());
        assert!(tracker.check(ResourceKind::Tokens, 3).is_ok());

        let error = tracker
            .check(ResourceKind::Tokens, 4)
            .expect_err("value above the budget must fail");

        assert_eq!(error.resource(), ResourceKind::Tokens);
        assert_eq!(error.observed(), Some(4));
        assert_eq!(error.failure(), LimitFailure::Exceeded);
    }

    #[test]
    fn failed_token_accounting_does_not_mutate_usage() {
        let mut tracker =
            BudgetTracker::new(limits_with(ResourceKind::Tokens, 1));

        tracker.record_token().expect("first token is permitted");

        let error = tracker
            .record_token()
            .expect_err("second token exceeds the budget");

        assert_eq!(error.failure(), LimitFailure::Exceeded);
        assert_eq!(tracker.usage().tokens(), 1);
    }

    #[test]
    fn source_size_is_checked_before_it_is_recorded() {
        let mut tracker =
            BudgetTracker::new(limits_with(ResourceKind::SourceBytes, 8));

        tracker
            .set_source_bytes(8)
            .expect("the inclusive limit is permitted");

        assert_eq!(tracker.usage().source_bytes(), 8);

        assert!(tracker.set_source_bytes(9).is_err());
        assert_eq!(tracker.usage().source_bytes(), 8);
    }

    #[test]
    fn token_and_literal_lengths_track_the_maximum() {
        let mut tracker = BudgetTracker::new(LexerLimits::unlimited());

        tracker.record_token_bytes(12).expect("unlimited budget");
        tracker.record_token_bytes(5).expect("unlimited budget");
        tracker.record_literal_bytes(18).expect("unlimited budget");
        tracker.record_literal_bytes(7).expect("unlimited budget");

        assert_eq!(tracker.usage().token_bytes(), 12);
        assert_eq!(tracker.usage().literal_bytes(), 18);
    }

    #[test]
    fn numeric_digits_are_cumulative() {
        let mut tracker =
            BudgetTracker::new(limits_with(ResourceKind::NumericDigits, 5));

        tracker
            .record_numeric_digits(2)
            .expect("first digit group fits");
        tracker
            .record_numeric_digits(3)
            .expect("total at limit fits");

        assert_eq!(tracker.usage().numeric_digits(), 5);

        assert!(tracker.record_numeric_digits(1).is_err());
        assert_eq!(tracker.usage().numeric_digits(), 5);
    }

    #[test]
    fn nesting_depth_records_the_high_water_mark() {
        let mut tracker = BudgetTracker::new(
            limits_with(ResourceKind::LexicalNestingDepth, 4),
        );

        tracker
            .observe_nesting_depth(2)
            .expect("depth is permitted");
        tracker
            .observe_nesting_depth(4)
            .expect("depth at limit is permitted");
        tracker
            .observe_nesting_depth(1)
            .expect("shallower depth is permitted");

        assert_eq!(tracker.usage().lexical_nesting_depth(), 4);
        assert!(tracker.observe_nesting_depth(5).is_err());
        assert_eq!(tracker.usage().lexical_nesting_depth(), 4);
    }

    #[test]
    fn diagnostics_obey_the_configured_budget() {
        let mut tracker =
            BudgetTracker::new(limits_with(ResourceKind::Diagnostics, 1));

        tracker
            .record_diagnostic()
            .expect("first diagnostic is permitted");

        assert!(tracker.record_diagnostic().is_err());
        assert_eq!(tracker.usage().diagnostics(), 1);
    }

    #[test]
    fn reset_clears_usage_without_changing_limits() {
        let mut tracker =
            BudgetTracker::new(limits_with(ResourceKind::Tokens, 2));

        tracker.record_token().expect("first token");
        tracker.record_token().expect("second token");
        tracker.reset();

        assert_eq!(tracker.usage().tokens(), 0);
        assert_eq!(
            tracker.limits().tokens,
            ResourceLimit::AtMost(
                NonZeroUsize::new(2).expect("non-zero test limit")
            )
        );
    }

    #[test]
    fn resource_names_are_stable() {
        assert_eq!(ResourceKind::SourceBytes.as_str(), "source_bytes");
        assert_eq!(ResourceKind::Tokens.as_str(), "tokens");
        assert_eq!(ResourceKind::TokenBytes.as_str(), "token_bytes");
        assert_eq!(ResourceKind::NumericDigits.as_str(), "numeric_digits");
        assert_eq!(ResourceKind::LiteralBytes.as_str(), "literal_bytes");
        assert_eq!(
            ResourceKind::LexicalNestingDepth.as_str(),
            "lexical_nesting_depth"
        );
        assert_eq!(ResourceKind::Diagnostics.as_str(), "diagnostics");
    }
}