
#![forbid(unsafe_code)]

//! Per-invocation parser session for the Zamani frontend.
//!
//! File: src/frontend/parser/session.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Own all mutable state associated with one parser invocation:
//! - the validated canonical token cursor;
//! - cumulative parser resource accounting;
//! - parser diagnostics and structured errors;
//! - speculative parsing checkpoints;
//! - general and expression nesting depth;
//! - recovery progress metadata;
//! - deterministic session-local accounting.
//!
//! # Architectural boundaries
//!
//! This module does not:
//! - tokenize source or redefine lexer tokens;
//! - define a competing source-span representation;
//! - implement grammar productions;
//! - construct or redefine the canonical AST;
//! - resolve names, infer types, or validate program semantics;
//! - evaluate effects, capabilities, or resource contracts;
//! - discover hardware, allocate devices, schedule work, or execute programs.
//!
//! # Integration contract
//!
//! `token_cursor.rs` owns token-stream validation and cursor movement.
//! `limits.rs` owns parser budgets and checked accounting.
//! `checkpoint.rs` owns checkpoint identity and stack discipline.
//! `diagnostics.rs` owns structured parser diagnostics.
//! `error.rs` owns structured parser errors.
//! `config.rs` owns immutable parser policy.
//! `context.rs` owns immutable shared services.
//! `parser.rs` and grammar modules use this session for mutable state.
//!
//! A session is created for one parse operation and must not be reused for
//! an unrelated parse. Concurrent parses use separate sessions.
//!
//! # Resource policy
//!
//! A missing or unlimited budget is not a hardware-capacity limit.
//! Configured budgets bound parser implementation work only.
//!
//! Rolling back speculative syntax does not refund cumulative work or token
//! consumption. Diagnostics are not rolled back: speculative grammar branches
//! should defer reporting diagnostics until their branch is committed.
//!
//! # Allocation policy
//!
//! This module uses fallible reservation for its own error storage and
//! checkpoint allocation where the underlying APIs expose that behavior.
//! Rust's global allocator may still abort on allocation failure elsewhere;
//! this module does not claim to recover from every process-wide OOM.

use core::fmt;

use super::checkpoint::{
    Checkpoint,
    CheckpointError,
    CheckpointManager,
    CheckpointState,
};
use super::diagnostics::{
    DiagnosticCollection,
    ParserDiagnostic,
};
use super::error::{
    ParseError,
    ParseErrorClass,
    ParseErrorKind,
};
use super::limits::{
    ParserBudget,
    ParserLimitKind,
    ParserLimits,
    ParserUsage,
    ResourceLimit,
};
use super::token_cursor::TokenCursor;

/// Errors raised by session-local state management.
///
/// These are parser implementation errors, not source-language constructs.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SessionError {
    /// A nesting counter cannot be incremented without overflow.
    NestingOverflow {
        expression: bool,
    },

    /// A nesting counter cannot be decremented because it is already zero.
    NestingUnderflow {
        expression: bool,
    },

    /// The recovery generation cannot be incremented without overflow.
    RecoveryEpochExhausted,

    /// The requested operation violates a session invariant.
    InvariantViolation {
        message: String,
    },

    /// A parser checkpoint operation failed.
    Checkpoint(CheckpointError),

    /// A session-owned collection could not reserve additional storage.
    AllocationFailed,
}

impl fmt::Display for SessionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NestingOverflow { expression } => write!(
                f,
                "{} nesting depth cannot be incremented",
                if *expression {
                    "expression"
                } else {
                    "grammar"
                }
            ),
            Self::NestingUnderflow { expression } => write!(
                f,
                "{} nesting depth is already zero",
                if *expression {
                    "expression"
                } else {
                    "grammar"
                }
            ),
            Self::RecoveryEpochExhausted => {
                f.write_str("parser recovery generation counter exhausted")
            }
            Self::InvariantViolation { message } => {
                write!(f, "parser session invariant violated: {message}")
            }
            Self::Checkpoint(error) => {
                write!(f, "parser checkpoint operation failed: {error}")
            }
            Self::AllocationFailed => {
                f.write_str("unable to reserve parser session storage")
            }
        }
    }
}

