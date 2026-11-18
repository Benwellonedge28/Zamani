
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Toolchain Diagnostics
//!
//! File: src/toolchain/diagnostics/diagnostic.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Define the canonical structured diagnostic record for the Zamani
//! toolchain.
//!
//! Toolchain diagnostics report conditions arising from project and
//! workspace management, manifest integration, dependency resolution,
//! build orchestration, task execution, package management, registries,
//! external tools, target validation, deployment, security, and providers.
//!
//! They are distinct from source-language diagnostics.
//!
//! ## OWNERSHIP
//!
//! This module owns:
//!
//! - ToolchainDiagnostic.
//! - DiagnosticCode.
//! - DiagnosticSeverity.
//! - DiagnosticCategory.
//! - DiagnosticPhase.
//! - DiagnosticOrigin.
//! - DiagnosticLabel.
//! - DiagnosticNote.
//! - DiagnosticFix.
//! - DiagnosticFixEdit.
//! - DiagnosticValue.
//! - DiagnosticValidationError.
//! - Diagnostic construction and local invariant validation.
//! - Stable machine-readable field access.
//!
//! It does not own:
//!
//! - Source maps or source-coordinate calculation.
//! - Frontend lexical, parser, or semantic diagnostics.
//! - Compiler diagnostics.
//! - Backend diagnostic classification.
//! - Diagnostic aggregation or deduplication.
//! - Diagnostic ordering algorithms.
//! - Terminal rendering.
//! - JSON report envelopes or report serialization policies.
//! - Diagnostic collection budgets.
//! - Recovery algorithms.
//! - Source editing.
//! - Build execution or provider selection.
//!
//! ## AUTHORITIES
//!
//! Normative diagnostic contract:
//! grammar/spec/diagnostics.md
//!
//! Toolchain error authority:
//! crate::toolchain::errors
//!
//! Frontend diagnostic authority:
//! crate::frontend::semantic::diagnostics
//!
//! Canonical source-span authority:
//! crate::frontend::ast::node::source::span
//!
//! ## INTEGRATION
//!
//! The diagnostic record uses the canonical frontend Span directly.
//!
//! It does not create another SourceId, SourceOffset, Span, AST,
//! semantic model, compiler error, or backend error.
//!
//! Aggregation belongs to aggregation.rs.
//! Ordering belongs to the designated ordering implementation.
//! Rendering belongs to rendering.rs.
//! Machine-readable report envelopes belong to reporting.rs.
//!
//! ## EXTENSIBILITY
//!
//! Diagnostic codes, categories, phases, component names and operation
//! names are open-world strings.
//!
//! New computing domains and providers must not require modification
//! of this module.
//!
//! ## SCALABILITY
//!
//! This module defines no universal diagnostic-count ceiling,
//! source-size ceiling, hardware limit, device limit, or domain limit.
//!
//! Collections grow according to available memory and the operational
//! budgets imposed by their owning callers.
//!
//! A diagnostic collection budget is not a language capacity limit.
//!
//! ## DETERMINISM
//!
//! Metadata uses BTreeMap so its iteration order is deterministic.
//! Labels, notes and fixes preserve insertion order.
//!
//! This module does not implement diagnostic sorting or deduplication.
//!
//! ## SECURITY
//!
//! Diagnostic messages and metadata must not contain credentials,
//! private keys, access tokens, or other secrets.
//!
//! Fixes are suggestions. This module never modifies source files.
//!
//! External provider text must be treated as untrusted data.
//!
//! ## COMPLETION
//!
//! A diagnostic describes an observed condition. It does not prove
//! that a build, analysis, deployment, or execution operation completed.
//!
//! Operation completeness belongs to the owning operation result.

use core::fmt;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::frontend::ast::node::source::span::Span;

// ============================================================================
// DIAGNOSTIC CODE
// ============================================================================

/// Stable machine-readable toolchain diagnostic identifier.
///
/// Codes are open-world strings rather than a closed enumeration.
///
/// Recommended form:
///
/// `toolchain.build.failed`
/// `toolchain.dependency.unresolved`
/// `toolchain.package.integrity_failure`
/// `toolchain.registry.authentication_failed`
///
/// Provider extensions may introduce their own namespaced identifiers.
///
/// Existing public codes must never be reassigned to different meanings.
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct DiagnosticCode(String);

