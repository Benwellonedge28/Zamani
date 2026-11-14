
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Observability Profiling
//!
//! Path: src/zur/observability/profiling.rs
//! Edition: Rust 2021
//! Minimum Rust: 1.97
//!
//! # Purpose
//!
//! Provides domain-neutral runtime profiling infrastructure.
//!
//! Profiling records operational observations about program execution,
//! including elapsed time, named measurements, execution outcomes,
//! and structured operational attributes.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - Runtime profiling scopes.
//! - Runtime profiling records.
//! - Profiling measurements.
//! - Profiling attributes.
//! - Profiling outcomes.
//! - Configurable sampling.
//! - Profiling sink contracts.
//! - Profiling budgets.
//! - Profiling session configuration.
//!
//! This module does not own:
//!
//! - Compiler profiling.
//! - JIT compilation-unit profiling.
//! - JIT hotness analysis.
//! - Compiler optimization.
//! - Hardware performance counter discovery.
//! - Hardware allocation.
//! - Runtime scheduling.
//! - Execution lifecycle.
//! - Distributed tracing.
//! - Generic metrics.
//! - Logging.
//! - Persistence.
//! - Telemetry transport.
//! - Source-language semantics.
//! - Computational IR.
//!
//! Those responsibilities remain with their existing authorities.
//!
//! # Integration
//!
//! Runtime operation identities:
//!     crate::zur::ids::RuntimeOperationId
//!
//! Generic runtime metrics:
//!     crate::zur::observability::metrics
//!
//! Runtime logging:
//!     crate::zur::observability::logging
//!
//! Distributed events:
//!     crate::distributed::observability::events
//!
//! Distributed tracing:
//!     crate::distributed::observability::tracing
//!
//! JIT profiling:
//!     crate::zur::jit::profiling
//!
//! Compiler profiling:
//!     crate::compiler::profiling
//!
//! AI profiling:
//!     crate::ai::observability::profiling
//!
//! # Scalability
//!
//! No universal maximum is imposed on:
//!
//! - Profiling scopes.
//! - Execution operations.
//! - Computational domains.
//! - Measurement names.
//! - Profiling sessions.
//! - Providers.
//! - Devices.
//! - Workers.
//! - Machines.
//!
//! Optional budgets are explicit operational constraints.
//!
//! This module does not retain an unbounded history of records.
//! Completed records are transferred to an injected sink.
//!
//! # Safety
//!
//! - No unsafe Rust.
//! - No raw pointers.
//! - No process-global mutable state.
//! - No hidden worker threads.
//! - No fabricated measurements.
//! - No implicit hardware discovery.
//! - No implicit network connections.
//! - No arbitrary hardware ceilings.
//!
//! # Observational non-interference
//!
//! Profiling must not change the functional meaning of a computation.
//!
//! A profiling error is not automatically an execution error.
//!
//! The caller determines how profiling failures are handled.
//!
//! Dropping an unfinished scope does not fabricate a successful
//! profiling record.

use std::collections::BTreeMap;
use std::fmt;
use std::num::{NonZeroU64, NonZeroUsize};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::zur::ids::RuntimeOperationId;

// ============================================================================
// API VERSION
// ============================================================================

/// Public API version of runtime profiling.
pub const RUNTIME_PROFILING_API_VERSION: u16 = 1;

// ============================================================================
// RESULTS
// ============================================================================

/// Result of a runtime profiling operation.
pub type ProfilingResult<T> = Result<T, ProfilingError>;

// ============================================================================
// ERRORS
// ============================================================================

/// Machine-readable profiling error categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ProfilingErrorKind {
    /// An invalid profiling identifier was supplied.
    InvalidIdentity,

    /// A profiling name is invalid.
    InvalidName,

    /// A profiling attribute is invalid.
    InvalidAttribute,

    /// An attribute may expose sensitive information.
    SensitiveAttribute,

    /// A measurement contains an invalid value.
    InvalidMeasurement,

    /// A configured profiling budget was exceeded.
    BudgetExceeded,

    /// A profiling counter exceeded its representable range.
    CounterOverflow,

    /// Profiling synchronization failed.
    SynchronizationFailure,

    /// A profiling sink failed.
    SinkFailure,

    /// A profiling scope was used incorrectly.
    InvalidScope,
}

