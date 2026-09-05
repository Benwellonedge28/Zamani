//! Zamani Quantum Scheduling — Scheduling Explanations
//!
//! This module converts structured scheduler trace events into deterministic,
//! human-readable explanations.
//!
//! # Architectural role
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! routing
//!      │
//!      ▼
//! scheduling
//!      │
//!      ▼
//! diagnostics::trace
//!      │
//!      ▼
//! diagnostics::explain
//! ```
//!
//! `trace.rs` answers:
//!
//! > What happened?
//!
//! This module answers:
//!
//! > Why did it happen, what influenced the decision, and what was the
//! > resulting scheduling consequence?
//!
//! The explanation layer is diagnostic infrastructure. It does not perform
//! scheduling, routing, resource allocation, timing calculation, verification,
//! hardware discovery, QEC decoding, or runtime execution.
//!
//! # Design goals
//!
//! This implementation is:
//!
//! - target independent;
//! - vendor independent;
//! - hardware independent;
//! - routing independent;
//! - QEC independent;
//! - algorithm independent;
//! - deterministic;
//! - streaming capable;
//! - suitable for tiny schedules;
//! - suitable for very large schedules;
//! - free of artificial machine-size limits;
//! - free of global mutable state;
//! - free of `unsafe`;
//! - compatible with Rust 1.97 and Rust 1.97.1.
//!
//! # Scalability
//!
//! There is intentionally no:
//!
//! - maximum number of explanations;
//! - maximum number of operations;
//! - maximum number of qubits;
//! - maximum number of resources;
//! - maximum schedule depth;
//! - maximum trace size.
//!
//! `explain_event` explains one event.
//!
//! `explain_snapshot` materializes explanations for an existing snapshot and
//! therefore consumes memory proportional to the requested output.
//!
//! `explain_stream` explains events one at a time and is the preferred API for
//! very large traces.
//!
//! A caller that needs effectively unbounded diagnostic processing should use
//! streaming rather than accumulating every explanation in memory.
//!
//! # Non-interference
//!
//! This module never changes scheduler state.
//!
//! Explanations are derived exclusively from already-produced trace events.
//! Generating an explanation cannot alter:
//!
//! - operation ordering;
//! - resource reservations;
//! - timing;
//! - routing;
//! - QEC state;
//! - hardware state;
//! - verification state.
//!
//! # Determinism
//!
//! Given the same trace event, configuration, and locale-independent formatting
//! rules, this module produces the same explanation.
//!
//! It does not use:
//!
//! - hash-map iteration;
//! - process IDs;
//! - memory addresses;
//! - wall-clock time;
//! - random numbers.
//!
//! # Integration contract
//!
//! `diagnostics/mod.rs` should expose this module with:
//!
//! ```text
//! pub mod explain;
//! pub mod profile;
//! pub mod trace;
//! ```
//!
//! Consumers then use:
//!
//! ```text
//! diagnostics::trace::TraceEvent
//!             │
//!             ▼
//! diagnostics::explain::explain_event()
//! ```
//!
//! For a complete trace:
//!
//! ```text
//! TraceSession
//!      │
//!      ▼
//! TraceSnapshot
//!      │
//!      ▼
//! explain_snapshot()
//! ```
//!
//! For large traces:
//!
//! ```text
//! TraceSink
//!      │
//!      ▼
//! explain_stream()
//!      │
//!      ▼
//! caller-owned output sink
//! ```
//!
//! # Important ownership rule
//!
//! This module does not recreate any quantum or scheduling identity.
//!
//! Operation, resource, dependency, reservation, logical-qubit, physical-qubit,
//! schedule, epoch, and scheduler-session identities are obtained from
//! `TraceReferences` in `trace.rs`.
//!
//! In particular, logical and physical qubit identities remain those defined
//! by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! # Explanation philosophy
//!
//! A useful explanation should distinguish:
//!
//! 1. subject — what object was affected;
//! 2. decision — what happened;
//! 3. cause — why it happened;
//! 4. timing — when it happened;
//! 5. resource — what resource influenced it;
//! 6. dependency — what dependency influenced it;
//! 7. consequence — what the decision means;
//! 8. diagnostic message — additional scheduler-provided context.
//!
//! The scheduler may supply richer information through `TraceEvent::message()`
//! and `TraceEvent::explanation()`. This module preserves that information
//! rather than attempting to reconstruct unavailable facts.
//!
//! # No false explanations
//!
//! This is a critical production invariant.
//!
//! The explanation layer must never claim a cause that the trace does not
//! actually establish.
//!
//! For example, if an event says only:
//!
//! ```text
//! operation.deferred
//! ```
//!
//! without a resource or dependency reference, this module must not invent a
//! resource conflict.
//!
//! It may say:
//!
//! ```text
//! The operation was deferred. The trace did not provide a specific cause.
//! ```
//!
//! This makes explanations trustworthy for production diagnostics.
//!
//! # Rust contract
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no `unsafe`.
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;

// =============================================================================
// Trace imports
// =============================================================================

use super::trace::{
    EmittedTraceEvent,
    TraceCategory,
    TraceDecision,
    TraceEvent,
    TraceLevel,
    TracePhase,
    TraceSnapshot,
};

// =============================================================================
// Explanation detail level
// =============================================================================

/// Controls how much information an explanation contains.
///
/// The levels affect presentation only. They never change the underlying trace
/// event or scheduler behaviour.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExplanationLevel {
    /// Compact one-line explanation.
    Compact,

    /// Normal operator-facing explanation.
    Normal,

    /// Detailed diagnostic explanation.
    Detailed,
}

impl Default for ExplanationLevel {
    fn default() -> Self {
        Self::Normal
    }
}

impl fmt::Display for ExplanationLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Compact => "compact",
            Self::Normal => "normal",
            Self::Detailed => "detailed",
        };

        formatter.write_str(value)
    }
}

// =============================================================================
// Explanation configuration
// =============================================================================

/// Immutable configuration for explanation generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExplanationConfig {
    level: ExplanationLevel,
    include_sequence: bool,
    include_event_id: bool,
    include_phase: bool,
    include_category: bool,
    include_decision: bool,
    include_timing: bool,
    include_references: bool,
    include_message: bool,
    include_explanation: bool,
}

