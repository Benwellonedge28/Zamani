
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani AI — Observability Events
//!
//! Path: src/ai/observability/events.rs
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # 1. PURPOSE
//!
//! Provides structured, portable observations for the AI computational
//! domain of Zamani.
//!
//! This module records facts established by AI-owned subsystems.
//!
//! It does not execute models, train networks, perform inference,
//! schedule workloads, allocate hardware, or change computation semantics.
//!
//! # 2. ARCHITECTURAL OWNERSHIP
//!
//! This module owns:
//!
//! - AI event domain classification.
//! - AI event subjects.
//! - AI-specific event construction.
//! - AI event naming conventions.
//! - AI event metadata.
//! - AI event sink interfaces.
//! - Optional composite event delivery.
//! - Integration with universal distributed event delivery.
//!
//! It does not own:
//!
//! - Model lifecycle state.
//! - Training lifecycle state.
//! - Dataset storage.
//! - Agent memory.
//! - Execution scheduling.
//! - Resource allocation.
//! - Distributed membership.
//! - Distributed identity allocation.
//! - Trace identity allocation.
//! - Generic metrics.
//! - Generic tracing.
//! - Event persistence.
//! - Authentication or authorization.
//! - AI semantic analysis.
//! - AST or IR representations.
//! - Hardware discovery.
//!
//! # 3. CANONICAL AUTHORITIES
//!
//! AI identities:
//!     crate::ai::ids
//!
//! Universal event delivery:
//!     crate::distributed::observability::events
//!
//! Distributed tracing:
//!     crate::distributed::observability::tracing
//!
//! AI provenance:
//!     crate::ai::provenance
//!
//! AI semantic authority:
//!     crate::frontend::semantic::domains::ai
//!
//! # 4. EVENT OWNERSHIP
//!
//! AI events describe observations from:
//!
//! - Models.
//! - Parameters.
//! - Datasets.
//! - Learning.
//! - Training.
//! - Inference.
//! - Reasoning.
//! - Knowledge.
//! - Evidence.
//! - Decisions.
//! - Agents.
//! - Pipelines.
//! - Differentiation.
//! - Optimization.
//! - Deployment.
//! - Security.
//! - Capabilities.
//! - Resource feasibility.
//! - External integrations.
//!
//! Event names are open-world identifiers.
//!
//! Adding a new AI event must not require modifying a closed event
//! enumeration.
//!
//! # 5. PORTABILITY
//!
//! AI events must remain independent of:
//!
//! - CPU architecture.
//! - GPU architecture.
//! - FPGA architecture.
//! - ASIC architecture.
//! - QPU architecture.
//! - Accelerator vendors.
//! - Operating systems.
//! - Physical memory size.
//! - Network topology.
//! - Number of machines.
//! - Number of workers.
//! - Number of AI models.
//!
//! # 6. SCALABILITY
//!
//! No arbitrary universal event, observer, model, or participant
//! capacity is imposed by this module.
//!
//! Actual consumption depends on:
//!
//! - Available memory.
//! - Processing capacity.
//! - Configured operational budgets.
//! - Selected event sinks.
//! - Storage providers.
//! - Retention policies.
//! - Backpressure policies.
//!
//! A resource limitation must produce an explicit error.
//!
//! This architecture does not promise mathematically infinite memory,
//! identifier space, storage, or event throughput.
//!
//! # 7. SEMANTIC INDEPENDENCE
//!
//! Instrumentation is optional.
//!
//! Failure to publish an event must not automatically mean that the
//! underlying AI computation failed.
//!
//! Event delivery errors are returned explicitly so callers can apply
//! their configured operational policies.
//!
//! # 8. SECURITY
//!
//! Event attributes are intended for operational metadata.
//!
//! Never publish:
//!
//! - Passwords.
//! - Authentication tokens.
//! - Private keys.
//! - Credentials.
//! - Private prompts.
//! - Unrestricted model inputs.
//! - Unrestricted model outputs.
//! - Private checkpoint contents.
//! - Sensitive personal information.
//!
//! Producers remain responsible for data classification and redaction.
//!
//! This module rejects several commonly sensitive attribute names,
//! but cannot establish the sensitivity of arbitrary string contents.
//!
//! # 9. DETERMINISM
//!
//! AI event metadata uses the canonical distributed event representation.
//!
//! Attribute ordering is deterministic through its BTreeMap representation.
//!
//! Wall-clock timestamps represent observation time and are not a
//! distributed causality guarantee.
//!
//! # 10. INTEGRATION
//!
//! AI subsystem:
//!     crate::ai::observability
//!
//! Distributed event subsystem:
//!     crate::distributed::observability::events
//!
//! Distributed tracing:
//!     crate::distributed::observability::tracing
//!
//! Execution, learning, inference, reasoning, agents and pipelines
//! provide observations to this module.
//!
//! They retain ownership of their actual execution state.