impl ProfilingErrorKind {
    /// Return the stable machine-readable error identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidIdentity => "profiling.invalid_identity",
            Self::InvalidName => "profiling.invalid_name",
            Self::InvalidAttribute => "profiling.invalid_attribute",
            Self::SensitiveAttribute => "profiling.sensitive_attribute",
            Self::InvalidMeasurement => "profiling.invalid_measurement",
            Self::BudgetExceeded => "profiling.budget_exceeded",
            Self::CounterOverflow => "profiling.counter_overflow",
            Self::SynchronizationFailure => "profiling.synchronization_failure",
            Self::SinkFailure => "profiling.sink_failure",
            Self::InvalidScope => "profiling.invalid_scope",
        }
    }
}

/// Error returned by runtime profiling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfilingError {
    kind: ProfilingErrorKind,
    code: Option<String>,
}

impl ProfilingError {
    /// Construct a profiling error.
    #[must_use]
    pub const fn new(kind: ProfilingErrorKind) -> Self {
        Self {
            kind,
            code: None,
        }
    }

    /// Construct an error with a stable provider error code.
    pub fn with_code(
        kind: ProfilingErrorKind,
        code: impl Into<String>,
    ) -> ProfilingResult<Self> {
        let code = code.into();

        if !valid_identifier(&code) {
            return Err(Self::new(
                ProfilingErrorKind::InvalidName,
            ));
        }

        Ok(Self {
            kind,
            code: Some(code),
        })
    }

    /// Return the error category.
    #[must_use]
    pub const fn kind(&self) -> ProfilingErrorKind {
        self.kind
    }

    /// Return the optional provider error code.
    #[must_use]
    pub fn code(&self) -> Option<&str> {
        self.code.as_deref()
    }
}

impl fmt::Display for ProfilingError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.kind.as_str())?;

        if let Some(code) = &self.code {
            write!(formatter, ": {code}")?;
        }

        Ok(())
    }
}

impl std::error::Error for ProfilingError {}

// ============================================================================
// IDENTIFIERS
// ============================================================================

/// Validates an extensible profiling identifier.
///
/// Identifiers may use namespaced forms such as:
///
/// - `classical.execution`
/// - `quantum.execution`
/// - `hardware.synthesis`
/// - `ai.inference`
/// - `distributed.communication`
/// - `org.example.future_domain.execution`
///
/// The runtime does not maintain a closed list of computational domains.
fn valid_identifier(value: &str) -> bool {
    !value.is_empty()
        && value.trim() == value
        && !value.chars().any(char::is_control)
}

/// Validates a profiling name.
fn validate_name(value: &str) -> ProfilingResult<()> {
    if !valid_identifier(value) {
        return Err(ProfilingError::new(
            ProfilingErrorKind::InvalidName,
        ));
    }

    Ok(())
}

/// Validates an attribute name and rejects obviously sensitive names.
fn validate_attribute_name(
    value: &str,
) -> ProfilingResult<()> {
    validate_name(value)?;

    let normalized = value.to_ascii_lowercase();

    let sensitive_fragments = [
        "password",
        "passwd",
        "secret",
        "credential",
        "private_key",
        "access_token",
        "auth_token",
        "checkpoint_data",
        "private_state",
        "model_input",
        "model_output",
        "prompt_content",
    ];

    if sensitive_fragments
        .iter()
        .any(|fragment| normalized.contains(fragment))
    {
        return Err(ProfilingError::new(
            ProfilingErrorKind::SensitiveAttribute,
        ));
    }

    Ok(())
}

// ============================================================================
// PROFILING OUTCOMES
// ============================================================================

/// Actual outcome of an observed profiling scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ProfilingOutcome {
    /// The observed operation completed successfully.
    Success,

    /// The observed operation failed.
    Failure,

    /// The observed operation was cancelled.
    Cancelled,

    /// The operation's outcome was not established.
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

// ============================================================================
// PROFILING VALUES
// ============================================================================