impl Default for ExplanationConfig {
    fn default() -> Self {
        Self {
            level: ExplanationLevel::Normal,
            include_sequence: true,
            include_event_id: false,
            include_phase: true,
            include_category: true,
            include_decision: true,
            include_timing: true,
            include_references: true,
            include_message: true,
            include_explanation: true,
        }
    }
}

impl ExplanationConfig {
    /// Creates a compact configuration.
    #[must_use]
    pub const fn compact() -> Self {
        Self {
            level: ExplanationLevel::Compact,
            include_sequence: false,
            include_event_id: false,
            include_phase: false,
            include_category: false,
            include_decision: true,
            include_timing: true,
            include_references: true,
            include_message: true,
            include_explanation: false,
        }
    }

    /// Creates a detailed configuration.
    #[must_use]
    pub const fn detailed() -> Self {
        Self {
            level: ExplanationLevel::Detailed,
            include_sequence: true,
            include_event_id: true,
            include_phase: true,
            include_category: true,
            include_decision: true,
            include_timing: true,
            include_references: true,
            include_message: true,
            include_explanation: true,
        }
    }

    /// Sets explanation detail level.
    #[must_use]
    pub const fn with_level(mut self, level: ExplanationLevel) -> Self {
        self.level = level;
        self
    }

    /// Enables or disables sequence output.
    #[must_use]
    pub const fn with_sequence(mut self, enabled: bool) -> Self {
        self.include_sequence = enabled;
        self
    }

    /// Enables or disables trace-event identity output.
    #[must_use]
    pub const fn with_event_id(mut self, enabled: bool) -> Self {
        self.include_event_id = enabled;
        self
    }

    /// Enables or disables phase output.
    #[must_use]
    pub const fn with_phase(mut self, enabled: bool) -> Self {
        self.include_phase = enabled;
        self
    }

    /// Enables or disables category output.
    #[must_use]
    pub const fn with_category(mut self, enabled: bool) -> Self {
        self.include_category = enabled;
        self
    }

    /// Enables or disables decision output.
    #[must_use]
    pub const fn with_decision(mut self, enabled: bool) -> Self {
        self.include_decision = enabled;
        self
    }

    /// Enables or disables timing output.
    #[must_use]
    pub const fn with_timing(mut self, enabled: bool) -> Self {
        self.include_timing = enabled;
        self
    }

    /// Enables or disables reference output.
    #[must_use]
    pub const fn with_references(mut self, enabled: bool) -> Self {
        self.include_references = enabled;
        self
    }

    /// Enables or disables scheduler message output.
    #[must_use]
    pub const fn with_message(mut self, enabled: bool) -> Self {
        self.include_message = enabled;
        self
    }

    /// Enables or disables explicit explanation output.
    #[must_use]
    pub const fn with_explanation(mut self, enabled: bool) -> Self {
        self.include_explanation = enabled;
        self
    }

    /// Returns the explanation detail level.
    #[must_use]
    pub const fn level(self) -> ExplanationLevel {
        self.level
    }

    /// Returns whether sequence numbers are included.
    #[must_use]
    pub const fn includes_sequence(self) -> bool {
        self.include_sequence
    }

    /// Returns whether event IDs are included.
    #[must_use]
    pub const fn includes_event_id(self) -> bool {
        self.include_event_id
    }

    /// Returns whether phases are included.
    #[must_use]
    pub const fn includes_phase(self) -> bool {
        self.include_phase
    }

    /// Returns whether categories are included.
    #[must_use]
    pub const fn includes_category(self) -> bool {
        self.include_category
    }

    /// Returns whether decisions are included.
    #[must_use]
    pub const fn includes_decision(self) -> bool {
        self.include_decision
    }

    /// Returns whether timing is included.
    #[must_use]
    pub const fn includes_timing(self) -> bool {
        self.include_timing
    }

    /// Returns whether references are included.
    #[must_use]
    pub const fn includes_references(self) -> bool {
        self.include_references
    }

    /// Returns whether messages are included.
    #[must_use]
    pub const fn includes_message(self) -> bool {
        self.include_message
    }

    /// Returns whether explicit explanations are included.
    #[must_use]
    pub const fn includes_explanation(self) -> bool {
        self.include_explanation
    }
}

// =============================================================================
// Explanation subject
// =============================================================================

/// Object primarily discussed by an explanation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExplanationSubject {
    /// No specific object was identified.
    None,

    /// A quantum operation.
    Operation,

    /// A scheduling dependency.
    Dependency,

    /// A physical/scheduling resource.
    Resource,

    /// A reservation.
    Reservation,

    /// A logical qubit.
    LogicalQubit,

    /// A physical qubit.
    PhysicalQubit,

    /// A complete schedule.
    Schedule,

    /// A scheduling epoch.
    Epoch,

    /// A scheduler session.
    Session,

    /// A complete scheduling system or phase.
    Scheduler,
}

impl fmt::Display for ExplanationSubject {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::None => "event",
            Self::Operation => "operation",
            Self::Dependency => "dependency",
            Self::Resource => "resource",
            Self::Reservation => "reservation",
            Self::LogicalQubit => "logical qubit",
            Self::PhysicalQubit => "physical qubit",
            Self::Schedule => "schedule",
            Self::Epoch => "epoch",
            Self::Session => "scheduler session",
            Self::Scheduler => "scheduler",
        };

        formatter.write_str(value)
    }
}

// =============================================================================
// Explanation
// =============================================================================

/// Human-readable explanation of one scheduler trace event.
///
/// This is an owned value and therefore has no lifetime dependency on the
/// scheduler, trace session, or source IR.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Explanation {
    event_name: String,
    level: TraceLevel,
    category: TraceCategory,
    decision: TraceDecision,
    subject: ExplanationSubject,
    text: String,
}

impl Explanation {
    fn new(
        event: &TraceEvent,
        subject: ExplanationSubject,
        text: String,
    ) -> Self {
        Self {
            event_name: event.name().to_owned(),
            level: event.level(),
            category: event.category(),
            decision: event.decision(),
            subject,
            text,
        }
    }

