
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Semantic Analysis Pipeline
//!
//! File: `src/frontend/semantic/pipeline.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Orchestrate semantic passes over the canonical frontend AST while sharing
//! an immutable semantic context and isolating mutable state in one analysis
//! session.
//!
//! ## AUTHORITY
//!
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/frontend/ast/node/`
//! - `src/frontend/semantic/config.rs`
//! - `src/frontend/semantic/context.rs`
//! - `src/frontend/semantic/session.rs`
//!
//! Normative language specifications take precedence over implementation
//! convenience.
//!
//! ## OWNS
//!
//! - Ordered registration and orchestration of semantic passes.
//! - Validation of pass IDs and declared pass dependencies.
//! - Pass-level work accounting.
//! - Pipeline failure propagation.
//! - The distinction between orchestration completion and semantic validity.
//!
//! ## DOES_NOT_OWN
//!
//! - Source syntax, tokens, parsing, AST definitions, or AST conversion.
//! - Name resolution, symbol storage, type inference, or type compatibility.
//! - Effect, capability, resource, contract, or domain-specific rules.
//! - A competing semantic model or intermediate representation.
//! - Target discovery, feasibility, placement, routing, scheduling, lowering,
//!   quantum error correction, or runtime execution.
//!
//! ## ARCHITECTURE
//!
//! Canonical AST
//!     -> structural validation
//!     -> declaration collection
//!     -> name resolution
//!     -> type and constraint analysis
//!     -> effects, capabilities, resources, and contracts
//!     -> domain validation
//!     -> semantic-model finalization
//!
//! The sequence above describes the intended architecture, not a claim that
//! every pass is currently implemented. The registered pass list is the
//! executable source of orchestration order.
//!
//! ## SCALABILITY
//!
//! This module imposes no language-level ceiling on machine resources,
//! computational domains, declarations, type arity, tensor dimensions,
//! quantum resources, or hardware widths.
//!
//! Configured work budgets are compiler-operational safeguards. Exhaustion
//! must result in an explicit incomplete session, never a successful
//! certification of semantic validity.
//!
//! No finite implementation can guarantee processing literally infinite
//! input. Practical work is bounded by available resources and any explicit
//! operational policy.
//!
//! ## DETERMINISM
//!
//! Passes execute in registration order. A pass may depend only on passes
//! registered before it. This prevents accidental dependency cycles and
//! makes execution order explicit and reproducible.
//!
//! Each pass is responsible for deterministic behavior within its own
//! implementation, including iteration and diagnostic production.
//!
//! ## INTEGRATION CONTRACT
//!
//! - `context.rs` provides immutable configuration and extension snapshots.
//! - `config.rs` owns the configurable semantic-work budgets.
//! - `session.rs` owns mutable per-invocation accounting and diagnostics.
//! - `symbols/`, `types/`, `effects/`, `capabilities/`, `resources/`,
//!   `contracts/`, and `domains/` provide concrete passes.
//! - `model/` owns the finalized semantic facts and program result.
//! - `api.rs` should construct the context and session, invoke this pipeline,
//!   and translate its outcome into the public analysis result.
//! - The AST remains owned by `src/frontend/ast/node/`.
//! - Classical IR, `quantum::ir`, and HDL IR remain downstream boundaries.
//!
//! ## COMPLETION INVARIANT
//!
//! A completed pipeline means only that every registered pass returned
//! successfully and the session completed. It does not, by itself, prove
//! that the source program is valid. The API must inspect semantic facts,
//! unresolved obligations, diagnostics, and finalization results.
//!
//! ## FREEZE CRITERIA
//!
//! This module is ready to freeze when:
//!
//! 1. The semantic module exports this file.
//! 2. The production passes implement `SemanticPass`.
//! 3. The public API handles `PipelineError` and session status explicitly.
//! 4. Pass dependency and deterministic-order tests pass.
//! 5. Budget exhaustion cannot be mistaken for completed analysis.
//! 6. The complete workspace builds and its relevant tests and lints pass.

use core::fmt;
use core::num::NonZeroU64;
use std::error::Error;
use std::sync::Arc;

use super::config::{SemanticConfig, WorkBudget};
use super::context::SemanticContext;
use super::session::{
    SemanticSession, SemanticSessionContext, SessionError, SessionPolicySnapshot,
    SessionStatus,
};

/// Connect the immutable semantic context to the session's policy contract.
///
/// The context remains the owner of configuration. This adapter merely
/// converts configured budgets into the session's accounting representation.
///
/// The session's work counter is `u64`; therefore a configured `usize` budget
/// that cannot fit into `u64` is represented as `u64::MAX`. Such a budget
/// still cannot permit counter overflow: the session reports that condition
/// as an incomplete analysis.
impl SemanticSessionContext for SemanticContext {
    fn session_policy(&self) -> SessionPolicySnapshot {
        let limits = self.config().limits();

        SessionPolicySnapshot {
            max_work_units: budget_as_u64(limits.total_work()),
            max_diagnostics: limits.diagnostics().limit(),
            max_scope_depth: limits.recursion_depth().limit(),
        }
    }
}

fn budget_as_u64(budget: WorkBudget) -> Option<NonZeroU64> {
    budget.limit().and_then(|limit| {
        let value = u64::try_from(limit.get()).unwrap_or(u64::MAX);
        NonZeroU64::new(value)
    })
}

/// Control returned by a semantic pass after it finishes its current work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PassControl {
    /// Continue with the next registered pass.
    Continue,

    /// Deliberately stop the pipeline.
    ///
    /// The session becomes `Aborted`. This is not semantic success, and the
    /// caller must preserve the aborted status in its public result.
    Abort,
}

/// Failure reported by an individual semantic pass.
///
/// This describes an inability to complete the pass, not a source-language
/// diagnostic. Source errors should normally be emitted through the session's
/// diagnostic interface while the pass continues where recovery is safe.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PassFailure {
    message: String,
}

impl PassFailure {
    /// Construct a pass failure with a useful implementation-level message.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    /// Return the failure message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for PassFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for PassFailure {}

/// A semantic analysis pass.
///
/// `A` is the repository's canonical AST type. `S` is the analysis state
/// owned by the semantic model or analyzer implementation. `D` is the
/// diagnostic type selected by the public API.
///
/// Implementations must not create a competing AST, symbol table, type
/// system, or IR. Mutable state belongs to `S` and the per-run session.
/// Shared services belong to `SemanticContext`.
///
/// Pass IDs must be stable, non-empty identifiers. Dependencies must name
/// passes registered earlier in the pipeline.
pub trait SemanticPass<A, S, D>: Send + Sync {
    /// Stable identifier used for dependency checks and diagnostics.
    fn id(&self) -> &'static str;

    /// IDs of passes that must have completed before this pass is run.
    ///
    /// Return an empty slice for an independent pass. The returned IDs must
    /// be stable and unique within the pipeline.
    fn requires(&self) -> &'static [&'static str] {
        &[]
    }

    /// Run the pass against the canonical AST.
    ///
    /// A pass should:
    ///
    /// - use the shared immutable context for configuration and extensions;
    /// - record diagnostics through `session.emit_diagnostic`;
    /// - avoid converting unresolved facts into valid facts;
    /// - preserve symbolic resource and dimension constraints;
    /// - return an error if required work cannot safely complete;
    /// - return `Abort` only when the caller explicitly needs to stop analysis.
    ///
    /// A source-level semantic error is not automatically a pass failure:
    /// recoverable errors should be recorded as diagnostics so later
    /// independent passes can continue safely.
    fn run(
        &self,
        ast: &A,
        context: &SemanticContext,
        state: &mut S,
        session: &mut SemanticSession<SemanticContext, (), D>,
    ) -> Result<PassControl, PassFailure>;
}

/// Errors encountered while constructing a pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PipelineBuildError {
    /// A pass ID was empty or contained only whitespace.
    EmptyPassId,

    /// Two registered passes declared the same ID.
    DuplicatePassId {
        /// The duplicate ID.
        pass_id: &'static str,
    },

    /// A pass depends on a pass that has not been registered yet.
    ///
    /// Registering dependencies first makes the pipeline order explicit and
    /// prevents dependency cycles without requiring graph traversal.
    MissingDependency {
        /// The pass that declared the dependency.
        pass_id: &'static str,

        /// The dependency that was not registered earlier.
        dependency: &'static str,
    },

    /// A pass declared itself as a dependency.
    SelfDependency {
        /// The pass that declared itself as a dependency.
        pass_id: &'static str,
    },
}

impl fmt::Display for PipelineBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPassId => f.write_str("semantic pass ID must not be empty"),
            Self::DuplicatePassId { pass_id } => {
                write!(f, "duplicate semantic pass ID: {pass_id}")
            }
            Self::MissingDependency {
                pass_id,
                dependency,
            } => write!(
                f,
                "semantic pass {pass_id} requires unregistered earlier pass {dependency}"
            ),
            Self::SelfDependency { pass_id } => {
                write!(f, "semantic pass {pass_id} depends on itself")
            }
        }
    }
}

impl Error for PipelineBuildError {}

/// Errors encountered while executing the semantic pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PipelineError {
    /// Session state or operational limits prevented execution.
    Session {
        /// Pass associated with the operation, if one had started.
        pass_id: Option<&'static str>,

        /// Underlying session error.
        source: SessionError,
    },

    /// A semantic pass could not complete required work.
    PassFailed {
        /// Stable ID of the failed pass.
        pass_id: &'static str,

        /// Explanation supplied by the pass.
        message: String,
    },
}

impl fmt::Display for PipelineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Session { pass_id, source } => match pass_id {
                Some(pass_id) => {
                    write!(f, "semantic pipeline failed during {pass_id}: {source}")
                }
                None => write!(f, "semantic pipeline session failure: {source}"),
            },
            Self::PassFailed { pass_id, message } => {
                write!(f, "semantic pass {pass_id} failed: {message}")
            }
        }
    }
}

impl Error for PipelineError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Session { source, .. } => Some(source),
            Self::PassFailed { .. } => None,
        }
    }
}

/// Ordered semantic-pass orchestrator.
///
/// Construct one pipeline for a stable pass configuration, then use it with
/// separate sessions for independent analysis invocations. A pipeline may be
/// shared across threads when its registered pass implementations are safe
/// to share; each run still requires its own mutable state and session.
pub struct SemanticPipeline<A, S, D> {
    passes: Vec<Box<dyn SemanticPass<A, S, D>>>,
}

impl<A, S, D> Default for SemanticPipeline<A, S, D> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A, S, D> SemanticPipeline<A, S, D> {
    /// Construct an empty pipeline.
    ///
    /// An empty pipeline is useful for integration tests and staged
    /// development. It does not establish that a program is semantically
    /// valid. Production API construction should ensure that all required
    /// passes for the requested language version are registered.
    #[must_use]
    pub const fn new() -> Self {
        Self { passes: Vec::new() }
    }

    /// Return the number of registered passes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.passes.len()
    }

    /// Return whether no passes have been registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.passes.is_empty()
    }

    /// Return registered pass IDs in execution order.
    #[must_use]
    pub fn pass_ids(&self) -> Vec<&'static str> {
        self.passes.iter().map(|pass| pass.id()).collect()
    }

    /// Register a pass after all of its dependencies.
    ///
    /// Registration order is execution order. This method rejects duplicate
    /// IDs, empty IDs, self-dependencies, and dependencies that have not
    /// already been registered.
    ///
    /// # Errors
    ///
    /// Returns a `PipelineBuildError` when the pass contract is inconsistent.
    pub fn register_pass<P>(
        &mut self,
        pass: P,
    ) -> Result<(), PipelineBuildError>
    where
        P: SemanticPass<A, S, D> + 'static,
    {
        let pass_id = pass.id();

        if pass_id.trim().is_empty() {
            return Err(PipelineBuildError::EmptyPassId);
        }

        if self.passes.iter().any(|existing| existing.id() == pass_id) {
            return Err(PipelineBuildError::DuplicatePassId { pass_id });
        }

        for dependency in pass.requires() {
            if *dependency == pass_id {
                return Err(PipelineBuildError::SelfDependency { pass_id });
            }

            if !self
                .passes
                .iter()
                .any(|existing| existing.id() == *dependency)
            {
                return Err(PipelineBuildError::MissingDependency {
                    pass_id,
                    dependency,
                });
            }
        }

        self.passes
            .try_reserve(1)
            .map_err(|_| PipelineBuildError::EmptyPassId)?;

        self.passes.push(Box::new(pass));
        Ok(())
    }

    /// Execute every registered pass in deterministic registration order.
    ///
    /// The caller supplies the canonical AST, immutable context, mutable
    /// semantic state, and a fresh session for this invocation.
    ///
    /// Each pass consumes one session work unit before it starts. Passes
    /// should charge additional work through the session or their dedicated
    /// subsystem when the agreed accounting model requires it.
    ///
    /// # Completion guarantees
    ///
    /// - A pass failure marks the session incomplete.
    /// - A session-budget failure propagates without reporting completion.
    /// - An explicit abort leaves the session aborted.
    /// - Successful orchestration calls `session.complete()`.
    /// - Successful orchestration does not imply semantic validity.
    ///
    /// # Errors
    ///
    /// Returns an error when a pass fails or session accounting prevents
    /// execution or completion. The session retains its diagnostics and
    /// explicit status for the API to inspect.
    pub fn execute(
        &self,
        ast: &A,
        context: &SemanticContext,
        state: &mut S,
        session: &mut SemanticSession<SemanticContext, (), D>,
    ) -> Result<(), PipelineError> {
        if !session.status().is_running() {
            return Err(PipelineError::Session {
                pass_id: None,
                source: SessionError::SessionNotRunning {
                    status: session.status(),
                },
            });
        }

        for pass in &self.passes {
            if let Err(source) = session.record_work() {
                return Err(PipelineError::Session {
                    pass_id: Some(pass.id()),
                    source,
                });
            }

            match pass.run(ast, context, state, session) {
                Ok(PassControl::Continue) => {}

                Ok(PassControl::Abort) => {
                    session.abort();
                    return Ok(());
                }

                Err(failure) => {
                    session.mark_incomplete();
                    return Err(PipelineError::PassFailed {
                        pass_id: pass.id(),
                        message: failure.message().to_owned(),
                    });
                }
            }
        }

        session.complete().map_err(|source| PipelineError::Session {
            pass_id: None,
            source,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::semantic::config::{
        SemanticConfig, SemanticLimits,
    };
    use crate::frontend::semantic::context::SemanticContext;
    use crate::frontend::semantic::session::{
        SemanticSession, SessionPolicySnapshot,
    };
    use core::num::NonZeroUsize;

    struct RecordingPass {
        id: &'static str,
        dependencies: &'static [&'static str],
    }

    impl SemanticPass<u8, Vec<&'static str>, String> for RecordingPass {
        fn id(&self) -> &'static str {
            self.id
        }

        fn requires(&self) -> &'static [&'static str] {
            self.dependencies
        }

        fn run(
            &self,
            _ast: &u8,
            _context: &SemanticContext,
            state: &mut Vec<&'static str>,
            _session: &mut SemanticSession<SemanticContext, (), String>,
        ) -> Result<PassControl, PassFailure> {
            state.push(self.id);
            Ok(PassControl::Continue)
        }
    }

    fn context() -> Arc<SemanticContext> {
        Arc::new(
            SemanticContext::without_extensions(SemanticConfig::default())
                .expect("default semantic configuration should be valid"),
        )
    }

    fn session(
        context: Arc<SemanticContext>,
    ) -> SemanticSession<SemanticContext, (), String> {
        SemanticSession::new(context)
            .expect("session should be constructed")
    }

    #[test]
    fn registration_order_is_execution_order() {
        let mut pipeline =
            SemanticPipeline::<u8, Vec<&'static str>, String>::new();

        pipeline
            .register_pass(RecordingPass {
                id: "structural",
                dependencies: &[],
            })
            .expect("first pass should register");

        pipeline
            .register_pass(RecordingPass {
                id: "declarations",
                dependencies: &["structural"],
            })
            .expect("dependent pass should register");

        assert_eq!(pipeline.pass_ids(), vec!["structural", "declarations"]);

        let context = context();
        let mut session = session(Arc::clone(&context));
        let mut state = Vec::new();

        pipeline
            .execute(&0, &context, &mut state, &mut session)
            .expect("pipeline should complete");

        assert_eq!(state, vec!["structural", "declarations"]);
        assert_eq!(session.status(), SessionStatus::Completed);
        assert_eq!(session.usage().work_units, 2);
    }

    #[test]
    fn registration_rejects_duplicate_ids() {
        let mut pipeline =
            SemanticPipeline::<u8, Vec<&'static str>, String>::new();

        for _ in 0..2 {
            let result = pipeline.register_pass(RecordingPass {
                id: "same",
                dependencies: &[],
            });

            if pipeline.is_empty() {
                assert!(result.is_ok());
            } else {
                assert_eq!(
                    result,
                    Err(PipelineBuildError::DuplicatePassId {
                        pass_id: "same"
                    })
                );
            }
        }
    }

    #[test]
    fn registration_rejects_missing_dependencies() {
        let mut pipeline =
            SemanticPipeline::<u8, Vec<&'static str>, String>::new();

        let result = pipeline.register_pass(RecordingPass {
            id: "resolution",
            dependencies: &["declarations"],
        });

        assert_eq!(
            result,
            Err(PipelineBuildError::MissingDependency {
                pass_id: "resolution",
                dependency: "declarations",
            })
        );
    }

    #[test]
    fn budget_exhaustion_never_completes_the_session() {
        let limits = SemanticLimits::unlimited()
            .with_total_work(
                WorkBudget::from_usize(1)
                    .expect("a positive work budget is valid"),
            );

        let config = SemanticConfig::default().with_limits(limits);
        let context = Arc::new(
            SemanticContext::without_extensions(config)
                .expect("configuration should be valid"),
        );

        let mut pipeline =
            SemanticPipeline::<u8, Vec<&'static str>, String>::new();

        pipeline
            .register_pass(RecordingPass {
                id: "first",
                dependencies: &[],
            })
            .expect("first pass should register");

        pipeline
            .register_pass(RecordingPass {
                id: "second",
                dependencies: &["first"],
            })
            .expect("second pass should register");

        let mut session = session(Arc::clone(&context));
        let mut state = Vec::new();

        let result =
            pipeline.execute(&0, &context, &mut state, &mut session);

        assert!(matches!(result, Err(PipelineError::Session { .. })));
        assert_eq!(state, vec!["first"]);
        assert_eq!(session.status(), SessionStatus::Incomplete);
        assert_eq!(session.usage().work_units, 1);
    }

    #[test]
    fn explicit_abort_is_not_semantic_success() {
        struct AbortPass;

        impl SemanticPass<(), (), String> for AbortPass {
            fn id(&self) -> &'static str {
                "abort"
            }

            fn run(
                &self,
                _ast: &(),
                _context: &SemanticContext,
                _state: &mut (),
                _session: &mut SemanticSession<SemanticContext, (), String>,
            ) -> Result<PassControl, PassFailure> {
                Ok(PassControl::Abort)
            }
        }

        let mut pipeline = SemanticPipeline::<(), (), String>::new();
        pipeline
            .register_pass(AbortPass)
            .expect("abort pass should register");

        let context = context();
        let mut session = session(Arc::clone(&context));

        pipeline
            .execute(&(), &context, &mut (), &mut session)
            .expect("an explicit abort is a reported lifecycle outcome");

        assert_eq!(session.status(), SessionStatus::Aborted);
        assert!(!session.status().orchestration_completed());
    }

    #[test]
    fn recursion_budget_becomes_session_scope_budget() {
        let limits = SemanticLimits::unlimited().with_recursion_depth(
            WorkBudget::from_usize(12)
                .expect("a positive recursion budget is valid"),
        );

        let config = SemanticConfig::default().with_limits(limits);
        let context = SemanticContext::without_extensions(config)
            .expect("configuration should be valid");

        let policy = context.session_policy();

        assert_eq!(policy.max_scope_depth, NonZeroUsize::new(12));
    }

    #[test]
    fn default_context_uses_unlimited_session_budgets() {
        let context = SemanticContext::without_extensions(
            SemanticConfig::default(),
        )
        .expect("default configuration should be valid");

        assert_eq!(
            context.session_policy(),
            SessionPolicySnapshot::unlimited()
        );
    }
}
