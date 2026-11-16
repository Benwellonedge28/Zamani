#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Toolchain Errors
//!
//! Path: `src/toolchain/error.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! This module defines the shared error contract for the Zamani toolchain.
//! It provides structured, machine-readable failures for project management,
//! workspace discovery, manifests, dependency resolution, build orchestration,
//! packages, registries, providers, external tools, and deployment.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Toolchain-level error categories.
//! - Stable toolchain error codes.
//! - The `ToolchainError` value.
//! - Structured operation, subject, and contextual information.
//! - Causal error preservation.
//! - Toolchain result aliases.
//! - Conversion from standard I/O errors.
//!
//! It does not own:
//!
//! - Source-language or semantic diagnostics.
//! - Compiler error definitions.
//! - Backend error definitions.
//! - External process execution.
//! - External executable discovery.
//! - Hardware capability or resource models.
//! - Retry, fallback, recovery, or scheduling policy.
//! - Diagnostic rendering or logging.
//!
//! ## Integration authorities
//!
//! - `crate::backend::errors` owns backend error classification.
//! - `crate::backend::toolchain` owns external tool discovery and validation.
//! - `crate::backend::process` owns external process execution.
//! - `crate::compiler` owns compiler error contracts.
//! - `crate::zamani_project_config` owns project configuration errors.
//! - `crate::frontend` owns source-language diagnostics.
//!
//! Adapters should preserve these original errors as causal sources instead
//! of translating them into misleading toolchain-specific error categories.
//!
//! ## Scalability
//!
//! This module introduces no universal limits on:
//!
//! - Project or workspace size.
//! - Number of packages or dependencies.
//! - Number of targets or providers.
//! - Build-task or graph size.
//! - Processor, accelerator, or quantum resource counts.
//! - Memory, storage, tensor dimensions, or distributed participants.
//!
//! Operational budgets are owned by the relevant configuration and execution
//! contracts. An exhausted operational budget must be reported as such; it
//! must not be presented as a language or hardware capacity ceiling.
//!
//! ## Error-handling guarantees
//!
//! - Errors are structured data, not console output.
//! - Error codes identify stable categories, not individual occurrences.
//! - Display messages are not machine-readable identifiers.
//! - Context is additive and does not change error classification.
//! - Original causal errors can be retained and inspected.
//! - Cancellation is distinct from failure and resource exhaustion.
//! - Unknown or unsupported capabilities must not be treated as available.
//! - Errors must not claim that an operation succeeded when it did not.

use core::fmt;
use std::error::Error;
use std::io;
use std::sync::Arc;

/// Standard result type for toolchain operations.
pub type Result<T> = core::result::Result<T, ToolchainError>;

/// Explicitly named result type for APIs that prefer an unambiguous alias.
pub type ToolchainResult<T> = Result<T>;

/// Stable classification of a toolchain failure.
///
/// The string returned by [`ToolchainErrorKind::code`] is part of the
/// toolchain's public diagnostic contract. Existing codes must never be
/// reassigned to different meanings.
///
/// This enum is non-exhaustive so future toolchain capabilities and
/// computational domains can add classifications without requiring every
/// downstream consumer to be updated immediately.
///
/// These categories describe failures at the toolchain boundary. More
/// specific compiler, backend, frontend, runtime, and provider errors remain
/// owned by their respective subsystems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ToolchainErrorKind {
    /// An operation received malformed or otherwise invalid input.
    InvalidInput,

    /// A toolchain configuration is invalid or internally inconsistent.
    InvalidConfiguration,

    /// A required project, workspace, package, or resource was not found.
    NotFound,

    /// An operation attempted to create an identity or resource that already
    /// exists where uniqueness is required.
    AlreadyExists,

    /// A supplied manifest, project, package, or request failed validation.
    ValidationFailed,

    /// The requested operation is not supported by the selected service or
    /// registered implementation.
    UnsupportedOperation,

    /// A requested version, format, API, ABI, or contract is incompatible.
    IncompatibleVersion,

    /// Dependency resolution could not produce a valid dependency solution.
    DependencyResolutionFailed,

    /// A build orchestration operation failed.
    BuildFailed,

    /// A package creation, extraction, installation, or inspection operation
    /// failed.
    PackageOperationFailed,

    /// A registry operation failed.
    RegistryOperationFailed,

    /// Authentication or authorization required by a toolchain operation
    /// failed.
    AuthenticationFailed,

    /// An integrity, signature, provenance, or trust verification failed.
    IntegrityVerificationFailed,

    /// A required provider is unavailable or failed to satisfy its declared
    /// provider contract.
    ProviderFailed,

    /// An external tool is unavailable, incompatible, or failed during an
    /// operation coordinated by the toolchain.
    ExternalToolFailed,

    /// A configured operational budget or available implementation resource
    /// was exhausted.
    ResourceExhausted,

    /// An operation was cancelled before completion.
    Cancelled,

    /// An input/output operation failed.
    Io,

    /// An unexpected toolchain invariant or implementation contract failed.
    Internal,

    /// An older or less-specific error was adapted into the current contract.
    Legacy,
}

