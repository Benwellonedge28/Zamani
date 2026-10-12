
#![forbid(unsafe_code)]

//! Transactional checkpoints for speculative Zamani parsing.
//!
//! # Purpose
//!
//! Save and restore the parser state needed when a grammar production must
//! tentatively parse an ambiguous construct and either commit or roll back.
//!
//! # Responsibilities
//!
//! This module owns:
//! - immutable snapshots of parser bookkeeping state;
//! - opaque checkpoint handles scoped to a parser session;
//! - nested, last-in-first-out checkpoint management;
//! - explicit commit and rollback operations;
//! - validation of stale, foreign, and out-of-order handles;
//! - checked checkpoint sequencing and fallible stack growth.
//!
//! # Does not own
//!
//! This module does not own:
//! - source scanning or lexical cursor checkpoints;
//! - token storage, token classification, or token lookahead;
//! - parser diagnostics or their actual storage;
//! - grammar productions or speculative parsing policy;
//! - AST construction or semantic analysis;
//! - type, effect, capability, or resource validation;
//! - hardware discovery, allocation, scheduling, or execution.
//!
//! # Integration contract
//!
//! - `token_cursor.rs` supplies the current token index.
//! - `session.rs` supplies a stable session identifier and captures the
//!   current parser bookkeeping state before speculation.
//! - `diagnostics.rs` owns diagnostic storage; rollback consumers truncate
//!   speculative diagnostics to the saved diagnostic count when appropriate.
//! - Grammar entry points use `begin`, then exactly one of `commit` or
//!   `rollback` for each active checkpoint.
//! - Nested checkpoints must be closed in last-in-first-out order.
//! - `recovery.rs` decides which diagnostic and recovery effects should
//!   survive a failed speculative branch.
//! - `error.rs` may wrap `CheckpointError` at the parser API boundary.
//!
//! The checkpoint manager does not mutate the parser session. `rollback`
//! returns the snapshot; the caller is responsible for restoring the
//! cursor and any state it owns.
//!
//! # Resource and scalability policy
//!
//! No hardware capacity or language-level size limit is defined here.
//! Checkpoint storage grows as required by active speculation and available
//! resources. Stack reservation is fallible and reports allocation-capacity
//! errors where Rust's allocator interface permits them to be reported.
//! The caller remains responsible for applying its configured parser budgets.
//!
//! A checkpoint restores parser bookkeeping, not arbitrary side effects.
//! Speculative grammar handlers must not perform irreversible external work.
//!
//! # Compatibility
//!
//! Public enums are non-exhaustive where future evolution is expected.
//! Checkpoint handles are opaque; consumers must use the accessors rather
//! than depend on their internal representation.

use core::fmt;

/// A snapshot of parser bookkeeping required for rollback.
///
/// `token_index` is the position in the parser's token stream, not a source
/// byte offset. `diagnostic_count` is the number of diagnostics retained
/// before speculation began.
///
/// Depth values are snapshots, not limits. No domain-specific quantity is
/// represented here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CheckpointState {
    token_index: usize,
    diagnostic_count: usize,
    nesting_depth: usize,
    expression_depth: usize,
    recovery_epoch: u64,
}

impl CheckpointState {
    /// Create a parser-state snapshot.
    ///
    /// The caller is responsible for passing values obtained from the same
    /// parser session at the same logical instant.
    #[must_use]
    pub const fn new(
        token_index: usize,
        diagnostic_count: usize,
        nesting_depth: usize,
        expression_depth: usize,
        recovery_epoch: u64,
    ) -> Self {
        Self {
            token_index,
            diagnostic_count,
            nesting_depth,
            expression_depth,
            recovery_epoch,
        }
    }

    /// Current token-stream index.
    #[must_use]
    pub const fn token_index(self) -> usize {
        self.token_index
    }

    /// Diagnostic count before speculation.
    #[must_use]
    pub const fn diagnostic_count(self) -> usize {
        self.diagnostic_count
    }

