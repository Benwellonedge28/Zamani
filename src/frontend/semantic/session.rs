
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Per-invocation semantic-analysis session.
//!
//! File: `src/frontend/semantic/session.rs`
//! Edition: Rust 2021
//! Minimum Rust version: Rust 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Own the mutable state of one semantic-analysis invocation while sharing
//! immutable semantic services through a context.
//!
//! This module provides:
//!
//! - isolated session state;
//! - deterministic FIFO work scheduling;
//! - checked work accounting;
//! - configurable operational budgets;
//! - explicit scope-depth accounting;
//! - structured diagnostic retention;
//! - explicit completion, incompleteness, and abortion states;
//! - fallible reservation for session-owned collections;
//! - an immutable context and policy boundary.
//!
//! # Architectural boundaries
//!
//! This module does not:
//!
//! - define or copy the canonical AST;
//! - tokenize or parse source code;
//! - define a second symbol table or semantic type system;
//! - define domain-specific diagnostic records;
//! - decide whether a source construct is semantically valid;
//! - define quantum operations or a competing quantum IR;
//! - select devices, allocate hardware, route qubits, schedule execution,
//!   or lower programs to a target;
//! - use global mutable state or unsafe Rust.
//!
//! # Integration contract
//!
//! `context.rs` supplies an immutable context that implements
//! [`SemanticSessionContext`].
//!
//! `config.rs` owns the actual semantic configuration and operational-budget
//! policy. The context exposes an immutable [`SessionPolicySnapshot`] to
//! this module; it does not delegate policy ownership to the session.
//!
//! `diagnostics/diagnostic.rs` owns the canonical semantic diagnostic type.
//! The analyzer should instantiate `SemanticSession` with that type.
//!
//! `symbols/` owns symbol records and name-resolution data. This session
//! tracks scope depth only; it does not duplicate symbol storage.
//!
//! `pipeline.rs` and the semantic pass implementations schedule work, record
//! progress, retain diagnostics, and finalize the session.
//!
//! `api.rs` owns public analyzer construction and conversion of the session's
//! outcome into the public semantic-analysis result.
//!
//! # Completion invariant
//!
//! A completed session means that its orchestration finished without a
//! session-level interruption or budget exhaustion. It does NOT mean that
//! the source program is semantically valid.
//!
//! Semantic validity is determined by the completed semantic result,
//! diagnostics, unresolved obligations, and final validation passes.
//!
//! A session that exhausts a configured budget becomes incomplete. It must
//! never be converted into a successful semantic result merely because
//! remaining work was skipped.
//!
//! # Scalability
//!
//! No language-level maximum is imposed on qubits, processors, devices,
//! tensor dimensions, declarations, or computational domains.
//!
//! Optional session budgets limit compiler work, retained diagnostics, or
//! tracked nesting depth for an individual analysis invocation. They are
//! operational safeguards, not language or hardware-capacity limits.
//!
//! An unlimited policy means no explicit budget is configured; it does not
//! promise infinite memory, stack, address space, or execution time.
//!
//! # Determinism
//!
//! Work items are removed in FIFO order. This module never iterates over a
//! hash map to determine work order and never uses wall-clock time or
//! randomness to decide session behavior.
//!
//! The caller is responsible for submitting work in the specified,
//! deterministic order and for ordering diagnostics before final output when
//! diagnostics originate from independent analysis passes.

use core::fmt;
use core::num::{NonZeroU64, NonZeroUsize};
use std::collections::VecDeque;
use std::sync::Arc;

/// Immutable operational limits for one semantic-analysis session.
///
/// `None` means that no explicit limit is configured for that dimension.
/// These limits govern compiler work, not the resources a Zamani program
/// describes or the capacities of its eventual execution target.
///
/// The authoritative configuration module should construct this snapshot
/// from the validated semantic configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SessionPolicySnapshot {
    /// Maximum cumulative work units recorded by the session.
    ///
    /// The semantic pipeline defines what constitutes one work unit and must
    /// account for expensive passes consistently.
    pub max_work_units: Option<NonZeroU64>,

    /// Maximum number of retained diagnostics.
    ///
    /// When this limit is reached, further diagnostic retention is refused
    /// and the session is marked incomplete.
    pub max_diagnostics: Option<NonZeroUsize>,

    /// Maximum tracked semantic scope nesting depth.
    ///
    /// The root scope is depth zero. Entering a nested scope increases depth
    /// by one. This limit does not automatically protect recursion in code
    /// paths that do not use the session's scope-accounting methods.
    pub max_scope_depth: Option<NonZeroUsize>,
}

