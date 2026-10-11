
#![forbid(unsafe_code)]

//! Conservative, deterministic lexical error recovery for the Zamani frontend.
//!
//! File: src/frontend/lexer/recovery.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provide reusable cursor synchronization primitives for recovering from
//! malformed lexical input without coupling recovery to language domains.
//!
//! # Responsibilities
//!
//! - Respect the configured `RecoveryPolicy`.
//! - Synchronize at caller-selected lexical boundaries.
//! - Preserve synchronization characters for the next scanner iteration.
//! - Guarantee measurable progress when recovery consumes source.
//! - Report exact source spans and skipped-source measurements.
//! - Distinguish stopping, successful synchronization, and EOF.
//! - Permit caller-defined recovery boundaries without a fixed vocabulary.
//! - Avoid manufacturing parser or semantic tokens.
//!
//! # Does not own
//!
//! - Token classification or token construction.
//! - Diagnostic code assignment or diagnostic collection.
//! - Lexical recognition of malformed constructs.
//! - Parser recovery or AST repair.
//! - Language-domain semantics.
//! - Hardware selection, resource allocation, or execution.
//!
//! # Integration
//!
//! - `cursor.rs` owns UTF-8-safe source advancement.
//! - `span.rs` owns canonical source ranges.
//! - `config.rs` owns `RecoveryPolicy`.
//! - `scanner.rs` invokes recovery after identifying malformed input.
//! - `diagnostics.rs` owns user-facing diagnostic records.
//! - `token.rs` and `token_kind.rs` own error-token representation.
//! - `api.rs` exposes the resulting token stream and scanner errors.
//!
//! The scanner must emit or retain the diagnostic for the original lexical
//! defect before treating recovery as successful.
//!
//! # Recovery contract
//!
//! Synchronization stops immediately before a boundary selected by the
//! caller. The boundary remains unconsumed so normal scanning can recognize
//! it. EOF is a valid synchronization result.
//!
//! Recovery cannot guarantee progress if it is called at EOF, if policy is
//! `Stop`, or if the current character is already a synchronization boundary.
//! Those conditions are reported explicitly; callers must not retry the same
//! unchanged state indefinitely.
//!
//! No fixed source-size, nesting, token-count, or hardware-capacity limit is
//! imposed by this module.

use core::fmt;
use core::ops::Range;

use super::config::RecoveryPolicy;
use super::cursor::{Cursor, CursorPosition};
use super::span::Span;

/// Why a recovery operation stopped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RecoveryStopReason {
    /// The configured policy requires scanning to stop.
    PolicyStop,

    /// A caller-defined synchronization boundary was reached.
    Boundary,

    /// The cursor reached the end of the source.
    EndOfInput,

    /// The cursor was already at a synchronization boundary.
    ///
    /// No input was consumed. The scanner should process that boundary
    /// normally instead of repeatedly invoking recovery at the same point.
    AlreadyAtBoundary,

    /// The caller requested recovery but the cursor was already exhausted.
    AlreadyAtEnd,
}

impl fmt::Display for RecoveryStopReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::PolicyStop => "recovery stopped by policy",
            Self::Boundary => "synchronization boundary reached",
            Self::EndOfInput => "end of input reached during recovery",
            Self::AlreadyAtBoundary => "cursor already at synchronization boundary",
            Self::AlreadyAtEnd => "cursor already at end of input",
        })
    }
}

/// The observable result of one recovery operation.
///
/// `recovery_span` is the half-open source span consumed by this operation.
/// `error_span` is supplied by the caller and identifies the original
/// malformed construct; it need not equal the recovery span.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryOutcome {
    error_span: Span,
    recovery_span: Span,
    reason: RecoveryStopReason,
    policy: RecoveryPolicy,
    consumed_bytes: usize,
    consumed_scalars: usize,
}

impl RecoveryOutcome {
    /// Returns the span of the original lexical error.
    #[must_use]
    pub const fn error_span(&self) -> Span {
        self.error_span
    }

