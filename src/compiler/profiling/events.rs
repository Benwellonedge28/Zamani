
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! Zamani Universal Meta-Compiler — Profiling Events.
//!
//! Path:
//!     src/compiler/profiling/events.rs
//!
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Defines structured, portable observations of compiler activity.
//!
//! Compiler profiling events describe facts established by compiler
//! stages. They do not execute stages, change compilation state,
//! allocate resources, perform hardware discovery, or modify program
//! semantics.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - Compiler profiling event classification.
//! - Compiler stage lifecycle observations.
//! - Compiler profiling event construction.
//! - Compilation and stage correlation metadata.
//! - Compiler-specific event attributes.
//! - Conversion into canonical distributed events.
//! - Publication through existing distributed event sinks.
//!
//! This module does not own:
//!
//! - Compiler stage identity allocation.
//! - Compilation lifecycle state.
//! - Pipeline orchestration.
//! - Stage execution.
//! - Generic event transport.
//! - Distributed event persistence.
//! - Distributed tracing.
//! - Generic metrics.
//! - Profiling measurements and aggregation.
//! - Hardware performance counters.
//! - Hardware discovery.
//! - Hardware allocation.
//! - Optimization algorithms.
//! - ZUIR or domain-specific IR.
//! - Runtime execution.
//!
//! # Canonical authorities
//!
//! Compiler stage identity:
//!     crate::compiler::pipeline::stage
//!
//! Compiler execution context:
//!     crate::compiler::context::compilation_context
//!
//! Compiler lifecycle:
//!     crate::compiler::context::compilation_state
//!
//! Universal event representation:
//!     crate::distributed::observability::events
//!
//! Distributed trace propagation:
//!     crate::distributed::observability::tracing
//!
//! Compiler profiling measurements:
//!     crate::compiler::profiling::measurements
//!
//! Compiler profiling aggregation:
//!     crate::compiler::profiling::aggregation
//!
//! # POCO-REAF
//!
//! Profiling events are independent of:
//!
//! - CPU architecture;
//! - GPU architecture;
//! - FPGA architecture;
//! - ASIC architecture;
//! - QPU architecture;
//! - accelerator vendors;
//! - operating systems;
//! - physical memory size;
//! - machine count;
//! - worker count;
//! - execution topology;
//! - computational domain.
//!
//! # Scalability
//!
//! This module introduces no arbitrary universal maximum for:
//!
//! - profiling events;
//! - compiler stages;
//! - compilation units;
//! - compilation sessions;
//! - event attributes;
//! - computational domains;
//! - target architectures;
//! - hardware devices;
//! - workers;
//! - operations.
//!
//! Storage, throughput, buffering, retention, and resource
//! consumption are responsibilities of configured providers.
//!
//! Rust's actual representation and available resources remain
//! the physical implementation boundaries.
//!
//! # Determinism
//!
//! Event metadata uses stable identifiers.
//!
//! Event attributes use BTreeMap.
//!
//! Explicit timestamps support deterministic tests.
//!
//! Wall-clock timestamps describe observation time and must not
//! be mistaken for distributed causality or deterministic ordering.
//!
//! # Security
//!
//! Events must contain operational metadata only.
//!
//! Producers are responsible for redacting sensitive information.
//!
//! Arbitrary source contents, compiler payloads, credentials,
//! private keys, authentication tokens, and execution data must
//! not be inserted as profiling attributes.
//!
//! # Semantic independence
//!
//! Profiling is optional instrumentation.
//!
//! Failure to publish a profiling event does not mean that
//! compilation failed.
//!
//! Event publication failures are returned separately so that
//! the caller can apply its configured observability policy.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;
use std::time::Duration;

use crate::compiler::pipeline::stage::StageId;

use crate::distributed::observability::events::{
    DistributedEvent,
    EventCategory,
    EventContext,
    EventError,
    EventName,
    EventOutcome,
    EventResult,
    EventSink,
    EventSubject,
    EventTimestamp,
    EventValue,
};