impl SessionPolicySnapshot {
    /// Return a policy with no explicit session budgets.
    ///
    /// This does not disable representational overflow checks or guarantee
    /// recovery from process-wide memory exhaustion.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_work_units: None,
            max_diagnostics: None,
            max_scope_depth: None,
        }
    }

    /// Return whether the snapshot contains no explicit limits.
    #[must_use]
    pub const fn is_unlimited(self) -> bool {
        self.max_work_units.is_none()
            && self.max_diagnostics.is_none()
            && self.max_scope_depth.is_none()
    }
}

/// Contract for the immutable context shared with a semantic session.
///
/// The production `SemanticContext` must implement this trait and return a
/// validated policy snapshot derived from its immutable configuration.
///
/// Implementations must not change the returned policy during a session.
pub trait SemanticSessionContext {
    /// Return the immutable operational policy for this analysis.
    fn session_policy(&self) -> SessionPolicySnapshot;
}

/// Lifecycle state of a semantic-analysis session.
///
/// `Completed` describes orchestration completion only; it does not certify
/// that the analyzed program is semantically valid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SessionStatus {
    /// Analysis work may still be scheduled or processed.
    #[default]
    Running,

    /// The orchestration finished without a session-level interruption.
    Completed,

    /// Analysis could not finish all required work.
    ///
    /// Examples include an exhausted budget, diagnostic truncation, or a
    /// required session operation that could not complete.
    Incomplete,

    /// The caller deliberately stopped the analysis.
    Aborted,
}

impl SessionStatus {
    /// Return whether the session is still accepting analysis work.
    #[must_use]
    pub const fn is_running(self) -> bool {
        matches!(self, Self::Running)
    }

    /// Return whether all scheduled session work may be considered complete.
    ///
    /// This is not a statement about semantic validity.
    #[must_use]
    pub const fn orchestration_completed(self) -> bool {
        matches!(self, Self::Completed)
    }
}

/// Kind of operational budget exhausted by a session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SessionLimitKind {
    /// The cumulative work counter reached its configured limit.
    WorkUnits,

    /// The configured number of retained diagnostics was reached.
    Diagnostics,

    /// The next scope would exceed the configured nesting depth.
    ScopeDepth,
}

impl fmt::Display for SessionLimitKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WorkUnits => f.write_str("semantic work-unit budget"),
            Self::Diagnostics => f.write_str("semantic diagnostic budget"),
            Self::ScopeDepth => f.write_str("semantic scope-depth budget"),
        }
    }
}

/// Errors returned by session state management.
///
/// These are implementation or orchestration errors, not source-language
/// constructs. The analyzer API should translate them into its canonical
/// diagnostic/result contract where appropriate.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SessionError {
    /// The session is no longer accepting work.
    SessionNotRunning {
        status: SessionStatus,
    },

    /// A configured operational budget prevented an operation.
    LimitExceeded {
        kind: SessionLimitKind,
    },

    /// A counter cannot be incremented without overflowing its representation.
    CounterOverflow {
        counter: &'static str,
    },

    /// A scope cannot be exited because no scope is active.
    ScopeUnderflow,

    /// The session-owned collection could not reserve additional storage.
    AllocationFailed,

    /// Completion was requested while analysis remained incomplete or aborted.
    CannotComplete {
        status: SessionStatus,
    },
}

impl fmt::Display for SessionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SessionNotRunning { status } => {
                write!(f, "semantic session is not running ({status:?})")
            }
            Self::LimitExceeded { kind } => {
                write!(f, "{kind} exceeded")
            }
            Self::CounterOverflow { counter } => {
                write!(f, "semantic session counter overflow: {counter}")
            }
            Self::ScopeUnderflow => {
                f.write_str("cannot exit a semantic scope when no scope is active")
            }
            Self::AllocationFailed => {
                f.write_str("unable to reserve semantic session storage")
            }
            Self::CannotComplete { status } => {
                write!(f, "cannot complete semantic session in state {status:?}")
            }
        }
    }
}