impl DiagnosticCode {
    /// Construct a diagnostic code.
    ///
    /// Validation is deliberately explicit and does not panic.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Return the stable code.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Whether the code is nonempty and contains non-whitespace text.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self.0.trim().is_empty()
    }
}

impl From<&str> for DiagnosticCode {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for DiagnosticCode {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for DiagnosticCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// SEVERITY
// ============================================================================

/// Diagnostic importance.
///
/// Severity is independent of operation completeness, recovery,
/// target feasibility and execution status.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticSeverity {
    /// An additional suggestion.
    Hint,

    /// Supplementary information.
    Note,

    /// Informational event.
    Information,

    /// A condition requiring attention.
    #[default]
    Warning,

    /// An operation or requirement failed.
    Error,

    /// A condition prevents safe continuation.
    Fatal,
}

impl DiagnosticSeverity {
    /// Whether the severity represents an error.
    #[must_use]
    pub const fn is_error(self) -> bool {
        matches!(self, Self::Error | Self::Fatal)
    }

    /// Whether the severity is fatal.
    #[must_use]
    pub const fn is_fatal(self) -> bool {
        matches!(self, Self::Fatal)
    }

    /// Stable machine-readable representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Hint => "hint",
            Self::Note => "note",
            Self::Information => "information",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Fatal => "fatal",
        }
    }
}

impl fmt::Display for DiagnosticSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// CATEGORY
// ============================================================================

/// Broad classification of a toolchain diagnostic.
///
/// Categories are open-world strings so new subsystems and computing
/// domains can report diagnostics without modifying this module.
///
/// Examples:
///
/// - project
/// - workspace
/// - manifest
/// - dependency
/// - build
/// - task
/// - package
/// - registry
/// - target
/// - provider
/// - external_tool
/// - security
/// - deployment
/// - portability
/// - extension-specific namespaces
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct DiagnosticCategory(String);

impl DiagnosticCategory {
    /// Construct an extensible category.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Return the category identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Whether the category contains non-whitespace text.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self.0.trim().is_empty()
    }
}

impl From<&str> for DiagnosticCategory {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for DiagnosticCategory {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for DiagnosticCategory {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// PHASE
// ============================================================================

/// Toolchain lifecycle phase that produced a diagnostic.
///
/// This records provenance. It does not execute a phase or impose
/// an exhaustive list of future toolchain operations.
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct DiagnosticPhase(String);

impl DiagnosticPhase {
    /// Construct a phase identifier.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Return the stable phase identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Whether the phase is nonempty.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self.0.trim().is_empty()
    }
}

impl From<&str> for DiagnosticPhase {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for DiagnosticPhase {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for DiagnosticPhase {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// ORIGIN AND PROVENANCE
// ============================================================================

/// Identifies the subsystem and operation responsible for a diagnostic.
///
/// These fields are descriptive provenance, not replacement subsystem
/// identities.
///
/// Examples:
///
/// component: "toolchain.dependencies"
/// operation: "dependency.resolve"
///
/// component: "backend.toolchain"
/// operation: "external_tool.validate"
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticOrigin {
    component: String,
    operation: Option<String>,
    subject: Option<String>,
    provider: Option<String>,
}

impl DiagnosticOrigin {
    /// Construct diagnostic provenance.
    #[must_use]
    pub fn new(component: impl Into<String>) -> Self {
        Self {
            component: component.into(),
            operation: None,
            subject: None,
            provider: None,
        }
    }

    /// Return the originating component.
    #[must_use]
    pub fn component(&self) -> &str {
        &self.component
    }

    /// Return the originating operation.
    #[must_use]
    pub fn operation(&self) -> Option<&str> {
        self.operation.as_deref()
    }

    /// Return the affected subject.
    #[must_use]
    pub fn subject(&self) -> Option<&str> {
        self.subject.as_deref()
    }

    /// Return the provider identifier, if applicable.
    #[must_use]
    pub fn provider(&self) -> Option<&str> {
        self.provider.as_deref()
    }

