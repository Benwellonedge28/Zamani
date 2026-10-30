
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! Zamani distributed observability event model.
//!
//! Path: src/distributed/observability/events.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! # Purpose
//!
//! Defines structured, portable observability events for distributed
//! computation throughout Zamani.
//!
//! Events describe observations already established by their owning
//! subsystem. They do not execute tasks, change lifecycle state,
//! schedule work, perform network communication, authorize operations,
//! or change program semantics.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - DistributedEvent and its structured event metadata.
//! - Stable event categories.
//! - Open-world event names.
//! - Logical event subjects.
//! - Correlation and causation context.
//! - Event timestamps.
//! - Structured, non-payload event attributes.
//! - Event sink interfaces.
//! - Optional multi-sink composition.
//! - Observer failure reporting.
//!
//! This module does not own:
//!
//! - Distributed identity allocation.
//! - Execution lifecycle state.
//! - Task scheduling or placement.
//! - Membership or health authority.
//! - Transport protocols or network I/O.
//! - Authentication or authorization.
//! - Retry decisions or checkpoint implementation.
//! - Metrics aggregation or tracing infrastructure.
//! - ZUIR, ZCIR, quantum::ir, HDLIR, or backend representations.
//! - Source syntax, AST, or semantic analysis.
//!
//! # Important integration rules
//!
//! 1. Use canonical identifiers from distributed::ids.
//! 2. Consume lifecycle observations from execution::lifecycle.
//! 3. Consume membership observations from membership.
//! 4. Consume retry and recovery observations from resilience.
//! 5. Propagate correlation context through transport.
//! 6. Do not introduce a competing execution event journal.
//! 7. Do not serialize arbitrary execution payloads into events.
//! 8. Observer failures must not silently change computation semantics.
//!
//! # Scalability
//!
//! This module imposes no arbitrary maximum on:
//!
//! - events;
//! - participants;
//! - tasks;
//! - plans;
//! - operations;
//! - messages;
//! - event attributes;
//! - execution contexts;
//! - distributed systems.
//!
//! Actual storage, queue, throughput, and retention limits belong to
//! injected event sinks and their explicitly configured resource policies.
//!
//! The event representation uses Rust's actual supported types.
//! It does not promise mathematically infinite storage or memory.
//!
//! # Security
//!
//! Events deliberately do not contain arbitrary execution payloads,
//! credentials, private keys, authentication tokens, or checkpoint bytes.
//!
//! Attribute producers remain responsible for redacting sensitive
//! information before constructing events.
//!
//! Event sinks must apply their own access-control, retention,
//! integrity, and encryption policies.
//!
//! # Determinism
//!
//! Event categories and names have stable machine-readable identifiers.
//! Attribute maps use BTreeMap so their iteration order is deterministic.
//!
//! Event occurrence timestamps describe actual observation time and
//! therefore are not expected to be identical across independent runs.

use core::fmt;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::distributed::ids::{
    CorrelationId,
    ExecutionContextId,
    MessageId,
    OperationId,
    ParticipantId,
    PlanId,
    TaskId,
};

/// Result returned by event construction and event publication.
pub type EventResult<T> = Result<T, EventError>;

/// Stable classification of distributed observability events.
///
/// Categories describe the subsystem that owns the observed fact.
///
/// This enum intentionally describes broad event families.
/// Individual event names remain open-world so future providers and
/// computational domains do not require changes to this enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum EventCategory {
    /// Distributed execution and task lifecycle.
    Execution,

    /// Distributed planning and feasibility.
    Planning,

    /// Task partitioning.
    Partitioning,

    /// Resource and participant placement.
    Placement,

    /// Execution scheduling.
    Scheduling,

    /// Logical communication and routing.
    Communication,

    /// Transport operations.
    Transport,

    /// Participant membership and discovery.
    Membership,

    /// Participant health and availability.
    Health,

    /// Retry decisions and execution reconciliation.
    Resilience,

    /// Checkpoint creation, persistence and restoration.
    Checkpoint,

    /// Idempotency and duplicate-operation handling.
    Idempotency,

    /// Authentication and authorization observations.
    Security,

    /// Distributed state transfer and migration.
    StateTransfer,

    /// Distributed runtime initialization and shutdown.
    Runtime,

    /// Domain-specific execution providers.
    Domain,

    /// Resource availability and utilization observations.
    Resources,

    /// Compatibility and protocol negotiation.
    Compatibility,

    /// External provider or integration observations.
    Integration,
}

