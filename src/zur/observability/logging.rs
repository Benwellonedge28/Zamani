
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Structured Logging
//!
//! Path: src/zur/observability/logging.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//!
//! # Purpose
//!
//! Provides structured, concurrent, policy-aware logging for the
//! Zamani Universal Runtime (ZUR).
//!
//! Logging is an observational runtime service. It must not change
//! program semantics, execution results, resource feasibility,
//! compiler decisions or physical execution.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - LogEvent.
//! - LogField.
//! - LogLevel.
//! - LogClass.
//! - LogConfig.
//! - LogDisposition.
//! - LogSink.
//! - Logger.
//! - JsonLineSink.
//! - Sensitive-value redaction.
//! - Logging policy enforcement.
//!
//! This module does NOT own:
//!
//! - Runtime identity definitions.
//! - Execution identity definitions.
//! - Runtime operation identity definitions.
//! - Runtime error definitions.
//! - Runtime configuration.
//! - Runtime policy definitions.
//! - Trace and span identities.
//! - Metrics.
//! - Profiling.
//! - Distributed event semantics.
//! - Source-language observability semantics.
//! - Security authorization.
//! - Audit persistence.
//! - Network transport.
//! - Hardware discovery.
//! - Provider selection.
//! - Execution scheduling.
//! - Program output.
//!
//! # Canonical authorities
//!
//! Runtime identities:
//!     crate::zur::ids
//!
//! Runtime errors:
//!     crate::zur::error
//!
//! Runtime policies:
//!     crate::zur::policy
//!
//! Runtime context:
//!     crate::zur::core::context
//!
//! Operation context:
//!     crate::stdlib::context
//!
//! Distributed observability:
//!     crate::distributed::observability
//!
//! Backend telemetry:
//!     crate::backend::telemetry
//!
//! # Architecture
//!
//! Runtime components
//!       |
//!       v
//!     Logger
//!       |
//!       v
//!  Policy evaluation
//!       |
//!       v
//!    LogEvent
//!       |
//!       v
//!    LogSink
//!       |
//!       +----> JSON Lines sink
//!       |
//!       +----> In-memory sink
//!       |
//!       +----> Persistent sink
//!       |
//!       +----> External logging adapter
//!
//! Sink implementations are explicitly injected.
//!
//! No global logger is installed.
//!
//! # Security
//!
//! Sensitive messages and fields are redacted before serialization.
//!
//! Debug formatting also respects sensitivity classification.
//!
//! Log sinks receive events through an interface that exposes
//! redacted message and field values.
//!
//! Logging does not establish authorization to export information.
//! External sinks remain responsible for destination authorization,
//! encryption, retention and access control.
//!
//! # Scalability
//!
//! There are no fixed limits on:
//!
//! - Runtime instances.
//! - Executions.
//! - Operations.
//! - Log events.
//! - Log fields.
//! - Field lengths.
//! - Computational domains.
//! - Providers.
//! - Devices.
//! - Distributed participants.
//! - Log sinks.
//!
//! Storage and throughput remain constrained by available resources.
//!
//! No finite machine promises literal infinite logging capacity.
//!
//! # Safety
//!
//! Safe Rust only.
//!
//! No unsafe blocks.
//! No unsafe implementations.
//! No global mutable state.
//! No implicit logger installation.
//! No fabricated logging success.
//!
//! # Integration
//!
//! RuntimeContext supplies the effective observability policy.
//!
//! RuntimeOperationId, ExecutionId and RuntimeInstanceId are
//! reused from zur::ids.
//!
//! ZurError and ZurResult are reused from zur::error.
//!
//! Metrics, tracing and profiling remain independently owned
//! observability components.
//!
//! Logging failures must not retroactively change the semantic
//! result of a computation. Callers must handle logging errors
//! separately from the computation's primary result.

use core::fmt;

use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

use crate::zur::core::context::RuntimeContext;

use crate::zur::error::{
    ErrorKind,
    ZurError,
    ZurResult,
};

use crate::zur::ids::{
    ExecutionId,
    RuntimeInstanceId,
    RuntimeOperationId,
};

use crate::zur::policy::ObservabilityPolicy;