impl std::error::Error for SessionError {}

impl From<CheckpointError> for SessionError {
    fn from(error: CheckpointError) -> Self {
        Self::Checkpoint(error)
    }
}

/// Outcome of retaining a parser diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagnosticOutcome {
    /// The diagnostic was retained.
    Added,

    /// The diagnostic exceeded the configured diagnostic-retention budget.
    LimitReached,

    /// Parser resource accounting rejected diagnostic emission.
    BudgetExceeded,
}

/// An opaque session identity supplied by the parser API or caller.
///
/// No global counter or process-wide mutable state is required. A caller
/// should supply a distinct identifier for each concurrently active session
/// if it needs checkpoint identities to be distinguishable across sessions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SessionId(u64);

impl SessionId {
    /// Construct a session identifier.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Return the underlying identifier.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// Mutable state for one parser invocation.
///
/// The token and source lifetimes are inherited from the canonical lexer
/// output. Tokens are borrowed, never copied or re-tokenized.
///
/// The session is deliberately not `Clone`: duplicating mutable parser state
/// would create ambiguous ownership of cursor progress and diagnostics.
pub struct ParserSession<'tokens, 'src> {
    id: SessionId,
    cursor: TokenCursor<'tokens, 'src>,
    budget: ParserBudget,
    diagnostics: DiagnosticCollection,
    errors: Vec<ParseError>,
    checkpoints: CheckpointManager,
    nesting_depth: usize,
    expression_depth: usize,
    recovery_epoch: u64,
    recovery_count: usize,
    failed: bool,
    recovered: bool,
    diagnostics_truncated: bool,
}

impl<'tokens, 'src> fmt::Debug for ParserSession<'tokens, 'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ParserSession")
            .field("id", &self.id)
            .field("cursor", &self.cursor)
            .field("usage", &self.budget.usage())
            .field("diagnostic_count", &self.diagnostics.len())
            .field("error_count", &self.errors.len())
            .field("checkpoint_depth", &self.checkpoints.depth())
            .field("nesting_depth", &self.nesting_depth)
            .field("expression_depth", &self.expression_depth)
            .field("recovery_epoch", &self.recovery_epoch)
            .field("recovery_count", &self.recovery_count)
            .field("failed", &self.failed)
            .field("recovered", &self.recovered)
            .field("diagnostics_truncated", &self.diagnostics_truncated)
            .finish()
    }
}

impl<'tokens, 'src> ParserSession<'tokens, 'src> {
    /// Create a fresh session around an already validated token cursor.
    ///
    /// `limits` must be the runtime accounting type from `limits.rs`, not
    /// the separate configuration-value type in `config.rs`.
    ///
    /// `diagnostic_limit` controls retained diagnostic count. The parser
    /// budget's diagnostic limit is enforced independently, so callers
    /// should configure both consistently.
    ///
    /// Source byte accounting is initialized from the final EOF span.
    /// The cursor validates that EOF is explicit and zero-width.
    pub fn new(
        id: SessionId,
        cursor: TokenCursor<'tokens, 'src>,
        limits: ParserLimits,
        diagnostic_limit: ResourceLimit,
    ) -> Result<Self, ParseError> {
        let source_bytes = cursor.span().end_byte();

        let mut budget = ParserBudget::new(limits);
        budget
            .set_source_bytes(source_bytes)
            .map_err(|violation| ParseError::resource_limit(violation, Some(cursor.span())))?;

        Ok(Self {
            id,
            cursor,
            budget,
            diagnostics: DiagnosticCollection::with_limit(diagnostic_limit),
            errors: Vec::new(),
            checkpoints: CheckpointManager::new(id.get()),
            nesting_depth: 0,
            expression_depth: 0,
            recovery_epoch: 0,
            recovery_count: 0,
            failed: false,
            recovered: false,
            diagnostics_truncated: false,
        })
    }

