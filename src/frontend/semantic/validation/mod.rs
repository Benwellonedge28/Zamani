
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Semantic Validation Orchestrator
//!
//! Path: `src/frontend/semantic/validation/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: `unsafe` code is forbidden.
//!
//! ## PURPOSE
//!
//! Provide the single orchestration boundary for semantic validation.
//! Coordinate structural, declaration, expression, statement, effect,
//! domain-rule, and semantic-invariant validation through explicit,
//! deterministic pass contracts.
//!
//! ## AUTHORITY
//!
//! The implementation must conform to:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `src/frontend/ast/node/`
//! - `src/frontend/ast/node/validation/`
//!
//! Normative language specifications take precedence over implementation
//! convenience and illustrative examples.
//!
//! ## OWNS
//!
//! - Validation module composition and public exports.
//! - Canonical validation stage ordering.
//! - Registration and uniqueness of validation passes.
//! - Required-pass completeness.
//! - Pass execution, blocking failures, and skipped-pass accounting.
//! - Deterministic aggregation of validation reports.
//! - The guarantee that missing or incomplete validation is never reported
//!   as complete and successful.
//!
//! ## DOES_NOT_OWN
//!
//! - AST nodes, node identities, source spans, or AST traversal algorithms.
//! - Parsing, lexical analysis, or grammar interpretation.
//! - Symbol resolution or the canonical type system.
//! - Effect, capability, resource, contract, or domain-specific algorithms.
//! - Hardware discovery, target feasibility, placement, routing, or scheduling.
//! - Classical IR, `quantum::ir`, HDL IR, optimization, or code generation.
//! - Runtime execution, authorization side effects, or device access.
//!
//! ## INTEGRATION CONTRACT
//!
//! The canonical AST structural validator remains authoritative for AST
//! structure. This module's `structural` module is the semantic-facing adapter.
//!
//! Each validation module retains ownership of its own algorithm and typed
//! input. A `ValidationPass` adapter bridges that API into the common
//! orchestration contract. The adapter must not duplicate the validator.
//!
//! The shared context supplied by the caller contains the canonical AST,
//! normalized facts, policies, registries, and other inputs needed by the
//! registered adapters. This module does not invent a second AST.
//!
//! ## EXECUTION ORDER
//!
//! 1. Structural validation.
//! 2. Declaration validation.
//! 3. Expression validation.
//! 4. Statement validation.
//! 5. Effect validation.
//! 6. Domain-rule validation.
//! 7. Semantic-invariant validation.
//!
//! The order is deterministic. A pass may declare that a failure blocks
//! dependent passes. Independent passes may continue after non-blocking
//! diagnostics, subject to the caller's policy.
//!
//! ## POCO-REAF AND SCALABILITY
//!
//! This module defines no maximum number of CPUs, GPUs, FPGAs, qubits,
//! registers, nodes, devices, tensor dimensions, domains, declarations,
//! or other language-level entities.
//!
//! The orchestrator's work scales with registered passes and their inputs.
//! Actual processing remains bounded by available memory, processing
//! capacity, representable values, and explicit operational budgets.
//!
//! Operational exhaustion must be represented as incomplete analysis,
//! never as proof that the program is valid.
//!
//! ## DETERMINISM
//!
//! Pass ordering, missing-pass reporting, and aggregated issue ordering
//! are deterministic. Pass implementations must themselves avoid
//! nondeterministic semantic results.
//!
//! ## FREEZE CRITERIA
//!
//! 1. Every declared child module exists and builds.
//! 2. Every required stage has a concrete adapter in the production pipeline.
//! 3. The adapters call the canonical validation implementations.
//! 4. Missing stages and incomplete reports cannot yield success.
//! 5. Blocking failures prevent dependent passes from executing.
//! 6. Diagnostics and reports are deterministic.
//! 7. Tests pass with the supported Rust toolchain.
//! 8. The complete frontend and workspace integration builds.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

// =============================================================================
// Authoritative validation modules
// =============================================================================

/// Structural validation adapter for the canonical AST validator.
pub mod structural;

