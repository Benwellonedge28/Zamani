
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR — Canonical Error Contract
//!
//! Path: `src/zuir/errors.rs`
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//!
//! ## PURPOSE
//!
//! Defines the stable, structured error vocabulary used by Zamani's
//! Universal Intermediate Representation (ZUIR).
//!
//! This module provides errors that can be propagated across IR construction,
//! verification, dialect registration, transformations, serialization,
//! version compatibility, interchange, and lowering.
//!
//! It is independent of concrete IR nodes, builders, verifier implementations,
//! diagnostic renderers, source files, and execution backends.
//!
//! ## OWNS
//!
//! - Canonical ZUIR error categories.
//! - Stable, machine-readable error codes.
//! - Typed references to affected IR entities.
//! - Optional phase, artifact-path, version, and contextual information.
//! - Standard Rust error chaining.
//! - The `ZuirResult<T>` convenience alias.
//!
//! ## DOES NOT OWN
//!
//! - Source-language syntax or semantic rules.
//! - Diagnostic severity, rendering, source spans, or editor integration.
//! - IR validation algorithms.
//! - Identifier allocation or identity definitions.
//! - Version definitions or compatibility policy.
//! - Serialization formats or migration implementations.
//! - Target capability discovery or physical resource allocation.
//! - Runtime recovery, retries, or execution policy.
//!
//! ## AUTHORITY
//!
//! This module must conform to:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/diagnostics.md`
//! - `grammar/specification/semantic-model.md`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/version.rs`
//!
//! ## DEPENDENCY CONTRACT
//!
//! This module may depend on `ids.rs` and `version.rs` because both define
//! foundational contracts. Those modules must not depend on this module.
//!
//! It must not depend on builders, verification, dialect registries,
//! diagnostic implementations, serialization, or lowering.
//!
//! ## ERROR-HANDLING CONTRACT
//!
//! Errors identify a failed operation or unmet contract. They do not, by
//! themselves, decide whether a diagnostic is a warning, whether compilation
//! can recover, or whether a fallback target should be selected.
//!
//! Human-readable messages are explanatory, not stable machine interfaces.
//! Consumers must branch on `ZuirErrorKind` or inspect `ZuirErrorCode`.
//!
//! Error codes are stable identifiers. Once published, their meaning must
//! not be silently changed. New categories and codes may be added without
//! requiring every consumer to understand them.
//!
//! Unknown external operations, unsupported versions, and missing required
//! capabilities must never be silently accepted.
//!
//! ## SCALABILITY CONTRACT
//!
//! This module imposes no hardware, operation-count, qubit-count, tensor-rank,
//! topology, or device-count ceiling.
//!
//! Context collections grow dynamically. Allocation can still fail because
//! the process and its environment have finite resources; this module does
//! not promise literal infinity or recover from every process-level
//! allocation failure.
//!
//! Any operational budget belongs to the caller's explicit policy. A budget
//! failure must not be misreported as proof that the IR is invalid.
//!
//! ## FREEZE CRITERIA
//!
//! - Error categories and default codes are stable and documented.
//! - Typed subjects reuse `ids.rs`.
//! - Version context reuses `version.rs`.
//! - Underlying errors remain available through standard error chaining.
//! - Diagnostic rendering and source locations remain separately owned.
//! - Tests cover codes, context, chaining, and extensibility.
//! - No hardware-specific assumptions or unsafe code are introduced.

use core::fmt;
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;

use crate::zuir::ids::{
    BlockId, DialectId, ExtensionId, FunctionId, ModuleId, OperationId, ProgramId, RegionId,
    SourceId, SymbolId, TypeId, ValueId,
};
use crate::zuir::version::{
    DialectVersion, LanguageVersion, ZuirFormatVersion, ZuirSchemaVersion,
};

// =============================================================================
// Result alias
// =============================================================================

