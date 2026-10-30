
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani AI Diagnostics
//!
//! File: src/ai/diagnostics.rs
//! Rust: 1.97+
//! Edition: 2021
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provide AI-domain-specific diagnostic identifiers and construction
//! facilities while reusing the canonical Zamani diagnostic architecture.
//!
//! This module reports AI-domain semantic conditions. It does not implement
//! another diagnostic engine.
//!
//! ## AUTHORITY
//!
//! Normative contracts:
//!
//! - grammar/spec/diagnostics.md
//! - grammar/spec/ai.md
//! - grammar/contracts/diagnostics.yaml
//! - grammar/ai/README.md
//! - grammar/ai/ai.g4
//!
//! Canonical implementation:
//!
//! - src/frontend/semantic/diagnostics/
//! - src/frontend/ast/node/source/span.rs
//!
//! ## OWNS
//!
//! - AI-specific diagnostic codes.
//! - AI-specific diagnostic construction.
//! - AI model and operation references.
//! - AI diagnostic origin and applicability metadata.
//! - AI-specific semantic identity metadata.
//! - AI-related source-location descriptions.
//! - AI diagnostic convenience constructors.
//!
//! ## DOES NOT OWN
//!
//! - Canonical diagnostic records.
//! - Diagnostic severity definitions.
//! - Diagnostic categories.
//! - Compiler phases.
//! - Source spans or source maps.
//! - Diagnostic collection.
//! - Diagnostic budgets.
//! - Diagnostic ordering.
//! - Diagnostic deduplication.
//! - Diagnostic rendering.
//! - Parser or lexer error handling.
//! - AI semantic validation.
//! - Model validation.
//! - Tensor validation.
//! - Capability negotiation.
//! - Resource feasibility.
//! - Hardware discovery.
//! - Scheduling or routing.
//! - AI execution.
//! - AI intermediate representations.
//!
//! ## INTEGRATION
//!
//! Canonical diagnostic representation:
//!
//! crate::frontend::semantic::diagnostics::Diagnostic
//!
//! Canonical source location:
//!
//! crate::frontend::ast::node::source::span::Span
//!
//! AI semantic analysis:
//!
//! crate::frontend::semantic::domains::ai
//!
//! AI validators construct diagnostics using this module and return the
//! canonical Diagnostic representation to the semantic pipeline.
//!
//! The semantic session remains responsible for accumulation and budgets.
//!
//! ## SCALABILITY
//!
//! No diagnostic-count, model-count, operation-count, source-size,
//! tensor-rank, parameter-count, accelerator-count or hardware-capacity
//! ceiling is introduced.
//!
//! Collections use dynamically allocated Rust structures.
//!
//! Operational budgets belong to the owning compilation or analysis session.
//! Budget exhaustion must be reported as incomplete analysis.
//!
//! ## POCO-REAF
//!
//! Diagnostic identity is independent of the target executing the program.
//!
//! The same AI semantic condition must retain the same diagnostic code
//! regardless of whether the eventual realization uses a microcontroller,
//! CPU, GPU, FPGA, ASIC, QPU, cluster or future computational substrate.
//!
//! Target infeasibility and runtime failures must be reported by their
//! authoritative downstream diagnostic domains, not disguised as AI
//! source-semantic failures.
//!
//! ## DETERMINISM
//!
//! This module uses:
//!
//! - stable diagnostic codes;
//! - ordered machine-readable metadata;
//! - explicitly ordered related locations;
//! - stable entity identities;
//! - no timestamps;
//! - no random identifiers;
//! - no pointer identities;
//! - no hash-map iteration.
//!
//! ## FREEZE CRITERIA
//!
//! - Canonical diagnostic types are reused.
//! - AI codes follow the normative code format.
//! - Entity references are validated.
//! - Model and operation identities are preserved.
//! - Related source locations are structured.
//! - Diagnostic metadata is deterministic.
//! - Invalid construction returns typed errors.
//! - No target-specific assumptions are introduced.
//! - Unit and integration tests pass.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::frontend::ast::node::source::span::Span;

use crate::frontend::semantic::diagnostics::{
    Diagnostic,
    DiagnosticCategory,
    DiagnosticCode,
    DiagnosticPhase,
    DiagnosticSeverity,
    DiagnosticValue,
    FixIt,
    RecoveryClassification,
};

/// Canonical diagnostic representation used by the AI subsystem.
///
/// This is an alias, not a second diagnostic implementation.
pub type AiDiagnostic = Diagnostic;

/// Stable diagnostic-code prefix for the AI domain.
pub const AI_DIAGNOSTIC_PREFIX: &str = "ZMN-AI-";

/// Canonical AI diagnostic identifiers.
///
/// These identifiers follow the examples already established in
/// grammar/contracts/diagnostics.yaml and grammar/spec/diagnostics.md.
///
/// Codes are open-world. Extensions may introduce additional valid
/// AI-domain identifiers without modifying this module.
///
/// Once published, a code must never be reused for an unrelated condition.
pub mod codes {
    /// An AI model declaration or model structure is invalid.
    pub const MODEL_INVALID: &str = "ZMN-AI-MODEL-INVALID";