// ============================================================================
// IMPORTS
// ============================================================================

use core::fmt;
use std::sync::{Arc, RwLock};

use crate::ai::ids::{
    AiAgentId,
    AiArtifactId,
    AiDatasetId,
    AiDecisionId,
    AiEvidenceId,
    AiKnowledgeId,
    AiModelId,
    AiParameterId,
    AiPipelineId,
    AiTrainingRunId,
    OperationId,
};

use crate::distributed::observability::events::{
    DistributedEvent,
    EventCategory,
    EventContext,
    EventError,
    EventName,
    EventOutcome,
    EventResult,
    EventSink as DistributedEventSink,
    EventSubject,
    EventTimestamp,
    EventValue,
};

// ============================================================================
// RESULT TYPES
// ============================================================================

/// Result of an AI event operation.
pub type AiEventResult<T> = Result<T, AiEventError>;

// ============================================================================
// AI EVENT DOMAIN
// ============================================================================

/// Broad classification of AI computational observations.
///
/// Individual event names remain extensible.
///
/// This enumeration classifies event ownership rather than
/// enumerating every possible AI operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AiEventDomain {
    /// AI model definition and lifecycle.
    Model,

    /// Model parameter operations.
    Parameter,

    /// Dataset operations.
    Data,

    /// Learning operations.
    Learning,

    /// Model training operations.
    Training,

    /// Model inference operations.
    Inference,

    /// Symbolic and computational reasoning.
    Reasoning,

    /// Knowledge operations.
    Knowledge,

    /// Evidence operations.
    Evidence,

    /// Decision operations.
    Decision,

    /// AI agent operations.
    Agent,

    /// AI pipeline operations.
    Pipeline,

    /// Automatic differentiation.
    Differentiation,

    /// AI computational optimization.
    Optimization,

    /// Model deployment and artifacts.
    Deployment,

    /// AI security and governance.
    Security,

    /// AI capability negotiation.
    Capability,

    /// AI resource requirements and feasibility.
    Resources,

    /// External AI integrations.
    Integration,

    /// AI subsystem lifecycle.
    Runtime,
}

impl AiEventDomain {
    /// Return the stable machine-readable domain identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Model => "model",
            Self::Parameter => "parameter",
            Self::Data => "data",
            Self::Learning => "learning",
            Self::Training => "training",
            Self::Inference => "inference",
            Self::Reasoning => "reasoning",
            Self::Knowledge => "knowledge",
            Self::Evidence => "evidence",
            Self::Decision => "decision",
            Self::Agent => "agent",
            Self::Pipeline => "pipeline",
            Self::Differentiation => "differentiation",
            Self::Optimization => "optimization",
            Self::Deployment => "deployment",
            Self::Security => "security",
            Self::Capability => "capability",
            Self::Resources => "resources",
            Self::Integration => "integration",
            Self::Runtime => "runtime",
        }
    }

    /// Return the canonical event-name prefix for this domain.
    #[must_use]
    pub fn event_prefix(self) -> String {
        format!("ai.{}.", self.as_str())
    }
}

impl fmt::Display for AiEventDomain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// AI EVENT SUBJECT
// ============================================================================

/// Typed logical subject associated with an AI observation.
///
/// The underlying identities are owned by their respective
/// canonical AI and universal IR subsystems.
///
/// An event subject is a reference, not proof that the entity
/// exists, is authorized, or is currently executing.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AiEventSubject {
    /// AI model.
    Model(AiModelId),

    /// AI model parameter.
    Parameter(AiParameterId),

    /// AI dataset.
    Dataset(AiDatasetId),

    /// Training run.
    TrainingRun(AiTrainingRunId),

    /// AI agent.
    Agent(AiAgentId),

    /// AI pipeline.
    Pipeline(AiPipelineId),

    /// AI artifact.
    Artifact(AiArtifactId),

    /// Knowledge entity.
    Knowledge(AiKnowledgeId),

    /// Evidence entity.
    Evidence(AiEvidenceId),

    /// AI decision.
    Decision(AiDecisionId),

    /// Universal ZUIR operation participating in AI computation.
    Operation(OperationId),

    /// AI runtime as a whole.
    Runtime,

    /// Provider-defined AI entity.
    ///
    /// The namespace and identity must be nonempty and must not
    /// contain whitespace or control characters.
    Extension {
        /// Provider or subsystem namespace.
        namespace: String,

        /// Provider-defined logical identity.
        identity: String,
    },
}

