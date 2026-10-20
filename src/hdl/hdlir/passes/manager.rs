#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # HDLIR Transformation Pass Manager
//!
//! Path: `src/hdl/hdlir/passes/manager.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Coordinate one HDLIR transformation application as a checked transaction:
//!
//! 1. Validate the pass declaration.
//! 2. Prepare required analyses for the input revision.
//! 3. Evaluate declared preconditions.
//! 4. Execute the pass to produce a candidate without mutating the input
//!    through the pass API.
//! 5. Evaluate postconditions.
//! 6. Discharge any declared semantic-refinement proof obligation.
//! 7. Check all declared preservation obligations.
//! 8. Invalidate analyses made stale by the accepted transformation.
//! 9. Return the candidate only after all mandatory checks succeed.
//!
//! This module does not implement HDL operations, graph verification,
//! equivalence algorithms, analysis algorithms, or physical realization.
//! Those responsibilities remain with their owning implementations.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`.
//! - `grammar/spec/hdl.md`.
//! - `grammar/hdl/hdl.g4` and applicable delegate grammars.
//! - `grammar/hdl/sequential.g4`.
//! - `grammar/hdl/processes.g4`.
//! - `src/hdl/hdlir/HDLIR-SPEC.md`.
//! - `src/hdl/hdlir/config.rs`.
//! - `src/hdl/hdlir/errors.rs`.
//! - `src/hdl/hdlir/passes/contracts.rs`.
//! - `src/hdl/hdlir/passes/preservation.rs`.
//! - `src/zuir/ZUIR-SPEC.md`.
//!
//! ## Ownership boundaries
//!
//! This manager owns orchestration and acceptance of pass candidates. It does
//! not own source parsing, semantic analysis, HDLIR graph storage, operation
//! semantics, target selection, synthesis, placement, routing, or timing
//! closure.
//!
//! The grammar and canonical frontend own source-language meaning. ZUIR owns
//! universal computational meaning. HDLIR owns hardware-domain meaning.
//! Downstream realization owns physical implementation.
//!
//! ## Transaction and soundness rules
//!
//! A pass must produce a candidate rather than mutate the authoritative input
//! through this API. The manager returns that candidate only after all
//! mandatory checks succeed. The caller remains responsible for committing
//! the returned candidate to its owning design or revision store.
//!
//! A preservation provider is trusted to implement its documented checking
//! method correctly. This manager cannot turn an unsound provider into a
//! sound proof engine. Inconclusive evidence is never success.
//!
//! ## Scalability
//!
//! No hardware-capacity limits are imposed. The manager counts pass
//! applications with checked arithmetic and consults the explicitly
//! configured transformation-application budget. IR-specific work, memory,
//! verification, and analysis budgets must be enforced by the corresponding
//! providers and implementations.
//!
//! The manager is intentionally single-application and generic over the IR
//! snapshot type. This avoids assuming an unfinished concrete HDLIR graph API.
//! A pipeline may call it repeatedly, retaining the returned candidate as
//! the next input revision.
//!
//! ## Integration
//!
//! - `passes/contracts.rs` supplies immutable pass declarations.
//! - `passes/preservation.rs` supplies preservation checking.
//! - `config.rs` supplies operational budgets.
//! - The IR owner supplies the concrete IR type and commit/revision policy.
//! - Analysis implementations supply an `AnalysisCoordinator`.
//! - Contract evaluators supply executable meanings for named conditions and
//!   proof obligations.
//! - Transformation implementations supply an `HdlirPass`.
//!
//! No evaluator is implicitly treated as available merely because a contract
//! declares a condition or analysis.
//!
//! ## Completion criteria
//!
//! This module is complete when it is registered through a real
//! `passes/mod.rs`, its provider implementations are connected to real HDLIR
//! verification and analysis APIs, its tests exercise real IR revisions, and
//! repository-wide formatting, checking, testing, and Clippy all pass.

use std::fmt;

use crate::hdl::hdlir::config::{BudgetKind, HdlirConfig};
use crate::hdl::hdlir::errors::{HdlirBudgetKind, HdlirError, HdlirResult};
use crate::hdl::hdlir::passes::contracts::{
    AnalysisRequirement, ConditionStrength, ContractCondition, PassContract,
    SemanticGuarantee,
};
use crate::hdl::hdlir::passes::preservation::{
    PreservationChecker, PreservationEvidenceProvider, PreservationGuarantee,
    PreservationReport,
};

// =============================================================================
// Evaluation contracts
// =============================================================================

/// Identifies the stage at which a declared pass condition is evaluated.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ConditionStage {
    /// A condition that must hold before the transformation runs.
    Precondition,

    /// A condition that must hold for the candidate before it is accepted.
    Postcondition,
}