/// Declaration-level semantic validation.
pub mod declarations;

/// Expression graph and expression-type validation.
pub mod expressions;

/// Statement graph and statement-semantic validation.
pub mod statements;

/// Effect compatibility and authorization validation.
pub mod effects;

/// Extensible computational-domain and domain-boundary validation.
pub mod domain_rules;

/// Cross-contract and semantic-model invariant validation.
pub mod invariants;

// =============================================================================
// Stable public convenience exports
// =============================================================================

pub use structural::{
    validate_program_structure,
    validate_structure,
    validate_structure_with_config,
};

pub use declarations::{
    validate_declarations,
    DeclarationDiagnostic,
    DeclarationDiagnosticCode,
    DeclarationForm,
    DeclarationId,
    DeclarationIndex,
    DeclarationKind,
    DeclarationPolicy,
    DeclarationRecord,
    DeclarationValidationResult,
    NamespaceId,
    SourceLocation as DeclarationSourceLocation,
};

pub use expressions::{
    validate_expressions,
    ExpressionDiagnostic,
    ExpressionDiagnosticCode,
    ExpressionId,
    ExpressionNode,
    ExpressionProvider,
    ExpressionStatus,
    ExpressionTypeChecker,
    ExpressionValidationResult,
    TypeCheckIssue,
    TypeCheckOutcome,
    TypeRef,
    SourceLocation as ExpressionSourceLocation,
};

pub use statements::{
    validate_statements,
    AstNodeInfo,
    ChildRole,
    NodeClass,
    StatementCheckIssue,
    StatementChild,
    StatementContext,
    StatementDiagnostic,
    StatementDiagnosticCode,
    StatementId,
    StatementKind,
    StatementNode,
    StatementProvider,
    StatementSemanticChecker,
    StatementValidationResult,
    SourceLocation as StatementSourceLocation,
};

pub use effects::{
    validate_effect_declaration,
    validate_effect_declaration_with,
    validate_effects,
    EffectValidationError,
    EffectValidationReport,
    EffectValidationRequest,
};

pub use domain_rules::{
    validate_domain_rules,
    BoundaryPolicy,
    DomainBoundary,
    DomainRegistration,
    DomainRuleCatalog,
    DomainRuleIdentity,
    DomainRuleIssue,
    DomainRuleIssueKind,
    DomainRuleProgram,
    DomainRulesBudget,
    DomainRulesCompletion,
    DomainRulesReport,
    DomainRulesResult,
    DomainRulesInputError,
    DomainUse,
};

pub use invariants::{
    validate_semantic_invariants,
    InvariantReport,
    InvariantViolation,
    InvariantViolationCode,
};

// =============================================================================
// Stage identity and order
// =============================================================================

/// A canonical stage in semantic validation.
///
/// The order of the variants is intentional and is part of the orchestration
/// contract. Additions must be reviewed for their dependencies and inserted
/// at the correct point rather than dispatched by hash-map iteration order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum ValidationStage {
    /// Validate AST structure before semantic interpretation.
    Structural,

    /// Validate declaration identities and declaration conflicts.
    Declarations,

    /// Validate expression relationships and type-checking outcomes.
    Expressions,

    /// Validate statement relationships and statement rules.
    Statements,

    /// Validate effect declarations, compatibility, and authorization.
    Effects,

    /// Validate domain registrations and cross-domain boundaries.
    DomainRules,

    /// Validate invariants across the completed semantic facts.
    Invariants,
}

impl ValidationStage {
    /// Canonical execution order.
    ///
    /// Keep this list in sync with the enum and the integration tests.
    pub const ORDERED: [Self; 7] = [
        Self::Structural,
        Self::Declarations,
        Self::Expressions,
        Self::Statements,
        Self::Effects,
        Self::DomainRules,
        Self::Invariants,
    ];

