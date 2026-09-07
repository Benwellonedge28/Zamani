//! Zamani Quantum Resilience — deterministic resilience state machine.
//!
//! Path:
//!     src/quantum/resilience/state/machine.rs
//!
//! Purpose:
//!     Owns the lifecycle state machine for a resilience execution.
//!
//! Architectural lifecycle:
//!
//!     Idle
//!       |
//!       v
//!     Detecting --> Diagnosing --> Planning --> Adapting --> Recovering --> Verifying
//!       |              |              |            |             |             |
//!       +------------> Escalated <----+------------+-------------+-------------+
//!       |              |
//!       +------------> Failed
//!
//! Successful execution normally follows:
//!
//!     Idle
//!       -> Detecting
//!       -> Diagnosing
//!       -> Planning
//!       -> Adapting
//!       -> Recovering
//!       -> Verifying
//!       -> Completed
//!
//! The machine also supports deterministic re-planning/recovery loops.
//!
//! -----------------------------------------------------------------------------
//! OWNERSHIP
//! -----------------------------------------------------------------------------
//!
//! This file owns:
//!
//! - lifecycle states;
//! - lifecycle events;
//! - legal lifecycle transitions;
//! - monotonic transition sequence numbers;
//! - optional caller-bounded transition history;
//! - transition validation;
//! - deterministic replay;
//! - immutable state snapshots.
//!
//! This file does NOT own:
//!
//! - quantum fault semantics;
//! - incident semantics;
//! - health semantics;
//! - severity;
//! - diagnosis;
//! - policy;
//! - recovery implementation;
//! - mitigation;
//! - QEC;
//! - routing;
//! - scheduling;
//! - compilation;
//! - optimization;
//! - hardware discovery;
//! - hardware execution;
//! - persistence;
//! - authorization;
//! - authentication;
//! - clocks;
//! - randomness;
//! - quantum identifiers.
//!
//! -----------------------------------------------------------------------------
//! WRITE ONCE / SCALE EVERYWHERE
//! -----------------------------------------------------------------------------
//!
//! The active state is constant-size.
//!
//! There are no:
//!
//! - maximum qubit constants;
//! - maximum device constants;
//! - provider-specific branches;
//! - retry constants;
//! - fixed-size resource arrays;
//! - topology assumptions;
//! - machine-size assumptions.
//!
//! Local history is explicitly caller-configured. A capacity of zero disables
//! local history and keeps only the active state and sequence number.
//!
//! Therefore a one-qubit execution and a distributed quantum execution use the
//! same state-machine representation.
//!
//! "Infinity" means that this module introduces no artificial quantum-machine
//! ceiling. Actual limits remain the available CPU, memory, storage and other
//! execution resources.
//!
//! -----------------------------------------------------------------------------
//! DETERMINISM
//! -----------------------------------------------------------------------------
//!
//! This module:
//!
//! - does not read the system clock;
//! - does not generate randomness;
//! - does not read environment variables;
//! - does not use process-global mutable state;
//! - does not depend on HashMap iteration;
//! - does not depend on thread completion order;
//! - does not silently wrap sequence arithmetic.
//!
//! Given the same initial state and ordered events, the resulting state and
//! transition sequence are identical.
//!
//! -----------------------------------------------------------------------------
//! SECURITY
//! -----------------------------------------------------------------------------
//!
//! This is a lifecycle-integrity mechanism, not an authorization mechanism.
//!
//! Higher layers MUST perform:
//!
//! - authentication;
//! - authorization;
//! - telemetry trust validation;
//! - policy validation;
//! - recovery-action safety validation.
//!
//! before requesting a transition.
//!
//! A state transition does not itself grant permission to perform hardware or
//! recovery operations.
//!
//! -----------------------------------------------------------------------------
//! INTEGRATION
//! -----------------------------------------------------------------------------
//!
//! model/*
//!     Provides normalized observations consumed by higher layers.
//!
//! detection/*
//!     Normally requests:
//!
//!         Idle -> Detecting
//!         Detecting -> Diagnosing
//!
//! diagnosis/*
//!     Normally requests:
//!
//!         Detecting -> Diagnosing
//!         Diagnosing -> Planning
//!
//! planning/*
//!     Normally requests:
//!
//!         Diagnosing -> Planning
//!         Planning -> Adapting
//!         Planning -> Planning       (re-plan)
//!
//! adaptation/*
//!     Normally requests:
//!
//!         Adapting -> Recovering
//!         Adapting -> Planning
//!         Adapting -> Detecting
//!
//! recovery/*
//!     Performs actual recovery and records:
//!
//!         Recovering -> Verifying
//!
//! verification/*
//!     Determines whether to:
//!
//!         Verifying -> Completed
//!         Verifying -> Planning
//!         Verifying -> Detecting
//!         Verifying -> Escalated
//!         Verifying -> Failed
//!
//! telemetry/*
//!     May observe transition records.
//!
//! state/persistence.rs
//!     May persist snapshots/records.
//!
//! serialization/*
//!     May serialize these public values.
//!
//! `quantum::ir::qubit::{QubitId, PhysicalQubitId}` are intentionally NOT
//! imported here. This lifecycle is resource-agnostic. Resource identity
//! belongs to `quantum::ir::qubit` and the resilience resource integration
//! boundary, not to the lifecycle state machine.
//!
//! -----------------------------------------------------------------------------
//! RUST
//! -----------------------------------------------------------------------------
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;
use std::collections::VecDeque;