    /// General grammar nesting depth before speculation.
    #[must_use]
    pub const fn nesting_depth(self) -> usize {
        self.nesting_depth
    }

    /// Expression nesting depth before speculation.
    #[must_use]
    pub const fn expression_depth(self) -> usize {
        self.expression_depth
    }

    /// Caller-maintained recovery generation at checkpoint creation.
    ///
    /// This is an opaque value. The checkpoint manager does not interpret
    /// it or mutate recovery state.
    #[must_use]
    pub const fn recovery_epoch(self) -> u64 {
        self.recovery_epoch
    }
}

/// An opaque handle identifying one active parser checkpoint.
///
/// Handles are scoped to the session identifier supplied to
/// [`CheckpointManager::new`]. A handle is valid only while it is the
/// manager's active top checkpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Checkpoint {
    session_id: u64,
    sequence: u64,
    state: CheckpointState,
}

impl Checkpoint {
    /// Session identifier associated with this checkpoint.
    #[must_use]
    pub const fn session_id(self) -> u64 {
        self.session_id
    }

    /// Monotonically increasing checkpoint sequence within this manager.
    #[must_use]
    pub const fn sequence(self) -> u64 {
        self.sequence
    }

    /// State snapshot captured when the checkpoint was opened.
    #[must_use]
    pub const fn state(self) -> CheckpointState {
        self.state
    }
}

/// Errors from checkpoint creation, commit, and rollback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CheckpointError {
    /// The supplied handle belongs to another parser session.
    ForeignSession {
        expected_session: u64,
        actual_session: u64,
    },

    /// The handle is not the currently active top checkpoint.
    ///
    /// This includes attempting to close a checkpoint that was already
    /// committed or rolled back, or closing an outer checkpoint before
    /// closing its inner checkpoint.
    NotActiveTop {
        sequence: u64,
    },

    /// No checkpoint is currently active.
    NoActiveCheckpoint,

    /// The sequence counter cannot represent another unique checkpoint.
    SequenceExhausted,

    /// The checkpoint stack could not reserve storage for a new entry.
    AllocationFailed,
}

impl fmt::Display for CheckpointError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ForeignSession {
                expected_session,
                actual_session,
            } => write!(
                formatter,
                "checkpoint belongs to session {actual_session}, \
                 but the active manager belongs to session {expected_session}"
            ),
            Self::NotActiveTop { sequence } => write!(
                formatter,
                "checkpoint {sequence} is not the active top checkpoint"
            ),
            Self::NoActiveCheckpoint => {
                formatter.write_str("no parser checkpoint is active")
            }
            Self::SequenceExhausted => {
                formatter.write_str("parser checkpoint sequence is exhausted")
            }
            Self::AllocationFailed => {
                formatter.write_str("unable to reserve parser checkpoint storage")
            }
        }
    }
}

impl std::error::Error for CheckpointError {}

/// Manages nested checkpoints for one parser session.
///
/// Create one manager per parse session. The session identifier must remain
/// stable for that session and must not be reused concurrently for another
/// session whose handles could cross manager boundaries.
///
/// The manager does not hold a mutable reference to the parser. The parser
/// session owns its state and applies the snapshot returned by `rollback`.
#[derive(Debug)]
pub struct CheckpointManager {
    session_id: u64,
    next_sequence: u64,
    stack: Vec<Checkpoint>,
}

impl CheckpointManager {
    /// Create a checkpoint manager for a parser session.
    ///
    /// The identifier is supplied by the session layer so this module does
    /// not need global mutable state, randomness, or process-wide counters.
    #[must_use]
    pub const fn new(session_id: u64) -> Self {
        Self {
            session_id,
            next_sequence: 0,
            stack: Vec::new(),
        }
    }

    /// Session identifier owned by this manager.
    #[must_use]
    pub const fn session_id(&self) -> u64 {
        self.session_id
    }

