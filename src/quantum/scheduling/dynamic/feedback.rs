//! Zamani Quantum Scheduling — Dynamic Feedback
//!
//! Production-grade modelling of measurement-driven and classical feedback
//! dependencies for dynamic quantum scheduling.
//!
//! # Responsibility
//!
//! This module represents the scheduler-visible contract between:
//!
//! ```text
//! quantum measurement / classical producer
//!                  |
//!                  v
//!          classical signal
//!                  |
//!                  v
//!        classical processing
//!                  |
//!                  v
//!          feedback readiness
//!                  |
//!                  v
//!       conditional quantum work
//! ```
//!
//! The module answers:
//!
//! - what feedback depends on;
//! - when feedback becomes eligible;
//! - which classical signals must be available;
//! - which predicates must be evaluated;
//! - which quantum operation or runtime action is released;
//! - whether feedback is compile-time resolvable or runtime dependent;
//! - what target-supplied latency must be respected;
//! - whether feedback has become stale, cancelled, superseded, or invalid.
//!
//! # Non-responsibilities
//!
//! This module does NOT:
//!
//! - parse Zamani source;
//! - execute classical programs;
//! - execute quantum operations;
//! - perform routing;
//! - own hardware calendars;
//! - discover QPU capabilities;
//! - perform QEC decoding;
//! - define a competing QuantumOperation;
//! - define a competing QubitId or PhysicalQubitId;
//! - choose the global scheduling algorithm;
//! - communicate directly with hardware;
//! - invent hardware timing values.
//!
//! Those responsibilities remain in the appropriate subsystems.
//!
//! # Canonical quantum identities
//!
//! When feedback is associated with qubits, canonical identities MUST come
//! from:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module does not define replacement qubit identities.
//!
//! # Integration
//!
//! The intended dependency direction is:
//!
//! ```text
//! quantum::ir
//!      |
//!      v
//! dynamic::classical
//!      |
//!      v
//! dynamic::feedback
//!      |
//!      +------------------+
//!      |                  |
//!      v                  v
//! dynamic::conditional   dynamic::runtime
//!      |                  |
//!      +--------+---------+
//!               |
//!               v
//! scheduling::planner
//!               |
//!               v
//! scheduling::verification
//! ```
//!
//! Hardware timing, resource availability and runtime events are supplied by
//! their respective adapters. Feedback only records the requirements and
//! readiness state.
//!
//! # Scalability
//!
//! No fixed maximum is imposed on:
//!
//! - feedback nodes;
//! - signals;
//! - predicates;
//! - consumers;
//! - qubits;
//! - dependencies;
//! - event generations;
//! - feedback chains;
//! - distributed endpoints.
//!
//! Collections are dynamically sized and allocation failures are allowed to
//! propagate normally through Rust's standard allocation behaviour.
//!
//! "Infinity" therefore means no artificial scheduler-imposed machine-size
//! ceiling. Actual executions remain bounded by available memory, CPU time,
//! explicit policy limits, operating-system resources and target resources.
//!
//! # Determinism
//!
//! The feedback graph uses ordered maps and sets. When the same state and
//! inputs are supplied, traversal and exported collections have deterministic
//! ordering.
//!
//! No wall-clock time is consulted.
//! No global mutable state is used.
//! No implicit randomness is used.
//!
//! # Safety
//!
//! Rust 1.97 / Rust 1.97.1.
//! Rust 2021.
//! Stable Rust.
//! No nightly features.
//! No unsafe code.
//!
//! `#![forbid(unsafe_code)]` makes the requirement compiler-enforced.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use super::classical::{
    ClassicalEventId,
    ClassicalLatency,
    ClassicalNodeId,
    ClassicalSignalId,
    ClassicalValue,
    PredicateId,
};

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

// ============================================================================
// Feedback identity
// ============================================================================

/// Stable identity of a scheduler-visible feedback dependency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FeedbackId(u64);

impl FeedbackId {
    /// Creates a feedback identifier.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying identifier.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Returns whether the identifier is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }
}

impl From<u64> for FeedbackId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<FeedbackId> for u64 {
    fn from(value: FeedbackId) -> Self {
        value.value()
    }
}

impl fmt::Display for FeedbackId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "feedback:{}", self.0)
    }
}

// ============================================================================
// Consumer identity
// ============================================================================

/// Identifies an entity released by feedback.
///
/// The scheduler does not require the consumer to be a particular IR type.
/// This keeps feedback independent from the exact canonical IR layout.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum FeedbackConsumer {
    /// A scheduler operation identified by an external stable identifier.
    Operation(u64),

    /// A classical computation node.
    ClassicalNode(ClassicalNodeId),

    /// A predicate evaluation.
    Predicate(PredicateId),

    /// A runtime event.
    RuntimeEvent(ClassicalEventId),

    /// A user/plugin-defined consumer.
    Custom(String),
}

impl FeedbackConsumer {
    /// Returns whether this consumer is an operation.
    #[must_use]
    pub const fn is_operation(&self) -> bool {
        matches!(self, Self::Operation(_))
    }

    /// Returns whether this consumer is runtime-controlled.
    #[must_use]
    pub const fn is_runtime_event(&self) -> bool {
        matches!(self, Self::RuntimeEvent(_))
    }
}

// ============================================================================
// Feedback phase
// ============================================================================

/// Phase of feedback processing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum FeedbackPhase {
    /// Waiting for all required producer signals.
    AwaitingSignals,

    /// Classical processing is ready to begin.
    ProcessingReady,

    /// Classical processing is executing.
    Processing,

    /// Predicate evaluation is required.
    AwaitingPredicate,

    /// Predicate has resolved and the consumer may be released.
    Ready,

    /// Feedback is intentionally deferred until runtime.
    RuntimeDeferred,

    /// Feedback has been superseded by a newer generation.
    Superseded,

    /// Feedback has been cancelled.
    Cancelled,

    /// Feedback encountered an unrecoverable failure.
    Failed,
}

impl FeedbackPhase {
    /// Returns whether the feedback can release its consumer.
    #[must_use]
    pub const fn is_ready(self) -> bool {
        matches!(self, Self::Ready)
    }

