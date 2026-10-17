#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! Zamani Classical Computing — Optimization Pass Orchestration.
//!
//! Path: `src/classical/optimization/passes.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Define a target-independent, extensible orchestration API for classical
//! optimization passes. The module handles pass registration, deterministic
//! execution order, explicit operational budgets, verification boundaries,
//! and structured failure reporting.
//!
//! It deliberately does not implement or own a second intermediate
//! representation. Concrete passes operate on the canonical IR through an
//! explicit adapter implementing [`OptimizationTarget`].
//!
//! # Architectural authority
//!
//! - `grammar/Zamani.g4`: root grammar composition.
//! - `grammar/specification/semantic-model.md`: source-language meaning.
//! - `grammar/validation/semantic-boundaries.md`: semantic ownership.
//! - `src/zuir/`: universal IR, where used by the compilation pipeline.
//! - `src/quantum/ir/classical/`: existing canonical classical IR boundary.
//! - `src/optimizer.rs`: legacy optimizer over `crate::ir_gen` types.
//! - `src/classical/config.rs`: intended shared optimization policies
//!   and operational budgets.
//! - `src/classical/error.rs`: intended canonical error boundary.
//!
//! The concrete IR selected by a compilation pipeline remains authoritative.
//! An adapter must not erase semantic information merely to satisfy this API.
//!
//! # Ownership
//!
//! This file owns:
//!
//! - The optimization-pass interface.
//! - The optimization-target adapter interface.
//! - Pass registration and deterministic pipeline order.
//! - Pass execution accounting and operational budget enforcement.
//! - Pipeline verification boundaries.
//! - Structured orchestration errors and run reports.
//!
//! It does not own:
//!
//! - AST, semantic models, classical IR, or ZUIR definitions.
//! - Type inference, effect analysis, alias analysis, or dependence analysis.
//! - The legality proof for a concrete transformation.
//! - Numerical, floating-point, concurrency, or memory semantics.
//! - Target discovery, resource allocation, placement, or scheduling.
//! - Machine-code generation, execution, or runtime fallback.
//!
//! # Transactionality
//!
//! A pass must either commit a complete, valid transformation or leave its
//! target unchanged when it returns an error. This interface cannot roll
//! back arbitrary target mutations; concrete IR adapters and passes must
//! provide the required staging or transactional behavior.
//!
//! # Scalability
//!
//! There is no universal maximum pass count, change count, IR size, operation
//! count, hardware count, or data dimension. Optional budgets are operational
//! policy, not language semantics. Actual execution remains bounded by memory,
//! representable values, time, and available processing resources.
//!
//! # Determinism
//!
//! Passes execute in registration order. This module does not reorder passes
//! according to hash iteration, hardware discovery, or nondeterministic
//! scheduling. Each pass is responsible for deterministic behavior when its
//! contract requires it.
//!
//! # Completion status
//!
//! This file defines the independent orchestration contract. Full integration
//! requires concrete canonical-IR adapters, real optimization passes,
//! semantic-legality tests, and registration through the parent module.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;

/// Result type for optimization orchestration.
pub type OptimizationResult<T> = Result<T, OptimizationError>;

/// Stable identifier for a registered optimization pass.
///
/// Identifiers are open-ended strings rather than a closed enum, allowing
/// future passes and downstream extensions without changing this module.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PassId(String);

impl PassId {
    /// Constructs a pass identifier.
    ///
    /// Empty identifiers and identifiers containing control characters are
    /// rejected. No fixed identifier-length limit is imposed by this API.
    pub fn new(value: impl Into<String>) -> OptimizationResult<Self> {
        let value = value.into();

        if value.trim().is_empty() || value.chars().any(char::is_control) {
            return Err(OptimizationError::InvalidPassId { value });
        }

        Ok(Self(value))
    }

