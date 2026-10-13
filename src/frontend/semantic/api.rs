
#![forbid(unsafe_code)]

//! Public API for Zamani's native frontend semantic analysis.
//!
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! This module defines the stable boundary between callers of semantic
//! analysis and the implementation that performs it. It accepts the
//! canonical frontend AST program, delegates analysis to a configured
//! semantic service, and preserves the distinction between complete,
//! incomplete, aborted, and failed analysis.
//!
//! # Architectural position
//!
//! ```text
//! Source
//!   |
//!   v
//! Lexer -> Parser -> Canonical frontend AST
//!                         |
//!                         v
//!                    Semantic API
//!                         |
//!                         v
//!                Semantic implementation
//!                         |
//!                         v
//!                  Semantic result
//!                         |
//!             +-----------+-----------+
//!             |           |           |
//!             v           v           v
//!        Classical     quantum::ir   HDL / other IR
//!             |           |           |
//!             +-----------+-----------+
//!                         |
//!                         v
//!             Target-specific compilation
//! ```
//!
//! The diagram describes the intended architecture. This API does not itself
//! implement the semantic engine or claim that all downstream paths are
//! already connected.
//!
//! # Ownership
//!
//! This module owns:
//! - the public semantic-service contract;
//! - the public analysis entry point;
//! - semantic result and completion-status contracts;
//! - the diagnostic interface required by result consumers;
//! - API-level validation of the canonical program root;
//! - API-level error reporting.
//!
//! This module does not own:
//! - AST node definitions or AST storage;
//! - lexical analysis or parsing;
//! - symbol tables or name resolution;
//! - type inference, unification, or type checking;
//! - effect inference or authorization;
//! - capability and resource requirement evaluation;
//! - contract checking;
//! - domain-specific semantic rules;
//! - semantic configuration policy;
//! - per-invocation mutable analysis state;
//! - IR generation, target feasibility, placement, scheduling, routing,
//!   quantum error correction, or runtime execution.
//!
//! # Integration contract
//!
//! - `crate::frontend::ast::node::program::Program` is the canonical program
//!   root. This module does not define another program or AST type.
//! - The AST storage/provider is owned by the AST subsystem or by the
//!   concrete semantic context. Since `Program` refers to children through
//!   canonical node identities, a concrete service must resolve those
//!   identities using the repository's authoritative AST storage contract.
//! - `config.rs` owns concrete semantic configuration and operational budgets.
//!   The API deliberately treats configuration as an associated type so it
//!   does not duplicate or prematurely freeze that implementation.
//! - `context.rs` owns the concrete read-only semantic context and extension
//!   snapshots.
//! - `session.rs` owns per-analysis mutable state, diagnostics accumulation,
//!   work accounting, cancellation/abort state, and incomplete-analysis state.
//! - `pipeline.rs` owns pass ordering and orchestration.
//! - `model/program.rs` owns the finalized semantic program model.
//! - `diagnostics/` owns concrete diagnostic records, codes, spans, labels,
//!   and ordering policy.
//! - Domain validators consume the shared semantic interfaces; they do not
//!   replace this API or create competing ASTs.
//! - Legacy `crate::semantic` remains available until a separate, tested
//!   migration changes the compiler entry points.
//!
//! # Completion and validity
//!
//! A completed analysis is not necessarily a valid program: it may have
//! completed successfully while finding semantic errors. Callers must inspect
//! `SemanticResult::is_valid()` or the structured diagnostics rather than
//! equating `Complete` with validity.
//!
//! An incomplete, aborted, or failed analysis must never be represented as a
//! valid result. Partial semantic facts may be retained for diagnostics or
//! tooling, but they are not a certificate of validity.
//!
//! # Portability and scalability
//!
//! This API does not encode machine-capacity limits, a closed list of
//! computational domains, or a fixed maximum number of AST nodes, declarations,
//! operations, qubits, devices, tensor dimensions, or resources.
//!
//! Practical analysis budgets belong to the supplied configuration and
//! per-analysis session. When a budget is exhausted, the implementation must
//! report an incomplete or aborted result rather than silently truncating work
//! or reporting success.
//!
//! No finite implementation can guarantee analysis of literally infinite
//! input. The architectural requirement is that the API imposes no arbitrary
//! language-level ceiling; practical work is bounded by representational
//! limits, configured policies, and available resources.
//!
//! # Determinism
//!
//! For the same AST, language version, options, AST storage, extension
//! definitions, and semantic policy snapshot, a conforming implementation
//! must produce equivalent semantic facts and deterministically ordered
//! diagnostics.
//!
//! The API itself introduces no time-, randomness-, hardware-, or
//! environment-dependent behavior.
//!
//! # Freeze criteria
//!
//! This module is ready to freeze when:
//! - the native frontend exposes the canonical `Program` type at the import
//!   path used below;
//! - the concrete `SemanticService` implementation is connected to the
//!   canonical AST storage, context, session, and pipeline;
//! - concrete diagnostics implement `SemanticDiagnostic`;
//! - incomplete and aborted work cannot be mistaken for a valid program;
//! - API contract tests and the full workspace build pass;
//! - legacy compiler entry points remain compatible until migration is
//!   separately approved.

