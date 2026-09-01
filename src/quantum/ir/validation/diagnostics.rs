//! Zamani Quantum IR — Validation Diagnostics
//!
//! Production-grade, deterministic, hardware-independent diagnostics for the
//! canonical Zamani Quantum IR validation subsystem.
//!
//! # Path
//!
//! ```text
//! src/quantum/ir/validation/diagnostics.rs
//! ```
//!
//! # Architectural responsibility
//!
//! This module owns the representation and aggregation of validation findings.
//!
//! It answers:
//!
//! > What validation findings were produced, where do they apply, how severe
//! > are they, and how can tooling consume them deterministically?
//!
//! It does NOT:
//!
//! - perform IR validation;
//! - define quantum semantics;
//! - define the canonical error taxonomy;
//! - parse source code;
//! - read source files;
//! - access hardware;
//! - route qubits;
//! - schedule operations;
//! - optimize programs;
//! - simulate quantum states;
//! - execute programs;
//! - perform QEC decoding;
//! - perform network I/O;
//! - print to stdout/stderr;
//! - mutate global state.
//!
//! Validation logic belongs to sibling modules such as:
//!
//! ```text
//! validation.rs
//! validation/structural.rs
//! validation/semantic.rs
//! validation/typing.rs
//! validation/resources.rs
//! validation/control_flow.rs
//! ```
//!
//! This module provides the common diagnostic transport used by those layers.
//!
//! # Canonical error boundary
//!
//! `errors.rs` owns the canonical IR error vocabulary:
//!
//! ```text
//! IrErrorKind
//! IrErrorCode
//! IrErrorSeverity
//! IrError
//! ```
//!
//! This module MUST NOT redefine those concepts.
//!
//! Instead, a `Diagnostic` can contain an `IrError` as its canonical semantic
//! cause while adding structured context such as:
//!
//! - operation identity;
//! - operation ordinal;
//! - logical qubit;
//! - physical qubit;
//! - classical resource;
//! - source-independent IR location;
//! - related location;
//! - notes;
//! - help;
//! - validation stage.
//!
//! This keeps one authoritative error taxonomy throughout the IR.
//!
//! # Canonical qubit identity
//!
//! Logical qubit references use:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! No local `QubitId` is defined here.
//!
//! Physical qubit references use:
//!
//! ```text
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! The distinction between logical and physical identity is preserved.
//!
//! # Scalability
//!
//! Zamani has no architectural maximum number of:
//!
//! - qubits;
//! - operations;
//! - diagnostics;
//! - registers;
//! - resources;
//! - validation stages.
//!
//! Therefore this module contains no semantic constants such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_OPERATIONS
//! MAX_DIAGNOSTICS_SUPPORTED_BY_ZAMANI
//! ```
//!
//! A diagnostic collection MAY be bounded by an explicit caller policy.
//! Such a bound is a security/resource policy, not a language limitation.
//!
//! `DiagnosticLimits::unbounded()` means that this module imposes no diagnostic
//! count/child-count limit beyond the representable host/resource domain.
//!
//! # Determinism
//!
//! Diagnostic insertion is deterministic.
//!
//! Sorting is deterministic and independent of:
//!
//! - HashMap iteration order;
//! - process address layout;
//! - operating-system state;
//! - wall-clock time;
//! - random state;
//! - backend state;
//! - hardware state.
//!
//! `BTree*` collections are used where keyed deterministic ordering is needed.
//!
//! # Trust boundary
//!
//! Validation diagnostics may be generated from:
//!
//! - trusted constructors;
//! - deserialized IR;
//! - generated IR;
//! - optimization passes;
//! - external tools;
//! - cache/replay artifacts;
//! - distributed compilation;
//! - future dialects.
//!
//! Consequently, this module must itself enforce diagnostic collection bounds.
//!
//! A malicious or malformed input must not be able to create an effectively
//! unbounded diagnostic tree merely by causing repeated failures.
//!
//! # Source independence
//!
//! Canonical IR validation must not depend on the frontend source system.
//!
//! Therefore this module uses an IR-native `DiagnosticLocation` instead of
//! importing frontend `SourceSpan`.
//!
//! A frontend or source-aware layer may translate its source span into a
//! diagnostic location externally, but the canonical IR remains independent
//! of source parsing infrastructure.
//!
//! # Integration
//!
//! The intended flow is:
//!
//! ```text
//! canonical IR
//!      │
//!      ▼
//! validation stages
//!      │
//!      ├── structural
//!      ├── typing
//!      ├── semantic
//!      ├── resources
//!      └── control flow
//!      │
//!      ▼
//! DiagnosticReport
//!      │
//!      ├── compiler diagnostics
//!      ├── IDE/tooling diagnostics
//!      ├── test assertions
//!      ├── logs
//!      └── API/serialization adapters
//! ```
//!
//! Rendering is deliberately outside this module.
//!
//! # Rust compatibility
//!
//! Required:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! No external dependency is required.
//!
//! # Integration contract
//!
//! Upstream:
//!
//! ```text
//! quantum::ir::errors
//! quantum::ir::qubit
//! quantum::ir::identity
//! ```
//!
//! Downstream:
//!
//! ```text
//! validation
//! validation::structural
//! validation::semantic
//! validation::typing
//! validation::resources
//! validation::control_flow
//! frontend adapters
//! compiler diagnostics
//! IDE tooling
//! ```
//!
//! This file does not depend on those downstream modules.
//!
//! # Ownership
//!
//! This file owns:
//!
//! - diagnostic limits;
//! - validation-stage identity;
//! - IR-native diagnostic locations;
//! - diagnostic context;
//! - diagnostic entries;
//! - related diagnostics;
//! - diagnostic collections;
//! - deterministic ordering;
//! - error/warning/note counts;
//! - truncation state;
//! - conversion from canonical `IrError`.
//!
//! It does not own:
//!
//! - `IrError` itself;
//! - validation algorithms;
//! - source files;
//! - source maps;
//! - hardware resources;
//! - compiler passes.
//!
//! # Production invariants
//!
//! 1. No unsafe code.
//! 2. No global mutable state.
//! 3. No semantic machine-size limit.
//! 4. No fixed qubit array.
//! 5. No fixed diagnostic count.
//! 6. Explicit diagnostic bounds are caller policy.
//! 7. Canonical qubit identity comes from `qubit.rs`.
//! 8. Canonical error identity comes from `errors.rs`.
//! 9. Diagnostic ordering is deterministic.
//! 10. Diagnostic truncation is explicit.
//! 11. Diagnostics never silently disappear without a truncation marker.
//! 12. Rendering is separate from storage.
//! 13. Machine consumers use structured fields rather than message parsing.
//! 14. Validation remains side-effect free.
//! 15. Diagnostic collection can safely be used independently by concurrent
//!     validation invocations because all state is caller-owned.
//!
//! # No silent failure
//!
//! When a collection limit is reached, the report records truncation state.
//!
//! It must never appear as though all diagnostics were collected when they were
//! not.
//!
//! -----------------------------------------------------------------------------
//! Implementation
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::cmp::Ordering;
use std::fmt;

