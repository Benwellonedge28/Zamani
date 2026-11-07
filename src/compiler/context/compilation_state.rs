
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Compilation State.
//!
//! Path:
//!     src/compiler/context/compilation_state.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum version: 1.97
//!
//! Safety:
//!     Safe Rust only.
//!     Unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Owns the lifecycle state of one compilation operation.
//!
//! This module coordinates:
//!
//! - Compilation initialization.
//! - Compilation execution.
//! - High-level compilation phases.
//! - Dynamically named compilation stages.
//! - Cooperative cancellation.
//! - Successful completion.
//! - Failure recording.
//! - Terminal-state enforcement.
//! - Concurrent state observation.
//! - Deterministic lifecycle snapshots.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - CompilationLifecycle.
//! - CompilationPhase.
//! - CompilationStateSnapshot.
//! - CompilationStateTracker.
//! - CompilationStateError.
//! - Legal lifecycle transitions.
//! - Logical state revision.
//!
//! This module does not own:
//!
//! - Compiler configuration.
//! - Resource budgets.
//! - CancellationToken implementation.
//! - Public compilation sessions.
//! - Pipeline orchestration.
//! - Stage execution.
//! - Semantic analysis.
//! - Ownership analysis.
//! - AST.
//! - ZUIR.
//! - Classical IR.
//! - Quantum IR.
//! - HDLIR.
//! - AI computation.
//! - Optimization.
//! - Hardware discovery.
//! - Hardware allocation.
//! - Backend implementation.
//! - Runtime execution.
//! - Distributed scheduling.
//! - Quantum routing.
//! - Quantum error correction.
//!
//! # Integration authorities
//!
//! crate::compiler::context::compilation_context
//!     CancellationToken
//!
//! crate::compiler::api::session
//!     CompilationSession
//!
//! crate::compiler::pipeline
//!     Stage execution and orchestration.
//!
//! crate::compiler::diagnostics
//!     Structured compiler diagnostics.
//!
//! crate::compiler::profiling
//!     Lifecycle observations.
//!
//! crate::compiler::context::provenance_context
//!     Compilation lineage.
//!
//! # POCO-REAF
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! This module imposes no universal hardware-capacity ceilings.
//!
//! Custom stages use dynamically sized identifiers.
//!
//! Actual resource constraints belong to the compilation context,
//! configured operational budgets and authoritative target providers.
//!
//! Unlimited operational budgets do not imply infinite physical resources.
//!
//! # Concurrency
//!
//! Cloned trackers share one synchronized lifecycle.
//!
//! All lifecycle mutations are serialized.
//!
//! External cancellation is observed through the shared cancellation token.
//!
//! No worker can publish successful completion after cancellation
//! has been observed by the lifecycle tracker.
//!
//! # Determinism
//!
//! Snapshots contain logical state only.
//!
//! Wall-clock timestamps, random identifiers and machine-specific
//! execution details belong to other subsystems.

use std::error::Error;
use std::fmt;
use std::sync::{
    Arc,
    Mutex,
    MutexGuard,
};

use super::compilation_context::CancellationToken;

// ============================================================================
// API VERSION
// ============================================================================

/// Public API version of the compilation-state contract.
///
/// Independent of:
///
/// - Zamani language version.
/// - Grammar version.
/// - Compiler release version.
/// - Semantic-model version.
/// - AST version.
/// - IR version.
/// - Backend ABI version.
pub const COMPILATION_STATE_API_VERSION: u16 = 1;

// ============================================================================
// COMPILATION LIFECYCLE
// ============================================================================

/// Lifecycle of one compilation operation.
///
/// Lifecycle state describes the status of the entire compilation,
/// not the implementation details of individual pipeline stages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CompilationLifecycle {

    /// Compilation state has been created.
    Created,

    /// Compilation is executing.
    Running,

    /// Cancellation has been requested.
    ///
    /// Active work may still need to stop cooperatively.
    CancellationRequested,

    /// Compilation completed successfully.
    Completed,

    /// Compilation failed.
    Failed,

    /// Compilation was cancelled.
    Cancelled,
}

impl CompilationLifecycle {

    /// Returns whether this state is terminal.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Completed
                | Self::Failed
                | Self::Cancelled
        )
    }

    /// Returns whether compilation has started and remains active.
    #[must_use]
    pub const fn is_active(self) -> bool {
        matches!(
            self,
            Self::Running
                | Self::CancellationRequested
        )
    }

    /// Returns whether cancellation was requested or completed.
    #[must_use]
    pub const fn cancellation_requested(self) -> bool {
        matches!(
            self,
            Self::CancellationRequested
                | Self::Cancelled
        )
    }

    /// Returns whether compilation completed successfully.
    #[must_use]
    pub const fn is_successful(self) -> bool {
        matches!(self, Self::Completed)
    }

    /// Returns whether compilation failed.
    #[must_use]
    pub const fn has_failed(self) -> bool {
        matches!(self, Self::Failed)
    }

    /// Returns whether compilation was cancelled.
    #[must_use]
    pub const fn is_cancelled(self) -> bool {
        matches!(self, Self::Cancelled)
    }
}