    /// Stable identifier suitable for logs, diagnostics, and test fixtures.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Structural => "validation.structural",
            Self::Declarations => "validation.declarations",
            Self::Expressions => "validation.expressions",
            Self::Statements => "validation.statements",
            Self::Effects => "validation.effects",
            Self::DomainRules => "validation.domain_rules",
            Self::Invariants => "validation.invariants",
        }
    }

    /// Return the canonical position of this stage.
    #[must_use]
    pub const fn ordinal(self) -> usize {
        match self {
            Self::Structural => 0,
            Self::Declarations => 1,
            Self::Expressions => 2,
            Self::Statements => 3,
            Self::Effects => 4,
            Self::DomainRules => 5,
            Self::Invariants => 6,
        }
    }
}

impl fmt::Display for ValidationStage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Structured orchestration issues
// =============================================================================

/// Severity of an orchestration-level validation issue.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ValidationIssueSeverity {
    /// Informational issue that does not invalidate the analysis by itself.
    Info,

    /// A warning that requires attention but is not necessarily invalid.
    Warning,

    /// A semantic or structural error.
    Error,

    /// The analysis cannot safely continue or certify a result.
    Fatal,
}

/// Stable category for an orchestration-level issue.
///
/// Validator-specific diagnostic codes remain owned by their originating
/// modules. These categories describe orchestration failures only.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ValidationIssueKind {
    /// A required pass was not registered.
    MissingRequiredPass,

    /// A pass was configured more than once.
    DuplicatePass,

    /// A pass reported invalid input or invalid semantic facts.
    PassFailure,

    /// A pass could not complete its work.
    IncompletePass,

    /// A pass blocked subsequent dependent validation.
    BlockedPass,

    /// A caller attempted to use an invalid orchestration configuration.
    InvalidConfiguration,
}

impl ValidationIssueKind {
    /// Stable identifier for machine-readable reporting.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingRequiredPass => "validation.orchestrator.missing_pass",
            Self::DuplicatePass => "validation.orchestrator.duplicate_pass",
            Self::PassFailure => "validation.orchestrator.pass_failure",
            Self::IncompletePass => "validation.orchestrator.incomplete_pass",
            Self::BlockedPass => "validation.orchestrator.blocked_pass",
            Self::InvalidConfiguration => "validation.orchestrator.invalid_config",
        }
    }
}

/// One structured issue produced by the orchestrator.
///
/// `code` is stable and machine-readable. `message` is for people and must
/// not be used by consumers as a programmatic discriminator.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationIssue {
    /// Canonical stage associated with this issue.
    pub stage: ValidationStage,

    /// Stable issue category.
    pub kind: ValidationIssueKind,

    /// Severity of the issue.
    pub severity: ValidationIssueSeverity,

    /// Stable machine-readable identifier.
    pub code: &'static str,

    /// Human-readable explanation.
    pub message: String,
}

impl ValidationIssue {
    /// Construct an orchestration issue.
    #[must_use]
    pub fn new(
        stage: ValidationStage,
        kind: ValidationIssueKind,
        severity: ValidationIssueSeverity,
        message: impl Into<String>,
    ) -> Self {
        Self {
            stage,
            kind,
            severity,
            code: kind.as_str(),
            message: message.into(),
        }
    }
}

// =============================================================================
// Pass reports
// =============================================================================

/// Result returned by one validation pass.
///
/// `complete` means the pass finished all work required by its input.
/// It does not mean that no semantic errors were found.
///
/// `blocks_dependents` means subsequent passes cannot safely rely on the
/// facts they would normally consume. This is distinct from a recoverable
/// diagnostic that allows further independent checks.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ValidationPassReport {
    complete: bool,
    blocks_dependents: bool,
    issues: Vec<ValidationIssue>,
}

impl ValidationPassReport {
    /// Construct a successful, complete pass report with no issues.
    #[must_use]
    pub const fn success() -> Self {
        Self {
            complete: true,
            blocks_dependents: false,
            issues: Vec::new(),
        }
    }

    /// Construct a completed report that contains issues.
    ///
    /// Use `push_issue` to attach diagnostics. The caller determines whether
    /// any issue should block dependent passes.
    #[must_use]
    pub const fn completed() -> Self {
        Self {
            complete: true,
            blocks_dependents: false,
            issues: Vec::new(),
        }
    }