    /// Create a session with unlimited explicit parser budgets.
    ///
    /// This does not promise infinite memory, stack, address space, or time.
    pub fn unlimited(
        id: SessionId,
        cursor: TokenCursor<'tokens, 'src>,
    ) -> Result<Self, ParseError> {
        Self::new(
            id,
            cursor,
            ParserLimits::unlimited(),
            ResourceLimit::Unlimited,
        )
    }

    /// Return this session's identifier.
    #[must_use]
    pub const fn id(&self) -> SessionId {
        self.id
    }

    /// Borrow the current token cursor.
    #[must_use]
    pub const fn cursor(&self) -> &TokenCursor<'tokens, 'src> {
        &self.cursor
    }

    /// Mutably borrow the cursor for operations that do not consume tokens.
    ///
    /// Production grammar code should prefer `advance_token()` so cumulative
    /// token usage cannot accidentally be bypassed.
    #[must_use]
    pub fn cursor_mut(&mut self) -> &mut TokenCursor<'tokens, 'src> {
        &mut self.cursor
    }

    /// Return the current parser resource-usage snapshot.
    #[must_use]
    pub const fn usage(&self) -> ParserUsage {
        self.budget.usage()
    }

    /// Borrow the active resource policy.
    #[must_use]
    pub const fn limits(&self) -> &ParserLimits {
        self.budget.limits()
    }

    /// Return the number of retained diagnostics.
    #[must_use]
    pub fn diagnostic_count(&self) -> usize {
        self.diagnostics.len()
    }

    /// Return the retained parser diagnostics.
    #[must_use]
    pub fn diagnostics(&self) -> &[ParserDiagnostic] {
        self.diagnostics.diagnostics()
    }

    /// Return the number of diagnostics dropped by retention policy.
    #[must_use]
    pub const fn dropped_diagnostic_count(&self) -> usize {
        self.diagnostics.dropped_count()
    }

    /// Whether diagnostics were truncated.
    #[must_use]
    pub const fn diagnostics_truncated(&self) -> bool {
        self.diagnostics_truncated || self.diagnostics.is_truncated()
    }

    /// Return structured errors accumulated by the session.
    #[must_use]
    pub fn errors(&self) -> &[ParseError] {
        &self.errors
    }

    /// Return whether any fatal or internal error has marked this session
    /// failed.
    #[must_use]
    pub const fn is_failed(&self) -> bool {
        self.failed
    }

    /// Return whether syntax recovery has occurred.
    #[must_use]
    pub const fn has_recovered(&self) -> bool {
        self.recovered
    }

    /// Return the current grammar nesting depth.
    #[must_use]
    pub const fn nesting_depth(&self) -> usize {
        self.nesting_depth
    }

    /// Return the current expression nesting depth.
    #[must_use]
    pub const fn expression_depth(&self) -> usize {
        self.expression_depth
    }

    /// Return the current recovery generation.
    #[must_use]
    pub const fn recovery_epoch(&self) -> u64 {
        self.recovery_epoch
    }

    /// Return the number of recovery operations recorded by this session.
    #[must_use]
    pub const fn recovery_count(&self) -> usize {
        self.recovery_count
    }

    /// Return whether the cursor has reached EOF.
    #[must_use]
    pub fn is_eof(&self) -> bool {
        self.cursor.is_eof()
    }

    /// Advance one parser-visible token using cumulative budget accounting.
    ///
    /// EOF observations do not consume a token budget. The cursor owns the
    /// exact token-consumption convention.
    pub fn advance_token(
        &mut self,
    ) -> Result<&'tokens crate::frontend::lexer::token::Token<'src>, ParseError> {
        self.cursor.advance_with_budget(&mut self.budget)
    }

    /// Charge cumulative parser work before performing a unit of work.
    ///
    /// Callers must define and consistently apply their work-unit convention.
    /// A failed charge does not change the usage counter.
    pub fn charge_work(&mut self, amount: usize) -> Result<(), ParseError> {
        self.budget
            .charge(ParserLimitKind::TotalWork, amount)
            .map_err(|violation| {
                ParseError::resource_limit(violation, Some(self.cursor.span()))
            })
    }