// ============================================================================
// CONSTANTS
// ============================================================================

const COMPONENT: &str = "zur.observability.logging";

const REDACTED: &str = "[REDACTED]";

// ============================================================================
// LOG LEVEL
// ============================================================================

/// Severity of a runtime logging event.
///
/// Ordering is from least severe to most severe.
///
/// A minimum-level filter may suppress lower-severity events.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
)]
#[serde(rename_all = "snake_case")]
pub enum LogLevel {
    /// Detailed diagnostic information.
    Trace,

    /// Development and troubleshooting information.
    Debug,

    /// Normal operational information.
    Info,

    /// Unexpected but non-fatal conditions.
    Warn,

    /// Operational failures.
    Error,

    /// Serious failures requiring attention.
    Critical,
}

impl LogLevel {
    /// Returns the stable textual representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Trace => "trace",
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Error => "error",
            Self::Critical => "critical",
        }
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// LOG CLASS
// ============================================================================

/// Classification of a logging event.
///
/// Classification determines which observability policies
/// permit an event to be emitted.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
)]
#[serde(rename_all = "snake_case")]
pub enum LogClass {
    /// Ordinary diagnostic information.
    Diagnostic,

    /// Operational information required to understand execution.
    Operational,

    /// Security or audit information that must not be suppressed
    /// by ordinary observability filtering.
    ///
    /// This classification does not itself establish that an event
    /// is an authoritative security audit record.
    SecurityAudit,
}

// ============================================================================
// FIELD VISIBILITY
// ============================================================================

/// Visibility classification for logging data.
///
/// Sensitive values are never exposed through the public value
/// accessor, Debug formatting or JSON serialization.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
)]
#[serde(rename_all = "snake_case")]
pub enum LogVisibility {
    /// Information approved for ordinary logging.
    Public,

    /// Information that must be redacted.
    Sensitive,
}

impl LogVisibility {
    /// Returns whether the value is sensitive.
    #[must_use]
    pub const fn is_sensitive(self) -> bool {
        matches!(self, Self::Sensitive)
    }
}

// ============================================================================
// VALIDATION
// ============================================================================

fn validate_text(
    value: &str,
    description: &str,
) -> ZurResult<()> {
    if value.trim().is_empty() {
        return Err(
            ZurError::invalid_input(
                format!("{description} must not be empty"),
            )
            .with_component(COMPONENT)
            .with_operation("validate_text"),
        );
    }

    if value.chars().any(char::is_control) {
        return Err(
            ZurError::invalid_input(
                format!("{description} contains control characters"),
            )
            .with_component(COMPONENT)
            .with_operation("validate_text"),
        );
    }

    Ok(())
}

// ============================================================================
// LOG FIELD
// ============================================================================

/// Structured logging attribute.
///
/// Fields are key-value pairs with explicit visibility.
///
/// Duplicate field names within one event are rejected.
#[derive(Clone, PartialEq, Eq)]
pub struct LogField {
    key: String,
    value: String,
    visibility: LogVisibility,
}

impl LogField {
    /// Creates a public logging field.
    ///
    /// # Errors
    ///
    /// Returns an error when the key is empty or contains control
    /// characters.
    pub fn new(
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> ZurResult<Self> {
        Self::with_visibility(
            key,
            value,
            LogVisibility::Public,
        )
    }

    /// Creates a sensitive logging field.
    ///
    /// The original value remains private to this structure.
    /// Public accessors and serialization return a redacted value.
    pub fn sensitive(
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> ZurResult<Self> {
        Self::with_visibility(
            key,
            value,
            LogVisibility::Sensitive,
        )
    }

    /// Creates a field with an explicit visibility classification.
    ///
    /// # Errors
    ///
    /// Returns an error for an invalid field name.
    pub fn with_visibility(
        key: impl Into<String>,
        value: impl Into<String>,
        visibility: LogVisibility,
    ) -> ZurResult<Self> {
        let key = key.into();

        validate_text(&key, "logging field name")?;

        Ok(Self {
            key,
            value: value.into(),
            visibility,
        })
    }

    /// Returns the field name.
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the field value, redacting sensitive information.
    #[must_use]
    pub fn value(&self) -> &str {
        if self.visibility.is_sensitive() {
            REDACTED
        } else {
            &self.value
        }
    }

    /// Returns the visibility classification.
    #[must_use]
    pub const fn visibility(&self) -> LogVisibility {
        self.visibility
    }

    /// Returns whether this field contains sensitive information.
    #[must_use]
    pub const fn is_sensitive(&self) -> bool {
        self.visibility.is_sensitive()
    }
}

impl fmt::Debug for LogField {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("LogField")
            .field("key", &self.key)
            .field("value", &self.value())
            .field("visibility", &self.visibility)
            .finish()
    }
}

impl Serialize for LogField {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct(
            "LogField",
            3,
        )?;