    /// Returns whether processing is still pending.
    #[must_use]
    pub const fn is_pending(self) -> bool {
        matches!(
            self,
            Self::AwaitingSignals
                | Self::ProcessingReady
                | Self::Processing
                | Self::AwaitingPredicate
                | Self::RuntimeDeferred
        )
    }

    /// Returns whether the feedback is terminal.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Ready | Self::Superseded | Self::Cancelled | Self::Failed
        )
    }
}

// ============================================================================
// Predicate outcome
// ============================================================================

/// Result of resolving feedback conditions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum FeedbackOutcome {
    /// Condition resolved to true and the consumer is enabled.
    Enabled,

    /// Condition resolved to false and the consumer is disabled.
    Disabled,

    /// Condition cannot yet be resolved.
    Pending,

    /// The condition was intentionally deferred to runtime.
    RuntimeDeferred,
}

impl FeedbackOutcome {
    /// Returns whether the consumer is enabled.
    #[must_use]
    pub const fn is_enabled(self) -> bool {
        matches!(self, Self::Enabled)
    }

    /// Returns whether the outcome is known.
    #[must_use]
    pub const fn is_resolved(self) -> bool {
        matches!(self, Self::Enabled | Self::Disabled)
    }
}

// ============================================================================
// Feedback condition
// ============================================================================

/// Condition that must resolve before a feedback consumer can be released.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum FeedbackCondition {
    /// All referenced signals must be available.
    AllSignals,

    /// At least one referenced signal must be available.
    AnySignal,

    /// A predicate supplied by the classical subsystem must resolve.
    Predicate(PredicateId),

    /// A specific signal must equal a specific value.
    SignalEquals {
        /// Signal to inspect.
        signal: ClassicalSignalId,

        /// Required value.
        value: ClassicalValue,
    },

    /// A specific signal must differ from a value.
    SignalNotEquals {
        /// Signal to inspect.
        signal: ClassicalSignalId,

        /// Forbidden value.
        value: ClassicalValue,
    },

    /// Runtime must decide the condition.
    Runtime,

    /// User/plugin-defined condition.
    Custom(String),
}

impl FeedbackCondition {
    /// Returns the classical signals directly referenced by this condition.
    #[must_use]
    pub fn referenced_signals(&self) -> Vec<ClassicalSignalId> {
        match self {
            Self::SignalEquals { signal, .. }
            | Self::SignalNotEquals { signal, .. } => vec![*signal],
            Self::AllSignals
            | Self::AnySignal
            | Self::Predicate(_)
            | Self::Runtime
            | Self::Custom(_) => Vec::new(),
        }
    }

    /// Returns the predicate when the condition directly references one.
    #[must_use]
    pub const fn predicate(&self) -> Option<PredicateId> {
        match self {
            Self::Predicate(id) => Some(*id),
            _ => None,
        }
    }

    /// Returns whether the condition must be resolved by the runtime.
    #[must_use]
    pub const fn is_runtime_deferred(&self) -> bool {
        matches!(self, Self::Runtime)
    }
}

// ============================================================================
// Feedback source
// ============================================================================

/// Source that initiated the feedback chain.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum FeedbackSource {
    /// A classical signal became available.
    Signal(ClassicalSignalId),

    /// A classical computation completed.
    Computation(ClassicalNodeId),

    /// A predicate became evaluable.
    Predicate(PredicateId),

    /// A runtime event produced feedback.
    RuntimeEvent(ClassicalEventId),

    /// A quantum measurement produced the originating result.
    Measurement {
        /// Logical qubits associated with the measurement.
        logical_qubits: Vec<QubitId>,

        /// Physical qubits when routing has resolved them.
        physical_qubits: Vec<PhysicalQubitId>,
    },

    /// Distributed classical feedback.
    Distributed {
        /// Source endpoint.
        source: String,

        /// Destination endpoint.
        destination: String,
    },

    /// User/plugin-defined source.
    Custom(String),
}

impl FeedbackSource {
    /// Validates source-specific invariants.
    pub fn validate(&self) -> Result<(), FeedbackError> {
        match self {
            Self::Measurement {
                logical_qubits,
                physical_qubits,
            } => {
                if logical_qubits.is_empty() {
                    return Err(FeedbackError::EmptyMeasurementQubitSet);
                }

                if contains_duplicate(logical_qubits) {
                    return Err(FeedbackError::DuplicateLogicalQubit);
                }

                if contains_duplicate(physical_qubits) {
                    return Err(FeedbackError::DuplicatePhysicalQubit);
                }

                Ok(())
            }

            Self::Distributed {
                source,
                destination,
            } => {
                if source.is_empty() {
                    return Err(FeedbackError::EmptyDistributedSource);
                }

                if destination.is_empty() {
                    return Err(FeedbackError::EmptyDistributedDestination);
                }

                Ok(())
            }

            Self::Signal(_)
            | Self::Computation(_)
            | Self::Predicate(_)
            | Self::RuntimeEvent(_)
            | Self::Custom(_) => Ok(()),
        }
    }
}

// ============================================================================
// Feedback dependency
// ============================================================================

/// One dependency that must become satisfied before feedback can progress.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeedbackDependency {
    /// Dependency signal.
    signal: ClassicalSignalId,

    /// Optional producer node.
    producer: Option<ClassicalNodeId>,

    /// Optional producing runtime event.
    event: Option<ClassicalEventId>,

    /// Target-supplied readiness latency.
    latency: ClassicalLatency,

    /// Whether this dependency must be known at compile time.
    compile_time_required: bool,
}

impl FeedbackDependency {
    /// Creates a dependency on a classical signal.
    #[must_use]
    pub fn new(
        signal: ClassicalSignalId,
        producer: Option<ClassicalNodeId>,
        event: Option<ClassicalEventId>,
        latency: ClassicalLatency,
        compile_time_required: bool,
    ) -> Self {
        Self {
            signal,
            producer,
            event,
            latency,
            compile_time_required,
        }
    }

    /// Returns the signal.
    #[must_use]
    pub const fn signal(&self) -> ClassicalSignalId {
        self.signal
    }