/// The standard result type for fallible ZUIR operations.
///
/// Use this alias for errors that belong to the ZUIR contract. Subsystems may
/// retain more specific error types internally and convert them at their
/// public ZUIR boundary.
pub type ZuirResult<T> = Result<T, ZuirError>;

// =============================================================================
// Error categories
// =============================================================================

/// Stable broad category for a ZUIR failure.
///
/// This enum is intentionally non-exhaustive so new categories can be added
/// without requiring every downstream consumer to enumerate every variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum ZuirErrorKind {
    /// An input or argument does not satisfy the required contract.
    InvalidInput,

    /// The supplied representation is malformed or internally inconsistent.
    MalformedRepresentation,

    /// An identifier or identifier reference is invalid.
    InvalidIdentifier,

    /// Two declarations conflict in a namespace where uniqueness is required.
    DuplicateDefinition,

    /// A required symbol, value, type, or entity reference cannot be resolved.
    UnresolvedReference,

    /// A type declaration, assignment, conversion, or type relationship is
    /// invalid under the applicable type contract.
    InvalidType,

    /// Control flow or a control-flow edge violates the IR contract.
    InvalidControlFlow,

    /// A value is used where its definition does not dominate the use,
    /// when the selected IR model requires dominance.
    DominanceViolation,

    /// An operation's declared or inferred effects violate its contract.
    InvalidEffect,

    /// Ownership, lifetime, move, borrowing, linearity, or affine rules fail.
    OwnershipViolation,

    /// A symbolic resource requirement or constraint is invalid or unsatisfied.
    ResourceConstraintViolation,

    /// A requested operation could not complete because a configured
    /// operational budget or available process resource was exhausted.
    ResourceExhausted,

    /// A required capability is absent or cannot be established.
    CapabilityUnavailable,

    /// A required dialect is not registered or cannot be resolved.
    UnknownDialect,

    /// A dialect declaration or its manifest violates the registry contract.
    InvalidDialectDefinition,

    /// An operation is unknown or unsupported in the selected context.
    UnsupportedOperation,

    /// An attribute is invalid or unsupported in the selected context.
    UnsupportedAttribute,

    /// A representation version is not supported by the selected policy.
    UnsupportedVersion,

    /// A representation requires an explicit migration before interpretation.
    MigrationRequired,

    /// Encoding or writing a representation failed.
    SerializationFailure,

    /// Decoding or reading a representation failed.
    DeserializationFailure,

    /// A requested verification could not establish the required invariants.
    VerificationFailed,

    /// A transformation failed to complete under its declared contract.
    TransformationFailed,

    /// A lowering path cannot preserve or implement the required semantics.
    LoweringUnsupported,

    /// A selected target cannot satisfy the required semantic contract.
    TargetIncompatible,

    /// Configuration is missing, invalid, or contradictory.
    ConfigurationInvalid,

    /// An extension failed to meet its registration or execution contract.
    ExtensionFailure,

    /// The operation was explicitly cancelled.
    Cancelled,

    /// An external I/O operation failed.
    IoFailure,

    /// An internal invariant or integration contract failed.
    InternalFailure,

    /// An explicitly configured operational limit was reached.
    LimitExceeded,

    /// An operation conflicts with an existing state or declaration.
    Conflict,

    /// A requested entity or required resource was not found.
    NotFound,
}