        state.serialize_field("key", &self.key)?;
        state.serialize_field("value", &self.value())?;
        state.serialize_field(
            "visibility",
            &self.visibility,
        )?;

        state.end()
    }
}

// ============================================================================
// LOG EVENT
// ============================================================================

/// Immutable structured runtime log event.
///
/// An event carries optional canonical runtime correlation identities.
///
/// Identity presence does not prove that an execution or operation
/// is still active.
#[derive(Clone, PartialEq, Eq, Serialize)]
pub struct LogEvent {
    /// Wall-clock timestamp in nanoseconds since the Unix epoch.
    timestamp_unix_nanos: u128,

    /// Event severity.
    level: LogLevel,

    /// Event classification.
    class: LogClass,

    /// Logging component or target.
    target: String,

    /// Original message, protected by message visibility.
    #[serde(skip)]
    message: String,

    /// Message visibility.
    message_visibility: LogVisibility,

    /// Canonical runtime instance identity.
    runtime_instance_id: Option<RuntimeInstanceId>,

    /// Canonical execution identity.
    execution_id: Option<ExecutionId>,

    /// Canonical runtime operation identity.
    operation_id: Option<RuntimeOperationId>,

    /// Structured attributes.
    fields: Vec<LogField>,
}

impl LogEvent {
    /// Creates a log event using the current system wall-clock time.
    ///
    /// The timestamp is observational metadata, not a monotonic
    /// execution-duration measurement.
    ///
    /// # Errors
    ///
    /// Returns an error when the target is invalid.
    pub fn new(
        level: LogLevel,
        target: impl Into<String>,
        message: impl Into<String>,
    ) -> ZurResult<Self> {
        let target = target.into();

        validate_text(&target, "logging target")?;

        let timestamp_unix_nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();

        Ok(Self {
            timestamp_unix_nanos,
            level,
            class: LogClass::Operational,
            target,
            message: message.into(),
            message_visibility: LogVisibility::Public,
            runtime_instance_id: None,
            execution_id: None,
            operation_id: None,
            fields: Vec::new(),
        })
    }

    /// Returns the event timestamp.
    #[must_use]
    pub const fn timestamp_unix_nanos(&self) -> u128 {
        self.timestamp_unix_nanos
    }

    /// Returns the event severity.
    #[must_use]
    pub const fn level(&self) -> LogLevel {
        self.level
    }

    /// Returns the event classification.
    #[must_use]
    pub const fn class(&self) -> LogClass {
        self.class
    }

    /// Returns the event target.
    #[must_use]
    pub fn target(&self) -> &str {
        &self.target
    }

    /// Returns the message, redacting sensitive messages.
    #[must_use]
    pub fn message(&self) -> &str {
        if self.message_visibility.is_sensitive() {
            REDACTED
        } else {
            &self.message
        }
    }

    /// Returns the message visibility.
    #[must_use]
    pub const fn message_visibility(&self) -> LogVisibility {
        self.message_visibility
    }

    /// Returns the runtime instance identity.
    #[must_use]
    pub fn runtime_instance_id(
        &self,
    ) -> Option<&RuntimeInstanceId> {
        self.runtime_instance_id.as_ref()
    }

    /// Returns the execution identity.
    #[must_use]
    pub fn execution_id(&self) -> Option<&ExecutionId> {
        self.execution_id.as_ref()
    }

    /// Returns the runtime operation identity.
    #[must_use]
    pub fn operation_id(&self) -> Option<&RuntimeOperationId> {
        self.operation_id.as_ref()
    }