    /// An AI inference declaration or invocation is invalid.
    pub const INFERENCE_INVALID: &str = "ZMN-AI-INFERENCE-INVALID";

    /// An AI learning declaration or operation is invalid.
    pub const LEARNING_INVALID: &str = "ZMN-AI-LEARNING-INVALID";

    /// An AI evidence declaration or relationship is invalid.
    pub const EVIDENCE_INVALID: &str = "ZMN-AI-EVIDENCE-INVALID";

    /// AI provenance information is invalid.
    pub const PROVENANCE_INVALID: &str = "ZMN-AI-PROVENANCE-INVALID";

    /// An AI uncertainty representation is invalid.
    pub const UNCERTAINTY_INVALID: &str = "ZMN-AI-UNCERTAINTY-INVALID";

    /// An AI tensor shape is incompatible with an operation.
    pub const SHAPE_MISMATCH: &str = "ZMN-AI-SHAPE-MISMATCH";

    /// An AI value has an incompatible canonical type.
    pub const TYPE_MISMATCH: &str = "ZMN-AI-TYPE-MISMATCH";
}

/// A validated, stable AI diagnostic identifier.
///
/// The representation remains open-world rather than using a closed
/// enumeration of every possible AI feature or operation.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AiDiagnosticCode(String);

impl AiDiagnosticCode {
    /// Construct and validate an AI diagnostic code.
    ///
    /// The required format is:
    ///
    /// ZMN-AI-<CONDITION>
    ///
    /// The condition must begin with an uppercase ASCII letter and may
    /// contain uppercase ASCII letters, digits, underscores and hyphens.
    ///
    /// The spelling is preserved exactly. No normalization or truncation
    /// occurs.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, AiDiagnosticCodeError> {
        let value = value.into();

        let condition = value
            .strip_prefix(AI_DIAGNOSTIC_PREFIX)
            .ok_or(AiDiagnosticCodeError::InvalidPrefix)?;

        if condition.is_empty() {
            return Err(AiDiagnosticCodeError::EmptyCondition);
        }

        let mut bytes = condition.bytes();

        let first = bytes
            .next()
            .ok_or(AiDiagnosticCodeError::EmptyCondition)?;

        if !first.is_ascii_uppercase() {
            return Err(AiDiagnosticCodeError::InvalidConditionStart);
        }

        for byte in bytes {
            if !(byte.is_ascii_uppercase()
                || byte.is_ascii_digit()
                || byte == b'_'
                || byte == b'-')
            {
                return Err(
                    AiDiagnosticCodeError::InvalidCharacter(byte)
                );
            }
        }

        Ok(Self(value))
    }

    /// Return the exact stable diagnostic code.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AiDiagnosticCode {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Errors produced when validating AI diagnostic codes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AiDiagnosticCodeError {
    /// The code does not begin with the canonical AI prefix.
    InvalidPrefix,

    /// The code has no condition component.
    EmptyCondition,

    /// The condition does not begin with an uppercase ASCII letter.
    InvalidConditionStart,

    /// The condition contains an unsupported byte.
    InvalidCharacter(u8),
}

impl fmt::Display for AiDiagnosticCodeError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidPrefix => {
                formatter.write_str(
                    "AI diagnostic code must begin with ZMN-AI-"
                )
            }

            Self::EmptyCondition => {
                formatter.write_str(
                    "AI diagnostic code must contain a condition"
                )
            }

            Self::InvalidConditionStart => {
                formatter.write_str(
                    "AI diagnostic condition must begin with an uppercase ASCII letter"
                )
            }

            Self::InvalidCharacter(byte) => {
                write!(
                    formatter,
                    "invalid byte in AI diagnostic code: {byte:#04x}"
                )
            }
        }
    }
}

impl Error for AiDiagnosticCodeError {}

/// A stable reference to an AI-domain entity.
///
/// This structure does not create model or operation identities.
/// It carries identities supplied by the authoritative AI model,
/// operation or semantic subsystem.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AiEntityReference {
    kind: String,
    identity: String,
}

impl AiEntityReference {
    /// Construct a validated AI entity reference.
    ///
    /// `kind` is extensible. Examples include:
    ///
    /// - model
    /// - operation
    /// - tensor
    /// - dataset
    /// - training
    /// - inference
    /// - agent
    /// - pipeline
    /// - artifact
    ///
    /// The identity must be stable and must not be derived from a memory
    /// address, random value or temporary allocation.
    pub fn new(
        kind: impl Into<String>,
        identity: impl Into<String>,
    ) -> Result<Self, AiEntityReferenceError> {
        let kind = kind.into();
        let identity = identity.into();

        if kind.trim().is_empty() {
            return Err(AiEntityReferenceError::EmptyKind);
        }

        if kind.chars().any(char::is_control) {
            return Err(AiEntityReferenceError::InvalidKind);
        }

        if identity.trim().is_empty() {
            return Err(AiEntityReferenceError::EmptyIdentity);
        }

        if identity.chars().any(char::is_control) {
            return Err(AiEntityReferenceError::InvalidIdentity);
        }

        Ok(Self { kind, identity })
    }

    /// Return the entity kind.
    #[must_use]
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// Return the stable entity identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        &self.identity
    }
}