impl ZuirErrorKind {
    /// Returns the canonical default code for this error category.
    ///
    /// These codes are compatibility identifiers. Do not change the meaning
    /// of an existing code after it has been published.
    #[must_use]
    pub const fn default_code(self) -> &'static str {
        match self {
            Self::InvalidInput => "ZMN-IR-INVALID-INPUT",
            Self::MalformedRepresentation => "ZMN-IR-MALFORMED-REPRESENTATION",
            Self::InvalidIdentifier => "ZMN-IR-INVALID-IDENTIFIER",
            Self::DuplicateDefinition => "ZMN-IR-DUPLICATE-DEFINITION",
            Self::UnresolvedReference => "ZMN-IR-UNRESOLVED-REFERENCE",
            Self::InvalidType => "ZMN-IR-INVALID-TYPE",
            Self::InvalidControlFlow => "ZMN-IR-INVALID-CONTROL-FLOW",
            Self::DominanceViolation => "ZMN-IR-DOMINANCE-VIOLATION",
            Self::InvalidEffect => "ZMN-IR-INVALID-EFFECT",
            Self::OwnershipViolation => "ZMN-IR-OWNERSHIP-VIOLATION",
            Self::ResourceConstraintViolation => "ZMN-IR-RESOURCE-CONSTRAINT",
            Self::ResourceExhausted => "ZMN-IR-RESOURCE-EXHAUSTED",
            Self::CapabilityUnavailable => "ZMN-IR-CAPABILITY-UNAVAILABLE",
            Self::UnknownDialect => "ZMN-IR-UNKNOWN-DIALECT",
            Self::InvalidDialectDefinition => "ZMN-IR-INVALID-DIALECT",
            Self::UnsupportedOperation => "ZMN-IR-UNSUPPORTED-OPERATION",
            Self::UnsupportedAttribute => "ZMN-IR-UNSUPPORTED-ATTRIBUTE",
            Self::UnsupportedVersion => "ZMN-IR-VERSION-UNSUPPORTED",
            Self::MigrationRequired => "ZMN-IR-VERSION-MIGRATION-REQUIRED",
            Self::SerializationFailure => "ZMN-IR-SERIALIZATION-FAILED",
            Self::DeserializationFailure => "ZMN-IR-DESERIALIZATION-FAILED",
            Self::VerificationFailed => "ZMN-IR-VERIFICATION-FAILED",
            Self::TransformationFailed => "ZMN-IR-TRANSFORMATION-FAILED",
            Self::LoweringUnsupported => "ZMN-IR-LOWERING-UNSUPPORTED",
            Self::TargetIncompatible => "ZMN-IR-TARGET-INCOMPATIBLE",
            Self::ConfigurationInvalid => "ZMN-IR-CONFIGURATION-INVALID",
            Self::ExtensionFailure => "ZMN-IR-EXTENSION-FAILED",
            Self::Cancelled => "ZMN-IR-CANCELLED",
            Self::IoFailure => "ZMN-IR-IO-FAILED",
            Self::InternalFailure => "ZMN-IR-INTERNAL-FAILURE",
            Self::LimitExceeded => "ZMN-IR-LIMIT-EXCEEDED",
            Self::Conflict => "ZMN-IR-CONFLICT",
            Self::NotFound => "ZMN-IR-NOT-FOUND",
        }
    }

    /// Returns whether this category normally indicates invalid representation
    /// or invalid program meaning rather than an operational interruption.
    ///
    /// This is a convenience classification, not a replacement for the
    /// responsible verifier's result or the diagnostic severity policy.
    #[must_use]
    pub const fn is_representation_failure(self) -> bool {
        matches!(
            self,
            Self::MalformedRepresentation
                | Self::InvalidIdentifier
                | Self::DuplicateDefinition
                | Self::UnresolvedReference
                | Self::InvalidType
                | Self::InvalidControlFlow
                | Self::DominanceViolation
                | Self::InvalidEffect
                | Self::OwnershipViolation
                | Self::ResourceConstraintViolation
                | Self::InvalidDialectDefinition
                | Self::UnsupportedOperation
                | Self::UnsupportedAttribute
                | Self::VerificationFailed
        )
    }

    /// Returns whether the error may indicate a missing implementation or
    /// target capability rather than invalid source-level meaning.
    #[must_use]
    pub const fn is_compatibility_or_realization_failure(self) -> bool {
        matches!(
            self,
            Self::CapabilityUnavailable
                | Self::UnknownDialect
                | Self::UnsupportedVersion
                | Self::MigrationRequired
                | Self::LoweringUnsupported
                | Self::TargetIncompatible
        )
    }

    /// Returns whether the category commonly indicates operational
    /// interruption or exhausted execution resources.
    #[must_use]
    pub const fn is_operational_failure(self) -> bool {
        matches!(
            self,
            Self::ResourceExhausted
                | Self::Cancelled
                | Self::IoFailure
                | Self::LimitExceeded
        )
    }
}