    /// Returns the span consumed during recovery.
    #[must_use]
    pub const fn recovery_span(&self) -> Span {
        self.recovery_span
    }

    /// Returns why recovery stopped.
    #[must_use]
    pub const fn reason(&self) -> RecoveryStopReason {
        self.reason
    }

    /// Returns the recovery policy used.
    #[must_use]
    pub const fn policy(&self) -> RecoveryPolicy {
        self.policy
    }

    /// Returns the number of UTF-8 bytes consumed during recovery.
    #[must_use]
    pub const fn consumed_bytes(&self) -> usize {
        self.consumed_bytes
    }

    /// Returns the number of Unicode scalar values consumed during recovery.
    #[must_use]
    pub const fn consumed_scalars(&self) -> usize {
        self.consumed_scalars
    }

    /// Returns whether the cursor advanced.
    #[must_use]
    pub const fn made_progress(&self) -> bool {
        self.consumed_bytes > 0
    }

    /// Returns whether a synchronization boundary was reached.
    #[must_use]
    pub const fn reached_boundary(&self) -> bool {
        matches!(
            self.reason,
            RecoveryStopReason::Boundary | RecoveryStopReason::AlreadyAtBoundary
        )
    }

    /// Returns whether the cursor reached EOF.
    #[must_use]
    pub const fn reached_eof(&self) -> bool {
        matches!(
            self.reason,
            RecoveryStopReason::EndOfInput | RecoveryStopReason::AlreadyAtEnd
        )
    }

    /// Returns the half-open byte range consumed during recovery.
    #[must_use]
    pub fn byte_range(&self) -> Range<usize> {
        self.recovery_span.byte_range()
    }
}

/// A reusable recovery engine.
///
/// The engine is stateless. Each operation uses the caller's cursor, policy,
/// error span, and boundary predicate. This avoids hidden state that could
/// make repeated scans nondeterministic.
#[derive(Debug, Clone, Copy, Default)]
pub struct RecoveryEngine;

impl RecoveryEngine {
    /// Creates a recovery engine.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Synchronizes the cursor using a caller-provided boundary predicate.
    ///
    /// The boundary predicate is evaluated on the next Unicode scalar.
    /// When it returns `true`, the scalar remains unconsumed.
    ///
    /// The caller should choose boundaries appropriate to the lexical
    /// construct being recovered. For example, an invalid character may
    /// synchronize at whitespace or punctuation, while a malformed string
    /// may synchronize at its closing quote or a line terminator.
    ///
    /// `error_span` must identify the original lexical error. This method
    /// does not infer that span or emit its diagnostic.
    ///
    /// # Determinism
    ///
    /// Given the same source, cursor position, policy, and pure boundary
    /// predicate, the operation produces the same result.
    ///
    /// # Progress
    ///
    /// This method deliberately preserves an immediate boundary. If called
    /// twice without moving the cursor in that state, it will return
    /// `AlreadyAtBoundary` twice. The scanner must process the boundary
    /// normally rather than retry recovery indefinitely.
    pub fn synchronize<F>(
        &self,
        cursor: &mut Cursor<'_>,
        policy: RecoveryPolicy,
        error_span: Span,
        mut is_boundary: F,
    ) -> RecoveryOutcome
    where
        F: FnMut(char) -> bool,
    {
        let start = cursor.position();

        if policy == RecoveryPolicy::Stop {
            return Self::outcome(
                error_span,
                start,
                start,
                RecoveryStopReason::PolicyStop,
                policy,
                0,
            );
        }

        if cursor.is_eof() {
            return Self::outcome(
                error_span,
                start,
                start,
                RecoveryStopReason::AlreadyAtEnd,
                policy,
                0,
            );
        }

        if cursor.peek().is_some_and(&mut is_boundary) {
            return Self::outcome(
                error_span,
                start,
                start,
                RecoveryStopReason::AlreadyAtBoundary,
                policy,
                0,
            );
        }

        let mut consumed_scalars = 0usize;

        while let Some(ch) = cursor.peek() {
            if is_boundary(ch) {
                return Self::outcome(
                    error_span,
                    start,
                    cursor.position(),
                    RecoveryStopReason::Boundary,
                    policy,
                    consumed_scalars,
                );
            }

            if cursor.consume().is_none() {
                // `peek()` observed a scalar immediately before this call.
                // The cursor contract makes this branch unreachable in
                // ordinary operation; retain a conservative terminal result
                // if the implementation contract ever changes.
                return Self::outcome(
                    error_span,
                    start,
                    cursor.position(),
                    RecoveryStopReason::EndOfInput,
                    policy,
                    consumed_scalars,
                );
            }

            consumed_scalars = consumed_scalars.saturating_add(1);
        }

        Self::outcome(
            error_span,
            start,
            cursor.position(),
            RecoveryStopReason::EndOfInput,
            policy,
            consumed_scalars,
        )
    }