    /// Construct a report for a pass that did not finish all required work.
    #[must_use]
    pub const fn incomplete() -> Self {
        Self {
            complete: false,
            blocks_dependents: true,
            issues: Vec::new(),
        }
    }

    /// Add an orchestration-level issue.
    pub fn push_issue(&mut self, issue: ValidationIssue) {
        if issue.severity >= ValidationIssueSeverity::Error {
            self.complete = false;
        }

        if issue.severity == ValidationIssueSeverity::Fatal {
            self.blocks_dependents = true;
        }

        self.issues.push(issue);
    }

    /// Mark the pass incomplete.
    ///
    /// Incomplete results must never be interpreted as successful validation.
    pub fn mark_incomplete(&mut self) {
        self.complete = false;
    }

    /// Mark the pass as blocking dependent passes.
    pub fn block_dependents(&mut self) {
        self.blocks_dependents = true;
    }

    /// Return whether all required work completed.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.complete
    }

    /// Return whether dependent stages must be skipped.
    #[must_use]
    pub const fn blocks_dependents(&self) -> bool {
        self.blocks_dependents
    }

    /// Return the issues emitted by this pass.
    #[must_use]
    pub fn issues(&self) -> &[ValidationIssue] {
        &self.issues
    }

    fn normalize(&mut self, stage: ValidationStage) {
        // A pass may report only issues belonging to its own stage.
        // Normalize stage attribution at this boundary to prevent an adapter
        // from accidentally mislabelling its diagnostics.
        for issue in &mut self.issues {
            issue.stage = stage;
        }

        self.issues.sort_by(|left, right| {
            left.code
                .cmp(right.code)
                .then_with(|| left.message.cmp(&right.message))
                .then_with(|| left.severity.cmp(&right.severity))
        });

        if self.issues.iter().any(|issue| {
            issue.severity >= ValidationIssueSeverity::Error
        }) {
            self.complete = false;
        }

        if self.issues.iter().any(|issue| {
            issue.severity == ValidationIssueSeverity::Fatal
        }) {
            self.blocks_dependents = true;
        }
    }
}

// =============================================================================
// Shared pass interface
// =============================================================================

/// Adapter contract implemented by a concrete validation pass.
///
/// `C` is owned by the calling pipeline and normally provides read access to
/// the canonical AST, normalized semantic facts, policies, registries and
/// diagnostic services. A pass may update explicitly designated analysis
/// state through that context.
///
/// Implementations must call their authoritative validation module rather
/// than reimplement its algorithm.
///
/// Passes must not perform hardware discovery, device allocation, code
/// generation, scheduling, routing, or runtime execution.
pub trait ValidationPass<C> {
    /// Return the unique canonical stage handled by this pass.
    fn stage(&self) -> ValidationStage;

    /// Validate the facts available in the shared context.
    ///
    /// A pass must return `incomplete()` if it cannot establish that its
    /// required work completed. Unsupported constructs must not be silently
    /// treated as valid.
    fn run(&self, context: &mut C) -> ValidationPassReport;
}

// =============================================================================
// Registration errors and configuration
// =============================================================================

/// Error returned when a validation pass cannot be registered.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValidationRegistrationError {
    /// Another pass already owns the requested stage.
    DuplicateStage {
        /// Stage whose registration was rejected.
        stage: ValidationStage,
    },

    /// The stage is not part of the canonical stage order.
    UnknownStage {
        /// Stage that was rejected.
        stage: ValidationStage,
    },
}

impl fmt::Display for ValidationRegistrationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateStage { stage } => {
                write!(formatter, "a validation pass is already registered for {stage}")
            }
            Self::UnknownStage { stage } => {
                write!(formatter, "validation stage {stage} is not in the canonical order")
            }
        }
    }
}

impl std::error::Error for ValidationRegistrationError {}