impl fmt::Display for ZuirErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidInput => "invalid input",
            Self::MalformedRepresentation => "malformed representation",
            Self::InvalidIdentifier => "invalid identifier",
            Self::DuplicateDefinition => "duplicate definition",
            Self::UnresolvedReference => "unresolved reference",
            Self::InvalidType => "invalid type",
            Self::InvalidControlFlow => "invalid control flow",
            Self::DominanceViolation => "dominance violation",
            Self::InvalidEffect => "invalid effect",
            Self::OwnershipViolation => "ownership violation",
            Self::ResourceConstraintViolation => "resource constraint violation",
            Self::ResourceExhausted => "resource exhausted",
            Self::CapabilityUnavailable => "capability unavailable",
            Self::UnknownDialect => "unknown dialect",
            Self::InvalidDialectDefinition => "invalid dialect definition",
            Self::UnsupportedOperation => "unsupported operation",
            Self::UnsupportedAttribute => "unsupported attribute",
            Self::UnsupportedVersion => "unsupported version",
            Self::MigrationRequired => "migration required",
            Self::SerializationFailure => "serialization failure",
            Self::DeserializationFailure => "deserialization failure",
            Self::VerificationFailed => "verification failed",
            Self::TransformationFailed => "transformation failed",
            Self::LoweringUnsupported => "lowering unsupported",
            Self::TargetIncompatible => "target incompatible",
            Self::ConfigurationInvalid => "invalid configuration",
            Self::ExtensionFailure => "extension failure",
            Self::Cancelled => "cancelled",
            Self::IoFailure => "I/O failure",
            Self::InternalFailure => "internal failure",
            Self::LimitExceeded => "limit exceeded",
            Self::Conflict => "conflict",
            Self::NotFound => "not found",
        })
    }
}

// =============================================================================
// Stable error codes
// =============================================================================

/// A stable machine-readable ZUIR error code.
///
/// Standard codes use the `ZMN-IR-...` namespace. Extension-defined codes
/// must use the repository-governed `ZMN-EXT-...` namespace and must be
/// registered under the extension policy before being treated as stable.
///
/// Human-readable error messages must not be parsed in place of this code.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ZuirErrorCode(String);

impl ZuirErrorCode {
    /// Creates a validated error code.
    ///
    /// Codes must start with `ZMN-`, contain at least three nonempty
    /// hyphen-separated segments, and use uppercase ASCII letters or digits.
    ///
    /// This validates syntax only; it does not register the code or establish
    /// that its meaning is approved by a specification.
    pub fn new(code: impl Into<String>) -> Result<Self, InvalidErrorCode> {
        let code = code.into();

        let valid = code.starts_with("ZMN-")
            && code
                .split('-')
                .count()
                >= 3
            && code.split('-').all(|segment| {
                !segment.is_empty()
                    && segment
                        .bytes()
                        .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit())
            });

        if valid {
            Ok(Self(code))
        } else {
            Err(InvalidErrorCode)
        }
    }

    /// Returns the code as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn from_standard(code: &'static str) -> Self {
        // All values passed here are compile-time constants defined by
        // `ZuirErrorKind::default_code`.
        Self(code.to_owned())
    }
}

impl fmt::Display for ZuirErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Error returned when a machine-readable error code has invalid syntax.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InvalidErrorCode;

impl fmt::Display for InvalidErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            "error code must use the ZMN- namespace and uppercase ASCII segments",
        )
    }
}

impl StdError for InvalidErrorCode {}

// =============================================================================
// IR entity context
// =============================================================================

