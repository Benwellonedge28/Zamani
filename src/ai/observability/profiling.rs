
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani AI Profiling
//!
//! Path: src/ai/observability/profiling.rs
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provides AI-specific profiling contracts and portable measurements.
//!
//! This module observes AI computation. It does not execute, schedule,
//! allocate, optimize, or select hardware for AI workloads.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - AI profiling categories.
//! - AI profiling scope descriptions.
//! - AI profiling records.
//! - AI profiling sink interfaces.
//! - Profiling configuration.
//! - Profiling sampling decisions.
//! - Local monotonic duration measurement.
//!
//! This module does not own:
//!
//! - Distributed trace identifiers.
//! - Distributed trace propagation.
//! - General-purpose metrics.
//! - General-purpose profiling infrastructure.
//! - Hardware counters.
//! - Device discovery.
//! - Model execution.
//! - Resource allocation.
//! - Execution scheduling.
//! - Distributed placement.
//! - AI semantic analysis.
//! - AST or IR representations.
//!
//! # Architectural integration
//!
//! AI tracing:
//!     crate::ai::observability::tracing
//!
//! AI events:
//!     crate::ai::observability::events
//!
//! AI metrics:
//!     crate::ai::observability::metrics
//!
//! Distributed tracing:
//!     crate::distributed::observability::tracing
//!
//! AI execution:
//!     crate::ai::execution
//!
//! # Scalability
//!
//! No universal model count, profiling scope count, device count,
//! tensor rank, worker count, or workload-size ceiling is imposed.
//!
//! Operational budgets are caller-supplied.
//!
//! Memory and throughput remain bounded by available resources.
//!
//! # Measurement semantics
//!
//! Durations are measured using a local monotonic clock.
//!
//! Durations from different machines must not be treated as synchronized
//! timestamps.
//!
//! A profiling record describes an observation. It is not an execution
//! instruction or proof of successful execution.
//!
//! # Privacy
//!
//! Profiling records contain operational metadata only.
//!
//! They must not contain credentials, private prompts, model input
//! payloads, model output payloads, private parameters, or secrets.

use core::fmt;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

// ============================================================================
// RESULT
// ============================================================================

/// Result of an AI profiling operation.
pub type AiProfilingResult<T> = Result<T, AiProfilingError>;

// ============================================================================
// ERROR
// ============================================================================

/// Machine-readable profiling error category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AiProfilingErrorKind {
    /// The profiling scope name is invalid.
    InvalidScopeName,

    /// A profiling attribute is invalid.
    InvalidAttribute,

    /// A profiling attribute may contain sensitive information.
    SensitiveAttribute,

    /// The supplied profiling budget was exceeded.
    BudgetExceeded,

    /// A profiling scope was already completed.
    ScopeAlreadyCompleted,

    /// The configured profiling sink failed.
    SinkFailure,

    /// A profiling sink returned an invalid result.
    InvalidSinkResult,

    /// A profiling scope could not be created.
    ScopeCreationFailure,
}

impl AiProfilingErrorKind {
    /// Return the stable machine-readable error code.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidScopeName => "ai.profiling.invalid_scope_name",
            Self::InvalidAttribute => "ai.profiling.invalid_attribute",
            Self::SensitiveAttribute => "ai.profiling.sensitive_attribute",
            Self::BudgetExceeded => "ai.profiling.budget_exceeded",
            Self::ScopeAlreadyCompleted => {
                "ai.profiling.scope_already_completed"
            }
            Self::SinkFailure => "ai.profiling.sink_failure",
            Self::InvalidSinkResult => "ai.profiling.invalid_sink_result",
            Self::ScopeCreationFailure => {
                "ai.profiling.scope_creation_failure"
            }
        }
    }
}

/// Error returned by AI profiling operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiProfilingError {
    kind: AiProfilingErrorKind,
    code: Option<String>,
}

impl AiProfilingError {
    /// Construct a profiling error.
    #[must_use]
    pub const fn new(kind: AiProfilingErrorKind) -> Self {
        Self {
            kind,
            code: None,
        }
    }