    /// Returns the original trace event name.
    #[must_use]
    pub fn event_name(&self) -> &str {
        &self.event_name
    }

    /// Returns the trace severity.
    #[must_use]
    pub const fn level(&self) -> TraceLevel {
        self.level
    }

    /// Returns the trace category.
    #[must_use]
    pub const fn category(&self) -> TraceCategory {
        self.category
    }

    /// Returns the scheduling decision.
    #[must_use]
    pub const fn decision(&self) -> TraceDecision {
        self.decision
    }

    /// Returns the primary subject.
    #[must_use]
    pub const fn subject(&self) -> ExplanationSubject {
        self.subject
    }

    /// Returns the human-readable explanation.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Consumes the explanation and returns its text.
    #[must_use]
    pub fn into_text(self) -> String {
        self.text
    }
}

impl fmt::Display for Explanation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.text)
    }
}

// =============================================================================
// Explanation statistics
// =============================================================================

/// Aggregate statistics collected while explaining trace events.
///
/// This structure has fixed semantic counters rather than storage proportional
/// to the number of operations or trace events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ExplanationStatistics {
    processed: u64,
    explained: u64,
    lifecycle: u64,
    planner: u64,
    algorithm: u64,
    dependency: u64,
    operation: u64,
    resource: u64,
    timing: u64,
    constraint: u64,
    qec: u64,
    dynamic: u64,
    distributed: u64,
    verification: u64,
    transformation: u64,
    optimization: u64,
    errors: u64,
    warnings: u64,
}

impl ExplanationStatistics {
    /// Number of events processed.
    #[must_use]
    pub const fn processed(self) -> u64 {
        self.processed
    }

    /// Number of explanations generated.
    #[must_use]
    pub const fn explained(self) -> u64 {
        self.explained
    }

    /// Number of error events processed.
    #[must_use]
    pub const fn errors(self) -> u64 {
        self.errors
    }

    /// Number of warning events processed.
    #[must_use]
    pub const fn warnings(self) -> u64 {
        self.warnings
    }

    /// Returns the number of dependency events.
    #[must_use]
    pub const fn dependency_events(self) -> u64 {
        self.dependency
    }

    /// Returns the number of operation events.
    #[must_use]
    pub const fn operation_events(self) -> u64 {
        self.operation
    }

    /// Returns the number of resource events.
    #[must_use]
    pub const fn resource_events(self) -> u64 {
        self.resource
    }

    /// Returns the number of timing events.
    #[must_use]
    pub const fn timing_events(self) -> u64 {
        self.timing
    }

    /// Returns the number of constraint events.
    #[must_use]
    pub const fn constraint_events(self) -> u64 {
        self.constraint
    }

    /// Returns the number of QEC events.
    #[must_use]
    pub const fn qec_events(self) -> u64 {
        self.qec
    }

    /// Returns the number of dynamic scheduling events.
    #[must_use]
    pub const fn dynamic_events(self) -> u64 {
        self.dynamic
    }

    /// Returns the number of distributed scheduling events.
    #[must_use]
    pub const fn distributed_events(self) -> u64 {
        self.distributed
    }

    /// Returns the number of verification events.
    #[must_use]
    pub const fn verification_events(self) -> u64 {
        self.verification
    }

    /// Returns the number of transformation events.
    #[must_use]
    pub const fn transformation_events(self) -> u64 {
        self.transformation
    }

    /// Returns the number of optimization events.
    #[must_use]
    pub const fn optimization_events(self) -> u64 {
        self.optimization
    }

    fn checked_increment(value: &mut u64) -> Result<(), ExplanationError> {
        *value = value.checked_add(1).ok_or(
            ExplanationError::CounterOverflow,
        )?;

        Ok(())
    }

    fn record(
        &mut self,
        event: &EmittedTraceEvent,
    ) -> Result<(), ExplanationError> {
        Self::checked_increment(&mut self.processed)?;

        match event.event().level() {
            TraceLevel::Error => Self::checked_increment(&mut self.errors)?,
            TraceLevel::Warn => Self::checked_increment(&mut self.warnings)?,
            TraceLevel::Trace | TraceLevel::Debug | TraceLevel::Info => {}
        }

        match event.event().category() {
            TraceCategory::Lifecycle => {
                Self::checked_increment(&mut self.lifecycle)?
            }
            TraceCategory::Planner => {
                Self::checked_increment(&mut self.planner)?
            }
            TraceCategory::Algorithm => {
                Self::checked_increment(&mut self.algorithm)?
            }
            TraceCategory::Dependency => {
                Self::checked_increment(&mut self.dependency)?
            }
            TraceCategory::Operation => {
                Self::checked_increment(&mut self.operation)?
            }
            TraceCategory::Resource => {
                Self::checked_increment(&mut self.resource)?
            }
            TraceCategory::Timing => {
                Self::checked_increment(&mut self.timing)?
            }
            TraceCategory::Constraint => {
                Self::checked_increment(&mut self.constraint)?
            }
            TraceCategory::Routing => {}
            TraceCategory::Qec => {
                Self::checked_increment(&mut self.qec)?
            }
            TraceCategory::Dynamic => {
                Self::checked_increment(&mut self.dynamic)?
            }
            TraceCategory::Distributed => {
                Self::checked_increment(&mut self.distributed)?
            }
            TraceCategory::Verification => {
                Self::checked_increment(&mut self.verification)?
            }
            TraceCategory::Transformation => {
                Self::checked_increment(&mut self.transformation)?
            }
            TraceCategory::Optimization => {
                Self::checked_increment(&mut self.optimization)?
            }
            TraceCategory::Capacity => {}
            TraceCategory::Serialization => {}
            TraceCategory::Profile => {}
            TraceCategory::Custom => {}
        }

        Ok(())
    }

    fn record_explanation(&mut self) -> Result<(), ExplanationError> {
        Self::checked_increment(&mut self.explained)
    }
}

// =============================================================================
// Explanation errors
// =============================================================================

/// Errors produced while constructing explanations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExplanationError {
    /// A statistics counter could not be incremented safely.
    CounterOverflow,

    /// A caller supplied an invalid output target.
    InvalidOutput {
        /// Explanation of the invalid target.
        message: String,
    },
}