use super::super::errors::{
    IrError,
    IrErrorCode,
    IrErrorKind,
    IrErrorSeverity,
};
use super::super::qubit::{
    PhysicalQubitId,
    QubitId,
};

// =============================================================================
// Diagnostic limits
// =============================================================================

/// Explicit resource policy for diagnostic collection.
///
/// These values are collection/security limits only.
///
/// They do NOT limit what Zamani can represent.
///
/// Use `DiagnosticLimits::unbounded()` when the caller wants this module to
/// impose no diagnostic collection bound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DiagnosticLimits {
    /// Maximum number of diagnostics stored in one report.
    ///
    /// `None` means unbounded by this module.
    max_diagnostics: Option<usize>,

    /// Maximum number of related locations stored by one diagnostic.
    ///
    /// `None` means unbounded by this module.
    max_related: Option<usize>,

    /// Maximum number of notes stored by one diagnostic.
    ///
    /// `None` means unbounded by this module.
    max_notes: Option<usize>,

    /// Maximum number of help entries stored by one diagnostic.
    ///
    /// `None` means unbounded by this module.
    max_help: Option<usize>,
}

impl DiagnosticLimits {
    /// Creates an explicitly bounded diagnostic policy.
    #[must_use]
    pub const fn new(
        max_diagnostics: usize,
        max_related: usize,
        max_notes: usize,
        max_help: usize,
    ) -> Self {
        Self {
            max_diagnostics: Some(max_diagnostics),
            max_related: Some(max_related),
            max_notes: Some(max_notes),
            max_help: Some(max_help),
        }
    }

    /// Creates a policy with no diagnostic bounds imposed by this module.
    ///
    /// Actual memory availability and allocator/OS limits still apply.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            max_diagnostics: None,
            max_related: None,
            max_notes: None,
            max_help: None,
        }
    }

    /// Creates a conservative production policy.
    ///
    /// This is a deployment fallback only. It is not a Zamani architectural
    /// limit. Compiler/service callers should normally derive explicit values
    /// from their own resource policy.
    #[must_use]
    pub const fn production() -> Self {
        Self::new(1024, 32, 32, 16)
    }

    /// Returns the diagnostic-count limit.
    #[must_use]
    pub const fn max_diagnostics(self) -> Option<usize> {
        self.max_diagnostics
    }

    /// Returns the related-location limit.
    #[must_use]
    pub const fn max_related(self) -> Option<usize> {
        self.max_related
    }

    /// Returns the note limit.
    #[must_use]
    pub const fn max_notes(self) -> Option<usize> {
        self.max_notes
    }

    /// Returns the help-entry limit.
    #[must_use]
    pub const fn max_help(self) -> Option<usize> {
        self.max_help
    }

    /// Changes the diagnostic-count limit.
    #[must_use]
    pub const fn with_max_diagnostics(
        mut self,
        value: Option<usize>,
    ) -> Self {
        self.max_diagnostics = value;
        self
    }

    /// Changes the related-location limit.
    #[must_use]
    pub const fn with_max_related(
        mut self,
        value: Option<usize>,
    ) -> Self {
        self.max_related = value;
        self
    }

    /// Changes the note limit.
    #[must_use]
    pub const fn with_max_notes(
        mut self,
        value: Option<usize>,
    ) -> Self {
        self.max_notes = value;
        self
    }

    /// Changes the help-entry limit.
    #[must_use]
    pub const fn with_max_help(
        mut self,
        value: Option<usize>,
    ) -> Self {
        self.max_help = value;
        self
    }
}