    /// Attach an operation identifier.
    #[must_use]
    pub fn with_operation(mut self, value: impl Into<String>) -> Self {
        self.operation = Some(value.into());
        self
    }

    /// Attach a subject identifier.
    #[must_use]
    pub fn with_subject(mut self, value: impl Into<String>) -> Self {
        self.subject = Some(value.into());
        self
    }

    /// Attach a provider identifier.
    #[must_use]
    pub fn with_provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    /// Validate mandatory provenance.
    pub fn validate(&self) -> Result<(), DiagnosticValidationError> {
        if self.component.trim().is_empty() {
            return Err(DiagnosticValidationError::EmptyOriginComponent);
        }

        if self.operation.as_deref().is_some_and(str::is_empty) {
            return Err(DiagnosticValidationError::EmptyOriginOperation);
        }

        Ok(())
    }
}

// ============================================================================
// STRUCTURED METADATA
// ============================================================================

/// Typed, serializable diagnostic metadata.
///
/// Metadata must remain data-only. Consumers must not parse human-readable
/// messages to discover structured diagnostic information.
///
/// BTreeMap provides deterministic object-key iteration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
#[non_exhaustive]
pub enum DiagnosticValue {
    Null,
    Boolean(bool),
    Signed(i128),
    Unsigned(u128),
    Float(f64),
    Text(String),
    Sequence(Vec<DiagnosticValue>),
    Object(BTreeMap<String, DiagnosticValue>),
}

impl DiagnosticValue {
    /// Construct an empty ordered object.
    #[must_use]
    pub fn object() -> Self {
        Self::Object(BTreeMap::new())
    }

    /// Insert a value into an object.
    ///
    /// Returns the previous value when the key already existed.
    /// Returns an error rather than changing a non-object metadata value.
    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: DiagnosticValue,
    ) -> Result<Option<DiagnosticValue>, DiagnosticValidationError> {
        match self {
            Self::Object(entries) => Ok(entries.insert(key.into(), value)),
            _ => Err(DiagnosticValidationError::MetadataIsNotObject),
        }
    }

    /// Return an object entry.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&DiagnosticValue> {
        match self {
            Self::Object(entries) => entries.get(key),
            _ => None,
        }
    }

    /// Return whether this value is an object.
    #[must_use]
    pub const fn is_object(&self) -> bool {
        matches!(self, Self::Object(_))
    }
}

impl From<bool> for DiagnosticValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<String> for DiagnosticValue {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<&str> for DiagnosticValue {
    fn from(value: &str) -> Self {
        Self::Text(value.to_owned())
    }
}

impl From<i128> for DiagnosticValue {
    fn from(value: i128) -> Self {
        Self::Signed(value)
    }
}

impl From<u128> for DiagnosticValue {
    fn from(value: u128) -> Self {
        Self::Unsigned(value)
    }
}

impl From<f64> for DiagnosticValue {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

// ============================================================================
// SOURCE LABELS
// ============================================================================

/// A secondary source location associated with a diagnostic.
///
/// The canonical frontend Span is reused directly.
///
/// The toolchain does not calculate source positions or construct
/// a competing source-map implementation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticLabel {
    span: Span,
    message: String,
}

impl DiagnosticLabel {
    /// Construct a secondary source label.
    #[must_use]
    pub fn new(span: Span, message: impl Into<String>) -> Self {
        Self {
            span,
            message: message.into(),
        }
    }

    /// Return the canonical source span.
    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }

    /// Return the explanatory label.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

// ============================================================================
// NOTES
// ============================================================================

/// An additional explanatory diagnostic note.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticNote {
    message: String,
}

impl DiagnosticNote {
    /// Construct a note.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    /// Return the note message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

// ============================================================================
// FIX-ITS
// ============================================================================

/// One proposed source edit.
///
/// The edit is descriptive only. Applying it requires a separate
/// source-editing service to validate source identity, source version,
/// UTF-8 boundaries, overlapping edits and authorization.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticFixEdit {
    span: Span,
    replacement: String,
}

impl DiagnosticFixEdit {
    /// Construct a proposed edit.
    #[must_use]
    pub fn new(span: Span, replacement: impl Into<String>) -> Self {
        Self {
            span,
            replacement: replacement.into(),
        }
    }