/// Stable schema identifier for the resilience state machine.
pub const STATE_MACHINE_SCHEMA_ID: &str =
    "zamani.quantum.resilience.state.machine";

/// Durable semantic schema version.
///
/// Increment when the externally observable representation or semantics
/// require compatibility handling.
pub const STATE_MACHINE_SCHEMA_VERSION: u16 = 1;

/// Implementation version.
///
/// This is independent from the durable schema version.
pub const STATE_MACHINE_VERSION: u32 = 1;

// ============================================================================
// RESILIENCE STATE
// ============================================================================

/// Canonical resilience lifecycle state.
///
/// This describes lifecycle position only.
///
/// It is NOT:
///
/// - a health state;
/// - a severity;
/// - a probability;
/// - an authorization level;
/// - a recovery strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResilienceState {
    /// No resilience lifecycle has started.
    Idle,

    /// Evidence is being collected and normalized.
    Detecting,

    /// Evidence is being interpreted and a diagnosis is being formed.
    Diagnosing,

    /// A resilience/recovery plan is being constructed or selected.
    Planning,

    /// A selected plan is being applied to the execution representation.
    Adapting,

    /// A recovery operation is being performed.
    Recovering,

    /// Execution/result correctness is being evaluated.
    Verifying,

    /// The lifecycle completed with an accepted result.
    Completed,

    /// Automatic processing stopped and requires higher-level intervention.
    Escalated,

    /// The lifecycle terminated without an accepted result.
    Failed,
}

impl ResilienceState {
    /// Returns the stable machine-readable representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Detecting => "detecting",
            Self::Diagnosing => "diagnosing",
            Self::Planning => "planning",
            Self::Adapting => "adapting",
            Self::Recovering => "recovering",
            Self::Verifying => "verifying",
            Self::Completed => "completed",
            Self::Escalated => "escalated",
            Self::Failed => "failed",
        }
    }

    /// Returns a stable ordinal for deterministic serialization/order.
    ///
    /// This is not a severity or quality score.
    #[must_use]
    pub const fn ordinal(self) -> u8 {
        match self {
            Self::Idle => 0,
            Self::Detecting => 1,
            Self::Diagnosing => 2,
            Self::Planning => 3,
            Self::Adapting => 4,
            Self::Recovering => 5,
            Self::Verifying => 6,
            Self::Completed => 7,
            Self::Escalated => 8,
            Self::Failed => 9,
        }
    }

    /// Converts a stable ordinal to a state.
    ///
    /// Unknown values are rejected rather than silently mapped.
    #[must_use]
    pub const fn from_ordinal(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Idle),
            1 => Some(Self::Detecting),
            2 => Some(Self::Diagnosing),
            3 => Some(Self::Planning),
            4 => Some(Self::Adapting),
            5 => Some(Self::Recovering),
            6 => Some(Self::Verifying),
            7 => Some(Self::Completed),
            8 => Some(Self::Escalated),
            9 => Some(Self::Failed),
            _ => None,
        }
    }

    /// Returns whether the state is terminal.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Completed | Self::Escalated | Self::Failed
        )
    }

    /// Returns whether lifecycle processing is still active.
    #[must_use]
    pub const fn is_active(self) -> bool {
        !self.is_terminal()
    }

    /// Returns whether this state belongs to an active recovery lifecycle.
    #[must_use]
    pub const fn is_recoverable_phase(self) -> bool {
        matches!(
            self,
            Self::Detecting
                | Self::Diagnosing
                | Self::Planning
                | Self::Adapting
                | Self::Recovering
                | Self::Verifying
        )
    }
}

impl fmt::Display for ResilienceState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// RESILIENCE EVENT
// ============================================================================