impl Default for DiagnosticLimits {
    fn default() -> Self {
        Self::production()
    }
}

// =============================================================================
// Validation stage
// =============================================================================

/// Identifies the validation layer that produced a diagnostic.
///
/// This is intentionally an open-ended string rather than an enum of every
/// current and future validator.
///
/// Standard values include:
///
/// - `policy`
/// - `structural`
/// - `typing`
/// - `semantic`
/// - `resources`
/// - `control_flow`
/// - `timing`
/// - `pulse`
/// - `extensions`
/// - `whole_program`
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ValidationStage(String);

impl ValidationStage {
    /// Creates a validation stage.
    ///
    /// Empty stage names are rejected.
    pub fn new(value: impl Into<String>) -> Option<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            None
        } else {
            Some(Self(value))
        }
    }

    /// Returns the stage name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Policy validation stage.
    #[must_use]
    pub fn policy() -> Self {
        Self(String::from("policy"))
    }

    /// Structural validation stage.
    #[must_use]
    pub fn structural() -> Self {
        Self(String::from("structural"))
    }

    /// Typing validation stage.
    #[must_use]
    pub fn typing() -> Self {
        Self(String::from("typing"))
    }

    /// Semantic validation stage.
    #[must_use]
    pub fn semantic() -> Self {
        Self(String::from("semantic"))
    }

    /// Resource validation stage.
    #[must_use]
    pub fn resources() -> Self {
        Self(String::from("resources"))
    }

    /// Control-flow validation stage.
    #[must_use]
    pub fn control_flow() -> Self {
        Self(String::from("control_flow"))
    }

    /// Timing validation stage.
    #[must_use]
    pub fn timing() -> Self {
        Self(String::from("timing"))
    }

    /// Pulse validation stage.
    #[must_use]
    pub fn pulse() -> Self {
        Self(String::from("pulse"))
    }

    /// Extension validation stage.
    #[must_use]
    pub fn extensions() -> Self {
        Self(String::from("extensions"))
    }

    /// Whole-program validation stage.
    #[must_use]
    pub fn whole_program() -> Self {
        Self(String::from("whole_program"))
    }
}

impl fmt::Display for ValidationStage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// IR-native location
// =============================================================================

/// Stable location inside canonical IR.
///
/// This deliberately does not depend on frontend source spans.
///
/// A location can identify:
///
/// - a validation-global position;
/// - an operation;
/// - an operand;
/// - a result;
/// - a logical qubit;
/// - a physical qubit;
/// - a classical resource;
/// - a parameter;
/// - an attribute;
/// - a region;
/// - a block.
///
/// Multiple dimensions may be populated simultaneously.
///
/// The structure is intentionally sparse: absent information remains `None`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct DiagnosticLocation {
    /// Stable operation identity, when available.
    operation: Option<u64>,

    /// Operation ordinal within a traversal, when available.
    ///
    /// This is diagnostic context, not semantic identity.
    operation_index: Option<u64>,

    /// Operand ordinal, when available.
    operand_index: Option<u64>,

    /// Result ordinal, when available.
    result_index: Option<u64>,

    /// Logical qubit identity.
    qubit: Option<QubitId>,

    /// Physical qubit identity.
    physical_qubit: Option<PhysicalQubitId>,

    /// Classical resource identity.
    classical_resource: Option<u64>,

    /// Parameter identity.
    parameter: Option<u64>,

    /// Region identity.
    region: Option<u64>,

    /// Block identity.
    block: Option<u64>,

    /// Optional attribute name.
    attribute: Option<String>,
}

