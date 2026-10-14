
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Public API for Zamani's native frontend semantic analysis.
//!
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # PURPOSE
//!
//! Define the stable boundary between the canonical frontend AST and the
//! semantic-analysis implementation. The API owns the service contract,
//! result lifecycle, validity gates, and API-level error reporting.
//!
//! # AUTHORITY
//!
//! Normative references:
//! - grammar/DESIGN.md
//! - grammar/Zamani.g4
//! - grammar/specification/semantic-model.md
//! - grammar/specification/semantic-boundaries.md
//! - grammar/specification/poco-reaf.md
//!
//! The canonical AST owns source structure. Semantic analysis establishes
//! program meaning. Canonical IR and target-specific systems determine
//! realization.
//!
//! # OWNERSHIP
//!
//! This module owns:
//! - SemanticService and its public invocation contract;
//! - SemanticResult and SemanticStatus;
//! - validity gates for completed results;
//! - SemanticDiagnostic and SemanticModelState contracts;
//! - SemanticApiError.
//!
//! It does not own parsing, AST storage, name resolution, type inference,
//! effect checking, capability checking, resource planning, domain rules,
//! diagnostics production, IR construction, scheduling, routing, or runtime.
//!
//! # POCO-REAF
//!
//! No hardware capacity or closed computational-domain list is encoded here.
//! Operational budgets are owned by semantic configuration and enforced by
//! the implementation. Exhausted budgets must produce an incomplete result.
//!
//! No finite implementation can process literally infinite input. The API
//! introduces no arbitrary language-level limit; actual work is bounded by
//! available resources, representation limits, and explicit operational
//! policies.
//!
//! # INTEGRATION
//!
//! - `crate::frontend::ast::node::program::Program` is the canonical AST root.
//! - `config.rs` owns concrete semantic options and operational budgets.
//! - `context.rs` owns immutable services and extension snapshots.
//! - `session.rs` owns per-invocation mutable state and work accounting.
//! - `pipeline.rs` owns pass ordering and analysis orchestration.
//! - `model/program.rs` owns the concrete semantic program model.
//! - `diagnostics/` owns structured diagnostic records and ordering.
//! - `domains/` owns extensible domain-specific validation.
//! - `crate::semantic` remains the legacy analyzer until migration is tested.
//!
//! This module intentionally does not construct a semantic model or claim
//! that the implementation of every analysis pass already exists.

use core::fmt;
use std::error::Error;

use crate::frontend::ast::node::program::{
    Program,
    ProgramValidationError,
};

/// Version of this public semantic API contract.
///
/// Independent of language, AST, semantic-model, and artifact versions.
pub const SEMANTIC_API_VERSION: u16 = 1;

/// Overall outcome of a semantic-analysis invocation.
///
/// Completion is distinct from semantic validity and target feasibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SemanticStatus {
    /// All requested analysis passes completed.
    ///
    /// The program may still have semantic errors.
    Complete,

    /// Analysis stopped before all required checks completed.
    ///
    /// Examples include exhausted operational budgets or unavailable
    /// analysis capabilities.
    Incomplete,

    /// Analysis was deliberately cancelled or aborted.
    Aborted,

    /// An internal or integration failure prevented reliable analysis.
    Failed,
}

impl SemanticStatus {
    /// Whether all requested analysis work completed.
    #[must_use]
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
    }

    /// Whether analysis did not complete successfully.
    #[must_use]
    pub const fn is_non_complete(self) -> bool {
        !self.is_complete()
    }
}

impl fmt::Display for SemanticStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Complete => "complete",
            Self::Incomplete => "incomplete",
            Self::Aborted => "aborted",
            Self::Failed => "failed",
        })
    }
}

/// Minimal diagnostic contract required by the public result API.
///
/// Implement this for the canonical structured diagnostic type in
/// `diagnostics/diagnostic.rs`. Error severity must never be classified as
/// a warning or informational diagnostic.
pub trait SemanticDiagnostic {
    /// Whether this diagnostic makes the analyzed program invalid.
    fn is_error(&self) -> bool;
}

