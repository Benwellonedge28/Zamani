#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]


```rust
//! AI-specific tracing contracts for Zamani.
//!
//! This module describes AI execution traces without implementing a second
//! distributed tracing engine.
//!
//! # Architecture
//!
//! AI tracing is an adapter over the canonical distributed observability
//! infrastructure. It describes AI execution intent and metadata while
//! delegating transport, export, propagation, and distributed coordination
//! to the corresponding infrastructure.
//!
//! # Ownership
//!
//! This module owns:
//! - AI-specific span kinds and attributes.
//! - AI-specific trace context.
//! - AI span lifecycle descriptions.
//! - Provider-independent tracing interfaces.
//! - Privacy-aware AI metadata.
//!
//! This module does not own:
//! - Distributed transport.
//! - Distributed scheduling or placement.
//! - General-purpose trace storage.
//! - General-purpose trace exporters.
//! - Metrics or profiling.
//! - Model execution.
//! - Authentication or authorization.
//!
//! # Portability
//!
//! The API contains no hardware-specific assumptions and imposes no
//! semantic ceilings on model sizes, workers, devices, or trace counts.
//!
//! Operational limits, buffering policies, and sampling policies belong
//! to explicitly configured providers.
//!
//! # Safety
//!
//! This module uses safe Rust only.

use std::borrow::Cow;
use std::fmt;
use std::sync::Arc;

/// Trace identifier.
///
/// The representation is opaque to AI computation. A provider or the
/// canonical distributed tracing implementation is responsible for
/// generating globally suitable identifiers.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TraceId(Cow<'static, str>);

impl TraceId {
    /// Construct a trace identifier.
    ///
    /// The caller must supply an identifier generated according to the
    /// canonical tracing provider's identifier contract.
    pub fn new(value: impl Into<Cow<'static, str>>) -> Result<Self, TraceError> {
        let value = value.into();

        if value.is_empty() {
            return Err(TraceError::InvalidIdentifier(
                "trace identifier must not be empty",
            ));
        }

        Ok(Self(value))
    }

    /// Return the identifier representation.
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

/// Span identifier.
///
/// Span identifiers are opaque references. AI tracing does not allocate
/// distributed span identifiers independently.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SpanId(Cow<'static, str>);

impl SpanId {
    /// Construct a span identifier.
    pub fn new(value: impl Into<Cow<'static, str>>) -> Result<Self, TraceError> {
        let value = value.into();

        if value.is_empty() {
            return Err(TraceError::InvalidIdentifier(
                "span identifier must not be empty",
            ));
        }

        Ok(Self(value))
    }

    /// Return the identifier representation.
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

/// AI tracing context.
///
/// This context is a portable correlation reference. It does not determine
/// execution placement, scheduling, or resource allocation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AiTraceContext {
    trace_id: TraceId,
    span_id: SpanId,
    parent_span_id: Option<SpanId>,
}

impl AiTraceContext {
    /// Create a context from identifiers supplied by the tracing provider.
    pub fn new(
        trace_id: TraceId,
        span_id: SpanId,
        parent_span_id: Option<SpanId>,
    ) -> Self {
        Self {
            trace_id,
            span_id,
            parent_span_id,
        }
    }

    /// Return the trace identifier.
    pub fn trace_id(&self) -> &TraceId {
        &self.trace_id
    }

    /// Return the span identifier.
    pub fn span_id(&self) -> &SpanId {
        &self.span_id
    }

    /// Return the parent span identifier, if present.
    pub fn parent_span_id(&self) -> Option<&SpanId> {
        self.parent_span_id.as_ref()
    }
}

/// AI computation category represented by a span.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum AiSpanKind {
    /// Model construction or model composition.
    Model,

    /// Model inference.
    Inference,

    /// Training computation.
    Training,

    /// Reasoning computation.
    Reasoning,

    /// Agent execution.
    Agent,

    /// Agent planning.
    Planning,

    /// Tool invocation by an AI agent.
    ToolInvocation,

    /// Dataset processing.
    DataProcessing,

    /// Differentiation or gradient computation.
    Differentiation,

    /// AI pipeline execution.
    Pipeline,

    /// Model optimization.
    Optimization,

    /// AI-specific distributed coordination.
    Distributed,

    /// A provider-defined extension.
    Extension(Cow<'static, str>),
}

/// AI operation identity.
///
/// This is descriptive metadata. It does not identify a physical device
/// or prescribe a particular backend implementation.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct AiOperationName(Cow<'static, str>);

impl AiOperationName {
    /// Construct an operation name.
    pub fn new(value: impl Into<Cow<'static, str>>) -> Result<Self, TraceError> {
        let value = value.into();

        if value.is_empty() {
            return Err(TraceError::InvalidOperationName);
        }

        Ok(Self(value))
    }

    /// Return the operation name.
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

/// AI span execution status.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AiSpanStatus {
    /// Execution status has not yet been finalized.
    Unset,

    /// The operation completed successfully.
    Ok,

    /// The operation failed.
    Error {
        /// Stable, provider-independent error classification.
        code: Cow<'static, str>,
    },

    /// Execution was cancelled.
    Cancelled,

    /// Execution could not proceed because a required capability was
    /// unavailable.
    Unsupported,

    /// Execution could not proceed because the requested realization
    /// was infeasible.
    Infeasible,
}

/// Privacy classification for AI tracing metadata.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AiAttributePrivacy {
    /// Public, non-sensitive operational metadata.
    Public,

    /// Metadata that may identify a user, dataset, or workload.
    Sensitive,

    /// Confidential material, including prompts, outputs, credentials,
    /// model parameters, and private dataset content.
    Confidential,
}

/// Attribute value supported by the AI tracing API.
///
/// Arbitrary serialized objects are intentionally excluded. This avoids
/// accidental capture of model payloads and credentials.
#[derive(Clone, Debug, PartialEq)]
pub enum AiAttributeValue {
    String(Cow<'static, str>),
    Boolean(bool),
    Signed(i64),
    Unsigned(u64),
    Float(f64),
}

/// AI-specific span attribute.
///
/// The privacy classification must be considered by the provider before
/// exporting the attribute.
#[derive(Clone, Debug, PartialEq)]
pub struct AiAttribute {
    name: Cow<'static, str>,
    value: AiAttributeValue,
    privacy: AiAttributePrivacy,
}

impl AiAttribute {
    /// Construct an attribute.
    pub fn new(
        name: impl Into<Cow<'static, str>>,
        value: AiAttributeValue,
        privacy: AiAttributePrivacy,
    ) -> Result<Self, TraceError> {
        let name = name.into();

        if name.is_empty() {
            return Err(TraceError::InvalidAttributeName);
        }

        if let AiAttributeValue::Float(value) = value {
            if !value.is_finite() {
                return Err(TraceError::InvalidAttributeValue);
            }
        }

        Ok(Self {
            name,
            value,
            privacy,
        })
    }

    /// Return the attribute name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Return the attribute value.
    pub fn value(&self) -> &AiAttributeValue {
        &self.value
    }

    /// Return the privacy classification.
    pub fn privacy(&self) -> AiAttributePrivacy {
        self.privacy
    }
}

/// AI span description.
///
/// This is a provider-independent description. The tracing provider
/// decides how the span is represented and exported.
#[derive(Clone, Debug, PartialEq)]
pub struct AiSpanDescription {
    operation: AiOperationName,
    kind: AiSpanKind,
    attributes: Vec<AiAttribute>,
}

impl AiSpanDescription {
    /// Create a span description.
    pub fn new(operation: AiOperationName, kind: AiSpanKind) -> Self {
        Self {
            operation,
            kind,
            attributes: Vec::new(),
        }
    }

    /// Add an attribute.
    ///
    /// Attribute collection size is governed by the provider's operational
    /// policy rather than an AI language-level ceiling.
    pub fn with_attribute(mut self, attribute: AiAttribute) -> Self {
        self.attributes.push(attribute);
        self
    }

    /// Return the operation name.
    pub fn operation(&self) -> &AiOperationName {
        &self.operation
    }

    /// Return the span kind.
    pub fn kind(&self) -> &AiSpanKind {
        &self.kind
    }

    /// Return the attributes.
    pub fn attributes(&self) -> &[AiAttribute] {
        &self.attributes
    }
}

/// Sampling decision.
///
/// Sampling is an observability policy. It must never affect the
/// correctness or semantics of AI computation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AiSamplingDecision {
    /// Record and export the span.
    Record,

    /// Do not record the span.
    Drop,

    /// Defer the decision to the provider.
    Defer,
}

/// Provider-independent AI tracing error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TraceError {
    InvalidIdentifier(&'static str),
    InvalidOperationName,
    InvalidAttributeName,
    InvalidAttributeValue,
    ProviderFailure(Cow<'static, str>),
    ContextPropagationFailure(Cow<'static, str>),
    InvalidSpanLifecycle,
}

impl fmt::Display for TraceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier(message) => {
                write!(formatter, "invalid tracing identifier: {message}")
            }
            Self::InvalidOperationName => {
                formatter.write_str("AI tracing operation name must not be empty")
            }
            Self::InvalidAttributeName => {
                formatter.write_str("AI tracing attribute name must not be empty")
            }
            Self::InvalidAttributeValue => {
                formatter.write_str("AI tracing attribute value is invalid")
            }
            Self::ProviderFailure(message) => {
                write!(formatter, "AI tracing provider failure: {message}")
            }
            Self::ContextPropagationFailure(message) => {
                write!(formatter, "AI tracing context propagation failure: {message}")
            }
            Self::InvalidSpanLifecycle => {
                formatter.write_str("invalid AI tracing span lifecycle")
            }
        }
    }
}

impl std::error::Error for TraceError {}

/// A started AI span.
///
/// The handle is provider-independent. The provider owns the actual span
/// lifecycle and any exporter state.
pub trait AiSpan: Send + Sync {
    /// Return the span context.
    fn context(&self) -> &AiTraceContext;

    /// Record a structured event on this span.
    ///
    /// Implementations must apply their configured privacy policy.
    fn record_event(
        &self,
        name: &str,
        attributes: &[AiAttribute],
    ) -> Result<(), TraceError>;

    /// Finalize the span.
    ///
    /// Implementations should make finalization idempotent where possible.
    fn finish(&self, status: AiSpanStatus) -> Result<(), TraceError>;
}

/// AI tracing provider.
///
/// This interface is implemented by an adapter to the canonical
/// distributed observability provider.
///
/// The provider is responsible for:
/// - Distributed identifier allocation.
/// - Span lifecycle management.
/// - Sampling implementation.
/// - Context propagation.
/// - Export and buffering.
/// - Operational resource budgets.
///
/// No AI execution operation may depend on tracing being enabled.
pub trait AiTracingProvider: Send + Sync {
    /// Decide whether a span should be recorded.
    fn sampling_decision(
        &self,
        parent: Option<&AiTraceContext>,
        description: &AiSpanDescription,
    ) -> AiSamplingDecision;

    /// Start an AI span.
    fn start_span(
        &self,
        parent: Option<&AiTraceContext>,
        description: AiSpanDescription,
    ) -> Result<Option<Arc<dyn AiSpan>>, TraceError>;
}

/// Tracing facade used by AI domain components.
#[derive(Clone)]
pub struct AiTracer {
    provider: Option<Arc<dyn AiTracingProvider>>,
}

impl AiTracer {
    /// Construct a tracer with an injected provider.
    pub fn with_provider(provider: Arc<dyn AiTracingProvider>) -> Self {
        Self {
            provider: Some(provider),
        }
    }

    /// Construct a disabled tracer.
    ///
    /// Disabled tracing is a valid configuration and does not affect
    /// AI execution semantics.
    pub fn disabled() -> Self {
        Self { provider: None }
    }

    /// Start an AI span.
    ///
    /// Returns `Ok(None)` when tracing is disabled or the provider elects
    /// not to record the span.
    pub fn start_span(
        &self,
        parent: Option<&AiTraceContext>,
        description: AiSpanDescription,
    ) -> Result<Option<Arc<dyn AiSpan>>, TraceError> {
        let Some(provider) = &self.provider else {
            return Ok(None);
        };

        match provider.sampling_decision(parent, &description) {
            AiSamplingDecision::Drop => Ok(None),
            AiSamplingDecision::Record | AiSamplingDecision::Defer => {
                provider.start_span(parent, description)
            }
        }
    }

    /// Return whether a provider is configured.
    pub fn is_enabled(&self) -> bool {
        self.provider.is_some()
    }
}

impl Default for AiTracer {
    fn default() -> Self {
        Self::disabled()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_trace_identifier() {
        assert!(TraceId::new(String::new()).is_err());
    }

    #[test]
    fn rejects_empty_span_identifier() {
        assert!(SpanId::new(String::new()).is_err());
    }

    #[test]
    fn rejects_empty_operation_name() {
        assert!(AiOperationName::new(String::new()).is_err());
    }

    #[test]
    fn rejects_empty_attribute_name() {
        assert!(
            AiAttribute::new(
                String::new(),
                AiAttributeValue::Boolean(true),
                AiAttributePrivacy::Public,
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_non_finite_attribute_values() {
        assert!(
            AiAttribute::new(
                "ai.numeric.value",
                AiAttributeValue::Float(f64::NAN),
                AiAttributePrivacy::Public,
            )
            .is_err()
        );
    }

    #[test]
    fn disabled_tracer_does_not_start_spans() {
        let tracer = AiTracer::disabled();

        let description = AiSpanDescription::new(
            AiOperationName::new("ai.inference").unwrap(),
            AiSpanKind::Inference,
        );

        assert!(!tracer.is_enabled());
        assert!(tracer.start_span(None, description).unwrap().is_none());
    }

    #[test]
    fn context_preserves_parent_relationship() {
        let trace_id = TraceId::new("trace-1").unwrap();
        let parent_id = SpanId::new("span-parent").unwrap();
        let child_id = SpanId::new("span-child").unwrap();

        let context = AiTraceContext::new(
            trace_id,
            child_id,
            Some(parent_id.clone()),
        );

        assert_eq!(
            context.parent_span_id(),
            Some(&parent_id),
        );
    }

    #[test]
    fn span_description_preserves_attributes() {
        let attribute = AiAttribute::new(
            "ai.operation.name",
            AiAttributeValue::String(Cow::Borrowed("inference")),
            AiAttributePrivacy::Public,
        )
        .unwrap();

        let description = AiSpanDescription::new(
            AiOperationName::new("ai.inference").unwrap(),
            AiSpanKind::Inference,
        )
        .with_attribute(attribute);

        assert_eq!(description.attributes().len(), 1);
    }
}
```