    /// Returns structured fields.
    #[must_use]
    pub fn fields(&self) -> &[LogField] {
        &self.fields
    }

    /// Changes the event classification.
    #[must_use]
    pub fn with_class(
        mut self,
        class: LogClass,
    ) -> Self {
        self.class = class;
        self
    }

    /// Marks the message as sensitive.
    ///
    /// Its original contents will not be exposed by public accessors,
    /// Debug formatting or serialization.
    #[must_use]
    pub fn with_sensitive_message(mut self) -> Self {
        self.message_visibility = LogVisibility::Sensitive;
        self
    }

    /// Associates a runtime instance identity.
    #[must_use]
    pub fn with_runtime_instance(
        mut self,
        identity: RuntimeInstanceId,
    ) -> Self {
        self.runtime_instance_id = Some(identity);
        self
    }

    /// Associates an execution identity.
    #[must_use]
    pub fn with_execution(
        mut self,
        identity: ExecutionId,
    ) -> Self {
        self.execution_id = Some(identity);
        self
    }

    /// Associates a runtime operation identity.
    #[must_use]
    pub fn with_operation(
        mut self,
        identity: RuntimeOperationId,
    ) -> Self {
        self.operation_id = Some(identity);
        self
    }

    /// Adds a structured field.
    ///
    /// Duplicate field names are rejected.
    ///
    /// The collection grows according to available memory.
    ///
    /// # Errors
    ///
    /// Returns an error for duplicate field names or unavailable
    /// allocation capacity.
    pub fn with_field(
        mut self,
        field: LogField,
    ) -> ZurResult<Self> {
        if self.fields.iter().any(
            |existing| existing.key() == field.key(),
        ) {
            return Err(
                ZurError::invalid_input(
                    format!(
                        "duplicate logging field: {}",
                        field.key()
                    ),
                )
                .with_component(COMPONENT)
                .with_operation("with_field"),
            );
        }

        self.fields.try_reserve(1).map_err(|error| {
            ZurError::resource_unavailable(
                format!("unable to reserve logging field storage: {error}"),
            )
            .with_component(COMPONENT)
            .with_operation("with_field")
        })?;

        self.fields.push(field);

        Ok(self)
    }
}

impl fmt::Debug for LogEvent {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("LogEvent")
            .field(
                "timestamp_unix_nanos",
                &self.timestamp_unix_nanos,
            )
            .field("level", &self.level)
            .field("class", &self.class)
            .field("target", &self.target)
            .field("message", &self.message())
            .field(
                "runtime_instance_id",
                &self.runtime_instance_id,
            )
            .field("execution_id", &self.execution_id)
            .field("operation_id", &self.operation_id)
            .field("fields", &self.fields)
            .finish()
    }
}

// ============================================================================
// LOGGING CONFIGURATION
// ============================================================================

/// Configuration for one explicitly constructed logger.
///
/// This configuration does not modify the runtime's canonical
/// RuntimeConfig or RuntimePolicy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LogConfig {
    policy: ObservabilityPolicy,
    minimum_level: LogLevel,
}

impl LogConfig {
    /// Constructs logging configuration from an observability policy.
    ///
    /// The default severity filter is derived from the policy.
    #[must_use]
    pub const fn new(policy: ObservabilityPolicy) -> Self {
        let minimum_level = match policy {
            ObservabilityPolicy::ErrorsOnly => LogLevel::Error,
            ObservabilityPolicy::Standard => LogLevel::Info,
            ObservabilityPolicy::Detailed => LogLevel::Trace,
            ObservabilityPolicy::Disabled => LogLevel::Error,
        };

        Self {
            policy,
            minimum_level,
        }
    }

    /// Returns the observability policy.
    #[must_use]
    pub const fn policy(self) -> ObservabilityPolicy {
        self.policy
    }

    /// Returns the minimum permitted severity.
    #[must_use]
    pub const fn minimum_level(self) -> LogLevel {
        self.minimum_level
    }

    /// Replaces the minimum severity filter.
    ///
    /// The filter can restrict ordinary logging further but cannot
    /// suppress mandatory security-audit classifications.
    #[must_use]
    pub const fn with_minimum_level(
        mut self,
        minimum_level: LogLevel,
    ) -> Self {
        self.minimum_level = minimum_level;
        self
    }

