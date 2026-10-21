#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Backend error contracts for the Zamani compiler.
//!
//! Path: `src/backend/errors.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Define stable, structured errors for backend validation, target selection,
//! lowering, code generation, toolchain execution, and artifact production.
//!
//! # Architectural authority
//!
//! This module follows the ownership boundaries established by:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/compiler/diagnostics.rs`
//! - `src/compiler/mod.rs`
//! - `src/compiler_types.rs`
//! - `src/backend.rs`
//!
//! These files have different responsibilities. This module does not replace
//! the grammar, semantic model, compiler diagnostics, IR verifiers, or
//! domain-specific error contracts.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - Backend error categories and stable error codes.
//! - Structured error context and optional causal descriptions.
//! - Human-readable formatting.
//! - A standard result alias.
//! - A compatibility error type for the legacy `CodeGenError` API.
//!
//! # Non-goals
//!
//! This module does not own:
//!
//! - Source-language syntax or semantic validity.
//! - Source spans, AST nodes, or IR definitions.
//! - Target discovery, capability negotiation, or hardware allocation.
//! - Filesystem operations or external process execution.
//! - Diagnostic rendering, logging, retries, or recovery policy.
//! - Universal hardware capacity limits.
//!
//! # Error-handling invariants
//!
//! - Errors are data, not console output.
//! - Stable codes are independent of display messages.
//! - Target incompatibility is not source-language invalidity.
//! - Resource-budget exhaustion is not proof of invalid IR.
//! - Incomplete verification must never be treated as successful verification.
//! - An unsupported capability must not be silently substituted.
//! - Error context is descriptive; it must not change program semantics.
//!
//! # Integration
//!
//! Once `src/backend/mod.rs` becomes the canonical module root, it should
//! declare `pub mod errors;` and re-export the approved public types.
//!
//! The existing `src/backend.rs` defines a separate `CodeGenError` struct.
//! During migration, callers can retain that API through the compatibility
//! type in this module. The duplicate definition must be removed or replaced
//! by an explicit re-export when the legacy backend is migrated.
//!
//! This file deliberately imports no sibling modules, external crates,
//! target implementations, or compiler IR types.

use core::fmt;

/// Result type for backend operations.
pub type BackendResult<T> = Result<T, BackendError>;

/// Stable machine-readable backend error classification.
///
/// The code returned by [`BackendErrorKind::code`] is part of the public
/// diagnostic contract. Existing codes must not be reassigned to different
/// meanings. New variants and codes may be added as the backend evolves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum BackendErrorKind {
    /// An input artifact or request violates the backend input contract.
    InvalidInput,

    /// A backend or target configuration is invalid.
    InvalidConfiguration,

    /// The requested target is unknown or unavailable.
    UnsupportedTarget,

    /// A required target capability is unavailable.
    UnsupportedCapability,

    /// An input, target, ABI, or artifact does not match the requested
    /// compilation contract.
    TargetMismatch,

    /// Input IR or a generated representation failed verification.
    VerificationFailed,

    /// A required external toolchain or tool could not be located or
    /// validated.
    ToolchainUnavailable,

    /// An external tool failed while performing a backend operation.
    ToolExecutionFailed,

    /// An artifact is malformed, incomplete, or inconsistent with its
    /// declared format or metadata.
    InvalidArtifact,

    /// An explicit operational budget or available process resource was
    /// exhausted before the requested operation completed.
    ResourceExhausted,

    /// The operation was cancelled before completion.
    Cancelled,

    /// An input/output operation failed.
    Io,

    /// An unexpected backend invariant or implementation contract failed.
    Internal,

    /// A legacy error was adapted without a more specific classification.
    Legacy,
}