// ============================================================================
// COMPILATION PHASE
// ============================================================================

/// Standard high-level compiler phases.
///
/// These are organizational identifiers.
///
/// They do not implement compilation stages.
///
/// Dynamic stage names allow future domains and extensions to
/// introduce new stages without modifying this enumeration.
///
/// A new computing domain must not require a new lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CompilationPhase {

    /// Compiler initialization.
    Initialization,

    /// Lexical analysis and parsing.
    Parsing,

    /// Name and symbol analysis.
    NameAnalysis,

    /// Semantic analysis.
    SemanticAnalysis,

    /// Ownership and borrow analysis.
    OwnershipAnalysis,

    /// Dependency and effect analysis.
    DependencyAnalysis,

    /// Canonical IR construction.
    IrGeneration,

    /// IR verification.
    IrVerification,

    /// Program optimization.
    Optimization,

    /// Capability analysis.
    CapabilityAnalysis,

    /// Feasibility analysis.
    FeasibilityAnalysis,

    /// Compilation planning.
    Planning,

    /// Domain-specific lowering.
    Lowering,

    /// Linking.
    Linking,

    /// Backend code generation.
    CodeGeneration,

    /// Artifact validation.
    ArtifactValidation,

    /// Compilation finalization.
    Finalization,
}

impl CompilationPhase {

    /// Returns the stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Initialization => "initialization",
            Self::Parsing => "parsing",
            Self::NameAnalysis => "name_analysis",
            Self::SemanticAnalysis => "semantic_analysis",
            Self::OwnershipAnalysis => "ownership_analysis",
            Self::DependencyAnalysis => "dependency_analysis",
            Self::IrGeneration => "ir_generation",
            Self::IrVerification => "ir_verification",
            Self::Optimization => "optimization",
            Self::CapabilityAnalysis => "capability_analysis",
            Self::FeasibilityAnalysis => "feasibility_analysis",
            Self::Planning => "planning",
            Self::Lowering => "lowering",
            Self::Linking => "linking",
            Self::CodeGeneration => "code_generation",
            Self::ArtifactValidation => "artifact_validation",
            Self::Finalization => "finalization",
        }
    }
}

impl fmt::Display for CompilationPhase {

    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// STATE SNAPSHOT
// ============================================================================

/// Immutable observation of one compilation lifecycle.
///
/// Snapshots do not expose internal synchronization primitives.
///
/// The last known phase and stage are retained after failure or
/// cancellation to support diagnostics and profiling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompilationStateSnapshot {

    /// Current lifecycle.
    pub lifecycle: CompilationLifecycle,

    /// Last active high-level phase.
    pub phase: Option<CompilationPhase>,

    /// Last active stage.
    ///
    /// Dynamic names support registered extensions and future domains.
    pub stage_name: Option<Arc<str>>,

    /// Monotonically increasing logical revision.
    ///
    /// Revision overflow is rejected rather than wrapping.
    pub revision: u64,

    /// Descriptive failure information.
    ///
    /// Structured diagnostics remain owned by the diagnostics subsystem.
    pub failure: Option<Arc<str>>,
}

impl CompilationStateSnapshot {

    /// Returns whether compilation is terminal.
    #[must_use]
    pub const fn is_terminal(&self) -> bool {
        self.lifecycle.is_terminal()
    }

    /// Returns whether compilation is active.
    #[must_use]
    pub const fn is_active(&self) -> bool {
        self.lifecycle.is_active()
    }

    /// Returns whether compilation succeeded.
    #[must_use]
    pub const fn is_successful(&self) -> bool {
        self.lifecycle.is_successful()
    }

    /// Returns whether compilation was cancelled.
    #[must_use]
    pub const fn is_cancelled(&self) -> bool {
        self.lifecycle.is_cancelled()
    }

    /// Returns whether compilation failed.
    #[must_use]
    pub const fn has_failed(&self) -> bool {
        self.lifecycle.has_failed()
    }
}

// ============================================================================
// STATE ERRORS
// ============================================================================

/// Errors associated with lifecycle management.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CompilationStateError {

    /// An invalid lifecycle transition was requested.
    InvalidTransition {
        from: CompilationLifecycle,
        to: CompilationLifecycle,
    },

    /// An operation was attempted on a terminal state.
    TerminalState {
        state: CompilationLifecycle,
    },

    /// Cancellation has been requested.
    CancellationRequested,

    /// A stage name is empty.
    EmptyStageName,

    /// A stage name contains control characters.
    InvalidStageName,

    /// A failure description is empty.
    EmptyFailureDescription,

    /// Revision accounting cannot continue.
    ///
    /// No revision wrapping is permitted.
    RevisionExhausted,

    /// Internal synchronization failed.
    SynchronizationPoisoned,
}