/// Typed profiling measurement values.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum ProfilingValue {
    /// Boolean observation.
    Boolean(bool),

    /// Signed integer measurement.
    Signed(i128),

    /// Unsigned integer measurement.
    Unsigned(u128),

    /// Finite floating-point measurement.
    Float(f64),

    /// Duration measurement.
    Duration(Duration),

    /// Public descriptive text.
    Text(String),

    /// Stable machine-readable code.
    Code(String),
}

impl ProfilingValue {
    /// Validate the measurement value.
    pub fn validate(&self) -> ProfilingResult<()> {
        match self {
            Self::Float(value) if !value.is_finite() => {
                Err(ProfilingError::new(
                    ProfilingErrorKind::InvalidMeasurement,
                ))
            }

            Self::Text(value) => {
                if value.chars().any(char::is_control) {
                    return Err(ProfilingError::new(
                        ProfilingErrorKind::InvalidMeasurement,
                    ));
                }

                Ok(())
            }

            Self::Code(value) => validate_name(value),

            _ => Ok(()),
        }
    }
}

/// Deterministically ordered profiling attributes.
pub type ProfilingAttributes = BTreeMap<String, ProfilingValue>;

/// Deterministically ordered profiling measurements.
pub type ProfilingMeasurements = BTreeMap<String, ProfilingValue>;

// ============================================================================
// SAMPLING
// ============================================================================

/// Profiling sampling policy.
///
/// Sampling changes observation collection only.
/// It never changes execution acceptance or program semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum ProfilingSampling {
    /// Record every eligible profiling scope.
    #[default]
    Always,

    /// Do not record profiling scopes.
    Never,

    /// Record every Nth eligible profiling scope.
    ///
    /// The first recorded scope is the Nth scope.
    EveryNth(NonZeroU64),
}

impl ProfilingSampling {
    /// Determine whether an invocation should be sampled.
    ///
    /// `ordinal` is the one-based observation number.
    #[must_use]
    pub const fn should_sample(
        self,
        ordinal: u128,
    ) -> bool {
        match self {
            Self::Always => true,
            Self::Never => false,
            Self::EveryNth(interval) => {
                ordinal % interval.get() as u128 == 0
            }
        }
    }
}

// ============================================================================
// PROFILING BUDGET
// ============================================================================

/// Optional profiling memory and metadata budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ProfilingBudget {
    max_fields_per_scope: Option<NonZeroUsize>,
}

impl ProfilingBudget {
    /// Create a budget without an explicit field-count limit.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_fields_per_scope: None,
        }
    }

    /// Create a budget with an explicit maximum number of fields.
    #[must_use]
    pub const fn with_max_fields_per_scope(
        maximum: NonZeroUsize,
    ) -> Self {
        Self {
            max_fields_per_scope: Some(maximum),
        }
    }

    /// Return the configured field-count budget.
    #[must_use]
    pub const fn max_fields_per_scope(
        self,
    ) -> Option<NonZeroUsize> {
        self.max_fields_per_scope
    }
}

// ============================================================================
// CONFIGURATION
// ============================================================================

/// Configuration for a runtime profiling provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProfilingConfig {
    enabled: bool,
    sampling: ProfilingSampling,
    budget: ProfilingBudget,
}

impl Default for ProfilingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            sampling: ProfilingSampling::Never,
            budget: ProfilingBudget::unlimited(),
        }
    }
}