/// Validity contract for semantic models returned by an implementation.
///
/// This interface prevents the API from assuming that the existence of a
/// model proves that the model is finalized or semantically valid.
///
/// Implement this for the canonical semantic program model. The implementation
/// must return `true` only when its own checked lifecycle and validity
/// invariants establish that the model is complete and valid.
pub trait SemanticModelState {
    /// Whether the model itself establishes completed semantic analysis.
    fn establishes_completion(&self) -> bool;

    /// Whether the model itself establishes semantic validity.
    ///
    /// Must be false for invalid, unresolved, unfinalized, or partial models.
    fn establishes_validity(&self) -> bool;
}

/// Result of one semantic-analysis invocation.
///
/// A result may contain a partial model for editor tooling when analysis
/// stops early. Such a model is never a validity certificate.
///
/// `diagnostics_complete` records whether the returned diagnostic collection
/// is known to contain all diagnostics required by the configured analysis.
/// It must be false if diagnostic limits or recovery policies may have omitted
/// diagnostics that affect correctness.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticResult<M, D> {
    status: SemanticStatus,
    model: Option<M>,
    diagnostics: Vec<D>,
    diagnostics_complete: bool,
}

impl<M, D> SemanticResult<M, D> {
    /// Construct a result from an explicit status, model, and diagnostics.
    ///
    /// This compatibility constructor means that the supplied diagnostic
    /// collection is complete. Use `from_parts_with_diagnostic_completeness`
    /// if truncation or omission is possible.
    ///
    /// The constructor does not certify model validity.
    #[must_use]
    pub fn from_parts(
        status: SemanticStatus,
        model: Option<M>,
        diagnostics: Vec<D>,
    ) -> Self {
        Self::from_parts_with_diagnostic_completeness(
            status,
            model,
            diagnostics,
            true,
        )
    }

    /// Construct a result while explicitly declaring diagnostic completeness.
    ///
    /// A caller must set `diagnostics_complete` to false whenever the
    /// diagnostic collection may omit correctness-relevant diagnostics.
    #[must_use]
    pub fn from_parts_with_diagnostic_completeness(
        status: SemanticStatus,
        model: Option<M>,
        diagnostics: Vec<D>,
        diagnostics_complete: bool,
    ) -> Self {
        Self {
            status,
            model,
            diagnostics,
            diagnostics_complete,
        }
    }

    /// Construct a result after all requested passes completed.
    ///
    /// The model must still independently establish completion and validity
    /// before `is_valid()` can return true.
    #[must_use]
    pub fn complete(model: M, diagnostics: Vec<D>) -> Self {
        Self::from_parts(SemanticStatus::Complete, Some(model), diagnostics)
    }

    /// Construct a result with partial analysis facts.
    #[must_use]
    pub fn incomplete(model: Option<M>, diagnostics: Vec<D>) -> Self {
        Self::from_parts(SemanticStatus::Incomplete, model, diagnostics)
    }

    /// Construct a result after deliberate cancellation or abortion.
    #[must_use]
    pub fn aborted(model: Option<M>, diagnostics: Vec<D>) -> Self {
        Self::from_parts(SemanticStatus::Aborted, model, diagnostics)
    }

    /// Construct a result after an internal or integration failure.
    #[must_use]
    pub fn failed(model: Option<M>, diagnostics: Vec<D>) -> Self {
        Self::from_parts(SemanticStatus::Failed, model, diagnostics)
    }

    /// Return the analysis status.
    #[must_use]
    pub const fn status(&self) -> SemanticStatus {
        self.status
    }

