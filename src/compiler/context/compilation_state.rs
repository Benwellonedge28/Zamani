
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
//!     Minimum version: Rust 1.97
//!
//! Safety:
//!     Safe Rust only.
//!     Unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Defines the lifecycle and progress state of one compiler operation.
//!
//! The state tracker coordinates:
//!
//! - Compilation initialization.
//! - Compilation execution.
//! - Compilation phase transitions.
//! - Dynamically named compilation stages.
//! - Cancellation requests.
//! - Successful completion.
//! - Compilation failure.
//! - Terminal-state enforcement.
//! - Thread-safe state observation.
//! - Deterministic state snapshots.
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
//! - Legal compilation-state transitions.
//! - Compilation progress metadata.
//!
//! This module does not own:
//!
//! - Compiler configuration.
//! - Resource budgets.
//! - Cancellation token implementation.
//! - Public compilation sessions.
//! - Pipeline orchestration.
//! - Semantic analysis.
//! - Ownership analysis.
//! - AST.
//! - Canonical IR.
//! - Optimization algorithms.
//! - Hardware discovery.
//! - Hardware allocation.
//! - Backend implementation.
//! - Runtime execution.
//! - Distributed scheduling.
//! - Quantum routing.
//! - Quantum error correction.
//!
//! # Existing integration authorities
//!
//! crate::compiler::context::compilation_context
//!     CompilationContext
//!     CancellationToken
//!
//! crate::compiler::api::session
//!     Public API session lifecycle.
//!
//! crate::compiler::pipeline
//!     Future pipeline orchestration.
//!
//! crate::compiler::diagnostics
//!     Compiler diagnostic reporting.
//!
//! # Lifecycle
//!
//! Created
//!    |
//!    v
//! Running
//!    |
//!    +--------------------------+
//!    |                          |
//!    v                          v
//! Completed              CancellationRequested
//!    ^                          |
//!    |                          v
//!    |                       Cancelled
//!    |
//!    +--------------------------
//!
//! Running may also transition to Failed.
//!
//! Created may transition directly to Failed or Cancelled.
//!
//! Terminal states cannot be reused.
//!
//! # POCO-REAF
//!
//! This module introduces no fixed hardware-capacity limits.
//!
//! It does not impose limits on:
//!
//! - CPUs.
//! - Cores.
//! - Threads.
//! - GPUs.
//! - FPGAs.
//! - QPUs.
//! - Accelerators.
//! - Nodes.
//! - Devices.
//! - Memory.
//! - Storage.
//! - Qubits.
//! - Tensor dimensions.
//! - Tensor rank.
//! - IR operations.
//! - Functions.
//! - Modules.
//! - Compilation stages.
//!
//! Custom compilation stages use dynamically sized identifiers.
//!
//! Actual resource constraints are managed by CompilationContext
//! and its authoritative budget implementation.
//!
//! # Concurrency
//!
//! Cloned state trackers share one synchronization-protected state.
//!
//! State transitions are serialized.
//!
//! A worker cannot independently complete a compilation after
//! another worker has cancelled or failed it.
//!
//! # Determinism
//!
//! State snapshots contain logical lifecycle information.
//!
//! They do not contain wall-clock timestamps or random identifiers.
//!
//! Identical sequences of accepted transitions produce identical
//! logical snapshots.

use std::error::Error;
use std::fmt;
use std::sync::{Arc, Mutex};

use super::compilation_context::CancellationToken;

// ============================================================================
// API VERSION
// ============================================================================

/// Public API version of the compilation-state contract.
///
/// This version is independent of:
///
/// - Zamani language version.
/// - Grammar version.
/// - Compiler release version.
/// - Semantic-model version.
/// - AST version.
/// - IR version.
/// - Backend ABI version.
///
/// Increment when an incompatible public state contract changes.
pub const COMPILATION_STATE_API_VERSION: u16 = 1;

// ============================================================================
// COMPILATION LIFECYCLE
// ============================================================================