    /// Construct a profiling error with a stable provider error code.
    pub fn with_code(
        kind: AiProfilingErrorKind,
        code: impl Into<String>,
    ) -> AiProfilingResult<Self> {
        let code = code.into();

        if !valid_identifier(&code) {
            return Err(Self::new(
                AiProfilingErrorKind::InvalidAttribute,
            ));
        }

        Ok(Self {
            kind,
            code: Some(code),
        })
    }

    /// Return the error category.
    #[must_use]
    pub const fn kind(&self) -> AiProfilingErrorKind {
        self.kind
    }

    /// Return the optional stable error code.
    #[must_use]
    pub fn code(&self) -> Option<&str> {
        self.code.as_deref()
    }
}

impl fmt::Display for AiProfilingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.kind.as_str())?;

        if let Some(code) = &self.code {
            write!(formatter, ": {code}")?;
        }

        Ok(())
    }
}

impl std::error::Error for AiProfilingError {}

// ============================================================================
// PROFILING CATEGORY
// ============================================================================

/// Broad classification of an AI profiling scope.
///
/// The category identifies the computational domain being observed.
/// Individual scope names remain open-world.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AiProfilingCategory {
    /// Model construction or composition.
    Model,

    /// Model inference.
    Inference,

    /// Training computation.
    Training,

    /// Learning computation.
    Learning,

    /// Symbolic or computational reasoning.
    Reasoning,

    /// Knowledge processing.
    Knowledge,

    /// Evidence processing.
    Evidence,

    /// Agent execution.
    Agent,

    /// Agent planning.
    Planning,

    /// Agent tool invocation.
    ToolInvocation,

    /// Dataset processing.
    DataProcessing,

    /// Tensor computation.
    Tensor,

    /// Automatic differentiation.
    Differentiation,

    /// AI pipeline execution.
    Pipeline,

    /// AI graph optimization.
    Optimization,

    /// Model deployment.
    Deployment,

    /// AI-specific interoperability.
    Interoperability,

    /// Provider-defined AI profiling category.
    Extension,
}

impl AiProfilingCategory {
    /// Return the stable category identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Model => "model",
            Self::Inference => "inference",
            Self::Training => "training",
            Self::Learning => "learning",
            Self::Reasoning => "reasoning",
            Self::Knowledge => "knowledge",
            Self::Evidence => "evidence",
            Self::Agent => "agent",
            Self::Planning => "planning",
            Self::ToolInvocation => "tool_invocation",
            Self::DataProcessing => "data_processing",
            Self::Tensor => "tensor",
            Self::Differentiation => "differentiation",
            Self::Pipeline => "pipeline",
            Self::Optimization => "optimization",
            Self::Deployment => "deployment",
            Self::Interoperability => "interoperability",
            Self::Extension => "extension",
        }
    }
}

// ============================================================================
// IDENTIFIERS
// ============================================================================

/// Opaque profiling scope identity.
///
/// Identity generation belongs to the caller or canonical observability
/// infrastructure. This type does not introduce another identity allocator.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct ProfilingScopeId(String);

impl ProfilingScopeId {
    /// Construct a profiling scope identity.
    pub fn new(value: impl Into<String>) -> AiProfilingResult<Self> {
        let value = value.into();

        if !valid_identifier(&value) {
            return Err(AiProfilingError::new(
                AiProfilingErrorKind::InvalidAttribute,
            ));
        }

        Ok(Self(value))
    }

    /// Return the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Optional parent scope identity.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProfilingParent(ProfilingScopeId);

impl ProfilingParent {
    /// Construct a parent reference.
    #[must_use]
    pub fn new(id: ProfilingScopeId) -> Self {
        Self(id)
    }