impl BackendErrorKind {
    /// Return the stable machine-readable error code.
    ///
    /// Codes identify categories, not individual error instances. Additional
    /// detail belongs in the error message and context.
    pub const fn code(self) -> &'static str {
        match self {
            Self::InvalidInput => "B0001",
            Self::InvalidConfiguration => "B0002",
            Self::UnsupportedTarget => "B0003",
            Self::UnsupportedCapability => "B0004",
            Self::TargetMismatch => "B0005",
            Self::VerificationFailed => "B0006",
            Self::ToolchainUnavailable => "B0007",
            Self::ToolExecutionFailed => "B0008",
            Self::InvalidArtifact => "B0009",
            Self::ResourceExhausted => "B0010",
            Self::Cancelled => "B0011",
            Self::Io => "B0012",
            Self::Internal => "B0013",
            Self::Legacy => "B0099",
        }
    }

    /// Return a stable, human-readable category name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidInput => "invalid backend input",
            Self::InvalidConfiguration => "invalid backend configuration",
            Self::UnsupportedTarget => "unsupported target",
            Self::UnsupportedCapability => "unsupported capability",
            Self::TargetMismatch => "target mismatch",
            Self::VerificationFailed => "verification failed",
            Self::ToolchainUnavailable => "toolchain unavailable",
            Self::ToolExecutionFailed => "tool execution failed",
            Self::InvalidArtifact => "invalid artifact",
            Self::ResourceExhausted => "resource budget exhausted",
            Self::Cancelled => "operation cancelled",
            Self::Io => "I/O failure",
            Self::Internal => "internal backend failure",
            Self::Legacy => "legacy backend error",
        }
    }
}

impl fmt::Display for BackendErrorKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A structured error raised by a backend operation.
///
/// `message` describes the immediate failure. `context` records additional
/// details in the order they are attached. `cause` optionally preserves a
/// textual description of an underlying failure when the underlying error
/// cannot be represented by a stable backend-specific type.
///
/// The structure intentionally does not require `Send + Sync` boxed dynamic
/// errors or any target-specific dependency. Adapters may retain the original
/// typed error separately when richer causal chaining is required.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendError {
    kind: BackendErrorKind,
    message: String,
    target: Option<String>,
    operation: Option<String>,
    context: Vec<String>,
    cause: Option<String>,
}

impl BackendError {
    /// Create an error with a classification and explanatory message.
    pub fn new(
        kind: BackendErrorKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            target: None,
            operation: None,
            context: Vec::new(),
            cause: None,
        }
    }

    /// Create an error associated with a target identifier or target name.
    pub fn for_target(
        kind: BackendErrorKind,
        target: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::new(kind, message).with_target(target)
    }

    /// Return the stable error classification.
    pub const fn kind(&self) -> BackendErrorKind {
        self.kind
    }

    /// Return the stable machine-readable error code.
    pub const fn code(&self) -> &'static str {
        self.kind.code()
    }

    /// Return the primary explanatory message.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Return the target identifier or name, if known.
    pub fn target(&self) -> Option<&str> {
        self.target.as_deref()
    }

    /// Return the operation identifier or name, if known.
    pub fn operation(&self) -> Option<&str> {
        self.operation.as_deref()
    }

    /// Return attached context in insertion order.
    pub fn context(&self) -> &[String] {
        &self.context
    }

    /// Return the optional underlying error description.
    pub fn cause(&self) -> Option<&str> {
        self.cause.as_deref()
    }

    /// Attach target information.
    pub fn with_target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }

    /// Attach operation information.
    pub fn with_operation(mut self, operation: impl Into<String>) -> Self {
        self.operation = Some(operation.into());
        self
    }

    /// Append descriptive context.
    ///
    /// Context should identify the relevant stage, artifact, or decision.
    /// Callers should not include credentials, secrets, or unnecessarily
    /// sensitive source contents.
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context.push(context.into());
        self
    }

    /// Attach a description of the underlying failure.
    ///
    /// This preserves useful causal information without requiring the
    /// underlying error type to be owned by the backend error module.
    pub fn with_cause(mut self, cause: impl Into<String>) -> Self {
        self.cause = Some(cause.into());
        self
    }

    /// Construct an invalid-input error.
    pub fn invalid_input(message: impl Into<String>) -> Self {
        Self::new(BackendErrorKind::InvalidInput, message)
    }

    /// Construct an invalid-configuration error.
    pub fn invalid_configuration(message: impl Into<String>) -> Self {
        Self::new(BackendErrorKind::InvalidConfiguration, message)
    }

    /// Construct an unsupported-target error.
    pub fn unsupported_target(
        target: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::for_target(BackendErrorKind::UnsupportedTarget, target, message)
    }

    /// Construct an unsupported-capability error.
    pub fn unsupported_capability(
        target: impl Into<String>,
        capability: impl Into<String>,
    ) -> Self {
        let capability = capability.into();
        Self::for_target(
            BackendErrorKind::UnsupportedCapability,
            target,
            format!("required capability `{capability}` is unavailable"),
        )
    }

    /// Construct a target-mismatch error.
    pub fn target_mismatch(
        target: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::for_target(BackendErrorKind::TargetMismatch, target, message)
    }

    /// Construct a verification error.
    pub fn verification_failed(message: impl Into<String>) -> Self {
        Self::new(BackendErrorKind::VerificationFailed, message)
    }

    /// Construct a toolchain-discovery or validation error.
    pub fn toolchain_unavailable(
        toolchain: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::new(BackendErrorKind::ToolchainUnavailable, message)
            .with_context(format!("toolchain: {}", toolchain.into()))
    }

    /// Construct a tool-execution error.
    pub fn tool_execution_failed(
        tool: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::new(BackendErrorKind::ToolExecutionFailed, message)
            .with_context(format!("tool: {}", tool.into()))
    }

    /// Construct an invalid-artifact error.
    pub fn invalid_artifact(message: impl Into<String>) -> Self {
        Self::new(BackendErrorKind::InvalidArtifact, message)
    }

    /// Construct an operational-budget or resource-exhaustion error.
    ///
    /// This error means the operation could not complete under the active
    /// resource policy. It does not establish that the program or IR is
    /// invalid, and it does not define a universal hardware-size ceiling.
    pub fn resource_exhausted(message: impl Into<String>) -> Self {
        Self::new(BackendErrorKind::ResourceExhausted, message)
    }

    /// Construct a cancellation error.
    pub fn cancelled(message: impl Into<String>) -> Self {
        Self::new(BackendErrorKind::Cancelled, message)
    }

    /// Construct an I/O error from a path-independent description.
    pub fn io(message: impl Into<String>) -> Self {
        Self::new(BackendErrorKind::Io, message)
    }

    /// Construct an internal-contract error.
    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(BackendErrorKind::Internal, message)
    }
}