impl DiagnosticLocation {
    /// Creates an empty location.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            operation: None,
            operation_index: None,
            operand_index: None,
            result_index: None,
            qubit: None,
            physical_qubit: None,
            classical_resource: None,
            parameter: None,
            region: None,
            block: None,
            attribute: None,
        }
    }

    /// Sets the stable operation identity.
    #[must_use]
    pub const fn with_operation(mut self, operation: u64) -> Self {
        self.operation = Some(operation);
        self
    }

    /// Sets the operation traversal index.
    #[must_use]
    pub const fn with_operation_index(mut self, index: u64) -> Self {
        self.operation_index = Some(index);
        self
    }

    /// Sets the operand index.
    #[must_use]
    pub const fn with_operand_index(mut self, index: u64) -> Self {
        self.operand_index = Some(index);
        self
    }

    /// Sets the result index.
    #[must_use]
    pub const fn with_result_index(mut self, index: u64) -> Self {
        self.result_index = Some(index);
        self
    }

    /// Sets the logical qubit.
    #[must_use]
    pub const fn with_qubit(mut self, qubit: QubitId) -> Self {
        self.qubit = Some(qubit);
        self
    }

    /// Sets the physical qubit.
    #[must_use]
    pub const fn with_physical_qubit(
        mut self,
        qubit: PhysicalQubitId,
    ) -> Self {
        self.physical_qubit = Some(qubit);
        self
    }

    /// Sets the classical resource identity.
    #[must_use]
    pub const fn with_classical_resource(
        mut self,
        resource: u64,
    ) -> Self {
        self.classical_resource = Some(resource);
        self
    }

    /// Sets the parameter identity.
    #[must_use]
    pub const fn with_parameter(mut self, parameter: u64) -> Self {
        self.parameter = Some(parameter);
        self
    }

    /// Sets the region identity.
    #[must_use]
    pub const fn with_region(mut self, region: u64) -> Self {
        self.region = Some(region);
        self
    }

    /// Sets the block identity.
    #[must_use]
    pub const fn with_block(mut self, block: u64) -> Self {
        self.block = Some(block);
        self
    }

    /// Sets the attribute name.
    #[must_use]
    pub fn with_attribute(
        mut self,
        attribute: impl Into<String>,
    ) -> Self {
        self.attribute = Some(attribute.into());
        self
    }

    /// Returns the stable operation identity.
    #[must_use]
    pub const fn operation(&self) -> Option<u64> {
        self.operation
    }

    /// Returns the operation index.
    #[must_use]
    pub const fn operation_index(&self) -> Option<u64> {
        self.operation_index
    }

    /// Returns the operand index.
    #[must_use]
    pub const fn operand_index(&self) -> Option<u64> {
        self.operand_index
    }

    /// Returns the result index.
    #[must_use]
    pub const fn result_index(&self) -> Option<u64> {
        self.result_index
    }

    /// Returns the logical qubit.
    #[must_use]
    pub const fn qubit(&self) -> Option<QubitId> {
        self.qubit
    }

    /// Returns the physical qubit.
    #[must_use]
    pub const fn physical_qubit(&self) -> Option<PhysicalQubitId> {
        self.physical_qubit
    }

    /// Returns the classical resource identity.
    #[must_use]
    pub const fn classical_resource(&self) -> Option<u64> {
        self.classical_resource
    }

    /// Returns the parameter identity.
    #[must_use]
    pub const fn parameter(&self) -> Option<u64> {
        self.parameter
    }

    /// Returns the region identity.
    #[must_use]
    pub const fn region(&self) -> Option<u64> {
        self.region
    }

    /// Returns the block identity.
    #[must_use]
    pub const fn block(&self) -> Option<u64> {
        self.block
    }

    /// Returns the attribute.
    #[must_use]
    pub fn attribute(&self) -> Option<&str> {
        self.attribute.as_deref()
    }

    /// Returns whether the location contains no contextual information.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operation.is_none()
            && self.operation_index.is_none()
            && self.operand_index.is_none()
            && self.result_index.is_none()
            && self.qubit.is_none()
            && self.physical_qubit.is_none()
            && self.classical_resource.is_none()
            && self.parameter.is_none()
            && self.region.is_none()
            && self.block.is_none()
            && self.attribute.is_none()
    }
}

// =============================================================================
// Related location
// =============================================================================

/// A secondary location associated with a diagnostic.
///
/// Examples:
///
/// - an operation conflicting with another operation;
/// - a measurement destination previously defined elsewhere;
/// - a parameter reference whose declaration is relevant;
/// - a duplicate qubit occurrence.
///
/// Related locations do not replace the primary diagnostic location.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RelatedLocation {
    location: DiagnosticLocation,
    message: String,
}

impl RelatedLocation {
    /// Creates a related location.
    #[must_use]
    pub fn new(
        location: DiagnosticLocation,
        message: impl Into<String>,
    ) -> Self {
        Self {
            location,
            message: message.into(),
        }
    }

    /// Returns the location.
    #[must_use]
    pub fn location(&self) -> &DiagnosticLocation {
        &self.location
    }

    /// Returns the explanation.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

// =============================================================================
// Diagnostic
// =============================================================================

/// One structured validation diagnostic.
///
/// A diagnostic is deliberately richer than a bare string while remaining
/// independent from frontend source infrastructure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    severity: IrErrorSeverity,
    kind: IrErrorKind,
    code: IrErrorCode,
    message: String,
    stage: ValidationStage,
    location: Option<DiagnosticLocation>,
    related: Vec<RelatedLocation>,
    notes: Vec<String>,
    help: Vec<String>,
    error: Option<IrError>,
    limits: DiagnosticLimits,
    related_truncated: bool,
    notes_truncated: bool,
    help_truncated: bool,
}

impl Diagnostic {
    /// Creates a diagnostic with the supplied canonical classification.
    #[must_use]
    pub fn new(
        severity: IrErrorSeverity,
        kind: IrErrorKind,
        code: IrErrorCode,
        stage: ValidationStage,
        message: impl Into<String>,
        limits: DiagnosticLimits,
    ) -> Self {
        Self {
            severity,
            kind,
            code,
            message: message.into(),
            stage,
            location: None,
            related: Vec::new(),
            notes: Vec::new(),
            help: Vec::new(),
            error: None,
            limits,
            related_truncated: false,
            notes_truncated: false,
            help_truncated: false,
        }
    }