    /// Number of currently active nested checkpoints.
    #[must_use]
    pub fn depth(&self) -> usize {
        self.stack.len()
    }

    /// Whether any checkpoint is currently active.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    /// Return the active top checkpoint, if any.
    #[must_use]
    pub fn top(&self) -> Option<Checkpoint> {
        self.stack.last().copied()
    }

    /// Open a checkpoint at the supplied parser-state snapshot.
    ///
    /// This method does not capture parser state itself. The session must
    /// capture a consistent snapshot and pass it in.
    ///
    /// A successful call pushes a new checkpoint. On failure, the active
    /// checkpoint stack and sequence counter remain unchanged.
    pub fn begin(
        &mut self,
        state: CheckpointState,
    ) -> Result<Checkpoint, CheckpointError> {
        let sequence = self
            .next_sequence
            .checked_add(1)
            .ok_or(CheckpointError::SequenceExhausted)?;

        self.stack
            .try_reserve(1)
            .map_err(|_| CheckpointError::AllocationFailed)?;

        let checkpoint = Checkpoint {
            session_id: self.session_id,
            sequence,
            state,
        };

        self.stack.push(checkpoint);
        self.next_sequence = sequence;

        Ok(checkpoint)
    }

    /// Commit the active top checkpoint.
    ///
    /// Committing removes the checkpoint without restoring its snapshot.
    /// Any enclosing checkpoint remains active and can still roll back to
    /// its own earlier state.
    ///
    /// The caller is responsible for deciding which diagnostics and parser
    /// state changes should survive the committed branch.
    pub fn commit(
        &mut self,
        checkpoint: Checkpoint,
    ) -> Result<(), CheckpointError> {
        self.validate_top(checkpoint)?;
        self.stack.pop();
        Ok(())
    }

    /// Roll back the active top checkpoint and return its saved state.
    ///
    /// This method removes the checkpoint but does not mutate the token
    /// cursor, diagnostic sink, nesting counters, or recovery state. The
    /// caller must restore the returned snapshot to the appropriate
    /// session-owned components.
    pub fn rollback(
        &mut self,
        checkpoint: Checkpoint,
    ) -> Result<CheckpointState, CheckpointError> {
        self.validate_top(checkpoint)?;
        let removed = self.stack.pop();

        // `validate_top` established that the stack is non-empty and that
        // its last entry equals `checkpoint`. Keep this branch defensive
        // rather than relying on an unchecked unwrap.
        match removed {
            Some(active) => Ok(active.state),
            None => Err(CheckpointError::NoActiveCheckpoint),
        }
    }

    /// Discard the active top checkpoint without returning its snapshot.
    ///
    /// This is useful when a caller has already captured or restored the
    /// state through a higher-level transaction. Prefer `commit` or
    /// `rollback` when their semantics apply.
    pub fn discard(
        &mut self,
        checkpoint: Checkpoint,
    ) -> Result<(), CheckpointError> {
        self.validate_top(checkpoint)?;
        self.stack.pop();
        Ok(())
    }

    /// Remove all active checkpoints.
    ///
    /// Intended for terminating or resetting a parser session. This does
    /// not change parser state and does not reset the sequence counter, so
    /// previously issued handles cannot become valid again by sequence
    /// reuse within this manager.
    pub fn clear(&mut self) {
        self.stack.clear();
    }