    /// Returns the identifier's stable textual representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PassId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Read-only context supplied to an optimization pass.
///
/// Options are explicitly supplied by the caller. Unknown options should be
/// rejected by the pass that owns their interpretation rather than silently
/// ignored when they affect correctness.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PassContext {
    options: BTreeMap<String, String>,
}

impl PassContext {
    /// Creates an empty context.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts or replaces an option.
    ///
    /// Option keys must not be empty or contain control characters.
    pub fn set_option(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> OptimizationResult<()> {
        let key = key.into();

        if key.trim().is_empty() || key.chars().any(char::is_control) {
            return Err(OptimizationError::InvalidOptionKey { key });
        }

        self.options.insert(key, value.into());
        Ok(())
    }

    /// Returns an option by key.
    #[must_use]
    pub fn option(&self, key: &str) -> Option<&str> {
        self.options.get(key).map(String::as_str)
    }

    /// Returns all options in deterministic key order.
    #[must_use]
    pub fn options(&self) -> &BTreeMap<String, String> {
        &self.options
    }
}

/// An adapter over an existing canonical IR.
///
/// Implementations must verify the invariants relevant to the selected IR.
/// Verification errors should identify the violated invariant in a stable,
/// useful way.
///
/// This trait intentionally exposes no IR representation or mutation API:
/// concrete passes may use a typed adapter of their own, while the manager
/// remains independent of the canonical IR's internal structure.
pub trait OptimizationTarget {
    /// Verifies structural and IR-specific invariants.
    ///
    /// This method must not silently repair invalid IR. Verification is
    /// observational and must not mutate the target.
    fn verify(&self) -> Result<(), String>;
}

/// Result of applying one optimization pass.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PassOutcome {
    /// Number of transformation units committed by the pass.
    ///
    /// A unit is defined by the concrete pass's documentation. It may mean
    /// rewritten operations, removed instructions, or another documented
    /// measure. Counts from different passes need not have identical meaning.
    pub changes: u64,

    /// Analysis names invalidated by this transformation.
    ///
    /// These names are opaque to the orchestrator. The compiler's analysis
    /// manager, if any, owns the authoritative analysis registry.
    pub invalidated_analyses: BTreeSet<String>,
}

impl PassOutcome {
    /// Constructs an outcome with no reported changes.
    #[must_use]
    pub fn unchanged() -> Self {
        Self::default()
    }

    /// Constructs an outcome reporting a number of changes.
    #[must_use]
    pub fn changed(changes: u64) -> Self {
        Self {
            changes,
            invalidated_analyses: BTreeSet::new(),
        }
    }

    /// Returns whether the pass reports any changes.
    #[must_use]
    pub fn has_changes(&self) -> bool {
        self.changes > 0
    }
}

/// Contract implemented by every concrete optimization pass.
///
/// A pass must:
///
/// - Have a stable, unique identifier within a pipeline.
/// - Preserve the language's observable semantics.
/// - Preserve required effects, resource intent, and numerical guarantees.
/// - Respect memory, aliasing, exception, and concurrency constraints.
/// - Report committed changes accurately.
/// - Return an error instead of silently accepting an illegal rewrite.
/// - Leave the target unchanged if execution returns an error.
///
/// A pass that cannot establish its transformation's legality must not apply
/// the transformation. The orchestration layer cannot prove semantic
/// equivalence merely from the implementation of this trait.
pub trait OptimizationPass: Send + Sync {
    /// Returns the pass's stable identifier.
    fn id(&self) -> &PassId;

    /// Returns a human-readable description.
    fn description(&self) -> &str;

    /// Applies the pass to a canonical-IR adapter.
    fn run(
        &self,
        target: &mut dyn OptimizationTarget,
        context: &PassContext,
    ) -> OptimizationResult<PassOutcome>;
}