/// Errors produced when constructing an AI entity reference.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AiEntityReferenceError {
    /// Entity kind is empty.
    EmptyKind,

    /// Entity kind contains control characters.
    InvalidKind,

    /// Entity identity is empty.
    EmptyIdentity,

    /// Entity identity contains control characters.
    InvalidIdentity,
}

impl fmt::Display for AiEntityReferenceError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyKind => {
                formatter.write_str("AI entity kind must not be empty")
            }

            Self::InvalidKind => {
                formatter.write_str(
                    "AI entity kind contains invalid control characters"
                )
            }

            Self::EmptyIdentity => {
                formatter.write_str(
                    "AI entity identity must not be empty"
                )
            }

            Self::InvalidIdentity => {
                formatter.write_str(
                    "AI entity identity contains invalid control characters"
                )
            }
        }
    }
}

impl Error for AiEntityReferenceError {}

/// Origin of an AI-domain diagnostic.
///
/// These classifications preserve the distinction between a user-source
/// condition and an observation made by compiler infrastructure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AiDiagnosticOrigin {
    /// The condition originates in user-authored Zamani source.
    UserSource,

    /// The condition originates in source/environment integration.
    SourceEnvironment,

    /// The condition was identified by the frontend.
    Frontend,

    /// The condition was identified during semantic analysis.
    Semantic,

    /// The condition is a compiler-generated observation.
    Compiler,
}

impl AiDiagnosticOrigin {
    /// Return the stable machine-readable origin.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::UserSource => "USER_SOURCE",
            Self::SourceEnvironment => "SOURCE_ENVIRONMENT",
            Self::Frontend => "FRONTEND",
            Self::Semantic => "SEMANTIC",
            Self::Compiler => "COMPILER",
        }
    }
}

/// Applicability of an AI-domain diagnostic.
///
/// This describes where the reported condition applies.
/// It does not determine target selection or resource feasibility.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AiDiagnosticApplicability {
    /// The condition concerns source structure or source intent.
    Source,

    /// The condition concerns AI-domain semantic validity.
    Semantic,

    /// The condition concerns interaction between computational domains.
    CrossDomain,

    /// The producer cannot establish a more precise classification.
    Unknown,
}

impl AiDiagnosticApplicability {
    /// Return the stable machine-readable applicability.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Source => "SOURCE",
            Self::Semantic => "SEMANTIC",
            Self::CrossDomain => "CROSS_DOMAIN",
            Self::Unknown => "UNKNOWN",
        }
    }
}

/// Role of an additional source location.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AiRelatedLocationRole {
    /// A supporting location.
    Secondary,

    /// Location of the referenced declaration.
    Definition,

    /// Location where an entity is referenced.
    Use,

    /// Previous relevant declaration or state.
    Previous,

    /// Location identifying a required construct.
    Required,

    /// Another location participating in a conflict.
    Conflict,

    /// A location produced by a transformation.
    Derived,

    /// A generated-source location.
    Generated,

    /// An intermediate-representation location.
    Ir,
}

impl AiRelatedLocationRole {
    /// Return the stable role identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Secondary => "SECONDARY",
            Self::Definition => "DEFINITION",
            Self::Use => "USE",
            Self::Previous => "PREVIOUS",
            Self::Required => "REQUIRED",
            Self::Conflict => "CONFLICT",
            Self::Derived => "DERIVED",
            Self::Generated => "GENERATED",
            Self::Ir => "IR",
        }
    }
}

/// An additional AI diagnostic source location.
///
/// The canonical Span is retained without replacing or converting its
/// source identity or UTF-8 byte offsets.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AiRelatedLocation {
    role: AiRelatedLocationRole,
    span: Span,
    message: String,
}

impl AiRelatedLocation {
    /// Construct a related source location.
    #[must_use]
    pub fn new(
        role: AiRelatedLocationRole,
        span: Span,
        message: impl Into<String>,
    ) -> Self {
        Self {
            role,
            span,
            message: message.into(),
        }
    }

    /// Return the location role.
    #[must_use]
    pub const fn role(&self) -> AiRelatedLocationRole {
        self.role
    }

    /// Return the canonical source span.
    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }

    /// Return the explanatory message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Errors produced while constructing an AI diagnostic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AiDiagnosticBuildError {
    /// The primary diagnostic message is empty.
    EmptyMessage,

    /// A related location has no explanatory message.
    EmptyRelatedLocationMessage,

    /// A model reference does not identify a model.
    InvalidModelReference,

    /// An operation reference does not identify an operation.
    InvalidOperationReference,

    /// A machine-data key is invalid.
    InvalidMachineDataKey,

    /// Provenance information is empty.
    EmptyProvenance,
}

impl fmt::Display for AiDiagnosticBuildError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyMessage => {
                formatter.write_str(
                    "AI diagnostic message must not be empty"
                )
            }

            Self::EmptyRelatedLocationMessage => {
                formatter.write_str(
                    "AI related location must have an explanatory message"
                )
            }

            Self::InvalidModelReference => {
                formatter.write_str(
                    "AI model reference must identify a model"
                )
            }

            Self::InvalidOperationReference => {
                formatter.write_str(
                    "AI operation reference must identify an operation"
                )
            }

            Self::InvalidMachineDataKey => {
                formatter.write_str(
                    "AI machine-data key is empty, invalid or reserved"
                )
            }

            Self::EmptyProvenance => {
                formatter.write_str(
                    "AI diagnostic provenance must not be empty"
                )
            }
        }
    }
}