impl fmt::Display for ExplanationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CounterOverflow => {
                formatter.write_str(
                    "explanation statistics counter overflowed",
                )
            }
            Self::InvalidOutput { message } => {
                write!(formatter, "invalid explanation output: {message}")
            }
        }
    }
}

impl std::error::Error for ExplanationError {}

// =============================================================================
// Explanation report
// =============================================================================

/// Result of explaining a complete retained trace.
///
/// The report owns its explanations.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ExplanationReport {
    explanations: Vec<Explanation>,
    statistics: ExplanationStatistics,
}

impl ExplanationReport {
    /// Returns all generated explanations.
    #[must_use]
    pub fn explanations(&self) -> &[Explanation] {
        &self.explanations
    }

    /// Returns report statistics.
    #[must_use]
    pub const fn statistics(&self) -> ExplanationStatistics {
        self.statistics
    }

    /// Returns the number of explanations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.explanations.len()
    }

    /// Returns whether the report contains no explanations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.explanations.is_empty()
    }

    /// Consumes the report and returns its explanations.
    #[must_use]
    pub fn into_explanations(self) -> Vec<Explanation> {
        self.explanations
    }
}

// =============================================================================
// Trace event explanation
// =============================================================================

/// Explains one emitted trace event using the default configuration.
///
/// This function is allocation-bearing because the resulting explanation owns
/// its text.
///
/// For very large streams, prefer [`explain_stream`].
#[must_use]
pub fn explain_event(event: &EmittedTraceEvent) -> Explanation {
    explain_event_with_config(event, ExplanationConfig::default())
}

/// Explains one emitted trace event using an explicit configuration.
#[must_use]
pub fn explain_event_with_config(
    event: &EmittedTraceEvent,
    config: ExplanationConfig,
) -> Explanation {
    let trace = event.event();
    let subject = subject_for(trace);
    let text = build_text(event, config);

    Explanation::new(trace, subject, text)
}

// =============================================================================
// Snapshot explanation
// =============================================================================

/// Explains every event retained in a trace snapshot.
///
/// This intentionally returns owned explanations because the caller explicitly
/// requested materialization.
///
/// For extremely large traces, use [`explain_stream`] instead.
pub fn explain_snapshot(
    snapshot: &TraceSnapshot,
) -> Result<ExplanationReport, ExplanationError> {
    explain_snapshot_with_config(
        snapshot,
        ExplanationConfig::default(),
    )
}

/// Explains every retained event using an explicit configuration.
pub fn explain_snapshot_with_config(
    snapshot: &TraceSnapshot,
    config: ExplanationConfig,
) -> Result<ExplanationReport, ExplanationError> {
    let mut report = ExplanationReport::default();

    for event in snapshot.events() {
        report.statistics.record(event)?;

        let explanation =
            explain_event_with_config(event, config);

        report.statistics.record_explanation()?;
        report.explanations.push(explanation);
    }

    Ok(report)
}

// =============================================================================
// Streaming explanation
// =============================================================================

/// Explains events incrementally without retaining the resulting explanations.
///
/// The callback owns each explanation for the duration of the callback. The
/// callback may immediately write it to a file, socket, console, logging
/// backend, database, or another caller-owned stream.
///
/// This is the preferred API for very large scheduling traces.
///
/// Returning `Err` from the callback stops processing immediately.
pub fn explain_stream<I, F>(
    events: I,
    config: ExplanationConfig,
    mut output: F,
) -> Result<ExplanationStatistics, ExplanationError>
where
    I: IntoIterator,
    I::Item: std::borrow::Borrow<EmittedTraceEvent>,
    F: FnMut(Explanation) -> Result<(), ExplanationError>,
{
    let mut statistics = ExplanationStatistics::default();

    for item in events {
        let event = item.borrow();

        statistics.record(event)?;

        let explanation =
            explain_event_with_config(event, config);

        statistics.record_explanation()?;
        output(explanation)?;
    }

    Ok(statistics)
}

// =============================================================================
// Human-readable formatting
// =============================================================================

fn build_text(
    emitted: &EmittedTraceEvent,
    config: ExplanationConfig,
) -> String {
    let event = emitted.event();

    let mut text = String::new();

    append_header(&mut text, emitted, config);
    append_primary_reason(&mut text, event);

    if config.includes_timing() {
        append_timing(&mut text, event);
    }

    if config.includes_references() {
        append_references(&mut text, event);
    }

    if config.includes_message() {
        append_message(&mut text, event);
    }

    if config.includes_explanation() {
        append_explicit_explanation(&mut text, event);
    }

    append_consequence(&mut text, event);

    text
}

fn append_header(
    output: &mut String,
    emitted: &EmittedTraceEvent,
    config: ExplanationConfig,
) {
    let event = emitted.event();

    if config.includes_sequence() {
        let _ = write!(
            output,
            "[sequence {}] ",
            emitted.sequence().value()
        );
    }

    if config.includes_event_id() {
        let _ = write!(
            output,
            "[event {}] ",
            emitted.id().value()
        );
    }

    if config.includes_category() {
        let _ = write!(
            output,
            "[{}] ",
            event.category()
        );
    }

    if config.includes_phase() {
        if let Some(phase) = event.phase() {
            let _ = write!(
                output,
                "[phase={}] ",
                phase
            );
        }
    }

    let _ = write!(
        output,
        "{}: ",
        event.name()
    );

    if config.includes_decision() {
        let _ = write!(
            output,
            "{}. ",
            decision_sentence(event.decision())
        );
    }
}