    /// Validates the logging configuration.
    ///
    /// Every current configuration is structurally valid.
    ///
    /// This method exists as the stable configuration-validation
    /// integration boundary for future policy extensions.
    pub fn validate(self) -> ZurResult<()> {
        Ok(())
    }

    fn permits(self, event: &LogEvent) -> bool {
        // Audit classification cannot be disabled through ordinary
        // observability filtering.
        if event.class() == LogClass::SecurityAudit {
            return true;
        }

        match self.policy {
            ObservabilityPolicy::Disabled => false,

            ObservabilityPolicy::ErrorsOnly => {
                event.class() == LogClass::Operational
                    && event.level() >= LogLevel::Error
            }

            ObservabilityPolicy::Standard => {
                event.level() >= self.minimum_level
                    && match event.class() {
                        LogClass::Diagnostic => {
                            event.level() >= LogLevel::Warn
                        }
                        LogClass::Operational => true,
                        LogClass::SecurityAudit => true,
                    }
            }

            ObservabilityPolicy::Detailed => {
                event.level() >= self.minimum_level
            }
        }
    }
}

// ============================================================================
// LOG DISPOSITION
// ============================================================================

/// Result of applying the logger's filtering policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogDisposition {
    /// The event was accepted by the configured sink.
    Emitted,

    /// The event was suppressed by the configured policy.
    SuppressedByPolicy,
}

// ============================================================================
// LOG SINK
// ============================================================================

/// Contract for an explicitly injected structured logging sink.
///
/// Implementations must:
///
/// - Respect the redacted public event interface.
/// - Be safe for concurrent calls.
/// - Report sink failures.
/// - Avoid changing program semantics.
/// - Avoid interpreting logs as source-language semantics.
///
/// A sink must not assume that receiving an event authorizes
/// arbitrary external export.
pub trait LogSink: Send + Sync {
    /// Writes one structured event.
    ///
    /// # Errors
    ///
    /// Returns an error when the destination cannot accept the event.
    fn write(&self, event: &LogEvent) -> ZurResult<()>;
}

// ============================================================================
// JSON LINES SINK
// ============================================================================

/// Thread-safe JSON Lines logging sink.
///
/// Each event is written as one JSON object followed by a newline.
///
/// The writer is explicitly supplied by the caller.
///
/// This sink does not decide whether the destination is a file,
/// terminal, socket or another supported writer.
pub struct JsonLineSink<W>
where
    W: Write + Send,
{
    writer: Mutex<W>,
}

impl<W> JsonLineSink<W>
where
    W: Write + Send,
{
    /// Constructs a JSON Lines sink around a caller-owned writer.
    #[must_use]
    pub fn new(writer: W) -> Self {
        Self {
            writer: Mutex::new(writer),
        }
    }

    /// Flushes the underlying writer.
    ///
    /// # Errors
    ///
    /// Returns an error if the lock is poisoned or the writer
    /// cannot be flushed.
    pub fn flush(&self) -> ZurResult<()> {
        let mut writer = self.writer.lock().map_err(|_| {
            ZurError::invalid_data(
                "JSON Lines writer lock is poisoned",
            )
            .with_component(COMPONENT)
            .with_operation("flush")
        })?;

        writer.flush().map_err(|error| {
            ZurError::from(error)
                .with_component(COMPONENT)
                .with_operation("flush")
        })
    }

    /// Consumes the sink and returns its underlying writer.
    ///
    /// This is useful for finalizing explicitly owned output.
    ///
    /// # Errors
    ///
    /// Returns an error if the writer mutex is poisoned.
    pub fn into_inner(self) -> ZurResult<W> {
        self.writer.into_inner().map_err(|_| {
            ZurError::invalid_data(
                "JSON Lines writer lock is poisoned",
            )
            .with_component(COMPONENT)
            .with_operation("into_inner")
        })
    }
}