impl Error for AiDiagnosticBuildError {}

/// Builder for the canonical AI diagnostic representation.
///
/// The builder stores AI-specific context and converts it into the existing
/// canonical Diagnostic type when build() succeeds.
///
/// It does not create another diagnostic engine.
#[derive(Clone, Debug)]
pub struct AiDiagnosticBuilder {
    code: AiDiagnosticCode,
    severity: DiagnosticSeverity,
    message: String,
    primary_span: Option<Span>,

    origin: AiDiagnosticOrigin,
    applicability: AiDiagnosticApplicability,

    entity: Option<AiEntityReference>,
    model: Option<AiEntityReference>,
    operation: Option<AiEntityReference>,

    related_locations: Vec<AiRelatedLocation>,
    notes: Vec<String>,
    help: Option<String>,
    fixes: Vec<FixIt>,

    expected: Option<String>,
    actual: Option<String>,

    provenance: Option<String>,
    causes: Vec<AiDiagnosticCode>,

    recovery: RecoveryClassification,

    machine_data: BTreeMap<String, DiagnosticValue>,
}

impl AiDiagnosticBuilder {
    /// Construct an AI diagnostic builder using a validated code.
    #[must_use]
    pub fn new(
        code: AiDiagnosticCode,
        severity: DiagnosticSeverity,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            severity,
            message: message.into(),
            primary_span: None,

            origin: AiDiagnosticOrigin::Semantic,
            applicability: AiDiagnosticApplicability::Semantic,

            entity: None,
            model: None,
            operation: None,

            related_locations: Vec::new(),
            notes: Vec::new(),
            help: None,
            fixes: Vec::new(),

            expected: None,
            actual: None,

            provenance: None,
            causes: Vec::new(),

            recovery: RecoveryClassification::Undetermined,

            machine_data: BTreeMap::new(),
        }
    }

    /// Construct a builder directly from an unvalidated code string.
    ///
    /// Invalid identifiers are returned as errors.
    pub fn try_new(
        code: impl Into<String>,
        severity: DiagnosticSeverity,
        message: impl Into<String>,
    ) -> Result<Self, AiDiagnosticCodeError> {
        let code = AiDiagnosticCode::new(code)?;

        Ok(Self::new(code, severity, message))
    }

    /// Attach the primary canonical source location.
    #[must_use]
    pub fn with_span(mut self, span: Span) -> Self {
        self.primary_span = Some(span);
        self
    }

    /// Set the diagnostic origin.
    #[must_use]
    pub fn with_origin(
        mut self,
        origin: AiDiagnosticOrigin,
    ) -> Self {
        self.origin = origin;
        self
    }

    /// Set diagnostic applicability.
    #[must_use]
    pub fn with_applicability(
        mut self,
        applicability: AiDiagnosticApplicability,
    ) -> Self {
        self.applicability = applicability;
        self
    }

    /// Attach an AI entity reference.
    #[must_use]
    pub fn with_entity(
        mut self,
        entity: AiEntityReference,
    ) -> Self {
        self.entity = Some(entity);
        self
    }

    /// Attach the model associated with the diagnostic.
    ///
    /// The supplied reference must have the entity kind `model`.
    pub fn with_model(
        mut self,
        model: AiEntityReference,
    ) -> Result<Self, AiDiagnosticBuildError> {
        if model.kind() != "model" {
            return Err(
                AiDiagnosticBuildError::InvalidModelReference
            );
        }

        self.model = Some(model);
        Ok(self)
    }

    /// Attach the AI operation associated with the diagnostic.
    ///
    /// The supplied reference must have the entity kind `operation`.
    pub fn with_operation(
        mut self,
        operation: AiEntityReference,
    ) -> Result<Self, AiDiagnosticBuildError> {
        if operation.kind() != "operation" {
            return Err(
                AiDiagnosticBuildError::InvalidOperationReference
            );
        }

        self.operation = Some(operation);
        Ok(self)
    }

    /// Attach an additional source location.
    ///
    /// Related locations retain insertion order.
    #[must_use]
    pub fn with_related_location(
        mut self,
        location: AiRelatedLocation,
    ) -> Self {
        self.related_locations.push(location);
        self
    }

    /// Append an explanatory note.
    #[must_use]
    pub fn with_note(
        mut self,
        note: impl Into<String>,
    ) -> Self {
        self.notes.push(note.into());
        self
    }

    /// Attach remediation guidance.
    #[must_use]
    pub fn with_help(
        mut self,
        help: impl Into<String>,
    ) -> Self {
        self.help = Some(help.into());
        self
    }

    /// Attach a canonical structured fix-it.
    ///
    /// This module never applies the edit to source text.
    #[must_use]
    pub fn with_fix(mut self, fix: FixIt) -> Self {
        self.fixes.push(fix);
        self
    }

    /// Attach structured expected and actual information.
    #[must_use]
    pub fn with_expected_actual(
        mut self,
        expected: impl Into<String>,
        actual: impl Into<String>,
    ) -> Self {
        self.expected = Some(expected.into());
        self.actual = Some(actual.into());
        self
    }

    /// Attach AI-specific provenance information.
    ///
    /// This is a stable reference or description, not a substitute for
    /// the canonical repository provenance infrastructure.
    pub fn with_provenance(
        mut self,
        provenance: impl Into<String>,
    ) -> Result<Self, AiDiagnosticBuildError> {
        let provenance = provenance.into();

        if provenance.trim().is_empty()
            || provenance.chars().any(char::is_control)
        {
            return Err(AiDiagnosticBuildError::EmptyProvenance);
        }

        self.provenance = Some(provenance);
        Ok(self)
    }

    /// Add a related diagnostic code to the causal chain.
    #[must_use]
    pub fn with_cause(
        mut self,
        cause: AiDiagnosticCode,
    ) -> Self {
        self.causes.push(cause);
        self
    }

    /// Set the producer's recovery assessment.
    #[must_use]
    pub fn with_recovery(
        mut self,
        recovery: RecoveryClassification,
    ) -> Self {
        self.recovery = recovery;
        self
    }

    /// Add structured, AI-specific machine-readable data.
    ///
    /// Custom keys are stored under the `ai.custom.` namespace.
    /// Reserved canonical AI metadata cannot be overwritten through this API.
    pub fn with_machine_data(
        mut self,
        key: impl Into<String>,
        value: DiagnosticValue,
    ) -> Result<Self, AiDiagnosticBuildError> {
        let key = key.into();

        if key.trim().is_empty()
            || key.chars().any(char::is_control)
            || key.starts_with("ai.")
        {
            return Err(
                AiDiagnosticBuildError::InvalidMachineDataKey
            );
        }

        self.machine_data.insert(key, value);

        Ok(self)
    }

    /// Build the canonical Zamani diagnostic.
    ///
    /// The resulting object is the repository's existing Diagnostic type.
    ///
    /// AI metadata is preserved through namespaced structured fields.
    pub fn build(
        self,
    ) -> Result<AiDiagnostic, AiDiagnosticBuildError> {
        if self.message.trim().is_empty() {
            return Err(AiDiagnosticBuildError::EmptyMessage);
        }

        if self.related_locations.iter().any(
            |location| location.message().trim().is_empty()
        ) {
            return Err(
                AiDiagnosticBuildError::EmptyRelatedLocationMessage
            );
        }

        let mut diagnostic = Diagnostic::new(
            self.code.as_str(),
            DiagnosticPhase::DomainAnalysis,
            DiagnosticCategory::Domain,
            self.severity,
            self.message,
            self.primary_span,
        )
        .with_recovery(self.recovery)
        .with_metadata("ai.domain", "AI")
        .with_metadata("ai.code_namespace", AI_DIAGNOSTIC_PREFIX)
        .with_metadata("ai.origin", self.origin.as_str())
        .with_metadata(
            "ai.applicability",
            self.applicability.as_str(),
        );

        // Preserve the AI entity identity.
        if let Some(entity) = &self.entity {
            diagnostic = diagnostic
                .with_metadata(
                    "ai.entity.kind",
                    entity.kind().to_owned(),
                )
                .with_metadata(
                    "ai.entity.identity",
                    entity.identity().to_owned(),
                );
        }

        // Preserve model identity independently of display text.
        if let Some(model) = &self.model {
            diagnostic = diagnostic
                .with_metadata(
                    "ai.model.kind",
                    model.kind().to_owned(),
                )
                .with_metadata(
                    "ai.model.identity",
                    model.identity().to_owned(),
                );
        }

        // Preserve operation identity independently of display text.
        if let Some(operation) = &self.operation {
            diagnostic = diagnostic
                .with_metadata(
                    "ai.operation.kind",
                    operation.kind().to_owned(),
                )
                .with_metadata(
                    "ai.operation.identity",
                    operation.identity().to_owned(),
                );
        }

        // Preserve a stable semantic identity for downstream tooling.
        let mut identity_parts = vec![
            self.code.as_str().to_owned()
        ];

        if let Some(entity) = &self.entity {
            identity_parts.push(format!(
                "entity:{}:{}",
                entity.kind(),
                entity.identity()
            ));
        }

        if let Some(model) = &self.model {
            identity_parts.push(format!(
                "model:{}",
                model.identity()
            ));
        }

        if let Some(operation) = &self.operation {
            identity_parts.push(format!(
                "operation:{}",
                operation.identity()
            ));
        }

        diagnostic = diagnostic.with_metadata(
            "ai.semantic_identity",
            identity_parts.join("|"),
        );

        // Preserve expected and actual information structurally.
        if let Some(expected) = self.expected {
            diagnostic = diagnostic.with_metadata(
                "ai.expected",
                expected,
            );
        }

        if let Some(actual) = self.actual {
            diagnostic = diagnostic.with_metadata(
                "ai.actual",
                actual,
            );
        }

        // Preserve provenance references.
        if let Some(provenance) = self.provenance {
            diagnostic = diagnostic.with_metadata(
                "ai.provenance",
                provenance,
            );
        }

        // Preserve causal diagnostic relationships.
        if !self.causes.is_empty() {
            let causes = DiagnosticValue::Sequence(
                self.causes
                    .iter()
                    .map(|cause| {
                        DiagnosticValue::Text(
                            cause.as_str().to_owned()
                        )
                    })
                    .collect(),
            );

            diagnostic = diagnostic.with_metadata_value(
                "ai.cause_codes",
                causes,
            );
        }

        // Preserve related source locations in the canonical label API
        // and in role-aware machine-readable metadata.
        if !self.related_locations.is_empty() {
            let mut related_data = Vec::with_capacity(
                self.related_locations.len()
            );

            for location in &self.related_locations {
                diagnostic = diagnostic.with_label(
                    location.span(),
                    location.message(),
                );

                let mut fields = BTreeMap::new();

                fields.insert(
                    "role".to_owned(),
                    DiagnosticValue::Text(
                        location.role().as_str().to_owned()
                    ),
                );

                fields.insert(
                    "message".to_owned(),
                    DiagnosticValue::Text(
                        location.message().to_owned()
                    ),
                );

                fields.insert(
                    "source_id".to_owned(),
                    DiagnosticValue::Unsigned(
                        u128::from(
                            location.span().source().as_raw()
                        )
                    ),
                );

                fields.insert(
                    "start_byte".to_owned(),
                    DiagnosticValue::Unsigned(
                        u128::from(
                            location.span().start_raw()
                        )
                    ),
                );

                fields.insert(
                    "end_byte".to_owned(),
                    DiagnosticValue::Unsigned(
                        u128::from(
                            location.span().end_raw()
                        )
                    ),
                );

                related_data.push(
                    DiagnosticValue::Object(fields)
                );
            }

            diagnostic = diagnostic.with_metadata_value(
                "ai.related_locations",
                DiagnosticValue::Sequence(related_data),
            );
        }

        // Preserve ordered explanatory notes.
        for note in self.notes {
            diagnostic = diagnostic.with_note(note);
        }

        // Preserve remediation guidance.
        if let Some(help) = self.help {
            diagnostic = diagnostic.with_help(help);
        }

        // Preserve canonical fix-it suggestions.
        for fix in self.fixes {
            diagnostic = diagnostic.with_fix(fix);
        }

        // Preserve extension-specific machine data.
        for (key, value) in self.machine_data {
            diagnostic = diagnostic.with_metadata_value(
                format!("ai.custom.{key}"),
                value,
            );
        }

        Ok(diagnostic)
    }
}