    /// Creates a diagnostic from a canonical `IrError`.
    ///
    /// The canonical error remains available through `error()`.
    #[must_use]
    pub fn from_error(
        error: IrError,
        stage: ValidationStage,
        limits: DiagnosticLimits,
    ) -> Self {
        let severity = error.severity();
        let kind = error.kind();
        let code = error.code();
        let message = error.to_string();

        Self {
            severity,
            kind,
            code,
            message,
            stage,
            location: None,
            related: Vec::new(),
            notes: Vec::new(),
            help: Vec::new(),
            error: Some(error),
            limits,
            related_truncated: false,
            notes_truncated: false,
            help_truncated: false,
        }
    }

    /// Returns the severity.
    #[must_use]
    pub const fn severity(&self) -> IrErrorSeverity {
        self.severity
    }

    /// Returns the error category.
    #[must_use]
    pub const fn kind(&self) -> IrErrorKind {
        self.kind
    }

    /// Returns the stable machine-readable code.
    #[must_use]
    pub const fn code(&self) -> IrErrorCode {
        self.code
    }

    /// Returns the human-readable message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the validation stage.
    #[must_use]
    pub fn stage(&self) -> &ValidationStage {
        &self.stage
    }

    /// Returns the primary IR location.
    #[must_use]
    pub fn location(&self) -> Option<&DiagnosticLocation> {
        self.location.as_ref()
    }

    /// Returns the canonical source error, when one exists.
    #[must_use]
    pub fn error(&self) -> Option<&IrError> {
        self.error.as_ref()
    }

    /// Returns related locations.
    #[must_use]
    pub fn related(&self) -> &[RelatedLocation] {
        &self.related
    }

    /// Returns notes.
    #[must_use]
    pub fn notes(&self) -> &[String] {
        &self.notes
    }

    /// Returns help entries.
    #[must_use]
    pub fn help(&self) -> &[String] {
        &self.help
    }

    /// Returns whether related locations were truncated.
    #[must_use]
    pub const fn related_truncated(&self) -> bool {
        self.related_truncated
    }

    /// Returns whether notes were truncated.
    #[must_use]
    pub const fn notes_truncated(&self) -> bool {
        self.notes_truncated
    }

    /// Returns whether help entries were truncated.
    #[must_use]
    pub const fn help_truncated(&self) -> bool {
        self.help_truncated
    }

    /// Sets the primary IR location.
    pub fn set_location(&mut self, location: DiagnosticLocation) {
        self.location = Some(location);
    }

    /// Sets the operation location.
    pub fn set_operation(&mut self, operation: u64) {
        let location = self
            .location
            .take()
            .unwrap_or_else(DiagnosticLocation::new)
            .with_operation(operation);

        self.location = Some(location);
    }

    /// Sets the logical-qubit location.
    pub fn set_qubit(&mut self, qubit: QubitId) {
        let location = self
            .location
            .take()
            .unwrap_or_else(DiagnosticLocation::new)
            .with_qubit(qubit);

        self.location = Some(location);
    }

    /// Sets the operation traversal index.
    pub fn set_operation_index(&mut self, index: u64) {
        let location = self
            .location
            .take()
            .unwrap_or_else(DiagnosticLocation::new)
            .with_operation_index(index);

        self.location = Some(location);
    }

    /// Adds a related location.
    ///
    /// Returns `true` if stored, `false` if the configured limit was reached.
    pub fn add_related(&mut self, related: RelatedLocation) -> bool {
        if let Some(limit) = self.limits.max_related {
            if self.related.len() >= limit {
                self.related_truncated = true;
                return false;
            }
        }

        self.related.push(related);
        true
    }

    /// Adds a note.
    ///
    /// Returns `true` if stored, `false` if the configured limit was reached.
    pub fn add_note(&mut self, note: impl Into<String>) -> bool {
        if let Some(limit) = self.limits.max_notes {
            if self.notes.len() >= limit {
                self.notes_truncated = true;
                return false;
            }
        }

        self.notes.push(note.into());
        true
    }

    /// Adds a help entry.
    ///
    /// Returns `true` if stored, `false` if the configured limit was reached.
    pub fn add_help(&mut self, help: impl Into<String>) -> bool {
        if let Some(limit) = self.limits.max_help {
            if self.help.len() >= limit {
                self.help_truncated = true;
                return false;
            }
        }

        self.help.push(help.into());
        true
    }

    /// Returns the number of explanatory children.
    #[must_use]
    pub fn child_count(&self) -> usize {
        self.related
            .len()
            .saturating_add(self.notes.len())
            .saturating_add(self.help.len())
    }

    /// Returns whether this diagnostic has no additional context.
    #[must_use]
    pub fn is_bare(&self) -> bool {
        self.location.is_none()
            && self.related.is_empty()
            && self.notes.is_empty()
            && self.help.is_empty()
    }

    /// Returns a deterministic ordering key.
    #[must_use]
    pub fn sort_key(&self) -> DiagnosticSortKey {
        DiagnosticSortKey::from_diagnostic(self)
    }
}

// =============================================================================
// Diagnostic sort key
// =============================================================================