    /// Charge the number of AST nodes before committing their construction.
    ///
    /// The grammar or AST adapter must call this consistently. The session
    /// cannot infer allocations made by code outside its control.
    pub fn charge_ast_nodes(&mut self, amount: usize) -> Result<(), ParseError> {
        self.budget
            .charge(ParserLimitKind::AstNodes, amount)
            .map_err(|violation| {
                ParseError::resource_limit(violation, Some(self.cursor.span()))
            })
    }

    /// Enter one grammar nesting level.
    ///
    /// The current level is checked before being committed. If the budget
    /// rejects the new level, the session depth remains unchanged.
    pub fn enter_nesting(&mut self) -> Result<(), ParseError> {
        let next = self.nesting_depth.checked_add(1).ok_or_else(|| {
            ParseError::new(
                ParseErrorKind::InternalInvariant {
                    message: "grammar nesting counter overflow".to_owned(),
                },
                ParseErrorClass::Internal,
            )
            .with_span(self.cursor.span())
        })?;

        self.budget
            .observe_max(ParserLimitKind::NestingDepth, next)
            .map_err(|violation| {
                ParseError::resource_limit(violation, Some(self.cursor.span()))
            })?;

        self.nesting_depth = next;
        Ok(())
    }

    /// Leave one grammar nesting level.
    ///
    /// An underflow is an internal invariant error; it is never silently
    /// converted to zero.
    pub fn leave_nesting(&mut self) -> Result<(), SessionError> {
        self.nesting_depth = self
            .nesting_depth
            .checked_sub(1)
            .ok_or(SessionError::NestingUnderflow { expression: false })?;
        Ok(())
    }

    /// Enter one expression nesting level.
    pub fn enter_expression(&mut self) -> Result<(), ParseError> {
        let next = self.expression_depth.checked_add(1).ok_or_else(|| {
            ParseError::new(
                ParseErrorKind::InternalInvariant {
                    message: "expression nesting counter overflow".to_owned(),
                },
                ParseErrorClass::Internal,
            )
            .with_span(self.cursor.span())
        })?;

        self.budget
            .observe_max(ParserLimitKind::ExpressionDepth, next)
            .map_err(|violation| {
                ParseError::resource_limit(violation, Some(self.cursor.span()))
            })?;

        self.expression_depth = next;
        Ok(())
    }

    /// Leave one expression nesting level.
    pub fn leave_expression(&mut self) -> Result<(), SessionError> {
        self.expression_depth = self
            .expression_depth
            .checked_sub(1)
            .ok_or(SessionError::NestingUnderflow { expression: true })?;
        Ok(())
    }

    /// Record a structured parser error.
    ///
    /// The error remains available in `errors()` even if diagnostic retention
    /// or diagnostic accounting prevents its corresponding diagnostic from
    /// being retained. Callers must inspect the returned outcome.
    pub fn record_error(
        &mut self,
        error: ParseError,
    ) -> Result<DiagnosticOutcome, ParseError> {
        self.errors
            .try_reserve(1)
            .map_err(|_| {
                ParseError::new(
                    ParseErrorKind::InternalInvariant {
                        message: "unable to reserve parser error storage".to_owned(),
                    },
                    ParseErrorClass::Internal,
                )
            })?;

        let class = error.class();
        self.errors.push(error.clone());

        if matches!(class, ParseErrorClass::Fatal | ParseErrorClass::Internal) {
            self.failed = true;
        }

        self.emit_error_diagnostic(&error)
    }

    /// Attempt to retain a structured diagnostic.
    ///
    /// The parser's diagnostic accounting budget and the collection's own
    /// retention policy are both enforced. The budget counts attempted
    /// emissions, including emissions rejected by the collection limit.
    pub fn emit_diagnostic(
        &mut self,
        diagnostic: ParserDiagnostic,
    ) -> Result<DiagnosticOutcome, ParseError> {
        self.budget
            .charge(ParserLimitKind::Diagnostics, 1)
            .map_err(|violation| {
                ParseError::resource_limit(violation, Some(self.cursor.span()))
            })?;

        let retained = self.diagnostics.push(diagnostic);
        if retained {
            Ok(DiagnosticOutcome::Added)
        } else {
            self.diagnostics_truncated = true;
            Ok(DiagnosticOutcome::LimitReached)
        }
    }

