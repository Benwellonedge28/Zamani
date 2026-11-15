
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Structured Hardware Diagnostics
//!
//! Path: src/zhal/observability/diagnostics.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines portable, structured, redaction-aware diagnostic records
//! for hardware abstraction operations.
//!
//! Diagnostics cover hardware-provider interactions across:
//!
//! - Classical computing.
//! - Quantum computing.
//! - HDL realization.
//! - FPGA and ASIC hardware.
//! - AI and computational accelerators.
//! - Embedded computing.
//! - Hybrid computing.
//! - Distributed and remote hardware.
//! - Future computational technologies.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Structured ZHAL diagnostic records.
//! - Diagnostic origin identities.
//! - Diagnostic schema identification.
//! - Diagnostic context snapshots.
//! - Redaction-aware error-report composition.
//! - Correlation-reference propagation.
//! - Structured diagnostic serialization.
//! - Diagnostic record validation.
//!
//! This module does NOT own:
//!
//! - ZHAL error classification.
//! - Hardware identity.
//! - Provider identity.
//! - Device discovery.
//! - Hardware health evaluation.
//! - Runtime logging.
//! - Runtime tracing.
//! - Metric collection.
//! - Event persistence.
//! - Distributed transport.
//! - Authentication.
//! - Authorization.
//! - Recovery.
//! - Scheduling.
//! - Resource allocation.
//! - Quantum fault correction.
//! - HDL synthesis or realization.
//! - Compiler diagnostics.
//! - Source-language semantic validation.
//!
//! ## Canonical authorities
//!
//! ZHAL errors:
//!     crate::zhal::errors
//!
//! Hardware identity:
//!     crate::zhal::identity
//!
//! Operation context:
//!     crate::zhal::context
//!
//! Standard-library provider identity:
//!     crate::stdlib::provider
//!
//! Standard-library cancellation:
//!     crate::stdlib::context
//!
//! ZUR logging:
//!     crate::zur::observability::logging
//!
//! Distributed tracing:
//!     crate::distributed::observability::tracing
//!
//! Compiler diagnostics:
//!     crate::compiler::diagnostics
//!
//! ## Diagnostic semantics
//!
//! A diagnostic is an observation about an operation or condition.
//!
//! It is not proof that a hardware operation succeeded or failed.
//!
//! In particular:
//!
//! - Unknown operation outcomes must remain unknown.
//! - Partial completion must remain explicit.
//! - Cancellation requests must not imply physical termination.
//! - Retryability must not trigger automatic retries.
//! - Missing diagnostic context must not imply successful execution.
//!
//! ## Security
//!
//! Diagnostic reports reuse ZhalError::report().
//!
//! Sensitive error context is therefore redacted by the canonical
//! ZHAL error implementation.
//!
//! Security-context references and credentials are deliberately
//! excluded from ordinary diagnostic snapshots.
//!
//! Error messages and public references must be safe summaries.
//! Callers must never put secrets in public diagnostic messages.
//!
//! ## Scalability
//!
//! No arbitrary maximum is imposed on:
//!
//! - Diagnostic origin length.
//! - Diagnostic context-reference length.
//! - Provider count.
//! - Device count.
//! - Hardware resource count.
//! - Computational domains.
//! - Diagnostic records.
//! - Operation count.
//!
//! This module does not maintain an unbounded in-memory journal.
//!
//! Records are created on demand and handed to externally owned
//! reporting infrastructure.
//!
//! Actual memory and serialization capacity remain constrained by
//! available resources and platform representation limits.
//!
//! Literal infinite memory or computation is not promised.
//!
//! ## Grammar boundary
//!
//! Relevant specifications:
//!
//! - grammar/effects/hardware.g4
//! - grammar/execution/execution-context.g4
//! - grammar/execution/distributed-execution.g4
//! - grammar/specification/semantic-model.md
//! - grammar/specification/execution-model.md
//!
//! Hardware effects describe computational intent.
//!
//! Execution-context grammar describes execution intent.
//!
//! Distributed execution grammar describes logical distributed intent.
//!
//! None of these grammars defines physical hardware diagnostics.
//!
//! This module reports runtime observations without interpreting
//! source syntax or modifying program semantics.
//!
//! ## Dependencies
//!
//! - Rust standard library.
//! - serde.
//! - crate::zhal::errors.
//! - crate::zhal::context.
//!
//! No hardware SDK or operating-system-specific API is required.