use core::fmt;
use std::error::Error;

use crate::frontend::ast::node::program::Program;

/// Version of this public semantic API contract.
///
/// This is independent of the Zamani language version, AST schema version,
/// semantic-model schema version, and serialized artifact version.
pub const SEMANTIC_API_VERSION: u16 = 1;

/// Overall outcome of a single semantic-analysis invocation.
///
/// This status describes analysis completion, not target feasibility and not
/// whether the source program is free of semantic errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SemanticStatus {
    /// The configured analysis pipeline finished its requested work.
    ///
    /// Diagnostics may still contain errors. Use `SemanticResult::is_valid`
    /// to determine whether the result is valid.
    Complete,

    /// Analysis stopped before all required checks could be completed.
    ///
    /// Examples include a configured work budget being exhausted or an
    /// explicitly unsupported analysis stage being encountered.
    Incomplete,

    /// Analysis was deliberately stopped by cancellation, policy, or an
    /// explicit abort decision.
    Aborted,

    /// The analysis implementation could not produce a reliable result
    /// because of an internal or integration failure.
    Failed,
}

impl SemanticStatus {
    /// Whether the analysis pipeline completed its requested work.
    #[must_use]
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
    }

    /// Whether the result cannot establish full semantic validity because
    /// analysis did not finish.
    #[must_use]
    pub const fn is_non_complete(self) -> bool {
        !matches!(self, Self::Complete)
    }
}

impl fmt::Display for SemanticStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Complete => "complete",
            Self::Incomplete => "incomplete",
            Self::Aborted => "aborted",
            Self::Failed => "failed",
        })
    }
}

/// Minimal diagnostic contract needed by the public result API.
///
/// Concrete diagnostics remain owned by `diagnostics/`. This trait prevents
/// the API from defining a competing diagnostic structure while allowing
/// callers to determine whether a diagnostic represents an error.
///
/// Implementations must return `true` for every severity that makes the
/// semantic result invalid. Warnings and informational diagnostics should
/// return `false`.
pub trait SemanticDiagnostic {
    /// Whether this diagnostic represents a semantic error.
    fn is_error(&self) -> bool;
}

/// Result of one semantic-analysis invocation.
///
/// `model` may contain a finalized semantic model when analysis completed, or
/// partial facts when analysis did not complete. Partial facts must not be
/// used as a validity certificate.
///
/// Fields are private to preserve the invariants enforced by the constructors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticResult<M, D> {
    status: SemanticStatus,
    model: Option<M>,
    diagnostics: Vec<D>,
}

impl<M, D> SemanticResult<M, D> {
    /// Construct a result from an explicit status, optional model, and
    /// diagnostics.
    ///
    /// This constructor is intentionally explicit: concrete session/pipeline
    /// code is responsible for selecting the correct status. Consumers must
    /// still use `is_valid()` instead of interpreting `Complete` alone.
    #[must_use]
    pub fn from_parts(
        status: SemanticStatus,
        model: Option<M>,
        diagnostics: Vec<D>,
    ) -> Self {
        Self {
            status,
            model,
            diagnostics,
        }
    }

    /// Construct a result after the pipeline completed all requested work.
    #[must_use]
    pub fn complete(model: M, diagnostics: Vec<D>) -> Self {
        Self::from_parts(SemanticStatus::Complete, Some(model), diagnostics)
    }

    /// Construct a result where analysis did not finish but partial facts are
    /// available for tooling or diagnostic reporting.
    #[must_use]
    pub fn incomplete(model: Option<M>, diagnostics: Vec<D>) -> Self {
        Self::from_parts(SemanticStatus::Incomplete, model, diagnostics)
    }