/// Deterministic diagnostic ordering key.
///
/// Ordering priority:
///
/// 1. diagnostics with an IR location;
/// 2. operation index;
/// 3. operation identity;
/// 4. qubit identity;
/// 5. physical-qubit identity;
/// 6. operand/result indices;
/// 7. validation stage;
/// 8. severity;
/// 9. error code;
/// 10. message.
///
/// The exact ordering is an API contract and is independent of insertion
/// order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticSortKey {
    has_location: bool,
    operation_index: Option<u64>,
    operation: Option<u64>,
    qubit: Option<QubitId>,
    physical_qubit: Option<PhysicalQubitId>,
    operand_index: Option<u64>,
    result_index: Option<u64>,
    region: Option<u64>,
    block: Option<u64>,
    stage: ValidationStage,
    severity: IrErrorSeverity,
    kind: IrErrorKind,
    code: IrErrorCode,
    message: String,
}

impl DiagnosticSortKey {
    fn from_diagnostic(diagnostic: &Diagnostic) -> Self {
        let location = diagnostic.location.as_ref();

        Self {
            has_location: location.is_some(),
            operation_index: location.and_then(
                DiagnosticLocation::operation_index,
            ),
            operation: location.and_then(DiagnosticLocation::operation),
            qubit: location.and_then(DiagnosticLocation::qubit),
            physical_qubit: location
                .and_then(DiagnosticLocation::physical_qubit),
            operand_index: location.and_then(
                DiagnosticLocation::operand_index,
            ),
            result_index: location.and_then(
                DiagnosticLocation::result_index,
            ),
            region: location.and_then(DiagnosticLocation::region),
            block: location.and_then(DiagnosticLocation::block),
            stage: diagnostic.stage.clone(),
            severity: diagnostic.severity,
            kind: diagnostic.kind,
            code: diagnostic.code,
            message: diagnostic.message.clone(),
        }
    }
}

impl Ord for DiagnosticSortKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.has_location
            .cmp(&other.has_location)
            .reverse()
            .then_with(|| {
                option_cmp_u64(
                    self.operation_index,
                    other.operation_index,
                )
            })
            .then_with(|| {
                option_cmp_u64(self.operation, other.operation)
            })
            .then_with(|| {
                option_cmp_qubit(self.qubit, other.qubit)
            })
            .then_with(|| {
                option_cmp_physical_qubit(
                    self.physical_qubit,
                    other.physical_qubit,
                )
            })
            .then_with(|| {
                option_cmp_u64(
                    self.operand_index,
                    other.operand_index,
                )
            })
            .then_with(|| {
                option_cmp_u64(
                    self.result_index,
                    other.result_index,
                )
            })
            .then_with(|| option_cmp_u64(self.region, other.region))
            .then_with(|| option_cmp_u64(self.block, other.block))
            .then_with(|| self.stage.cmp(&other.stage))
            .then_with(|| self.severity.cmp(&other.severity).reverse())
            .then_with(|| self.kind.cmp(&other.kind))
            .then_with(|| self.code.cmp(&other.code))
            .then_with(|| self.message.cmp(&other.message))
    }
}

impl PartialOrd for DiagnosticSortKey {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn option_cmp_u64(
    left: Option<u64>,
    right: Option<u64>,
) -> Ordering {
    match (left, right) {
        (Some(left), Some(right)) => left.cmp(&right),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}

fn option_cmp_qubit(
    left: Option<QubitId>,
    right: Option<QubitId>,
) -> Ordering {
    match (left, right) {
        (Some(left), Some(right)) => left.cmp(&right),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}

fn option_cmp_physical_qubit(
    left: Option<PhysicalQubitId>,
    right: Option<PhysicalQubitId>,
) -> Ordering {
    match (left, right) {
        (Some(left), Some(right)) => left.cmp(&right),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}

// =============================================================================
// Diagnostic report
// =============================================================================

/// Deterministic collection of validation diagnostics.
///
/// The report preserves insertion order for streaming consumers while also
/// providing a deterministic sorted view.
///
/// A report does not render, print, serialize, or mutate the IR.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticReport {
    diagnostics: Vec<Diagnostic>,
    limits: DiagnosticLimits,
    diagnostics_truncated: bool,
    attempted_diagnostics: u64,
}

impl DiagnosticReport {
    /// Creates an empty report with the supplied collection policy.
    #[must_use]
    pub fn new(limits: DiagnosticLimits) -> Self {
        Self {
            diagnostics: Vec::new(),
            limits,
            diagnostics_truncated: false,
            attempted_diagnostics: 0,
        }
    }

    /// Creates an empty production report.
    #[must_use]
    pub fn production() -> Self {
        Self::new(DiagnosticLimits::production())
    }

    /// Creates an unbounded report.
    ///
    /// The absence of a diagnostic policy bound does not remove actual host
    /// memory/address-space constraints.
    #[must_use]
    pub fn unbounded() -> Self {
        Self::new(DiagnosticLimits::unbounded())
    }

    /// Returns the report's limits.
    #[must_use]
    pub const fn limits(&self) -> DiagnosticLimits {
        self.limits
    }

    /// Returns diagnostics in insertion order.
    #[must_use]
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// Returns the number of stored diagnostics.
    #[must_use]
    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }

    /// Returns whether no diagnostics are stored.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    /// Returns whether the collection was truncated.
    #[must_use]
    pub const fn diagnostics_truncated(&self) -> bool {
        self.diagnostics_truncated
    }

    /// Returns the total number of attempted diagnostic insertions.
    ///
    /// The value saturates instead of wrapping on overflow.
    #[must_use]
    pub const fn attempted_diagnostics(&self) -> u64 {
        self.attempted_diagnostics
    }

    /// Adds a diagnostic.
    ///
    /// Returns `true` when stored and `false` when rejected by the collection
    /// limit.
    pub fn push(&mut self, diagnostic: Diagnostic) -> bool {
        self.attempted_diagnostics =
            self.attempted_diagnostics.saturating_add(1);

        if let Some(limit) = self.limits.max_diagnostics {
            if self.diagnostics.len() >= limit {
                self.diagnostics_truncated = true;
                return false;
            }
        }

        self.diagnostics.push(diagnostic);
        true
    }

    /// Adds a canonical IR error as a diagnostic.
    pub fn push_error(
        &mut self,
        error: IrError,
        stage: ValidationStage,
    ) -> bool {
        let diagnostic =
            Diagnostic::from_error(error, stage, self.limits);

        self.push(diagnostic)
    }

    /// Returns the first error diagnostic.
    #[must_use]
    pub fn first_error(&self) -> Option<&Diagnostic> {
        self.diagnostics
            .iter()
            .find(|diagnostic| {
                diagnostic.severity() == IrErrorSeverity::Error
            })
    }

    /// Returns the number of fatal error diagnostics.
    #[must_use]
    pub fn error_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|diagnostic| {
                diagnostic.severity() == IrErrorSeverity::Error
            })
            .count()
    }