use core::fmt;

use serde::Serialize;

use crate::zhal::context::ZhalContext;

use crate::zhal::errors::{
    ZhalError,
    ZhalErrorKind,
    ZhalErrorReport,
    ZhalOperationOutcome,
    ZhalRetryability,
    ZhalSeverity,
};

// ============================================================================
// MODULE CONTRACT
// ============================================================================

/// Stable diagnostic component identity.
pub const COMPONENT: &str = "zhal.observability.diagnostics";

/// Stable diagnostic schema identifier.
///
/// Incompatible changes to the serialized diagnostic contract
/// require a new schema identifier.
pub const DIAGNOSTIC_SCHEMA: &str =
    "zamani.zhal.diagnostics.v1";

/// Diagnostic schema version.
///
/// This is independent of the Zamani language version,
/// Cargo package version, ZHAL API version and provider API version.
pub const DIAGNOSTIC_SCHEMA_VERSION: u16 = 1;

/// Safety policy.
pub const SAFETY_POLICY: &str =
    "unsafe-rust-forbidden";

/// Scalability policy.
pub const SCALABILITY_POLICY: &str =
    "open-world-resource-bounded-diagnostics";

// ============================================================================
// VALIDATION
// ============================================================================

/// Classification of diagnostic validation failures.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum DiagnosticValidationError {
    /// The diagnostic origin is empty.
    EmptyOrigin,

    /// The diagnostic origin contains whitespace.
    WhitespaceInOrigin,

    /// The diagnostic origin contains control characters.
    ControlCharacterInOrigin,
}

impl fmt::Display for DiagnosticValidationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let message = match self {
            Self::EmptyOrigin => {
                "diagnostic origin must not be empty"
            }

            Self::WhitespaceInOrigin => {
                "diagnostic origin must not contain whitespace"
            }

            Self::ControlCharacterInOrigin => {
                "diagnostic origin must not contain control characters"
            }
        };

        formatter.write_str(message)
    }
}

impl std::error::Error for DiagnosticValidationError {}

// ============================================================================
// DIAGNOSTIC ORIGIN
// ============================================================================

/// Open-world identity describing where a diagnostic originated.
///
/// Examples:
///
/// - `zhal.providers.discovery`
/// - `zhal.devices.access`
/// - `zhal.devices.health`
/// - `zhal.operations.submission`
/// - `zhal.adapters.quantum`
/// - `zhal.adapters.hdl`
/// - `zhal.adapters.classical`
/// - `zhal.adapters.distributed`
/// - `future.photonic.provider`
///
/// These examples are not a closed enumeration.
///
/// Future hardware technologies and providers can introduce
/// new diagnostic origins without modifying this module.
///
/// An origin identifies a reporting component, not a physical device.
#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[serde(transparent)]
pub struct DiagnosticOrigin(String);

impl DiagnosticOrigin {
    /// Constructs a validated diagnostic origin.
    ///
    /// Exact spelling is preserved.
    ///
    /// No arbitrary maximum length is imposed.
    ///
    /// # Errors
    ///
    /// Returns a validation error for empty values,
    /// whitespace or control characters.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, DiagnosticValidationError> {
        let value = value.into();

        if value.is_empty() {
            return Err(
                DiagnosticValidationError::EmptyOrigin,
            );
        }

        if value.chars().any(char::is_control) {
            return Err(
                DiagnosticValidationError::ControlCharacterInOrigin,
            );
        }

        if value.chars().any(char::is_whitespace) {
            return Err(
                DiagnosticValidationError::WhitespaceInOrigin,
            );
        }

        Ok(Self(value))
    }

    /// Returns the exact origin identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the origin and returns its owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }

    /// Returns the byte length.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns whether the origin is empty.
    ///
    /// Valid instances cannot be empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        false
    }
}

impl fmt::Display for DiagnosticOrigin {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl AsRef<str> for DiagnosticOrigin {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

// ============================================================================
// DIAGNOSTIC CONTEXT SNAPSHOT
// ============================================================================

/// Immutable diagnostic correlation information.
///
/// This is a snapshot of descriptive references from an existing
/// ZHAL operation context.
///
/// It does not own cancellation, deadlines, security or authorization.
///
/// Security-context references are deliberately excluded.
///
/// The snapshot contains no physical hardware handles or credentials.
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Serialize,
)]
pub struct DiagnosticContextSnapshot {
    /// External correlation reference, if available.
    ///
    /// This is supplied by the existing tracing or caller authority.
    pub correlation_id: Option<String>,