/// Semantic lifecycle event.
///
/// Events describe what has happened or what lifecycle decision has been
/// established. They do not implement the underlying operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResilienceEvent {
    /// Begin observation/detection.
    StartDetection,

    /// Detection completed and produced evidence requiring diagnosis.
    DetectionComplete,

    /// Diagnosis completed and planning is required.
    DiagnosisComplete,

    /// A plan has been selected and adaptation is required.
    PlanReady,

    /// Adaptation completed and recovery may execute.
    AdaptationComplete,

    /// Recovery completed and verification is required.
    RecoveryComplete,

    /// Verification accepted the result.
    VerificationAccepted,

    /// Verification requires another recovery cycle.
    VerificationRequiresRecovery,

    /// Automatic processing cannot safely continue.
    Escalate,

    /// Processing terminated unsuccessfully.
    Fail,

    /// The current plan must be reconsidered.
    Replan,

    /// A new detection cycle is required.
    Redetect,
}

impl ResilienceEvent {
    /// Returns the stable machine-readable representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StartDetection => "start_detection",
            Self::DetectionComplete => "detection_complete",
            Self::DiagnosisComplete => "diagnosis_complete",
            Self::PlanReady => "plan_ready",
            Self::AdaptationComplete => "adaptation_complete",
            Self::RecoveryComplete => "recovery_complete",
            Self::VerificationAccepted => "verification_accepted",
            Self::VerificationRequiresRecovery => {
                "verification_requires_recovery"
            }
            Self::Escalate => "escalate",
            Self::Fail => "fail",
            Self::Replan => "replan",
            Self::Redetect => "redetect",
        }
    }
}

impl fmt::Display for ResilienceEvent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// TRANSITION
// ============================================================================

/// Immutable record of one successful lifecycle transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Transition {
    sequence: u64,
    from: ResilienceState,
    event: ResilienceEvent,
    to: ResilienceState,
}

impl Transition {
    const fn new(
        sequence: u64,
        from: ResilienceState,
        event: ResilienceEvent,
        to: ResilienceState,
    ) -> Self {
        Self {
            sequence,
            from,
            event,
            to,
        }
    }

    /// Returns the monotonically increasing transition sequence.
    #[must_use]
    pub const fn sequence(self) -> u64 {
        self.sequence
    }

    /// Returns the state before the transition.
    #[must_use]
    pub const fn from(self) -> ResilienceState {
        self.from
    }

    /// Returns the event that caused the transition.
    #[must_use]
    pub const fn event(self) -> ResilienceEvent {
        self.event
    }

    /// Returns the state after the transition.
    #[must_use]
    pub const fn to(self) -> ResilienceState {
        self.to
    }
}

// ============================================================================
// SNAPSHOT
// ============================================================================

/// Immutable active-state snapshot.
///
/// This snapshot contains no:
///
/// - credentials;
/// - provider configuration;
/// - pointers;
/// - timestamps;
/// - random identifiers;
/// - hardware addresses.
///
/// It is therefore suitable as an input to persistence/serialization layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StateMachineSnapshot {
    state: ResilienceState,
    sequence: u64,
    history_capacity: usize,
}

impl StateMachineSnapshot {
    /// Returns the current lifecycle state.
    #[must_use]
    pub const fn state(self) -> ResilienceState {
        self.state
    }

    /// Returns the last transition sequence.
    #[must_use]
    pub const fn sequence(self) -> u64 {
        self.sequence
    }

    /// Returns the configured local history capacity.
    #[must_use]
    pub const fn history_capacity(self) -> usize {
        self.history_capacity
    }
}

// ============================================================================
// ERROR
// ============================================================================

/// Error returned when a lifecycle operation cannot be applied safely.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateMachineError {
    /// The event is not legal from the current state.
    InvalidTransition {
        from: ResilienceState,
        event: ResilienceEvent,
    },

    /// The sequence number cannot advance without wrapping.
    SequenceOverflow,

    /// A replay record has a non-contiguous sequence number.
    ReplaySequenceMismatch {
        expected: u64,
        actual: u64,
    },

    /// A replay record does not match the canonical transition graph.
    ReplayTransitionMismatch {
        expected_from: ResilienceState,
        actual_from: ResilienceState,
        event: ResilienceEvent,
        expected_to: ResilienceState,
        actual_to: ResilienceState,
    },
}

impl fmt::Display for StateMachineError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTransition { from, event } => write!(
                formatter,
                "invalid resilience transition: state={from}, event={event}"
            ),

            Self::SequenceOverflow => formatter.write_str(
                "resilience state-machine sequence overflow",
            ),

            Self::ReplaySequenceMismatch { expected, actual } => write!(
                formatter,
                "resilience replay sequence mismatch: expected={expected}, actual={actual}"
            ),

            Self::ReplayTransitionMismatch {
                expected_from,
                actual_from,
                event,
                expected_to,
                actual_to,
            } => write!(
                formatter,
                "resilience replay transition mismatch: \
                 event={event}, expected={expected_from}->{expected_to}, \
                 actual={actual_from}->{actual_to}"
            ),
        }
    }
}