    /// Returns the number of warnings.
    #[must_use]
    pub fn warning_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|diagnostic| {
                diagnostic.severity() == IrErrorSeverity::Warning
            })
            .count()
    }

    /// Returns the number of informational diagnostics.
    #[must_use]
    pub fn info_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|diagnostic| {
                diagnostic.severity() == IrErrorSeverity::Info
            })
            .count()
    }

    /// Returns whether the report contains at least one fatal error.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.error_count() != 0
    }

    /// Returns whether the report contains warnings.
    #[must_use]
    pub fn has_warnings(&self) -> bool {
        self.warning_count() != 0
    }

    /// Returns diagnostics in deterministic validation/source-oriented order.
    ///
    /// A new vector is returned so callers cannot mutate the report's insertion
    /// order.
    #[must_use]
    pub fn sorted(&self) -> Vec<&Diagnostic> {
        let mut sorted: Vec<&Diagnostic> =
            self.diagnostics.iter().collect();

        sorted.sort_by(|left, right| {
            left.sort_key().cmp(&right.sort_key())
        });

        sorted
    }

    /// Returns an owned deterministic diagnostic vector.
    #[must_use]
    pub fn sorted_owned(&self) -> Vec<Diagnostic> {
        let mut sorted = self.diagnostics.clone();

        sorted.sort_by(|left, right| {
            left.sort_key().cmp(&right.sort_key())
        });

        sorted
    }

    /// Removes all diagnostics while retaining the report policy.
    pub fn clear(&mut self) {
        self.diagnostics.clear();
        self.diagnostics_truncated = false;
        self.attempted_diagnostics = 0;
    }

    /// Returns an iterator over insertion-order diagnostics.
    pub fn iter(&self) -> std::slice::Iter<'_, Diagnostic> {
        self.diagnostics.iter()
    }
}

impl Default for DiagnosticReport {
    fn default() -> Self {
        Self::production()
    }
}

impl IntoIterator for DiagnosticReport {
    type Item = Diagnostic;
    type IntoIter = std::vec::IntoIter<Diagnostic>;

    fn into_iter(self) -> Self::IntoIter {
        self.diagnostics.into_iter()
    }
}

impl<'a> IntoIterator for &'a DiagnosticReport {
    type Item = &'a Diagnostic;
    type IntoIter = std::slice::Iter<'a, Diagnostic>;

    fn into_iter(self) -> Self::IntoIter {
        self.diagnostics.iter()
    }
}

// =============================================================================
// Convenience constructors
// =============================================================================

/// Creates an error diagnostic from an `IrError`.
#[must_use]
pub fn diagnostic_from_error(
    error: IrError,
    stage: ValidationStage,
    limits: DiagnosticLimits,
) -> Diagnostic {
    Diagnostic::from_error(error, stage, limits)
}

/// Creates a production diagnostic report.
#[must_use]
pub fn production_report() -> DiagnosticReport {
    DiagnosticReport::production()
}

