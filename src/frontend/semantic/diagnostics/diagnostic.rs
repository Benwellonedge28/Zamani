
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Canonical diagnostics and semantic-validation orchestration.
//!
//! Path: src/frontend/semantic/diagnostics/diagnostic.rs
//!
//! PURPOSE
//! =======
//! Define the structured diagnostic contract shared by semantic analysis
//! and orchestrate registered semantic-validation passes.
//!
//! OWNERSHIP
//! =========
//! This module owns:
//! - Diagnostic and DiagnosticSeverity.
//! - DiagnosticCode and DiagnosticLabel.
//! - Structured notes and machine-readable metadata.
//! - DiagnosticBag collection, deduplication, and deterministic ordering.
//! - ValidationPass, ValidationOrchestrator, and ValidationReport.
//!
//! This module does NOT own:
//! - The canonical AST or its source-span implementation.
//! - Name resolution, type inference, effect checking, or domain rules.
//! - AST structural validation.
//! - Target discovery, hardware allocation, scheduling, or lowering.
//! - The legacy analyzer in src/semantic.rs.
//!
//! INTEGRATION
//! ===========
//! - Uses the native AST's canonical Span type.
//! - validation/*.rs implements ValidationPass<Context> for its own passes.
//! - validation/mod.rs owns the validation-module aggregation and registers
//!   the desired passes in the approved pipeline order.
//! - diagnostics/mod.rs re-exports the public diagnostic API.
//! - session.rs and pipeline.rs own analysis-session lifetime and pass order.
//!
//! SCALABILITY
//! ===========
//! No hardware capacity, domain count, diagnostic count, or source-size
//! ceiling is embedded here. Callers may impose configurable operational
//! budgets outside this module. Exhausting such a budget must be represented
//! as incomplete analysis, never as successful validation.
//!
//! DETERMINISM
//! ===========
//! Diagnostic finalization sorts by stable diagnostic content, not hash-map
//! iteration order or memory addresses. Validation passes run in registration
//! order. Pass implementations must themselves be deterministic.
//!
//! STATUS
//! ======
//! Proposed integration file. Its imports and APIs must be checked against
//! the final frontend module exports and validation context before merge.

use std::collections::BTreeSet;
use std::fmt;

use crate::frontend::ast::node::Span;

/// Machine-readable diagnostic identifier.
///
/// Codes are open-world strings rather than a closed enum, so new semantic
/// domains can add diagnostic codes without modifying this module.
///
/// Use stable, qualified identifiers such as:
/// - `semantic::name::unresolved`
/// - `semantic::type::mismatch`
/// - `semantic::effect::unauthorized`
/// - `semantic::resource::invalid_requirement`
/// - `semantic::validation::incomplete`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DiagnosticCode(String);

impl DiagnosticCode {
    /// Construct a diagnostic code.
    ///
    /// Empty and whitespace-only codes are rejected. The code is otherwise
    /// intentionally extensible and is not restricted to a hard-coded list.
    pub fn new(value: impl Into<String>) -> Result<Self, DiagnosticCodeError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(DiagnosticCodeError::Empty);
        }

        if value.chars().any(char::is_control) {
            return Err(DiagnosticCodeError::ControlCharacter);
        }

        Ok(Self(value))
    }

    /// Return the stable textual code.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DiagnosticCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Failure to construct a diagnostic code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiagnosticCodeError {
    Empty,
    ControlCharacter,
}

impl fmt::Display for DiagnosticCodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("diagnostic code must not be empty"),
            Self::ControlCharacter => {
                f.write_str("diagnostic code must not contain control characters")
            }
        }
    }
}

impl std::error::Error for DiagnosticCodeError {}

/// Severity of a diagnostic.
///
/// Severity describes the diagnostic, not whether a compilation pipeline
/// has completed. In particular, an incomplete analysis must not be treated
/// as valid merely because it emitted no error-severity diagnostic.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Information,
    Hint,
}