    /// Return the canonical replacement span.
    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }

    /// Return replacement text.
    #[must_use]
    pub fn replacement(&self) -> &str {
        &self.replacement
    }
}

/// A proposed correction associated with a diagnostic.
///
/// `safe` means the producer has evidence that the proposed edit is
/// appropriate within the producer's stated scope. It is not permission
/// to apply the edit automatically.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticFix {
    title: String,
    edits: Vec<DiagnosticFixEdit>,
    safe: bool,
}

impl DiagnosticFix {
    /// Construct a fix containing ordered edits.
    #[must_use]
    pub fn new(
        title: impl Into<String>,
        edits: Vec<DiagnosticFixEdit>,
        safe: bool,
    ) -> Self {
        Self {
            title: title.into(),
            edits,
            safe,
        }
    }

    /// Return the fix title.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Return edits in producer-defined deterministic order.
    #[must_use]
    pub fn edits(&self) -> &[DiagnosticFixEdit] {
        &self.edits
    }

    /// Whether the producer classifies this fix as safe.
    #[must_use]
    pub const fn is_safe(&self) -> bool {
        self.safe
    }

    /// Whether this fix contains no edits.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.edits.is_empty()
    }
}

// ============================================================================
// VALIDATION ERRORS
// ============================================================================

/// Validation failures for locally constructed diagnostic records.
///
/// These are diagnostic-record errors, not source-language errors.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum DiagnosticValidationError {
    EmptyCode,
    EmptyCategory,
    EmptyPhase,
    EmptyMessage,
    EmptyOriginComponent,
    EmptyOriginOperation,
    MetadataIsNotObject,
}

impl fmt::Display for DiagnosticValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::EmptyCode => "diagnostic code must not be empty",
            Self::EmptyCategory => "diagnostic category must not be empty",
            Self::EmptyPhase => "diagnostic phase must not be empty",
            Self::EmptyMessage => "diagnostic message must not be empty",
            Self::EmptyOriginComponent => {
                "diagnostic origin component must not be empty"
            }
            Self::EmptyOriginOperation => {
                "diagnostic origin operation must not be empty"
            }
            Self::MetadataIsNotObject => {
                "diagnostic metadata value is not an object"
            }
        };

        formatter.write_str(message)
    }
}

impl std::error::Error for DiagnosticValidationError {}

// ============================================================================
// CANONICAL TOOLCHAIN DIAGNOSTIC
// ============================================================================

/// A complete structured toolchain diagnostic.
///
/// This is the canonical diagnostic record for the toolchain subsystem.
///
/// It may describe a source-related issue, a project configuration issue,
/// an infrastructure failure, a target compatibility problem, a provider
/// failure or an operational budget exhaustion.
///
/// Source locations are optional because many toolchain operations do not
/// originate from a particular source construct.
///
/// A diagnostic is not an operation result and cannot independently
/// certify operation completion or successful execution.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToolchainDiagnostic {
    schema_version: u16,
    code: DiagnosticCode,
    severity: DiagnosticSeverity,
    category: DiagnosticCategory,
    phase: DiagnosticPhase,
    message: String,
    primary_span: Option<Span>,
    labels: Vec<DiagnosticLabel>,
    notes: Vec<DiagnosticNote>,
    help: Option<String>,
    fixes: Vec<DiagnosticFix>,
    origin: DiagnosticOrigin,
    metadata: BTreeMap<String, DiagnosticValue>,
}

impl ToolchainDiagnostic {
    /// Construct a diagnostic with the required structured fields.
    ///
    /// Use `validate()` before publishing externally constructed or
    /// deserialized diagnostics.
    #[must_use]
    pub fn new(
        code: impl Into<DiagnosticCode>,
        severity: DiagnosticSeverity,
        category: impl Into<DiagnosticCategory>,
        phase: impl Into<DiagnosticPhase>,
        message: impl Into<String>,
        origin: DiagnosticOrigin,
    ) -> Self {
        Self {
            schema_version: 1,
            code: code.into(),
            severity,
            category: category.into(),
            phase: phase.into(),
            message: message.into(),
            primary_span: None,
            labels: Vec::new(),
            notes: Vec::new(),
            help: None,
            fixes: Vec::new(),
            origin,
            metadata: BTreeMap::new(),
        }
    }