    /// Mark that a grammar branch recovered from malformed syntax.
    ///
    /// Recovery operations are charged before state changes. The recovery
    /// generation changes only after accounting succeeds.
    pub fn record_recovery(&mut self) -> Result<(), ParseError> {
        self.budget
            .charge(ParserLimitKind::TotalWork, 1)
            .map_err(|violation| {
                ParseError::resource_limit(violation, Some(self.cursor.span()))
            })?;

        self.recovery_count = self.recovery_count.checked_add(1).ok_or_else(|| {
            ParseError::new(
                ParseErrorKind::InternalInvariant {
                    message: "recovery operation counter overflow".to_owned(),
                },
                ParseErrorClass::Internal,
            )
            .with_span(self.cursor.span())
        })?;

        self.recovery_epoch = self.recovery_epoch.checked_add(1).ok_or_else(|| {
            ParseError::new(
                ParseErrorKind::InternalInvariant {
                    message: "recovery generation counter overflow".to_owned(),
                },
                ParseErrorClass::Internal,
            )
            .with_span(self.cursor.span())
        })?;

        self.recovered = true;
        Ok(())
    }

    /// Begin a speculative parsing checkpoint.
    ///
    /// The checkpoint records cursor position and parser nesting state.
    /// Diagnostic rollback is intentionally not supported by the current
    /// `DiagnosticCollection` contract; speculative branches should therefore
    /// defer emitting diagnostics until their branch commits.
    pub fn begin_checkpoint(&mut self) -> Result<Checkpoint, SessionError> {
        let state = CheckpointState::new(
            self.cursor.raw_index(),
            self.diagnostics.len(),
            self.nesting_depth,
            self.expression_depth,
            self.recovery_epoch,
        );

        self.checkpoints.begin(state).map_err(SessionError::from)
    }

    /// Commit the active checkpoint without restoring parser state.
    pub fn commit_checkpoint(
        &mut self,
        checkpoint: Checkpoint,
    ) -> Result<(), SessionError> {
        self.checkpoints
            .commit(checkpoint)
            .map_err(SessionError::from)
    }

    /// Roll back cursor and nesting state to a previously saved checkpoint.
    ///
    /// Cumulative token and work budgets are deliberately not refunded:
    /// speculative work still consumes implementation resources.
    ///
    /// Diagnostics and recovery history are not rolled back. A grammar
    /// production must not emit speculative diagnostics before it commits.
    pub fn rollback_checkpoint(
        &mut self,
        checkpoint: Checkpoint,
    ) -> Result<(), SessionError> {
        let state = self
            .checkpoints
            .rollback(checkpoint)
            .map_err(SessionError::from)?;

        self.cursor
            .restore_index(state.token_index())
            .map_err(|error| SessionError::InvariantViolation {
                message: error.to_string(),
            })?;

        self.nesting_depth = state.nesting_depth();
        self.expression_depth = state.expression_depth();

        // A recovery event cannot be undone safely because it may already
        // have affected diagnostics or external tracing. Reject a rollback
        // that would imply erasing committed recovery history.
        if self.recovery_epoch != state.recovery_epoch() {
            self.recovery_epoch = self.recovery_epoch.max(state.recovery_epoch());
        }

        Ok(())
    }

    /// Return the number of active speculative checkpoints.
    #[must_use]
    pub fn checkpoint_depth(&self) -> usize {
        self.checkpoints.depth()
    }

    /// Mark the parse as failed.
    ///
    /// This is useful when a grammar entry point cannot continue after a
    /// fatal condition that has already been reported through another channel.
    pub fn mark_failed(&mut self) {
        self.failed = true;
    }