/// Public API version for compiler profiling events.
///
/// Independent of the Zamani language, grammar, compiler release,
/// semantic model, IR, and backend ABI versions.
pub const COMPILER_PROFILE_EVENTS_API_VERSION: u16 = 1;

/// Result returned by compiler profiling event operations.
pub type CompilerProfileEventResult<T> =
    Result<T, CompilerProfileEventError>;

/// Stable classification of compiler stage lifecycle observations.
///
/// This enum describes broad stage lifecycle transitions.
/// It deliberately does not enumerate individual compiler stages.
///
/// Stage identity is provided by the canonical StageId contract.
///
/// Future compiler stages therefore do not require modifications
/// to this enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum StageEventKind {
    /// A compiler stage has started execution.
    Started,

    /// A compiler stage completed successfully.
    Completed,

    /// A compiler stage failed.
    Failed,

    /// A compiler stage was intentionally skipped.
    ///
    /// Skipping must be established by the pipeline's execution
    /// contract, not inferred from the absence of an event.
    Skipped,
}

impl StageEventKind {
    /// Returns the stable machine-readable event identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Started => "started",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Skipped => "skipped",
        }
    }

    /// Returns the canonical distributed event outcome.
    #[must_use]
    pub const fn outcome(self) -> EventOutcome {
        match self {
            Self::Started => EventOutcome::InProgress,
            Self::Completed => EventOutcome::Succeeded,
            Self::Failed => EventOutcome::Failed,
            Self::Skipped => EventOutcome::Unknown,
        }
    }

    /// Returns the canonical event name.
    #[must_use]
    pub const fn event_name(self) -> &'static str {
        match self {
            Self::Started => "compiler.stage.started",
            Self::Completed => "compiler.stage.completed",
            Self::Failed => "compiler.stage.failed",
            Self::Skipped => "compiler.stage.skipped",
        }
    }
}

impl fmt::Display for StageEventKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Errors associated with compiler profiling event construction
/// and publication.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CompilerProfileEventError {
    /// An invalid compilation identity was supplied.
    InvalidCompilationIdentity,

    /// An invalid parent stage identity was supplied.
    InvalidParentStageIdentity,

    /// A profiling attribute key is invalid.
    InvalidAttributeKey,

    /// The canonical event name could not be constructed.
    InvalidEventName(String),

    /// Canonical distributed event construction failed.
    Distributed(EventError),
}

impl fmt::Display for CompilerProfileEventError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidCompilationIdentity => {
                formatter.write_str(
                    "invalid compiler profiling compilation identity",
                )
            }

            Self::InvalidParentStageIdentity => {
                formatter.write_str(
                    "invalid compiler profiling parent stage identity",
                )
            }

            Self::InvalidAttributeKey => {
                formatter.write_str(
                    "invalid compiler profiling attribute key",
                )
            }

            Self::InvalidEventName(message) => {
                write!(
                    formatter,
                    "invalid compiler profiling event name: {message}"
                )
            }

            Self::Distributed(error) => {
                write!(
                    formatter,
                    "distributed profiling event error: {error}"
                )
            }
        }
    }
}

impl Error for CompilerProfileEventError {}

impl From<EventError> for CompilerProfileEventError {
    fn from(error: EventError) -> Self {
        Self::Distributed(error)
    }
}

/// Immutable structured observation of compiler stage activity.
///
/// Stage identity is borrowed from the canonical compiler pipeline
/// stage contract through an owned StageId clone.
///
/// This module does not create a competing stage identity system.
///
/// Compilation identity is optional because the canonical compiler
/// context remains responsible for its own identity and lifecycle.
///
/// Duration is supplied by the stage execution instrumentation.
/// This event module does not independently measure or infer duration.
///
/// Attributes are ordered deterministically.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompilerProfileEvent {
    stage: StageId,
    kind: StageEventKind,
    timestamp: EventTimestamp,
    compilation_identity: Option<String>,
    parent_stage: Option<StageId>,
    duration: Option<Duration>,
    context: EventContext,
    attributes: BTreeMap<String, EventValue>,
}