    /// Returns the producer node, if known.
    #[must_use]
    pub const fn producer(&self) -> Option<ClassicalNodeId> {
        self.producer
    }

    /// Returns the event, if known.
    #[must_use]
    pub const fn event(&self) -> Option<ClassicalEventId> {
        self.event
    }

    /// Returns the latency.
    #[must_use]
    pub fn latency(&self) -> &ClassicalLatency {
        &self.latency
    }

    /// Returns whether compile-time resolution is required.
    #[must_use]
    pub const fn compile_time_required(&self) -> bool {
        self.compile_time_required
    }

    /// Validates this dependency.
    pub fn validate(&self) -> Result<(), FeedbackError> {
        if self.signal.is_zero() {
            return Err(FeedbackError::ZeroSignalId);
        }

        Ok(())
    }
}

// ============================================================================
// Feedback readiness
// ============================================================================

/// Current readiness information for a feedback dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeedbackReadiness {
    /// Current phase.
    phase: FeedbackPhase,

    /// Resolved outcome, if any.
    outcome: FeedbackOutcome,

    /// Signals that are currently available.
    available_signals: BTreeSet<ClassicalSignalId>,

    /// Signals still required.
    missing_signals: BTreeSet<ClassicalSignalId>,

    /// Signals whose values remain symbolic or runtime-only.
    unresolved_signals: BTreeSet<ClassicalSignalId>,

    /// Reason for the current state.
    reason: Option<String>,
}

impl FeedbackReadiness {
    /// Creates an initial readiness state.
    #[must_use]
    pub fn new(required: &BTreeSet<ClassicalSignalId>) -> Self {
        Self {
            phase: if required.is_empty() {
                FeedbackPhase::ProcessingReady
            } else {
                FeedbackPhase::AwaitingSignals
            },
            outcome: if required.is_empty() {
                FeedbackOutcome::Pending
            } else {
                FeedbackOutcome::Pending
            },
            available_signals: BTreeSet::new(),
            missing_signals: required.clone(),
            unresolved_signals: BTreeSet::new(),
            reason: None,
        }
    }

    /// Returns the current phase.
    #[must_use]
    pub const fn phase(&self) -> FeedbackPhase {
        self.phase
    }

    /// Returns the current outcome.
    #[must_use]
    pub const fn outcome(&self) -> FeedbackOutcome {
        self.outcome
    }

    /// Returns available signals.
    #[must_use]
    pub fn available_signals(&self) -> &BTreeSet<ClassicalSignalId> {
        &self.available_signals
    }

    /// Returns missing signals.
    #[must_use]
    pub fn missing_signals(&self) -> &BTreeSet<ClassicalSignalId> {
        &self.missing_signals
    }

    /// Returns unresolved signals.
    #[must_use]
    pub fn unresolved_signals(&self) -> &BTreeSet<ClassicalSignalId> {
        &self.unresolved_signals
    }

    /// Returns the current explanation.
    #[must_use]
    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    fn refresh(&mut self, required: &BTreeSet<ClassicalSignalId>) {
        self.missing_signals = required
            .difference(&self.available_signals)
            .copied()
            .collect();

        if self.missing_signals.is_empty() && self.unresolved_signals.is_empty() {
            self.phase = FeedbackPhase::ProcessingReady;
            self.outcome = FeedbackOutcome::Pending;
            self.reason = None;
        } else if !self.unresolved_signals.is_empty() {
            self.phase = FeedbackPhase::RuntimeDeferred;
            self.outcome = FeedbackOutcome::RuntimeDeferred;
            self.reason = Some(
                "one or more required classical values are unresolved at scheduling time"
                    .to_owned(),
            );
        } else {
            self.phase = FeedbackPhase::AwaitingSignals;
            self.outcome = FeedbackOutcome::Pending;
            self.reason = Some("waiting for required classical signals".to_owned());
        }
    }
}

// ============================================================================
// Feedback event
// ============================================================================

/// Runtime event emitted by feedback processing.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum FeedbackEvent {
    /// A signal became available.
    SignalAvailable(ClassicalSignalId),

    /// Classical processing became ready.
    ProcessingReady(FeedbackId),

    /// Classical processing started.
    ProcessingStarted(FeedbackId),

    /// Classical processing completed.
    ProcessingCompleted(FeedbackId),

    /// Predicate evaluation became ready.
    PredicateReady(PredicateId),

    /// Feedback released a consumer.
    ConsumerReleased(FeedbackConsumer),

    /// Feedback disabled a consumer.
    ConsumerDisabled(FeedbackConsumer),

    /// Feedback became runtime-dependent.
    RuntimeDeferred(FeedbackId),

    /// Feedback was cancelled.
    Cancelled(FeedbackId),

    /// Feedback was superseded.
    Superseded(FeedbackId),

    /// Feedback failed.
    Failed(FeedbackId),
}

// ============================================================================
// Feedback generation
// ============================================================================

/// Generation number for repeated or iterative feedback.
///
/// Generations allow loops and repeated runtime rounds without reusing a
/// semantic feedback identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FeedbackGeneration(u64);

impl FeedbackGeneration {
    /// Initial generation.
    #[must_use]
    pub const fn initial() -> Self {
        Self(0)
    }

    /// Creates an explicit generation.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the generation number.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Advances to the next generation without overflowing.
    pub fn next(self) -> Result<Self, FeedbackError> {
        self.0
            .checked_add(1)
            .map(Self)
            .ok_or(FeedbackError::GenerationOverflow)
    }
}

// ============================================================================
// Feedback request
// ============================================================================

/// Immutable description of one feedback dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeedbackRequest {
    id: FeedbackId,
    generation: FeedbackGeneration,
    source: FeedbackSource,
    dependencies: Vec<FeedbackDependency>,
    condition: FeedbackCondition,
    consumer: FeedbackConsumer,
    latency: ClassicalLatency,
    runtime_allowed: bool,
    cancellable: bool,
    metadata: BTreeMap<String, String>,
}