impl AiEventSubject {
    /// Return the stable subject category.
    #[must_use]
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::Model(_) => "model",
            Self::Parameter(_) => "parameter",
            Self::Dataset(_) => "dataset",
            Self::TrainingRun(_) => "training_run",
            Self::Agent(_) => "agent",
            Self::Pipeline(_) => "pipeline",
            Self::Artifact(_) => "artifact",
            Self::Knowledge(_) => "knowledge",
            Self::Evidence(_) => "evidence",
            Self::Decision(_) => "decision",
            Self::Operation(_) => "operation",
            Self::Runtime => "runtime",
            Self::Extension { .. } => "extension",
        }
    }

    /// Convert the AI subject into the canonical distributed event subject.
    ///
    /// This conversion preserves logical identity and does not introduce
    /// another identity allocator.
    fn to_distributed_subject(&self) -> AiEventResult<EventSubject> {
        let (namespace, identity) = match self {
            Self::Model(id) => ("model", id.to_string()),
            Self::Parameter(id) => ("parameter", id.to_string()),
            Self::Dataset(id) => ("dataset", id.to_string()),
            Self::TrainingRun(id) => ("training_run", id.to_string()),
            Self::Agent(id) => ("agent", id.to_string()),
            Self::Pipeline(id) => ("pipeline", id.to_string()),
            Self::Artifact(id) => ("artifact", id.to_string()),
            Self::Knowledge(id) => ("knowledge", id.to_string()),
            Self::Evidence(id) => ("evidence", id.to_string()),
            Self::Decision(id) => ("decision", id.to_string()),
            Self::Operation(id) => ("operation", id.to_string()),
            Self::Runtime => ("runtime", "runtime".to_owned()),

            Self::Extension {
                namespace,
                identity,
            } => {
                validate_identity(namespace)?;
                validate_identity(identity)?;

                return Ok(EventSubject::Extension {
                    namespace: format!("zamani.ai.extension.{namespace}"),
                    identity: identity.clone(),
                });
            }
        };

        Ok(EventSubject::Extension {
            namespace: format!("zamani.ai.{namespace}"),
            identity,
        })
    }
}

// ============================================================================
// AI EVENT ERROR
// ============================================================================

/// Errors returned by AI event construction and delivery.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AiEventError {
    /// Event name does not belong to its declared AI domain.
    InvalidEventDomain {
        /// Expected event-name prefix.
        expected_prefix: String,
    },

    /// Event subject contains an invalid extension identity.
    InvalidExtensionIdentity,

    /// An attribute name is invalid.
    InvalidAttributeName,

    /// Attribute name indicates potentially sensitive information.
    SensitiveAttributeName,

    /// Canonical distributed event construction failed.
    Distributed(EventError),

    /// Event observer registry synchronization failed.
    ObserverRegistryUnavailable,

    /// One or more AI event observers failed.
    ObserverFailures {
        /// Number of observers that failed.
        failures: usize,

        /// Observer failure descriptions.
        descriptions: Vec<String>,
    },
}

impl fmt::Display for AiEventError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidEventDomain { expected_prefix } => {
                write!(
                    f,
                    "AI event name must begin with {expected_prefix}"
                )
            }

            Self::InvalidExtensionIdentity => {
                f.write_str("invalid AI extension identity")
            }

            Self::InvalidAttributeName => {
                f.write_str("invalid AI event attribute name")
            }

            Self::SensitiveAttributeName => {
                f.write_str(
                    "potentially sensitive AI event attribute name rejected"
                )
            }

            Self::Distributed(error) => {
                write!(f, "distributed observability error: {error}")
            }

            Self::ObserverRegistryUnavailable => {
                f.write_str("AI event observer registry unavailable")
            }

            Self::ObserverFailures {
                failures,
                descriptions,
            } => {
                write!(f, "{failures} AI event observer(s) failed")?;

                for description in descriptions {
                    write!(f, "; {description}")?;
                }

                Ok(())
            }
        }
    }
}