impl EventCategory {
    /// Return the stable machine-readable category identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Execution => "execution",
            Self::Planning => "planning",
            Self::Partitioning => "partitioning",
            Self::Placement => "placement",
            Self::Scheduling => "scheduling",
            Self::Communication => "communication",
            Self::Transport => "transport",
            Self::Membership => "membership",
            Self::Health => "health",
            Self::Resilience => "resilience",
            Self::Checkpoint => "checkpoint",
            Self::Idempotency => "idempotency",
            Self::Security => "security",
            Self::StateTransfer => "state_transfer",
            Self::Runtime => "runtime",
            Self::Domain => "domain",
            Self::Resources => "resources",
            Self::Compatibility => "compatibility",
            Self::Integration => "integration",
        }
    }
}

impl fmt::Display for EventCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Open-world distributed event name.
///
/// Names are qualified identifiers, not a closed enumeration.
/// This allows new execution providers, transports, computational
/// domains, and future hardware technologies to introduce events
/// without modifying the distributed core.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventName(String);

impl EventName {
    /// Construct a validated event name.
    ///
    /// Names must be nonempty, contain no whitespace or control
    /// characters, and use a qualified identifier convention.
    ///
    /// Examples:
    ///
    /// - `execution.task_started`
    /// - `execution.task_completed`
    /// - `planning.feasibility_rejected`
    /// - `membership.participant_joined`
    /// - `checkpoint.restoration_completed`
    /// - `vendor.extension.operation_observed`
    pub fn new(value: impl Into<String>) -> EventResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(EventError::InvalidName(
                "event name cannot be empty".into(),
            ));
        }

        if value.trim() != value {
            return Err(EventError::InvalidName(
                "event name cannot contain surrounding whitespace".into(),
            ));
        }

        if value.chars().any(|c| {
            c.is_whitespace() || c.is_control()
        }) {
            return Err(EventError::InvalidName(
                "event name cannot contain whitespace or control characters".into(),
            ));
        }

        if !value.contains('.') {
            return Err(EventError::InvalidName(
                "event name must be qualified".into(),
            ));
        }

        if value.split('.').any(|part| {
            part.is_empty()
                || !part.chars().enumerate().all(|(index, c)| {
                    c.is_ascii_alphanumeric()
                        || c == '_'
                        || (index > 0 && c == '-')
                })
        }) {
            return Err(EventError::InvalidName(
                "event name contains an invalid identifier component".into(),
            ));
        }

        Ok(Self(value))
    }

    /// Return the event name as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for EventName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Identity of the logical entity associated with an event.
///
/// These identifiers refer to logical distributed entities.
/// They are not hostnames, IP addresses, operating-system process IDs,
/// physical device identifiers, or memory addresses.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum EventSubject {
    /// Distributed task.
    Task(TaskId),

    /// Distributed execution context.
    ExecutionContext(ExecutionContextId),

    /// Distributed execution plan.
    Plan(PlanId),

    /// Logical participant.
    Participant(ParticipantId),

    /// Logical operation.
    Operation(OperationId),

    /// Logical transport message.
    Message(MessageId),

    /// Event concerning the distributed runtime as a whole.
    Runtime,

    /// Provider-defined logical entity.
    ///
    /// The namespace identifies the subsystem or provider that
    /// defines the entity's identity.
    Extension {
        /// Owning namespace.
        namespace: String,

        /// Logical entity identifier.
        identity: String,
    },
}