impl FeedbackRequest {
    /// Creates a feedback request.
    #[must_use]
    pub fn new(
        id: FeedbackId,
        generation: FeedbackGeneration,
        source: FeedbackSource,
        dependencies: Vec<FeedbackDependency>,
        condition: FeedbackCondition,
        consumer: FeedbackConsumer,
        latency: ClassicalLatency,
    ) -> Self {
        Self {
            id,
            generation,
            source,
            dependencies,
            condition,
            consumer,
            latency,
            runtime_allowed: true,
            cancellable: true,
            metadata: BTreeMap::new(),
        }
    }

    /// Returns the feedback identifier.
    #[must_use]
    pub const fn id(&self) -> FeedbackId {
        self.id
    }

    /// Returns the generation.
    #[must_use]
    pub const fn generation(&self) -> FeedbackGeneration {
        self.generation
    }

    /// Returns the source.
    #[must_use]
    pub fn source(&self) -> &FeedbackSource {
        &self.source
    }

    /// Returns dependencies.
    #[must_use]
    pub fn dependencies(&self) -> &[FeedbackDependency] {
        &self.dependencies
    }

    /// Returns the condition.
    #[must_use]
    pub fn condition(&self) -> &FeedbackCondition {
        &self.condition
    }

    /// Returns the consumer.
    #[must_use]
    pub fn consumer(&self) -> &FeedbackConsumer {
        &self.consumer
    }

    /// Returns feedback latency.
    #[must_use]
    pub fn latency(&self) -> &ClassicalLatency {
        &self.latency
    }

    /// Returns whether runtime resolution is allowed.
    #[must_use]
    pub const fn runtime_allowed(&self) -> bool {
        self.runtime_allowed
    }

    /// Returns whether the request may be cancelled.
    #[must_use]
    pub const fn cancellable(&self) -> bool {
        self.cancellable
    }

    /// Returns metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Enables or disables runtime resolution.
    pub const fn with_runtime_allowed(mut self, allowed: bool) -> Self {
        self.runtime_allowed = allowed;
        self
    }

    /// Enables or disables cancellation.
    pub const fn with_cancellable(mut self, cancellable: bool) -> Self {
        self.cancellable = cancellable;
        self
    }

    /// Adds deterministic metadata.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Returns all directly referenced signal identities.
    #[must_use]
    pub fn required_signals(&self) -> BTreeSet<ClassicalSignalId> {
        self.dependencies
            .iter()
            .map(FeedbackDependency::signal)
            .chain(self.condition.referenced_signals())
            .collect()
    }

    /// Validates all request invariants.
    pub fn validate(&self) -> Result<(), FeedbackError> {
        if self.id.is_zero() {
            return Err(FeedbackError::ZeroFeedbackId);
        }

        self.source.validate()?;

        let mut seen = BTreeSet::new();

        for dependency in &self.dependencies {
            dependency.validate()?;

            if !seen.insert(dependency.signal()) {
                return Err(FeedbackError::DuplicateDependency(
                    dependency.signal(),
                ));
            }
        }

        match &self.condition {
            FeedbackCondition::SignalEquals { signal, .. }
            | FeedbackCondition::SignalNotEquals { signal, .. } => {
                if signal.is_zero() {
                    return Err(FeedbackError::ZeroSignalId);
                }
            }
            FeedbackCondition::Predicate(predicate) => {
                if predicate.value() == 0 {
                    return Err(FeedbackError::ZeroPredicateId);
                }
            }
            FeedbackCondition::AllSignals
            | FeedbackCondition::AnySignal
            | FeedbackCondition::Runtime
            | FeedbackCondition::Custom(_) => {}
        }

        if matches!(self.condition, FeedbackCondition::Runtime) && !self.runtime_allowed {
            return Err(FeedbackError::RuntimeResolutionForbidden);
        }

        Ok(())
    }
}

// ============================================================================
// Feedback state
// ============================================================================

/// Mutable scheduler state for one feedback request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeedbackState {
    request: FeedbackRequest,
    phase: FeedbackPhase,
    readiness: FeedbackReadiness,
    values: BTreeMap<ClassicalSignalId, ClassicalValue>,
    completed_events: BTreeSet<ClassicalEventId>,
    cancelled: bool,
    superseded: bool,
    outcome: FeedbackOutcome,
}

impl FeedbackState {
    /// Creates state from a validated request.
    pub fn new(request: FeedbackRequest) -> Result<Self, FeedbackError> {
        request.validate()?;

        let required = request.required_signals();

        Ok(Self {
            request,
            phase: if required.is_empty() {
                FeedbackPhase::ProcessingReady
            } else {
                FeedbackPhase::AwaitingSignals
            },
            readiness: FeedbackReadiness::new(&required),
            values: BTreeMap::new(),
            completed_events: BTreeSet::new(),
            cancelled: false,
            superseded: false,
            outcome: FeedbackOutcome::Pending,
        })
    }

    /// Returns the immutable request.
    #[must_use]
    pub fn request(&self) -> &FeedbackRequest {
        &self.request
    }

    /// Returns the current phase.
    #[must_use]
    pub const fn phase(&self) -> FeedbackPhase {
        self.phase
    }

    /// Returns readiness details.
    #[must_use]
    pub fn readiness(&self) -> &FeedbackReadiness {
        &self.readiness
    }

    /// Returns the resolved values.
    #[must_use]
    pub fn values(&self) -> &BTreeMap<ClassicalSignalId, ClassicalValue> {
        &self.values
    }

    /// Returns the current outcome.
    #[must_use]
    pub const fn outcome(&self) -> FeedbackOutcome {
        self.outcome
    }

    /// Returns whether feedback has been cancelled.
    #[must_use]
    pub const fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    /// Returns whether feedback has been superseded.
    #[must_use]
    pub const fn is_superseded(&self) -> bool {
        self.superseded
    }