    /// Whether the requested analysis pipeline completed.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.status.is_complete()
    }

    /// Borrow the semantic model, if present.
    ///
    /// A model in an incomplete, aborted, or failed result is partial.
    #[must_use]
    pub fn model(&self) -> Option<&M> {
        self.model.as_ref()
    }

    /// Consume the result and return its model, if present.
    #[must_use]
    pub fn into_model(self) -> Option<M> {
        self.model
    }

    /// Borrow diagnostics in the order supplied by the implementation.
    ///
    /// A conforming service must apply the canonical diagnostic ordering
    /// policy before returning its result.
    #[must_use]
    pub fn diagnostics(&self) -> &[D] {
        &self.diagnostics
    }

    /// Consume the result and return its diagnostics.
    #[must_use]
    pub fn into_diagnostics(self) -> Vec<D> {
        self.diagnostics
    }

    /// Whether a model is present.
    #[must_use]
    pub const fn has_model(&self) -> bool {
        self.model.is_some()
    }

    /// Number of retained diagnostics.
    #[must_use]
    pub fn diagnostic_count(&self) -> usize {
        self.diagnostics.len()
    }

    /// Whether the returned diagnostics are known to be complete.
    #[must_use]
    pub const fn diagnostics_complete(&self) -> bool {
        self.diagnostics_complete
    }

    /// Whether any retained diagnostic represents a semantic error.
    #[must_use]
    pub fn has_errors(&self) -> bool
    where
        D: SemanticDiagnostic,
    {
        self.diagnostics.iter().any(SemanticDiagnostic::is_error)
    }

    /// Whether this result proves semantic validity.
    ///
    /// Validity requires every gate below:
    ///
    /// - the analysis status is complete;
    /// - a semantic model exists;
    /// - the model independently establishes completion;
    /// - the model independently establishes validity;
    /// - the diagnostic collection is complete;
    /// - no retained diagnostic represents an error.
    ///
    /// Target feasibility is not established by this method.
    #[must_use]
    pub fn is_valid(&self) -> bool
    where
        M: SemanticModelState,
        D: SemanticDiagnostic,
    {
        if self.status != SemanticStatus::Complete
            || !self.diagnostics_complete
            || self.has_errors()
        {
            return false;
        }

        self.model.as_ref().is_some_and(|model| {
            model.establishes_completion() && model.establishes_validity()
        })
    }
}

/// Implementation-independent semantic-analysis service.
///
/// The implementation owns or receives canonical AST storage, immutable
/// context, extension snapshots, per-invocation sessions, and the pipeline.
/// Mutable analysis state must not be shared across independent invocations
/// unless the implementation explicitly provides safe isolation.
///
/// Implementations must preserve target-independent program meaning, symbolic
/// requirements, explicit error states, deterministic diagnostics, and
/// configured operational budgets.
pub trait SemanticService {
    /// Concrete configuration type owned by `config.rs`.
    type Options;

    /// Concrete semantic model type owned by `model/`.
    type Model: SemanticModelState;

    /// Structured diagnostic type owned by `diagnostics/`.
    type Diagnostic: SemanticDiagnostic;

    /// Error returned when the service cannot execute reliably.
    type Error: fmt::Display;

    /// Validate configuration before starting expensive analysis.
    ///
    /// Implementations should validate supported language versions, extension
    /// policy, budget consistency, and other cross-field invariants here.
    /// The default is suitable only when the options type has no additional
    /// validation requirements.
    fn validate_options(
        &self,
        _options: &Self::Options,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Analyze a canonical source-level program.
    ///
    /// The implementation must resolve child `NodeId`s using the authoritative
    /// AST storage/provider. A root-only analysis is not a complete analysis
    /// of the program.
    ///
    /// Source-level errors belong in structured diagnostics. The returned
    /// status must be incomplete, aborted, or failed when the corresponding
    /// condition occurs.
    fn analyze_program(
        &self,
        program: &Program,
        options: &Self::Options,
    ) -> Result<SemanticResult<Self::Model, Self::Diagnostic>, Self::Error>;
}

/// Public entry point for semantic analysis.
///
/// Validates the canonical program root and service options before delegating
/// to the configured service. Root validation is not a substitute for
/// validating all reachable AST nodes through the authoritative AST storage.
pub fn analyze<S>(
    service: &S,
    program: &Program,
    options: &S::Options,
) -> Result<
    SemanticResult<S::Model, S::Diagnostic>,
    SemanticApiError<S::Error>,
>
where
    S: SemanticService + ?Sized,
{
    program
        .validate()
        .map_err(SemanticApiError::InvalidProgram)?;

    service
        .validate_options(options)
        .map_err(SemanticApiError::Service)?;

    service
        .analyze_program(program, options)
        .map_err(SemanticApiError::Service)
}

/// Error returned before the API can provide a reliable semantic result.
///
/// Ordinary source-language errors should be represented as diagnostics
/// inside `SemanticResult`, not as API errors.
#[derive(Debug)]
#[non_exhaustive]
pub enum SemanticApiError<E> {
    /// The canonical program root failed structural validation.
    InvalidProgram(ProgramValidationError),