/// Timestamp of an observation.
///
/// Unix time is represented as seconds plus nanoseconds.
/// Nanoseconds must be less than one billion.
///
/// This is wall-clock observation time, not a distributed ordering
/// guarantee. Cross-participant causality must use explicit correlation,
/// causation, lifecycle revisions, or the appropriate clock protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventTimestamp {
    unix_seconds: u64,
    nanoseconds: u32,
}

impl EventTimestamp {
    /// Construct a timestamp from Unix seconds and nanoseconds.
    pub fn new(
        unix_seconds: u64,
        nanoseconds: u32,
    ) -> EventResult<Self> {
        if nanoseconds >= 1_000_000_000 {
            return Err(EventError::InvalidTimestamp);
        }

        Ok(Self {
            unix_seconds,
            nanoseconds,
        })
    }

    /// Obtain the current system wall-clock timestamp.
    ///
    /// A system clock earlier than the Unix epoch is reported rather
    /// than silently replaced with an invented timestamp.
    pub fn now() -> EventResult<Self> {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| EventError::ClockUnavailable)?;

        let unix_seconds = u64::try_from(duration.as_secs())
            .map_err(|_| EventError::TimestampOutOfRange)?;

        Ok(Self {
            unix_seconds,
            nanoseconds: duration.subsec_nanos(),
        })
    }

    /// Return Unix seconds.
    #[must_use]
    pub const fn unix_seconds(self) -> u64 {
        self.unix_seconds
    }

    /// Return nanoseconds within the second.
    #[must_use]
    pub const fn nanoseconds(self) -> u32 {
        self.nanoseconds
    }
}

/// Structured, non-payload event attribute value.
///
/// Attribute values are intended for operational metadata only.
/// They must never contain credentials, authentication tokens,
/// private keys, checkpoint contents, or private execution payloads.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum EventValue {
    /// Boolean observation.
    Boolean(bool),

    /// Signed integer observation.
    Signed(i128),

    /// Unsigned integer observation.
    Unsigned(u128),

    /// Public descriptive string.
    ///
    /// The caller must redact sensitive content before insertion.
    Text(String),

    /// Stable machine-readable code.
    Code(String),
}

/// Structured event attributes.
///
/// BTreeMap provides deterministic iteration order.
///
/// Keys should be qualified and stable. Attribute values must contain
/// operational metadata rather than private computational data.
pub type EventAttributes = BTreeMap<String, EventValue>;

/// Result classification of an observed operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum EventOutcome {
    /// Operation was accepted.
    Accepted,

    /// Operation completed successfully.
    Succeeded,

    /// Operation failed.
    Failed,

    /// Operation was rejected.
    Rejected,

    /// Operation was cancelled.
    Cancelled,

    /// Operation remains in progress.
    InProgress,

    /// Outcome cannot currently be established.
    Unknown,

    /// Operation is not supported by the selected provider.
    Unsupported,

    /// Operation requires a new planning decision.
    RequiresReplanning,
}

impl EventOutcome {
    /// Return a stable machine-readable outcome identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::Rejected => "rejected",
            Self::Cancelled => "cancelled",
            Self::InProgress => "in_progress",
            Self::Unknown => "unknown",
            Self::Unsupported => "unsupported",
            Self::RequiresReplanning => "requires_replanning",
        }
    }
}

/// Event correlation context.
///
/// Correlation IDs connect observations belonging to the same logical
/// request or distributed operation.
///
/// Causation identifies the message that directly caused an observation,
/// when applicable.
///
/// These identifiers are references, not proof of authorization,
/// successful delivery, or execution.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EventContext {
    correlation_id: Option<CorrelationId>,
    causation_message_id: Option<MessageId>,
}

impl EventContext {
    /// Construct an empty event context.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            correlation_id: None,
            causation_message_id: None,
        }
    }

    /// Associate a correlation identity.
    #[must_use]
    pub fn with_correlation(
        mut self,
        correlation_id: CorrelationId,
    ) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }

    /// Associate the message that directly caused the observation.
    #[must_use]
    pub fn with_causation(
        mut self,
        message_id: MessageId,
    ) -> Self {
        self.causation_message_id = Some(message_id);
        self
    }

    /// Return the correlation identity.
    #[must_use]
    pub fn correlation_id(&self) -> Option<&CorrelationId> {
        self.correlation_id.as_ref()
    }

    /// Return the causation message identity.
    #[must_use]
    pub fn causation_message_id(&self) -> Option<&MessageId> {
        self.causation_message_id.as_ref()
    }
}