/// High-level lifecycle state of one compilation operation.
///
/// Lifecycle states describe whether compilation exists,
/// is executing, has completed, has failed or has been cancelled.
///
/// They do not describe the implementation details of a
/// particular compilation stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CompilationLifecycle {
    /// Compilation state has been created.
    ///
    /// No compilation work has started.
    Created,

    /// Compilation is executing.
    Running,

    /// Cancellation has been requested.
    ///
    /// The operation must not publish successful completion.
    ///
    /// Active work may still need to cooperate with cancellation.
    CancellationRequested,

    /// Compilation completed successfully.
    Completed,

    /// Compilation failed.
    Failed,

    /// Compilation was cancelled.
    Cancelled,
}

impl CompilationLifecycle {
    /// Returns whether this lifecycle state is terminal.
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

    /// Returns whether cancellation has been requested or completed.
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
}

// ============================================================================
// COMPILATION PHASE
// ============================================================================

/// Standard high-level compilation phases.
///
/// These phases are organizational identifiers.
///
/// They do not own the implementations of their corresponding
/// compiler subsystems.
///
/// The active stage name provides an extensibility mechanism
/// for compiler extensions and future computing domains.
///
/// A compiler extension does not need to modify this enum
/// merely to introduce another compilation stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CompilationPhase {
    /// Compiler initialization and preparation.
    Initialization,

    /// Lexical analysis and parsing.
    Parsing,

    /// Name and symbol analysis.
    NameAnalysis,

    /// Type and semantic analysis.
    SemanticAnalysis,

    /// Ownership and borrow analysis.
    OwnershipAnalysis,

    /// Effect and dependency analysis.
    DependencyAnalysis,

    /// Canonical IR construction.
    IrGeneration,

    /// IR verification.
    IrVerification,

    /// Program optimization.
    Optimization,

    /// Capability and requirement analysis.
    CapabilityAnalysis,

    /// Compilation feasibility analysis.
    FeasibilityAnalysis,

    /// Heterogeneous workload planning.
    Planning,

    /// IR conversion and target legalization.
    Lowering,

    /// Symbol resolution and linking.
    Linking,

    /// Backend code generation.
    CodeGeneration,

    /// Artifact verification.
    ArtifactValidation,

    /// Final compilation result preparation.
    Finalization,
}

impl CompilationPhase {
    /// Returns a stable machine-readable phase identifier.
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

// ============================================================================
// COMPILATION STATE SNAPSHOT
// ============================================================================

/// Immutable snapshot of compilation lifecycle information.
///
/// Snapshots can safely be returned to:
///
/// - Compiler APIs.
/// - IDE integrations.
/// - Language servers.
/// - Pipeline observers.
/// - Parallel compilation workers.
/// - Diagnostics.
/// - Profiling systems.
///
/// A snapshot does not expose internal synchronization primitives.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompilationStateSnapshot {
    /// Current lifecycle state.
    pub lifecycle: CompilationLifecycle,

    /// Current high-level compilation phase.
    pub phase: Option<CompilationPhase>,

    /// Dynamically named active compilation stage.
    ///
    /// This supports future compiler extensions without
    /// modifying the lifecycle enum.
    pub stage_name: Option<Arc<str>>,

    /// Monotonically increasing logical transition counter.
    ///
    /// Every accepted state modification increments this value.
    pub revision: u64,

    /// Failure description when compilation has failed.
    ///
    /// This is descriptive information, not a replacement
    /// for the structured compiler diagnostic system.
    pub failure: Option<Arc<str>>,
}

impl CompilationStateSnapshot {
    /// Returns whether compilation is terminal.
    #[must_use]
    pub const fn is_terminal(&self) -> bool {
        self.lifecycle.is_terminal()
    }

    /// Returns whether compilation is actively executing.
    #[must_use]
    pub const fn is_active(&self) -> bool {
        self.lifecycle.is_active()
    }

    /// Returns whether compilation succeeded.
    #[must_use]
    pub const fn is_successful(&self) -> bool {
        self.lifecycle.is_successful()
    }
}

// ============================================================================
// COMPILATION STATE ERROR
// ============================================================================