/// Identifies an IR entity associated with an error.
///
/// These variants reuse the canonical typed identifiers from `ids.rs`.
/// Numeric identifiers are opaque identities; they are not array indices,
/// hardware addresses, source offsets, or content fingerprints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(tag = "entity_kind", content = "id", rename_all = "snake_case")]
pub enum ErrorSubject {
    Program(ProgramId),
    Module(ModuleId),
    Function(FunctionId),
    Region(RegionId),
    Block(BlockId),
    Operation(OperationId),
    Value(ValueId),
    Type(TypeId),
    Dialect(DialectId),
    Extension(ExtensionId),
    Symbol(SymbolId),
    Source(SourceId),
}

impl fmt::Display for ErrorSubject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Program(id) => write!(f, "program {id}"),
            Self::Module(id) => write!(f, "module {id}"),
            Self::Function(id) => write!(f, "function {id}"),
            Self::Region(id) => write!(f, "region {id}"),
            Self::Block(id) => write!(f, "block {id}"),
            Self::Operation(id) => write!(f, "operation {id}"),
            Self::Value(id) => write!(f, "value {id}"),
            Self::Type(id) => write!(f, "type {id}"),
            Self::Dialect(id) => write!(f, "dialect {id}"),
            Self::Extension(id) => write!(f, "extension {id}"),
            Self::Symbol(id) => write!(f, "symbol {id}"),
            Self::Source(id) => write!(f, "source {id}"),
        }
    }
}

// =============================================================================
// Error phase
// =============================================================================

/// Broad phase in which the error was detected.
///
/// These values describe ZUIR processing boundaries, not individual
/// implementation modules or user-interface states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum ErrorPhase {
    Construction,
    Verification,
    Analysis,
    Transformation,
    DialectRegistration,
    Compatibility,
    Migration,
    Serialization,
    Deserialization,
    Interchange,
    Lowering,
    Configuration,
    Extension,
    RuntimeBoundary,
}

impl fmt::Display for ErrorPhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Construction => "construction",
            Self::Verification => "verification",
            Self::Analysis => "analysis",
            Self::Transformation => "transformation",
            Self::DialectRegistration => "dialect registration",
            Self::Compatibility => "compatibility",
            Self::Migration => "migration",
            Self::Serialization => "serialization",
            Self::Deserialization => "deserialization",
            Self::Interchange => "interchange",
            Self::Lowering => "lowering",
            Self::Configuration => "configuration",
            Self::Extension => "extension",
            Self::RuntimeBoundary => "runtime boundary",
        })
    }
}

// =============================================================================
// Version context
// =============================================================================

/// Version mismatch details attached to an error.
///
/// Concrete version categories remain distinct. A serialization format
/// version must not be accidentally treated as a schema or dialect version.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(tag = "version_kind", rename_all = "snake_case")]
pub enum VersionErrorContext {
    Schema {
        supplied: ZuirSchemaVersion,
        expected: ZuirSchemaVersion,
    },
    Format {
        supplied: ZuirFormatVersion,
        expected: ZuirFormatVersion,
    },
    Dialect {
        dialect_name: String,
        supplied: DialectVersion,
        expected: DialectVersion,
    },
    Language {
        supplied: LanguageVersion,
        expected: LanguageVersion,
    },
}

// =============================================================================
// Structured context
// =============================================================================

/// Optional structured context accompanying a ZUIR error.
///
/// This type intentionally does not contain source spans. Source locations,
/// labels, severity, related diagnostics, and rendering are owned by the
/// repository's diagnostic system and source-map contracts.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZuirErrorContext {
    subject: Option<ErrorSubject>,
    phase: Option<ErrorPhase>,
    artifact_path: Option<String>,
    version: Option<VersionErrorContext>,
    related_subjects: Vec<ErrorSubject>,
    notes: Vec<String>,
}