    /// Supplies a classical value.
    ///
    /// This method only updates scheduler-visible state. It does not execute
    /// or derive the value.
    pub fn provide_value(
        &mut self,
        signal: ClassicalSignalId,
        value: ClassicalValue,
    ) -> Result<FeedbackEvent, FeedbackError> {
        self.ensure_mutable()?;

        if signal.is_zero() {
            return Err(FeedbackError::ZeroSignalId);
        }

        let required = self.request.required_signals();

        if !required.contains(&signal) {
            return Err(FeedbackError::UnexpectedSignal(signal));
        }

        if self.values.insert(signal, value).is_some() {
            return Err(FeedbackError::DuplicateSignalValue(signal));
        }

        self.readiness.available_signals.insert(signal);
        self.readiness.unresolved_signals.remove(&signal);
        self.readiness.refresh(&required);

        if self.readiness.missing_signals.is_empty()
            && self.readiness.unresolved_signals.is_empty()
        {
            self.phase = FeedbackPhase::ProcessingReady;
        }

        Ok(FeedbackEvent::SignalAvailable(signal))
    }

    /// Marks a signal as runtime-unresolved.
    pub fn mark_runtime_unresolved(
        &mut self,
        signal: ClassicalSignalId,
    ) -> Result<FeedbackEvent, FeedbackError> {
        self.ensure_mutable()?;

        let required = self.request.required_signals();

        if !required.contains(&signal) {
            return Err(FeedbackError::UnexpectedSignal(signal));
        }

        self.readiness.unresolved_signals.insert(signal);
        self.readiness.refresh(&required);
        self.phase = FeedbackPhase::RuntimeDeferred;

        if !self.request.runtime_allowed() {
            return Err(FeedbackError::RuntimeResolutionForbidden);
        }

        Ok(FeedbackEvent::RuntimeDeferred(self.request.id()))
    }

    /// Records completion of a producer runtime event.
    pub fn complete_event(
        &mut self,
        event: ClassicalEventId,
    ) -> Result<(), FeedbackError> {
        self.ensure_mutable()?;

        if event.value() == 0 {
            return Err(FeedbackError::ZeroEventId);
        }

        self.completed_events.insert(event);

        Ok(())
    }

    /// Marks classical processing as started.
    pub fn processing_started(&mut self) -> Result<FeedbackEvent, FeedbackError> {
        self.ensure_mutable()?;

        if !self.readiness.missing_signals.is_empty() {
            return Err(FeedbackError::NotReady {
                reason: "classical processing cannot begin before required signals are available"
                    .to_owned(),
            });
        }

        if !self.readiness.unresolved_signals.is_empty() {
            return Err(FeedbackError::RuntimeValueUnavailable);
        }

        self.phase = FeedbackPhase::Processing;

        Ok(FeedbackEvent::ProcessingStarted(self.request.id()))
    }

    /// Marks classical processing as complete.
    pub fn processing_completed(&mut self) -> Result<FeedbackEvent, FeedbackError> {
        self.ensure_mutable()?;

        if self.phase != FeedbackPhase::Processing
            && self.phase != FeedbackPhase::ProcessingReady
        {
            return Err(FeedbackError::InvalidPhaseTransition {
                from: self.phase,
                to: FeedbackPhase::AwaitingPredicate,
            });
        }

        self.phase = match self.request.condition() {
            FeedbackCondition::Runtime => FeedbackPhase::RuntimeDeferred,
            FeedbackCondition::Predicate(_) => FeedbackPhase::AwaitingPredicate,
            _ => FeedbackPhase::AwaitingPredicate,
        };

        Ok(FeedbackEvent::ProcessingCompleted(self.request.id()))
    }

    /// Records predicate evaluation readiness.
    pub fn predicate_ready(
        &mut self,
        predicate: PredicateId,
    ) -> Result<FeedbackEvent, FeedbackError> {
        self.ensure_mutable()?;

        match self.request.condition().predicate() {
            Some(expected) if expected == predicate => {
                self.phase = FeedbackPhase::AwaitingPredicate;
                Ok(FeedbackEvent::PredicateReady(predicate))
            }
            Some(_) => Err(FeedbackError::UnexpectedPredicate(predicate)),
            None => Err(FeedbackError::PredicateNotRequired),
        }
    }

    /// Resolves the feedback condition from the currently known values.
    pub fn resolve(
        &mut self,
        outcome: FeedbackOutcome,
    ) -> Result<FeedbackEvent, FeedbackError> {
        self.ensure_mutable()?;

        match outcome {
            FeedbackOutcome::Pending => {
                return Err(FeedbackError::InvalidOutcome);
            }

            FeedbackOutcome::RuntimeDeferred => {
                if !self.request.runtime_allowed() {
                    return Err(FeedbackError::RuntimeResolutionForbidden);
                }

                self.phase = FeedbackPhase::RuntimeDeferred;
                self.outcome = FeedbackOutcome::RuntimeDeferred;

                Ok(FeedbackEvent::RuntimeDeferred(self.request.id()))
            }

            FeedbackOutcome::Enabled => {
                self.phase = FeedbackPhase::Ready;
                self.outcome = FeedbackOutcome::Enabled;

                Ok(FeedbackEvent::ConsumerReleased(
                    self.request.consumer().clone(),
                ))
            }

            FeedbackOutcome::Disabled => {
                self.phase = FeedbackPhase::Ready;
                self.outcome = FeedbackOutcome::Disabled;

                Ok(FeedbackEvent::ConsumerDisabled(
                    self.request.consumer().clone(),
                ))
            }
        }
    }

    /// Cancels the feedback request.
    pub fn cancel(&mut self) -> Result<FeedbackEvent, FeedbackError> {
        if !self.request.cancellable() {
            return Err(FeedbackError::CancellationForbidden);
        }

        if self.phase.is_terminal() {
            return Err(FeedbackError::AlreadyTerminal);
        }

        self.cancelled = true;
        self.phase = FeedbackPhase::Cancelled;

        Ok(FeedbackEvent::Cancelled(self.request.id()))
    }

    /// Marks the feedback as superseded by another generation.
    pub fn supersede(&mut self) -> Result<FeedbackEvent, FeedbackError> {
        if self.phase.is_terminal() {
            return Err(FeedbackError::AlreadyTerminal);
        }

        self.superseded = true;
        self.phase = FeedbackPhase::Superseded;

        Ok(FeedbackEvent::Superseded(self.request.id()))
    }