impl CompilerProfileEvent {
    /// Constructs a compiler profiling event using an explicit
    /// timestamp.
    ///
    /// Explicit timestamps support deterministic testing, replay,
    /// and integration with externally established clocks.
    #[must_use]
    pub fn new(
        stage: StageId,
        kind: StageEventKind,
        timestamp: EventTimestamp,
    ) -> Self {
        Self {
            stage,
            kind,
            timestamp,
            compilation_identity: None,
            parent_stage: None,
            duration: None,
            context: EventContext::new(),
            attributes: BTreeMap::new(),
        }
    }

    /// Constructs an event using the current system wall clock.
    ///
    /// Clock failures are returned explicitly.
    pub fn observed(
        stage: StageId,
        kind: StageEventKind,
    ) -> CompilerProfileEventResult<Self> {
        let timestamp = EventTimestamp::now()?;

        Ok(Self::new(stage, kind, timestamp))
    }

    /// Associates the logical compilation identity.
    ///
    /// The identity is descriptive correlation metadata.
    ///
    /// This method does not allocate a compilation identifier.
    ///
    /// The authoritative identifier must be supplied by the
    /// compiler session or compilation-context owner.
    pub fn with_compilation_identity(
        mut self,
        identity: impl Into<String>,
    ) -> CompilerProfileEventResult<Self> {
        let identity = identity.into();

        if identity.trim().is_empty()
            || identity.chars().any(char::is_control)
        {
            return Err(
                CompilerProfileEventError::InvalidCompilationIdentity,
            );
        }

        self.compilation_identity = Some(identity);

        Ok(self)
    }

    /// Associates the parent compiler stage.
    ///
    /// The parent relationship describes pipeline execution
    /// context. It does not independently establish execution
    /// ordering or dependency legality.
    #[must_use]
    pub fn with_parent_stage(
        mut self,
        parent: StageId,
    ) -> Self {
        self.parent_stage = Some(parent);
        self
    }

    /// Associates an externally measured stage duration.
    ///
    /// The duration must be measured by the authoritative stage
    /// execution instrumentation.
    ///
    /// This method does not read clocks or execute profiling logic.
    #[must_use]
    pub fn with_duration(
        mut self,
        duration: Duration,
    ) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Associates distributed correlation and causation metadata.
    ///
    /// Correlation and causation identities remain owned by
    /// distributed::ids.
    ///
    /// Trace propagation remains owned by distributed observability.
    #[must_use]
    pub fn with_context(
        mut self,
        context: EventContext,
    ) -> Self {
        self.context = context;
        self
    }

    /// Inserts one compiler profiling attribute.
    ///
    /// The supplied key is placed under the namespace:
    ///
    /// compiler.stage.attribute
    ///
    /// Dotted subkeys are permitted.
    ///
    /// Attribute values must contain operational metadata only.
    ///
    /// Arbitrary execution payloads and private source contents
    /// must not be inserted.
    pub fn with_attribute(
        mut self,
        key: impl Into<String>,
        value: EventValue,
    ) -> CompilerProfileEventResult<Self> {
        let key = key.into();

        validate_attribute_key(&key)?;

        let qualified_key =
            format!("compiler.stage.attribute.{key}");

        self.attributes.insert(qualified_key, value);

        Ok(self)
    }

    /// Returns the compiler stage identity.
    #[must_use]
    pub fn stage(&self) -> &StageId {
        &self.stage
    }

    /// Returns the stage lifecycle event classification.
    #[must_use]
    pub const fn kind(&self) -> StageEventKind {
        self.kind
    }

    /// Returns the event timestamp.
    #[must_use]
    pub const fn timestamp(&self) -> EventTimestamp {
        self.timestamp
    }