/// Immutable structured distributed observation.
///
/// An event describes an observation. It is not an instruction to
/// perform the operation represented by that observation.
///
/// Event identity is assigned by the event sink when durable storage
/// requires a globally unique storage identifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DistributedEvent {
    category: EventCategory,
    name: EventName,
    timestamp: EventTimestamp,
    subject: EventSubject,
    outcome: EventOutcome,
    context: EventContext,
    attributes: EventAttributes,
}

impl DistributedEvent {
    /// Construct an event with an explicit timestamp.
    ///
    /// Supplying the timestamp explicitly allows deterministic tests,
    /// replay, and integration with externally established clocks.
    pub fn new(
        category: EventCategory,
        name: EventName,
        timestamp: EventTimestamp,
        subject: EventSubject,
        outcome: EventOutcome,
    ) -> Self {
        Self {
            category,
            name,
            timestamp,
            subject,
            outcome,
            context: EventContext::new(),
            attributes: EventAttributes::new(),
        }
    }

    /// Construct an event using the current system clock.
    pub fn observed(
        category: EventCategory,
        name: EventName,
        subject: EventSubject,
        outcome: EventOutcome,
    ) -> EventResult<Self> {
        Ok(Self::new(
            category,
            name,
            EventTimestamp::now()?,
            subject,
            outcome,
        ))
    }

    /// Attach correlation and causation context.
    #[must_use]
    pub fn with_context(mut self, context: EventContext) -> Self {
        self.context = context;
        self
    }

    /// Insert a structured operational attribute.
    ///
    /// Attribute names must be qualified.
    ///
    /// This method deliberately does not accept arbitrary serialized
    /// execution payloads.
    pub fn with_attribute(
        mut self,
        key: impl Into<String>,
        value: EventValue,
    ) -> EventResult<Self> {
        let key = key.into();

        if key.is_empty()
            || key.trim() != key
            || !key.contains('.')
            || key.chars().any(|c| {
                c.is_whitespace() || c.is_control()
            })
        {
            return Err(EventError::InvalidAttributeKey);
        }

        self.attributes.insert(key, value);

        Ok(self)
    }

    /// Return the event category.
    #[must_use]
    pub const fn category(&self) -> EventCategory {
        self.category
    }

    /// Return the qualified event name.
    #[must_use]
    pub fn name(&self) -> &EventName {
        &self.name
    }

    /// Return the occurrence timestamp.
    #[must_use]
    pub const fn timestamp(&self) -> EventTimestamp {
        self.timestamp
    }

    /// Return the logical event subject.
    #[must_use]
    pub fn subject(&self) -> &EventSubject {
        &self.subject
    }

    /// Return the event outcome.
    #[must_use]
    pub const fn outcome(&self) -> EventOutcome {
        self.outcome
    }

    /// Return correlation and causation context.
    #[must_use]
    pub fn context(&self) -> &EventContext {
        &self.context
    }

    /// Return structured operational attributes.
    #[must_use]
    pub fn attributes(&self) -> &EventAttributes {
        &self.attributes
    }
}

/// Errors associated with observability event construction and delivery.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum EventError {
    /// Event name is invalid.
    InvalidName(String),

    /// Event timestamp is invalid.
    InvalidTimestamp,

    /// System clock is unavailable.
    ClockUnavailable,

    /// Timestamp exceeds the supported representation.
    TimestampOutOfRange,

    /// Attribute key is invalid.
    InvalidAttributeKey,

    /// Event observer rejected or failed to accept an event.
    SinkFailure(String),

    /// Event observer registry synchronization failed.
    ObserverRegistryUnavailable,

    /// One or more event observers failed.
    MultipleObserverFailures {
        /// Number of observers that failed.
        failures: usize,

        /// Descriptions of observer failures.
        descriptions: Vec<String>,
    },
}