impl ProfilingConfig {
    /// Construct a profiling configuration.
    #[must_use]
    pub const fn new(
        enabled: bool,
        sampling: ProfilingSampling,
        budget: ProfilingBudget,
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

    /// Return the profiling budget.
    #[must_use]
    pub const fn budget(self) -> ProfilingBudget {
        self.budget
    }
}

// ============================================================================
// PROFILING RECORD
// ============================================================================

/// Immutable completed profiling record.
///
/// Runtime operation identities are reused rather than creating
/// another runtime or distributed identity allocator.
#[derive(Debug, Clone, PartialEq)]
pub struct ProfilingRecord {
    operation_id: RuntimeOperationId,
    parent_operation_id: Option<RuntimeOperationId>,
    domain: String,
    name: String,
    duration: Duration,
    outcome: ProfilingOutcome,
    attributes: ProfilingAttributes,
    measurements: ProfilingMeasurements,
}

impl ProfilingRecord {
    /// Construct a completed profiling record.
    pub fn new(
        operation_id: RuntimeOperationId,
        parent_operation_id: Option<RuntimeOperationId>,
        domain: impl Into<String>,
        name: impl Into<String>,
        duration: Duration,
        outcome: ProfilingOutcome,
        attributes: ProfilingAttributes,
        measurements: ProfilingMeasurements,
    ) -> ProfilingResult<Self> {
        let domain = domain.into();
        let name = name.into();

        validate_name(&domain)?;
        validate_name(&name)?;

        if parent_operation_id.as_ref() == Some(&operation_id) {
            return Err(ProfilingError::new(
                ProfilingErrorKind::InvalidIdentity,
            ));
        }

        for (key, value) in &attributes {
            validate_attribute_name(key)?;
            value.validate()?;
        }

        for (key, value) in &measurements {
            validate_name(key)?;
            value.validate()?;
        }

        Ok(Self {
            operation_id,
            parent_operation_id,
            domain,
            name,
            duration,
            outcome,
            attributes,
            measurements,
        })
    }

    /// Return the canonical runtime operation identity.
    #[must_use]
    pub fn operation_id(&self) -> &RuntimeOperationId {
        &self.operation_id
    }

    /// Return the parent operation identity.
    #[must_use]
    pub fn parent_operation_id(
        &self,
    ) -> Option<&RuntimeOperationId> {
        self.parent_operation_id.as_ref()
    }

    /// Return the computational-domain identity.
    #[must_use]
    pub fn domain(&self) -> &str {
        &self.domain
    }

    /// Return the profiling scope name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the locally measured elapsed duration.
    #[must_use]
    pub const fn duration(&self) -> Duration {
        self.duration
    }

    /// Return elapsed duration in nanoseconds.
    #[must_use]
    pub fn duration_nanoseconds(&self) -> u128 {
        self.duration.as_nanos()
    }

    /// Return the actual observed outcome.
    #[must_use]
    pub const fn outcome(&self) -> ProfilingOutcome {
        self.outcome
    }

    /// Return structured attributes.
    #[must_use]
    pub fn attributes(&self) -> &ProfilingAttributes {
        &self.attributes
    }

    /// Return structured measurements.
    #[must_use]
    pub fn measurements(&self) -> &ProfilingMeasurements {
        &self.measurements
    }
}

// ============================================================================
// SINK CONTRACT
// ============================================================================

/// Stable error returned by a profiling sink.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfilingSinkError {
    code: String,
}

impl ProfilingSinkError {
    /// Construct a profiling sink error.
    pub fn new(
        code: impl Into<String>,
    ) -> ProfilingResult<Self> {
        let code = code.into();

        if !valid_identifier(&code) {
            return Err(ProfilingError::new(
                ProfilingErrorKind::InvalidName,
            ));
        }

        Ok(Self { code })
    }

    /// Return the stable provider error code.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }
}

/// Result returned by a profiling sink.
pub type ProfilingSinkResult = Result<(), ProfilingSinkError>;

/// Destination for completed profiling records.
///
/// Implementations may forward records to:
///
/// - Local aggregation.
/// - Existing runtime metrics.
/// - Existing logging infrastructure.
/// - Distributed observability.
/// - Persistent telemetry storage.
/// - External profiling providers.
///
/// Sink implementations own their buffering, delivery, backpressure,
/// retention, and shutdown policies.
///
/// A sink must not silently claim durable delivery.
pub trait ProfilingSink: Send + Sync + fmt::Debug {
    /// Submit a completed profiling record.
    fn record(
        &self,
        record: ProfilingRecord,
    ) -> ProfilingSinkResult;
}

// ============================================================================
// SHARED PROFILER STATE
// ============================================================================

#[derive(Debug, Default)]
struct ProfilerState {
    observations_started: u128,
}

/// Shared internal profiler implementation.
struct ProfilerInner {
    config: ProfilingConfig,
    sink: Arc<dyn ProfilingSink>,
    state: Mutex<ProfilerState>,
}

impl fmt::Debug for ProfilerInner {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("ProfilerInner")
            .field("config", &self.config)
            .finish_non_exhaustive()
    }
}

// ============================================================================
// PROFILER
// ============================================================================