    /// Consumes one known-invalid scalar, then synchronizes at a boundary.
    ///
    /// Use this variant only when the cursor still points at the first
    /// malformed scalar and the caller has established that consuming that
    /// scalar is correct. Do not use it if the recognizer already consumed
    /// the malformed character.
    ///
    /// If policy is `Stop`, no source is consumed.
    /// If the cursor is at EOF, the result reports that condition.
    /// After consuming the invalid scalar, synchronization preserves the
    /// next matching boundary.
    pub fn consume_invalid_and_synchronize<F>(
        &self,
        cursor: &mut Cursor<'_>,
        policy: RecoveryPolicy,
        error_span: Span,
        mut is_boundary: F,
    ) -> RecoveryOutcome
    where
        F: FnMut(char) -> bool,
    {
        let start = cursor.position();

        if policy == RecoveryPolicy::Stop {
            return Self::outcome(
                error_span,
                start,
                start,
                RecoveryStopReason::PolicyStop,
                policy,
                0,
            );
        }

        let Some(invalid) = cursor.peek() else {
            return Self::outcome(
                error_span,
                start,
                start,
                RecoveryStopReason::AlreadyAtEnd,
                policy,
                0,
            );
        };

        // Consume exactly one scalar first. This guarantees progress even
        // when the malformed scalar itself matches a broad boundary rule.
        let _ = cursor.consume();
        let mut consumed_scalars = 1usize;

        while let Some(ch) = cursor.peek() {
            if is_boundary(ch) {
                return Self::outcome(
                    error_span,
                    start,
                    cursor.position(),
                    RecoveryStopReason::Boundary,
                    policy,
                    consumed_scalars,
                );
            }

            if cursor.consume().is_none() {
                break;
            }

            consumed_scalars = consumed_scalars.saturating_add(1);
        }

        let reason = if cursor.is_eof() {
            RecoveryStopReason::EndOfInput
        } else if is_boundary(invalid) {
            RecoveryStopReason::Boundary
        } else {
            RecoveryStopReason::EndOfInput
        };

        Self::outcome(
            error_span,
            start,
            cursor.position(),
            reason,
            policy,
            consumed_scalars,
        )
    }

    fn outcome(
        error_span: Span,
        start: CursorPosition,
        end: CursorPosition,
        reason: RecoveryStopReason,
        policy: RecoveryPolicy,
        consumed_scalars: usize,
    ) -> RecoveryOutcome {
        // Cursor positions are monotonically ordered. This is an internal
        // invariant guaranteed by Cursor's public advancement operations.
        let recovery_span = Span::new(start, end);
        let consumed_bytes = end.byte_offset().saturating_sub(start.byte_offset());

        RecoveryOutcome {
            error_span,
            recovery_span,
            reason,
            policy,
            consumed_bytes,
            consumed_scalars,
        }
    }
}

/// Common synchronization predicates for lexical recovery.
///
/// These are helpers, not a complete language vocabulary. A scanner may use
/// its registries or construct-specific rules to select additional
/// boundaries without editing this module.
pub mod boundaries {
    /// Matches Unicode whitespace.
    ///
    /// The cursor and whitespace recognizer remain responsible for the
    /// language's exact whitespace policy.
    #[must_use]
    pub fn whitespace(ch: char) -> bool {
        ch.is_whitespace()
    }