impl DiagnosticSeverity {
    /// Stable severity ordering used when finalizing diagnostic output.
    const fn sort_rank(self) -> u8 {
        match self {
            Self::Error => 0,
            Self::Warning => 1,
            Self::Information => 2,
            Self::Hint => 3,
        }
    }

    /// Whether this severity represents a semantic error.
    pub const fn is_error(self) -> bool {
        matches!(self, Self::Error)
    }
}

/// A source-labelled diagnostic location.
///
/// `span` uses the AST's canonical span type. Labels may omit a span for
/// diagnostics associated with a whole program, external declaration, or
/// semantic obligation that has no single source location.
#[derive(Clone, Debug)]
pub struct DiagnosticLabel {
    pub span: Option<Span>,
    pub message: String,
    pub primary: bool,
}

impl DiagnosticLabel {
    pub fn primary(span: Span, message: impl Into<String>) -> Self {
        Self {
            span: Some(span),
            message: message.into(),
            primary: true,
        }
    }

    pub fn secondary(span: Span, message: impl Into<String>) -> Self {
        Self {
            span: Some(span),
            message: message.into(),
            primary: false,
        }
    }

    pub fn without_span(message: impl Into<String>, primary: bool) -> Self {
        Self {
            span: None,
            message: message.into(),
            primary,
        }
    }
}

/// One structured semantic diagnostic.
///
/// Human-readable messages are for users; callers must use `code` and
/// structured fields for tooling, tests, filtering, and automation.
#[derive(Clone, Debug)]
pub struct Diagnostic {
    pub code: DiagnosticCode,
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub labels: Vec<DiagnosticLabel>,
    pub notes: Vec<String>,
    pub help: Option<String>,
    pub metadata: Vec<(String, String)>,
}

impl Diagnostic {
    pub fn new(
        code: DiagnosticCode,
        severity: DiagnosticSeverity,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            severity,
            message: message.into(),
            labels: Vec::new(),
            notes: Vec::new(),
            help: None,
            metadata: Vec::new(),
        }
    }

    pub fn error(code: DiagnosticCode, message: impl Into<String>) -> Self {
        Self::new(code, DiagnosticSeverity::Error, message)
    }

    pub fn warning(code: DiagnosticCode, message: impl Into<String>) -> Self {
        Self::new(code, DiagnosticSeverity::Warning, message)
    }

    pub fn information(code: DiagnosticCode, message: impl Into<String>) -> Self {
        Self::new(code, DiagnosticSeverity::Information, message)
    }

    pub fn hint(code: DiagnosticCode, message: impl Into<String>) -> Self {
        Self::new(code, DiagnosticSeverity::Hint, message)
    }

    pub fn with_label(mut self, label: DiagnosticLabel) -> Self {
        self.labels.push(label);
        self
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    /// Add machine-readable metadata.
    ///
    /// Duplicate keys are permitted at construction time to avoid silently
    /// discarding information. Finalization sorts metadata deterministically.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.metadata.push((key.into(), value.into()));
        self
    }

    pub fn is_error(&self) -> bool {
        self.severity.is_error()
    }

    /// Stable sort key derived from diagnostic content.
    ///
    /// Span formatting is used only as a tie-breaker; source-span formatting
    /// must remain deterministic for reproducible diagnostic output.
    fn sort_key(&self) -> String {
        let mut key = format!(
            "{}\u{1f}{}\u{1f}{}",
            self.severity.sort_rank(),
            self.code.as_str(),
            self.message
        );

        for label in &self.labels {
            key.push_str("\u{1e}");
            key.push(if label.primary { 'P' } else { 'S' });
            key.push_str(&label.message);
            key.push('\u{1f}');

            if let Some(span) = &label.span {
                key.push_str(&format!("{span:?}"));
            }
        }

        for note in &self.notes {
            key.push_str("\u{1d}");
            key.push_str(note);
        }

        if let Some(help) = &self.help {
            key.push_str("\u{1c}");
            key.push_str(help);
        }

        for (name, value) in &self.metadata {
            key.push_str("\u{1b}");
            key.push_str(name);
            key.push('\u{1f}');
            key.push_str(value);
        }

        key
    }

    fn normalize(&mut self) {
        self.labels.sort_by_key(|label| {
            (
                !label.primary,
                label.message.clone(),
                label
                    .span
                    .as_ref()
                    .map(|span| format!("{span:?}"))
                    .unwrap_or_default(),
            )
        });

        self.notes.sort();
        self.notes.dedup();

        self.metadata.sort();
        self.metadata.dedup();
    }
}