    /// Configuration, AST storage, context, or the analysis service failed.
    Service(E),
}

impl<E: fmt::Display> fmt::Display for SemanticApiError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidProgram(error) => {
                write!(f, "invalid canonical AST program: {error}")
            }
            Self::Service(error) => {
                write!(f, "semantic analysis service failed: {error}")
            }
        }
    }
}

impl<E> Error for SemanticApiError<E>
where
    E: Error + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidProgram(error) => Some(error),
            Self::Service(error) => Some(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestDiagnostic {
        error: bool,
    }

    impl SemanticDiagnostic for TestDiagnostic {
        fn is_error(&self) -> bool {
            self.error
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestModel {
        complete: bool,
        valid: bool,
    }

    impl SemanticModelState for TestModel {
        fn establishes_completion(&self) -> bool {
            self.complete
        }

        fn establishes_validity(&self) -> bool {
            self.valid
        }
    }

    fn valid_model() -> TestModel {
        TestModel {
            complete: true,
            valid: true,
        }
    }

    #[test]
    fn complete_valid_model_without_errors_is_valid() {
        let result = SemanticResult::complete(
            valid_model(),
            Vec::<TestDiagnostic>::new(),
        );

        assert_eq!(result.status(), SemanticStatus::Complete);
        assert!(result.is_complete());
        assert!(result.has_model());
        assert!(result.diagnostics_complete());
        assert!(!result.has_errors());
        assert!(result.is_valid());
    }

    #[test]
    fn diagnostic_error_prevents_validity() {
        let result = SemanticResult::complete(
            valid_model(),
            vec![TestDiagnostic { error: true }],
        );

        assert!(result.is_complete());
        assert!(result.has_errors());
        assert!(!result.is_valid());
    }

    #[test]
    fn incomplete_diagnostic_collection_prevents_validity() {
        let result = SemanticResult::from_parts_with_diagnostic_completeness(
            SemanticStatus::Complete,
            Some(valid_model()),
            Vec::<TestDiagnostic>::new(),
            false,
        );

        assert!(!result.diagnostics_complete());
        assert!(!result.is_valid());
    }

    #[test]
    fn incomplete_model_cannot_establish_validity() {
        let model = TestModel {
            complete: false,
            valid: true,
        };
        let result =
            SemanticResult::complete(model, Vec::<TestDiagnostic>::new());

        assert!(!result.is_valid());
    }

    #[test]
    fn invalid_model_cannot_establish_validity() {
        let model = TestModel {
            complete: true,
            valid: false,
        };
        let result =
            SemanticResult::complete(model, Vec::<TestDiagnostic>::new());

        assert!(!result.is_valid());
    }

    #[test]
    fn warnings_do_not_invalidate_a_valid_model() {
        let result = SemanticResult::complete(
            valid_model(),
            vec![TestDiagnostic { error: false }],
        );

        assert!(!result.has_errors());
        assert!(result.is_valid());
    }

    #[test]
    fn incomplete_analysis_never_establishes_validity() {
        let result = SemanticResult::incomplete(
            Some(valid_model()),
            Vec::<TestDiagnostic>::new(),
        );

        assert_eq!(result.status(), SemanticStatus::Incomplete);
        assert!(result.has_model());
        assert!(!result.is_valid());
    }

    #[test]
    fn aborted_analysis_never_establishes_validity() {
        let result = SemanticResult::aborted(
            Some(valid_model()),
            Vec::<TestDiagnostic>::new(),
        );

        assert_eq!(result.status(), SemanticStatus::Aborted);
        assert!(!result.is_valid());
    }

    #[test]
    fn failed_analysis_never_establishes_validity() {
        let result = SemanticResult::<TestModel, TestDiagnostic>::failed(
            None,
            Vec::new(),
        );

        assert_eq!(result.status(), SemanticStatus::Failed);
        assert!(!result.has_model());
        assert!(!result.is_valid());
    }

    #[test]
    fn complete_status_without_model_is_not_valid() {
        let result = SemanticResult::<TestModel, TestDiagnostic>::from_parts(
            SemanticStatus::Complete,
            None,
            Vec::new(),
        );

        assert!(result.is_complete());
        assert!(!result.is_valid());
    }
}