/// Result of evaluating one declared pass condition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConditionResult {
    /// The condition has been established.
    Satisfied,

    /// The condition has been established not to hold.
    Unsatisfied {
        /// Actionable explanation suitable for diagnostics.
        detail: String,
    },

    /// The evaluator could not establish whether the condition holds.
    Inconclusive {
        /// Explanation of the missing evidence or incomplete check.
        detail: String,
    },
}

impl ConditionResult {
    /// Construct a negative condition result with a diagnostic explanation.
    #[must_use]
    pub fn unsatisfied(detail: impl Into<String>) -> Self {
        Self::Unsatisfied {
            detail: detail.into(),
        }
    }

    /// Construct an inconclusive condition result.
    #[must_use]
    pub fn inconclusive(detail: impl Into<String>) -> Self {
        Self::Inconclusive {
            detail: detail.into(),
        }
    }

    /// Return whether the condition was conclusively satisfied.
    #[must_use]
    pub const fn is_satisfied(&self) -> bool {
        matches!(self, Self::Satisfied)
    }
}

/// Provides executable semantics for named conditions and proof obligations.
///
/// Contract descriptions are human-readable documentation, not executable
/// predicates. Implementations must dispatch using the stable condition key
/// and the applicable IR semantics.
///
/// A condition evaluator must not report `Satisfied` when its required check
/// did not complete. Budget exhaustion may be represented as `Inconclusive`
/// when that is the appropriate recoverable result, or as a structured error
/// when evaluation cannot continue.
pub trait ContractEvaluator<I> {
    /// Evaluate a named condition against one immutable IR revision.
    ///
    /// # Errors
    ///
    /// Returns a structured error if evaluation itself cannot proceed.
    fn evaluate_condition(
        &mut self,
        condition: &ContractCondition,
        stage: ConditionStage,
        ir: &I,
    ) -> HdlirResult<ConditionResult>;

    /// Discharge a declared semantic-refinement proof obligation.
    ///
    /// The default implementation refuses to certify a proof obligation for
    /// which no proof evaluator has been supplied. Implementations must only
    /// return `Ok(())` when the named obligation has actually been discharged
    /// for the candidate and the relevant before/after boundary.
    ///
    /// # Errors
    ///
    /// Returns an error if the proof is invalid, unsupported, or incomplete.
    fn verify_refinement(
        &mut self,
        _key: &crate::hdl::hdlir::passes::contracts::ContractKey,
        _before: &I,
        _after: &I,
    ) -> HdlirResult<()> {
        Err(HdlirError::incomplete(
            "pass_refinement",
            "the pass declares a refinement proof obligation, but no proof evaluator is implemented",
        ))
    }
}

/// Prepares required analyses and invalidates stale analysis results.
///
/// Implementations own analysis storage, revision tracking, dependency graphs,
/// and analysis algorithms. A matching analysis key alone does not establish
/// that its result is current for the exact IR revision.
pub trait AnalysisCoordinator<I> {
    /// Ensure that every required analysis is available and valid for `ir`.
    ///
    /// Implementations must honor each requirement's declared freshness
    /// policy and report missing or incomplete analyses explicitly.
    ///
    /// # Errors
    ///
    /// Returns an error if a required analysis cannot be prepared.
    fn prepare(
        &mut self,
        requirements: &[AnalysisRequirement],
        ir: &I,
    ) -> HdlirResult<()>;

    /// Invalidate analysis results made stale by an accepted transformation.
    ///
    /// This method is called only after all candidate checks have succeeded.
    /// An implementation should perform invalidation atomically or return an
    /// error without leaving its analysis state inconsistent.
    ///
    /// # Errors
    ///
    /// Returns an error if invalidation cannot be completed safely.
    fn invalidate(
        &mut self,
        keys: &std::collections::BTreeSet<
            crate::hdl::hdlir::passes::contracts::ContractKey,
        >,
    ) -> HdlirResult<()>;
}

/// A transformation that creates a candidate IR revision.
///
/// Implementations must not mutate the input through interior mutability or
/// external shared state. They should treat it as an immutable snapshot and
/// return a distinct candidate. The Rust type system cannot enforce the
/// absence of all interior-mutability side effects; this requirement is part
/// of the pass implementation contract.
pub trait HdlirPass<I, C> {
    /// Return the immutable declaration for this transformation.
    fn contract(&self) -> &PassContract;

    /// Execute the transformation and produce a candidate IR revision.
    ///
    /// This method must not commit the candidate to an authoritative design
    /// store. The manager owns acceptance; the caller owns final commitment.
    ///
    /// # Errors
    ///
    /// Returns an error if the transformation cannot produce a candidate.
    fn execute(&self, input: &I, context: &mut C) -> HdlirResult<I>;
}

// =============================================================================
// Reports
// =============================================================================