    /// External diagnostic-context reference, if available.
    pub diagnostic_reference: Option<String>,

    /// Canonical provider identity, if associated with the context.
    pub provider_id: Option<String>,
}

impl DiagnosticContextSnapshot {
    /// Creates an empty diagnostic context snapshot.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            correlation_id: None,
            diagnostic_reference: None,
            provider_id: None,
        }
    }

    /// Captures reporting-safe references from an existing ZHAL context.
    ///
    /// The original context is not modified.
    ///
    /// Security-context references are intentionally excluded.
    #[must_use]
    pub fn capture(
        context: &ZhalContext,
    ) -> Self {
        Self {
            correlation_id: context
                .correlation_id()
                .map(str::to_owned),

            diagnostic_reference: context
                .diagnostic_reference()
                .map(str::to_owned),

            provider_id: context
                .provider_id()
                .map(ToString::to_string),
        }
    }

    /// Returns the correlation reference.
    #[must_use]
    pub fn correlation_id(&self) -> Option<&str> {
        self.correlation_id.as_deref()
    }

    /// Returns the diagnostic reference.
    #[must_use]
    pub fn diagnostic_reference(&self) -> Option<&str> {
        self.diagnostic_reference.as_deref()
    }

    /// Returns the provider identity.
    #[must_use]
    pub fn provider_id(&self) -> Option<&str> {
        self.provider_id.as_deref()
    }

    /// Returns whether all contextual references are absent.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.correlation_id.is_none()
            && self.diagnostic_reference.is_none()
            && self.provider_id.is_none()
    }
}

// ============================================================================
// DIAGNOSTIC RECORD
// ============================================================================

/// Structured, portable ZHAL diagnostic record.
///
/// A diagnostic is an immutable reporting snapshot.
///
/// The canonical error authority remains `ZhalError`.
///
/// The diagnostic record uses `ZhalError::report()` to preserve:
///
/// - Error classification.
/// - Extensible error code.
/// - Safe message.
/// - Provider attribution.
/// - Device attribution.
/// - Session attribution.
/// - Operation attribution.
/// - Severity.
/// - Retryability.
/// - Operation outcome.
/// - Redaction-aware structured context.
///
/// Arbitrary causal error objects are not serialized.
///
/// This record does not contain execution payloads, credentials,
/// physical device handles or resource reservations.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ZhalDiagnostic {
    /// Diagnostic schema identifier.
    pub schema: &'static str,

    /// Diagnostic schema version.
    pub schema_version: u16,

    /// Reporting component identity.
    pub component: &'static str,

    /// Component that produced the diagnostic.
    pub origin: DiagnosticOrigin,

    /// Redaction-aware canonical ZHAL error report.
    pub error: ZhalErrorReport,

    /// Safe operation-correlation snapshot.
    pub context: DiagnosticContextSnapshot,
}

impl ZhalDiagnostic {
    /// Constructs a diagnostic record from an existing ZHAL error.
    ///
    /// The original error is not modified or consumed.
    ///
    /// The supplied origin identifies the component reporting
    /// the diagnostic, not necessarily the component that first
    /// caused the underlying failure.
    ///
    /// # Errors
    ///
    /// Returns a validation error when the origin is invalid.
    pub fn from_error(
        origin: impl Into<String>,
        error: &ZhalError,
        context: Option<&ZhalContext>,
    ) -> Result<Self, DiagnosticValidationError> {
        let origin = DiagnosticOrigin::new(origin)?;

        let context = context
            .map(DiagnosticContextSnapshot::capture)
            .unwrap_or_else(DiagnosticContextSnapshot::empty);

        Ok(Self {
            schema: DIAGNOSTIC_SCHEMA,
            schema_version: DIAGNOSTIC_SCHEMA_VERSION,
            component: COMPONENT,
            origin,
            error: error.report(),
            context,
        })
    }