/// Errors produced by compilation-state management.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CompilationStateError {
    /// A requested transition is illegal.
    InvalidTransition {
        from: CompilationLifecycle,
        to: CompilationLifecycle,
    },

    /// An operation was attempted after reaching a terminal state.
    TerminalState {
        state: CompilationLifecycle,
    },

    /// Cancellation was requested.
    CancellationRequested,

    /// The supplied stage name is empty.
    EmptyStageName,

    /// The supplied failure description is empty.
    EmptyFailureDescription,

    /// The transition revision counter cannot be incremented.
    ///
    /// This is an implementation accounting failure, not a
    /// semantic or hardware capacity limit.
    RevisionExhausted,

    /// Synchronization state was poisoned.
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
                    "invalid compilation lifecycle transition: \
                     {from:?} -> {to:?}"
                )
            }

            Self::TerminalState { state } => {
                write!(
                    formatter,
                    "compilation state is terminal: {state:?}"
                )
            }

            Self::CancellationRequested => {
                formatter.write_str(
                    "compilation cancellation has been requested",
                )
            }

            Self::EmptyStageName => {
                formatter.write_str(
                    "compilation stage name cannot be empty",
                )
            }

            Self::EmptyFailureDescription => {
                formatter.write_str(
                    "compilation failure description cannot be empty",
                )
            }

            Self::RevisionExhausted => {
                formatter.write_str(
                    "compilation state revision counter exhausted",
                )
            }

            Self::SynchronizationPoisoned => {
                formatter.write_str(
                    "compilation state synchronization is poisoned",
                )
            }
        }
    }
}

impl Error for CompilationStateError {}

// ============================================================================
// INTERNAL STATE
// ============================================================================

/// Internal mutable state.
///
/// All modifications are protected by the owning Mutex.
///
/// The state is never exposed directly.
#[derive(Debug)]
struct CompilationStateInner {
    lifecycle: CompilationLifecycle,
    phase: Option<CompilationPhase>,
    stage_name: Option<Arc<str>>,
    revision: u64,
    failure: Option<Arc<str>>,
}

impl CompilationStateInner {
    /// Creates an initial state.
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

    /// Increments the logical revision safely.
    fn next_revision(&self) -> Result<u64, CompilationStateError> {
        self.revision
            .checked_add(1)
            .ok_or(CompilationStateError::RevisionExhausted)
    }

    /// Rejects changes to terminal states.
    fn ensure_non_terminal(
        &self,
    ) -> Result<(), CompilationStateError> {
        if self.lifecycle.is_terminal() {
            return Err(
                CompilationStateError::TerminalState {
                    state: self.lifecycle,
                },
            );
        }

        Ok(())
    }
}

// ============================================================================
// SHARED COMPILATION STATE TRACKER
// ============================================================================