impl fmt::Display for EventError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidName(message) => {
                write!(f, "invalid observability event name: {message}")
            }
            Self::InvalidTimestamp => {
                f.write_str("invalid observability event timestamp")
            }
            Self::ClockUnavailable => {
                f.write_str("system clock is unavailable")
            }
            Self::TimestampOutOfRange => {
                f.write_str("event timestamp exceeds supported range")
            }
            Self::InvalidAttributeKey => {
                f.write_str("invalid observability attribute key")
            }
            Self::SinkFailure(message) => {
                write!(f, "observability event sink failed: {message}")
            }
            Self::ObserverRegistryUnavailable => {
                f.write_str("observability observer registry unavailable")
            }
            Self::MultipleObserverFailures {
                failures,
                descriptions,
            } => {
                write!(
                    f,
                    "{failures} observability observer(s) failed"
                )?;

                for description in descriptions {
                    write!(f, "; {description}")?;
                }

                Ok(())
            }
        }
    }
}

impl std::error::Error for EventError {}

/// Contract implemented by an observability event consumer.
///
/// Implementations may forward events to:
///
/// - an in-memory collector;
/// - a structured logging system;
/// - a persistent event store;
/// - a metrics or tracing adapter;
/// - a remote telemetry service;
/// - a future observability provider.
///
/// The distributed core does not prescribe a particular backend.
///
/// Implementations must not modify program semantics.
///
/// Implementations must define their own persistence, backpressure,
/// retention, ordering, and resource-budget behavior.
pub trait EventSink: Send + Sync + fmt::Debug {
    /// Publish one immutable observation.
    ///
    /// Returning an error means that this observer did not confirm
    /// successful acceptance. It does not mean that the underlying
    /// computational operation failed.
    fn emit(&self, event: &DistributedEvent) -> EventResult<()>;
}

/// Optional collection of event observers.
///
/// This is a process-local fan-out adapter, not a durable event broker.
///
/// Observer failures are collected and reported explicitly.
/// A failure in one observer does not prevent attempting delivery
/// to the remaining observers.
///
/// No event queue or unbounded background worker is created here.
#[derive(Debug, Default)]
pub struct CompositeEventSink {
    observers: RwLock<Vec<Arc<dyn EventSink>>>,
}

impl CompositeEventSink {
    /// Construct an empty observer collection.
    #[must_use]
    pub fn new() -> Self {
        Self {
            observers: RwLock::new(Vec::new()),
        }
    }

    /// Register an additional observer.
    ///
    /// The caller controls observer lifecycle through its Arc ownership.
    pub fn register(
        &self,
        observer: Arc<dyn EventSink>,
    ) -> EventResult<()> {
        let mut observers = self
            .observers
            .write()
            .map_err(|_| EventError::ObserverRegistryUnavailable)?;

        observers.push(observer);

        Ok(())
    }

    /// Return the number of registered observers.
    pub fn observer_count(&self) -> EventResult<usize> {
        let observers = self
            .observers
            .read()
            .map_err(|_| EventError::ObserverRegistryUnavailable)?;

        Ok(observers.len())
    }

    /// Attempt delivery to every registered observer.
    ///
    /// A failed observer does not prevent subsequent observers from
    /// receiving the same event.
    ///
    /// The result reports all observer failures encountered.
    pub fn emit_all(
        &self,
        event: &DistributedEvent,
    ) -> EventResult<()> {
        let observers = self
            .observers
            .read()
            .map_err(|_| EventError::ObserverRegistryUnavailable)?;

        let mut failures = Vec::new();

        for observer in observers.iter() {
            if let Err(error) = observer.emit(event) {
                failures.push(error.to_string());
            }
        }

        if failures.is_empty() {
            Ok(())
        } else {
            Err(EventError::MultipleObserverFailures {
                failures: failures.len(),
                descriptions: failures,
            })
        }
    }
}