/// Cloneable runtime profiling provider.
///
/// Clones share the same explicitly owned profiling state and sink.
///
/// The profiler does not create global state, worker threads,
/// storage databases, or network connections.
#[derive(Clone)]
pub struct Profiler {
    inner: Option<Arc<ProfilerInner>>,
}

impl fmt::Debug for Profiler {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("Profiler")
            .field("active", &self.is_active())
            .finish()
    }
}

impl Default for Profiler {
    fn default() -> Self {
        Self::disabled()
    }
}

impl Profiler {
    /// Construct a disabled profiler.
    #[must_use]
    pub const fn disabled() -> Self {
        Self { inner: None }
    }

    /// Construct an explicitly configured profiler.
    ///
    /// The supplied sink is shared by all profiler clones.
    #[must_use]
    pub fn new(
        config: ProfilingConfig,
        sink: Arc<dyn ProfilingSink>,
    ) -> Self {
        if !config.enabled
            || matches!(config.sampling, ProfilingSampling::Never)
        {
            return Self::disabled();
        }

        Self {
            inner: Some(Arc::new(ProfilerInner {
                config,
                sink,
                state: Mutex::new(ProfilerState::default()),
            })),
        }
    }

    /// Return whether profiling is active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.inner.is_some()
    }

    /// Return the active configuration, if profiling is enabled.
    #[must_use]
    pub fn config(&self) -> Option<ProfilingConfig> {
        self.inner.as_ref().map(|inner| inner.config)
    }

    /// Start a runtime profiling scope.
    ///
    /// The operation identity is supplied by the existing ZUR
    /// identity authority.
    ///
    /// `domain` is extensible and must identify the actual domain
    /// associated with the observed operation.
    ///
    /// `parent` preserves logical parent-child relationships.
    ///
    /// Sampling applies only to observation collection.
    /// It does not prevent the underlying computation from running.
    pub fn start(
        &self,
        operation_id: RuntimeOperationId,
        parent: Option<RuntimeOperationId>,
        domain: impl Into<String>,
        name: impl Into<String>,
    ) -> ProfilingResult<ProfilingScope> {
        let domain = domain.into();
        let name = name.into();

        validate_name(&domain)?;
        validate_name(&name)?;

        if parent.as_ref() == Some(&operation_id) {
            return Err(ProfilingError::new(
                ProfilingErrorKind::InvalidIdentity,
            ));
        }

        let Some(inner) = &self.inner else {
            return Ok(ProfilingScope::disabled(
                operation_id,
                parent,
                domain,
                name,
            ));
        };

        let ordinal = {
            let mut state = inner
                .state
                .lock()
                .map_err(|_| {
                    ProfilingError::new(
                        ProfilingErrorKind::SynchronizationFailure,
                    )
                })?;

            state.observations_started = state
                .observations_started
                .checked_add(1)
                .ok_or_else(|| {
                    ProfilingError::new(
                        ProfilingErrorKind::CounterOverflow,
                    )
                })?;

            state.observations_started
        };

        if !inner.config.sampling.should_sample(ordinal) {
            return Ok(ProfilingScope::disabled(
                operation_id,
                parent,
                domain,
                name,
            ));
        }

        Ok(ProfilingScope {
            operation_id,
            parent_operation_id: parent,
            domain,
            name,
            started_at: Some(Instant::now()),
            attributes: ProfilingAttributes::new(),
            measurements: ProfilingMeasurements::new(),
            profiler: Some(self.clone()),
            completed: false,
        })
    }

    /// Submit an already completed profiling record.
    ///
    /// This method does not execute or alter the observed operation.
    pub fn emit(
        &self,
        record: ProfilingRecord,
    ) -> ProfilingResult<()> {
        let Some(inner) = &self.inner else {
            return Ok(());
        };

        inner.sink.record(record).map_err(|error| {
            ProfilingError {
                kind: ProfilingErrorKind::SinkFailure,
                code: Some(error.code),
            }
        })
    }
}

// ============================================================================
// PROFILING SCOPE
// ============================================================================