/// Construct a general AI-domain error.
///
/// Use this for an established AI semantic violation, not for a failure
/// belonging to resource negotiation, target realization or runtime.
pub fn error(
    code: impl Into<String>,
    message: impl Into<String>,
    span: Option<Span>,
) -> Result<AiDiagnostic, AiDiagnosticCodeError> {
    let mut builder = AiDiagnosticBuilder::try_new(
        code,
        DiagnosticSeverity::Error,
        message,
    )?;

    if let Some(span) = span {
        builder = builder.with_span(span);
    }

    builder
        .with_origin(AiDiagnosticOrigin::Semantic)
        .with_applicability(AiDiagnosticApplicability::Semantic)
        .with_recovery(RecoveryClassification::Undetermined)
        .build()
        .map_err(|_| {
            // The helper has already supplied a non-empty code.
            // The only remaining build failures concern invalid message
            // data, which callers should prevent before invoking this
            // convenience constructor.
            AiDiagnosticCodeError::EmptyCondition
        })
}

/// Construct a recoverable AI-domain error.
///
/// Recovery is an explicit producer assessment. It is never inferred
/// automatically from severity.
pub fn recoverable_error(
    code: impl Into<String>,
    message: impl Into<String>,
    span: Option<Span>,
) -> Result<AiDiagnostic, AiDiagnosticCodeError> {
    let mut builder = AiDiagnosticBuilder::try_new(
        code,
        DiagnosticSeverity::Error,
        message,
    )?;

    if let Some(span) = span {
        builder = builder.with_span(span);
    }

    builder
        .with_origin(AiDiagnosticOrigin::Semantic)
        .with_applicability(AiDiagnosticApplicability::Semantic)
        .with_recovery(RecoveryClassification::Recoverable)
        .build()
        .map_err(|_| AiDiagnosticCodeError::EmptyCondition)
}