    /// Returns the optional compilation identity.
    #[must_use]
    pub fn compilation_identity(&self) -> Option<&str> {
        self.compilation_identity.as_deref()
    }

    /// Returns the optional parent stage.
    #[must_use]
    pub fn parent_stage(&self) -> Option<&StageId> {
        self.parent_stage.as_ref()
    }

    /// Returns the externally measured stage duration.
    #[must_use]
    pub const fn duration(&self) -> Option<Duration> {
        self.duration
    }

    /// Returns the distributed correlation context.
    #[must_use]
    pub fn context(&self) -> &EventContext {
        &self.context
    }

    /// Returns the deterministically ordered profiling attributes.
    #[must_use]
    pub fn attributes(&self) -> &BTreeMap<String, EventValue> {
        &self.attributes
    }

    /// Converts the compiler profiling observation into the
    /// canonical distributed event representation.
    ///
    /// The existing distributed event system remains the sole
    /// owner of generic event representation and delivery.
    ///
    /// Compiler profiling does not introduce a second event
    /// transport, journal, or observer registry.
    pub fn to_distributed_event(
        &self,
    ) -> EventResult<DistributedEvent> {
        let name = EventName::new(
            self.kind.event_name(),
        )?;

        let subject = EventSubject::Extension {
            namespace: "zamani.compiler.stage".to_owned(),
            identity: self.stage.as_str().to_owned(),
        };

        let mut event = DistributedEvent::new(
            EventCategory::Integration,
            name,
            self.timestamp,
            subject,
            self.kind.outcome(),
        )
        .with_context(self.context.clone());

        event = event.with_attribute(
            "compiler.stage.id",
            EventValue::Text(
                self.stage.as_str().to_owned(),
            ),
        )?;

        event = event.with_attribute(
            "compiler.stage.event_kind",
            EventValue::Code(
                self.kind.as_str().to_owned(),
            ),
        )?;

        if let Some(identity) = &self.compilation_identity {
            event = event.with_attribute(
                "compiler.compilation.identity",
                EventValue::Text(identity.clone()),
            )?;
        }

        if let Some(parent) = &self.parent_stage {
            event = event.with_attribute(
                "compiler.stage.parent",
                EventValue::Text(
                    parent.as_str().to_owned(),
                ),
            )?;
        }

        if let Some(duration) = self.duration {
            event = event.with_attribute(
                "compiler.stage.duration_nanoseconds",
                EventValue::Unsigned(duration.as_nanos()),
            )?;
        }

        for (key, value) in &self.attributes {
            event = event.with_attribute(
                key.clone(),
                value.clone(),
            )?;
        }

        Ok(event)
    }

    /// Publishes this observation through the canonical distributed
    /// event sink.
    ///
    /// A successful return means that the supplied sink confirmed
    /// acceptance of the event.
    ///
    /// It does not mean that the compiler stage succeeded.
    ///
    /// Event delivery failure must be handled independently from
    /// the compiler stage's execution result.
    pub fn emit_to(
        &self,
        sink: &dyn EventSink,
    ) -> EventResult<()> {
        let event = self.to_distributed_event()?;

        sink.emit(&event)
    }
}