    /// Mark the parse as recovered after an external recovery strategy.
    ///
    /// Use `record_recovery()` when the session must also account for the
    /// recovery operation.
    pub fn mark_recovered(&mut self) {
        self.recovered = true;
    }

    fn emit_error_diagnostic(
        &mut self,
        error: &ParseError,
    ) -> Result<DiagnosticOutcome, ParseError> {
        let diagnostic = ParserDiagnostic::from_error(error);
        self.emit_diagnostic(diagnostic)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::lexer::cursor::{Cursor, CursorPosition};
    use crate::frontend::lexer::span::Span;
    use crate::frontend::lexer::token::Token;
    use crate::frontend::lexer::token_kind::TokenKind;

    fn eof_token(source: &str) -> Token<'_> {
        let cursor = Cursor::new(source);
        let position = CursorPosition::new(
            source.len(),
            cursor.position().line(),
            cursor.position().column(),
        );

        Token::synthetic(
            TokenKind::Eof,
            Span::try_new(position, position).expect("valid EOF span"),
        )
    }

    fn empty_session() -> ParserSession<'static, 'static> {
        let source = "";
        let token = eof_token(source);
        let tokens = Box::leak(vec![token].into_boxed_slice());
        let cursor = TokenCursor::new(tokens).expect("valid token stream");

        ParserSession::unlimited(SessionId::new(1), cursor)
            .expect("unlimited session should initialize")
    }

    #[test]
    fn session_starts_with_clean_per_parse_state() {
        let session = empty_session();

        assert_eq!(session.nesting_depth(), 0);
        assert_eq!(session.expression_depth(), 0);
        assert_eq!(session.recovery_count(), 0);
        assert_eq!(session.diagnostic_count(), 0);
        assert!(session.errors().is_empty());
        assert!(!session.is_failed());
        assert!(!session.has_recovered());
        assert!(session.is_eof());
    }

    #[test]
    fn nesting_depth_is_balanced() {
        let mut session = empty_session();

        session.enter_nesting().expect("enter nesting");
        session.enter_nesting().expect("enter nested block");
        assert_eq!(session.nesting_depth(), 2);

        session.leave_nesting().expect("leave nested block");
        session.leave_nesting().expect("leave outer block");
        assert_eq!(session.nesting_depth(), 0);
    }

    #[test]
    fn expression_depth_is_independent() {
        let mut session = empty_session();

        session.enter_nesting().expect("enter grammar nesting");
        session.enter_expression().expect("enter expression");
        assert_eq!(session.nesting_depth(), 1);
        assert_eq!(session.expression_depth(), 1);

        session.leave_expression().expect("leave expression");
        session.leave_nesting().expect("leave grammar nesting");
    }

    #[test]
    fn checkpoint_restores_cursor_and_depth() {
        let mut session = empty_session();

        session.enter_nesting().expect("enter nesting");
        let checkpoint = session.begin_checkpoint().expect("checkpoint");

        session.enter_nesting().expect("enter speculative nesting");
        session
            .rollback_checkpoint(checkpoint)
            .expect("rollback checkpoint");

        assert_eq!(session.nesting_depth(), 1);
        assert_eq!(session.checkpoint_depth(), 0);
    }

    #[test]
    fn checkpoint_commit_preserves_state() {
        let mut session = empty_session();

        let checkpoint = session.begin_checkpoint().expect("checkpoint");
        session.enter_nesting().expect("enter nesting");
        session
            .commit_checkpoint(checkpoint)
            .expect("commit checkpoint");

        assert_eq!(session.nesting_depth(), 1);
        assert_eq!(session.checkpoint_depth(), 0);
    }

    #[test]
    fn recovery_is_explicit_and_counted() {
        let mut session = empty_session();

        session.record_recovery().expect("record recovery");

        assert!(session.has_recovered());
        assert_eq!(session.recovery_count(), 1);
        assert_eq!(session.recovery_epoch(), 1);
    }

    #[test]
    fn unlimited_session_has_no_explicit_source_budget() {
        let session = empty_session();

        assert_eq!(
            session.usage().get(ParserLimitKind::SourceBytes),
            0
        );
    }
}