/// Orchestration policy.
///
/// The default requires all canonical stages. A deliberately reduced
/// configuration is allowed for specialized tools, but the resulting report
/// records exactly which stages were not required and which were not run.
///
/// Production compilation should use the default policy unless the compiler
/// specification explicitly authorizes a narrower validation mode.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationPolicy {
    required_stages: BTreeSet<ValidationStage>,

    /// Continue with later non-dependent passes after a non-blocking failure.
    continue_after_non_blocking_failure: bool,
}

impl Default for ValidationPolicy {
    fn default() -> Self {
        Self {
            required_stages: ValidationStage::ORDERED.into_iter().collect(),
            continue_after_non_blocking_failure: true,
        }
    }
}

impl ValidationPolicy {
    /// Construct the production policy requiring every canonical stage.
    #[must_use]
    pub fn production() -> Self {
        Self::default()
    }

    /// Construct a policy with an explicitly selected set of required stages.
    ///
    /// This is intended for tools that validate a well-defined fragment or
    /// partial semantic model. A reduced policy must not be presented as
    /// full-program production validation.
    #[must_use]
    pub fn requiring(
        stages: impl IntoIterator<Item = ValidationStage>,
    ) -> Self {
        Self {
            required_stages: stages.into_iter().collect(),
            continue_after_non_blocking_failure: true,
        }
    }

    /// Require a stage.
    pub fn require_stage(&mut self, stage: ValidationStage) {
        self.required_stages.insert(stage);
    }

    /// Remove a stage from the required set.
    ///
    /// Use only when the caller can prove that the corresponding analysis is
    /// outside the declared validation scope.
    pub fn make_stage_optional(&mut self, stage: ValidationStage) {
        self.required_stages.remove(&stage);
    }

    /// Configure whether later non-blocked stages continue after an error.
    pub fn set_continue_after_non_blocking_failure(&mut self, value: bool) {
        self.continue_after_non_blocking_failure = value;
    }

    /// Return the required stage set.
    #[must_use]
    pub fn required_stages(&self) -> &BTreeSet<ValidationStage> {
        &self.required_stages
    }

    /// Return whether later non-blocked stages continue after an error.
    #[must_use]
    pub const fn continues_after_non_blocking_failure(&self) -> bool {
        self.continue_after_non_blocking_failure
    }
}

// =============================================================================
// Final report
// =============================================================================

/// Execution state of one validation stage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValidationStageState {
    /// A registered pass ran and completed its work.
    Completed,

    /// A registered pass ran but could not complete all required work.
    Incomplete,

    /// The required pass was not registered.
    Missing,

    /// The pass was not run because a previous stage blocked its dependencies.
    SkippedBlocked,

    /// The pass was not run because the policy stopped after an error.
    SkippedPolicy,
}

/// Summary for one stage.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationStageResult {
    /// Stage identity.
    pub stage: ValidationStage,

    /// Stage execution state.
    pub state: ValidationStageState,

    /// Number of issues produced by this stage.
    pub issue_count: usize,
}

/// Aggregated result of the entire validation pipeline.
///
/// A result is successful only if all required stages ran to completion,
/// no required stage is missing, and no error or fatal issue was emitted.
/// Warnings do not, by themselves, invalidate a complete result.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ValidationReport {
    stages: Vec<ValidationStageResult>,
    issues: Vec<ValidationIssue>,
    complete: bool,
    blocked: bool,
}

impl ValidationReport {
    /// Return per-stage outcomes in canonical stage order.
    #[must_use]
    pub fn stages(&self) -> &[ValidationStageResult] {
        &self.stages
    }

    /// Return all orchestration-level issues in deterministic order.
    #[must_use]
    pub fn issues(&self) -> &[ValidationIssue] {
        &self.issues
    }