/// Validates a compiler profiling attribute subkey.
///
/// Attribute names must:
///
/// - Be nonempty.
/// - Have no surrounding whitespace.
/// - Contain no whitespace.
/// - Contain no control characters.
/// - Not contain empty dotted components.
///
/// No arbitrary maximum attribute-name length is imposed.
fn validate_attribute_key(
    key: &str,
) -> CompilerProfileEventResult<()> {
    if key.is_empty()
        || key.trim() != key
        || key.chars().any(|character| {
            character.is_whitespace()
                || character.is_control()
        })
        || key.split('.').any(str::is_empty)
    {
        return Err(
            CompilerProfileEventError::InvalidAttributeKey,
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stage(value: &str) -> StageId {
        StageId::new(value.to_owned())
            .expect("valid test stage identifier")
    }

    fn timestamp() -> EventTimestamp {
        EventTimestamp::new(1_800_000_000, 123_456_789)
            .expect("valid test timestamp")
    }

    #[test]
    fn stage_event_kinds_have_stable_names() {
        assert_eq!(
            StageEventKind::Started.event_name(),
            "compiler.stage.started"
        );

        assert_eq!(
            StageEventKind::Completed.event_name(),
            "compiler.stage.completed"
        );

        assert_eq!(
            StageEventKind::Failed.event_name(),
            "compiler.stage.failed"
        );

        assert_eq!(
            StageEventKind::Skipped.event_name(),
            "compiler.stage.skipped"
        );
    }

    #[test]
    fn event_construction_preserves_stage_identity() {
        let event = CompilerProfileEvent::new(
            stage("zamani.compiler.optimization"),
            StageEventKind::Started,
            timestamp(),
        );

        assert_eq!(
            event.stage().as_str(),
            "zamani.compiler.optimization"
        );

        assert_eq!(
            event.kind(),
            StageEventKind::Started
        );
    }

    #[test]
    fn event_preserves_explicit_timestamp() {
        let expected = timestamp();

        let event = CompilerProfileEvent::new(
            stage("zamani.compiler.frontend"),
            StageEventKind::Completed,
            expected,
        );

        assert_eq!(event.timestamp(), expected);
    }

    #[test]
    fn compilation_identity_is_preserved() {
        let event = CompilerProfileEvent::new(
            stage("zamani.compiler.semantic"),
            StageEventKind::Completed,
            timestamp(),
        )
        .with_compilation_identity("compilation-001")
        .expect("valid compilation identity");

        assert_eq!(
            event.compilation_identity(),
            Some("compilation-001")
        );
    }

    #[test]
    fn empty_compilation_identity_is_rejected() {
        let result = CompilerProfileEvent::new(
            stage("zamani.compiler.semantic"),
            StageEventKind::Completed,
            timestamp(),
        )
        .with_compilation_identity(" ");

        assert!(matches!(
            result,
            Err(
                CompilerProfileEventError::
                    InvalidCompilationIdentity
            )
        ));
    }

    #[test]
    fn parent_stage_is_preserved() {
        let parent = stage("zamani.compiler.pipeline");

        let event = CompilerProfileEvent::new(
            stage("zamani.compiler.lowering"),
            StageEventKind::Started,
            timestamp(),
        )
        .with_parent_stage(parent.clone());

        assert_eq!(
            event.parent_stage(),
            Some(&parent)
        );
    }

    #[test]
    fn duration_is_preserved_without_remeasurement() {
        let duration = Duration::from_nanos(987_654);

        let event = CompilerProfileEvent::new(
            stage("zamani.compiler.backend"),
            StageEventKind::Completed,
            timestamp(),
        )
        .with_duration(duration);

        assert_eq!(event.duration(), Some(duration));
    }

    #[test]
    fn attributes_are_deterministically_ordered() {
        let event = CompilerProfileEvent::new(
            stage("zamani.compiler.optimization"),
            StageEventKind::Completed,
            timestamp(),
        )
        .with_attribute(
            "zeta",
            EventValue::Unsigned(2),
        )
        .unwrap()
        .with_attribute(
            "alpha",
            EventValue::Unsigned(1),
        )
        .unwrap();

        let keys: Vec<&String> =
            event.attributes().keys().collect();

        assert!(keys[0] < keys[1]);
    }

    #[test]
    fn invalid_attribute_names_are_rejected() {
        let result = CompilerProfileEvent::new(
            stage("zamani.compiler.optimization"),
            StageEventKind::Completed,
            timestamp(),
        )
        .with_attribute(
            "invalid attribute",
            EventValue::Boolean(true),
        );

        assert!(matches!(
            result,
            Err(
                CompilerProfileEventError::InvalidAttributeKey
            )
        ));
    }

    #[test]
    fn empty_attribute_components_are_rejected() {
        let result = CompilerProfileEvent::new(
            stage("zamani.compiler.optimization"),
            StageEventKind::Completed,
            timestamp(),
        )
        .with_attribute(
            "compiler..invalid",
            EventValue::Boolean(true),
        );

        assert!(matches!(
            result,
            Err(
                CompilerProfileEventError::InvalidAttributeKey
            )
        ));
    }

    #[test]
    fn event_converts_to_canonical_distributed_event() {
        let event = CompilerProfileEvent::new(
            stage("zamani.compiler.optimization"),
            StageEventKind::Completed,
            timestamp(),
        );

        let distributed = event
            .to_distributed_event()
            .expect("valid distributed event");

        assert_eq!(
            distributed.name().as_str(),
            "compiler.stage.completed"
        );

        assert_eq!(
            distributed.category(),
            EventCategory::Integration
        );

        assert_eq!(
            distributed.outcome(),
            EventOutcome::Succeeded
        );
    }

    #[test]
    fn converted_event_contains_stage_metadata() {
        let event = CompilerProfileEvent::new(
            stage("zamani.compiler.lowering"),
            StageEventKind::Completed,
            timestamp(),
        );

        let distributed = event
            .to_distributed_event()
            .unwrap();

        assert_eq!(
            distributed.attributes().get(
                "compiler.stage.id"
            ),
            Some(&EventValue::Text(
                "zamani.compiler.lowering".to_owned()
            ))
        );
    }

    #[test]
    fn converted_event_contains_duration_metadata() {
        let event = CompilerProfileEvent::new(
            stage("zamani.compiler.backend"),
            StageEventKind::Completed,
            timestamp(),
        )
        .with_duration(Duration::from_nanos(1234));

        let distributed = event
            .to_distributed_event()
            .unwrap();

        assert_eq!(
            distributed.attributes().get(
                "compiler.stage.duration_nanoseconds"
            ),
            Some(&EventValue::Unsigned(1234))
        );
    }

    #[test]
    fn failed_stage_is_not_reported_as_successful() {
        let event = CompilerProfileEvent::new(
            stage("zamani.compiler.backend"),
            StageEventKind::Failed,
            timestamp(),
        );

        let distributed = event
            .to_distributed_event()
            .unwrap();

        assert_eq!(
            distributed.outcome(),
            EventOutcome::Failed
        );
    }

    #[test]
    fn skipped_stage_is_not_reported_as_successful() {
        let event = CompilerProfileEvent::new(
            stage("zamani.compiler.quantum_lowering"),
            StageEventKind::Skipped,
            timestamp(),
        );

        let distributed = event
            .to_distributed_event()
            .unwrap();

        assert_eq!(
            distributed.outcome(),
            EventOutcome::Unknown
        );
    }

    #[test]
    fn nanosecond_duration_supports_large_measurements() {
        let duration = Duration::from_nanos(u64::MAX);

        let event = CompilerProfileEvent::new(
            stage("zamani.compiler.optimization"),
            StageEventKind::Completed,
            timestamp(),
        )
        .with_duration(duration);

        let distributed = event
            .to_distributed_event()
            .unwrap();

        assert_eq!(
            distributed.attributes().get(
                "compiler.stage.duration_nanoseconds"
            ),
            Some(&EventValue::Unsigned(
                duration.as_nanos()
            ))
        );
    }

    #[test]
    fn explicit_timestamps_produce_equivalent_metadata() {
        let first = CompilerProfileEvent::new(
            stage("zamani.compiler.optimization"),
            StageEventKind::Completed,
            timestamp(),
        );

        let second = CompilerProfileEvent::new(
            stage("zamani.compiler.optimization"),
            StageEventKind::Completed,
            timestamp(),
        );

        assert_eq!(
            first.to_distributed_event().unwrap(),
            second.to_distributed_event().unwrap()
        );
    }
}