impl fmt::Display for CompilationStateError {

    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {

        match self {

            Self::InvalidTransition { from, to } => {
                write!(
                    formatter,
                    "invalid compilation transition: {from:?} -> {to:?}"
                )
            }

            Self::TerminalState { state } => {
                write!(
                    formatter,
                    "compilation lifecycle is terminal: {state:?}"
                )
            }

            Self::CancellationRequested => {
                formatter.write_str(
                    "compilation cancellation has been requested"
                )
            }

            Self::EmptyStageName => {
                formatter.write_str(
                    "compilation stage name cannot be empty"
                )
            }

            Self::InvalidStageName => {
                formatter.write_str(
                    "compilation stage name contains control characters"
                )
            }

            Self::EmptyFailureDescription => {
                formatter.write_str(
                    "compilation failure description cannot be empty"
                )
            }

            Self::RevisionExhausted => {
                formatter.write_str(
                    "compilation state revision exhausted"
                )
            }

            Self::SynchronizationPoisoned => {
                formatter.write_str(
                    "compilation state synchronization is poisoned"
                )
            }
        }
    }
}

impl Error for CompilationStateError {}

// ============================================================================
// INTERNAL STATE
// ============================================================================

/// Internal lifecycle data.
///
/// Access is exclusively protected by the owning mutex.
#[derive(Debug)]
struct CompilationStateInner {

    lifecycle: CompilationLifecycle,

    phase: Option<CompilationPhase>,

    stage_name: Option<Arc<str>>,

    revision: u64,

    failure: Option<Arc<str>>,
}

impl CompilationStateInner {

    /// Creates a fresh lifecycle.
    fn new() -> Self {
        Self {
            lifecycle: CompilationLifecycle::Created,
            phase: None,
            stage_name: None,
            revision: 0,
            failure: None,
        }
    }

    /// Produces an immutable snapshot.
    fn snapshot(&self) -> CompilationStateSnapshot {
        CompilationStateSnapshot {
            lifecycle: self.lifecycle,
            phase: self.phase,
            stage_name: self.stage_name.clone(),
            revision: self.revision,
            failure: self.failure.clone(),
        }
    }

    /// Calculates the next revision without modifying state.
    fn next_revision(&self) -> Result<u64, CompilationStateError> {
        self.revision
            .checked_add(1)
            .ok_or(CompilationStateError::RevisionExhausted)
    }

    /// Ensures that the lifecycle is not terminal.
    fn ensure_non_terminal(
        &self,
    ) -> Result<(), CompilationStateError> {

        if self.lifecycle.is_terminal() {
            return Err(
                CompilationStateError::TerminalState {
                    state: self.lifecycle,
                }
            );
        }

        Ok(())
    }

    /// Records a new revision.
    fn commit_revision(&mut self, revision: u64) {
        self.revision = revision;
    }
}

// ============================================================================
// TRACKER
// ============================================================================

/// Thread-safe lifecycle tracker for one compilation operation.
///
/// Clones share the same lifecycle and cancellation authority.
///
/// The tracker does not create a second compilation operation when cloned.
#[derive(Debug, Clone)]
pub struct CompilationStateTracker {

    inner: Arc<Mutex<CompilationStateInner>>,

    cancellation: CancellationToken,
}

impl Default for CompilationStateTracker {

    fn default() -> Self {
        Self::new()
    }
}

impl CompilationStateTracker {

    /// Creates a fresh compilation lifecycle.
    #[must_use]
    pub fn new() -> Self {
        Self::with_cancellation(CancellationToken::new())
    }

    /// Creates a lifecycle using an existing cancellation authority.
    ///
    /// This is the integration point for CompilationContext.
    #[must_use]
    pub fn with_cancellation(
        cancellation: CancellationToken,
    ) -> Self {

        Self {
            inner: Arc::new(
                Mutex::new(
                    CompilationStateInner::new()
                )
            ),
            cancellation,
        }
    }

    // ------------------------------------------------------------------------
    // CANCELLATION
    // ------------------------------------------------------------------------