impl<W> LogSink for JsonLineSink<W>
where
    W: Write + Send,
{
    fn write(&self, event: &LogEvent) -> ZurResult<()> {
        let mut writer = self.writer.lock().map_err(|_| {
            ZurError::invalid_data(
                "JSON Lines writer lock is poisoned",
            )
            .with_component(COMPONENT)
            .with_operation("write")
        })?;

        serde_json::to_writer(&mut *writer, event).map_err(
            |error| {
                ZurError::with_code(
                    ErrorKind::Serialization,
                    "zur.logging.serialization",
                    error.to_string(),
                )
                .unwrap_or_else(|_| {
                    ZurError::invalid_data(
                        "unable to serialize logging event",
                    )
                })
                .with_component(COMPONENT)
                .with_operation("write")
            },
        )?;

        writer.write_all(b"\n").map_err(|error| {
            ZurError::from(error)
                .with_component(COMPONENT)
                .with_operation("write")
        })
    }
}

// ============================================================================
// LOGGER
// ============================================================================

/// Explicitly constructed structured logger.
///
/// A Logger owns:
///
/// - Immutable logging configuration.
/// - A shared logging sink.
///
/// It does not own global state, runtime lifecycle, identity
/// allocation, tracing, metrics or profiling.
///
/// Cloning a Logger shares the same configuration and sink.
#[derive(Clone)]
pub struct Logger {
    configuration: LogConfig,
    sink: Arc<dyn LogSink>,
}

impl Logger {
    /// Constructs a logger with an explicitly supplied sink.
    ///
    /// # Errors
    ///
    /// Returns an error when configuration validation fails.
    pub fn new<S>(
        configuration: LogConfig,
        sink: Arc<S>,
    ) -> ZurResult<Self>
    where
        S: LogSink + 'static,
    {
        configuration.validate()?;

        Ok(Self {
            configuration,
            sink,
        })
    }

    /// Constructs a logger using an existing runtime context.
    ///
    /// The logger consumes the canonical runtime observability
    /// policy without taking ownership of runtime configuration.
    ///
    /// # Errors
    ///
    /// Returns an error when configuration validation fails.
    pub fn from_context<S>(
        context: &RuntimeContext,
        sink: Arc<S>,
    ) -> ZurResult<Self>
    where
        S: LogSink + 'static,
    {
        let configuration = LogConfig::new(
            context
                .runtime_policy()
                .observability_policy(),
        );

        Self::new(configuration, sink)
    }

    /// Returns the immutable logging configuration.
    #[must_use]
    pub const fn configuration(&self) -> LogConfig {
        self.configuration
    }

    /// Emits a structured event.
    ///
    /// Policy filtering is applied before the sink is invoked.
    ///
    /// Security-audit events are not suppressed by ordinary
    /// observability policies.
    ///
    /// # Errors
    ///
    /// Returns an error if the sink fails.
    ///
    /// Logging errors must be handled separately from the primary
    /// computation result.
    pub fn emit(
        &self,
        event: &LogEvent,
    ) -> ZurResult<LogDisposition> {
        if !self.configuration.permits(event) {
            return Ok(LogDisposition::SuppressedByPolicy);
        }

        self.sink
            .write(event)
            .map_err(|error| {
                error
                    .with_component(COMPONENT)
                    .with_operation("emit")
            })?;

        Ok(LogDisposition::Emitted)
    }

    /// Emits an event with an explicit runtime identity.
    ///
    /// The identity is supplied by the caller. This logger does
    /// not create or allocate runtime identities.
    pub fn emit_for_runtime(
        &self,
        identity: RuntimeInstanceId,
        event: LogEvent,
    ) -> ZurResult<LogDisposition> {
        self.emit(
            &event.with_runtime_instance(identity),
        )
    }

    /// Emits an event with an explicit execution identity.
    pub fn emit_for_execution(
        &self,
        identity: ExecutionId,
        event: LogEvent,
    ) -> ZurResult<LogDisposition> {
        self.emit(
            &event.with_execution(identity),
        )
    }

    /// Emits an event with an explicit operation identity.
    pub fn emit_for_operation(
        &self,
        identity: RuntimeOperationId,
        event: LogEvent,
    ) -> ZurResult<LogDisposition> {
        self.emit(
            &event.with_operation(identity),
        )
    }
}