fn append_primary_reason(
    output: &mut String,
    event: &TraceEvent,
) {
    match event.category() {
        TraceCategory::Dependency => {
            append_dependency_reason(output, event)
        }

        TraceCategory::Resource => {
            append_resource_reason(output, event)
        }

        TraceCategory::Timing => {
            append_timing_reason(output, event)
        }

        TraceCategory::Constraint => {
            append_constraint_reason(output, event)
        }

        TraceCategory::Operation => {
            append_operation_reason(output, event)
        }

        TraceCategory::Verification => {
            append_verification_reason(output, event)
        }

        TraceCategory::Qec => {
            append_qec_reason(output, event)
        }

        TraceCategory::Dynamic => {
            append_dynamic_reason(output, event)
        }

        TraceCategory::Distributed => {
            append_distributed_reason(output, event)
        }

        TraceCategory::Transformation => {
            append_transformation_reason(output, event)
        }

        TraceCategory::Optimization => {
            append_optimization_reason(output, event)
        }

        TraceCategory::Planner => {
            append_planner_reason(output, event)
        }

        TraceCategory::Algorithm => {
            append_algorithm_reason(output, event)
        }

        TraceCategory::Routing => {
            output.push_str(
                "The event records information supplied by the routing stage. ",
            );
        }

        TraceCategory::Capacity => {
            output.push_str(
                "The decision was influenced by an explicit capacity or limit. ",
            );
        }

        TraceCategory::Lifecycle => {
            output.push_str(
                "The event describes scheduler lifecycle activity. ",
            );
        }

        TraceCategory::Transformation
        | TraceCategory::Serialization
        | TraceCategory::Profile
        | TraceCategory::Custom => {}
    }
}

fn append_dependency_reason(
    output: &mut String,
    event: &TraceEvent,
) {
    let references = event.references();

    match (
        references.operation(),
        references.dependency(),
        event.decision(),
    ) {
        (Some(operation), Some(dependency), TraceDecision::Deferred) => {
            let _ = write!(
                output,
                "Operation {operation} could not proceed because dependency \
                 {dependency} was still relevant to its readiness. "
            );
        }

        (Some(operation), Some(dependency), _) => {
            let _ = write!(
                output,
                "Operation {operation} was evaluated in relation to \
                 dependency {dependency}. "
            );
        }

        (Some(operation), None, _) => {
            let _ = write!(
                output,
                "Operation {operation} was evaluated with respect to \
                 scheduling dependencies. "
            );
        }

        _ => {
            output.push_str(
                "The event concerns dependency ordering. ",
            );
        }
    }
}

fn append_resource_reason(
    output: &mut String,
    event: &TraceEvent,
) {
    let references = event.references();

    match (
        references.operation(),
        references.resource(),
        references.reservation(),
        event.decision(),
    ) {
        (
            Some(operation),
            Some(resource),
            _,
            TraceDecision::Deferred,
        ) => {
            let _ = write!(
                output,
                "Operation {operation} was deferred because resource \
                 {resource} influenced its availability. "
            );
        }

        (
            Some(operation),
            Some(resource),
            Some(reservation),
            TraceDecision::Reserved,
        ) => {
            let _ = write!(
                output,
                "Resource {resource} was reserved for operation {operation} \
                 under reservation {reservation}. "
            );
        }

        (Some(operation), Some(resource), _, _) => {
            let _ = write!(
                output,
                "Resource {resource} was considered for operation \
                 {operation}. "
            );
        }

        (_, Some(resource), _, _) => {
            let _ = write!(
                output,
                "Resource {resource} participated in this scheduling \
                 decision. "
            );
        }

        _ => {
            output.push_str(
                "The event concerns resource availability or allocation. ",
            );
        }
    }
}

fn append_timing_reason(
    output: &mut String,
    event: &TraceEvent,
) {
    match event.decision() {
        TraceDecision::TimingAccepted => {
            output.push_str(
                "The candidate timing satisfied the timing decision \
                 represented by this event. ",
            );
        }

        TraceDecision::TimingRejected => {
            output.push_str(
                "The candidate timing was rejected by the timing logic \
                 represented by this event. ",
            );
        }

        TraceDecision::Deferred => {
            output.push_str(
                "The operation could not use the currently considered \
                 timing position. ",
            );
        }

        _ => {
            output.push_str(
                "The event records a scheduling-time decision. ",
            );
        }
    }
}

fn append_constraint_reason(
    output: &mut String,
    event: &TraceEvent,
) {
    match event.decision() {
        TraceDecision::ConstraintSatisfied => {
            output.push_str(
                "The evaluated scheduling constraint was satisfied. ",
            );
        }

        TraceDecision::ConstraintViolated => {
            output.push_str(
                "The evaluated scheduling constraint was violated. ",
            );
        }

        TraceDecision::Rejected => {
            output.push_str(
                "The candidate was rejected in relation to a scheduling \
                 constraint. ",
            );
        }

        _ => {
            output.push_str(
                "The event records constraint evaluation. ",
            );
        }
    }
}

fn append_operation_reason(
    output: &mut String,
    event: &TraceEvent,
) {
    if let Some(operation) = event.references().operation() {
        let _ = write!(
            output,
            "Operation {operation} is the primary scheduling subject. "
        );
    } else {
        output.push_str(
            "The event concerns an operation-level scheduling decision. ",
        );
    }

    if let Some(qubit) = event.references().logical_qubit() {
        let _ = write!(
            output,
            "Logical qubit {qubit} is associated with the event. "
        );
    }

    if let Some(qubit) = event.references().physical_qubit() {
        let _ = write!(
            output,
            "Physical qubit {qubit} is associated with the event. "
        );
    }
}

fn append_verification_reason(
    output: &mut String,
    event: &TraceEvent,
) {
    match event.decision() {
        TraceDecision::VerificationPassed => {
            output.push_str(
                "The represented verification check passed. ",
            );
        }

        TraceDecision::VerificationFailed => {
            output.push_str(
                "The represented verification check failed. ",
            );
        }

        _ => {
            output.push_str(
                "The event records verification activity. ",
            );
        }
    }
}

fn append_qec_reason(
    output: &mut String,
    event: &TraceEvent,
) {
    output.push_str(
        "The event represents a QEC-related scheduling requirement or \
         observation. ",
    );

    if let Some(operation) = event.references().operation() {
        let _ = write!(
            output,
            "The associated operation is {operation}. "
        );
    }
}

fn append_dynamic_reason(
    output: &mut String,
    event: &TraceEvent,
) {
    output.push_str(
        "The event represents scheduling affected by dynamic or classical \
         execution state. ",
    );
}