/// Optional operational budgets for one optimization run.
///
/// An absent budget means this policy imposes no limit for that dimension.
/// It does not imply infinite memory, processing time, or host resources.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OptimizationBudget {
    /// Maximum number of pass invocations permitted in one run.
    pub max_pass_invocations: Option<u64>,

    /// Maximum cumulative number of reported transformation units.
    pub max_changes: Option<u64>,
}

impl OptimizationBudget {
    /// Constructs an unlimited budget under this policy.
    #[must_use]
    pub fn unlimited() -> Self {
        Self::default()
    }

    /// Constructs a budget with explicit invocation and change limits.
    #[must_use]
    pub fn new(
        max_pass_invocations: Option<u64>,
        max_changes: Option<u64>,
    ) -> Self {
        Self {
            max_pass_invocations,
            max_changes,
        }
    }
}

/// Verification policy for an optimization pipeline.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum VerificationPolicy {
    /// Verify the target before and after the complete pipeline.
    ///
    /// Concrete passes remain responsible for preserving invariants
    /// throughout their own transformations.
    #[default]
    PipelineBoundaries,

    /// Also verify after every successfully completed pass.
    ///
    /// This is more expensive but helps isolate the pass that introduced
    /// an invalid IR state.
    EveryPass,
}

/// Configuration for the pass manager.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PassManagerConfig {
    /// Operational budgets for this run.
    pub budget: OptimizationBudget,

    /// Verification frequency.
    pub verification: VerificationPolicy,
}

/// Per-pass accounting for a completed pass.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PassRecord {
    /// Identifier of the completed pass.
    pub pass_id: PassId,

    /// Number of committed changes reported by the pass.
    pub changes: u64,

    /// Analyses invalidated by the pass.
    pub invalidated_analyses: BTreeSet<String>,
}

/// Summary of a successfully completed optimization pipeline.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OptimizationReport {
    /// Records in actual execution order.
    pub passes: Vec<PassRecord>,

    /// Total reported transformation units.
    pub total_changes: u64,

    /// Total pass invocations.
    pub pass_invocations: u64,
}

impl OptimizationReport {
    /// Returns true when no pass reported a transformation.
    #[must_use]
    pub fn is_unchanged(&self) -> bool {
        self.total_changes == 0
    }
}

/// Failure from pass registration, execution, verification, or budgeting.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OptimizationError {
    /// A pass identifier is empty or contains control characters.
    InvalidPassId {
        /// Rejected identifier.
        value: String,
    },

    /// A duplicate pass identifier was registered.
    DuplicatePass {
        /// Identifier already registered.
        pass_id: PassId,
    },

    /// An option key is empty or contains control characters.
    InvalidOptionKey {
        /// Rejected option key.
        key: String,
    },

    /// The target was invalid before the pipeline started.
    InitialVerificationFailed {
        /// Description supplied by the target verifier.
        reason: String,
    },

    /// A pass failed while executing.
    PassFailed {
        /// Pass that returned the failure.
        pass_id: PassId,

        /// Underlying failure.
        reason: String,
    },

    /// A pass completed but the target failed verification.
    VerificationFailed {
        /// Pass whose output failed verification.
        pass_id: PassId,

        /// Underlying verification failure.
        reason: String,
    },

    /// The configured invocation budget would be exceeded.
    InvocationBudgetExceeded {
        /// Pass that could not be invoked.
        next_pass: PassId,

        /// Configured invocation limit.
        limit: u64,
    },

    /// The configured change budget would be exceeded.
    ChangeBudgetExceeded {
        /// Pass whose reported result exceeded the remaining budget.
        pass_id: PassId,

        /// Configured cumulative change limit.
        limit: u64,
    },

    /// An accounting operation exceeded the representable range.
    AccountingOverflow {
        /// Pass associated with the overflowing operation.
        pass_id: PassId,
    },
}