impl ZuirErrorContext {
    /// Creates an empty context.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            subject: None,
            phase: None,
            artifact_path: None,
            version: None,
            related_subjects: Vec::new(),
            notes: Vec::new(),
        }
    }

    /// Returns the primary entity associated with the error, if known.
    #[must_use]
    pub const fn subject(&self) -> Option<ErrorSubject> {
        self.subject
    }

    /// Returns the processing phase, if known.
    #[must_use]
    pub const fn phase(&self) -> Option<ErrorPhase> {
        self.phase
    }

    /// Returns the associated artifact path, if any.
    ///
    /// The path is contextual information supplied by the caller. This module
    /// does not access the filesystem or interpret the path.
    #[must_use]
    pub fn artifact_path(&self) -> Option<&str> {
        self.artifact_path.as_deref()
    }

    /// Returns version mismatch details, if present.
    #[must_use]
    pub fn version(&self) -> Option<&VersionErrorContext> {
        self.version.as_ref()
    }

    /// Returns additional IR subjects associated with the error.
    #[must_use]
    pub fn related_subjects(&self) -> &[ErrorSubject] {
        &self.related_subjects
    }

    /// Returns contextual notes in insertion order.
    #[must_use]
    pub fn notes(&self) -> &[String] {
        &self.notes
    }

    /// Sets the primary IR entity associated with the error.
    #[must_use]
    pub fn with_subject(mut self, subject: ErrorSubject) -> Self {
        self.subject = Some(subject);
        self
    }

    /// Sets the phase in which the error was detected.
    #[must_use]
    pub fn with_phase(mut self, phase: ErrorPhase) -> Self {
        self.phase = Some(phase);
        self
    }

    /// Sets an artifact path associated with the error.
    #[must_use]
    pub fn with_artifact_path(mut self, path: impl Into<String>) -> Self {
        self.artifact_path = Some(path.into());
        self
    }

    /// Sets version mismatch context.
    #[must_use]
    pub fn with_version(mut self, version: VersionErrorContext) -> Self {
        self.version = Some(version);
        self
    }

    /// Appends a related IR entity.
    pub fn push_related_subject(&mut self, subject: ErrorSubject) {
        self.related_subjects.push(subject);
    }

    /// Appends an explanatory context note.
    ///
    /// Notes are not stable machine interfaces and must not replace error
    /// codes or structured fields.
    pub fn push_note(&mut self, note: impl Into<String>) {
        self.notes.push(note.into());
    }
}

// =============================================================================
// Canonical error value
// =============================================================================

/// A structured ZUIR failure.
///
/// `ZuirError` implements the standard Rust error trait and supports chaining
/// an underlying error. It deliberately does not implement `Clone` or
/// serialize the underlying error object; diagnostic and interchange layers
/// should serialize structured error records rather than opaque error chains.
#[derive(Debug)]
pub struct ZuirError {
    kind: ZuirErrorKind,
    code: ZuirErrorCode,
    message: String,
    context: ZuirErrorContext,
    source: Option<Box<dyn StdError + Send + Sync + 'static>>,
}