    /// Return whether all required validation stages completed.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.complete
    }

    /// Return whether any stage blocked dependent analysis.
    #[must_use]
    pub const fn is_blocked(&self) -> bool {
        self.blocked
    }

    /// Return whether the result has no error or fatal issues.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.issues.iter().any(|issue| {
            issue.severity >= ValidationIssueSeverity::Error
        })
    }

    /// Return whether the pipeline completed successfully under its policy.
    #[must_use]
    pub fn is_success(&self) -> bool {
        self.complete && !self.blocked && !self.has_errors()
    }

    /// Return whether a stage completed.
    #[must_use]
    pub fn stage_completed(&self, stage: ValidationStage) -> bool {
        self.stages.iter().any(|result| {
            result.stage == stage
                && result.state == ValidationStageState::Completed
        })
    }

    /// Return the number of stages that actually executed.
    #[must_use]
    pub fn executed_stage_count(&self) -> usize {
        self.stages
            .iter()
            .filter(|result| {
                matches!(
                    result.state,
                    ValidationStageState::Completed
                        | ValidationStageState::Incomplete
                )
            })
            .count()
    }
}

// =============================================================================
// Orchestrator
// =============================================================================

/// Deterministic orchestrator for semantic validation passes.
///
/// The orchestrator owns pass registration and execution order, but not the
/// algorithms implemented by individual validators.
///
/// `C` is the caller's canonical analysis context. The same context is
/// supplied to each executed pass, so outputs from earlier stages can be
/// consumed by later stages without creating a second AST or semantic model.
pub struct ValidationOrchestrator<C> {
    policy: ValidationPolicy,
    passes: BTreeMap<ValidationStage, Box<dyn ValidationPass<C>>>,
}

impl<C> Default for ValidationOrchestrator<C> {
    fn default() -> Self {
        Self::new(ValidationPolicy::production())
    }
}

impl<C> ValidationOrchestrator<C> {
    /// Construct an orchestrator with an explicit policy.
    #[must_use]
    pub fn new(policy: ValidationPolicy) -> Self {
        Self {
            policy,
            passes: BTreeMap::new(),
        }
    }

    /// Return the active orchestration policy.
    #[must_use]
    pub fn policy(&self) -> &ValidationPolicy {
        &self.policy
    }

    /// Register a pass for its canonical stage.
    ///
    /// Registration rejects duplicate stages. Silently replacing a pass
    /// would make semantic results dependent on construction order.
    pub fn register<P>(
        &mut self,
        pass: P,
    ) -> Result<(), ValidationRegistrationError>
    where
        P: ValidationPass<C> + 'static,
    {
        let stage = pass.stage();

        if !ValidationStage::ORDERED.contains(&stage) {
            return Err(ValidationRegistrationError::UnknownStage { stage });
        }

        if self.passes.contains_key(&stage) {
            return Err(ValidationRegistrationError::DuplicateStage { stage });
        }

        self.passes.insert(stage, Box::new(pass));
        Ok(())
    }

    /// Return whether a pass is registered for a stage.
    #[must_use]
    pub fn has_pass(&self, stage: ValidationStage) -> bool {
        self.passes.contains_key(&stage)
    }

    /// Return registered stages in canonical order.
    #[must_use]
    pub fn registered_stages(&self) -> Vec<ValidationStage> {
        ValidationStage::ORDERED
            .into_iter()
            .filter(|stage| self.passes.contains_key(stage))
            .collect()
    }