impl fmt::Display for OptimizationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPassId { value } => {
                write!(f, "invalid optimization pass identifier: {value:?}")
            }
            Self::DuplicatePass { pass_id } => {
                write!(f, "optimization pass registered more than once: {pass_id}")
            }
            Self::InvalidOptionKey { key } => {
                write!(f, "invalid optimization option key: {key:?}")
            }
            Self::InitialVerificationFailed { reason } => {
                write!(f, "optimization target failed initial verification: {reason}")
            }
            Self::PassFailed { pass_id, reason } => {
                write!(f, "optimization pass {pass_id} failed: {reason}")
            }
            Self::VerificationFailed { pass_id, reason } => {
                write!(
                    f,
                    "optimization pass {pass_id} produced an invalid target: {reason}"
                )
            }
            Self::InvocationBudgetExceeded { next_pass, limit } => {
                write!(
                    f,
                    "invoking pass {next_pass} would exceed the pass invocation budget of {limit}"
                )
            }
            Self::ChangeBudgetExceeded { pass_id, limit } => {
                write!(
                    f,
                    "pass {pass_id} would exceed the cumulative change budget of {limit}"
                )
            }
            Self::AccountingOverflow { pass_id } => {
                write!(f, "optimization accounting overflow in pass {pass_id}")
            }
        }
    }
}

impl Error for OptimizationError {}

/// Ordered optimization pipeline.
///
/// Registration order is execution order. This manager does not infer
/// dependencies between passes, reorder passes, or silently skip a pass.
///
/// A compiler that needs dependency-driven scheduling must resolve that order
/// in a separate, explicit pipeline-construction phase.
#[derive(Default)]
pub struct PassManager {
    config: PassManagerConfig,
    passes: Vec<Box<dyn OptimizationPass>>,
    registered_ids: BTreeSet<PassId>,
}

impl PassManager {
    /// Creates a manager with the supplied configuration.
    #[must_use]
    pub fn new(config: PassManagerConfig) -> Self {
        Self {
            config,
            passes: Vec::new(),
            registered_ids: BTreeSet::new(),
        }
    }

    /// Returns the manager configuration.
    #[must_use]
    pub fn config(&self) -> &PassManagerConfig {
        &self.config
    }