fn append_distributed_reason(
    output: &mut String,
    event: &TraceEvent,
) {
    output.push_str(
        "The event represents distributed execution, communication, or \
         synchronization constraints. ",
    );
}

fn append_transformation_reason(
    output: &mut String,
    event: &TraceEvent,
) {
    output.push_str(
        "The event records a transformation applied to the scheduled \
         representation. ",
    );
}

fn append_optimization_reason(
    output: &mut String,
    event: &TraceEvent,
) {
    output.push_str(
        "The event records evaluation or application of a scheduling \
         optimization objective. ",
    );
}

fn append_planner_reason(
    output: &mut String,
    event: &TraceEvent,
) {
    output.push_str(
        "The planner generated or evaluated a scheduling decision. ",
    );
}

fn append_algorithm_reason(
    output: &mut String,
    event: &TraceEvent,
) {
    output.push_str(
        "A scheduling algorithm or algorithmic strategy generated or \
         evaluated this event. ",
    );
}

fn append_timing(
    output: &mut String,
    event: &TraceEvent,
) {
    let timing = event.timing();

    if let Some(start) = timing.schedule_time() {
        let _ = write!(
            output,
            "The scheduler time associated with the event is {start}. "
        );
    }

    if let Some(duration) = timing.duration() {
        let _ = write!(
            output,
            "The associated abstract duration is {}. ",
            duration.value()
        );
    }

    if let Some(host_elapsed) = timing.host_elapsed_nanos() {
        let _ = write!(
            output,
            "Host-side processing elapsed time was \
             {host_elapsed} ns. "
        );
    }
}

fn append_references(
    output: &mut String,
    event: &TraceEvent,
) {
    let references = event.references();
    let mut wrote = false;

    if let Some(operation) = references.operation() {
        let _ = write!(
            output,
            "Operation={operation}"
        );
        wrote = true;
    }

    if let Some(dependency) = references.dependency() {
        append_reference_separator(output, &mut wrote);
        let _ = write!(
            output,
            "Dependency={dependency}"
        );
    }

    if let Some(resource) = references.resource() {
        append_reference_separator(output, &mut wrote);
        let _ = write!(
            output,
            "Resource={resource}"
        );
    }

    if let Some(reservation) = references.reservation() {
        append_reference_separator(output, &mut wrote);
        let _ = write!(
            output,
            "Reservation={reservation}"
        );
    }

    if let Some(logical) = references.logical_qubit() {
        append_reference_separator(output, &mut wrote);
        let _ = write!(
            output,
            "LogicalQubit={logical}"
        );
    }

    if let Some(physical) = references.physical_qubit() {
        append_reference_separator(output, &mut wrote);
        let _ = write!(
            output,
            "PhysicalQubit={physical}"
        );
    }

    if let Some(schedule) = references.schedule() {
        append_reference_separator(output, &mut wrote);
        let _ = write!(
            output,
            "Schedule={schedule}"
        );
    }

    if let Some(epoch) = references.epoch() {
        append_reference_separator(output, &mut wrote);
        let _ = write!(
            output,
            "Epoch={epoch}"
        );
    }

    if let Some(session) = references.session() {
        append_reference_separator(output, &mut wrote);
        let _ = write!(
            output,
            "Session={session}"
        );
    }

    if wrote {
        output.push('.');
        output.push(' ');
    }
}

fn append_reference_separator(
    output: &mut String,
    wrote: &mut bool,
) {
    if *wrote {
        output.push_str(", ");
    } else {
        *wrote = true;
    }
}

fn append_message(
    output: &mut String,
    event: &TraceEvent,
) {
    if let Some(message) = event.message() {
        if !message.is_empty() {
            output.push_str("Scheduler message: ");
            output.push_str(message);
            output.push_str(". ");
        }
    }
}

fn append_explicit_explanation(
    output: &mut String,
    event: &TraceEvent,
) {
    if let Some(explanation) = event.explanation() {
        if !explanation.is_empty() {
            output.push_str("Recorded explanation: ");
            output.push_str(explanation);
            output.push_str(". ");
        }
    }
}

fn append_consequence(
    output: &mut String,
    event: &TraceEvent,
) {
    match event.decision() {
        TraceDecision::Ready => {
            output.push_str(
                "The operation became eligible for further scheduling \
                 consideration.",
            );
        }

        TraceDecision::Selected => {
            output.push_str(
                "The operation was selected by the scheduling process.",
            );
        }

        TraceDecision::Scheduled => {
            output.push_str(
                "The operation received a scheduling position.",
            );
        }

        TraceDecision::Deferred => {
            output.push_str(
                "The operation was not advanced at the represented \
                 scheduling point.",
            );
        }

        TraceDecision::Rejected => {
            output.push_str(
                "The candidate was rejected and must not be treated as \
                 accepted scheduling output.",
            );
        }

        TraceDecision::Reserved => {
            output.push_str(
                "The resource reservation became part of the represented \
                 scheduling state.",
            );
        }

        TraceDecision::Released => {
            output.push_str(
                "The represented resource reservation became releasable \
                 or was released.",
            );
        }

        TraceDecision::TimingAccepted => {
            output.push_str(
                "The timing candidate was accepted.",
            );
        }

        TraceDecision::TimingRejected => {
            output.push_str(
                "The timing candidate was not accepted.",
            );
        }

        TraceDecision::ConstraintSatisfied => {
            output.push_str(
                "The candidate satisfied the represented constraint.",
            );
        }

        TraceDecision::ConstraintViolated => {
            output.push_str(
                "The candidate did not satisfy the represented constraint.",
            );
        }

        TraceDecision::VerificationPassed => {
            output.push_str(
                "The represented verification check passed.",
            );
        }

        TraceDecision::VerificationFailed => {
            output.push_str(
                "The represented verification check failed.",
            );
        }

        TraceDecision::Observed => {
            output.push_str(
                "The event is observational and does not by itself \
                 establish a scheduling decision.",
            );
        }
    }
}