    fn validate_top(
        &self,
        checkpoint: Checkpoint,
    ) -> Result<(), CheckpointError> {
        if checkpoint.session_id != self.session_id {
            return Err(CheckpointError::ForeignSession {
                expected_session: self.session_id,
                actual_session: checkpoint.session_id,
            });
        }

        match self.stack.last() {
            Some(active) if *active == checkpoint => Ok(()),
            Some(_) => Err(CheckpointError::NotActiveTop {
                sequence: checkpoint.sequence,
            }),
            None => Err(CheckpointError::NoActiveCheckpoint),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CheckpointError, CheckpointManager, CheckpointState,
    };

    fn state(token_index: usize) -> CheckpointState {
        CheckpointState::new(token_index, 2, 3, 1, 7)
    }

    #[test]
    fn begin_exposes_the_saved_state() {
        let mut manager = CheckpointManager::new(11);
        let snapshot = state(5);

        let checkpoint = manager
            .begin(snapshot)
            .expect("checkpoint creation should succeed");

        assert_eq!(checkpoint.session_id(), 11);
        assert_eq!(checkpoint.state(), snapshot);
        assert_eq!(manager.depth(), 1);
        assert_eq!(manager.top(), Some(checkpoint));
    }

    #[test]
    fn rollback_returns_snapshot_and_removes_checkpoint() {
        let mut manager = CheckpointManager::new(11);
        let snapshot = state(8);
        let checkpoint = manager.begin(snapshot).expect("begin");

        let restored = manager.rollback(checkpoint).expect("rollback");

        assert_eq!(restored, snapshot);
        assert!(manager.is_empty());
        assert_eq!(manager.depth(), 0);
    }

    #[test]
    fn commit_removes_checkpoint_without_restoring_state() {
        let mut manager = CheckpointManager::new(11);
        let checkpoint = manager.begin(state(3)).expect("begin");

        manager.commit(checkpoint).expect("commit");

        assert!(manager.is_empty());
    }

    #[test]
    fn nested_checkpoints_must_close_in_lifo_order() {
        let mut manager = CheckpointManager::new(11);
        let outer = manager.begin(state(1)).expect("outer");
        let inner = manager.begin(state(2)).expect("inner");

        assert_eq!(
            manager.rollback(outer),
            Err(CheckpointError::NotActiveTop {
                sequence: outer.sequence(),
            })
        );

        assert_eq!(manager.depth(), 2);
        assert_eq!(manager.top(), Some(inner));

        manager.commit(inner).expect("commit inner");
        assert_eq!(manager.rollback(outer), Ok(state(1)));
        assert!(manager.is_empty());
    }

    #[test]
    fn foreign_session_handle_is_rejected() {
        let mut owner = CheckpointManager::new(1);
        let mut other = CheckpointManager::new(2);
        let checkpoint = owner.begin(state(4)).expect("begin");

        assert_eq!(
            other.commit(checkpoint),
            Err(CheckpointError::ForeignSession {
                expected_session: 2,
                actual_session: 1,
            })
        );

        assert_eq!(owner.top(), Some(checkpoint));
    }

    #[test]
    fn closed_handles_cannot_be_reused() {
        let mut manager = CheckpointManager::new(4);
        let checkpoint = manager.begin(state(1)).expect("begin");

        manager.commit(checkpoint).expect("commit");

        assert_eq!(
            manager.rollback(checkpoint),
            Err(CheckpointError::NoActiveCheckpoint)
        );
    }

    #[test]
    fn clear_does_not_reset_sequence_numbers() {
        let mut manager = CheckpointManager::new(4);
        let first = manager.begin(state(1)).expect("first");
        manager.clear();

        let second = manager.begin(state(2)).expect("second");

        assert!(second.sequence() > first.sequence());
        assert_eq!(manager.depth(), 1);
    }

    #[test]
    fn state_accessors_preserve_each_field() {
        let snapshot = CheckpointState::new(13, 5, 4, 2, 99);

        assert_eq!(snapshot.token_index(), 13);
        assert_eq!(snapshot.diagnostic_count(), 5);
        assert_eq!(snapshot.nesting_depth(), 4);
        assert_eq!(snapshot.expression_depth(), 2);
        assert_eq!(snapshot.recovery_epoch(), 99);
    }

    #[test]
    fn manager_starts_without_active_checkpoints() {
        let manager = CheckpointManager::new(0);

        assert!(manager.is_empty());
        assert_eq!(manager.depth(), 0);
        assert_eq!(manager.top(), None);
    }
}