    /// Return the parent scope identity.
    #[must_use]
    pub fn id(&self) -> &ProfilingScopeId {
        &self.0
    }
}

// ============================================================================
// ATTRIBUTES
// ============================================================================

/// Safe profiling attribute value.
///
/// Attribute values are operational metadata, not model payloads.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum ProfilingValue {
    /// Boolean observation.
    Boolean(bool),

    /// Signed integer observation.
    Signed(i128),

    /// Unsigned integer observation.
    Unsigned(u128),

    /// Finite floating-point observation.
    Float(f64),

    /// Public descriptive text.
    Text(String),

    /// Stable machine-readable code.
    Code(String),
}

impl ProfilingValue {
    /// Validate the attribute value.
    pub fn validate(&self) -> AiProfilingResult<()> {
        match self {
            Self::Float(value) if !value.is_finite() => {
                Err(AiProfilingError::new(
                    AiProfilingErrorKind::InvalidAttribute,
                ))
            }
            Self::Text(value) if value.chars().any(char::is_control) => {
                Err(AiProfilingError::new(
                    AiProfilingErrorKind::InvalidAttribute,
                ))
            }
            Self::Code(value) if !valid_identifier(value) => {
                Err(AiProfilingError::new(
                    AiProfilingErrorKind::InvalidAttribute,
                ))
            }
            _ => Ok(()),
        }
    }
}

/// Deterministically ordered profiling attributes.
pub type ProfilingAttributes = BTreeMap<String, ProfilingValue>;

// ============================================================================
// CONFIGURATION
// ============================================================================

/// Optional operational budget for profiling.
///
/// This is an instrumentation budget, not a semantic AI workload limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AiProfilingBudget {
    /// Optional maximum number of emitted records.
    max_records: Option<usize>,
}

impl AiProfilingBudget {
    /// Create a budget without an explicit record limit.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_records: None,
        }
    }

    /// Create a budget with an explicit record limit.
    #[must_use]
    pub const fn with_max_records(maximum: usize) -> Self {
        Self {
            max_records: Some(maximum),
        }
    }

    /// Return the configured record limit.
    #[must_use]
    pub const fn max_records(self) -> Option<usize> {
        self.max_records
    }
}

/// Profiling sampling policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ProfilingSampling {
    /// Profile every scope.
    Always,

    /// Do not profile scopes.
    Never,

    /// Profile according to an externally determined decision.
    External,
}

/// AI profiling configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AiProfilingConfig {
    enabled: bool,
    sampling: ProfilingSampling,
    budget: AiProfilingBudget,
}

impl Default for AiProfilingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            sampling: ProfilingSampling::Never,
            budget: AiProfilingBudget::unlimited(),
        }
    }
}

impl AiProfilingConfig {
    /// Construct a profiling configuration.
    #[must_use]
    pub const fn new(
        enabled: bool,
        sampling: ProfilingSampling,
        budget: AiProfilingBudget,
    ) -> Self {
        Self {
            enabled,
            sampling,
            budget,
        }
    }

    /// Return whether profiling is enabled.
    #[must_use]
    pub const fn enabled(self) -> bool {
        self.enabled
    }

    /// Return the sampling policy.
    #[must_use]
    pub const fn sampling(self) -> ProfilingSampling {
        self.sampling
    }

    /// Return the operational budget.
    #[must_use]
    pub const fn budget(self) -> AiProfilingBudget {
        self.budget
    }
}

// ============================================================================
// PROFILE RECORD
// ============================================================================

/// Result classification for a profiling scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ProfilingOutcome {
    /// Scope completed successfully.
    Success,

    /// Scope completed with a failure.
    Failure,

    /// Scope was cancelled.
    Cancelled,

    /// Scope outcome was not established.
    Unknown,
}

impl ProfilingOutcome {
    /// Return the stable outcome identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::Failure => "failure",
            Self::Cancelled => "cancelled",
            Self::Unknown => "unknown",
        }
    }
}

/// Immutable AI profiling record.
#[derive(Debug, Clone, PartialEq)]
pub struct AiProfilingRecord {
    id: ProfilingScopeId,
    parent: Option<ProfilingParent>,
    category: AiProfilingCategory,
    name: String,
    duration: Duration,
    outcome: ProfilingOutcome,
    attributes: ProfilingAttributes,
}