    fn ensure_mutable(&self) -> Result<(), FeedbackError> {
        if self.cancelled {
            return Err(FeedbackError::AlreadyCancelled);
        }

        if self.superseded {
            return Err(FeedbackError::AlreadySuperseded);
        }

        if self.phase.is_terminal() {
            return Err(FeedbackError::AlreadyTerminal);
        }

        Ok(())
    }
}

// ============================================================================
// Feedback graph
// ============================================================================

/// Collection of feedback states.
///
/// This is intentionally an owned graph rather than a global registry.
/// It can therefore be embedded into a scheduler, runtime, simulation or
/// distributed execution context.
#[derive(Debug, Clone, Default)]
pub struct FeedbackGraph {
    states: BTreeMap<FeedbackId, FeedbackState>,
    consumers: BTreeMap<FeedbackConsumer, BTreeSet<FeedbackId>>,
    signals: BTreeMap<ClassicalSignalId, BTreeSet<FeedbackId>>,
}

impl FeedbackGraph {
    /// Creates an empty graph.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of feedback states.
    #[must_use]
    pub fn len(&self) -> usize {
        self.states.len()
    }

    /// Returns whether the graph is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.states.is_empty()
    }

    /// Inserts a validated feedback state.
    pub fn insert(&mut self, state: FeedbackState) -> Result<(), FeedbackError> {
        let id = state.request().id();

        if self.states.contains_key(&id) {
            return Err(FeedbackError::DuplicateFeedbackId(id));
        }

        for signal in state.request().required_signals() {
            self.signals.entry(signal).or_default().insert(id);
        }

        self.consumers
            .entry(state.request().consumer().clone())
            .or_default()
            .insert(id);

        self.states.insert(id, state);

        Ok(())
    }

    /// Returns a feedback state.
    #[must_use]
    pub fn get(&self, id: FeedbackId) -> Option<&FeedbackState> {
        self.states.get(&id)
    }

    /// Returns a mutable feedback state.
    #[must_use]
    pub fn get_mut(&mut self, id: FeedbackId) -> Option<&mut FeedbackState> {
        self.states.get_mut(&id)
    }

    /// Returns feedback states waiting on a signal.
    #[must_use]
    pub fn waiting_on(
        &self,
        signal: ClassicalSignalId,
    ) -> impl Iterator<Item = &FeedbackState> {
        self.signals
            .get(&signal)
            .into_iter()
            .flat_map(|ids| ids.iter())
            .filter_map(|id| self.states.get(id))
    }

    /// Returns feedback IDs associated with a consumer.
    #[must_use]
    pub fn feedback_for_consumer(
        &self,
        consumer: &FeedbackConsumer,
    ) -> impl Iterator<Item = FeedbackId> + '_ {
        self.consumers
            .get(consumer)
            .into_iter()
            .flat_map(|ids| ids.iter())
            .copied()
    }

    /// Supplies a signal value to all feedback requests waiting on it.
    ///
    /// The returned events are deterministically ordered by FeedbackId.
    pub fn provide_signal(
        &mut self,
        signal: ClassicalSignalId,
        value: ClassicalValue,
    ) -> Result<Vec<FeedbackEvent>, FeedbackError> {
        if signal.is_zero() {
            return Err(FeedbackError::ZeroSignalId);
        }

        let ids: Vec<FeedbackId> = self
            .signals
            .get(&signal)
            .map(|values| values.iter().copied().collect())
            .unwrap_or_default();

        let mut events = Vec::with_capacity(ids.len());

        for id in ids {
            if let Some(state) = self.states.get_mut(&id) {
                if !state.phase().is_terminal() {
                    events.push(state.provide_value(signal, value.clone())?);
                }
            }
        }

        Ok(events)
    }

    /// Cancels a feedback state.
    pub fn cancel(&mut self, id: FeedbackId) -> Result<FeedbackEvent, FeedbackError> {
        self.states
            .get_mut(&id)
            .ok_or(FeedbackError::UnknownFeedback(id))?
            .cancel()
    }

    /// Supersedes a feedback state.
    pub fn supersede(&mut self, id: FeedbackId) -> Result<FeedbackEvent, FeedbackError> {
        self.states
            .get_mut(&id)
            .ok_or(FeedbackError::UnknownFeedback(id))?
            .supersede()
    }

    /// Returns all currently ready feedback states.
    #[must_use]
    pub fn ready(&self) -> impl Iterator<Item = &FeedbackState> {
        self.states
            .values()
            .filter(|state| state.phase() == FeedbackPhase::Ready)
    }

    /// Validates the complete graph.
    pub fn validate(&self) -> Result<(), FeedbackError> {
        for (id, state) in &self.states {
            if *id != state.request().id() {
                return Err(FeedbackError::GraphIdentityMismatch);
            }

            state.request().validate()?;
        }

        Ok(())
    }
}

// ============================================================================
// Feedback scheduler adapter contract
// ============================================================================

/// Target-independent interface used by a scheduling engine to consume
/// feedback readiness.
///
/// Implementations should remain lightweight. The actual scheduler owns
/// resource reservation and operation timing.
pub trait FeedbackSchedulerView {
    /// Returns whether a feedback consumer is currently eligible.
    fn is_consumer_released(&self, consumer: &FeedbackConsumer) -> bool;

    /// Returns the feedback state.
    fn feedback_state(&self, id: FeedbackId) -> Option<&FeedbackState>;

    /// Returns all ready feedback states.
    fn ready_feedback(&self) -> Vec<FeedbackId>;
}

/// Default view over [`FeedbackGraph`].
impl FeedbackSchedulerView for FeedbackGraph {
    fn is_consumer_released(&self, consumer: &FeedbackConsumer) -> bool {
        self.consumers
            .get(consumer)
            .map(|ids| {
                ids.iter().any(|id| {
                    self.states
                        .get(id)
                        .is_some_and(|state| state.outcome().is_enabled())
                })
            })
            .unwrap_or(false)
    }

    fn feedback_state(&self, id: FeedbackId) -> Option<&FeedbackState> {
        self.states.get(&id)
    }