    /// Returns whether cancellation has been requested.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.cancellation.is_cancelled()
    }

    /// Returns whether cancellation has been requested.
    #[must_use]
    pub fn cancellation_requested(&self) -> bool {
        self.is_cancelled()
    }

    /// Returns the shared cancellation token.
    ///
    /// This does not create another cancellation authority.
    #[must_use]
    pub fn cancellation_token(&self) -> CancellationToken {
        self.cancellation.clone()
    }

    /// Synchronizes externally requested cancellation with lifecycle state.
    ///
    /// Running becomes CancellationRequested.
    ///
    /// Created becomes Cancelled because no active work exists to stop.
    ///
    /// Terminal states are never overwritten.
    fn synchronize_cancellation(
        &self,
        state: &mut CompilationStateInner,
    ) -> Result<(), CompilationStateError> {

        if !self.cancellation.is_cancelled() {
            return Ok(());
        }

        match state.lifecycle {

            CompilationLifecycle::Created => {
                let revision = state.next_revision()?;

                state.lifecycle = CompilationLifecycle::Cancelled;

                state.commit_revision(revision);
            }

            CompilationLifecycle::Running => {
                let revision = state.next_revision()?;

                state.lifecycle =
                    CompilationLifecycle::CancellationRequested;

                state.commit_revision(revision);
            }

            CompilationLifecycle::CancellationRequested
            | CompilationLifecycle::Completed
            | CompilationLifecycle::Failed
            | CompilationLifecycle::Cancelled => {}
        }

        Ok(())
    }

    // ------------------------------------------------------------------------
    // OBSERVATION
    // ------------------------------------------------------------------------

    /// Returns an immutable lifecycle snapshot.
    ///
    /// External cancellation is synchronized before observation.
    pub fn snapshot(
        &self,
    ) -> Result<CompilationStateSnapshot, CompilationStateError> {

        let mut state = self.lock_state()?;

        self.synchronize_cancellation(&mut state)?;

        Ok(state.snapshot())
    }

    /// Returns the current lifecycle.
    pub fn lifecycle(
        &self,
    ) -> Result<CompilationLifecycle, CompilationStateError> {

        Ok(self.snapshot()?.lifecycle)
    }

    // ------------------------------------------------------------------------
    // START
    // ------------------------------------------------------------------------

    /// Starts compilation.
    ///
    /// Only Created may transition to Running.
    ///
    /// The initial phase is Initialization.
    pub fn start(
        &self,
    ) -> Result<(), CompilationStateError> {

        let mut state = self.lock_state()?;

        self.synchronize_cancellation(&mut state)?;

        if state.lifecycle == CompilationLifecycle::Cancelled {
            return Err(
                CompilationStateError::CancellationRequested
            );
        }

        state.ensure_non_terminal()?;

        if state.lifecycle != CompilationLifecycle::Created {
            return Err(
                CompilationStateError::InvalidTransition {
                    from: state.lifecycle,
                    to: CompilationLifecycle::Running,
                }
            );
        }

        let revision = state.next_revision()?;

        state.lifecycle = CompilationLifecycle::Running;

        state.phase = Some(CompilationPhase::Initialization);

        state.stage_name = Some(
            Arc::from("initialization")
        );

        state.failure = None;

        state.commit_revision(revision);

        Ok(())
    }

    // ------------------------------------------------------------------------
    // PHASE ADVANCEMENT
    // ------------------------------------------------------------------------

    /// Advances compilation to another phase and stage.
    ///
    /// Stage identifiers are dynamically sized.
    ///
    /// No fixed number of stages or computing domains is imposed.
    pub fn advance(
        &self,
        phase: CompilationPhase,
        stage_name: impl Into<Arc<str>>,
    ) -> Result<(), CompilationStateError> {

        let stage_name = validate_stage_name(stage_name)?;

        let mut state = self.lock_state()?;

        self.synchronize_cancellation(&mut state)?;

        state.ensure_non_terminal()?;

        ensure_running(&state)?;

        let revision = state.next_revision()?;

        state.phase = Some(phase);

        state.stage_name = Some(stage_name);

        state.commit_revision(revision);

        Ok(())
    }

    /// Changes the active stage without changing the phase.
    ///
    /// Supports dynamically registered compiler extensions.
    pub fn set_stage(
        &self,
        stage_name: impl Into<Arc<str>>,
    ) -> Result<(), CompilationStateError> {

        let stage_name = validate_stage_name(stage_name)?;

        let mut state = self.lock_state()?;

        self.synchronize_cancellation(&mut state)?;

        state.ensure_non_terminal()?;

        ensure_running(&state)?;

        let revision = state.next_revision()?;

        state.stage_name = Some(stage_name);

        state.commit_revision(revision);

        Ok(())
    }

    // ------------------------------------------------------------------------
    // CANCELLATION TRANSITIONS
    // ------------------------------------------------------------------------

    /// Requests cancellation.
    ///
    /// Created becomes Cancelled.
    ///
    /// Running becomes CancellationRequested.
    ///
    /// Repeated cancellation requests are idempotent.
    pub fn request_cancel(
        &self,
    ) -> Result<(), CompilationStateError> {

        let mut state = self.lock_state()?;

        if state.lifecycle.is_terminal() {
            return Err(
                CompilationStateError::TerminalState {
                    state: state.lifecycle,
                }
            );
        }

        match state.lifecycle {

            CompilationLifecycle::Created => {
                let revision = state.next_revision()?;

                self.cancellation.cancel();

                state.lifecycle = CompilationLifecycle::Cancelled;

                state.commit_revision(revision);
            }

            CompilationLifecycle::Running => {
                let revision = state.next_revision()?;

                self.cancellation.cancel();

                state.lifecycle =
                    CompilationLifecycle::CancellationRequested;

                state.commit_revision(revision);
            }

            CompilationLifecycle::CancellationRequested => {
                self.cancellation.cancel();
            }

            CompilationLifecycle::Completed
            | CompilationLifecycle::Failed
            | CompilationLifecycle::Cancelled => {
                return Err(
                    CompilationStateError::TerminalState {
                        state: state.lifecycle,
                    }
                );
            }
        }

        Ok(())
    }

    /// Acknowledges cooperative cancellation.
    ///
    /// Running work must first have cancellation requested.
    ///
    /// A Created operation may be cancelled before execution.
    pub fn mark_cancelled(
        &self,
    ) -> Result<(), CompilationStateError> {

        let mut state = self.lock_state()?;

        self.synchronize_cancellation(&mut state)?;

        state.ensure_non_terminal()?;

        match state.lifecycle {

            CompilationLifecycle::Created => {
                let revision = state.next_revision()?;

                self.cancellation.cancel();

                state.lifecycle = CompilationLifecycle::Cancelled;

                state.commit_revision(revision);
            }

            CompilationLifecycle::CancellationRequested => {
                let revision = state.next_revision()?;

                state.lifecycle = CompilationLifecycle::Cancelled;

                state.commit_revision(revision);
            }

            CompilationLifecycle::Running => {
                if !self.cancellation.is_cancelled() {
                    return Err(
                        CompilationStateError::InvalidTransition {
                            from: state.lifecycle,
                            to: CompilationLifecycle::Cancelled,
                        }
                    );
                }

                let revision = state.next_revision()?;

                state.lifecycle = CompilationLifecycle::Cancelled;

                state.commit_revision(revision);
            }

            CompilationLifecycle::Completed
            | CompilationLifecycle::Failed
            | CompilationLifecycle::Cancelled => {
                return Err(
                    CompilationStateError::TerminalState {
                        state: state.lifecycle,
                    }
                );
            }
        }

        Ok(())
    }

    // ------------------------------------------------------------------------
    // COMPLETION
    // ------------------------------------------------------------------------

    /// Completes compilation successfully.
    ///
    /// Only Running may transition to Completed.
    ///
    /// Cancellation prevents successful completion.
    pub fn complete(
        &self,
    ) -> Result<(), CompilationStateError> {

        let mut state = self.lock_state()?;

        self.synchronize_cancellation(&mut state)?;

        state.ensure_non_terminal()?;

        ensure_running(&state)?;

        let revision = state.next_revision()?;

        state.lifecycle = CompilationLifecycle::Completed;

        state.phase = Some(CompilationPhase::Finalization);

        state.stage_name = Some(
            Arc::from("completed")
        );

        state.commit_revision(revision);

        Ok(())
    }

    // ------------------------------------------------------------------------
    // FAILURE
    // ------------------------------------------------------------------------

    /// Records compilation failure.
    ///
    /// Failure may occur before or during execution.
    ///
    /// The last known phase and stage are retained.
    ///
    /// Structured diagnostics remain authoritative for detailed
    /// error information.
    pub fn fail(
        &self,
        description: impl Into<Arc<str>>,
    ) -> Result<(), CompilationStateError> {

        let description = description.into();

        if description.trim().is_empty() {
            return Err(
                CompilationStateError::EmptyFailureDescription
            );
        }

        let mut state = self.lock_state()?;

        self.synchronize_cancellation(&mut state)?;

        state.ensure_non_terminal()?;

        if state.lifecycle ==
            CompilationLifecycle::CancellationRequested
        {
            return Err(
                CompilationStateError::CancellationRequested
            );
        }

        let revision = state.next_revision()?;

        state.lifecycle = CompilationLifecycle::Failed;

        state.failure = Some(description);

        state.commit_revision(revision);

        Ok(())
    }

    // ------------------------------------------------------------------------
    // EXECUTION VALIDATION
    // ------------------------------------------------------------------------

    /// Checks whether compilation may continue.
    ///
    /// Synchronizes external cancellation before returning.
    pub fn check_active(
        &self,
    ) -> Result<(), CompilationStateError> {

        let mut state = self.lock_state()?;

        self.synchronize_cancellation(&mut state)?;

        state.ensure_non_terminal()?;

        ensure_running(&state)?;

        Ok(())
    }

    // ------------------------------------------------------------------------
    // OBSERVABILITY
    // ------------------------------------------------------------------------

    /// Returns the number of strong references to shared state.
    ///
    /// Observability only. Never use this to determine whether
    /// compilation has finished.
    #[must_use]
    pub fn reference_count(&self) -> usize {
        Arc::strong_count(&self.inner)
    }

    // ------------------------------------------------------------------------
    // SYNCHRONIZATION
    // ------------------------------------------------------------------------

    /// Acquires the lifecycle lock.
    fn lock_state(
        &self,
    ) -> Result<
        MutexGuard<'_, CompilationStateInner>,
        CompilationStateError,
    > {

        self.inner
            .lock()
            .map_err(|_| {
                CompilationStateError::SynchronizationPoisoned
            })
    }
}