impl fmt::Display for BackendError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code(), self.message)?;

        if let Some(target) = &self.target {
            write!(formatter, " [target: {target}]")?;
        }

        if let Some(operation) = &self.operation {
            write!(formatter, " [operation: {operation}]")?;
        }

        for context in &self.context {
            write!(formatter, "\n  context: {context}")?;
        }

        if let Some(cause) = &self.cause {
            write!(formatter, "\n  caused by: {cause}")?;
        }

        Ok(())
    }
}

impl std::error::Error for BackendError {}

/// Compatibility representation for the existing `src/backend.rs` API.
///
/// The current legacy backend exposes `CodeGenError { message, target }`
/// with public fields and a `new(message, target)` constructor. Retaining this
/// shape allows existing callers to migrate incrementally.
///
/// This compatibility type should eventually be replaced by a re-export or
/// conversion at the legacy backend boundary. New backend APIs should use
/// [`BackendError`] and [`BackendResult`] instead.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeGenError {
    /// Human-readable error message retained for existing callers.
    pub message: String,

    /// Target identifier or name retained for existing callers.
    pub target: String,
}

impl CodeGenError {
    /// Construct a legacy-compatible code-generation error.
    pub fn new(
        message: impl Into<String>,
        target: impl Into<String>,
    ) -> Self {
        Self {
            message: message.into(),
            target: target.into(),
        }
    }

    /// Adapt a structured backend error to the legacy representation.
    ///
    /// The legacy type cannot retain structured categories, context, or
    /// causal information. Callers that need those fields should retain the
    /// original [`BackendError`] instead of converting it.
    pub fn from_backend_error(error: BackendError) -> Self {
        let target = error.target.clone().unwrap_or_default();
        Self {
            message: error.to_string(),
            target,
        }
    }
}

impl From<BackendError> for CodeGenError {
    fn from(error: BackendError) -> Self {
        Self::from_backend_error(error)
    }
}

impl fmt::Display for CodeGenError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.target.is_empty() {
            formatter.write_str(&self.message)
        } else {
            write!(formatter, "{} [target: {}]", self.message, self.target)
        }
    }
}