impl ToolchainErrorKind {
    /// Return the stable machine-readable code for this error category.
    ///
    /// Codes are permanent identifiers. Do not change an existing code's
    /// meaning when adding new categories.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::InvalidInput => "ZT0001",
            Self::InvalidConfiguration => "ZT0002",
            Self::NotFound => "ZT0003",
            Self::AlreadyExists => "ZT0004",
            Self::ValidationFailed => "ZT0005",
            Self::UnsupportedOperation => "ZT0006",
            Self::IncompatibleVersion => "ZT0007",
            Self::DependencyResolutionFailed => "ZT0008",
            Self::BuildFailed => "ZT0009",
            Self::PackageOperationFailed => "ZT0010",
            Self::RegistryOperationFailed => "ZT0011",
            Self::AuthenticationFailed => "ZT0012",
            Self::IntegrityVerificationFailed => "ZT0013",
            Self::ProviderFailed => "ZT0014",
            Self::ExternalToolFailed => "ZT0015",
            Self::ResourceExhausted => "ZT0016",
            Self::Cancelled => "ZT0017",
            Self::Io => "ZT0018",
            Self::Internal => "ZT0019",
            Self::Legacy => "ZT0099",
        }
    }

    /// Return a stable descriptive category name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidInput => "invalid toolchain input",
            Self::InvalidConfiguration => "invalid toolchain configuration",
            Self::NotFound => "resource not found",
            Self::AlreadyExists => "resource already exists",
            Self::ValidationFailed => "validation failed",
            Self::UnsupportedOperation => "unsupported operation",
            Self::IncompatibleVersion => "incompatible version or contract",
            Self::DependencyResolutionFailed => "dependency resolution failed",
            Self::BuildFailed => "build failed",
            Self::PackageOperationFailed => "package operation failed",
            Self::RegistryOperationFailed => "registry operation failed",
            Self::AuthenticationFailed => "authentication or authorization failed",
            Self::IntegrityVerificationFailed => "integrity verification failed",
            Self::ProviderFailed => "provider operation failed",
            Self::ExternalToolFailed => "external tool operation failed",
            Self::ResourceExhausted => "operational resource exhausted",
            Self::Cancelled => "operation cancelled",
            Self::Io => "input/output failure",
            Self::Internal => "internal toolchain failure",
            Self::Legacy => "legacy toolchain failure",
        }
    }
}

impl fmt::Display for ToolchainErrorKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// An owned causal error.
///
/// `Arc` allows a `ToolchainError` to be cloned without discarding the
/// original source error. The source remains available through the standard
/// [`Error::source`] interface.
///
/// The wrapped source must be `Send + Sync + 'static`, making the error
/// suitable for sharing across ordinary thread-safe toolchain boundaries.
type SharedErrorSource = Arc<dyn Error + Send + Sync + 'static>;

/// A structured error raised by the Zamani toolchain.
///
/// The error stores:
///
/// - A stable category.
/// - A human-readable explanation.
/// - An optional operation identifier.
/// - An optional subject identifier.
/// - Additional context in insertion order.
/// - An optional original causal error.
///
/// `operation` identifies the activity that failed, for example
/// `"dependency.resolve"` or `"package.extract"`.
///
/// `subject` identifies the relevant project, package, task, registry,
/// provider, artifact, or other entity. It is descriptive text and does not
/// replace canonical typed identities owned by other modules.
///
/// Context must not contain credentials, access tokens, private keys,
/// unredacted secrets, or unnecessarily sensitive source contents.
#[derive(Clone)]
pub struct ToolchainError {
    kind: ToolchainErrorKind,
    message: String,
    operation: Option<String>,
    subject: Option<String>,
    context: Vec<String>,
    source: Option<SharedErrorSource>,
}