/// A diagnostic from a failed or advisory pass condition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConditionDiagnostic {
    stage: ConditionStage,
    key: String,
    strength: ConditionStrength,
    result: ConditionResult,
}

impl ConditionDiagnostic {
    /// Return the stage at which the condition was evaluated.
    #[must_use]
    pub const fn stage(&self) -> ConditionStage {
        self.stage
    }

    /// Return the stable condition identifier.
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Return whether the condition is mandatory or advisory.
    #[must_use]
    pub const fn strength(&self) -> ConditionStrength {
        self.strength
    }

    /// Return the evaluation result.
    #[must_use]
    pub const fn result(&self) -> &ConditionResult {
        &self.result
    }
}

/// Accepted candidate and evidence from one pass application.
///
/// A value of this type is returned only after the manager has passed the
/// mandatory checks. `candidate` is not automatically installed into a
/// design; the caller owns that commit boundary.
#[derive(Clone, Debug)]
pub struct PassExecutionReport<I> {
    candidate: I,
    preservation: PreservationReport,
    condition_diagnostics: Vec<ConditionDiagnostic>,
    application_index: u64,
}

impl<I> PassExecutionReport<I> {
    /// Borrow the accepted candidate IR revision.
    #[must_use]
    pub const fn candidate(&self) -> &I {
        &self.candidate
    }

    /// Consume the report and return its accepted candidate.
    #[must_use]
    pub fn into_candidate(self) -> I {
        self.candidate
    }

    /// Return the preservation report that passed the acceptance gate.
    #[must_use]
    pub const fn preservation(&self) -> &PreservationReport {
        &self.preservation
    }

    /// Return advisory condition diagnostics in deterministic declaration
    /// order, followed by the postcondition advisory diagnostics.
    #[must_use]
    pub fn condition_diagnostics(&self) -> &[ConditionDiagnostic] {
        &self.condition_diagnostics
    }

    /// Return the one-based application number within this manager instance.
    #[must_use]
    pub const fn application_index(&self) -> u64 {
        self.application_index
    }
}

/// Result of evaluating a single condition collection.
#[derive(Default)]
struct ConditionSummary {
    diagnostics: Vec<ConditionDiagnostic>,
}

// =============================================================================
// Manager
// =============================================================================

/// Coordinates contract-checked HDLIR transformation applications.
///
/// The manager stores only immutable policy and a checked application count.
/// It does not own an IR, a global pass registry, a scheduler, or shared mutable
/// process state.
#[derive(Clone, Debug)]
pub struct PassManager {
    config: HdlirConfig,
    applications: u64,
    required_preservation: Vec<PreservationGuarantee>,
}

impl PassManager {
    /// Create a pass manager with the supplied operational configuration.
    #[must_use]
    pub fn new(config: HdlirConfig) -> Self {
        Self {
            config,
            applications: 0,
            required_preservation: Vec::new(),
        }
    }

    /// Create a pass manager with default HDLIR policies.
    #[must_use]
    pub fn with_defaults() -> Self {
        Self::new(HdlirConfig::default())
    }

    /// Return the immutable configuration used by this manager.
    #[must_use]
    pub const fn config(&self) -> &HdlirConfig {
        &self.config
    }

    /// Return the number of successfully accepted pass applications.
    #[must_use]
    pub const fn applications(&self) -> u64 {
        self.applications
    }

    /// Add a pipeline-level preservation obligation.
    ///
    /// These guarantees are required in addition to the pass's own declared
    /// guarantees. Duplicate declarations are harmless and are deduplicated
    /// by the preservation checker.
    #[must_use]
    pub fn require_preservation(
        mut self,
        guarantee: PreservationGuarantee,
    ) -> Self {
        self.required_preservation.push(guarantee);
        self
    }