impl std::error::Error for CodeGenError {}

#[cfg(test)]
mod tests {
    use super::{
        BackendError, BackendErrorKind, BackendResult, CodeGenError,
    };

    #[test]
    fn error_codes_are_stable_and_distinct() {
        let kinds = [
            BackendErrorKind::InvalidInput,
            BackendErrorKind::InvalidConfiguration,
            BackendErrorKind::UnsupportedTarget,
            BackendErrorKind::UnsupportedCapability,
            BackendErrorKind::TargetMismatch,
            BackendErrorKind::VerificationFailed,
            BackendErrorKind::ToolchainUnavailable,
            BackendErrorKind::ToolExecutionFailed,
            BackendErrorKind::InvalidArtifact,
            BackendErrorKind::ResourceExhausted,
            BackendErrorKind::Cancelled,
            BackendErrorKind::Io,
            BackendErrorKind::Internal,
            BackendErrorKind::Legacy,
        ];

        for (index, kind) in kinds.iter().enumerate() {
            assert!(kind.code().starts_with('B'));
            assert!(kinds[..index].iter().all(|previous| {
                previous.code() != kind.code()
            }));
        }
    }

    #[test]
    fn error_context_is_preserved_in_order() {
        let error = BackendError::invalid_input("invalid input")
            .with_target("example-target")
            .with_operation("lower")
            .with_context("validation stage")
            .with_context("input artifact")
            .with_cause("representation was incomplete");

        assert_eq!(error.kind(), BackendErrorKind::InvalidInput);
        assert_eq!(error.code(), "B0001");
        assert_eq!(error.target(), Some("example-target"));
        assert_eq!(error.operation(), Some("lower"));
        assert_eq!(
            error.context(),
            &[
                "validation stage".to_owned(),
                "input artifact".to_owned(),
            ]
        );
        assert_eq!(
            error.cause(),
            Some("representation was incomplete")
        );

        let rendered = error.to_string();
        assert!(rendered.contains("B0001"));
        assert!(rendered.contains("example-target"));
        assert!(rendered.contains("validation stage"));
        assert!(rendered.contains("representation was incomplete"));
    }

    #[test]
    fn unsupported_capability_is_not_invalid_source() {
        let error = BackendError::unsupported_capability(
            "target-a",
            "quantum.measurement",
        );

        assert_eq!(
            error.kind(),
            BackendErrorKind::UnsupportedCapability
        );
        assert_eq!(error.target(), Some("target-a"));
        assert!(error.message().contains("quantum.measurement"));
    }

    #[test]
    fn resource_exhaustion_is_distinct_from_verification_failure() {
        let exhausted = BackendError::resource_exhausted(
            "configured compilation-work budget exhausted",
        );
        let invalid = BackendError::verification_failed(
            "input representation violates an invariant",
        );

        assert_ne!(exhausted.kind(), invalid.kind());
        assert_eq!(
            exhausted.kind(),
            BackendErrorKind::ResourceExhausted
        );
        assert_eq!(
            invalid.kind(),
            BackendErrorKind::VerificationFailed
        );
    }

    #[test]
    fn legacy_error_conversion_preserves_target_and_message() {
        let structured = BackendError::unsupported_target(
            "aarch64",
            "no implementation is registered",
        );

        let legacy = CodeGenError::from(structured);

        assert_eq!(legacy.target, "aarch64");
        assert!(legacy.message.contains("B0003"));
        assert!(legacy.message.contains("no implementation is registered"));
    }

    #[test]
    fn legacy_constructor_remains_compatible() {
        let error = CodeGenError::new("generation failed", "llvm-ir");

        assert_eq!(error.message, "generation failed");
        assert_eq!(error.target, "llvm-ir");
        assert_eq!(
            error.to_string(),
            "generation failed [target: llvm-ir]"
        );
    }

    #[test]
    fn result_alias_accepts_structured_errors() {
        fn operation(fail: bool) -> BackendResult<()> {
            if fail {
                Err(BackendError::invalid_input("invalid test input"))
            } else {
                Ok(())
            }
        }

        assert!(operation(false).is_ok());
        assert_eq!(
            operation(true).expect_err("failure expected").kind(),
            BackendErrorKind::InvalidInput
        );
    }
}