impl AiProfilingRecord {
    /// Construct a completed profiling record.
    pub fn new(
        id: ProfilingScopeId,
        parent: Option<ProfilingParent>,
        category: AiProfilingCategory,
        name: impl Into<String>,
        duration: Duration,
        outcome: ProfilingOutcome,
        attributes: ProfilingAttributes,
    ) -> AiProfilingResult<Self> {
        let name = name.into();

        validate_scope_name(&name)?;

        for (key, value) in &attributes {
            validate_attribute_name(key)?;
            value.validate()?;
        }

        Ok(Self {
            id,
            parent,
            category,
            name,
            duration,
            outcome,
            attributes,
        })
    }

    /// Return the scope identity.
    #[must_use]
    pub fn id(&self) -> &ProfilingScopeId {
        &self.id
    }

    /// Return the parent scope.
    #[must_use]
    pub fn parent(&self) -> Option<&ProfilingParent> {
        self.parent.as_ref()
    }

    /// Return the category.
    #[must_use]
    pub const fn category(&self) -> AiProfilingCategory {
        self.category
    }

    /// Return the scope name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the measured duration.
    #[must_use]
    pub const fn duration(&self) -> Duration {
        self.duration
    }

    /// Return the outcome.
    #[must_use]
    pub const fn outcome(&self) -> ProfilingOutcome {
        self.outcome
    }

    /// Return the profiling attributes.
    #[must_use]
    pub fn attributes(&self) -> &ProfilingAttributes {
        &self.attributes
    }
}

// ============================================================================
// PROFILING SINK
// ============================================================================

/// Receives completed AI profiling records.
///
/// Implementations may forward records to the canonical tracing,
/// metrics, or external observability infrastructure.
///
/// Sink implementations must document their buffering, delivery,
/// retention, and failure semantics.
pub trait AiProfilingSink: Send + Sync {
    /// Submit a completed profiling record.
    fn record(
        &self,
        record: AiProfilingRecord,
    ) -> AiProfilingResult<()>;
}

// ============================================================================
// PROFILER
// ============================================================================

/// AI profiling provider.
///
/// The provider owns the configured sink and profiling policy.
/// It does not own the AI computation being observed.
#[derive(Clone)]
pub struct AiProfiler {
    config: AiProfilingConfig,
    sink: Option<Arc<dyn AiProfilingSink>>,
}

impl fmt::Debug for AiProfiler {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AiProfiler")
            .field("config", &self.config)
            .field("has_sink", &self.sink.is_some())
            .finish()
    }
}

impl AiProfiler {
    /// Construct an AI profiler.
    #[must_use]
    pub fn new(
        config: AiProfilingConfig,
        sink: Option<Arc<dyn AiProfilingSink>>,
    ) -> Self {
        Self { config, sink }
    }

    /// Return the profiler configuration.
    #[must_use]
    pub const fn config(&self) -> AiProfilingConfig {
        self.config
    }

    /// Return whether profiling is active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.config.enabled
            && !matches!(
                self.config.sampling,
                ProfilingSampling::Never
            )
            && self.sink.is_some()
    }

    /// Start a profiling scope.
    ///
    /// The caller supplies the scope identity. This avoids introducing
    /// a second distributed identity allocator.
    pub fn start(
        &self,
        id: ProfilingScopeId,
        parent: Option<ProfilingParent>,
        category: AiProfilingCategory,
        name: impl Into<String>,
    ) -> AiProfilingResult<AiProfilingScope> {
        let name = name.into();

        validate_scope_name(&name)?;

        if !self.is_active() {
            return Ok(AiProfilingScope::disabled(
                id,
                parent,
                category,
                name,
            ));
        }

        Ok(AiProfilingScope {
            id,
            parent,
            category,
            name,
            started: Some(Instant::now()),
            attributes: ProfilingAttributes::new(),
            outcome: ProfilingOutcome::Unknown,
            profiler: Some(self.clone()),
            completed: false,
        })
    }

    /// Emit a completed profiling record.
    ///
    /// This operation is explicit. It does not alter the computation
    /// represented by the record.
    pub fn emit(
        &self,
        record: AiProfilingRecord,
    ) -> AiProfilingResult<()> {
        if !self.config.enabled {
            return Ok(());
        }

        if let Some(sink) = &self.sink {
            sink.record(record)?;
        }

        Ok(())
    }
}