/// Active or disabled profiling scope.
///
/// Scope duration is measured with a local monotonic clock.
///
/// An unfinished scope does not automatically emit a successful record.
///
/// Call `finish` explicitly to establish the actual outcome.
pub struct ProfilingScope {
    operation_id: RuntimeOperationId,
    parent_operation_id: Option<RuntimeOperationId>,
    domain: String,
    name: String,
    started_at: Option<Instant>,
    attributes: ProfilingAttributes,
    measurements: ProfilingMeasurements,
    profiler: Option<Profiler>,
    completed: bool,
}

impl fmt::Debug for ProfilingScope {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("ProfilingScope")
            .field("operation_id", &self.operation_id)
            .field("domain", &self.domain)
            .field("name", &self.name)
            .field("recording", &self.started_at.is_some())
            .field("completed", &self.completed)
            .finish()
    }
}

impl ProfilingScope {
    fn disabled(
        operation_id: RuntimeOperationId,
        parent_operation_id: Option<RuntimeOperationId>,
        domain: String,
        name: String,
    ) -> Self {
        Self {
            operation_id,
            parent_operation_id,
            domain,
            name,
            started_at: None,
            attributes: ProfilingAttributes::new(),
            measurements: ProfilingMeasurements::new(),
            profiler: None,
            completed: true,
        }
    }

    /// Return whether this scope is actually recording.
    #[must_use]
    pub fn is_recording(&self) -> bool {
        self.started_at.is_some() && !self.completed
    }

    /// Return the observed operation identity.
    #[must_use]
    pub fn operation_id(&self) -> &RuntimeOperationId {
        &self.operation_id
    }

    /// Return the scope name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Attach an operational profiling attribute.
    ///
    /// Sensitive attribute names are rejected.
    pub fn with_attribute(
        &mut self,
        key: impl Into<String>,
        value: ProfilingValue,
    ) -> ProfilingResult<()> {
        if self.completed {
            return Ok(());
        }

        let key = key.into();

        validate_attribute_name(&key)?;
        value.validate()?;

        self.check_field_budget(
            self.attributes.contains_key(&key),
        )?;

        self.attributes.insert(key, value);

        Ok(())
    }

    /// Attach a named measurement.
    ///
    /// Measurement names are open-ended rather than tied to a
    /// closed list of computational domains or hardware devices.
    pub fn with_measurement(
        &mut self,
        name: impl Into<String>,
        value: ProfilingValue,
    ) -> ProfilingResult<()> {
        if self.completed {
            return Ok(());
        }

        let name = name.into();

        validate_name(&name)?;
        value.validate()?;

        self.check_field_budget(
            self.measurements.contains_key(&name),
        )?;

        self.measurements.insert(name, value);

        Ok(())
    }

    fn check_field_budget(
        &self,
        replacing_existing: bool,
    ) -> ProfilingResult<()> {
        let Some(profiler) = &self.profiler else {
            return Ok(());
        };

        let Some(config) = profiler.config() else {
            return Ok(());
        };

        let Some(maximum) = config.budget.max_fields_per_scope else {
            return Ok(());
        };

        let current = self.attributes.len()
            .checked_add(self.measurements.len())
            .ok_or_else(|| {
                ProfilingError::new(
                    ProfilingErrorKind::CounterOverflow,
                )
            })?;

        let next = if replacing_existing {
            current
        } else {
            current.checked_add(1).ok_or_else(|| {
                ProfilingError::new(
                    ProfilingErrorKind::CounterOverflow,
                )
            })?
        };

        if next > maximum.get() {
            return Err(ProfilingError::new(
                ProfilingErrorKind::BudgetExceeded,
            ));
        }

        Ok(())
    }