    fn ready_feedback(&self) -> Vec<FeedbackId> {
        self.states
            .iter()
            .filter_map(|(id, state)| {
                if state.phase() == FeedbackPhase::Ready {
                    Some(*id)
                } else {
                    None
                }
            })
            .collect()
    }
}

// ============================================================================
// Validation errors
// ============================================================================

/// Errors produced by the feedback model.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum FeedbackError {
    /// Feedback identifier is zero.
    ZeroFeedbackId,

    /// Classical signal identifier is zero.
    ZeroSignalId,

    /// Classical predicate identifier is zero.
    ZeroPredicateId,

    /// Classical event identifier is zero.
    ZeroEventId,

    /// Feedback dependency was repeated.
    DuplicateDependency(ClassicalSignalId),

    /// Feedback ID already exists.
    DuplicateFeedbackId(FeedbackId),

    /// Unknown feedback ID.
    UnknownFeedback(FeedbackId),

    /// Signal was supplied to a feedback request that does not require it.
    UnexpectedSignal(ClassicalSignalId),

    /// A signal value was supplied more than once.
    DuplicateSignalValue(ClassicalSignalId),

    /// Predicate does not match the request.
    UnexpectedPredicate(PredicateId),

    /// Predicate is not part of the request.
    PredicateNotRequired,

    /// Required runtime resolution is forbidden.
    RuntimeResolutionForbidden,

    /// Operation cannot continue in its current phase.
    InvalidPhaseTransition {
        /// Current phase.
        from: FeedbackPhase,

        /// Requested phase.
        to: FeedbackPhase,
    },

    /// Feedback cannot proceed because a prerequisite is missing.
    NotReady {
        /// Human-readable reason.
        reason: String,
    },

    /// A required runtime value is unavailable.
    RuntimeValueUnavailable,

    /// Feedback has already been cancelled.
    AlreadyCancelled,

    /// Feedback has already been superseded.
    AlreadySuperseded,

    /// Feedback has reached a terminal state.
    AlreadyTerminal,

    /// Cancellation is not permitted.
    CancellationForbidden,

    /// Feedback outcome cannot be pending during resolution.
    InvalidOutcome,

    /// Feedback generation overflowed its representable range.
    GenerationOverflow,

    /// Measurement source has no logical qubits.
    EmptyMeasurementQubitSet,

    /// Measurement source contains a duplicate logical qubit.
    DuplicateLogicalQubit,

    /// Measurement source contains a duplicate physical qubit.
    DuplicatePhysicalQubit,

    /// Distributed feedback has no source endpoint.
    EmptyDistributedSource,

    /// Distributed feedback has no destination endpoint.
    EmptyDistributedDestination,

    /// Internal graph identity inconsistency.
    GraphIdentityMismatch,
}

impl fmt::Display for FeedbackError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroFeedbackId => write!(formatter, "feedback identifier must not be zero"),
            Self::ZeroSignalId => write!(formatter, "classical signal identifier must not be zero"),
            Self::ZeroPredicateId => {
                write!(formatter, "predicate identifier must not be zero")
            }
            Self::ZeroEventId => write!(formatter, "classical event identifier must not be zero"),
            Self::DuplicateDependency(signal) => {
                write!(formatter, "feedback dependency repeated for {}", signal)
            }
            Self::DuplicateFeedbackId(id) => {
                write!(formatter, "feedback identifier already exists: {}", id)
            }
            Self::UnknownFeedback(id) => {
                write!(formatter, "unknown feedback identifier: {}", id)
            }
            Self::UnexpectedSignal(signal) => {
                write!(formatter, "unexpected classical signal: {}", signal)
            }
            Self::DuplicateSignalValue(signal) => {
                write!(formatter, "classical signal already has a value: {}", signal)
            }
            Self::UnexpectedPredicate(predicate) => {
                write!(formatter, "unexpected predicate: {}", predicate.value())
            }
            Self::PredicateNotRequired => {
                write!(formatter, "feedback request does not require predicate evaluation")
            }
            Self::RuntimeResolutionForbidden => {
                write!(formatter, "runtime feedback resolution is not permitted")
            }
            Self::InvalidPhaseTransition { from, to } => {
                write!(formatter, "invalid feedback phase transition: {:?} -> {:?}", from, to)
            }
            Self::NotReady { reason } => write!(formatter, "feedback is not ready: {}", reason),
            Self::RuntimeValueUnavailable => {
                write!(formatter, "required runtime classical value is unavailable")
            }
            Self::AlreadyCancelled => write!(formatter, "feedback has already been cancelled"),
            Self::AlreadySuperseded => write!(formatter, "feedback has already been superseded"),
            Self::AlreadyTerminal => write!(formatter, "feedback is already in a terminal state"),
            Self::CancellationForbidden => write!(formatter, "feedback cancellation is forbidden"),
            Self::InvalidOutcome => write!(formatter, "feedback outcome is not resolvable"),
            Self::GenerationOverflow => write!(formatter, "feedback generation overflow"),
            Self::EmptyMeasurementQubitSet => {
                write!(formatter, "measurement feedback must reference at least one logical qubit")
            }
            Self::DuplicateLogicalQubit => {
                write!(formatter, "measurement feedback contains duplicate logical qubits")
            }
            Self::DuplicatePhysicalQubit => {
                write!(formatter, "measurement feedback contains duplicate physical qubits")
            }
            Self::EmptyDistributedSource => {
                write!(formatter, "distributed feedback source must not be empty")
            }
            Self::EmptyDistributedDestination => {
                write!(formatter, "distributed feedback destination must not be empty")
            }
            Self::GraphIdentityMismatch => {
                write!(formatter, "feedback graph contains an identity mismatch")
            }
        }
    }
}

impl std::error::Error for FeedbackError {}

// ============================================================================
// Utility helpers
// ============================================================================