// ============================================================================
// PROFILING SCOPE
// ============================================================================

/// A locally measured AI profiling scope.
///
/// A scope measures elapsed time using a monotonic clock.
///
/// Dropping a scope does not emit a record. Callers must explicitly
/// complete the scope so that error and cancellation outcomes remain
/// observable and deterministic.
pub struct AiProfilingScope {
    id: ProfilingScopeId,
    parent: Option<ProfilingParent>,
    category: AiProfilingCategory,
    name: String,
    started: Option<Instant>,
    attributes: ProfilingAttributes,
    outcome: ProfilingOutcome,
    profiler: Option<AiProfiler>,
    completed: bool,
}

impl fmt::Debug for AiProfilingScope {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AiProfilingScope")
            .field("id", &self.id)
            .field("category", &self.category)
            .field("name", &self.name)
            .field("completed", &self.completed)
            .finish()
    }
}

impl AiProfilingScope {
    fn disabled(
        id: ProfilingScopeId,
        parent: Option<ProfilingParent>,
        category: AiProfilingCategory,
        name: String,
    ) -> Self {
        Self {
            id,
            parent,
            category,
            name,
            started: None,
            attributes: ProfilingAttributes::new(),
            outcome: ProfilingOutcome::Unknown,
            profiler: None,
            completed: true,
        }
    }

    /// Attach a profiling attribute.
    pub fn with_attribute(
        &mut self,
        key: impl Into<String>,
        value: ProfilingValue,
    ) -> AiProfilingResult<()> {
        if self.completed {
            return Err(AiProfilingError::new(
                AiProfilingErrorKind::ScopeAlreadyCompleted,
            ));
        }

        let key = key.into();

        validate_attribute_name(&key)?;
        value.validate()?;

        self.attributes.insert(key, value);

        Ok(())
    }

    /// Set the scope outcome.
    pub fn set_outcome(
        &mut self,
        outcome: ProfilingOutcome,
    ) -> AiProfilingResult<()> {
        if self.completed {
            return Err(AiProfilingError::new(
                AiProfilingErrorKind::ScopeAlreadyCompleted,
            ));
        }

        self.outcome = outcome;

        Ok(())
    }

    /// Return the scope identity.
    #[must_use]
    pub fn id(&self) -> &ProfilingScopeId {
        &self.id
    }

    /// Return the scope name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Complete the scope and emit its profiling record.
    ///
    /// If profiling is disabled, no record is emitted.
    pub fn finish(mut self) -> AiProfilingResult<Option<AiProfilingRecord>> {
        if self.completed {
            return Ok(None);
        }

        let duration = self
            .started
            .take()
            .map(|started| started.elapsed())
            .unwrap_or_default();

        self.completed = true;

        let record = AiProfilingRecord::new(
            self.id.clone(),
            self.parent.clone(),
            self.category,
            self.name.clone(),
            duration,
            self.outcome,
            self.attributes.clone(),
        )?;

        if let Some(profiler) = &self.profiler {
            profiler.emit(record.clone())?;
        }

        Ok(Some(record))
    }
}

// ============================================================================
// VALIDATION
// ============================================================================

fn valid_identifier(value: &str) -> bool {
    !value.trim().is_empty()
        && !value.chars().any(char::is_control)
}

fn validate_scope_name(value: &str) -> AiProfilingResult<()> {
    if !valid_identifier(value) {
        return Err(AiProfilingError::new(
            AiProfilingErrorKind::InvalidScopeName,
        ));
    }

    Ok(())
}