impl EventSink for CompositeEventSink {
    fn emit(&self, event: &DistributedEvent) -> EventResult<()> {
        self.emit_all(event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[derive(Debug)]
    struct CountingSink {
        count: AtomicUsize,
    }

    impl CountingSink {
        fn new() -> Self {
            Self {
                count: AtomicUsize::new(0),
            }
        }

        fn count(&self) -> usize {
            self.count.load(Ordering::Relaxed)
        }
    }

    impl EventSink for CountingSink {
        fn emit(&self, _event: &DistributedEvent) -> EventResult<()> {
            self.count.fetch_add(1, Ordering::Relaxed);
            Ok(())
        }
    }

    #[derive(Debug)]
    struct FailingSink;

    impl EventSink for FailingSink {
        fn emit(&self, _event: &DistributedEvent) -> EventResult<()> {
            Err(EventError::SinkFailure(
                "test observer failure".into(),
            ))
        }
    }

    fn sample_event() -> DistributedEvent {
        let name = EventName::new("execution.task_started")
            .expect("valid event name");

        let subject = EventSubject::Runtime;

        DistributedEvent::new(
            EventCategory::Execution,
            name,
            EventTimestamp::new(1, 0).expect("valid timestamp"),
            subject,
            EventOutcome::InProgress,
        )
    }

    #[test]
    fn event_name_accepts_open_world_identifiers() {
        assert!(
            EventName::new("execution.task_started").is_ok()
        );

        assert!(
            EventName::new("vendor.future_domain.operation")
                .is_ok()
        );
    }

    #[test]
    fn event_name_rejects_invalid_identifiers() {
        assert!(EventName::new("").is_err());
        assert!(EventName::new("execution").is_err());
        assert!(EventName::new("execution task.started").is_err());
        assert!(EventName::new("execution..started").is_err());
    }

    #[test]
    fn timestamp_validates_nanoseconds() {
        assert!(EventTimestamp::new(1, 999_999_999).is_ok());
        assert!(EventTimestamp::new(1, 1_000_000_000).is_err());
    }

    #[test]
    fn event_attributes_are_deterministically_ordered() {
        let event = sample_event()
            .with_attribute(
                "execution.worker_count",
                EventValue::Unsigned(8),
            )
            .unwrap()
            .with_attribute(
                "execution.parallel",
                EventValue::Boolean(true),
            )
            .unwrap();

        let keys: Vec<&String> =
            event.attributes().keys().collect();

        assert_eq!(
            keys,
            vec![
                &"execution.parallel".to_string(),
                &"execution.worker_count".to_string(),
            ]
        );
    }

    #[test]
    fn event_context_preserves_correlation() {
        let correlation = CorrelationId::new(1, 1).unwrap();

        let context = EventContext::new()
            .with_correlation(correlation);

        assert!(context.correlation_id().is_some());
    }

    #[test]
    fn composite_sink_delivers_to_all_observers() {
        let composite = CompositeEventSink::new();

        let first = Arc::new(CountingSink::new());
        let second = Arc::new(CountingSink::new());

        composite.register(first.clone()).unwrap();
        composite.register(second.clone()).unwrap();

        composite.emit_all(&sample_event()).unwrap();

        assert_eq!(first.count(), 1);
        assert_eq!(second.count(), 1);
    }

    #[test]
    fn observer_failure_does_not_block_other_observers() {
        let composite = CompositeEventSink::new();

        let failing = Arc::new(FailingSink);
        let counting = Arc::new(CountingSink::new());

        composite.register(failing).unwrap();
        composite.register(counting.clone()).unwrap();

        let result = composite.emit_all(&sample_event());

        assert!(result.is_err());
        assert_eq!(counting.count(), 1);
    }

    #[test]
    fn empty_composite_is_valid() {
        let composite = CompositeEventSink::new();

        assert_eq!(composite.observer_count().unwrap(), 0);
        assert!(composite.emit_all(&sample_event()).is_ok());
    }
}