    /// Execute the pipeline and aggregate every stage result.
    ///
    /// Missing required passes are reported even if a preceding stage blocks
    /// execution. Optional unregistered stages are recorded as missing but
    /// do not make the result incomplete by themselves.
    ///
    /// A blocked stage is not called. Its absence of diagnostics is not
    /// interpreted as successful validation.
    pub fn run(&self, context: &mut C) -> ValidationReport {
        let mut report = ValidationReport {
            complete: true,
            ..ValidationReport::default()
        };

        let mut blocked = false;
        let mut stop_after_error = false;

        for stage in ValidationStage::ORDERED {
            let required = self.policy.required_stages.contains(&stage);
            let Some(pass) = self.passes.get(&stage) else {
                report.stages.push(ValidationStageResult {
                    stage,
                    state: ValidationStageState::Missing,
                    issue_count: usize::from(required),
                });

                if required {
                    report.complete = false;
                    report.issues.push(ValidationIssue::new(
                        stage,
                        ValidationIssueKind::MissingRequiredPass,
                        ValidationIssueSeverity::Fatal,
                        format!(
                            "Required validation pass `{stage}` is not registered."
                        ),
                    ));
                }

                continue;
            };

            if blocked {
                report.stages.push(ValidationStageResult {
                    stage,
                    state: ValidationStageState::SkippedBlocked,
                    issue_count: 0,
                });

                if required {
                    report.complete = false;
                    report.issues.push(ValidationIssue::new(
                        stage,
                        ValidationIssueKind::BlockedPass,
                        ValidationIssueSeverity::Fatal,
                        format!(
                            "Required validation pass `{stage}` was skipped \
                             because an earlier stage blocked dependent analysis."
                        ),
                    ));
                }

                continue;
            }

            if stop_after_error {
                report.stages.push(ValidationStageResult {
                    stage,
                    state: ValidationStageState::SkippedPolicy,
                    issue_count: 0,
                });

                if required {
                    report.complete = false;
                    report.issues.push(ValidationIssue::new(
                        stage,
                        ValidationIssueKind::BlockedPass,
                        ValidationIssueSeverity::Fatal,
                        format!(
                            "Required validation pass `{stage}` was skipped \
                             by the configured stop-after-error policy."
                        ),
                    ));
                }

                continue;
            }

            let mut pass_report = pass.run(context);
            pass_report.normalize(stage);

            let issue_count = pass_report.issues.len();
            let pass_complete = pass_report.complete;
            let pass_blocks = pass_report.blocks_dependents;

            report.issues.extend(pass_report.issues);

            report.stages.push(ValidationStageResult {
                stage,
                state: if pass_complete {
                    ValidationStageState::Completed
                } else {
                    ValidationStageState::Incomplete
                },
                issue_count,
            });

            if required && !pass_complete {
                report.complete = false;
            }

            if pass_blocks {
                blocked = true;
                report.blocked = true;
                report.complete = false;
            }

            if !pass_complete
                && !self.policy.continue_after_non_blocking_failure
            {
                stop_after_error = true;
            }
        }

        // A diagnostic error invalidates success even if a pass completed its
        // own work. Completion and semantic validity are distinct properties.
        if report.has_errors() {
            report.complete = false;
        }

        report.issues.sort_by(|left, right| {
            left.stage
                .cmp(&right.stage)
                .then_with(|| left.code.cmp(right.code))
                .then_with(|| left.message.cmp(&right.message))
                .then_with(|| left.severity.cmp(&right.severity))
        });

        report
    }
}