fn decision_sentence(decision: TraceDecision) -> &'static str {
    match decision {
        TraceDecision::Ready => "readiness was recorded",
        TraceDecision::Selected => "selection was recorded",
        TraceDecision::Scheduled => "scheduling was recorded",
        TraceDecision::Deferred => "deferral was recorded",
        TraceDecision::Rejected => "rejection was recorded",
        TraceDecision::Reserved => "reservation was recorded",
        TraceDecision::Released => "release was recorded",
        TraceDecision::TimingAccepted => {
            "a timing candidate was accepted"
        }
        TraceDecision::TimingRejected => {
            "a timing candidate was rejected"
        }
        TraceDecision::ConstraintSatisfied => {
            "a constraint was satisfied"
        }
        TraceDecision::ConstraintViolated => {
            "a constraint was violated"
        }
        TraceDecision::VerificationPassed => {
            "verification passed"
        }
        TraceDecision::VerificationFailed => {
            "verification failed"
        }
        TraceDecision::Observed => "an observation was recorded",
    }
}

fn subject_for(event: &TraceEvent) -> ExplanationSubject {
    let references = event.references();

    if references.operation().is_some() {
        return ExplanationSubject::Operation;
    }

    if references.dependency().is_some() {
        return ExplanationSubject::Dependency;
    }

    if references.reservation().is_some() {
        return ExplanationSubject::Reservation;
    }

    if references.resource().is_some() {
        return ExplanationSubject::Resource;
    }

    if references.logical_qubit().is_some() {
        return ExplanationSubject::LogicalQubit;
    }

    if references.physical_qubit().is_some() {
        return ExplanationSubject::PhysicalQubit;
    }

    if references.schedule().is_some() {
        return ExplanationSubject::Schedule;
    }

    if references.epoch().is_some() {
        return ExplanationSubject::Epoch;
    }

    if references.session().is_some() {
        return ExplanationSubject::Session;
    }

    ExplanationSubject::Scheduler
}

// =============================================================================
// Phase explanation helpers
// =============================================================================

/// Returns a concise explanation of a scheduler phase.
#[must_use]
pub fn explain_phase(phase: TracePhase) -> &'static str {
    match phase {
        TracePhase::Initialization => {
            "The scheduler is establishing the diagnostic and planning \
             session."
        }

        TracePhase::Input => {
            "The scheduler is normalizing or validating scheduling input."
        }

        TracePhase::DependencyAnalysis => {
            "The scheduler is determining ordering relationships between \
             operations."
        }

        TracePhase::ResourceAnalysis => {
            "The scheduler is analyzing resource availability and \
             contention."
        }

        TracePhase::TimingAnalysis => {
            "The scheduler is determining legal temporal positions."
        }

        TracePhase::Planning => {
            "The scheduler is constructing or evaluating a schedule."
        }

        TracePhase::Transformation => {
            "The scheduled representation is being transformed while \
             preserving its required semantics."
        }

        TracePhase::Verification => {
            "The generated schedule is being checked against its required \
             invariants."
        }

        TracePhase::Finalization => {
            "The scheduler is finalizing its result and associated \
             diagnostics."
        }

        TracePhase::Runtime => {
            "Scheduling information is being evaluated or updated during \
             runtime or dynamic execution."
        }

        TracePhase::Custom => {
            "A caller-defined scheduling phase is being diagnosed."
        }
    }
}

// =============================================================================
// Convenience explanation queries
// =============================================================================

/// Returns true when the event represents a potentially blocking/deferred
/// scheduling decision.
#[must_use]
pub fn is_blocking_event(event: &TraceEvent) -> bool {
    matches!(
        event.decision(),
        TraceDecision::Deferred
            | TraceDecision::TimingRejected
            | TraceDecision::ConstraintViolated
            | TraceDecision::Rejected
    )
}

/// Returns true when the event represents a successful scheduling outcome.
#[must_use]
pub fn is_success_event(event: &TraceEvent) -> bool {
    matches!(
        event.decision(),
        TraceDecision::Scheduled
            | TraceDecision::Reserved
            | TraceDecision::Released
            | TraceDecision::TimingAccepted
            | TraceDecision::ConstraintSatisfied
            | TraceDecision::VerificationPassed
    )
}