// ============================================================================
// VALIDATION HELPERS
// ============================================================================

/// Validates a dynamic compilation-stage identifier.
fn validate_stage_name(
    stage_name: impl Into<Arc<str>>,
) -> Result<Arc<str>, CompilationStateError> {

    let stage_name = stage_name.into();

    if stage_name.trim().is_empty() {
        return Err(
            CompilationStateError::EmptyStageName
        );
    }

    if stage_name.chars().any(char::is_control) {
        return Err(
            CompilationStateError::InvalidStageName
        );
    }

    Ok(stage_name)
}

/// Ensures that lifecycle is Running and cancellation is absent.
///
/// Called while the lifecycle mutex is held.
fn ensure_running(
    state: &CompilationStateInner,
) -> Result<(), CompilationStateError> {

    if state.lifecycle ==
        CompilationLifecycle::CancellationRequested
    {
        return Err(
            CompilationStateError::CancellationRequested
        );
    }

    if state.lifecycle != CompilationLifecycle::Running {
        return Err(
            CompilationStateError::InvalidTransition {
                from: state.lifecycle,
                to: CompilationLifecycle::Running,
            }
        );
    }

    Ok(())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {

    use super::*;

    // ------------------------------------------------------------------------
    // Construction
    // ------------------------------------------------------------------------

    #[test]
    fn initial_state_is_created() {
        let tracker = CompilationStateTracker::new();

        let snapshot = tracker.snapshot().unwrap();

        assert_eq!(
            snapshot.lifecycle,
            CompilationLifecycle::Created
        );

        assert_eq!(snapshot.phase, None);

        assert_eq!(snapshot.stage_name, None);

        assert_eq!(snapshot.revision, 0);

        assert_eq!(snapshot.failure, None);
    }

    #[test]
    fn default_tracker_is_created() {
        let tracker = CompilationStateTracker::default();

        assert_eq!(
            tracker.lifecycle().unwrap(),
            CompilationLifecycle::Created
        );
    }

    // ------------------------------------------------------------------------
    // Lifecycle
    // ------------------------------------------------------------------------

    #[test]
    fn compilation_can_start() {
        let tracker = CompilationStateTracker::new();

        tracker.start().unwrap();

        assert_eq!(
            tracker.lifecycle().unwrap(),
            CompilationLifecycle::Running
        );
    }

    #[test]
    fn start_sets_initialization_phase() {
        let tracker = CompilationStateTracker::new();

        tracker.start().unwrap();

        let snapshot = tracker.snapshot().unwrap();

        assert_eq!(
            snapshot.phase,
            Some(CompilationPhase::Initialization)
        );

        assert_eq!(
            snapshot.stage_name.as_deref(),
            Some("initialization")
        );
    }

    #[test]
    fn compilation_can_advance() {
        let tracker = CompilationStateTracker::new();

        tracker.start().unwrap();

        tracker.advance(
            CompilationPhase::SemanticAnalysis,
            "semantic_validation",
        ).unwrap();

        let snapshot = tracker.snapshot().unwrap();

        assert_eq!(
            snapshot.phase,
            Some(CompilationPhase::SemanticAnalysis)
        );

        assert_eq!(
            snapshot.stage_name.as_deref(),
            Some("semantic_validation")
        );
    }

    #[test]
    fn custom_stages_are_supported() {
        let tracker = CompilationStateTracker::new();

        tracker.start().unwrap();

        tracker.set_stage(
            "future_neuromorphic_lowering"
        ).unwrap();

        assert_eq!(
            tracker.snapshot().unwrap().stage_name.as_deref(),
            Some("future_neuromorphic_lowering")
        );
    }

    #[test]
    fn unicode_stage_names_are_supported() {
        let tracker = CompilationStateTracker::new();

        tracker.start().unwrap();

        tracker.set_stage("量子混合编译").unwrap();

        assert_eq!(
            tracker.snapshot().unwrap().stage_name.as_deref(),
            Some("量子混合编译")
        );
    }

    #[test]
    fn empty_stage_names_are_rejected() {
        let tracker = CompilationStateTracker::new();

        tracker.start().unwrap();

        assert_eq!(
            tracker.set_stage("   "),
            Err(CompilationStateError::EmptyStageName)
        );
    }

    #[test]
    fn control_characters_in_stage_names_are_rejected() {
        let tracker = CompilationStateTracker::new();

        tracker.start().unwrap();

        assert_eq!(
            tracker.set_stage("stage\nname"),
            Err(CompilationStateError::InvalidStageName)
        );
    }

    // ------------------------------------------------------------------------
    // Completion
    // ------------------------------------------------------------------------

    #[test]
    fn running_compilation_can_complete() {
        let tracker = CompilationStateTracker::new();

        tracker.start().unwrap();

        tracker.complete().unwrap();

        assert_eq!(
            tracker.lifecycle().unwrap(),
            CompilationLifecycle::Completed
        );
    }

    #[test]
    fn completed_state_is_terminal() {
        let tracker = CompilationStateTracker::new();

        tracker.start().unwrap();

        tracker.complete().unwrap();

        assert!(matches!(
            tracker.set_stage("another"),
            Err(CompilationStateError::TerminalState { .. })
        ));
    }

    #[test]
    fn completion_before_start_is_rejected() {
        let tracker = CompilationStateTracker::new();

        assert!(tracker.complete().is_err());
    }

    // ------------------------------------------------------------------------
    // Failure
    // ------------------------------------------------------------------------

    #[test]
    fn compilation_can_fail_before_start() {
        let tracker = CompilationStateTracker::new();

        tracker.fail("configuration validation failed").unwrap();

        let snapshot = tracker.snapshot().unwrap();

        assert_eq!(
            snapshot.lifecycle,
            CompilationLifecycle::Failed
        );

        assert_eq!(
            snapshot.failure.as_deref(),
            Some("configuration validation failed")
        );
    }

    #[test]
    fn failure_retains_last_known_stage() {
        let tracker = CompilationStateTracker::new();

        tracker.start().unwrap();

        tracker.advance(
            CompilationPhase::IrVerification,
            "verify_quantum_ir",
        ).unwrap();

        tracker.fail("verification failed").unwrap();

        let snapshot = tracker.snapshot().unwrap();

        assert_eq!(
            snapshot.lifecycle,
            CompilationLifecycle::Failed
        );

        assert_eq!(
            snapshot.phase,
            Some(CompilationPhase::IrVerification)
        );

        assert_eq!(
            snapshot.stage_name.as_deref(),
            Some("verify_quantum_ir")
        );
    }

    #[test]
    fn empty_failure_description_is_rejected() {
        let tracker = CompilationStateTracker::new();

        assert_eq!(
            tracker.fail(" "),
            Err(CompilationStateError::EmptyFailureDescription)
        );
    }

    #[test]
    fn failure_cannot_overwrite_success() {
        let tracker = CompilationStateTracker::new();

        tracker.start().unwrap();

        tracker.complete().unwrap();

        assert!(tracker.fail("late failure").is_err());
    }

    // ------------------------------------------------------------------------
    // Cancellation
    // ------------------------------------------------------------------------

    #[test]
    fn cancellation_before_start_is_terminal() {
        let tracker = CompilationStateTracker::new();

        tracker.request_cancel().unwrap();

        assert_eq!(
            tracker.lifecycle().unwrap(),
            CompilationLifecycle::Cancelled
        );
    }

    #[test]
    fn cancellation_during_execution_is_requested() {
        let tracker = CompilationStateTracker::new();

        tracker.start().unwrap();

        tracker.request_cancel().unwrap();

        assert_eq!(
            tracker.lifecycle().unwrap(),
            CompilationLifecycle::CancellationRequested
        );
    }

    #[test]
    fn cancellation_must_be_acknowledged() {
        let tracker = CompilationStateTracker::new();

        tracker.start().unwrap();

        tracker.request_cancel().unwrap();

        tracker.mark_cancelled().unwrap();

        assert_eq!(
            tracker.lifecycle().unwrap(),
            CompilationLifecycle::Cancelled
        );
    }

    #[test]
    fn cancellation_prevents_completion() {
        let tracker = CompilationStateTracker::new();

        tracker.start().unwrap();

        tracker.request_cancel().unwrap();

        assert_eq!(
            tracker.complete(),
            Err(CompilationStateError::CancellationRequested)
        );
    }

    #[test]
    fn cancellation_prevents_stage_advancement() {
        let tracker = CompilationStateTracker::new();

        tracker.start().unwrap();

        tracker.request_cancel().unwrap();

        assert_eq!(
            tracker.set_stage("late_stage"),
            Err(CompilationStateError::CancellationRequested)
        );
    }

    #[test]
    fn external_cancellation_is_synchronized() {
        let token = CancellationToken::new();

        let tracker =
            CompilationStateTracker::with_cancellation(token.clone());

        tracker.start().unwrap();

        token.cancel();

        assert_eq!(
            tracker.lifecycle().unwrap(),
            CompilationLifecycle::CancellationRequested
        );
    }

    #[test]
    fn external_cancellation_before_start_is_observed() {
        let token = CancellationToken::new();

        let tracker =
            CompilationStateTracker::with_cancellation(token.clone());

        token.cancel();

        assert_eq!(
            tracker.lifecycle().unwrap(),
            CompilationLifecycle::Cancelled
        );
    }

    #[test]
    fn external_cancellation_prevents_completion() {
        let token = CancellationToken::new();

        let tracker =
            CompilationStateTracker::with_cancellation(token.clone());

        tracker.start().unwrap();

        token.cancel();

        assert_eq!(
            tracker.complete(),
            Err(CompilationStateError::CancellationRequested)
        );
    }

    #[test]
    fn cancellation_is_shared_between_clones() {
        let tracker = CompilationStateTracker::new();

        let clone = tracker.clone();

        tracker.start().unwrap();

        tracker.request_cancel().unwrap();

        assert!(clone.is_cancelled());

        assert_eq!(
            clone.lifecycle().unwrap(),
            CompilationLifecycle::CancellationRequested
        );
    }

    #[test]
    fn repeated_cancellation_requests_are_idempotent() {
        let tracker = CompilationStateTracker::new();

        tracker.start().unwrap();

        tracker.request_cancel().unwrap();

        let revision = tracker.snapshot().unwrap().revision;

        tracker.request_cancel().unwrap();

        assert_eq!(
            tracker.snapshot().unwrap().revision,
            revision
        );
    }

    // ------------------------------------------------------------------------
    // State invariants
    // ------------------------------------------------------------------------

    #[test]
    fn terminal_states_cannot_be_reused() {
        let tracker = CompilationStateTracker::new();

        tracker.start().unwrap();

        tracker.complete().unwrap();

        assert!(tracker.start().is_err());
    }

    #[test]
    fn failed_transitions_do_not_change_state() {
        let tracker = CompilationStateTracker::new();

        let before = tracker.snapshot().unwrap();

        assert!(tracker.complete().is_err());

        let after = tracker.snapshot().unwrap();

        assert_eq!(before, after);
    }

    #[test]
    fn revisions_increase_monotonically() {
        let tracker = CompilationStateTracker::new();

        let initial = tracker.snapshot().unwrap();

        tracker.start().unwrap();

        let started = tracker.snapshot().unwrap();

        tracker.advance(
            CompilationPhase::Parsing,
            "parser",
        ).unwrap();

        let advanced = tracker.snapshot().unwrap();

        tracker.complete().unwrap();

        let completed = tracker.snapshot().unwrap();

        assert_eq!(initial.revision, 0);

        assert!(started.revision > initial.revision);

        assert!(advanced.revision > started.revision);

        assert!(completed.revision > advanced.revision);
    }

    // ------------------------------------------------------------------------
    // Snapshot invariants
    // ------------------------------------------------------------------------

    #[test]
    fn previous_snapshots_are_immutable() {
        let tracker = CompilationStateTracker::new();

        tracker.start().unwrap();

        let previous = tracker.snapshot().unwrap();

        tracker.advance(
            CompilationPhase::Parsing,
            "parser",
        ).unwrap();

        let current = tracker.snapshot().unwrap();

        assert_ne!(previous, current);

        assert_eq!(
            previous.phase,
            Some(CompilationPhase::Initialization)
        );

        assert_eq!(
            current.phase,
            Some(CompilationPhase::Parsing)
        );
    }

    #[test]
    fn completed_snapshot_reports_success() {
        let tracker = CompilationStateTracker::new();

        tracker.start().unwrap();

        tracker.complete().unwrap();

        let snapshot = tracker.snapshot().unwrap();

        assert!(snapshot.is_terminal());

        assert!(snapshot.is_successful());

        assert!(!snapshot.is_active());
    }

    // ------------------------------------------------------------------------
    // Concurrency
    // ------------------------------------------------------------------------

    #[test]
    fn tracker_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<CompilationStateTracker>();

        assert_send_sync::<CompilationStateSnapshot>();
    }

    #[test]
    fn clones_share_lifecycle() {
        let tracker = CompilationStateTracker::new();

        let clone = tracker.clone();

        tracker.start().unwrap();

        assert_eq!(
            clone.lifecycle().unwrap(),
            CompilationLifecycle::Running
        );
    }

    #[test]
    fn reference_count_tracks_shared_ownership() {
        let tracker = CompilationStateTracker::new();

        let clone = tracker.clone();

        assert_eq!(tracker.reference_count(), 2);

        drop(clone);

        assert_eq!(tracker.reference_count(), 1);
    }

    // ------------------------------------------------------------------------
    // Phase identifiers
    // ------------------------------------------------------------------------

    #[test]
    fn phase_identifiers_are_stable() {
        assert_eq!(
            CompilationPhase::SemanticAnalysis.as_str(),
            "semantic_analysis"
        );

        assert_eq!(
            CompilationPhase::CodeGeneration.as_str(),
            "code_generation"
        );

        assert_eq!(
            CompilationPhase::ArtifactValidation.as_str(),
            "artifact_validation"
        );
    }
}