impl std::error::Error for SessionError {}

/// Immutable snapshot of the session's operational accounting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SessionUsage {
    /// Number of work units recorded so far.
    pub work_units: u64,

    /// Number of diagnostics successfully retained.
    pub diagnostics_retained: usize,

    /// Current tracked scope nesting depth.
    pub scope_depth: usize,

    /// Whether diagnostic retention was truncated by a configured budget.
    pub diagnostics_truncated: bool,
}

/// Mutable state for one semantic-analysis invocation.
///
/// `C` is the immutable context type. `W` is a pass-defined work item, and
/// `D` is the canonical diagnostic type. The session deliberately does not
/// require a particular AST node type, diagnostic representation, or symbol
/// implementation.
///
/// Create a separate session for each analysis invocation. Sharing the
/// immutable context between sessions is supported; sharing a mutable session
/// between concurrent analyses is not.
pub struct SemanticSession<C, W, D>
where
    C: SemanticSessionContext,
{
    context: Arc<C>,
    policy: SessionPolicySnapshot,
    work_queue: VecDeque<W>,
    diagnostics: Vec<D>,
    usage: SessionUsage,
    status: SessionStatus,
}

impl<C, W, D> fmt::Debug for SemanticSession<C, W, D>
where
    C: SemanticSessionContext,
    W: fmt::Debug,
    D: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SemanticSession")
            .field("policy", &self.policy)
            .field("queued_work_items", &self.work_queue.len())
            .field("usage", &self.usage)
            .field("status", &self.status)
            .finish_non_exhaustive()
    }
}