fn contains_duplicate<T>(values: &[T]) -> bool
where
    T: Ord + Copy,
{
    let mut seen = BTreeSet::new();

    values.iter().any(|value| !seen.insert(*value))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn signal(value: u64) -> ClassicalSignalId {
        ClassicalSignalId::new(value)
    }

    fn feedback(value: u64) -> FeedbackId {
        FeedbackId::new(value)
    }

    #[test]
    fn feedback_request_collects_dependency_signals() {
        let dependency = FeedbackDependency::new(
            signal(1),
            None,
            None,
            ClassicalLatency::Immediate,
            false,
        );

        let request = FeedbackRequest::new(
            feedback(1),
            FeedbackGeneration::initial(),
            FeedbackSource::Signal(signal(1)),
            vec![dependency],
            FeedbackCondition::SignalEquals {
                signal: signal(1),
                value: ClassicalValue::Bool(true),
            },
            FeedbackConsumer::Operation(42),
            ClassicalLatency::Immediate,
        );

        let required = request.required_signals();

        assert_eq!(required.len(), 1);
        assert!(required.contains(&signal(1)));
    }

    #[test]
    fn feedback_waits_for_required_signal() {
        let dependency = FeedbackDependency::new(
            signal(1),
            None,
            None,
            ClassicalLatency::Immediate,
            false,
        );

        let request = FeedbackRequest::new(
            feedback(1),
            FeedbackGeneration::initial(),
            FeedbackSource::Signal(signal(1)),
            vec![dependency],
            FeedbackCondition::AllSignals,
            FeedbackConsumer::Operation(42),
            ClassicalLatency::Immediate,
        );

        let state = FeedbackState::new(request).expect("valid feedback request");

        assert_eq!(state.phase(), FeedbackPhase::AwaitingSignals);
        assert_eq!(state.readiness().missing_signals().len(), 1);
    }

    #[test]
    fn providing_required_signal_makes_processing_ready() {
        let dependency = FeedbackDependency::new(
            signal(1),
            None,
            None,
            ClassicalLatency::Immediate,
            false,
        );

        let request = FeedbackRequest::new(
            feedback(1),
            FeedbackGeneration::initial(),
            FeedbackSource::Signal(signal(1)),
            vec![dependency],
            FeedbackCondition::AllSignals,
            FeedbackConsumer::Operation(42),
            ClassicalLatency::Immediate,
        );

        let mut state = FeedbackState::new(request).expect("valid feedback request");

        state
            .provide_value(signal(1), ClassicalValue::Bool(true))
            .expect("signal accepted");

        assert_eq!(state.phase(), FeedbackPhase::ProcessingReady);
        assert!(state.readiness().missing_signals().is_empty());
    }

    #[test]
    fn runtime_unresolved_feedback_is_deferred() {
        let dependency = FeedbackDependency::new(
            signal(1),
            None,
            None,
            ClassicalLatency::Runtime,
            false,
        );

        let request = FeedbackRequest::new(
            feedback(1),
            FeedbackGeneration::initial(),
            FeedbackSource::RuntimeEvent(ClassicalEventId::new(1)),
            vec![dependency],
            FeedbackCondition::Runtime,
            FeedbackConsumer::RuntimeEvent(ClassicalEventId::new(2)),
            ClassicalLatency::Runtime,
        );

        let mut state = FeedbackState::new(request).expect("valid feedback request");

        state
            .mark_runtime_unresolved(signal(1))
            .expect("runtime deferral allowed");

        assert_eq!(state.phase(), FeedbackPhase::RuntimeDeferred);
        assert_eq!(state.outcome(), FeedbackOutcome::RuntimeDeferred);
    }

    #[test]
    fn graph_routes_signal_to_waiting_feedback() {
        let dependency = FeedbackDependency::new(
            signal(1),
            None,
            None,
            ClassicalLatency::Immediate,
            false,
        );

        let request = FeedbackRequest::new(
            feedback(1),
            FeedbackGeneration::initial(),
            FeedbackSource::Signal(signal(1)),
            vec![dependency],
            FeedbackCondition::AllSignals,
            FeedbackConsumer::Operation(42),
            ClassicalLatency::Immediate,
        );

        let state = FeedbackState::new(request).expect("valid state");

        let mut graph = FeedbackGraph::new();
        graph.insert(state).expect("insert succeeds");

        let events = graph
            .provide_signal(signal(1), ClassicalValue::Bool(true))
            .expect("signal propagation succeeds");

        assert_eq!(events.len(), 1);

        let state = graph.get(feedback(1)).expect("state exists");

        assert_eq!(state.phase(), FeedbackPhase::ProcessingReady);
    }

    #[test]
    fn cancellation_is_terminal() {
        let request = FeedbackRequest::new(
            feedback(1),
            FeedbackGeneration::initial(),
            FeedbackSource::RuntimeEvent(ClassicalEventId::new(1)),
            Vec::new(),
            FeedbackCondition::Runtime,
            FeedbackConsumer::Operation(42),
            ClassicalLatency::Runtime,
        );

        let mut state = FeedbackState::new(request).expect("valid state");

        state.cancel().expect("cancellation succeeds");

        assert_eq!(state.phase(), FeedbackPhase::Cancelled);
        assert!(state.is_cancelled());
    }

    #[test]
    fn generation_is_checked_for_overflow() {
        let generation = FeedbackGeneration::new(u64::MAX);

        assert_eq!(
            generation.next(),
            Err(FeedbackError::GenerationOverflow)
        );
    }

    #[test]
    fn duplicate_dependencies_are_rejected() {
        let dependency_a = FeedbackDependency::new(
            signal(1),
            None,
            None,
            ClassicalLatency::Immediate,
            false,
        );

        let dependency_b = FeedbackDependency::new(
            signal(1),
            None,
            None,
            ClassicalLatency::Immediate,
            false,
        );

        let request = FeedbackRequest::new(
            feedback(1),
            FeedbackGeneration::initial(),
            FeedbackSource::Signal(signal(1)),
            vec![dependency_a, dependency_b],
            FeedbackCondition::AllSignals,
            FeedbackConsumer::Operation(42),
            ClassicalLatency::Immediate,
        );

        assert!(matches!(
            FeedbackState::new(request),
            Err(FeedbackError::DuplicateDependency(_))
        ));
    }
}