impl fmt::Debug for Logger {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("Logger")
            .field("configuration", &self.configuration)
            .field("sink", &"injected")
            .finish()
    }
}

// ============================================================================
// ERROR ADAPTERS
// ============================================================================

/// Converts a standard I/O logging error into a structured ZUR error.
///
/// This preserves the original causal error.
pub fn logging_io_error(
    error: io::Error,
    operation: &'static str,
) -> ZurError {
    ZurError::from(error)
        .with_component(COMPONENT)
        .with_operation(operation)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Mutex;

    #[derive(Default)]
    struct TestSink {
        events: Mutex<Vec<String>>,
    }

    impl LogSink for TestSink {
        fn write(
            &self,
            event: &LogEvent,
        ) -> ZurResult<()> {
            let serialized = serde_json::to_string(event)
                .map_err(|error| {
                    ZurError::invalid_data(error.to_string())
                })?;

            self.events
                .lock()
                .map_err(|_| {
                    ZurError::invalid_data(
                        "test sink lock poisoned",
                    )
                })?
                .push(serialized);

            Ok(())
        }
    }

    fn test_event(
        level: LogLevel,
    ) -> LogEvent {
        LogEvent::new(
            level,
            "zur.test",
            "test event",
        )
        .expect("valid test event")
    }

    fn test_logger(
        policy: ObservabilityPolicy,
        sink: Arc<TestSink>,
    ) -> Logger {
        Logger::new(
            LogConfig::new(policy),
            sink,
        )
        .expect("valid logger")
    }

    #[test]
    fn event_creation_preserves_target_and_message() {
        let event = test_event(LogLevel::Info);

        assert_eq!(event.target(), "zur.test");
        assert_eq!(event.message(), "test event");
        assert_eq!(event.level(), LogLevel::Info);
    }

    #[test]
    fn empty_target_is_rejected() {
        assert!(
            LogEvent::new(
                LogLevel::Info,
                " ",
                "message",
            )
            .is_err()
        );
    }

    #[test]
    fn control_characters_in_target_are_rejected() {
        assert!(
            LogEvent::new(
                LogLevel::Info,
                "zur.\ninvalid",
                "message",
            )
            .is_err()
        );
    }

    #[test]
    fn sensitive_fields_are_redacted() {
        let field = LogField::sensitive(
            "credential",
            "secret-value",
        )
        .expect("valid field");

        assert_eq!(field.value(), REDACTED);
        assert!(field.is_sensitive());

        let event = test_event(LogLevel::Info)
            .with_field(field)
            .expect("field insertion");

        let serialized = serde_json::to_string(&event)
            .expect("serialization");

        assert!(!serialized.contains("secret-value"));
        assert!(serialized.contains(REDACTED));
    }

    #[test]
    fn sensitive_messages_are_redacted() {
        let event = LogEvent::new(
            LogLevel::Info,
            "zur.test",
            "private information",
        )
        .expect("event")
        .with_sensitive_message();

        assert_eq!(event.message(), REDACTED);

        let serialized = serde_json::to_string(&event)
            .expect("serialization");

        assert!(!serialized.contains("private information"));
    }

    #[test]
    fn duplicate_fields_are_rejected() {
        let event = test_event(LogLevel::Info)
            .with_field(
                LogField::new("operation", "first")
                    .expect("field"),
            )
            .expect("first insertion");

        assert!(
            event
                .with_field(
                    LogField::new("operation", "second")
                        .expect("field"),
                )
                .is_err()
        );
    }

    #[test]
    fn errors_only_policy_emits_operational_errors() {
        let sink = Arc::new(TestSink::default());

        let logger = test_logger(
            ObservabilityPolicy::ErrorsOnly,
            Arc::clone(&sink),
        );

        assert_eq!(
            logger.emit(&test_event(LogLevel::Info))
                .expect("emit"),
            LogDisposition::SuppressedByPolicy
        );

        assert_eq!(
            logger.emit(&test_event(LogLevel::Error))
                .expect("emit"),
            LogDisposition::Emitted
        );

        assert_eq!(
            sink.events.lock().expect("lock").len(),
            1
        );
    }

    #[test]
    fn disabled_policy_preserves_security_audit_events() {
        let sink = Arc::new(TestSink::default());

        let logger = test_logger(
            ObservabilityPolicy::Disabled,
            Arc::clone(&sink),
        );

        let event = test_event(LogLevel::Info)
            .with_class(LogClass::SecurityAudit);

        assert_eq!(
            logger.emit(&event).expect("emit"),
            LogDisposition::Emitted
        );

        assert_eq!(
            sink.events.lock().expect("lock").len(),
            1
        );
    }

    #[test]
    fn disabled_policy_suppresses_ordinary_events() {
        let sink = Arc::new(TestSink::default());

        let logger = test_logger(
            ObservabilityPolicy::Disabled,
            Arc::clone(&sink),
        );

        assert_eq!(
            logger.emit(&test_event(LogLevel::Critical))
                .expect("emit"),
            LogDisposition::SuppressedByPolicy
        );

        assert!(sink.events.lock().expect("lock").is_empty());
    }

    #[test]
    fn standard_policy_respects_minimum_level() {
        let sink = Arc::new(TestSink::default());

        let configuration = LogConfig::new(
            ObservabilityPolicy::Standard,
        )
        .with_minimum_level(LogLevel::Warn);

        let logger = Logger::new(
            configuration,
            Arc::clone(&sink),
        )
        .expect("logger");

        assert_eq!(
            logger.emit(&test_event(LogLevel::Info))
                .expect("emit"),
            LogDisposition::SuppressedByPolicy
        );

        assert_eq!(
            logger.emit(&test_event(LogLevel::Warn))
                .expect("emit"),
            LogDisposition::Emitted
        );
    }

    #[test]
    fn debug_formatting_redacts_sensitive_fields() {
        let event = test_event(LogLevel::Info)
            .with_field(
                LogField::sensitive(
                    "token",
                    "private-token",
                )
                .expect("field"),
            )
            .expect("event");

        let debug = format!("{event:?}");

        assert!(!debug.contains("private-token"));
        assert!(debug.contains(REDACTED));
    }

    #[test]
    fn json_lines_sink_writes_separate_records() {
        let sink = JsonLineSink::new(Vec::<u8>::new());

        sink.write(&test_event(LogLevel::Info))
            .expect("first write");

        sink.write(&test_event(LogLevel::Error))
            .expect("second write");

        let bytes = sink.into_inner()
            .expect("writer extraction");

        let output = String::from_utf8(bytes)
            .expect("UTF-8");

        let records: Vec<&str> = output.lines().collect();

        assert_eq!(records.len(), 2);

        for record in records {
            serde_json::from_str::<serde_json::Value>(record)
                .expect("valid JSON record");
        }
    }

    #[test]
    fn json_lines_sink_flushes() {
        let sink = JsonLineSink::new(Vec::<u8>::new());

        assert!(sink.flush().is_ok());
    }

    #[test]
    fn cloned_loggers_share_the_same_sink() {
        let sink = Arc::new(TestSink::default());

        let logger = test_logger(
            ObservabilityPolicy::Detailed,
            Arc::clone(&sink),
        );

        let cloned = logger.clone();

        logger.emit(&test_event(LogLevel::Info))
            .expect("first emit");

        cloned.emit(&test_event(LogLevel::Warn))
            .expect("second emit");

        assert_eq!(
            sink.events.lock().expect("lock").len(),
            2
        );
    }

    #[test]
    fn field_visibility_is_preserved() {
        let public = LogField::new(
            "status",
            "ready",
        )
        .expect("public");

        let sensitive = LogField::sensitive(
            "password",
            "secret",
        )
        .expect("sensitive");

        assert_eq!(public.value(), "ready");
        assert_eq!(sensitive.value(), REDACTED);
    }

    #[test]
    fn audit_classification_is_independent_of_severity() {
        let sink = Arc::new(TestSink::default());

        let logger = test_logger(
            ObservabilityPolicy::Disabled,
            Arc::clone(&sink),
        );

        let event = test_event(LogLevel::Trace)
            .with_class(LogClass::SecurityAudit);

        assert_eq!(
            logger.emit(&event).expect("emit"),
            LogDisposition::Emitted
        );
    }
}