    /// Construct and validate a diagnostic.
    pub fn try_new(
        code: impl Into<DiagnosticCode>,
        severity: DiagnosticSeverity,
        category: impl Into<DiagnosticCategory>,
        phase: impl Into<DiagnosticPhase>,
        message: impl Into<String>,
        origin: DiagnosticOrigin,
    ) -> Result<Self, DiagnosticValidationError> {
        let diagnostic = Self::new(
            code,
            severity,
            category,
            phase,
            message,
            origin,
        );

        diagnostic.validate()?;
        Ok(diagnostic)
    }

    /// Return the diagnostic schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    /// Return the stable diagnostic code.
    #[must_use]
    pub fn code(&self) -> &DiagnosticCode {
        &self.code
    }

    /// Return severity.
    #[must_use]
    pub const fn severity(&self) -> DiagnosticSeverity {
        self.severity
    }

    /// Return category.
    #[must_use]
    pub fn category(&self) -> &DiagnosticCategory {
        &self.category
    }

    /// Return phase.
    #[must_use]
    pub fn phase(&self) -> &DiagnosticPhase {
        &self.phase
    }

    /// Return the primary human-readable message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Return the primary source span, if available.
    #[must_use]
    pub const fn primary_span(&self) -> Option<Span> {
        self.primary_span
    }

    /// Return secondary source labels.
    #[must_use]
    pub fn labels(&self) -> &[DiagnosticLabel] {
        &self.labels
    }

    /// Return explanatory notes.
    #[must_use]
    pub fn notes(&self) -> &[DiagnosticNote] {
        &self.notes
    }

    /// Return optional remediation guidance.
    #[must_use]
    pub fn help(&self) -> Option<&str> {
        self.help.as_deref()
    }

    /// Return proposed corrections.
    #[must_use]
    pub fn fixes(&self) -> &[DiagnosticFix] {
        &self.fixes
    }

    /// Return diagnostic provenance.
    #[must_use]
    pub fn origin(&self) -> &DiagnosticOrigin {
        &self.origin
    }

    /// Return deterministic structured metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, DiagnosticValue> {
        &self.metadata
    }

    /// Whether this diagnostic represents an error.
    #[must_use]
    pub const fn is_error(&self) -> bool {
        self.severity.is_error()
    }

    /// Whether this diagnostic is fatal.
    #[must_use]
    pub const fn is_fatal(&self) -> bool {
        self.severity.is_fatal()
    }

    /// Attach a primary canonical source span.
    #[must_use]
    pub fn with_primary_span(mut self, span: Span) -> Self {
        self.primary_span = Some(span);
        self
    }

    /// Append a related source label.
    #[must_use]
    pub fn with_label(mut self, label: DiagnosticLabel) -> Self {
        self.labels.push(label);
        self
    }

    /// Append an explanatory note.
    #[must_use]
    pub fn with_note(mut self, note: DiagnosticNote) -> Self {
        self.notes.push(note);
        self
    }

    /// Attach remediation guidance.
    #[must_use]
    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    /// Append a proposed correction.
    #[must_use]
    pub fn with_fix(mut self, fix: DiagnosticFix) -> Self {
        self.fixes.push(fix);
        self
    }

    /// Insert structured metadata.
    ///
    /// Replacing an existing key is explicit through the returned value.
    pub fn insert_metadata(
        &mut self,
        key: impl Into<String>,
        value: DiagnosticValue,
    ) -> Option<DiagnosticValue> {
        self.metadata.insert(key.into(), value)
    }

    /// Attach a collection of metadata fields.
    #[must_use]
    pub fn with_metadata(
        mut self,
        metadata: BTreeMap<String, DiagnosticValue>,
    ) -> Self {
        self.metadata = metadata;
        self
    }

    /// Validate the local invariants of this diagnostic.
    ///
    /// This does not prove that a source span belongs to a registered source
    /// map, that an extension code is registered, or that the producer's
    /// message is factually correct. Those checks belong to their authorities.
    pub fn validate(&self) -> Result<(), DiagnosticValidationError> {
        if !self.code.is_valid() {
            return Err(DiagnosticValidationError::EmptyCode);
        }

        if !self.category.is_valid() {
            return Err(DiagnosticValidationError::EmptyCategory);
        }

        if !self.phase.is_valid() {
            return Err(DiagnosticValidationError::EmptyPhase);
        }

        if self.message.trim().is_empty() {
            return Err(DiagnosticValidationError::EmptyMessage);
        }

        self.origin.validate()?;

        Ok(())
    }

    /// Return the stable schema identifier.
    #[must_use]
    pub const fn schema_id() -> &'static str {
        "zamani.toolchain.diagnostic.v1"
    }
}