impl std::error::Error for StateMachineError {}

// ============================================================================
// STATE MACHINE
// ============================================================================

/// Deterministic resilience lifecycle state machine.
///
/// The active state is constant-size.
///
/// Local history is optional and bounded by an explicitly supplied capacity.
///
/// A capacity of zero disables local history and is appropriate when the caller
/// streams transitions to telemetry or persistence instead of buffering them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResilienceStateMachine {
    state: ResilienceState,
    sequence: u64,
    history: VecDeque<Transition>,
    history_capacity: usize,
}

impl Default for ResilienceStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl ResilienceStateMachine {
    /// Creates a new state machine in `Idle` with no local history.
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: ResilienceState::Idle,
            sequence: 0,
            history: VecDeque::new(),
            history_capacity: 0,
        }
    }

    /// Creates a new state machine with caller-selected local history.
    ///
    /// The capacity is a memory-management decision, not a quantum-machine
    /// limit.
    #[must_use]
    pub fn with_history_capacity(history_capacity: usize) -> Self {
        Self {
            state: ResilienceState::Idle,
            sequence: 0,
            history: VecDeque::with_capacity(history_capacity),
            history_capacity,
        }
    }

    /// Restores active state from an explicit snapshot.
    ///
    /// The caller remains responsible for authenticity and authorization of
    /// persisted state.
    #[must_use]
    pub const fn from_snapshot(snapshot: StateMachineSnapshot) -> Self {
        Self {
            state: snapshot.state,
            sequence: snapshot.sequence,
            history: VecDeque::new(),
            history_capacity: snapshot.history_capacity,
        }
    }

    /// Returns the current lifecycle state.
    #[must_use]
    pub const fn state(&self) -> ResilienceState {
        self.state
    }

    /// Returns the last applied transition sequence.
    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    /// Returns whether the machine is terminal.
    #[must_use]
    pub const fn is_terminal(&self) -> bool {
        self.state.is_terminal()
    }

    /// Returns whether the machine is still active.
    #[must_use]
    pub const fn is_active(&self) -> bool {
        self.state.is_active()
    }

    /// Returns the configured local history capacity.
    #[must_use]
    pub const fn history_capacity(&self) -> usize {
        self.history_capacity
    }

    /// Returns the number of locally retained transitions.
    #[must_use]
    pub fn history_len(&self) -> usize {
        self.history.len()
    }

    /// Returns the most recently retained transition.
    #[must_use]
    pub fn last_transition(&self) -> Option<Transition> {
        self.history.back().copied()
    }

    /// Returns retained transitions in chronological sequence order.
    #[must_use]
    pub fn history(&self) -> &VecDeque<Transition> {
        &self.history
    }

    /// Creates an immutable active-state snapshot.
    #[must_use]
    pub const fn snapshot(&self) -> StateMachineSnapshot {
        StateMachineSnapshot {
            state: self.state,
            sequence: self.sequence,
            history_capacity: self.history_capacity,
        }
    }

    /// Changes local history capacity.
    ///
    /// If the new capacity is smaller than the current history, the oldest
    /// records are discarded deterministically.
    ///
    /// Active state and sequence are never changed.
    pub fn set_history_capacity(&mut self, history_capacity: usize) {
        self.history_capacity = history_capacity;

        if history_capacity == 0 {
            self.history.clear();
            return;
        }

        while self.history.len() > history_capacity {
            let _ = self.history.pop_front();
        }
    }

    /// Returns whether an event is legal from the current state.
    #[must_use]
    pub const fn can_apply(&self, event: ResilienceEvent) -> bool {
        Self::next_state(self.state, event).is_some()
    }

    /// Computes the destination state without mutating the machine.
    ///
    /// This is the authoritative transition graph.
    #[must_use]
    pub const fn next_state(
        state: ResilienceState,
        event: ResilienceEvent,
    ) -> Option<ResilienceState> {
        use ResilienceEvent as E;
        use ResilienceState as S;

        match (state, event) {
            // ----------------------------------------------------------------
            // Start
            // ----------------------------------------------------------------
            (S::Idle, E::StartDetection) => Some(S::Detecting),

            // ----------------------------------------------------------------
            // Detection
            // ----------------------------------------------------------------
            (S::Detecting, E::DetectionComplete) => Some(S::Diagnosing),
            (S::Detecting, E::Escalate) => Some(S::Escalated),
            (S::Detecting, E::Fail) => Some(S::Failed),

            // ----------------------------------------------------------------
            // Diagnosis
            // ----------------------------------------------------------------
            (S::Diagnosing, E::DiagnosisComplete) => Some(S::Planning),
            (S::Diagnosing, E::Escalate) => Some(S::Escalated),
            (S::Diagnosing, E::Fail) => Some(S::Failed),

            // ----------------------------------------------------------------
            // Planning
            // ----------------------------------------------------------------
            (S::Planning, E::PlanReady) => Some(S::Adapting),
            (S::Planning, E::Replan) => Some(S::Planning),
            (S::Planning, E::Escalate) => Some(S::Escalated),
            (S::Planning, E::Fail) => Some(S::Failed),

            // ----------------------------------------------------------------
            // Adaptation
            // ----------------------------------------------------------------
            (S::Adapting, E::AdaptationComplete) => Some(S::Recovering),
            (S::Adapting, E::Replan) => Some(S::Planning),
            (S::Adapting, E::Redetect) => Some(S::Detecting),
            (S::Adapting, E::Escalate) => Some(S::Escalated),
            (S::Adapting, E::Fail) => Some(S::Failed),

            // ----------------------------------------------------------------
            // Recovery
            // ----------------------------------------------------------------
            (S::Recovering, E::RecoveryComplete) => Some(S::Verifying),
            (S::Recovering, E::Replan) => Some(S::Planning),
            (S::Recovering, E::Redetect) => Some(S::Detecting),
            (S::Recovering, E::Escalate) => Some(S::Escalated),
            (S::Recovering, E::Fail) => Some(S::Failed),

            // ----------------------------------------------------------------
            // Verification
            // ----------------------------------------------------------------
            (S::Verifying, E::VerificationAccepted) => Some(S::Completed),
            (S::Verifying, E::VerificationRequiresRecovery) => {
                Some(S::Planning)
            }
            (S::Verifying, E::Replan) => Some(S::Planning),
            (S::Verifying, E::Redetect) => Some(S::Detecting),
            (S::Verifying, E::Escalate) => Some(S::Escalated),
            (S::Verifying, E::Fail) => Some(S::Failed),

            // ----------------------------------------------------------------
            // Terminal states are absorbing.
            //
            // A completed, escalated, or failed lifecycle must not be
            // resurrected by an ordinary lifecycle event. A new execution
            // should create a new state-machine instance.
            // ----------------------------------------------------------------
            (S::Completed | S::Escalated | S::Failed, _) => None,

            // All other combinations are structurally invalid.
            _ => None,
        }
    }

    /// Applies one lifecycle event.
    ///
    /// This operation is atomic with respect to the machine:
    ///
    /// - invalid transition => no mutation;
    /// - sequence overflow => no mutation;
    /// - valid transition => state, sequence and optional history advance.
    pub fn apply(
        &mut self,
        event: ResilienceEvent,
    ) -> Result<Transition, StateMachineError> {
        let from = self.state;

        let to = Self::next_state(from, event).ok_or(
            StateMachineError::InvalidTransition { from, event },
        )?;

        let sequence = self
            .sequence
            .checked_add(1)
            .ok_or(StateMachineError::SequenceOverflow)?;

        let transition = Transition::new(sequence, from, event, to);

        self.state = to;
        self.sequence = sequence;
        self.record(transition);

        Ok(transition)
    }

    /// Applies multiple events transactionally.
    ///
    /// If any event is invalid, the original machine remains unchanged.
    ///
    /// The caller owns the input collection, so this method does not introduce
    /// a hidden lifecycle-size limit.
    pub fn apply_all<I>(
        &mut self,
        events: I,
    ) -> Result<Vec<Transition>, StateMachineError>
    where
        I: IntoIterator<Item = ResilienceEvent>,
    {
        let events: Vec<ResilienceEvent> = events.into_iter().collect();

        let mut candidate = self.clone();
        let mut transitions = Vec::with_capacity(events.len());

        for event in events {
            transitions.push(candidate.apply(event)?);
        }

        *self = candidate;

        Ok(transitions)
    }

    /// Replays a sequence of previously recorded transitions.
    ///
    /// Replay always starts from `Idle`.
    ///
    /// Every supplied transition is checked against:
    ///
    /// - contiguous sequence numbering;
    /// - the canonical transition graph;
    /// - the recorded source state;
    /// - the recorded destination state.
    ///
    /// A failed replay leaves the original machine unchanged.
    pub fn replay<I>(
        &mut self,
        transitions: I,
    ) -> Result<(), StateMachineError>
    where
        I: IntoIterator<Item = Transition>,
    {
        let transitions: Vec<Transition> = transitions.into_iter().collect();

        let mut candidate =
            Self::with_history_capacity(self.history_capacity);

        for expected in transitions {
            let actual_sequence = candidate
                .sequence
                .checked_add(1)
                .ok_or(StateMachineError::SequenceOverflow)?;

            if expected.sequence != actual_sequence {
                return Err(
                    StateMachineError::ReplaySequenceMismatch {
                        expected: actual_sequence,
                        actual: expected.sequence,
                    },
                );
            }

            let actual = candidate.apply(expected.event)?;

            if actual != expected {
                return Err(
                    StateMachineError::ReplayTransitionMismatch {
                        expected_from: expected.from,
                        actual_from: actual.from,
                        event: expected.event,
                        expected_to: expected.to,
                        actual_to: actual.to,
                    },
                );
            }
        }

        *self = candidate;

        Ok(())
    }

    /// Explicitly resets the lifecycle.
    ///
    /// Reset is deliberately not represented as a normal lifecycle event.
    /// Terminal states therefore cannot be accidentally resurrected by event
    /// processing.
    ///
    /// For independent executions, creating a new state-machine instance is
    /// normally preferable.
    pub fn reset(&mut self) {
        self.state = ResilienceState::Idle;
        self.sequence = 0;
        self.history.clear();
    }

    fn record(&mut self, transition: Transition) {
        if self.history_capacity == 0 {
            return;
        }

        self.history.push_back(transition);

        while self.history.len() > self.history_capacity {
            let _ = self.history.pop_front();
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_idle() {
        let machine = ResilienceStateMachine::new();

        assert_eq!(machine.state(), ResilienceState::Idle);
        assert_eq!(machine.sequence(), 0);
        assert!(!machine.is_terminal());
        assert!(machine.is_active());
    }

    #[test]
    fn happy_path_is_deterministic() {
        let mut machine =
            ResilienceStateMachine::with_history_capacity(32);

        let events = [
            ResilienceEvent::StartDetection,
            ResilienceEvent::DetectionComplete,
            ResilienceEvent::DiagnosisComplete,
            ResilienceEvent::PlanReady,
            ResilienceEvent::AdaptationComplete,
            ResilienceEvent::RecoveryComplete,
            ResilienceEvent::VerificationAccepted,
        ];

        for (index, event) in events.into_iter().enumerate() {
            let transition =
                machine.apply(event).expect("transition must be valid");

            assert_eq!(transition.sequence(), (index as u64) + 1);
        }

        assert_eq!(
            machine.state(),
            ResilienceState::Completed
        );
        assert!(machine.is_terminal());
        assert!(!machine.is_active());
        assert_eq!(machine.history_len(), 7);
    }

    #[test]
    fn invalid_transition_does_not_mutate() {
        let mut machine = ResilienceStateMachine::new();

        let result =
            machine.apply(ResilienceEvent::DiagnosisComplete);

        assert!(matches!(
            result,
            Err(StateMachineError::InvalidTransition {
                from: ResilienceState::Idle,
                event: ResilienceEvent::DiagnosisComplete,
            })
        ));

        assert_eq!(machine.state(), ResilienceState::Idle);
        assert_eq!(machine.sequence(), 0);
        assert_eq!(machine.history_len(), 0);
    }

    #[test]
    fn terminal_states_are_absorbing() {
        let mut machine = ResilienceStateMachine::new();

        machine
            .apply(ResilienceEvent::StartDetection)
            .expect("valid");

        machine
            .apply(ResilienceEvent::Fail)
            .expect("valid");

        assert_eq!(machine.state(), ResilienceState::Failed);
        assert!(!machine.can_apply(ResilienceEvent::StartDetection));

        assert!(matches!(
            machine.apply(ResilienceEvent::StartDetection),
            Err(StateMachineError::InvalidTransition {
                from: ResilienceState::Failed,
                event: ResilienceEvent::StartDetection,
            })
        ));
    }

    #[test]
    fn history_is_bounded() {
        let mut machine =
            ResilienceStateMachine::with_history_capacity(2);

        machine
            .apply_all([
                ResilienceEvent::StartDetection,
                ResilienceEvent::DetectionComplete,
                ResilienceEvent::DiagnosisComplete,
            ])
            .expect("valid lifecycle");

        assert_eq!(machine.history_len(), 2);
        assert_eq!(machine.history()[0].sequence(), 2);
        assert_eq!(machine.history()[1].sequence(), 3);
        assert_eq!(machine.sequence(), 3);
        assert_eq!(machine.state(), ResilienceState::Planning);
    }

    #[test]
    fn zero_capacity_disables_history() {
        let mut machine = ResilienceStateMachine::new();

        machine
            .apply(ResilienceEvent::StartDetection)
            .expect("valid");

        assert_eq!(machine.history_len(), 0);
        assert!(machine.last_transition().is_none());
        assert_eq!(machine.sequence(), 1);
        assert_eq!(
            machine.state(),
            ResilienceState::Detecting
        );
    }

    #[test]
    fn capacity_can_be_changed_without_changing_state() {
        let mut machine =
            ResilienceStateMachine::with_history_capacity(4);

        machine
            .apply_all([
                ResilienceEvent::StartDetection,
                ResilienceEvent::DetectionComplete,
                ResilienceEvent::DiagnosisComplete,
            ])
            .expect("valid lifecycle");

        machine.set_history_capacity(1);

        assert_eq!(machine.state(), ResilienceState::Planning);
        assert_eq!(machine.sequence(), 3);
        assert_eq!(machine.history_len(), 1);
        assert_eq!(machine.history()[0].sequence(), 3);
    }

    #[test]
    fn transactional_apply_all_rolls_back_on_error() {
        let mut machine = ResilienceStateMachine::new();

        let result = machine.apply_all([
            ResilienceEvent::StartDetection,
            ResilienceEvent::DiagnosisComplete,
        ]);

        assert!(result.is_err());
        assert_eq!(machine.state(), ResilienceState::Idle);
        assert_eq!(machine.sequence(), 0);
        assert_eq!(machine.history_len(), 0);
    }

    #[test]
    fn replay_reconstructs_same_state() {
        let mut original =
            ResilienceStateMachine::with_history_capacity(16);

        original
            .apply_all([
                ResilienceEvent::StartDetection,
                ResilienceEvent::DetectionComplete,
                ResilienceEvent::DiagnosisComplete,
                ResilienceEvent::PlanReady,
                ResilienceEvent::AdaptationComplete,
                ResilienceEvent::RecoveryComplete,
            ])
            .expect("valid lifecycle");

        let records: Vec<Transition> =
            original.history().iter().copied().collect();

        let mut replayed =
            ResilienceStateMachine::with_history_capacity(16);

        replayed
            .replay(records)
            .expect("replay must succeed");

        assert_eq!(replayed.state(), original.state());
        assert_eq!(replayed.sequence(), original.sequence());
        assert_eq!(replayed.history(), original.history());
    }

    #[test]
    fn invalid_replay_is_transactional() {
        let mut machine =
            ResilienceStateMachine::with_history_capacity(8);

        machine
            .apply(ResilienceEvent::StartDetection)
            .expect("valid");

        let original = machine.clone();

        let invalid = Transition::new(
            99,
            ResilienceState::Detecting,
            ResilienceEvent::DetectionComplete,
            ResilienceState::Diagnosing,
        );

        assert!(machine.replay([invalid]).is_err());

        assert_eq!(machine, original);
    }

    #[test]
    fn replan_does_not_require_retry_constants() {
        let mut machine = ResilienceStateMachine::new();

        machine
            .apply_all([
                ResilienceEvent::StartDetection,
                ResilienceEvent::DetectionComplete,
                ResilienceEvent::DiagnosisComplete,
                ResilienceEvent::PlanReady,
                ResilienceEvent::Replan,
            ])
            .expect("valid replan");

        assert_eq!(machine.state(), ResilienceState::Planning);
        assert_eq!(machine.sequence(), 5);
    }

    #[test]
    fn verification_can_request_another_recovery_cycle() {
        let mut machine = ResilienceStateMachine::new();

        machine
            .apply_all([
                ResilienceEvent::StartDetection,
                ResilienceEvent::DetectionComplete,
                ResilienceEvent::DiagnosisComplete,
                ResilienceEvent::PlanReady,
                ResilienceEvent::AdaptationComplete,
                ResilienceEvent::RecoveryComplete,
                ResilienceEvent::VerificationRequiresRecovery,
            ])
            .expect("valid recovery cycle");

        assert_eq!(machine.state(), ResilienceState::Planning);
    }

    #[test]
    fn verification_can_request_redetection() {
        let mut machine = ResilienceStateMachine::new();

        machine
            .apply_all([
                ResilienceEvent::StartDetection,
                ResilienceEvent::DetectionComplete,
                ResilienceEvent::DiagnosisComplete,
                ResilienceEvent::PlanReady,
                ResilienceEvent::AdaptationComplete,
                ResilienceEvent::RecoveryComplete,
                ResilienceEvent::Redetect,
            ])
            .expect("valid redetection");

        assert_eq!(machine.state(), ResilienceState::Detecting);
    }

    #[test]
    fn adaptation_can_return_to_planning() {
        let mut machine = ResilienceStateMachine::new();

        machine
            .apply_all([
                ResilienceEvent::StartDetection,
                ResilienceEvent::DetectionComplete,
                ResilienceEvent::DiagnosisComplete,
                ResilienceEvent::PlanReady,
                ResilienceEvent::Replan,
            ])
            .expect("valid replanning");

        assert_eq!(machine.state(), ResilienceState::Planning);
    }

    #[test]
    fn snapshot_contains_only_deterministic_state() {
        let mut machine =
            ResilienceStateMachine::with_history_capacity(8);

        machine
            .apply(ResilienceEvent::StartDetection)
            .expect("valid");

        let snapshot = machine.snapshot();

        assert_eq!(
            snapshot.state(),
            ResilienceState::Detecting
        );
        assert_eq!(snapshot.sequence(), 1);
        assert_eq!(snapshot.history_capacity(), 8);
    }

    #[test]
    fn snapshot_restore_preserves_active_state() {
        let mut machine =
            ResilienceStateMachine::with_history_capacity(8);

        machine
            .apply_all([
                ResilienceEvent::StartDetection,
                ResilienceEvent::DetectionComplete,
                ResilienceEvent::DiagnosisComplete,
            ])
            .expect("valid");

        let restored =
            ResilienceStateMachine::from_snapshot(machine.snapshot());

        assert_eq!(restored.state(), machine.state());
        assert_eq!(restored.sequence(), machine.sequence());

        // Snapshot restoration deliberately does not claim to restore history.
        assert_eq!(restored.history_len(), 0);
        assert_eq!(
            restored.history_capacity(),
            machine.history_capacity()
        );
    }

    #[test]
    fn reset_returns_to_initial_state() {
        let mut machine =
            ResilienceStateMachine::with_history_capacity(8);

        machine
            .apply_all([
                ResilienceEvent::StartDetection,
                ResilienceEvent::DetectionComplete,
                ResilienceEvent::DiagnosisComplete,
            ])
            .expect("valid");

        machine.reset();

        assert_eq!(machine.state(), ResilienceState::Idle);
        assert_eq!(machine.sequence(), 0);
        assert_eq!(machine.history_len(), 0);
        assert!(!machine.is_terminal());
    }

    #[test]
    fn state_ordinals_are_round_trip_stable() {
        let states = [
            ResilienceState::Idle,
            ResilienceState::Detecting,
            ResilienceState::Diagnosing,
            ResilienceState::Planning,
            ResilienceState::Adapting,
            ResilienceState::Recovering,
            ResilienceState::Verifying,
            ResilienceState::Completed,
            ResilienceState::Escalated,
            ResilienceState::Failed,
        ];

        for state in states {
            assert_eq!(
                ResilienceState::from_ordinal(state.ordinal()),
                Some(state)
            );
        }
    }

    #[test]
    fn event_names_are_non_empty_and_stable() {
        let events = [
            ResilienceEvent::StartDetection,
            ResilienceEvent::DetectionComplete,
            ResilienceEvent::DiagnosisComplete,
            ResilienceEvent::PlanReady,
            ResilienceEvent::AdaptationComplete,
            ResilienceEvent::RecoveryComplete,
            ResilienceEvent::VerificationAccepted,
            ResilienceEvent::VerificationRequiresRecovery,
            ResilienceEvent::Escalate,
            ResilienceEvent::Fail,
            ResilienceEvent::Replan,
            ResilienceEvent::Redetect,
        ];

        for event in events {
            assert!(!event.as_str().is_empty());
        }
    }

    #[test]
    fn every_nonterminal_state_has_defined_behavior_for_terminal_events() {
        let states = [
            ResilienceState::Idle,
            ResilienceState::Detecting,
            ResilienceState::Diagnosing,
            ResilienceState::Planning,
            ResilienceState::Adapting,
            ResilienceState::Recovering,
            ResilienceState::Verifying,
        ];

        for state in states {
            assert_eq!(
                ResilienceStateMachine::next_state(
                    state,
                    ResilienceEvent::Escalate
                ),
                match state {
                    ResilienceState::Idle => None,
                    ResilienceState::Detecting
                    | ResilienceState::Diagnosing
                    | ResilienceState::Planning
                    | ResilienceState::Adapting
                    | ResilienceState::Recovering
                    | ResilienceState::Verifying => {
                        Some(ResilienceState::Escalated)
                    }
                    _ => None,
                }
            );
        }
    }

    #[test]
    fn schema_identifiers_are_stable() {
        assert_eq!(
            STATE_MACHINE_SCHEMA_ID,
            "zamani.quantum.resilience.state.machine"
        );
        assert_eq!(STATE_MACHINE_SCHEMA_VERSION, 1);
        assert_eq!(STATE_MACHINE_VERSION, 1);
    }
}