    /// Matches a logical line terminator recognized by the cursor.
    #[must_use]
    pub const fn line_terminator(ch: char) -> bool {
        matches!(ch, '\r' | '\n' | '\u{2028}' | '\u{2029}')
    }

    /// Matches a common statement or grouping delimiter.
    ///
    /// This is only a convenience predicate. Use the canonical token and
    /// punctuation registries when a construct requires exact token rules.
    #[must_use]
    pub const fn delimiter(ch: char) -> bool {
        matches!(
            ch,
            ';' | ',' | ')' | ']' | '}' | '(' | '[' | '{'
        )
    }

    /// Matches a quote used to terminate a string or character literal.
    #[must_use]
    pub const fn quote(ch: char) -> bool {
        matches!(ch, '"' | '\'')
    }

    /// Matches any of the common lexical recovery boundaries.
    #[must_use]
    pub fn common(ch: char) -> bool {
        whitespace(ch) || line_terminator(ch) || delimiter(ch)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        boundaries, RecoveryEngine, RecoveryStopReason,
    };
    use crate::frontend::lexer::config::RecoveryPolicy;
    use crate::frontend::lexer::cursor::Cursor;
    use crate::frontend::lexer::span::Span;

    fn error_span(source: &str, start: usize, end: usize) -> Span {
        let mut cursor = Cursor::new(source);

        while cursor.byte_offset() < start {
            assert!(cursor.consume().is_some());
        }
        let start_position = cursor.position();

        while cursor.byte_offset() < end {
            assert!(cursor.consume().is_some());
        }
        let end_position = cursor.position();

        Span::new(start_position, end_position)
    }

    #[test]
    fn synchronization_preserves_the_boundary() {
        let source = "bad; next";
        let mut cursor = Cursor::new(source);
        let engine = RecoveryEngine::new();

        let outcome = engine.synchronize(
            &mut cursor,
            RecoveryPolicy::EmitErrorTokens,
            error_span(source, 0, 3),
            |ch| ch == ';',
        );

        assert_eq!(outcome.reason(), RecoveryStopReason::Boundary);
        assert_eq!(outcome.byte_range(), 0..3);
        assert_eq!(cursor.peek(), Some(';'));
        assert!(outcome.made_progress());
    }

    #[test]
    fn synchronization_reports_immediate_boundary_without_consuming_it() {
        let source = ";valid";
        let mut cursor = Cursor::new(source);
        let engine = RecoveryEngine::new();

        let outcome = engine.synchronize(
            &mut cursor,
            RecoveryPolicy::EmitErrorTokens,
            error_span(source, 0, 0),
            |ch| ch == ';',
        );

        assert_eq!(
            outcome.reason(),
            RecoveryStopReason::AlreadyAtBoundary
        );
        assert!(!outcome.made_progress());
        assert_eq!(cursor.byte_offset(), 0);
        assert_eq!(cursor.peek(), Some(';'));
    }

    #[test]
    fn stop_policy_never_advances_the_cursor() {
        let source = "invalid source";
        let mut cursor = Cursor::new(source);
        let engine = RecoveryEngine::new();

        let outcome = engine.synchronize(
            &mut cursor,
            RecoveryPolicy::Stop,
            error_span(source, 0, 7),
            boundaries::common,
        );

        assert_eq!(outcome.reason(), RecoveryStopReason::PolicyStop);
        assert_eq!(cursor.byte_offset(), 0);
        assert!(!outcome.made_progress());
    }

    #[test]
    fn synchronization_can_reach_eof() {
        let source = "malformed";
        let mut cursor = Cursor::new(source);
        let engine = RecoveryEngine::new();

        let outcome = engine.synchronize(
            &mut cursor,
            RecoveryPolicy::SkipInvalidInput,
            error_span(source, 0, source.len()),
            |_| false,
        );

        assert_eq!(outcome.reason(), RecoveryStopReason::EndOfInput);
        assert!(outcome.reached_eof());
        assert_eq!(outcome.byte_range(), 0..source.len());
        assert_eq!(cursor.byte_offset(), source.len());
    }