/// Construct an AI-domain warning.
///
/// A warning does not prove target portability or execution feasibility.
pub fn warning(
    code: impl Into<String>,
    message: impl Into<String>,
    span: Option<Span>,
) -> Result<AiDiagnostic, AiDiagnosticCodeError> {
    let mut builder = AiDiagnosticBuilder::try_new(
        code,
        DiagnosticSeverity::Warning,
        message,
    )?;

    if let Some(span) = span {
        builder = builder.with_span(span);
    }

    builder
        .with_origin(AiDiagnosticOrigin::Semantic)
        .with_applicability(AiDiagnosticApplicability::Semantic)
        .with_recovery(RecoveryClassification::Continue)
        .build()
        .map_err(|_| AiDiagnosticCodeError::EmptyCondition)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::ast::node::source::source_id::SourceId;
    use crate::frontend::ast::node::source::span::SourceOffset;

    fn test_span(start: u64, end: u64) -> Span {
        Span::new(
            SourceId::from_raw(1),
            SourceOffset::from_raw(start),
            SourceOffset::from_raw(end),
        )
    }

    fn model_reference() -> AiEntityReference {
        AiEntityReference::new("model", "model.encoder")
            .expect("valid model reference")
    }

    fn operation_reference() -> AiEntityReference {
        AiEntityReference::new(
            "operation",
            "operation.forward",
        )
        .expect("valid operation reference")
    }

    #[test]
    fn canonical_ai_codes_are_valid() {
        for code in [
            codes::MODEL_INVALID,
            codes::INFERENCE_INVALID,
            codes::LEARNING_INVALID,
            codes::EVIDENCE_INVALID,
            codes::PROVENANCE_INVALID,
            codes::UNCERTAINTY_INVALID,
            codes::SHAPE_MISMATCH,
            codes::TYPE_MISMATCH,
        ] {
            assert!(
                AiDiagnosticCode::new(code).is_ok(),
                "invalid canonical code: {code}"
            );
        }
    }

    #[test]
    fn invalid_code_prefix_is_rejected() {
        assert_eq!(
            AiDiagnosticCode::new("ZMN-HDL-INVALID-CONNECTION"),
            Err(AiDiagnosticCodeError::InvalidPrefix),
        );
    }

    #[test]
    fn empty_code_condition_is_rejected() {
        assert_eq!(
            AiDiagnosticCode::new("ZMN-AI-"),
            Err(AiDiagnosticCodeError::EmptyCondition),
        );
    }

    #[test]
    fn invalid_code_characters_are_rejected() {
        assert!(AiDiagnosticCode::new(
            "ZMN-AI-invalid-model"
        ).is_err());

        assert!(AiDiagnosticCode::new(
            "ZMN-AI-MODEL INVALID"
        ).is_err());
    }

    #[test]
    fn entity_references_preserve_identity() {
        let reference = model_reference();

        assert_eq!(reference.kind(), "model");
        assert_eq!(reference.identity(), "model.encoder");
    }

    #[test]
    fn invalid_entity_references_are_rejected() {
        assert_eq!(
            AiEntityReference::new("", "model.encoder"),
            Err(AiEntityReferenceError::EmptyKind),
        );

        assert_eq!(
            AiEntityReference::new("model", ""),
            Err(AiEntityReferenceError::EmptyIdentity),
        );
    }

    #[test]
    fn model_reference_requires_model_kind() {
        let result = AiDiagnosticBuilder::new(
            AiDiagnosticCode::new(codes::MODEL_INVALID)
                .expect("valid code"),
            DiagnosticSeverity::Error,
            "invalid model",
        )
        .with_model(operation_reference());

        assert_eq!(
            result.err(),
            Some(
                AiDiagnosticBuildError::InvalidModelReference
            )
        );
    }

    #[test]
    fn operation_reference_requires_operation_kind() {
        let result = AiDiagnosticBuilder::new(
            AiDiagnosticCode::new(codes::INFERENCE_INVALID)
                .expect("valid code"),
            DiagnosticSeverity::Error,
            "invalid inference operation",
        )
        .with_operation(model_reference());

        assert_eq!(
            result.err(),
            Some(
                AiDiagnosticBuildError::InvalidOperationReference
            )
        );
    }

    #[test]
    fn diagnostic_uses_canonical_representation() {
        let diagnostic = AiDiagnosticBuilder::new(
            AiDiagnosticCode::new(codes::MODEL_INVALID)
                .expect("valid code"),
            DiagnosticSeverity::Error,
            "AI model declaration is invalid",
        )
        .with_span(test_span(10, 20))
        .with_model(model_reference())
        .expect("valid model reference")
        .build()
        .expect("valid diagnostic");

        assert_eq!(
            diagnostic.code().as_str(),
            codes::MODEL_INVALID
        );

        assert_eq!(
            diagnostic.phase(),
            &DiagnosticPhase::DomainAnalysis
        );

        assert_eq!(
            diagnostic.category(),
            &DiagnosticCategory::Domain
        );

        assert_eq!(
            diagnostic.severity(),
            DiagnosticSeverity::Error
        );

        assert_eq!(
            diagnostic.primary_span(),
            Some(test_span(10, 20))
        );

        assert_eq!(
            diagnostic.metadata_value("ai.model.identity"),
            Some(&DiagnosticValue::Text(
                "model.encoder".to_owned()
            ))
        );
    }

    #[test]
    fn model_and_operation_metadata_are_preserved() {
        let diagnostic = AiDiagnosticBuilder::new(
            AiDiagnosticCode::new(codes::INFERENCE_INVALID)
                .expect("valid code"),
            DiagnosticSeverity::Error,
            "inference operation is invalid",
        )
        .with_model(model_reference())
        .expect("valid model")
        .with_operation(operation_reference())
        .expect("valid operation")
        .build()
        .expect("valid diagnostic");

        assert_eq!(
            diagnostic.metadata_value("ai.model.identity"),
            Some(&DiagnosticValue::Text(
                "model.encoder".to_owned()
            ))
        );

        assert_eq!(
            diagnostic.metadata_value("ai.operation.identity"),
            Some(&DiagnosticValue::Text(
                "operation.forward".to_owned()
            ))
        );
    }

    #[test]
    fn related_locations_preserve_roles_and_source_identity() {
        let location = AiRelatedLocation::new(
            AiRelatedLocationRole::Definition,
            test_span(2, 8),
            "model definition is here",
        );

        let diagnostic = AiDiagnosticBuilder::new(
            AiDiagnosticCode::new(codes::MODEL_INVALID)
                .expect("valid code"),
            DiagnosticSeverity::Error,
            "model reference is invalid",
        )
        .with_related_location(location)
        .build()
        .expect("valid diagnostic");

        assert_eq!(diagnostic.labels().len(), 1);

        assert_eq!(
            diagnostic.labels()[0].message(),
            "model definition is here"
        );

        assert!(
            diagnostic.metadata_value("ai.related_locations").is_some()
        );
    }

    #[test]
    fn expected_and_actual_values_are_structured() {
        let diagnostic = AiDiagnosticBuilder::new(
            AiDiagnosticCode::new(codes::SHAPE_MISMATCH)
                .expect("valid code"),
            DiagnosticSeverity::Error,
            "tensor shapes are incompatible",
        )
        .with_expected_actual(
            "symbolic dimension compatible with input",
            "incompatible symbolic dimension",
        )
        .build()
        .expect("valid diagnostic");

        assert_eq!(
            diagnostic.metadata_value("ai.expected"),
            Some(&DiagnosticValue::Text(
                "symbolic dimension compatible with input".to_owned()
            ))
        );

        assert_eq!(
            diagnostic.metadata_value("ai.actual"),
            Some(&DiagnosticValue::Text(
                "incompatible symbolic dimension".to_owned()
            ))
        );
    }

    #[test]
    fn recovery_is_explicit() {
        let diagnostic = AiDiagnosticBuilder::new(
            AiDiagnosticCode::new(codes::MODEL_INVALID)
                .expect("valid code"),
            DiagnosticSeverity::Error,
            "invalid model",
        )
        .with_recovery(RecoveryClassification::Recoverable)
        .build()
        .expect("valid diagnostic");

        assert_eq!(
            diagnostic.recovery(),
            RecoveryClassification::Recoverable
        );

        assert!(diagnostic.is_error());
    }

    #[test]
    fn custom_machine_data_is_namespaced() {
        let diagnostic = AiDiagnosticBuilder::new(
            AiDiagnosticCode::new(codes::MODEL_INVALID)
                .expect("valid code"),
            DiagnosticSeverity::Information,
            "model information",
        )
        .with_machine_data(
            "model.version",
            DiagnosticValue::Text("1.0".to_owned()),
        )
        .expect("valid machine data")
        .build()
        .expect("valid diagnostic");

        assert_eq!(
            diagnostic.metadata_value(
                "ai.custom.model.version"
            ),
            Some(&DiagnosticValue::Text("1.0".to_owned()))
        );
    }

    #[test]
    fn reserved_machine_data_cannot_be_overwritten() {
        let result = AiDiagnosticBuilder::new(
            AiDiagnosticCode::new(codes::MODEL_INVALID)
                .expect("valid code"),
            DiagnosticSeverity::Error,
            "invalid model",
        )
        .with_machine_data(
            "ai.semantic_identity",
            DiagnosticValue::Text("forged".to_owned()),
        );

        assert_eq!(
            result.err(),
            Some(
                AiDiagnosticBuildError::InvalidMachineDataKey
            )
        );
    }

    #[test]
    fn empty_messages_are_rejected() {
        let result = AiDiagnosticBuilder::new(
            AiDiagnosticCode::new(codes::MODEL_INVALID)
                .expect("valid code"),
            DiagnosticSeverity::Error,
            " ",
        )
        .build();

        assert_eq!(
            result.err(),
            Some(AiDiagnosticBuildError::EmptyMessage)
        );
    }

    #[test]
    fn warning_does_not_become_an_error() {
        let diagnostic = warning(
            codes::MODEL_INVALID,
            "model uses a potentially non-portable feature",
            None,
        )
        .expect("valid diagnostic");

        assert_eq!(
            diagnostic.severity(),
            DiagnosticSeverity::Warning
        );

        assert!(!diagnostic.is_error());
    }

    #[test]
    fn diagnostic_ordering_uses_canonical_implementation() {
        let mut diagnostics = vec![
            error(
                codes::MODEL_INVALID,
                "invalid model",
                Some(test_span(20, 30)),
            )
            .expect("valid diagnostic"),

            error(
                codes::INFERENCE_INVALID,
                "invalid inference",
                Some(test_span(2, 5)),
            )
            .expect("valid diagnostic"),
        ];

        crate::frontend::semantic::diagnostics::sort_diagnostics(
            &mut diagnostics
        );

        assert_eq!(
            diagnostics[0].primary_span(),
            Some(test_span(2, 5))
        );
    }
}