impl ZuirError {
    /// Creates an error using the standard code associated with `kind`.
    #[must_use]
    pub fn new(kind: ZuirErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            code: ZuirErrorCode::from_standard(kind.default_code()),
            message: message.into(),
            context: ZuirErrorContext::new(),
            source: None,
        }
    }

    /// Creates an error with a validated custom code.
    ///
    /// This is intended for registered extensions or cases where a more
    /// specific stable code has been approved. Syntax validation does not
    /// establish registry membership.
    pub fn with_code(
        kind: ZuirErrorKind,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Result<Self, InvalidErrorCode> {
        Ok(Self {
            kind,
            code: ZuirErrorCode::new(code)?,
            message: message.into(),
            context: ZuirErrorContext::new(),
            source: None,
        })
    }

    /// Creates an error while preserving an underlying standard error.
    pub fn from_source<E>(
        kind: ZuirErrorKind,
        message: impl Into<String>,
        source: E,
    ) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self::new(kind, message).with_source(source)
    }

    /// Returns the broad error category.
    #[must_use]
    pub const fn kind(&self) -> ZuirErrorKind {
        self.kind
    }

    /// Returns the stable machine-readable code.
    #[must_use]
    pub fn code(&self) -> &ZuirErrorCode {
        &self.code
    }

    /// Returns the explanatory message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns structured context.
    #[must_use]
    pub fn context(&self) -> &ZuirErrorContext {
        &self.context
    }

    /// Replaces the current structured context.
    #[must_use]
    pub fn with_context(mut self, context: ZuirErrorContext) -> Self {
        self.context = context;
        self
    }

    /// Sets the primary IR entity associated with the error.
    #[must_use]
    pub fn with_subject(mut self, subject: ErrorSubject) -> Self {
        self.context.subject = Some(subject);
        self
    }

    /// Sets the processing phase in which the error was detected.
    #[must_use]
    pub fn with_phase(mut self, phase: ErrorPhase) -> Self {
        self.context.phase = Some(phase);
        self
    }

    /// Sets an associated artifact path.
    #[must_use]
    pub fn with_artifact_path(mut self, path: impl Into<String>) -> Self {
        self.context.artifact_path = Some(path.into());
        self
    }

    /// Attaches version mismatch details.
    #[must_use]
    pub fn with_version(mut self, version: VersionErrorContext) -> Self {
        self.context.version = Some(version);
        self
    }

    /// Adds an associated IR entity.
    #[must_use]
    pub fn with_related_subject(mut self, subject: ErrorSubject) -> Self {
        self.context.push_related_subject(subject);
        self
    }

    /// Adds an explanatory note.
    #[must_use]
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.context.push_note(note);
        self
    }

    /// Attaches the underlying cause while preserving its error chain.
    #[must_use]
    pub fn with_source<E>(mut self, source: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        self.source = Some(Box::new(source));
        self
    }

    /// Returns the underlying error, if present.
    #[must_use]
    pub fn source_error(&self) -> Option<&(dyn StdError + Send + Sync + 'static)> {
        self.source.as_deref()
    }
}

impl fmt::Display for ZuirError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)?;

        if let Some(phase) = self.context.phase {
            write!(f, " (phase: {phase}")?;

            if let Some(subject) = self.context.subject {
                write!(f, ", subject: {subject}")?;
            }

            f.write_str(")")?;
        } else if let Some(subject) = self.context.subject {
            write!(f, " (subject: {subject})")?;
        }

        Ok(())
    }
}