    /// Returns the number of registered passes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.passes.len()
    }

    /// Returns whether the pipeline has no registered passes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.passes.is_empty()
    }

    /// Returns registered pass identifiers in execution order.
    #[must_use]
    pub fn pass_ids(&self) -> Vec<&PassId> {
        self.passes.iter().map(|pass| pass.id()).collect()
    }

    /// Registers a pass at the end of the pipeline.
    ///
    /// Duplicate identifiers are rejected to avoid ambiguous reports,
    /// diagnostics, and configuration.
    pub fn register(
        &mut self,
        pass: Box<dyn OptimizationPass>,
    ) -> OptimizationResult<()> {
        let id = pass.id().clone();

        if !self.registered_ids.insert(id.clone()) {
            return Err(OptimizationError::DuplicatePass { pass_id: id });
        }

        self.passes.push(pass);
        Ok(())
    }

    /// Runs the registered pipeline.
    ///
    /// The target is verified before execution. Under `EveryPass`, each
    /// successful pass is followed by verification; under either policy,
    /// the final target is verified before success is returned.
    ///
    /// If a pass fails or a budget is exceeded, the run returns an error.
    /// The manager does not claim that the whole pipeline succeeded.
    ///
    /// Because arbitrary IR cannot be rolled back by this generic manager,
    /// every pass must honor its transactionality contract. Budget exhaustion
    /// is checked before invoking a pass where possible. A pass's change
    /// count can only be checked after it returns; a pass must therefore
    /// honor any finer-grained budget supplied through its own context or
    /// concrete IR adapter if it needs to stop before exceeding that budget.
    pub fn run(
        &self,
        target: &mut dyn OptimizationTarget,
        context: &PassContext,
    ) -> OptimizationResult<OptimizationReport> {
        target
            .verify()
            .map_err(|reason| OptimizationError::InitialVerificationFailed { reason })?;

        let mut report = OptimizationReport::default();

        for pass in &self.passes {
            let pass_id = pass.id().clone();

            if let Some(limit) = self.config.budget.max_pass_invocations {
                if report.pass_invocations >= limit {
                    return Err(OptimizationError::InvocationBudgetExceeded {
                        next_pass: pass_id,
                        limit,
                    });
                }
            }

            let outcome = pass.run(target, context).map_err(|error| {
                OptimizationError::PassFailed {
                    pass_id: pass_id.clone(),
                    reason: error.to_string(),
                }
            })?;

            let next_invocations = report
                .pass_invocations
                .checked_add(1)
                .ok_or_else(|| OptimizationError::AccountingOverflow {
                    pass_id: pass_id.clone(),
                })?;

            let next_changes = report
                .total_changes
                .checked_add(outcome.changes)
                .ok_or_else(|| OptimizationError::AccountingOverflow {
                    pass_id: pass_id.clone(),
                })?;

            if let Some(limit) = self.config.budget.max_changes {
                if next_changes > limit {
                    return Err(OptimizationError::ChangeBudgetExceeded {
                        pass_id,
                        limit,
                    });
                }
            }

            if self.config.verification == VerificationPolicy::EveryPass {
                target.verify().map_err(|reason| {
                    OptimizationError::VerificationFailed {
                        pass_id: pass_id.clone(),
                        reason,
                    }
                })?;
            }

            report.pass_invocations = next_invocations;
            report.total_changes = next_changes;
            report.passes.push(PassRecord {
                pass_id,
                changes: outcome.changes,
                invalidated_analyses: outcome.invalidated_analyses,
            });
        }

        target
            .verify()
            .map_err(|reason| OptimizationError::VerificationFailed {
                pass_id: PassId("pipeline.final-verification".to_owned()),
                reason,
            })?;

        Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct TestTarget {
        valid: bool,
        value: u64,
    }

    impl OptimizationTarget for TestTarget {
        fn verify(&self) -> Result<(), String> {
            if self.valid {
                Ok(())
            } else {
                Err("test target is marked invalid".to_owned())
            }
        }
    }

    struct IncrementPass {
        id: PassId,
        amount: u64,
    }

    impl IncrementPass {
        fn new(id: &str, amount: u64) -> Self {
            Self {
                id: PassId::new(id).expect("test pass identifier should be valid"),
                amount,
            }
        }
    }

    impl OptimizationPass for IncrementPass {
        fn id(&self) -> &PassId {
            &self.id
        }

        fn description(&self) -> &str {
            "Test-only target transformation"
        }

        fn run(
            &self,
            target: &mut dyn OptimizationTarget,
            _context: &PassContext,
        ) -> OptimizationResult<PassOutcome> {
            let _ = target;

            // This test pass deliberately does not mutate the opaque trait
            // object. Concrete production passes use their typed IR adapter.
            Ok(PassOutcome::changed(self.amount))
        }
    }

    struct InvalidatingPass {
        id: PassId,
    }

    impl OptimizationPass for InvalidatingPass {
        fn id(&self) -> &PassId {
            &self.id
        }

        fn description(&self) -> &str {
            "Test pass that reports an invalidated analysis"
        }

        fn run(
            &self,
            _target: &mut dyn OptimizationTarget,
            _context: &PassContext,
        ) -> OptimizationResult<PassOutcome> {
            let mut outcome = PassOutcome::changed(1);
            outcome
                .invalidated_analyses
                .insert("control-flow".to_owned());
            Ok(outcome)
        }
    }

    #[test]
    fn pass_ids_reject_empty_and_control_characters() {
        assert!(PassId::new("").is_err());
        assert!(PassId::new("   ").is_err());
        assert!(PassId::new("bad\nid").is_err());
        assert!(PassId::new("constant-folding.v2").is_ok());
    }

    #[test]
    fn options_are_accessible_by_key() {
        let mut context = PassContext::new();
        context
            .set_option("arithmetic.mode", "strict")
            .expect("valid option should be accepted");

        assert_eq!(context.option("arithmetic.mode"), Some("strict"));
        assert_eq!(context.option("unknown"), None);
    }

    #[test]
    fn duplicate_pass_ids_are_rejected() {
        let mut manager = PassManager::default();
        manager
            .register(Box::new(IncrementPass::new("fold", 1)))
            .expect("first registration should succeed");

        assert!(matches!(
            manager.register(Box::new(IncrementPass::new("fold", 2))),
            Err(OptimizationError::DuplicatePass { .. })
        ));
    }

    #[test]
    fn empty_pipeline_still_verifies_target() {
        let manager = PassManager::default();
        let mut target = TestTarget {
            valid: true,
            value: 0,
        };

        let report = manager
            .run(&mut target, &PassContext::new())
            .expect("valid target should pass");

        assert!(report.is_unchanged());
        assert_eq!(report.pass_invocations, 0);
    }

    #[test]
    fn invalid_initial_target_is_rejected() {
        let manager = PassManager::default();
        let mut target = TestTarget::default();

        assert!(matches!(
            manager.run(&mut target, &PassContext::new()),
            Err(OptimizationError::InitialVerificationFailed { .. })
        ));
    }

    #[test]
    fn change_budget_is_enforced() {
        let config = PassManagerConfig {
            budget: OptimizationBudget::new(None, Some(1)),
            verification: VerificationPolicy::EveryPass,
        };
        let mut manager = PassManager::new(config);

        manager
            .register(Box::new(IncrementPass::new("pass.one", 1)))
            .expect("registration should succeed");
        manager
            .register(Box::new(IncrementPass::new("pass.two", 1)))
            .expect("registration should succeed");

        let mut target = TestTarget {
            valid: true,
            value: 0,
        };

        assert!(matches!(
            manager.run(&mut target, &PassContext::new()),
            Err(OptimizationError::ChangeBudgetExceeded { limit: 1, .. })
        ));
    }

    #[test]
    fn invocation_budget_is_checked_before_running_next_pass() {
        let config = PassManagerConfig {
            budget: OptimizationBudget::new(Some(1), None),
            verification: VerificationPolicy::PipelineBoundaries,
        };
        let mut manager = PassManager::new(config);

        manager
            .register(Box::new(IncrementPass::new("pass.one", 0)))
            .expect("registration should succeed");
        manager
            .register(Box::new(IncrementPass::new("pass.two", 0)))
            .expect("registration should succeed");

        let mut target = TestTarget {
            valid: true,
            value: 0,
        };

        assert!(matches!(
            manager.run(&mut target, &PassContext::new()),
            Err(OptimizationError::InvocationBudgetExceeded { limit: 1, .. })
        ));
    }

    #[test]
    fn invalidated_analysis_is_reported() {
        let mut manager = PassManager::default();
        manager
            .register(Box::new(InvalidatingPass {
                id: PassId::new("test.invalidate").expect("valid id"),
            }))
            .expect("registration should succeed");

        let mut target = TestTarget {
            valid: true,
            value: 0,
        };

        let report = manager
            .run(&mut target, &PassContext::new())
            .expect("pipeline should succeed");

        assert!(report.passes[0]
            .invalidated_analyses
            .contains("control-flow"));
    }

    #[test]
    fn registration_order_is_preserved() {
        let mut manager = PassManager::default();
        manager
            .register(Box::new(IncrementPass::new("first", 0)))
            .expect("registration should succeed");
        manager
            .register(Box::new(IncrementPass::new("second", 0)))
            .expect("registration should succeed");

        let ids: Vec<String> = manager
            .pass_ids()
            .into_iter()
            .map(|id| id.as_str().to_owned())
            .collect();

        assert_eq!(ids, ["first", "second"]);
    }
}