impl<C, W, D> SemanticSession<C, W, D>
where
    C: SemanticSessionContext,
{
    /// Create a fresh session using a shared immutable context.
    ///
    /// The policy is snapshotted once, so later activity in another session
    /// cannot change this session's operational limits.
    ///
    /// The caller should create the context only after its configuration and
    /// extension registry have been validated.
    pub fn new(context: Arc<C>) -> Result<Self, SessionError> {
        let policy = context.session_policy();

        let mut work_queue = VecDeque::new();
        work_queue
            .try_reserve(0)
            .map_err(|_| SessionError::AllocationFailed)?;

        let mut diagnostics = Vec::new();
        diagnostics
            .try_reserve(0)
            .map_err(|_| SessionError::AllocationFailed)?;

        Ok(Self {
            context,
            policy,
            work_queue,
            diagnostics,
            usage: SessionUsage::default(),
            status: SessionStatus::Running,
        })
    }

    /// Borrow the immutable context.
    #[must_use]
    pub fn context(&self) -> &C {
        self.context.as_ref()
    }

    /// Clone the shared context handle.
    ///
    /// This clones only the `Arc`; it does not clone or mutate the context.
    #[must_use]
    pub fn shared_context(&self) -> Arc<C> {
        Arc::clone(&self.context)
    }

    /// Return the immutable policy snapshot used by this session.
    #[must_use]
    pub const fn policy(&self) -> SessionPolicySnapshot {
        self.policy
    }

    /// Return the current lifecycle status.
    #[must_use]
    pub const fn status(&self) -> SessionStatus {
        self.status
    }

    /// Return the current operational-usage snapshot.
    #[must_use]
    pub const fn usage(&self) -> SessionUsage {
        self.usage
    }

    /// Borrow the retained diagnostics in insertion order.
    ///
    /// The pipeline is responsible for applying the canonical diagnostic
    /// ordering policy before exposing final results.
    #[must_use]
    pub fn diagnostics(&self) -> &[D] {
        &self.diagnostics
    }

    /// Borrow the pending work queue in FIFO order.
    ///
    /// This is intended for debugging and orchestration inspection. Passes
    /// should normally use `schedule_work` and `next_work`.
    #[must_use]
    pub fn pending_work(&self) -> &VecDeque<W> {
        &self.work_queue
    }

    /// Return whether this session has no pending work.
    ///
    /// An empty queue alone does not prove that analysis is complete: a
    /// pipeline may still need to run finalization or validation passes.
    #[must_use]
    pub fn is_work_queue_empty(&self) -> bool {
        self.work_queue.is_empty()
    }

    /// Record one unit of semantic-analysis work.
    ///
    /// Callers must define work units consistently for each pass. A failed
    /// budget check marks the session incomplete and does not increment the
    /// counter.
    pub fn record_work(&mut self) -> Result<(), SessionError> {
        self.ensure_running()?;

        let next = match self.usage.work_units.checked_add(1) {
            Some(value) => value,
            None => {
                self.mark_incomplete();
                return Err(SessionError::CounterOverflow {
                    counter: "work_units",
                });
            }
        };

        if self
            .policy
            .max_work_units
            .is_some_and(|limit| next > limit.get())
        {
            self.mark_incomplete();
            return Err(SessionError::LimitExceeded {
                kind: SessionLimitKind::WorkUnits,
            });
        }

        self.usage.work_units = next;
        Ok(())
    }

    /// Schedule work at the end of the FIFO queue.
    ///
    /// Scheduling consumes one work unit. If either the work budget or
    /// collection reservation fails, the session becomes incomplete so the
    /// caller cannot mistake omitted work for completed analysis.
    pub fn schedule_work(&mut self, item: W) -> Result<(), SessionError> {
        self.ensure_running()?;
        self.record_work()?;

        if self.work_queue.try_reserve(1).is_err() {
            self.mark_incomplete();
            return Err(SessionError::AllocationFailed);
        }

        self.work_queue.push_back(item);
        Ok(())
    }

    /// Remove and return the next work item in FIFO order.
    ///
    /// Returning `None` means only that the queue is empty. It does not
    /// transition the session to `Completed`.
    pub fn next_work(&mut self) -> Result<Option<W>, SessionError> {
        self.ensure_running()?;
        Ok(self.work_queue.pop_front())
    }

    /// Increase tracked semantic scope depth.
    ///
    /// Scope contents and symbol lookup remain owned by `symbols/`. This
    /// method tracks nesting and enforces the configured operational budget.
    pub fn enter_scope(&mut self) -> Result<(), SessionError> {
        self.ensure_running()?;

        let next = match self.usage.scope_depth.checked_add(1) {
            Some(value) => value,
            None => {
                self.mark_incomplete();
                return Err(SessionError::CounterOverflow {
                    counter: "scope_depth",
                });
            }
        };

        if self
            .policy
            .max_scope_depth
            .is_some_and(|limit| next > limit.get())
        {
            self.mark_incomplete();
            return Err(SessionError::LimitExceeded {
                kind: SessionLimitKind::ScopeDepth,
            });
        }

        self.usage.scope_depth = next;
        Ok(())
    }

    /// Decrease tracked semantic scope depth.
    ///
    /// Calling this method at depth zero is an orchestration error. It does
    /// not silently saturate the counter or conceal an unbalanced scope.
    pub fn exit_scope(&mut self) -> Result<(), SessionError> {
        self.ensure_running()?;

        if self.usage.scope_depth == 0 {
            return Err(SessionError::ScopeUnderflow);
        }

        self.usage.scope_depth -= 1;
        Ok(())
    }

    /// Retain a diagnostic in insertion order.
    ///
    /// Returns `Ok(())` when the diagnostic is retained. If the configured
    /// diagnostic budget has already been reached, the diagnostic is not
    /// retained, `diagnostics_truncated` is set, and the session becomes
    /// incomplete.
    ///
    /// Diagnostic ordering and deduplication remain the responsibility of
    /// `diagnostics/ordering.rs` and the pipeline.
    pub fn emit_diagnostic(&mut self, diagnostic: D) -> Result<(), SessionError> {
        self.ensure_running()?;

        if self
            .policy
            .max_diagnostics
            .is_some_and(|limit| self.diagnostics.len() >= limit.get())
        {
            self.usage.diagnostics_truncated = true;
            self.mark_incomplete();

            return Err(SessionError::LimitExceeded {
                kind: SessionLimitKind::Diagnostics,
            });
        }

        if self.diagnostics.try_reserve(1).is_err() {
            self.mark_incomplete();
            return Err(SessionError::AllocationFailed);
        }

        self.diagnostics.push(diagnostic);
        self.usage.diagnostics_retained = self.diagnostics.len();
        Ok(())
    }

    /// Mark the session incomplete because required analysis could not finish.
    ///
    /// This transition is monotonic: a later call cannot restore `Running`
    /// or `Completed`.
    pub fn mark_incomplete(&mut self) {
        if self.status == SessionStatus::Running {
            self.status = SessionStatus::Incomplete;
        }
    }

    /// Deliberately abort the analysis.
    ///
    /// Abortion is distinct from a semantic error and from successful
    /// completion. The analyzer API must expose this outcome explicitly.
    pub fn abort(&mut self) {
        if self.status == SessionStatus::Running {
            self.status = SessionStatus::Aborted;
        }
    }

    /// Mark orchestration complete if the session is still running and no
    /// scheduled work remains.
    ///
    /// This does not certify semantic validity. Final semantic validation and
    /// the public result constructor must separately establish that the
    /// program has no unresolved required obligations.
    pub fn complete(&mut self) -> Result<(), SessionError> {
        self.ensure_running()?;

        if !self.work_queue.is_empty() {
            return Err(SessionError::CannotComplete {
                status: self.status,
            });
        }

        self.status = SessionStatus::Completed;
        Ok(())
    }

    fn ensure_running(&self) -> Result<(), SessionError> {
        if self.status.is_running() {
            Ok(())
        } else {
            Err(SessionError::SessionNotRunning {
                status: self.status,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestContext {
        policy: SessionPolicySnapshot,
    }

    impl SemanticSessionContext for TestContext {
        fn session_policy(&self) -> SessionPolicySnapshot {
            self.policy
        }
    }

    fn context(policy: SessionPolicySnapshot) -> Arc<TestContext> {
        Arc::new(TestContext { policy })
    }

    #[test]
    fn new_session_starts_running_with_empty_state() {
        let session =
            SemanticSession::<_, u32, String>::new(context(SessionPolicySnapshot::unlimited()))
                .expect("session construction should succeed");

        assert_eq!(session.status(), SessionStatus::Running);
        assert_eq!(session.usage(), SessionUsage::default());
        assert!(session.is_work_queue_empty());
        assert!(session.diagnostics().is_empty());
    }

    #[test]
    fn work_queue_is_fifo() {
        let mut session =
            SemanticSession::<_, u32, String>::new(context(SessionPolicySnapshot::unlimited()))
                .expect("session construction should succeed");

        session.schedule_work(10).expect("schedule first item");
        session.schedule_work(20).expect("schedule second item");

        assert_eq!(session.next_work().expect("pop first item"), Some(10));
        assert_eq!(session.next_work().expect("pop second item"), Some(20));
        assert_eq!(session.next_work().expect("empty queue"), None);
        assert_eq!(session.usage().work_units, 2);
    }

    #[test]
    fn work_budget_marks_session_incomplete() {
        let policy = SessionPolicySnapshot {
            max_work_units: NonZeroU64::new(1),
            ..SessionPolicySnapshot::unlimited()
        };

        let mut session =
            SemanticSession::<_, u32, String>::new(context(policy))
                .expect("session construction should succeed");

        session.schedule_work(1).expect("first work item");
        let error = session.schedule_work(2).expect_err("second item exceeds budget");

        assert_eq!(
            error,
            SessionError::LimitExceeded {
                kind: SessionLimitKind::WorkUnits
            }
        );
        assert_eq!(session.status(), SessionStatus::Incomplete);
        assert_eq!(session.usage().work_units, 1);
    }

    #[test]
    fn diagnostic_budget_is_explicit_and_never_silent() {
        let policy = SessionPolicySnapshot {
            max_diagnostics: NonZeroUsize::new(1),
            ..SessionPolicySnapshot::unlimited()
        };

        let mut session =
            SemanticSession::<_, (), String>::new(context(policy))
                .expect("session construction should succeed");

        session
            .emit_diagnostic("first".to_owned())
            .expect("first diagnostic should be retained");

        let error = session
            .emit_diagnostic("second".to_owned())
            .expect_err("second diagnostic exceeds budget");

        assert_eq!(
            error,
            SessionError::LimitExceeded {
                kind: SessionLimitKind::Diagnostics
            }
        );
        assert_eq!(session.diagnostics(), &["first".to_owned()]);
        assert!(session.usage().diagnostics_truncated);
        assert_eq!(session.status(), SessionStatus::Incomplete);
    }

    #[test]
    fn scope_depth_is_checked_without_hardware_assumptions() {
        let policy = SessionPolicySnapshot {
            max_scope_depth: NonZeroUsize::new(1),
            ..SessionPolicySnapshot::unlimited()
        };

        let mut session =
            SemanticSession::<_, (), String>::new(context(policy))
                .expect("session construction should succeed");

        session.enter_scope().expect("first nested scope");
        assert_eq!(session.usage().scope_depth, 1);

        let error = session.enter_scope().expect_err("depth budget exceeded");
        assert_eq!(
            error,
            SessionError::LimitExceeded {
                kind: SessionLimitKind::ScopeDepth
            }
        );
        assert_eq!(session.status(), SessionStatus::Incomplete);
    }

    #[test]
    fn unbalanced_scope_exit_is_reported() {
        let mut session =
            SemanticSession::<_, (), String>::new(context(SessionPolicySnapshot::unlimited()))
                .expect("session construction should succeed");

        assert_eq!(session.exit_scope(), Err(SessionError::ScopeUnderflow));
        assert_eq!(session.usage().scope_depth, 0);
    }

    #[test]
    fn completion_requires_an_empty_work_queue() {
        let mut session =
            SemanticSession::<_, u32, String>::new(context(SessionPolicySnapshot::unlimited()))
                .expect("session construction should succeed");

        session.schedule_work(7).expect("schedule work");

        assert_eq!(
            session.complete(),
            Err(SessionError::CannotComplete {
                status: SessionStatus::Running
            })
        );

        assert_eq!(session.status(), SessionStatus::Running);

        let _ = session.next_work().expect("remove pending work");
        session.complete().expect("complete empty session");

        assert_eq!(session.status(), SessionStatus::Completed);
    }

    #[test]
    fn incomplete_session_cannot_be_completed() {
        let policy = SessionPolicySnapshot {
            max_work_units: NonZeroU64::new(1),
            ..SessionPolicySnapshot::unlimited()
        };

        let mut session =
            SemanticSession::<_, u32, String>::new(context(policy))
                .expect("session construction should succeed");

        session.schedule_work(1).expect("first item");
        let _ = session.schedule_work(2);

        assert_eq!(
            session.complete(),
            Err(SessionError::SessionNotRunning {
                status: SessionStatus::Incomplete
            })
        );
        assert_eq!(session.status(), SessionStatus::Incomplete);
    }

    #[test]
    fn abort_is_terminal() {
        let mut session =
            SemanticSession::<_, (), String>::new(context(SessionPolicySnapshot::unlimited()))
                .expect("session construction should succeed");

        session.abort();

        assert_eq!(session.status(), SessionStatus::Aborted);
        assert!(matches!(
            session.schedule_work(()),
            Err(SessionError::SessionNotRunning {
                status: SessionStatus::Aborted
            })
        ));
    }

    #[test]
    fn independent_sessions_share_context_but_not_mutable_state() {
        let shared = context(SessionPolicySnapshot::unlimited());

        let mut first =
            SemanticSession::<_, u32, String>::new(Arc::clone(&shared))
                .expect("first session");
        let second =
            SemanticSession::<_, u32, String>::new(Arc::clone(&shared))
                .expect("second session");

        first.schedule_work(99).expect("schedule first session");

        assert_eq!(first.pending_work().len(), 1);
        assert_eq!(second.pending_work().len(), 0);
        assert_eq!(first.context().session_policy(), second.context().session_policy());
    }
}