    /// Construct a result after analysis was explicitly aborted.
    #[must_use]
    pub fn aborted(model: Option<M>, diagnostics: Vec<D>) -> Self {
        Self::from_parts(SemanticStatus::Aborted, model, diagnostics)
    }

    /// Construct a result after an internal or integration failure.
    #[must_use]
    pub fn failed(model: Option<M>, diagnostics: Vec<D>) -> Self {
        Self::from_parts(SemanticStatus::Failed, model, diagnostics)
    }

    /// Return the analysis completion status.
    #[must_use]
    pub const fn status(&self) -> SemanticStatus {
        self.status
    }

    /// Whether the configured analysis pipeline completed.
    ///
    /// This does not imply that the program is semantically valid.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.status.is_complete()
    }

    /// Borrow the semantic model, if one was produced.
    ///
    /// A model from an incomplete, aborted, or failed result is partial and
    /// must not be treated as fully validated.
    #[must_use]
    pub fn model(&self) -> Option<&M> {
        self.model.as_ref()
    }

    /// Consume the result and return its model, if present.
    #[must_use]
    pub fn into_model(self) -> Option<M> {
        self.model
    }

    /// Borrow the retained diagnostics in their implementation-defined,
    /// deterministic reporting order.
    #[must_use]
    pub fn diagnostics(&self) -> &[D] {
        &self.diagnostics
    }

    /// Consume the result and return its diagnostics.
    #[must_use]
    pub fn into_diagnostics(self) -> Vec<D> {
        self.diagnostics
    }

    /// Whether the result contains a model.
    #[must_use]
    pub const fn has_model(&self) -> bool {
        self.model.is_some()
    }

    /// Number of retained diagnostics.
    #[must_use]
    pub fn diagnostic_count(&self) -> usize {
        self.diagnostics.len()
    }

    /// Whether any retained diagnostic is an error.
    #[must_use]
    pub fn has_errors(&self) -> bool
    where
        D: SemanticDiagnostic,
    {
        self.diagnostics.iter().any(SemanticDiagnostic::is_error)
    }

    /// Whether this result establishes semantic validity.
    ///
    /// A result is valid only if:
    /// - the pipeline completed;
    /// - a semantic model exists; and
    /// - no retained diagnostic represents an error.
    ///
    /// The concrete pipeline must therefore ensure that errors are retained
    /// according to its configured diagnostic policy and that diagnostic
    /// truncation cannot silently certify an invalid program.
    #[must_use]
    pub fn is_valid(&self) -> bool
    where
        D: SemanticDiagnostic,
    {
        self.status == SemanticStatus::Complete
            && self.model.is_some()
            && !self.has_errors()
    }
}

/// Implementation-independent interface for a semantic-analysis service.
///
/// The concrete implementation owns or receives the canonical AST storage,
/// immutable context, extension registry snapshot, session, and analysis
/// pipeline. These details are associated with the implementation rather than
/// being duplicated in the public API.
///
/// A service should be shareable across independent analyses when its concrete
/// type permits it. Per-invocation mutable state must live in a session, not
/// in shared service state.
///
/// # Required behavior
///
/// Implementations must:
/// - analyze the canonical frontend AST rather than inventing another AST;
/// - validate source declarations and expressions according to the normative
///   semantic specification and active language version;
/// - distinguish source-language validity from target feasibility;
/// - preserve symbolic resource and type constraints where resolution is not
///   yet justified;
/// - produce structured diagnostics with canonical source locations;
/// - keep diagnostic ordering deterministic;
/// - enforce configured operational budgets;
/// - return `Incomplete`, `Aborted`, or `Failed` when appropriate;
/// - never convert an unknown or unresolved fact into a valid fact silently;
/// - avoid panics for ordinary invalid source programs;
/// - keep physical placement, scheduling, routing, and runtime allocation
///   outside semantic analysis.
pub trait SemanticService {
    /// Concrete semantic configuration type owned by `config.rs`.
    type Options;

    /// Finalized or partial semantic model type owned by `model/`.
    type Model;

    /// Structured diagnostic type owned by `diagnostics/`.
    type Diagnostic: SemanticDiagnostic;

    /// Concrete implementation/integration error.
    type Error: fmt::Display;