/// Collection of diagnostics produced during one analysis.
///
/// This collection intentionally has no built-in maximum size. If a compiler
/// configures a diagnostic budget, that budget belongs to analysis
/// configuration and must explicitly mark the analysis incomplete when hit.
#[derive(Clone, Debug, Default)]
pub struct DiagnosticBag {
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticBag {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn extend<I>(&mut self, diagnostics: I)
    where
        I: IntoIterator<Item = Diagnostic>,
    {
        self.diagnostics.extend(diagnostics);
    }

    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(Diagnostic::is_error)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Diagnostic> {
        self.diagnostics.iter()
    }

    /// Consume the bag and return normalized, deterministically ordered
    /// diagnostics with exact duplicate records removed.
    pub fn into_sorted_deduplicated(mut self) -> Vec<Diagnostic> {
        for diagnostic in &mut self.diagnostics {
            diagnostic.normalize();
        }

        self.diagnostics.sort_by_key(Diagnostic::sort_key);

        let mut seen = BTreeSet::new();
        self.diagnostics
            .into_iter()
            .filter(|diagnostic| seen.insert(diagnostic.sort_key()))
            .collect()
    }

    /// Consume the bag without sorting or deduplicating.
    ///
    /// Intended for debugging or callers that need to inspect pass emission
    /// order. Production-facing output should use finalization.
    pub fn into_vec(self) -> Vec<Diagnostic> {
        self.diagnostics
    }
}

/// Stable identifier for a validation pass.
///
/// Pass identifiers must be unique within an orchestrator. They are strings
/// rather than a closed enum so additional semantic domains can be registered
/// without modifying this file.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValidationPassId(String);

impl ValidationPassId {
    pub fn new(value: impl Into<String>) -> Result<Self, ValidationPassIdError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(ValidationPassIdError::Empty);
        }

        if value.chars().any(char::is_control) {
            return Err(ValidationPassIdError::ControlCharacter);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ValidationPassId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValidationPassIdError {
    Empty,
    ControlCharacter,
}

impl fmt::Display for ValidationPassIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("validation pass ID must not be empty"),
            Self::ControlCharacter => {
                f.write_str("validation pass ID must not contain control characters")
            }
        }
    }
}

impl std::error::Error for ValidationPassIdError {}

/// Failure to register a validation pass.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValidationRegistrationError {
    DuplicatePassId(String),
}

impl fmt::Display for ValidationRegistrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicatePassId(id) => {
                write!(f, "validation pass `{id}` is already registered")
            }
        }
    }
}

impl std::error::Error for ValidationRegistrationError {}

/// Failure reported by a validation pass.
///
/// A pass should return an error only when it cannot complete its own work,
/// not for an ordinary semantic violation. Ordinary violations should be
/// appended to the supplied DiagnosticBag so later passes can still run.
#[derive(Clone, Debug)]
pub struct ValidationPassFailure {
    pub diagnostic: Diagnostic,
}

impl ValidationPassFailure {
    pub fn new(diagnostic: Diagnostic) -> Self {
        Self { diagnostic }
    }
}

/// Contract implemented by every semantic validation pass.
///
/// `C` is the shared context selected by the semantic session. The context
/// should expose canonical AST access, configuration, resolved facts, and
/// extension services without giving validators ownership of those systems.
pub trait ValidationPass<C>: Send + Sync {
    fn id(&self) -> ValidationPassId;