impl std::error::Error for AiEventError {}

impl From<EventError> for AiEventError {
    fn from(error: EventError) -> Self {
        Self::Distributed(error)
    }
}

// ============================================================================
// AI EVENT
// ============================================================================

/// Immutable AI computational observation.
///
/// The canonical event representation is the existing distributed
/// observability event. This wrapper adds AI-specific domain and
/// subject information without creating a competing event envelope.
///
/// Events describe observations, not execution instructions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiEvent {
    domain: AiEventDomain,
    subject: AiEventSubject,
    event: DistributedEvent,
}

impl AiEvent {
    /// Construct an AI event with an explicit timestamp.
    ///
    /// The event name must follow:
    ///
    /// `ai.<domain>.<event>`
    ///
    /// For example:
    ///
    /// - `ai.model.created`
    /// - `ai.training.started`
    /// - `ai.training.completed`
    /// - `ai.inference.completed`
    /// - `ai.reasoning.conclusion_generated`
    /// - `ai.agent.coordination_completed`
    /// - `ai.pipeline.stage_completed`
    /// - `ai.differentiation.gradient_computed`
    ///
    /// Event names are open-world.
    pub fn new(
        domain: AiEventDomain,
        name: impl Into<String>,
        timestamp: EventTimestamp,
        subject: AiEventSubject,
        outcome: EventOutcome,
    ) -> AiEventResult<Self> {
        let name = name.into();
        let expected_prefix = domain.event_prefix();

        if !name.starts_with(&expected_prefix)
            || name.len() == expected_prefix.len()
        {
            return Err(AiEventError::InvalidEventDomain {
                expected_prefix,
            });
        }

        let name = EventName::new(name)?;
        let distributed_subject = subject.to_distributed_subject()?;

        let event = DistributedEvent::new(
            EventCategory::Domain,
            name,
            timestamp,
            distributed_subject,
            outcome,
        );

        Ok(Self {
            domain,
            subject,
            event,
        })
    }

    /// Construct an AI event using the current system clock.
    ///
    /// Clock failures are returned explicitly.
    pub fn observed(
        domain: AiEventDomain,
        name: impl Into<String>,
        subject: AiEventSubject,
        outcome: EventOutcome,
    ) -> AiEventResult<Self> {
        Self::new(
            domain,
            name,
            EventTimestamp::now()?,
            subject,
            outcome,
        )
    }

    /// Attach distributed correlation and causation context.
    ///
    /// Correlation identifiers remain owned by distributed::ids.
    ///
    /// Trace propagation remains owned by distributed observability.
    #[must_use]
    pub fn with_context(mut self, context: EventContext) -> Self {
        self.event = self.event.with_context(context);
        self
    }

    /// Attach one structured operational attribute.
    ///
    /// Attribute keys must be qualified.
    ///
    /// Arbitrary execution payloads are not accepted as a separate
    /// event payload field.
    pub fn with_attribute(
        mut self,
        key: impl Into<String>,
        value: EventValue,
    ) -> AiEventResult<Self> {
        let key = key.into();

        validate_attribute_name(&key)?;

        self.event = self.event.with_attribute(key, value)?;

        Ok(self)
    }

    /// Return the AI event domain.
    #[must_use]
    pub const fn domain(&self) -> AiEventDomain {
        self.domain
    }

    /// Return the logical AI event subject.
    #[must_use]
    pub fn subject(&self) -> &AiEventSubject {
        &self.subject
    }

    /// Return the canonical distributed event representation.
    ///
    /// This allows existing distributed event sinks to consume AI events.
    #[must_use]
    pub const fn as_distributed_event(&self) -> &DistributedEvent {
        &self.event
    }

    /// Return the canonical event name.
    #[must_use]
    pub fn name(&self) -> &EventName {
        self.event.name()
    }

    /// Return the event timestamp.
    #[must_use]
    pub const fn timestamp(&self) -> EventTimestamp {
        self.event.timestamp()
    }

    /// Return the event outcome.
    #[must_use]
    pub const fn outcome(&self) -> EventOutcome {
        self.event.outcome()
    }

    /// Return the distributed event context.
    #[must_use]
    pub fn context(&self) -> &EventContext {
        self.event.context()
    }
}

// ============================================================================
// VALIDATION
// ============================================================================