impl StdError for ZuirError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source
            .as_deref()
            .map(|source| source as &(dyn StdError + 'static))
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_codes_are_stable_and_machine_readable() {
        let error = ZuirError::new(
            ZuirErrorKind::VerificationFailed,
            "IR verification did not establish the required invariants",
        );

        assert_eq!(error.kind(), ZuirErrorKind::VerificationFailed);
        assert_eq!(error.code().as_str(), "ZMN-IR-VERIFICATION-FAILED");
        assert!(error.to_string().contains("ZMN-IR-VERIFICATION-FAILED"));
    }

    #[test]
    fn custom_codes_are_validated() {
        assert!(ZuirErrorCode::new("ZMN-EXT-EXAMPLE-CUSTOM-FAILURE").is_ok());
        assert!(ZuirErrorCode::new("ZMN-IR-INVALID-TYPE").is_ok());

        assert!(ZuirErrorCode::new("invalid-code").is_err());
        assert!(ZuirErrorCode::new("ZMN--EMPTY").is_err());
        assert!(ZuirErrorCode::new("ZMN-ir-lowercase").is_err());
        assert!(ZuirErrorCode::new("ZMN-TRAILING-").is_err());
    }

    #[test]
    fn custom_error_codes_can_be_used_without_changing_the_category() {
        let error = ZuirError::with_code(
            ZuirErrorKind::ExtensionFailure,
            "ZMN-EXT-EXAMPLE-VALIDATION-FAILED",
            "the registered extension rejected its operation schema",
        )
        .expect("code syntax is valid");

        assert_eq!(error.kind(), ZuirErrorKind::ExtensionFailure);
        assert_eq!(
            error.code().as_str(),
            "ZMN-EXT-EXAMPLE-VALIDATION-FAILED"
        );
    }

    #[test]
    fn context_preserves_typed_entity_identity() {
        let operation = OperationId::new(17).expect("nonzero ID");
        let value = ValueId::new(23).expect("nonzero ID");

        let error = ZuirError::new(
            ZuirErrorKind::InvalidType,
            "operation operand has an incompatible type",
        )
        .with_subject(ErrorSubject::Operation(operation))
        .with_related_subject(ErrorSubject::Value(value))
        .with_phase(ErrorPhase::Verification)
        .with_note("operand and result constraints must be checked");

        assert_eq!(
            error.context().subject(),
            Some(ErrorSubject::Operation(operation))
        );
        assert_eq!(
            error.context().related_subjects(),
            &[ErrorSubject::Value(value)]
        );
        assert_eq!(error.context().phase(), Some(ErrorPhase::Verification));
        assert_eq!(error.context().notes().len(), 1);
    }

    #[test]
    fn version_context_keeps_schema_and_format_versions_distinct() {
        let supplied = ZuirSchemaVersion::new(1, 0, 0);
        let expected = ZuirSchemaVersion::new(2, 0, 0);

        let error = ZuirError::new(
            ZuirErrorKind::UnsupportedVersion,
            "the supplied schema version is not supported",
        )
        .with_version(VersionErrorContext::Schema { supplied, expected })
        .with_phase(ErrorPhase::Compatibility);

        assert!(matches!(
            error.context().version(),
            Some(VersionErrorContext::Schema {
                supplied: actual_supplied,
                expected: actual_expected,
            }) if *actual_supplied == supplied && *actual_expected == expected
        ));
    }

    #[test]
    fn source_error_is_preserved_for_standard_error_chaining() {
        let underlying = std::io::Error::other("simulated I/O failure");

        let error = ZuirError::from_source(
            ZuirErrorKind::IoFailure,
            "unable to read the interchange artifact",
            underlying,
        );

        assert_eq!(error.kind(), ZuirErrorKind::IoFailure);
        assert!(StdError::source(&error).is_some());
        assert!(error.source_error().is_some());
    }

    #[test]
    fn error_context_serializes_and_round_trips() {
        let operation = OperationId::new(5).expect("nonzero ID");
        let context = ZuirErrorContext::new()
            .with_subject(ErrorSubject::Operation(operation))
            .with_phase(ErrorPhase::Verification)
            .with_artifact_path("module.zir");

        let encoded =
            serde_json::to_string(&context).expect("context serialization succeeds");
        let decoded: ZuirErrorContext =
            serde_json::from_str(&encoded).expect("context deserialization succeeds");

        assert_eq!(decoded, context);
    }

    #[test]
    fn error_classification_does_not_conflate_target_and_representation_failures() {
        assert!(ZuirErrorKind::InvalidType.is_representation_failure());
        assert!(ZuirErrorKind::TargetIncompatible.is_compatibility_or_realization_failure());
        assert!(ZuirErrorKind::ResourceExhausted.is_operational_failure());

        assert!(!ZuirErrorKind::TargetIncompatible.is_representation_failure());
        assert!(!ZuirErrorKind::ResourceExhausted.is_representation_failure());
    }

    #[test]
    fn error_display_is_useful_without_a_terminal_renderer() {
        let operation = OperationId::new(9).expect("nonzero ID");

        let error = ZuirError::new(
            ZuirErrorKind::LoweringUnsupported,
            "no semantics-preserving lowering is registered",
        )
        .with_subject(ErrorSubject::Operation(operation))
        .with_phase(ErrorPhase::Lowering);

        let rendered = error.to_string();

        assert!(rendered.contains("ZMN-IR-LOWERING-UNSUPPORTED"));
        assert!(rendered.contains("operation 9"));
        assert!(rendered.contains("lowering"));
    }
}