    /// Returns the diagnostic schema identifier.
    #[must_use]
    pub const fn schema(&self) -> &'static str {
        self.schema
    }

    /// Returns the schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    /// Returns the reporting component.
    #[must_use]
    pub const fn component(&self) -> &'static str {
        self.component
    }

    /// Returns the diagnostic origin.
    #[must_use]
    pub fn origin(&self) -> &DiagnosticOrigin {
        &self.origin
    }

    /// Returns the canonical redaction-aware error report.
    #[must_use]
    pub fn error(&self) -> &ZhalErrorReport {
        &self.error
    }

    /// Returns the contextual correlation snapshot.
    #[must_use]
    pub fn context(&self) -> &DiagnosticContextSnapshot {
        &self.context
    }

    /// Returns the canonical error classification.
    #[must_use]
    pub const fn kind(&self) -> ZhalErrorKind {
        self.error.kind
    }

    /// Returns the canonical reporting severity.
    #[must_use]
    pub const fn severity(&self) -> ZhalSeverity {
        self.error.severity
    }

    /// Returns the canonical retryability classification.
    ///
    /// This is descriptive metadata only.
    #[must_use]
    pub const fn retryability(&self) -> ZhalRetryability {
        self.error.retryability
    }

    /// Returns the known operation outcome.
    ///
    /// An unknown outcome is never converted into a failed outcome.
    #[must_use]
    pub const fn outcome(&self) -> ZhalOperationOutcome {
        self.error.outcome
    }

    /// Returns whether the operation outcome is unknown.
    #[must_use]
    pub const fn outcome_is_unknown(&self) -> bool {
        matches!(
            self.error.outcome,
            ZhalOperationOutcome::Unknown,
        )
    }

    /// Returns whether the operation partially completed.
    #[must_use]
    pub const fn is_partial_completion(&self) -> bool {
        matches!(
            self.error.outcome,
            ZhalOperationOutcome::PartiallyCompleted,
        )
    }

    /// Returns whether the canonical error explicitly establishes
    /// that retrying is safe.
    ///
    /// This does not initiate a retry.
    #[must_use]
    pub const fn is_safe_to_retry(&self) -> bool {
        self.error.retryability.is_safe_to_retry()
    }

    /// Returns the safe error message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.error.message
    }

    /// Returns the canonical provider identity, if available.
    #[must_use]
    pub fn provider_id(&self) -> Option<&str> {
        self.error.provider_id.as_deref()
    }

    /// Returns the provider-scoped device identity, if available.
    #[must_use]
    pub fn device(&self) -> Option<&str> {
        self.error.device.as_deref()
    }

    /// Returns the provider-scoped session identity, if available.
    #[must_use]
    pub fn session(&self) -> Option<&str> {
        self.error.session.as_deref()
    }

    /// Returns the hardware operation identity, if available.
    #[must_use]
    pub fn operation(&self) -> Option<&str> {
        self.error.operation.as_deref()
    }

    /// Returns whether the record contains an operation correlation ID.
    #[must_use]
    pub fn has_correlation_id(&self) -> bool {
        self.context.correlation_id.is_some()
    }
}

// ============================================================================
// COMPONENT API
// ============================================================================

/// Returns the stable component identity.
#[must_use]
pub const fn component_id() -> &'static str {
    COMPONENT
}

/// Returns the diagnostic schema identifier.
#[must_use]
pub const fn schema_id() -> &'static str {
    DIAGNOSTIC_SCHEMA
}

/// Returns the diagnostic schema version.
#[must_use]
pub const fn schema_version() -> u16 {
    DIAGNOSTIC_SCHEMA_VERSION
}

/// Returns the component safety policy.
#[must_use]
pub const fn safety_policy() -> &'static str {
    SAFETY_POLICY
}