impl ToolchainError {
    /// Create a toolchain error with a category and explanatory message.
    #[must_use]
    pub fn new(
        kind: ToolchainErrorKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            operation: None,
            subject: None,
            context: Vec::new(),
            source: None,
        }
    }

    /// Create an error associated with a named operation.
    #[must_use]
    pub fn for_operation(
        kind: ToolchainErrorKind,
        operation: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::new(kind, message).with_operation(operation)
    }

    /// Create an error associated with a subject.
    ///
    /// A subject may identify a project, package, task, registry, provider,
    /// artifact, or another toolchain entity.
    #[must_use]
    pub fn for_subject(
        kind: ToolchainErrorKind,
        subject: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::new(kind, message).with_subject(subject)
    }

    /// Return the error category.
    #[must_use]
    pub const fn kind(&self) -> ToolchainErrorKind {
        self.kind
    }

    /// Return the stable machine-readable error code.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        self.kind.code()
    }

    /// Return the stable category description.
    #[must_use]
    pub const fn category(&self) -> &'static str {
        self.kind.as_str()
    }

    /// Return the primary explanatory message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Return the operation identifier, if one was attached.
    #[must_use]
    pub fn operation(&self) -> Option<&str> {
        self.operation.as_deref()
    }

    /// Return the subject identifier, if one was attached.
    #[must_use]
    pub fn subject(&self) -> Option<&str> {
        self.subject.as_deref()
    }

    /// Return contextual information in insertion order.
    #[must_use]
    pub fn context(&self) -> &[String] {
        &self.context
    }

    /// Return whether an original causal error is retained.
    #[must_use]
    pub fn has_source(&self) -> bool {
        self.source.is_some()
    }

    /// Attach an operation identifier.
    ///
    /// This method does not change the error category or stable code.
    #[must_use]
    pub fn with_operation(
        mut self,
        operation: impl Into<String>,
    ) -> Self {
        self.operation = Some(operation.into());
        self
    }

    /// Attach a subject identifier or descriptive name.
    ///
    /// This method does not create or replace a canonical project, package,
    /// task, registry, provider, or artifact identity.
    #[must_use]
    pub fn with_subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    /// Append descriptive context.
    ///
    /// Context is retained in insertion order. It should describe the
    /// relevant stage, decision, or boundary without leaking secrets.
    #[must_use]
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context.push(context.into());
        self
    }

    /// Preserve an original typed causal error.
    ///
    /// This is the preferred integration mechanism for errors owned by
    /// compiler, backend, manifest, registry, provider, or runtime modules.
    /// The toolchain records the cause without taking ownership of its
    /// classification or replacing its public API.
    #[must_use]
    pub fn with_source<E>(mut self, source: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        self.source = Some(Arc::new(source));
        self
    }

    /// Construct an invalid-input error.
    #[must_use]
    pub fn invalid_input(message: impl Into<String>) -> Self {
        Self::new(ToolchainErrorKind::InvalidInput, message)
    }

    /// Construct an invalid-configuration error.
    #[must_use]
    pub fn invalid_configuration(message: impl Into<String>) -> Self {
        Self::new(ToolchainErrorKind::InvalidConfiguration, message)
    }

    /// Construct a not-found error.
    #[must_use]
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(ToolchainErrorKind::NotFound, message)
    }

    /// Construct a validation error.
    #[must_use]
    pub fn validation_failed(message: impl Into<String>) -> Self {
        Self::new(ToolchainErrorKind::ValidationFailed, message)
    }

    /// Construct an unsupported-operation error.
    #[must_use]
    pub fn unsupported_operation(message: impl Into<String>) -> Self {
        Self::new(ToolchainErrorKind::UnsupportedOperation, message)
    }

    /// Construct a compatibility error.
    #[must_use]
    pub fn incompatible_version(message: impl Into<String>) -> Self {
        Self::new(ToolchainErrorKind::IncompatibleVersion, message)
    }

    /// Construct a dependency-resolution error.
    #[must_use]
    pub fn dependency_resolution_failed(
        message: impl Into<String>,
    ) -> Self {
        Self::new(ToolchainErrorKind::DependencyResolutionFailed, message)
    }

    /// Construct a build error.
    #[must_use]
    pub fn build_failed(message: impl Into<String>) -> Self {
        Self::new(ToolchainErrorKind::BuildFailed, message)
    }

    /// Construct a package-operation error.
    #[must_use]
    pub fn package_operation_failed(
        message: impl Into<String>,
    ) -> Self {
        Self::new(ToolchainErrorKind::PackageOperationFailed, message)
    }

    /// Construct a registry-operation error.
    #[must_use]
    pub fn registry_operation_failed(
        message: impl Into<String>,
    ) -> Self {
        Self::new(ToolchainErrorKind::RegistryOperationFailed, message)
    }

    /// Construct an authentication or authorization error.
    #[must_use]
    pub fn authentication_failed(message: impl Into<String>) -> Self {
        Self::new(ToolchainErrorKind::AuthenticationFailed, message)
    }

    /// Construct an integrity-verification error.
    #[must_use]
    pub fn integrity_verification_failed(
        message: impl Into<String>,
    ) -> Self {
        Self::new(ToolchainErrorKind::IntegrityVerificationFailed, message)
    }

    /// Construct a provider-operation error.
    #[must_use]
    pub fn provider_failed(message: impl Into<String>) -> Self {
        Self::new(ToolchainErrorKind::ProviderFailed, message)
    }

    /// Construct an external-tool error.
    #[must_use]
    pub fn external_tool_failed(message: impl Into<String>) -> Self {
        Self::new(ToolchainErrorKind::ExternalToolFailed, message)
    }

    /// Construct an operational resource-exhaustion error.
    ///
    /// This category describes an actual operational budget or available
    /// implementation resource being exhausted. It must not be used to
    /// represent a universal language or hardware capacity ceiling.
    #[must_use]
    pub fn resource_exhausted(message: impl Into<String>) -> Self {
        Self::new(ToolchainErrorKind::ResourceExhausted, message)
    }

    /// Construct a cancellation error.
    #[must_use]
    pub fn cancelled(message: impl Into<String>) -> Self {
        Self::new(ToolchainErrorKind::Cancelled, message)
    }

    /// Construct an internal-invariant error.
    #[must_use]
    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(ToolchainErrorKind::Internal, message)
    }

    /// Construct an I/O error while preserving its original cause.
    #[must_use]
    pub fn from_io(
        operation: impl Into<String>,
        source: io::Error,
    ) -> Self {
        let message = source.to_string();

        Self::new(ToolchainErrorKind::Io, message)
            .with_operation(operation)
            .with_source(source)
    }
}