// =============================================================================
// Unit tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct TestContext {
        executed: Vec<ValidationStage>,
    }

    struct TestPass {
        stage: ValidationStage,
        report: ValidationPassReport,
    }

    impl TestPass {
        fn successful(stage: ValidationStage) -> Self {
            Self {
                stage,
                report: ValidationPassReport::success(),
            }
        }

        fn incomplete(stage: ValidationStage) -> Self {
            Self {
                stage,
                report: ValidationPassReport::incomplete(),
            }
        }
    }

    impl ValidationPass<TestContext> for TestPass {
        fn stage(&self) -> ValidationStage {
            self.stage
        }

        fn run(&self, context: &mut TestContext) -> ValidationPassReport {
            context.executed.push(self.stage);
            self.report.clone()
        }
    }

    fn register_all(
        orchestrator: &mut ValidationOrchestrator<TestContext>,
    ) {
        for stage in ValidationStage::ORDERED {
            orchestrator
                .register(TestPass::successful(stage))
                .expect("each canonical stage should register once");
        }
    }

    #[test]
    fn canonical_order_is_stable() {
        assert_eq!(
            ValidationStage::ORDERED.map(ValidationStage::ordinal),
            [0, 1, 2, 3, 4, 5, 6]
        );
    }

    #[test]
    fn complete_registered_pipeline_succeeds() {
        let mut orchestrator = ValidationOrchestrator::default();
        register_all(&mut orchestrator);

        let mut context = TestContext::default();
        let report = orchestrator.run(&mut context);

        assert!(report.is_complete());
        assert!(!report.is_blocked());
        assert!(report.is_success());
        assert_eq!(
            context.executed,
            ValidationStage::ORDERED.to_vec()
        );
    }

    #[test]
    fn missing_required_pass_never_succeeds() {
        let mut orchestrator = ValidationOrchestrator::default();

        for stage in ValidationStage::ORDERED {
            if stage != ValidationStage::Effects {
                orchestrator
                    .register(TestPass::successful(stage))
                    .expect("unique stage");
            }
        }

        let mut context = TestContext::default();
        let report = orchestrator.run(&mut context);

        assert!(!report.is_complete());
        assert!(!report.is_success());
        assert!(report.issues().iter().any(|issue| {
            issue.stage == ValidationStage::Effects
                && issue.kind == ValidationIssueKind::MissingRequiredPass
        }));
    }

    #[test]
    fn duplicate_registration_is_rejected() {
        let mut orchestrator = ValidationOrchestrator::<TestContext>::default();

        orchestrator
            .register(TestPass::successful(ValidationStage::Structural))
            .expect("first registration");

        let result =
            orchestrator.register(TestPass::successful(ValidationStage::Structural));

        assert!(matches!(
            result,
            Err(ValidationRegistrationError::DuplicateStage {
                stage: ValidationStage::Structural
            })
        ));
    }

    #[test]
    fn incomplete_blocking_pass_skips_dependent_stages() {
        let mut orchestrator = ValidationOrchestrator::<TestContext>::default();

        orchestrator
            .register(TestPass::incomplete(ValidationStage::Structural))
            .expect("structural registration");

        for stage in ValidationStage::ORDERED.into_iter().skip(1) {
            orchestrator
                .register(TestPass::successful(stage))
                .expect("unique stage");
        }

        let mut context = TestContext::default();
        let report = orchestrator.run(&mut context);

        assert!(!report.is_success());
        assert!(report.is_blocked());
        assert_eq!(context.executed, vec![ValidationStage::Structural]);

        assert!(report.stages().iter().skip(1).all(|stage| {
            stage.state == ValidationStageState::SkippedBlocked
        }));
    }

    #[test]
    fn diagnostics_are_normalized_to_their_owning_stage() {
        let mut pass = ValidationPassReport::completed();

        pass.push_issue(ValidationIssue::new(
            ValidationStage::Invariants,
            ValidationIssueKind::PassFailure,
            ValidationIssueSeverity::Warning,
            "test warning",
        ));

        struct WarningPass(ValidationPassReport);

        impl ValidationPass<TestContext> for WarningPass {
            fn stage(&self) -> ValidationStage {
                ValidationStage::Declarations
            }

            fn run(&self, _: &mut TestContext) -> ValidationPassReport {
                self.0.clone()
            }
        }

        let mut orchestrator = ValidationOrchestrator::new(
            ValidationPolicy::requiring([ValidationStage::Declarations]),
        );

        orchestrator
            .register(WarningPass(pass))
            .expect("registration");

        let report = orchestrator.run(&mut TestContext::default());

        assert_eq!(
            report.issues()[0].stage,
            ValidationStage::Declarations
        );
    }

    #[test]
    fn a_warning_does_not_make_a_complete_pass_invalid() {
        let mut pass = ValidationPassReport::completed();

        pass.push_issue(ValidationIssue::new(
            ValidationStage::Structural,
            ValidationIssueKind::PassFailure,
            ValidationIssueSeverity::Warning,
            "non-fatal warning",
        ));

        assert!(pass.is_complete());
        assert!(!pass.blocks_dependents());
    }

    #[test]
    fn reduced_policy_is_explicit_and_deterministic() {
        let policy = ValidationPolicy::requiring([
            ValidationStage::Structural,
            ValidationStage::Expressions,
        ]);

        assert_eq!(
            policy.required_stages(),
            &BTreeSet::from([
                ValidationStage::Structural,
                ValidationStage::Expressions,
            ])
        );
    }
}