    /// Validate the relevant semantic facts and append any findings.
    ///
    /// The pass must not mutate the canonical AST, make target-allocation
    /// decisions, or suppress diagnostics produced by other passes.
    fn validate(
        &self,
        context: &C,
        diagnostics: &mut DiagnosticBag,
    ) -> Result<(), ValidationPassFailure>;
}

/// Outcome of running all registered passes.
#[derive(Clone, Debug)]
pub struct ValidationReport {
    pub diagnostics: Vec<Diagnostic>,
    pub completed_passes: Vec<ValidationPassId>,
    pub failed_passes: Vec<ValidationPassId>,
    pub aborted: bool,
}

impl ValidationReport {
    /// True only when validation completed and no error-severity diagnostic
    /// was emitted. Warnings, information, and hints do not alone fail it.
    pub fn is_success(&self) -> bool {
        !self.aborted
            && self.failed_passes.is_empty()
            && !self.diagnostics.iter().any(Diagnostic::is_error)
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(Diagnostic::is_error)
    }

    pub fn is_complete(&self) -> bool {
        !self.aborted && self.failed_passes.is_empty()
    }
}

/// Orchestrates the registered semantic validation passes.
///
/// This is an orchestration mechanism, not a replacement for `validation/mod.rs`.
/// The validation module should construct and register the authoritative pass
/// sequence. Registration order is execution order and therefore part of the
/// integration contract.
pub struct ValidationOrchestrator<C> {
    passes: Vec<Box<dyn ValidationPass<C>>>,
    registered_ids: BTreeSet<ValidationPassId>,
}

impl<C> Default for ValidationOrchestrator<C> {
    fn default() -> Self {
        Self {
            passes: Vec::new(),
            registered_ids: BTreeSet::new(),
        }
    }
}

impl<C> ValidationOrchestrator<C> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a pass. Duplicate IDs are rejected rather than silently
    /// replacing or running two competing implementations of the same pass.
    pub fn register<P>(
        &mut self,
        pass: P,
    ) -> Result<(), ValidationRegistrationError>
    where
        P: ValidationPass<C> + 'static,
    {
        let id = pass.id();

        if !self.registered_ids.insert(id.clone()) {
            return Err(ValidationRegistrationError::DuplicatePassId(
                id.to_string(),
            ));
        }

        self.passes.push(Box::new(pass));
        Ok(())
    }

    pub fn registered_passes(&self) -> impl Iterator<Item = &ValidationPassId> {
        self.passes.iter().map(|pass| {
            // The ID is owned by the pass contract. Avoid exposing internal
            // trait objects or requiring a second mutable registry.
            // See `pass_ids()` for an owned snapshot.
            let _ = pass;
            unreachable!("use pass_ids() for an owned pass-ID snapshot")
        })
    }

    /// Return pass IDs in execution order.
    pub fn pass_ids(&self) -> Vec<ValidationPassId> {
        self.passes.iter().map(|pass| pass.id()).collect()
    }

    /// Run all passes in registration order.
    ///
    /// A failed pass is recorded and the orchestrator continues with later
    /// passes. This maximizes useful diagnostics while ensuring that a failed
    /// pass can never be mistaken for a successful complete analysis.
    pub fn run(&self, context: &C) -> ValidationReport {
        let mut bag = DiagnosticBag::new();
        let mut completed_passes = Vec::new();
        let mut failed_passes = Vec::new();

        for pass in &self.passes {
            let id = pass.id();

            match pass.validate(context, &mut bag) {
                Ok(()) => completed_passes.push(id),
                Err(failure) => {
                    bag.push(failure.diagnostic);
                    failed_passes.push(id);
                }
            }
        }

        ValidationReport {
            diagnostics: bag.into_sorted_deduplicated(),
            completed_passes,
            failed_passes,
            aborted: false,
        }
    }
}