    #[test]
    fn already_at_eof_is_a_valid_result() {
        let source = "";
        let mut cursor = Cursor::new(source);
        let engine = RecoveryEngine::new();

        let outcome = engine.synchronize(
            &mut cursor,
            RecoveryPolicy::EmitErrorTokens,
            error_span(source, 0, 0),
            boundaries::common,
        );

        assert_eq!(outcome.reason(), RecoveryStopReason::AlreadyAtEnd);
        assert_eq!(outcome.consumed_bytes(), 0);
        assert!(outcome.reached_eof());
    }

    #[test]
    fn consume_invalid_guarantees_progress_on_nonempty_input() {
        let source = "§; let value = 1;";
        let mut cursor = Cursor::new(source);
        let engine = RecoveryEngine::new();

        let outcome = engine.consume_invalid_and_synchronize(
            &mut cursor,
            RecoveryPolicy::EmitErrorTokens,
            error_span(source, 0, "§".len()),
            |ch| ch == ';',
        );

        assert!(outcome.made_progress());
        assert_eq!(outcome.reason(), RecoveryStopReason::Boundary);
        assert_eq!(cursor.peek(), Some(';'));
        assert_eq!(outcome.consumed_bytes(), "§".len());
        assert_eq!(outcome.consumed_scalars(), 1);
    }

    #[test]
    fn recovery_counts_unicode_scalars_and_utf8_bytes_separately() {
        let source = "λ界;";
        let mut cursor = Cursor::new(source);
        let engine = RecoveryEngine::new();

        let outcome = engine.synchronize(
            &mut cursor,
            RecoveryPolicy::SkipInvalidInput,
            error_span(source, 0, 5),
            |ch| ch == ';',
        );

        assert_eq!(outcome.consumed_scalars(), 2);
        assert_eq!(outcome.consumed_bytes(), 5);
        assert_eq!(cursor.peek(), Some(';'));
    }

    #[test]
    fn newline_predicate_covers_supported_line_terminators() {
        for ch in ['\r', '\n', '\u{2028}', '\u{2029}'] {
            assert!(boundaries::line_terminator(ch));
        }
        assert!(!boundaries::line_terminator('x'));
    }

    #[test]
    fn common_boundary_is_deterministic() {
        for ch in [' ', '\t', '\n', ';', ')', '}', ','] {
            assert!(boundaries::common(ch));
        }
        for ch in ['x', 'λ', '§'] {
            assert!(!boundaries::common(ch));
        }
    }

    #[test]
    fn error_span_is_preserved_separately_from_recovery_span() {
        let source = "§ invalid;";
        let mut cursor = Cursor::new(source);
        let engine = RecoveryEngine::new();
        let original_error = error_span(source, 0, "§".len());

        let outcome = engine.synchronize(
            &mut cursor,
            RecoveryPolicy::SkipInvalidInput,
            original_error,
            |ch| ch == ';',
        );

        assert_eq!(outcome.error_span(), original_error);
        assert_ne!(outcome.error_span(), outcome.recovery_span());
        assert_eq!(outcome.recovery_span().start_byte(), 0);
        assert_eq!(outcome.recovery_span().end_byte(), source.len() - 1);
    }

    #[test]
    fn recovery_does_not_consume_a_boundary_after_invalid_input() {
        let source = "§\nlet x = 1;";
        let mut cursor = Cursor::new(source);
        let engine = RecoveryEngine::new();

        let outcome = engine.consume_invalid_and_synchronize(
            &mut cursor,
            RecoveryPolicy::EmitErrorTokens,
            error_span(source, 0, "§".len()),
            boundaries::line_terminator,
        );

        assert_eq!(outcome.reason(), RecoveryStopReason::Boundary);
        assert_eq!(cursor.peek(), Some('\n'));
        assert!(outcome.made_progress());
    }
}