fn validate_identity(value: &str) -> AiEventResult<()> {
    if value.is_empty()
        || value.trim() != value
        || value.chars().any(|character| {
            character.is_control() || character.is_whitespace()
        })
    {
        return Err(AiEventError::InvalidExtensionIdentity);
    }

    Ok(())
}

fn validate_attribute_name(value: &str) -> AiEventResult<()> {
    if value.is_empty()
        || value.trim() != value
        || !value.contains('.')
        || value.chars().any(|character| {
            character.is_control() || character.is_whitespace()
        })
    {
        return Err(AiEventError::InvalidAttributeName);
    }

    let normalized = value.to_ascii_lowercase();

    let sensitive_names = [
        "password",
        "passwd",
        "secret",
        "private_key",
        "access_token",
        "auth_token",
        "credential",
        "authorization",
        "prompt",
        "payload",
        "raw_input",
        "raw_output",
    ];

    if sensitive_names.iter().any(|sensitive| {
        normalized == *sensitive
            || normalized.ends_with(&format!(".{sensitive}"))
    }) {
        return Err(AiEventError::SensitiveAttributeName);
    }

    Ok(())
}

// ============================================================================
// AI EVENT SINK
// ============================================================================

/// Consumer of immutable AI observations.
///
/// Implementations may forward events to:
///
/// - The distributed event subsystem.
/// - An in-memory observer.
/// - A persistent event store.
/// - An operational telemetry provider.
/// - An external observability integration.
///
/// Implementations must not change AI computation semantics.
///
/// Implementations must document their delivery guarantees.
pub trait AiEventSink: Send + Sync + fmt::Debug {
    /// Publish one AI observation.
    ///
    /// An error indicates that the observer did not confirm successful
    /// acceptance. It does not establish that the AI computation failed.
    fn emit(&self, event: &AiEvent) -> AiEventResult<()>;
}

// ============================================================================
// DISTRIBUTED EVENT ADAPTER
// ============================================================================

/// Adapter connecting AI events to the canonical distributed event sink.
///
/// This adapter does not introduce another transport, journal,
/// serialization protocol, or event queue.
#[derive(Debug)]
pub struct DistributedEventSinkAdapter {
    sink: Arc<dyn DistributedEventSink>,
}

impl DistributedEventSinkAdapter {
    /// Construct an adapter around an existing distributed event sink.
    #[must_use]
    pub fn new(sink: Arc<dyn DistributedEventSink>) -> Self {
        Self { sink }
    }
}

impl AiEventSink for DistributedEventSinkAdapter {
    fn emit(&self, event: &AiEvent) -> AiEventResult<()> {
        self.sink
            .emit(event.as_distributed_event())
            .map_err(AiEventError::from)
    }
}

// ============================================================================
// COMPOSITE AI EVENT SINK
// ============================================================================

/// Optional process-local collection of AI event observers.
///
/// This component provides synchronous fan-out only.
///
/// It does not create:
///
/// - A background worker.
/// - An unbounded queue.
/// - A persistent event journal.
/// - A network connection.
/// - A process-global registry.
///
/// Observer ownership remains with the caller through Arc.
#[derive(Debug, Default)]
pub struct CompositeAiEventSink {
    observers: RwLock<Vec<Arc<dyn AiEventSink>>>,
}

impl CompositeAiEventSink {
    /// Construct an empty observer collection.
    #[must_use]
    pub fn new() -> Self {
        Self {
            observers: RwLock::new(Vec::new()),
        }
    }

    /// Register an observer.
    ///
    /// Cloned Arc ownership permits independent observer lifecycle
    /// management.
    pub fn register(
        &self,
        observer: Arc<dyn AiEventSink>,
    ) -> AiEventResult<()> {
        let mut observers = self
            .observers
            .write()
            .map_err(|_| AiEventError::ObserverRegistryUnavailable)?;

        observers.push(observer);

        Ok(())
    }

    /// Return the current number of registered observers.
    pub fn observer_count(&self) -> AiEventResult<usize> {
        let observers = self
            .observers
            .read()
            .map_err(|_| AiEventError::ObserverRegistryUnavailable)?;

        Ok(observers.len())
    }