    /// Analyze a canonical source-level program.
    ///
    /// Implementations must use the authoritative AST storage/provider to
    /// resolve child node identities. A service that lacks the storage needed
    /// to inspect a program must return an error or an explicitly incomplete
    /// result; it must not pretend that the root alone is sufficient.
    fn analyze_program(
        &self,
        program: &Program,
        options: &Self::Options,
    ) -> Result<SemanticResult<Self::Model, Self::Diagnostic>, Self::Error>;
}

/// Public entry point for semantic analysis.
///
/// This function validates the canonical program root before invoking the
/// configured semantic service. Root validation is a defensive API boundary;
/// it does not replace complete AST structural validation or semantic checks.
///
/// The caller supplies the service and options explicitly. This avoids hidden
/// global state and allows separate compiler instances to use different
/// language editions, extension snapshots, analysis policies, and budgets.
pub fn analyze<S>(
    service: &S,
    program: &Program,
    options: &S::Options,
) -> Result<SemanticResult<S::Model, S::Diagnostic>, SemanticApiError<S::Error>>
where
    S: SemanticService + ?Sized,
{
    program
        .validate()
        .map_err(SemanticApiError::InvalidProgram)?;

    service
        .analyze_program(program, options)
        .map_err(SemanticApiError::Service)
}

/// Errors returned by the public semantic API before a semantic result can be
/// returned.
#[derive(Debug)]
pub enum SemanticApiError<E> {
    /// The canonical program root failed its own validation contract.
    ///
    /// This usually indicates malformed or incorrectly constructed AST input.
    InvalidProgram(crate::frontend::ast::node::program::ProgramValidationError),

    /// The semantic service could not execute its configured analysis.
    ///
    /// Source-level semantic errors should normally be returned as structured
    /// diagnostics inside `SemanticResult`, not as this variant. This variant
    /// is for service, configuration, context, storage, or integration
    /// failures that prevent the service from returning a reliable result.
    Service(E),
}

impl<E: fmt::Display> fmt::Display for SemanticApiError<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidProgram(error) => {
                write!(formatter, "invalid canonical AST program: {error}")
            }
            Self::Service(error) => {
                write!(formatter, "semantic analysis service failed: {error}")
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

    #[test]
    fn complete_result_without_errors_is_valid() {
        let result = SemanticResult::complete((), Vec::<TestDiagnostic>::new());

        assert_eq!(result.status(), SemanticStatus::Complete);
        assert!(result.is_complete());
        assert!(result.has_model());
        assert!(!result.has_errors());
        assert!(result.is_valid());
    }

    #[test]
    fn completed_result_with_errors_is_not_valid() {
        let result = SemanticResult::complete(
            (),
            vec![TestDiagnostic { error: true }],
        );

        assert!(result.is_complete());
        assert!(result.has_errors());
        assert!(!result.is_valid());
    }

    #[test]
    fn warnings_do_not_invalidate_completed_result() {
        let result = SemanticResult::complete(
            (),
            vec![TestDiagnostic { error: false }],
        );

        assert!(!result.has_errors());
        assert!(result.is_valid());
    }

    #[test]
    fn incomplete_result_never_establishes_validity() {
        let result = SemanticResult::incomplete(
            Some(()),
            Vec::<TestDiagnostic>::new(),
        );

        assert_eq!(result.status(), SemanticStatus::Incomplete);
        assert!(result.has_model());
        assert!(!result.is_valid());
    }

    #[test]
    fn aborted_result_never_establishes_validity() {
        let result = SemanticResult::aborted(
            Some(()),
            Vec::<TestDiagnostic>::new(),
        );

        assert_eq!(result.status(), SemanticStatus::Aborted);
        assert!(!result.is_valid());
    }

    #[test]
    fn failed_result_never_establishes_validity() {
        let result = SemanticResult::failed(
            None::<()>,
            Vec::<TestDiagnostic>::new(),
        );

        assert_eq!(result.status(), SemanticStatus::Failed);
        assert!(!result.has_model());
        assert!(!result.is_valid());
    }

    #[test]
    fn empty_complete_result_is_not_valid_without_a_model() {
        let result = SemanticResult::<(), TestDiagnostic>::from_parts(
            SemanticStatus::Complete,
            None,
            Vec::new(),
        );

        assert!(result.is_complete());
        assert!(!result.is_valid());
    }
}