/// Creates an unbounded diagnostic report.
#[must_use]
pub fn unbounded_report() -> DiagnosticReport {
    DiagnosticReport::unbounded()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn code(value: u32) -> IrErrorCode {
        IrErrorCode::new(value).expect("non-zero diagnostic code")
    }

    #[test]
    fn empty_location_is_empty() {
        let location = DiagnosticLocation::new();

        assert!(location.is_empty());
    }

    #[test]
    fn qubit_location_uses_canonical_qubit_type() {
        let qubit = QubitId::new(7);

        let location =
            DiagnosticLocation::new().with_qubit(qubit);

        assert_eq!(location.qubit(), Some(qubit));
        assert!(!location.is_empty());
    }

    #[test]
    fn diagnostic_preserves_canonical_error_information() {
        let error = IrError::InvalidStructure {
            message: "invalid test structure",
        };

        let diagnostic = Diagnostic::from_error(
            error,
            ValidationStage::structural(),
            DiagnosticLimits::production(),
        );

        assert_eq!(
            diagnostic.kind(),
            IrErrorKind::Validation
        );
        assert_eq!(
            diagnostic.severity(),
            IrErrorSeverity::Error
        );
    }

    #[test]
    fn related_locations_are_bounded() {
        let limits = DiagnosticLimits::new(10, 1, 1, 1);

        let mut diagnostic = Diagnostic::new(
            IrErrorSeverity::Error,
            IrErrorKind::Validation,
            code(1),
            ValidationStage::semantic(),
            "test",
            limits,
        );

        assert!(diagnostic.add_related(
            RelatedLocation::new(
                DiagnosticLocation::new(),
                "first",
            ),
        ));

        assert!(!diagnostic.add_related(
            RelatedLocation::new(
                DiagnosticLocation::new(),
                "second",
            ),
        ));

        assert!(diagnostic.related_truncated());
        assert_eq!(diagnostic.related().len(), 1);
    }

    #[test]
    fn report_is_bounded_and_marks_truncation() {
        let limits = DiagnosticLimits::new(1, 1, 1, 1);
        let mut report = DiagnosticReport::new(limits);

        let first = Diagnostic::new(
            IrErrorSeverity::Error,
            IrErrorKind::Validation,
            code(1),
            ValidationStage::semantic(),
            "first",
            limits,
        );

        let second = Diagnostic::new(
            IrErrorSeverity::Error,
            IrErrorKind::Validation,
            code(2),
            ValidationStage::semantic(),
            "second",
            limits,
        );

        assert!(report.push(first));
        assert!(!report.push(second));

        assert_eq!(report.len(), 1);
        assert!(report.diagnostics_truncated());
        assert_eq!(report.attempted_diagnostics(), 2);
    }

    #[test]
    fn report_counts_severities() {
        let limits = DiagnosticLimits::unbounded();
        let mut report = DiagnosticReport::unbounded();

        assert!(report.push(Diagnostic::new(
            IrErrorSeverity::Error,
            IrErrorKind::Validation,
            code(1),
            ValidationStage::structural(),
            "error",
            limits,
        )));

        assert!(report.push(Diagnostic::new(
            IrErrorSeverity::Warning,
            IrErrorKind::Validation,
            code(2),
            ValidationStage::semantic(),
            "warning",
            limits,
        )));

        assert!(report.push(Diagnostic::new(
            IrErrorSeverity::Info,
            IrErrorKind::Validation,
            code(3),
            ValidationStage::typing(),
            "info",
            limits,
        )));

        assert_eq!(report.error_count(), 1);
        assert_eq!(report.warning_count(), 1);
        assert_eq!(report.info_count(), 1);
        assert!(report.has_errors());
        assert!(report.has_warnings());
    }

    #[test]
    fn sorted_order_is_deterministic() {
        let limits = DiagnosticLimits::unbounded();
        let mut report = DiagnosticReport::unbounded();

        let late = Diagnostic::new(
            IrErrorSeverity::Error,
            IrErrorKind::Validation,
            code(2),
            ValidationStage::semantic(),
            "late",
            limits,
        );

        let mut early = Diagnostic::new(
            IrErrorSeverity::Error,
            IrErrorKind::Validation,
            code(1),
            ValidationStage::structural(),
            "early",
            limits,
        );

        early.set_location(
            DiagnosticLocation::new().with_operation_index(1),
        );

        let mut later = late;
        later.set_location(
            DiagnosticLocation::new().with_operation_index(2),
        );

        report.push(later);
        report.push(early);

        let sorted = report.sorted();

        assert_eq!(
            sorted[0].message(),
            "early"
        );
        assert_eq!(
            sorted[1].message(),
            "late"
        );
    }

    #[test]
    fn diagnostic_can_store_help_and_notes() {
        let limits = DiagnosticLimits::new(10, 10, 1, 1);

        let mut diagnostic = Diagnostic::new(
            IrErrorSeverity::Error,
            IrErrorKind::Validation,
            code(10),
            ValidationStage::structural(),
            "invalid operation",
            limits,
        );

        assert!(diagnostic.add_note("operation is malformed"));
        assert!(!diagnostic.add_note("second note"));

        assert!(diagnostic.add_help("check the operand count"));
        assert!(!diagnostic.add_help("second help"));

        assert!(diagnostic.notes_truncated());
        assert!(diagnostic.help_truncated());
    }

    #[test]
    fn report_clear_resets_state() {
        let mut report = DiagnosticReport::unbounded();

        report.push(Diagnostic::new(
            IrErrorSeverity::Error,
            IrErrorKind::Validation,
            code(11),
            ValidationStage::semantic(),
            "failure",
            DiagnosticLimits::unbounded(),
        ));

        assert!(!report.is_empty());

        report.clear();

        assert!(report.is_empty());
        assert_eq!(report.error_count(), 0);
        assert_eq!(report.attempted_diagnostics(), 0);
        assert!(!report.diagnostics_truncated());
    }
}