    /// Apply one pass and return its candidate only after mandatory checks.
    ///
    /// The caller supplies the pass implementation, execution context,
    /// analysis coordinator, contract evaluator, and preservation provider.
    /// Those dependencies are explicit so the manager cannot silently invent
    /// missing analysis or proof capabilities.
    ///
    /// # Acceptance sequence
    ///
    /// 1. Validate the declaration.
    /// 2. Check the configured transformation-application budget.
    /// 3. Prepare required analyses for the input.
    /// 4. Evaluate preconditions.
    /// 5. Execute the pass.
    /// 6. Evaluate postconditions.
    /// 7. Verify any declared refinement proof.
    /// 8. Check and require all preservation guarantees.
    /// 9. Invalidate declared stale analyses.
    /// 10. Increment the accepted-application count and return the candidate.
    ///
    /// The manager does not commit the candidate to the caller's design.
    /// On a mandatory check failure, no candidate is returned as accepted.
    ///
    /// # Errors
    ///
    /// Returns a structured error for invalid declarations, failed required
    /// conditions, missing analyses, pass failures, failed proof obligations,
    /// violated or inconclusive preservation, budget exhaustion, or failed
    /// analysis invalidation.
    pub fn run<I, C, P, A, E>(
        &mut self,
        pass: &P,
        input: &I,
        context: &mut C,
        analyses: &mut A,
        evaluator: &mut E,
        preservation_provider: &mut dyn PreservationEvidenceProvider<I>,
    ) -> HdlirResult<PassExecutionReport<I>>
    where
        I: Clone,
        P: HdlirPass<I, C>,
        A: AnalysisCoordinator<I>,
        E: ContractEvaluator<I>,
    {
        let contract = pass.contract();
        contract.validate_declaration()?;

        self.check_application_budget()?;

        // The preservation boundary uses an immutable before-snapshot.
        // Pass implementations are additionally required not to mutate the
        // supplied input through interior mutability or external state.
        let before = input.clone();

        analyses.prepare(contract.required_analyses(), input)?;

        let mut diagnostics = Vec::new();
        self.evaluate_conditions(
            contract.preconditions(),
            ConditionStage::Precondition,
            input,
            evaluator,
            &mut diagnostics,
        )?;

        let candidate = pass.execute(input, context)?;

        self.evaluate_conditions(
            contract.postconditions(),
            ConditionStage::Postcondition,
            &candidate,
            evaluator,
            &mut diagnostics,
        )?;

        if let SemanticGuarantee::RefinesUnderProof(key) =
            contract.semantic_guarantee()
        {
            evaluator.verify_refinement(key, &before, &candidate)?;
        }

        let preservation = PreservationChecker.check(
            contract,
            &before,
            &candidate,
            self.required_preservation.iter().cloned(),
            preservation_provider,
        )?;

        // An inconclusive report is not accepted. This call rejects both
        // definite violations and incomplete evidence.
        preservation.require_preserved()?;

        // Invalidate only after the candidate has passed all checks. If
        // invalidation fails, the candidate is not returned as accepted.
        analyses.invalidate(contract.invalidated_analyses())?;

        self.applications = self.applications.checked_add(1).ok_or_else(|| {
            HdlirError::invalid_invariant(
                "pass_application_counter",
                "accepted pass application counter overflowed",
            )
        })?;

        Ok(PassExecutionReport {
            candidate,
            preservation,
            condition_diagnostics: diagnostics,
            application_index: self.applications,
        })
    }

    fn check_application_budget(&self) -> HdlirResult<()> {
        let next = self.applications.checked_add(1).ok_or_else(|| {
            HdlirError::invalid_invariant(
                "pass_application_counter",
                "pass application counter cannot represent the next application",
            )
        })?;

        let kind = BudgetKind::TransformationApplications;
        let budget = self.config.budget(&kind);

        if let Some(limit) = budget.limit() {
            if next > limit {
                return Err(HdlirError::budget_exceeded(
                    HdlirBudgetKind::Transformation,
                    Some(u128::from(limit)),
                    Some(u128::from(next)),
                    "HDLIR pass application",
                ));
            }
        }

        Ok(())
    }

    fn evaluate_conditions<I, E>(
        &self,
        conditions: &[ContractCondition],
        stage: ConditionStage,
        ir: &I,
        evaluator: &mut E,
        diagnostics: &mut Vec<ConditionDiagnostic>,
    ) -> HdlirResult<()>
    where
        E: ContractEvaluator<I>,
    {
        for condition in conditions {
            let result = evaluator.evaluate_condition(condition, stage, ir)?;

            if result.is_satisfied() {
                continue;
            }

            let diagnostic = ConditionDiagnostic {
                stage,
                key: condition.key().to_string(),
                strength: condition.strength(),
                result,
            };

            if condition.strength() == ConditionStrength::Required {
                return Err(condition_failure(&diagnostic));
            }

            diagnostics.push(diagnostic);
        }

        Ok(())
    }
}

fn condition_failure(diagnostic: &ConditionDiagnostic) -> HdlirError {
    let detail = match &diagnostic.result {
        ConditionResult::Satisfied => {
            "internal error: satisfied condition was treated as a failure"
                .to_owned()
        }
        ConditionResult::Unsatisfied { detail } => detail.clone(),
        ConditionResult::Inconclusive { detail } => {
            format!("condition evaluation was inconclusive: {detail}")
        }
    };

    HdlirError::invalid_invariant(
        "pass_condition",
        format!(
            "{:?} `{}` failed for a required condition: {}",
            diagnostic.stage, diagnostic.key, detail
        ),
    )
}

impl fmt::Display for ConditionStage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Precondition => formatter.write_str("precondition"),
            Self::Postcondition => formatter.write_str("postcondition"),
        }
    }
}