impl fmt::Debug for ToolchainError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = formatter.debug_struct("ToolchainError");

        debug
            .field("kind", &self.kind)
            .field("code", &self.code())
            .field("message", &self.message)
            .field("operation", &self.operation)
            .field("subject", &self.subject)
            .field("context", &self.context);

        if let Some(source) = &self.source {
            debug.field("source", source);
        }

        debug.finish()
    }
}

impl fmt::Display for ToolchainError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} [{}]: {}",
            self.category(),
            self.code(),
            self.message
        )?;

        if let Some(operation) = &self.operation {
            write!(formatter, " (operation: {operation})")?;
        }

        if let Some(subject) = &self.subject {
            write!(formatter, " (subject: {subject})")?;
        }

        for context in &self.context {
            write!(formatter, ": {context}")?;
        }

        Ok(())
    }
}

impl Error for ToolchainError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| source.as_ref() as &(dyn Error + 'static))
    }
}

impl From<io::Error> for ToolchainError {
    fn from(source: io::Error) -> Self {
        Self::from_io("filesystem", source)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Result, ToolchainError, ToolchainErrorKind, ToolchainResult,
    };
    use core::fmt;
    use std::error::Error;
    use std::io;

    #[derive(Debug)]
    struct TestSourceError;

    impl fmt::Display for TestSourceError {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("underlying test failure")
        }
    }

    impl Error for TestSourceError {}

    #[test]
    fn error_codes_are_stable_and_distinct() {
        let kinds = [
            ToolchainErrorKind::InvalidInput,
            ToolchainErrorKind::InvalidConfiguration,
            ToolchainErrorKind::NotFound,
            ToolchainErrorKind::AlreadyExists,
            ToolchainErrorKind::ValidationFailed,
            ToolchainErrorKind::UnsupportedOperation,
            ToolchainErrorKind::IncompatibleVersion,
            ToolchainErrorKind::DependencyResolutionFailed,
            ToolchainErrorKind::BuildFailed,
            ToolchainErrorKind::PackageOperationFailed,
            ToolchainErrorKind::RegistryOperationFailed,
            ToolchainErrorKind::AuthenticationFailed,
            ToolchainErrorKind::IntegrityVerificationFailed,
            ToolchainErrorKind::ProviderFailed,
            ToolchainErrorKind::ExternalToolFailed,
            ToolchainErrorKind::ResourceExhausted,
            ToolchainErrorKind::Cancelled,
            ToolchainErrorKind::Io,
            ToolchainErrorKind::Internal,
            ToolchainErrorKind::Legacy,
        ];

        for (index, kind) in kinds.iter().enumerate() {
            assert!(!kind.code().is_empty());
            assert!(!kind.as_str().is_empty());

            for other in kinds.iter().skip(index + 1) {
                assert_ne!(kind.code(), other.code());
            }
        }
    }

    #[test]
    fn error_exposes_structured_information() {
        let error = ToolchainError::build_failed("compiler invocation failed")
            .with_operation("build.execute")
            .with_subject("project:example")
            .with_context("compilation stage")
            .with_context("target realization");

        assert_eq!(error.kind(), ToolchainErrorKind::BuildFailed);
        assert_eq!(error.code(), "ZT0009");
        assert_eq!(error.message(), "compiler invocation failed");
        assert_eq!(error.operation(), Some("build.execute"));
        assert_eq!(error.subject(), Some("project:example"));
        assert_eq!(
            error.context(),
            &[
                "compilation stage".to_owned(),
                "target realization".to_owned()
            ]
        );
    }

    #[test]
    fn attaching_context_does_not_change_error_identity() {
        let original = ToolchainError::invalid_input("invalid request");
        let enriched = original
            .clone()
            .with_context("request validation")
            .with_operation("api.submit");

        assert_eq!(original.kind(), enriched.kind());
        assert_eq!(original.code(), enriched.code());
        assert_eq!(original.message(), enriched.message());
    }

    #[test]
    fn original_causal_error_is_preserved() {
        let error = ToolchainError::provider_failed("provider operation failed")
            .with_source(TestSourceError);

        assert!(error.has_source());

        let source = Error::source(&error)
            .expect("the original source error must be available");

        assert_eq!(source.to_string(), "underlying test failure");
    }

    #[test]
    fn cloning_preserves_the_causal_error() {
        let original = ToolchainError::provider_failed("provider failed")
            .with_source(TestSourceError);

        let cloned = original.clone();

        assert!(cloned.has_source());
        assert_eq!(
            Error::source(&cloned)
                .expect("cloned error must preserve its source")
                .to_string(),
            "underlying test failure"
        );
    }

    #[test]
    fn io_conversion_preserves_the_source() {
        let error = ToolchainError::from(io::Error::new(
            io::ErrorKind::NotFound,
            "file does not exist",
        ));

        assert_eq!(error.kind(), ToolchainErrorKind::Io);
        assert_eq!(error.operation(), Some("filesystem"));
        assert!(error.has_source());
        assert!(Error::source(&error).is_some());
    }

    #[test]
    fn display_includes_code_and_context() {
        let error = ToolchainError::not_found("manifest not found")
            .with_operation("manifest.load")
            .with_subject("project:sample")
            .with_context("workspace discovery");

        let rendered = error.to_string();

        assert!(rendered.contains("ZT0003"));
        assert!(rendered.contains("manifest not found"));
        assert!(rendered.contains("manifest.load"));
        assert!(rendered.contains("project:sample"));
        assert!(rendered.contains("workspace discovery"));
    }

    #[test]
    fn result_aliases_use_toolchain_error() {
        fn accepts_result(
            result: Result<()>,
        ) -> ToolchainResult<()> {
            result
        }

        let result: Result<()> = Err(ToolchainError::cancelled("cancelled"));

        assert_eq!(
            accepts_result(result)
                .expect_err("the test result must remain an error")
                .kind(),
            ToolchainErrorKind::Cancelled
        );
    }

    #[test]
    fn resource_exhaustion_is_distinct_from_invalid_input() {
        let exhausted = ToolchainError::resource_exhausted("build budget exhausted");
        let invalid = ToolchainError::invalid_input("invalid build request");

        assert_ne!(exhausted.kind(), invalid.kind());
        assert_ne!(exhausted.code(), invalid.code());
    }

    #[test]
    fn cancellation_is_distinct_from_failure() {
        let cancelled = ToolchainError::cancelled("operation cancelled");
        let failed = ToolchainError::build_failed("build failed");

        assert_ne!(cancelled.kind(), failed.kind());
        assert_ne!(cancelled.code(), failed.code());
    }
}