/// Returns the component scalability policy.
#[must_use]
pub const fn scalability_policy() -> &'static str {
    SCALABILITY_POLICY
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zhal::context::ZhalContext;

    // ------------------------------------------------------------------------
    // Origin validation
    // ------------------------------------------------------------------------

    #[test]
    fn origin_accepts_existing_zhal_components() {
        let origin = DiagnosticOrigin::new(
            "zhal.providers.discovery",
        )
        .expect("valid origin");

        assert_eq!(
            origin.as_str(),
            "zhal.providers.discovery",
        );
    }

    #[test]
    fn origin_accepts_future_hardware_domains() {
        let origin = DiagnosticOrigin::new(
            "future.photonic.provider",
        )
        .expect("future origin");

        assert_eq!(
            origin.as_str(),
            "future.photonic.provider",
        );
    }

    #[test]
    fn origin_rejects_empty_values() {
        assert_eq!(
            DiagnosticOrigin::new(""),
            Err(DiagnosticValidationError::EmptyOrigin),
        );
    }

    #[test]
    fn origin_rejects_whitespace() {
        assert_eq!(
            DiagnosticOrigin::new(
                "zhal hardware diagnostics",
            ),
            Err(
                DiagnosticValidationError::WhitespaceInOrigin,
            ),
        );
    }

    #[test]
    fn origin_rejects_control_characters() {
        assert_eq!(
            DiagnosticOrigin::new("zhal\nprovider"),
            Err(
                DiagnosticValidationError::ControlCharacterInOrigin,
            ),
        );
    }

    #[test]
    fn origin_has_no_arbitrary_length_ceiling() {
        let value = format!(
            "future.{}",
            "photonic.".repeat(1_000),
        );

        let origin = DiagnosticOrigin::new(
            value.clone(),
        )
        .expect("long valid origin");

        assert_eq!(origin.as_str(), value);
    }

    // ------------------------------------------------------------------------
    // Context snapshots
    // ------------------------------------------------------------------------

    #[test]
    fn empty_context_contains_no_references() {
        let context = DiagnosticContextSnapshot::empty();

        assert!(context.is_empty());
        assert_eq!(context.correlation_id(), None);
        assert_eq!(context.diagnostic_reference(), None);
        assert_eq!(context.provider_id(), None);
    }

    #[test]
    fn context_snapshot_preserves_correlation() {
        let context = ZhalContext::builder()
            .with_component("zhal")
            .expect("component")
            .with_operation("hardware.discovery")
            .expect("operation")
            .with_correlation_id("trace.request.001")
            .expect("correlation")
            .with_diagnostic_reference("diagnostic.001")
            .expect("diagnostic reference")
            .build();

        let snapshot = DiagnosticContextSnapshot::capture(
            &context,
        );

        assert_eq!(
            snapshot.correlation_id(),
            Some("trace.request.001"),
        );

        assert_eq!(
            snapshot.diagnostic_reference(),
            Some("diagnostic.001"),
        );
    }

    #[test]
    fn context_snapshot_does_not_modify_original_context() {
        let context = ZhalContext::builder()
            .with_component("zhal")
            .expect("component")
            .with_operation("hardware.discovery")
            .expect("operation")
            .with_correlation_id("trace.001")
            .expect("correlation")
            .build();

        let snapshot = DiagnosticContextSnapshot::capture(
            &context,
        );

        assert_eq!(
            context.correlation_id(),
            snapshot.correlation_id(),
        );
    }

    // ------------------------------------------------------------------------
    // Diagnostic construction
    // ------------------------------------------------------------------------

    #[test]
    fn diagnostic_preserves_canonical_error_classification() {
        let error = ZhalError::new(
            ZhalErrorKind::HardwareCommunication,
            "hardware communication failed",
        );

        let diagnostic = ZhalDiagnostic::from_error(
            "zhal.adapters.classical",
            &error,
            None,
        )
        .expect("diagnostic");

        assert_eq!(
            diagnostic.kind(),
            ZhalErrorKind::HardwareCommunication,
        );

        assert_eq!(
            diagnostic.message(),
            "hardware communication failed",
        );
    }

    #[test]
    fn diagnostic_has_stable_schema_identity() {
        let error = ZhalError::new(
            ZhalErrorKind::HardwareFault,
            "hardware fault observed",
        );

        let diagnostic = ZhalDiagnostic::from_error(
            "zhal.devices.health",
            &error,
            None,
        )
        .expect("diagnostic");

        assert_eq!(
            diagnostic.schema(),
            DIAGNOSTIC_SCHEMA,
        );

        assert_eq!(
            diagnostic.schema_version(),
            DIAGNOSTIC_SCHEMA_VERSION,
        );

        assert_eq!(
            diagnostic.component(),
            COMPONENT,
        );
    }

    #[test]
    fn diagnostic_preserves_unknown_operation_outcome() {
        let error = ZhalError::unknown_outcome(
            "remote operation outcome is unknown",
        );

        let diagnostic = ZhalDiagnostic::from_error(
            "zhal.adapters.distributed",
            &error,
            None,
        )
        .expect("diagnostic");

        assert!(diagnostic.outcome_is_unknown());

        assert_eq!(
            diagnostic.outcome(),
            ZhalOperationOutcome::Unknown,
        );
    }

    #[test]
    fn diagnostic_preserves_partial_completion() {
        let error = ZhalError::partial_completion(
            "hardware operation partially completed",
        );

        let diagnostic = ZhalDiagnostic::from_error(
            "zhal.operations.completion",
            &error,
            None,
        )
        .expect("diagnostic");

        assert!(diagnostic.is_partial_completion());
    }

    #[test]
    fn diagnostic_does_not_invent_retry_safety() {
        let error = ZhalError::new(
            ZhalErrorKind::HardwareCommunication,
            "communication failure",
        );

        let diagnostic = ZhalDiagnostic::from_error(
            "zhal.providers.discovery",
            &error,
            None,
        )
        .expect("diagnostic");

        assert!(!diagnostic.is_safe_to_retry());
    }

    #[test]
    fn diagnostic_preserves_explicit_retry_safety() {
        let error = ZhalError::new(
            ZhalErrorKind::ProviderUnavailable,
            "provider temporarily unavailable",
        )
        .with_retryability(
            ZhalRetryability::SafeToRetry,
        );

        let diagnostic = ZhalDiagnostic::from_error(
            "zhal.providers.lifecycle",
            &error,
            None,
        )
        .expect("diagnostic");

        assert!(diagnostic.is_safe_to_retry());
    }

    #[test]
    fn diagnostic_preserves_severity() {
        let error = ZhalError::new(
            ZhalErrorKind::HardwareFault,
            "hardware fault",
        )
        .with_severity(ZhalSeverity::Critical);

        let diagnostic = ZhalDiagnostic::from_error(
            "zhal.devices.health",
            &error,
            None,
        )
        .expect("diagnostic");

        assert_eq!(
            diagnostic.severity(),
            ZhalSeverity::Critical,
        );
    }

    // ------------------------------------------------------------------------
    // Security and redaction
    // ------------------------------------------------------------------------

    #[test]
    fn sensitive_error_context_is_redacted() {
        let mut error = ZhalError::new(
            ZhalErrorKind::HardwareCommunication,
            "hardware communication failed",
        );

        error
            .add_sensitive_context(
                "credential",
                "secret-value",
            )
            .expect("sensitive context");

        let diagnostic = ZhalDiagnostic::from_error(
            "zhal.providers.discovery",
            &error,
            None,
        )
        .expect("diagnostic");

        let entry = diagnostic.error
            .context
            .iter()
            .find(|entry| entry.key == "credential")
            .expect("context entry");

        assert_eq!(entry.value, "[REDACTED]");
    }

    #[test]
    fn diagnostic_does_not_copy_security_context_reference() {
        let context = ZhalContext::builder()
            .with_component("zhal")
            .expect("component")
            .with_operation("hardware.access")
            .expect("operation")
            .with_security_context_reference(
                "private.security.reference",
            )
            .expect("security reference")
            .build();

        let snapshot = DiagnosticContextSnapshot::capture(
            &context,
        );

        assert!(snapshot.is_empty());
    }

    // ------------------------------------------------------------------------
    // Scalability and integration
    // ------------------------------------------------------------------------

    #[test]
    fn diagnostic_origin_is_independent_of_hardware_domain() {
        for origin in [
            "zhal.adapters.classical",
            "zhal.adapters.quantum",
            "zhal.adapters.hdl",
            "zhal.adapters.accelerator",
            "zhal.adapters.distributed",
            "zhal.adapters.embedded",
            "future.photonic.provider",
            "future.neuromorphic.provider",
        ] {
            assert!(
                DiagnosticOrigin::new(origin).is_ok(),
            );
        }
    }

    #[test]
    fn diagnostic_component_policies_are_available() {
        assert_eq!(
            safety_policy(),
            "unsafe-rust-forbidden",
        );

        assert!(
            scalability_policy().contains("open-world"),
        );
    }

    #[test]
    fn diagnostic_without_context_remains_valid() {
        let error = ZhalError::new(
            ZhalErrorKind::DeviceNotFound,
            "requested device was not found",
        );

        let diagnostic = ZhalDiagnostic::from_error(
            "zhal.devices.discovery",
            &error,
            None,
        )
        .expect("diagnostic");

        assert!(diagnostic.context().is_empty());
    }
}

// ============================================================================
// END
// ============================================================================