/// Returns true when the event should normally attract operator attention.
#[must_use]
pub fn requires_attention(event: &TraceEvent) -> bool {
    matches!(
        event.level(),
        TraceLevel::Warn | TraceLevel::Error
    ) || is_blocking_event(event)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::ir::core::identity::{
        OperationId,
        ResourceId,
    };
    use crate::quantum::ir::qubit::{
        PhysicalQubitId,
        QubitId,
    };
    use crate::quantum::scheduling::types::{
        DependencyId,
        Duration,
        ReservationId,
        TimePoint,
    };

    fn event() -> EmittedTraceEvent {
        let mut session = super::super::trace::TraceSession::new(
            super::super::trace::TraceConfig::default()
                .with_retention(
                    super::super::trace::TraceRetention::Unlimited,
                ),
        );

        session
            .emit(
                TraceEvent::new(
                    TraceLevel::Debug,
                    TraceCategory::Resource,
                    "resource.conflict",
                )
                .with_phase(TracePhase::Planning)
                .with_decision(TraceDecision::Deferred)
                .with_references(
                    super::super::trace::TraceReferences::new()
                        .with_operation(OperationId::new(1))
                        .with_dependency(DependencyId::new(2))
                        .with_resource(ResourceId::new(3))
                        .with_reservation(
                            ReservationId::new(4),
                        )
                        .with_logical_qubit(QubitId::new(5))
                        .with_physical_qubit(
                            PhysicalQubitId::new(6),
                        ),
                )
                .with_timing(
                    super::super::trace::TraceTiming::new()
                        .with_schedule_time(
                            TimePoint::new(100),
                        )
                        .with_duration(
                            Duration::new(20),
                        ),
                )
                .with_message(
                    "resource is occupied until the candidate interval",
                )
                .with_explanation(
                    "the candidate interval overlaps an existing reservation",
                ),
            )
            .expect("trace emission must succeed")
            .expect("event must pass filtering")
    }

    #[test]
    fn explains_resource_conflict_without_inventing_cause() {
        let emitted = event();

        let explanation = explain_event(&emitted);

        assert_eq!(
            explanation.category(),
            TraceCategory::Resource
        );
        assert_eq!(
            explanation.decision(),
            TraceDecision::Deferred
        );

        assert!(
            explanation
                .text()
                .contains("resource")
        );

        assert!(
            explanation
                .text()
                .contains("resource:3")
        );
    }

    #[test]
    fn event_with_no_specific_cause_does_not_invent_one() {
        let mut session = super::super::trace::TraceSession::new(
            super::super::trace::TraceConfig::default()
                .with_retention(
                    super::super::trace::TraceRetention::Unlimited,
                ),
        );

        let emitted = session
            .emit(
                TraceEvent::new(
                    TraceLevel::Debug,
                    TraceCategory::Operation,
                    "operation.deferred",
                )
                .with_decision(TraceDecision::Deferred),
            )
            .expect("trace emission must succeed")
            .expect("event must pass filtering");

        let explanation = explain_event(&emitted);

        assert!(
            explanation
                .text()
                .contains("operation")
        );

        assert!(
            !explanation
                .text()
                .contains("resource")
                || explanation
                    .text()
                    .contains("resource availability")
        );
    }

    #[test]
    fn phase_explanation_is_deterministic() {
        assert_eq!(
            explain_phase(TracePhase::Planning),
            explain_phase(TracePhase::Planning)
        );
    }

    #[test]
    fn blocking_classification_is_correct() {
        let mut session = super::super::trace::TraceSession::new(
            super::super::trace::TraceConfig::default()
                .with_retention(
                    super::super::trace::TraceRetention::Unlimited,
                ),
        );

        let emitted = session
            .emit(
                TraceEvent::new(
                    TraceLevel::Debug,
                    TraceCategory::Timing,
                    "timing.decision",
                )
                .with_decision(
                    TraceDecision::TimingRejected,
                ),
            )
            .expect("trace emission must succeed")
            .expect("event must pass filtering");

        assert!(is_blocking_event(emitted.event()));
        assert!(!is_success_event(emitted.event()));
    }

    #[test]
    fn successful_classification_is_correct() {
        let mut session = super::super::trace::TraceSession::new(
            super::super::trace::TraceConfig::default()
                .with_retention(
                    super::super::trace::TraceRetention::Unlimited,
                ),
        );

        let emitted = session
            .emit(
                TraceEvent::new(
                    TraceLevel::Debug,
                    TraceCategory::Operation,
                    "operation.scheduled",
                )
                .with_decision(
                    TraceDecision::Scheduled,
                ),
            )
            .expect("trace emission must succeed")
            .expect("event must pass filtering");

        assert!(is_success_event(emitted.event()));
        assert!(!is_blocking_event(emitted.event()));
    }

    #[test]
    fn snapshot_explanation_is_owned() {
        let mut session = super::super::trace::TraceSession::new(
            super::super::trace::TraceConfig::default()
                .with_retention(
                    super::super::trace::TraceRetention::Unlimited,
                ),
        );

        session
            .emit(
                TraceEvent::new(
                    TraceLevel::Info,
                    TraceCategory::Lifecycle,
                    "session.started",
                ),
            )
            .expect("trace emission must succeed");

        let snapshot = session.snapshot();

        let report = explain_snapshot(&snapshot)
            .expect("snapshot explanation must succeed");

        assert_eq!(report.len(), 1);
        assert_eq!(
            report.statistics().processed(),
            1
        );
        assert_eq!(
            report.statistics().explained(),
            1
        );
    }

    #[test]
    fn streaming_does_not_require_explanation_retention() {
        let mut session = super::super::trace::TraceSession::new(
            super::super::trace::TraceConfig::default()
                .with_retention(
                    super::super::trace::TraceRetention::Unlimited,
                ),
        );

        for index in 0_u64..4 {
            session
                .emit(
                    TraceEvent::new(
                        TraceLevel::Info,
                        TraceCategory::Lifecycle,
                        "scheduler.event",
                    )
                    .with_message(
                        format!("event {index}"),
                    ),
                )
                .expect("trace emission must succeed");
        }

        let snapshot = session.snapshot();

        let mut count = 0_u64;

        let statistics = explain_stream(
            snapshot.events(),
            ExplanationConfig::compact(),
            |explanation| {
                assert!(!explanation.text().is_empty());
                count = count
                    .checked_add(1)
                    .ok_or(
                        ExplanationError::CounterOverflow
                    )?;
                Ok(())
            },
        )
        .expect("stream explanation must succeed");

        assert_eq!(count, 4);
        assert_eq!(statistics.processed(), 4);
        assert_eq!(statistics.explained(), 4);
    }

    #[test]
    fn canonical_qubit_references_are_explained() {
        let mut session = super::super::trace::TraceSession::new(
            super::super::trace::TraceConfig::default()
                .with_retention(
                    super::super::trace::TraceRetention::Unlimited,
                ),
        );

        let emitted = session
            .emit(
                TraceEvent::new(
                    TraceLevel::Info,
                    TraceCategory::Operation,
                    "operation.inspect",
                )
                .with_references(
                    super::super::trace::TraceReferences::new()
                        .with_logical_qubit(
                            QubitId::new(10)
                        )
                        .with_physical_qubit(
                            PhysicalQubitId::new(20)
                        ),
                ),
            )
            .expect("trace emission must succeed")
            .expect("event must pass filtering");

        let explanation = explain_event(&emitted);

        assert!(
            explanation
                .text()
                .contains("Logical qubit")
        );

        assert!(
            explanation
                .text()
                .contains("Physical qubit")
        );
    }

    #[test]
    fn attention_detection_includes_warnings() {
        let mut session = super::super::trace::TraceSession::new(
            super::super::trace::TraceConfig::default()
                .with_retention(
                    super::super::trace::TraceRetention::Unlimited,
                ),
        );

        let emitted = session
            .emit(
                TraceEvent::new(
                    TraceLevel::Warn,
                    TraceCategory::Constraint,
                    "constraint.warning",
                )
                .with_decision(
                    TraceDecision::Observed,
                ),
            )
            .expect("trace emission must succeed")
            .expect("event must pass filtering");

        assert!(requires_attention(emitted.event()));
    }
}