    /// Complete the scope and emit its profiling record.
    ///
    /// The caller must supply the actual observed outcome.
    ///
    /// If profiling is disabled or the scope was not sampled,
    /// no record is emitted.
    ///
    /// Sink failures are returned separately from the computation
    /// being observed.
    pub fn finish(
        mut self,
        outcome: ProfilingOutcome,
    ) -> ProfilingResult<Option<ProfilingRecord>> {
        if self.completed {
            return Ok(None);
        }

        let started_at = self.started_at.take().ok_or_else(|| {
            ProfilingError::new(
                ProfilingErrorKind::InvalidScope,
            )
        })?;

        let duration = started_at.elapsed();

        self.completed = true;

        let record = ProfilingRecord::new(
            self.operation_id.clone(),
            self.parent_operation_id.clone(),
            self.domain.clone(),
            self.name.clone(),
            duration,
            outcome,
            self.attributes.clone(),
            self.measurements.clone(),
        )?;

        if let Some(profiler) = &self.profiler {
            profiler.emit(record.clone())?;
        }

        Ok(Some(record))
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Mutex;

    use crate::zur::ids::{
        IdAllocator,
        IdNamespace,
    };

    #[derive(Debug, Default)]
    struct TestSink {
        records: Mutex<Vec<ProfilingRecord>>,
    }

    impl ProfilingSink for TestSink {
        fn record(
            &self,
            record: ProfilingRecord,
        ) -> ProfilingSinkResult {
            self.records
                .lock()
                .map_err(|_| {
                    ProfilingSinkError {
                        code: "test.sink_lock_failed".to_owned(),
                    }
                })?
                .push(record);

            Ok(())
        }
    }

    fn test_id() -> RuntimeOperationId {
        let namespace = IdNamespace::new(
            "zamani.runtime.operation.test",
        )
        .expect("valid namespace");

        IdAllocator::<RuntimeOperationId>::new(namespace)
            .try_next()
            .expect("valid operation identity")
    }

    fn test_profiler(
        sink: Arc<TestSink>,
    ) -> Profiler {
        let config = ProfilingConfig::new(
            true,
            ProfilingSampling::Always,
            ProfilingBudget::unlimited(),
        );

        Profiler::new(config, sink)
    }

    #[test]
    fn disabled_profiler_does_not_emit_records() {
        let profiler = Profiler::disabled();

        assert!(!profiler.is_active());

        let scope = profiler
            .start(
                test_id(),
                None,
                "classical.execution",
                "program.run",
            )
            .expect("scope creation");

        assert!(!scope.is_recording());

        assert!(
            scope.finish(ProfilingOutcome::Success)
                .expect("disabled profiling")
                .is_none()
        );
    }

    #[test]
    fn completed_scope_emits_actual_outcome() {
        let sink = Arc::new(TestSink::default());

        let profiler = test_profiler(sink.clone());

        let record = profiler
            .start(
                test_id(),
                None,
                "classical.execution",
                "program.run",
            )
            .expect("scope creation")
            .finish(ProfilingOutcome::Success)
            .expect("scope completion")
            .expect("record exists");

        assert_eq!(
            record.outcome(),
            ProfilingOutcome::Success
        );

        assert_eq!(
            record.domain(),
            "classical.execution"
        );

        assert_eq!(
            sink.records.lock().expect("sink lock").len(),
            1
        );
    }

    #[test]
    fn failed_execution_is_not_reported_as_success() {
        let sink = Arc::new(TestSink::default());

        let profiler = test_profiler(sink.clone());

        let record = profiler
            .start(
                test_id(),
                None,
                "quantum.execution",
                "quantum.submit",
            )
            .expect("scope creation")
            .finish(ProfilingOutcome::Failure)
            .expect("profiling completion")
            .expect("record exists");

        assert_eq!(
            record.outcome(),
            ProfilingOutcome::Failure
        );
    }

    #[test]
    fn cancellation_is_preserved() {
        let sink = Arc::new(TestSink::default());

        let profiler = test_profiler(sink.clone());

        let record = profiler
            .start(
                test_id(),
                None,
                "distributed.execution",
                "distributed.task",
            )
            .expect("scope creation")
            .finish(ProfilingOutcome::Cancelled)
            .expect("profiling completion")
            .expect("record exists");

        assert_eq!(
            record.outcome(),
            ProfilingOutcome::Cancelled
        );
    }

    #[test]
    fn measurements_are_preserved() {
        let sink = Arc::new(TestSink::default());

        let profiler = test_profiler(sink);

        let mut scope = profiler
            .start(
                test_id(),
                None,
                "ai.inference",
                "model.execute",
            )
            .expect("scope creation");

        scope
            .with_measurement(
                "ai.tensor_operations",
                ProfilingValue::Unsigned(100),
            )
            .expect("measurement");

        let record = scope
            .finish(ProfilingOutcome::Success)
            .expect("completion")
            .expect("record");

        assert_eq!(
            record.measurements().get("ai.tensor_operations"),
            Some(&ProfilingValue::Unsigned(100))
        );
    }

    #[test]
    fn attributes_are_preserved() {
        let sink = Arc::new(TestSink::default());

        let profiler = test_profiler(sink);

        let mut scope = profiler
            .start(
                test_id(),
                None,
                "hardware.execution",
                "device.operation",
            )
            .expect("scope creation");

        scope
            .with_attribute(
                "hardware.provider",
                ProfilingValue::Text("example".to_owned()),
            )
            .expect("attribute");

        let record = scope
            .finish(ProfilingOutcome::Success)
            .expect("completion")
            .expect("record");

        assert!(
            record.attributes().contains_key("hardware.provider")
        );
    }

    #[test]
    fn sensitive_attributes_are_rejected() {
        assert!(
            validate_attribute_name("runtime.execution.duration")
                .is_ok()
        );

        assert!(
            validate_attribute_name("runtime.password.value")
                .is_err()
        );

        assert!(
            validate_attribute_name("runtime.access_token")
                .is_err()
        );
    }

    #[test]
    fn non_finite_measurements_are_rejected() {
        assert!(
            ProfilingValue::Float(f64::NAN)
                .validate()
                .is_err()
        );

        assert!(
            ProfilingValue::Float(f64::INFINITY)
                .validate()
                .is_err()
        );

        assert!(
            ProfilingValue::Float(1.25)
                .validate()
                .is_ok()
        );
    }

    #[test]
    fn sampling_policy_is_deterministic() {
        let policy = ProfilingSampling::EveryNth(
            NonZeroU64::new(3).expect("nonzero"),
        );

        assert!(!policy.should_sample(1));
        assert!(!policy.should_sample(2));
        assert!(policy.should_sample(3));
        assert!(!policy.should_sample(4));
        assert!(policy.should_sample(6));
    }

    #[test]
    fn field_budget_is_enforced() {
        let sink = Arc::new(TestSink::default());

        let config = ProfilingConfig::new(
            true,
            ProfilingSampling::Always,
            ProfilingBudget::with_max_fields_per_scope(
                NonZeroUsize::new(1).expect("nonzero"),
            ),
        );

        let profiler = Profiler::new(config, sink);

        let mut scope = profiler
            .start(
                test_id(),
                None,
                "classical.execution",
                "program.run",
            )
            .expect("scope creation");

        scope
            .with_attribute(
                "runtime.execution.kind",
                ProfilingValue::Text("native".to_owned()),
            )
            .expect("first field");

        assert!(
            scope
                .with_measurement(
                    "runtime.operations",
                    ProfilingValue::Unsigned(10),
                )
                .is_err()
        );
    }

    #[test]
    fn arbitrary_domains_are_supported() {
        let sink = Arc::new(TestSink::default());

        let profiler = test_profiler(sink);

        let record = profiler
            .start(
                test_id(),
                None,
                "org.example.future_domain",
                "future.execute",
            )
            .expect("future domain")
            .finish(ProfilingOutcome::Success)
            .expect("completion")
            .expect("record");

        assert_eq!(
            record.domain(),
            "org.example.future_domain"
        );
    }

    #[test]
    fn parent_operation_identity_is_preserved() {
        let sink = Arc::new(TestSink::default());

        let profiler = test_profiler(sink);

        let parent = test_id();

        let child = profiler
            .start(
                test_id(),
                Some(parent.clone()),
                "classical.execution",
                "child.execute",
            )
            .expect("child scope")
            .finish(ProfilingOutcome::Success)
            .expect("completion")
            .expect("record");

        assert_eq!(
            child.parent_operation_id(),
            Some(&parent)
        );
    }

    #[test]
    fn duration_uses_monotonic_elapsed_time() {
        let sink = Arc::new(TestSink::default());

        let profiler = test_profiler(sink);

        let record = profiler
            .start(
                test_id(),
                None,
                "classical.execution",
                "program.run",
            )
            .expect("scope")
            .finish(ProfilingOutcome::Success)
            .expect("completion")
            .expect("record");

        assert!(
            record.duration() <= Duration::from_secs(60)
        );
    }
}