    /// Attempt publication to every observer.
    ///
    /// The observer list is cloned before invoking callbacks.
    ///
    /// This avoids holding the registry lock while user-provided
    /// observer code executes.
    ///
    /// A failure in one observer does not prevent attempts to deliver
    /// the event to the remaining observers.
    pub fn emit_all(&self, event: &AiEvent) -> AiEventResult<()> {
        let observers = {
            let registered = self
                .observers
                .read()
                .map_err(|_| AiEventError::ObserverRegistryUnavailable)?;

            registered.clone()
        };

        let mut failures = Vec::new();

        for observer in observers {
            if let Err(error) = observer.emit(event) {
                failures.push(error.to_string());
            }
        }

        if failures.is_empty() {
            Ok(())
        } else {
            Err(AiEventError::ObserverFailures {
                failures: failures.len(),
                descriptions: failures,
            })
        }
    }
}

impl AiEventSink for CompositeAiEventSink {
    fn emit(&self, event: &AiEvent) -> AiEventResult<()> {
        self.emit_all(event)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::atomic::{
        AtomicUsize,
        Ordering,
    };

    use std::sync::Mutex;

    fn model_id() -> AiModelId {
        AiModelId::new(1).expect("valid model identity")
    }

    fn sample_event() -> AiEvent {
        AiEvent::observed(
            AiEventDomain::Training,
            "ai.training.started",
            AiEventSubject::Model(model_id()),
            EventOutcome::InProgress,
        )
        .expect("valid AI event")
    }

    #[derive(Debug, Default)]
    struct CountingSink {
        count: AtomicUsize,
    }

    impl AiEventSink for CountingSink {
        fn emit(&self, _: &AiEvent) -> AiEventResult<()> {
            self.count.fetch_add(1, Ordering::Relaxed);
            Ok(())
        }
    }

    #[derive(Debug)]
    struct FailingSink;

    impl AiEventSink for FailingSink {
        fn emit(&self, _: &AiEvent) -> AiEventResult<()> {
            Err(AiEventError::ObserverRegistryUnavailable)
        }
    }

    #[derive(Debug, Default)]
    struct DistributedRecordingSink {
        names: Mutex<Vec<String>>,
    }

    impl DistributedEventSink for DistributedRecordingSink {
        fn emit(
            &self,
            event: &DistributedEvent,
        ) -> EventResult<()> {
            self.names
                .lock()
                .map_err(|_| EventError::ObserverRegistryUnavailable)?
                .push(event.name().to_string());

            Ok(())
        }
    }

    #[test]
    fn creates_ai_training_event() {
        let event = sample_event();

        assert_eq!(event.domain(), AiEventDomain::Training);
        assert_eq!(event.name().as_str(), "ai.training.started");
        assert_eq!(event.outcome(), EventOutcome::InProgress);
    }

    #[test]
    fn event_name_must_match_declared_domain() {
        let result = AiEvent::observed(
            AiEventDomain::Training,
            "ai.inference.started",
            AiEventSubject::Model(model_id()),
            EventOutcome::InProgress,
        );

        assert!(matches!(
            result,
            Err(AiEventError::InvalidEventDomain { .. })
        ));
    }

    #[test]
    fn rejects_unqualified_event_names() {
        let result = AiEvent::observed(
            AiEventDomain::Training,
            "training_started",
            AiEventSubject::Model(model_id()),
            EventOutcome::InProgress,
        );

        assert!(result.is_err());
    }

    #[test]
    fn event_subject_preserves_model_identity() {
        let event = sample_event();

        assert_eq!(
            event.subject(),
            &AiEventSubject::Model(model_id())
        );
    }

    #[test]
    fn event_maps_to_canonical_distributed_representation() {
        let event = sample_event();
        let distributed = event.as_distributed_event();

        assert_eq!(distributed.category(), EventCategory::Domain);
        assert_eq!(distributed.name().as_str(), "ai.training.started");
        assert_eq!(
            distributed.outcome(),
            EventOutcome::InProgress
        );

        assert!(matches!(
            distributed.subject(),
            EventSubject::Extension {
                namespace,
                identity
            } if namespace == "zamani.ai.model" && identity == "1"
        ));
    }

    #[test]
    fn accepts_structured_operational_attributes() {
        let event = sample_event()
            .with_attribute(
                "ai.training.epoch",
                EventValue::Unsigned(10),
            )
            .expect("valid operational attribute");

        assert_eq!(
            event
                .as_distributed_event()
                .attributes()
                .get("ai.training.epoch"),
            Some(&EventValue::Unsigned(10))
        );
    }

    #[test]
    fn rejects_invalid_attribute_names() {
        assert!(matches!(
            sample_event().with_attribute(
                "invalid name",
                EventValue::Boolean(true)
            ),
            Err(AiEventError::InvalidAttributeName)
        ));
    }

    #[test]
    fn rejects_sensitive_attribute_names() {
        assert!(matches!(
            sample_event().with_attribute(
                "ai.training.password",
                EventValue::Text("sensitive".to_owned())
            ),
            Err(AiEventError::SensitiveAttributeName)
        ));

        assert!(matches!(
            sample_event().with_attribute(
                "ai.model.private_key",
                EventValue::Text("sensitive".to_owned())
            ),
            Err(AiEventError::SensitiveAttributeName)
        ));
    }

    #[test]
    fn accepts_valid_extension_subject() {
        let subject = AiEventSubject::Extension {
            namespace: "research.provider".to_owned(),
            identity: "model_123".to_owned(),
        };

        let event = AiEvent::observed(
            AiEventDomain::Integration,
            "ai.integration.provider_registered",
            subject,
            EventOutcome::Succeeded,
        );

        assert!(event.is_ok());
    }

    #[test]
    fn rejects_invalid_extension_subject() {
        let subject = AiEventSubject::Extension {
            namespace: "invalid namespace".to_owned(),
            identity: "model_123".to_owned(),
        };

        let event = AiEvent::observed(
            AiEventDomain::Integration,
            "ai.integration.provider_registered",
            subject,
            EventOutcome::Succeeded,
        );

        assert!(matches!(
            event,
            Err(AiEventError::InvalidExtensionIdentity)
        ));
    }

    #[test]
    fn composite_sink_registers_observers() {
        let composite = CompositeAiEventSink::new();

        composite
            .register(Arc::new(CountingSink::default()))
            .expect("observer registration");

        assert_eq!(composite.observer_count().unwrap(), 1);
    }

    #[test]
    fn composite_sink_delivers_events() {
        let composite = CompositeAiEventSink::new();
        let counter = Arc::new(CountingSink::default());

        composite
            .register(counter.clone())
            .expect("observer registration");

        composite
            .emit_all(&sample_event())
            .expect("event delivery");

        assert_eq!(counter.count.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn observer_failure_does_not_prevent_other_observers() {
        let composite = CompositeAiEventSink::new();
        let counter = Arc::new(CountingSink::default());

        composite
            .register(Arc::new(FailingSink))
            .expect("observer registration");

        composite
            .register(counter.clone())
            .expect("observer registration");

        let result = composite.emit_all(&sample_event());

        assert!(matches!(
            result,
            Err(AiEventError::ObserverFailures {
                failures: 1,
                ..
            })
        ));

        assert_eq!(counter.count.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn distributed_adapter_reuses_existing_event_sink() {
        let distributed = Arc::new(DistributedRecordingSink::default());

        let adapter = DistributedEventSinkAdapter::new(
            distributed.clone()
        );

        adapter
            .emit(&sample_event())
            .expect("distributed event delivery");

        let names = distributed.names.lock().unwrap();

        assert_eq!(
            names.as_slice(),
            ["ai.training.started"]
        );
    }

    #[test]
    fn timestamps_are_preserved() {
        let timestamp = EventTimestamp::new(100, 500).unwrap();

        let event = AiEvent::new(
            AiEventDomain::Inference,
            "ai.inference.completed",
            timestamp,
            AiEventSubject::Model(model_id()),
            EventOutcome::Succeeded,
        )
        .unwrap();

        assert_eq!(event.timestamp(), timestamp);
    }

    #[test]
    fn event_metadata_is_deterministic() {
        let first = sample_event()
            .with_attribute(
                "ai.training.z",
                EventValue::Unsigned(2),
            )
            .unwrap()
            .with_attribute(
                "ai.training.a",
                EventValue::Unsigned(1),
            )
            .unwrap();

        let attributes = first
            .as_distributed_event()
            .attributes();

        let keys: Vec<_> = attributes.keys().map(String::as_str).collect();

        assert_eq!(
            keys,
            ["ai.training.a", "ai.training.z"]
        );
    }

    #[test]
    fn empty_composite_sink_is_valid() {
        let composite = CompositeAiEventSink::new();

        assert_eq!(composite.observer_count().unwrap(), 0);
        assert!(composite.emit_all(&sample_event()).is_ok());
    }
}