/// Thread-safe tracker for one compilation operation.
///
/// Cloning this tracker does not create another compilation.
///
/// Every clone observes and modifies the same lifecycle state.
///
/// The tracker can be used by:
///
/// - Pipeline orchestration.
/// - Compiler API services.
/// - Compilation workers.
/// - IR transformation passes.
/// - Backend invocation.
/// - Diagnostics and monitoring.
///
/// Cancellation is delegated to the existing CancellationToken.
///
/// This avoids creating another cancellation implementation.
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
    /// Creates a new compilation state tracker.
    ///
    /// The initial lifecycle state is Created.
    ///
    /// A new cancellation token is created for this tracker.
    #[must_use]
    pub fn new() -> Self {
        Self::with_cancellation(
            CancellationToken::new(),
        )
    }

    /// Creates a tracker using an existing cancellation token.
    ///
    /// This constructor is the integration point for
    /// CompilationContext and other cancellation-aware systems.
    ///
    /// Cloned trackers share both lifecycle state and cancellation.
    #[must_use]
    pub fn with_cancellation(
        cancellation: CancellationToken,
    ) -> Self {
        Self {
            inner: Arc::new(
                Mutex::new(
                    CompilationStateInner::new(),
                ),
            ),
            cancellation,
        }
    }

    /// Returns whether the tracker has already been cancelled.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.cancellation.is_cancelled()
    }

    /// Returns whether cancellation has been requested.
    #[must_use]
    pub fn cancellation_requested(&self) -> bool {
        self.cancellation.is_cancelled()
    }

    /// Returns the associated cancellation token.
    ///
    /// Cloning this token shares its cancellation state.
    #[must_use]
    pub fn cancellation_token(&self) -> CancellationToken {
        self.cancellation.clone()
    }

    /// Returns an immutable snapshot of the current state.
    ///
    /// # Errors
    ///
    /// Returns SynchronizationPoisoned if the mutex is poisoned.
    pub fn snapshot(
        &self,
    ) -> Result<CompilationStateSnapshot, CompilationStateError> {
        let state = self
            .inner
            .lock()
            .map_err(|_| {
                CompilationStateError::SynchronizationPoisoned
            })?;

        Ok(state.snapshot())
    }

    /// Returns the current lifecycle state.
    pub fn lifecycle(
        &self,
    ) -> Result<CompilationLifecycle, CompilationStateError> {
        Ok(self.snapshot()?.lifecycle)
    }

    /// Starts compilation.
    ///
    /// The only permitted source lifecycle state is Created.
    ///
    /// Cancellation is checked before the transition.
    pub fn start(&self) -> Result<(), CompilationStateError> {
        let mut state = self.lock_state()?;

        state.ensure_non_terminal()?;

        if self.cancellation.is_cancelled() {
            return Err(
                CompilationStateError::CancellationRequested,
            );
        }

        if state.lifecycle != CompilationLifecycle::Created {
            return Err(
                CompilationStateError::InvalidTransition {
                    from: state.lifecycle,
                    to: CompilationLifecycle::Running,
                },
            );
        }

        let revision = state.next_revision()?;

        state.lifecycle = CompilationLifecycle::Running;
        state.phase = Some(CompilationPhase::Initialization);
        state.stage_name = Some(Arc::from("initialization"));
        state.revision = revision;

        Ok(())
    }

    /// Advances compilation to a named stage.
    ///
    /// The stage identifier is dynamically sized.
    ///
    /// No fixed number of stages is imposed.
    ///
    /// This operation does not implement or execute the stage.
    ///
    /// It records progress for pipeline orchestration and observers.
    ///
    /// The lifecycle must already be Running.
    pub fn advance(
        &self,
        phase: CompilationPhase,
        stage_name: impl Into<Arc<str>>,
    ) -> Result<(), CompilationStateError> {
        let stage_name = stage_name.into();

        if stage_name.trim().is_empty() {
            return Err(
                CompilationStateError::EmptyStageName,
            );
        }

        let mut state = self.lock_state()?;

        state.ensure_non_terminal()?;

        if self.cancellation.is_cancelled()
            || state.lifecycle
                == CompilationLifecycle::CancellationRequested
        {
            return Err(
                CompilationStateError::CancellationRequested,
            );
        }

        if state.lifecycle != CompilationLifecycle::Running {
            return Err(
                CompilationStateError::InvalidTransition {
                    from: state.lifecycle,
                    to: CompilationLifecycle::Running,
                },
            );
        }

        let revision = state.next_revision()?;

        state.phase = Some(phase);
        state.stage_name = Some(stage_name);
        state.revision = revision;

        Ok(())
    }

    /// Updates the active stage without changing its high-level phase.
    ///
    /// Useful for dynamically registered compiler extensions.
    ///
    /// The lifecycle must be Running.
    pub fn set_stage(
        &self,
        stage_name: impl Into<Arc<str>>,
    ) -> Result<(), CompilationStateError> {
        let stage_name = stage_name.into();

        if stage_name.trim().is_empty() {
            return Err(
                CompilationStateError::EmptyStageName,
            );
        }

        let mut state = self.lock_state()?;

        state.ensure_non_terminal()?;

        if self.cancellation.is_cancelled()
            || state.lifecycle
                == CompilationLifecycle::CancellationRequested
        {
            return Err(
                CompilationStateError::CancellationRequested,
            );
        }

        if state.lifecycle != CompilationLifecycle::Running {
            return Err(
                CompilationStateError::InvalidTransition {
                    from: state.lifecycle,
                    to: CompilationLifecycle::Running,
                },
            );
        }

        let revision = state.next_revision()?;

        state.stage_name = Some(stage_name);
        state.revision = revision;

        Ok(())
    }

    /// Requests cancellation.
    ///
    /// The existing cancellation token is notified first.
    ///
    /// Created becomes Cancelled.
    ///
    /// Running becomes CancellationRequested.
    ///
    /// A cancellation request against a terminal state is rejected.
    ///
    /// The actual cancellation of active work remains cooperative.
    pub fn request_cancel(
        &self,
    ) -> Result<(), CompilationStateError> {
        let mut state = self.lock_state()?;

        state.ensure_non_terminal()?;

        match state.lifecycle {
            CompilationLifecycle::Created => {
                let revision = state.next_revision()?;

                self.cancellation.cancel();

                state.lifecycle = CompilationLifecycle::Cancelled;
                state.phase = None;
                state.stage_name = None;
                state.revision = revision;
            }

            CompilationLifecycle::Running => {
                let revision = state.next_revision()?;

                self.cancellation.cancel();

                state.lifecycle =
                    CompilationLifecycle::CancellationRequested;

                state.revision = revision;
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
                    },
                );
            }
        }

        Ok(())
    }

    /// Marks a cancellation-requested operation as cancelled.
    ///
    /// This transition is separate from request_cancel because
    /// active compiler stages may need time to stop cooperatively.
    pub fn mark_cancelled(
        &self,
    ) -> Result<(), CompilationStateError> {
        let mut state = self.lock_state()?;

        state.ensure_non_terminal()?;

        if state.lifecycle != CompilationLifecycle::CancellationRequested
            && state.lifecycle != CompilationLifecycle::Created
            && !self.cancellation.is_cancelled()
        {
            return Err(
                CompilationStateError::InvalidTransition {
                    from: state.lifecycle,
                    to: CompilationLifecycle::Cancelled,
                },
            );
        }

        let revision = state.next_revision()?;

        self.cancellation.cancel();

        state.lifecycle = CompilationLifecycle::Cancelled;
        state.phase = None;
        state.stage_name = None;
        state.revision = revision;

        Ok(())
    }

    /// Completes compilation successfully.
    ///
    /// Successful completion is rejected if cancellation was requested.
    ///
    /// Only Running may transition to Completed.
    pub fn complete(
        &self,
    ) -> Result<(), CompilationStateError> {
        let mut state = self.lock_state()?;

        state.ensure_non_terminal()?;

        if self.cancellation.is_cancelled()
            || state.lifecycle
                == CompilationLifecycle::CancellationRequested
        {
            return Err(
                CompilationStateError::CancellationRequested,
            );
        }

        if state.lifecycle != CompilationLifecycle::Running {
            return Err(
                CompilationStateError::InvalidTransition {
                    from: state.lifecycle,
                    to: CompilationLifecycle::Completed,
                },
            );
        }

        let revision = state.next_revision()?;

        state.lifecycle = CompilationLifecycle::Completed;
        state.phase = Some(CompilationPhase::Finalization);
        state.stage_name = Some(Arc::from("completed"));
        state.revision = revision;

        Ok(())
    }

    /// Marks compilation as failed.
    ///
    /// Failure may occur before or during compilation.
    ///
    /// Terminal states cannot be overwritten.
    ///
    /// The structured diagnostic system remains authoritative
    /// for detailed diagnostic information.
    pub fn fail(
        &self,
        description: impl Into<Arc<str>>,
    ) -> Result<(), CompilationStateError> {
        let description = description.into();

        if description.trim().is_empty() {
            return Err(
                CompilationStateError::EmptyFailureDescription,
            );
        }

        let mut state = self.lock_state()?;

        state.ensure_non_terminal()?;

        let revision = state.next_revision()?;

        state.lifecycle = CompilationLifecycle::Failed;
        state.phase = None;
        state.stage_name = None;
        state.failure = Some(description);
        state.revision = revision;

        Ok(())
    }

    /// Checks whether compilation may continue.
    ///
    /// This combines lifecycle and cancellation inspection.
    pub fn check_active(
        &self,
    ) -> Result<(), CompilationStateError> {
        let state = self.lock_state()?;

        state.ensure_non_terminal()?;

        if self.cancellation.is_cancelled()
            || state.lifecycle
                == CompilationLifecycle::CancellationRequested
        {
            return Err(
                CompilationStateError::CancellationRequested,
            );
        }

        if state.lifecycle != CompilationLifecycle::Running {
            return Err(
                CompilationStateError::InvalidTransition {
                    from: state.lifecycle,
                    to: CompilationLifecycle::Running,
                },
            );
        }

        Ok(())
    }

    /// Returns the number of strong references to shared state.
    ///
    /// This is an observability method only.
    #[must_use]
    pub fn reference_count(&self) -> usize {
        Arc::strong_count(&self.inner)
    }

    /// Acquires the internal state lock.
    fn lock_state(
        &self,
    ) -> Result<
        std::sync::MutexGuard<'_, CompilationStateInner>,
        CompilationStateError,
    > {
        self.inner.lock().map_err(|_| {
            CompilationStateError::SynchronizationPoisoned
        })
    }
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
    fn new_tracker_starts_in_created_state() {
        let tracker = CompilationStateTracker::new();

        let snapshot = tracker
            .snapshot()
            .expect("snapshot should succeed");

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
            tracker.lifecycle().expect("lifecycle should be readable"),
            CompilationLifecycle::Created
        );
    }

    // ------------------------------------------------------------------------
    // Starting and progressing
    // ------------------------------------------------------------------------

    #[test]
    fn compilation_can_start() {
        let tracker = CompilationStateTracker::new();

        tracker.start().expect("start should succeed");

        assert_eq!(
            tracker.lifecycle().expect("state should be readable"),
            CompilationLifecycle::Running
        );
    }

    #[test]
    fn starting_sets_initialization_phase() {
        let tracker = CompilationStateTracker::new();

        tracker.start().expect("start should succeed");

        let snapshot = tracker
            .snapshot()
            .expect("snapshot should succeed");

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
    fn compilation_can_advance_through_phases() {
        let tracker = CompilationStateTracker::new();

        tracker.start().expect("start should succeed");

        tracker
            .advance(
                CompilationPhase::SemanticAnalysis,
                "semantic_validation",
            )
            .expect("phase advancement should succeed");

        let snapshot = tracker
            .snapshot()
            .expect("snapshot should succeed");

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
    fn custom_stage_names_are_supported() {
        let tracker = CompilationStateTracker::new();

        tracker.start().expect("start should succeed");

        tracker
            .set_stage("future_neuromorphic_lowering")
            .expect("custom stage should succeed");

        assert_eq!(
            tracker
                .snapshot()
                .expect("snapshot should succeed")
                .stage_name
                .as_deref(),
            Some("future_neuromorphic_lowering")
        );
    }

    #[test]
    fn unicode_stage_names_are_supported() {
        let tracker = CompilationStateTracker::new();

        tracker.start().expect("start should succeed");

        tracker
            .set_stage("量子混合编译")
            .expect("Unicode stage should succeed");

        assert_eq!(
            tracker
                .snapshot()
                .expect("snapshot should succeed")
                .stage_name
                .as_deref(),
            Some("量子混合编译")
        );
    }

    #[test]
    fn empty_stage_names_are_rejected() {
        let tracker = CompilationStateTracker::new();

        tracker.start().expect("start should succeed");

        let result = tracker.set_stage("   ");

        assert_eq!(
            result,
            Err(CompilationStateError::EmptyStageName)
        );
    }

    // ------------------------------------------------------------------------
    // Completion
    // ------------------------------------------------------------------------

    #[test]
    fn running_compilation_can_complete() {
        let tracker = CompilationStateTracker::new();

        tracker.start().expect("start should succeed");

        tracker
            .complete()
            .expect("completion should succeed");

        assert_eq!(
            tracker.lifecycle().expect("state should be readable"),
            CompilationLifecycle::Completed
        );
    }

    #[test]
    fn completed_state_is_terminal() {
        let tracker = CompilationStateTracker::new();

        tracker.start().expect("start should succeed");
        tracker.complete().expect("completion should succeed");

        let result = tracker.set_stage("another_stage");

        assert!(matches!(
            result,
            Err(CompilationStateError::TerminalState {
                state: CompilationLifecycle::Completed
            })
        ));
    }

    #[test]
    fn completed_compilation_cannot_be_completed_twice() {
        let tracker = CompilationStateTracker::new();

        tracker.start().expect("start should succeed");
        tracker.complete().expect("completion should succeed");

        assert!(matches!(
            tracker.complete(),
            Err(CompilationStateError::TerminalState {
                state: CompilationLifecycle::Completed
            })
        ));
    }

    #[test]
    fn compilation_cannot_complete_before_starting() {
        let tracker = CompilationStateTracker::new();

        assert!(matches!(
            tracker.complete(),
            Err(CompilationStateError::InvalidTransition { .. })
        ));
    }

    // ------------------------------------------------------------------------
    // Failure
    // ------------------------------------------------------------------------

    #[test]
    fn compilation_can_fail_before_starting() {
        let tracker = CompilationStateTracker::new();

        tracker
            .fail("configuration validation failed")
            .expect("failure should be recorded");

        let snapshot = tracker
            .snapshot()
            .expect("snapshot should succeed");

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
    fn compilation_can_fail_while_running() {
        let tracker = CompilationStateTracker::new();

        tracker.start().expect("start should succeed");

        tracker
            .fail("IR verification failed")
            .expect("failure should be recorded");

        assert_eq!(
            tracker.lifecycle().expect("state should be readable"),
            CompilationLifecycle::Failed
        );
    }

    #[test]
    fn failure_description_cannot_be_empty() {
        let tracker = CompilationStateTracker::new();

        assert_eq!(
            tracker.fail(" "),
            Err(CompilationStateError::EmptyFailureDescription)
        );
    }

    #[test]
    fn failure_cannot_overwrite_completed_state() {
        let tracker = CompilationStateTracker::new();

        tracker.start().expect("start should succeed");
        tracker.complete().expect("completion should succeed");

        assert!(matches!(
            tracker.fail("late failure"),
            Err(CompilationStateError::TerminalState {
                state: CompilationLifecycle::Completed
            })
        ));
    }

    // ------------------------------------------------------------------------
    // Cancellation
    // ------------------------------------------------------------------------

    #[test]
    fn cancellation_before_start_is_terminal() {
        let tracker = CompilationStateTracker::new();

        tracker
            .request_cancel()
            .expect("cancellation should succeed");

        assert_eq!(
            tracker.lifecycle().expect("state should be readable"),
            CompilationLifecycle::Cancelled
        );

        assert!(tracker.is_cancelled());
    }

    #[test]
    fn cancellation_during_compilation_is_requested() {
        let tracker = CompilationStateTracker::new();

        tracker.start().expect("start should succeed");

        tracker
            .request_cancel()
            .expect("cancellation request should succeed");

        assert_eq!(
            tracker.lifecycle().expect("state should be readable"),
            CompilationLifecycle::CancellationRequested
        );

        assert!(tracker.is_cancelled());
    }

    #[test]
    fn cancellation_must_be_acknowledged() {
        let tracker = CompilationStateTracker::new();

        tracker.start().expect("start should succeed");

        tracker
            .request_cancel()
            .expect("cancellation request should succeed");

        tracker
            .mark_cancelled()
            .expect("cancellation acknowledgement should succeed");

        assert_eq!(
            tracker.lifecycle().expect("state should be readable"),
            CompilationLifecycle::Cancelled
        );
    }

    #[test]
    fn cancellation_prevents_successful_completion() {
        let tracker = CompilationStateTracker::new();

        tracker.start().expect("start should succeed");

        tracker
            .request_cancel()
            .expect("cancellation request should succeed");

        assert_eq!(
            tracker.complete(),
            Err(CompilationStateError::CancellationRequested)
        );
    }

    #[test]
    fn cancellation_prevents_further_stage_advancement() {
        let tracker = CompilationStateTracker::new();

        tracker.start().expect("start should succeed");

        tracker
            .request_cancel()
            .expect("cancellation request should succeed");

        assert_eq!(
            tracker.set_stage("late_stage"),
            Err(CompilationStateError::CancellationRequested)
        );
    }

    #[test]
    fn cancellation_is_shared_with_clones() {
        let tracker = CompilationStateTracker::new();

        let clone = tracker.clone();

        tracker.start().expect("start should succeed");

        tracker
            .request_cancel()
            .expect("cancellation request should succeed");

        assert!(clone.is_cancelled());

        assert_eq!(
            clone.lifecycle().expect("state should be readable"),
            CompilationLifecycle::CancellationRequested
        );
    }

    #[test]
    fn external_cancellation_token_is_shared() {
        let token = CancellationToken::new();

        let tracker =
            CompilationStateTracker::with_cancellation(token.clone());

        tracker.start().expect("start should succeed");

        token.cancel();

        assert!(tracker.is_cancelled());

        assert_eq!(
            tracker.complete(),
            Err(CompilationStateError::CancellationRequested)
        );
    }

    // ------------------------------------------------------------------------
    // State transition safety
    // ------------------------------------------------------------------------

    #[test]
    fn terminal_states_cannot_be_reused() {
        let tracker = CompilationStateTracker::new();

        tracker.start().expect("start should succeed");
        tracker.complete().expect("completion should succeed");

        assert!(matches!(
            tracker.start(),
            Err(CompilationStateError::TerminalState { .. })
        ));
    }

    #[test]
    fn revision_increases_after_accepted_transitions() {
        let tracker = CompilationStateTracker::new();

        let initial = tracker.snapshot().expect("snapshot should succeed");

        tracker.start().expect("start should succeed");

        let started = tracker.snapshot().expect("snapshot should succeed");

        tracker
            .advance(
                CompilationPhase::Parsing,
                "source_parsing",
            )
            .expect("advance should succeed");

        let advanced = tracker.snapshot().expect("snapshot should succeed");

        tracker.complete().expect("completion should succeed");

        let completed = tracker.snapshot().expect("snapshot should succeed");

        assert_eq!(initial.revision, 0);
        assert!(started.revision > initial.revision);
        assert!(advanced.revision > started.revision);
        assert!(completed.revision > advanced.revision);
    }

    #[test]
    fn failed_transitions_do_not_modify_state() {
        let tracker = CompilationStateTracker::new();

        let before = tracker.snapshot().expect("snapshot should succeed");

        assert!(tracker.complete().is_err());

        let after = tracker.snapshot().expect("snapshot should succeed");

        assert_eq!(before, after);
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
    fn tracker_clones_share_state() {
        let tracker = CompilationStateTracker::new();

        let clone = tracker.clone();

        tracker.start().expect("start should succeed");

        assert_eq!(
            clone.lifecycle().expect("state should be readable"),
            CompilationLifecycle::Running
        );
    }

    #[test]
    fn cloned_tracker_reference_count_is_shared() {
        let tracker = CompilationStateTracker::new();

        let clone = tracker.clone();

        assert_eq!(tracker.reference_count(), 2);

        drop(clone);

        assert_eq!(tracker.reference_count(), 1);
    }

    // ------------------------------------------------------------------------
    // Snapshot behavior
    // ------------------------------------------------------------------------

    #[test]
    fn snapshots_are_independent_of_future_changes() {
        let tracker = CompilationStateTracker::new();

        tracker.start().expect("start should succeed");

        let previous = tracker.snapshot().expect("snapshot should succeed");

        tracker
            .advance(
                CompilationPhase::Parsing,
                "parser",
            )
            .expect("advance should succeed");

        let current = tracker.snapshot().expect("snapshot should succeed");

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
    fn snapshot_reports_success() {
        let tracker = CompilationStateTracker::new();

        tracker.start().expect("start should succeed");
        tracker.complete().expect("completion should succeed");

        let snapshot = tracker.snapshot().expect("snapshot should succeed");

        assert!(snapshot.is_terminal());
        assert!(snapshot.is_successful());
        assert!(!snapshot.is_active());
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