fn validate_attribute_name(value: &str) -> AiProfilingResult<()> {
    if !valid_identifier(value)
        || !value.contains('.')
        || value.chars().any(char::is_whitespace)
    {
        return Err(AiProfilingError::new(
            AiProfilingErrorKind::InvalidAttribute,
        ));
    }

    let normalized = value.to_ascii_lowercase();

    let sensitive = [
        "password",
        "passwd",
        "secret",
        "token",
        "credential",
        "private_key",
        "prompt",
        "model_input",
        "model_output",
        "checkpoint_data",
    ];

    if sensitive.iter().any(|word| normalized.contains(word)) {
        return Err(AiProfilingError::new(
            AiProfilingErrorKind::SensitiveAttribute,
        ));
    }

    Ok(())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    #[derive(Debug, Default)]
    struct TestSink {
        records: Mutex<Vec<AiProfilingRecord>>,
    }

    impl AiProfilingSink for TestSink {
        fn record(
            &self,
            record: AiProfilingRecord,
        ) -> AiProfilingResult<()> {
            self.records
                .lock()
                .map_err(|_| {
                    AiProfilingError::new(
                        AiProfilingErrorKind::SinkFailure,
                    )
                })?
                .push(record);

            Ok(())
        }
    }

    fn test_id(value: &str) -> ProfilingScopeId {
        ProfilingScopeId::new(value).expect("valid test identity")
    }

    #[test]
    fn identifiers_reject_empty_values() {
        assert!(ProfilingScopeId::new("").is_err());
        assert!(ProfilingScopeId::new("scope-1").is_ok());
    }

    #[test]
    fn profiling_record_preserves_duration() {
        let duration = Duration::from_nanos(123);

        let record = AiProfilingRecord::new(
            test_id("scope-1"),
            None,
            AiProfilingCategory::Inference,
            "model.inference",
            duration,
            ProfilingOutcome::Success,
            ProfilingAttributes::new(),
        )
        .expect("record should be valid");

        assert_eq!(record.duration(), duration);
        assert_eq!(record.name(), "model.inference");
    }

    #[test]
    fn sensitive_attribute_names_are_rejected() {
        assert!(
            validate_attribute_name("ai.prompt.content").is_err()
        );

        assert!(
            validate_attribute_name("ai.execution.duration").is_ok()
        );
    }

    #[test]
    fn non_finite_values_are_rejected() {
        assert!(
            ProfilingValue::Float(f64::NAN)
                .validate()
                .is_err()
        );

        assert!(
            ProfilingValue::Float(1.5)
                .validate()
                .is_ok()
        );
    }

    #[test]
    fn profiler_emits_completed_scope() {
        let sink = Arc::new(TestSink::default());

        let profiler = AiProfiler::new(
            AiProfilingConfig::new(
                true,
                ProfilingSampling::Always,
                AiProfilingBudget::unlimited(),
            ),
            Some(sink.clone()),
        );

        let scope = profiler
            .start(
                test_id("scope-2"),
                None,
                AiProfilingCategory::Training,
                "training.step",
            )
            .expect("scope should start");

        let record = scope
            .finish()
            .expect("scope should finish")
            .expect("record should exist");

        assert_eq!(
            record.category(),
            AiProfilingCategory::Training
        );

        assert_eq!(
            sink.records
                .lock()
                .expect("sink lock")
                .len(),
            1
        );
    }

    #[test]
    fn disabled_profiler_does_not_emit() {
        let profiler = AiProfiler::new(
            AiProfilingConfig::default(),
            None,
        );

        assert!(!profiler.is_active());

        let scope = profiler
            .start(
                test_id("scope-3"),
                None,
                AiProfilingCategory::Inference,
                "inference.step",
            )
            .expect("disabled scope should start");

        assert!(
            scope
                .finish()
                .expect("disabled scope should finish")
                .is_none()
        );
    }
}