impl fmt::Display for ToolchainDiagnostic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} [{}] {}",
            self.severity,
            self.code,
            self.message
        )
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::ast::node::source::span::{
        SourceId,
        SourceOffset,
    };

    fn sample_origin() -> DiagnosticOrigin {
        DiagnosticOrigin::new("toolchain.build")
            .with_operation("build.execute")
            .with_subject("project:example")
    }

    fn sample_span() -> Span {
        Span::new(
            SourceId::from_raw(1),
            SourceOffset::from_raw(10),
            SourceOffset::from_raw(20),
        )
        .expect("valid test span")
    }

    fn sample_diagnostic() -> ToolchainDiagnostic {
        ToolchainDiagnostic::new(
            "toolchain.build.failed",
            DiagnosticSeverity::Error,
            "build",
            "build_execution",
            "The build operation failed.",
            sample_origin(),
        )
    }

    #[test]
    fn diagnostic_has_stable_schema_identity() {
        assert_eq!(
            ToolchainDiagnostic::schema_id(),
            "zamani.toolchain.diagnostic.v1"
        );

        assert_eq!(sample_diagnostic().schema_version(), 1);
    }

    #[test]
    fn diagnostic_preserves_open_world_codes() {
        let diagnostic = ToolchainDiagnostic::new(
            "future.quantum_provider.unsupported_feature",
            DiagnosticSeverity::Warning,
            "future_computing_domain",
            "provider_validation",
            "The provider does not establish the requested capability.",
            DiagnosticOrigin::new("toolchain.targets"),
        );

        assert_eq!(
            diagnostic.code().as_str(),
            "future.quantum_provider.unsupported_feature"
        );

        assert!(diagnostic.validate().is_ok());
    }

    #[test]
    fn diagnostic_validation_rejects_empty_code() {
        let diagnostic = ToolchainDiagnostic::new(
            "",
            DiagnosticSeverity::Error,
            "build",
            "build_execution",
            "Build failed.",
            sample_origin(),
        );

        assert_eq!(
            diagnostic.validate(),
            Err(DiagnosticValidationError::EmptyCode)
        );
    }

    #[test]
    fn diagnostic_validation_rejects_empty_message() {
        let diagnostic = ToolchainDiagnostic::new(
            "toolchain.build.failed",
            DiagnosticSeverity::Error,
            "build",
            "build_execution",
            "   ",
            sample_origin(),
        );

        assert_eq!(
            diagnostic.validate(),
            Err(DiagnosticValidationError::EmptyMessage)
        );
    }

    #[test]
    fn diagnostic_validation_requires_origin_component() {
        let diagnostic = ToolchainDiagnostic::new(
            "toolchain.build.failed",
            DiagnosticSeverity::Error,
            "build",
            "build_execution",
            "Build failed.",
            DiagnosticOrigin::new(" "),
        );

        assert_eq!(
            diagnostic.validate(),
            Err(DiagnosticValidationError::EmptyOriginComponent)
        );
    }

    #[test]
    fn diagnostic_uses_the_canonical_frontend_span() {
        let diagnostic = sample_diagnostic()
            .with_primary_span(sample_span());

        let span = diagnostic
            .primary_span()
            .expect("primary span must be present");

        assert_eq!(span.source().as_raw(), 1);
        assert_eq!(span.start_raw(), 10);
        assert_eq!(span.end_raw(), 20);
    }

    #[test]
    fn related_labels_preserve_insertion_order() {
        let first = DiagnosticLabel::new(
            sample_span(),
            "First related location.",
        );

        let second = DiagnosticLabel::new(
            sample_span(),
            "Second related location.",
        );

        let diagnostic = sample_diagnostic()
            .with_label(first)
            .with_label(second);

        assert_eq!(diagnostic.labels().len(), 2);
        assert_eq!(
            diagnostic.labels()[0].message(),
            "First related location."
        );
        assert_eq!(
            diagnostic.labels()[1].message(),
            "Second related location."
        );
    }

    #[test]
    fn metadata_uses_deterministic_key_order() {
        let mut diagnostic = sample_diagnostic();

        diagnostic.insert_metadata(
            "provider",
            DiagnosticValue::from("provider-a"),
        );

        diagnostic.insert_metadata(
            "operation",
            DiagnosticValue::from("compile"),
        );

        let keys: Vec<&str> = diagnostic
            .metadata()
            .keys()
            .map(String::as_str)
            .collect();

        assert_eq!(keys, vec!["operation", "provider"]);
    }

    #[test]
    fn metadata_rejects_insertion_into_non_object_values() {
        let mut value = DiagnosticValue::Text("not an object".to_owned());

        assert_eq!(
            value.insert("key", DiagnosticValue::Null),
            Err(DiagnosticValidationError::MetadataIsNotObject)
        );
    }

    #[test]
    fn error_and_fatal_severities_are_distinguished() {
        let error = sample_diagnostic();

        let fatal = ToolchainDiagnostic::new(
            "toolchain.security.policy_rejected",
            DiagnosticSeverity::Fatal,
            "security",
            "security_validation",
            "The operation cannot safely continue.",
            DiagnosticOrigin::new("toolchain.security"),
        );

        assert!(error.is_error());
        assert!(!error.is_fatal());

        assert!(fatal.is_error());
        assert!(fatal.is_fatal());
    }

    #[test]
    fn diagnostic_fixes_are_suggestions_not_mutations() {
        let fix = DiagnosticFix::new(
            "Correct the project configuration",
            vec![
                DiagnosticFixEdit::new(
                    sample_span(),
                    "corrected-value",
                ),
            ],
            true,
        );

        let diagnostic = sample_diagnostic().with_fix(fix);

        assert_eq!(diagnostic.fixes().len(), 1);
        assert!(diagnostic.fixes()[0].is_safe());
        assert_eq!(diagnostic.fixes()[0].edits().len(), 1);
    }

    #[test]
    fn diagnostic_serialization_round_trips() {
        let diagnostic = sample_diagnostic()
            .with_primary_span(sample_span())
            .with_note(DiagnosticNote::new(
                "Inspect the compiler invocation.",
            ))
            .with_help("Check the configured compiler provider.");

        let encoded = serde_json::to_string(&diagnostic)
            .expect("diagnostic serialization");

        let decoded: ToolchainDiagnostic =
            serde_json::from_str(&encoded)
                .expect("diagnostic deserialization");

        assert_eq!(decoded, diagnostic);
        assert!(decoded.validate().is_ok());
    }

    #[test]
    fn display_uses_stable_machine_code() {
        let diagnostic = sample_diagnostic();

        assert_eq!(
            diagnostic.to_string(),
            "error [toolchain.build.failed] The build operation failed."
        );
    }

    #[test]
    fn diagnostic_origin_preserves_provider_provenance() {
        let origin = DiagnosticOrigin::new("toolchain.targets")
            .with_operation("target.validate")
            .with_provider("example.provider")
            .with_subject("target:portable");

        assert_eq!(origin.component(), "toolchain.targets");
        assert_eq!(origin.operation(), Some("target.validate"));
        assert_eq!(origin.provider(), Some("example.provider"));
        assert_eq!(origin.subject(), Some("target:portable"));
        assert!(origin.validate().is_ok());
    }
}
