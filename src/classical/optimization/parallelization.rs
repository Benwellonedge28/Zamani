
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! Zamani Classical Computing — Parallelization Planning.
//!
//! Path: `src/classical/optimization/parallelization.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Assess and plan legal parallelization opportunities independently of
//! physical worker counts, machine topology, and target-specific execution.
//!
//! This module separates:
//!
//! - parallelization legality from target feasibility;
//! - logical parallelism from physical execution width;
//! - candidate assessment from canonical-IR mutation;
//! - parallel execution models from vendor-specific implementations;
//! - planning from scheduling, placement, and resource allocation;
//! - semantic determinism from incidental execution order.
//!
//! # Authority
//!
//! - `grammar/statements/loops.g4`: canonical loop syntax.
//! - `grammar/spec/classical.md`: classical computation requirements.
//! - `grammar/specification/semantic-model.md`: source-language semantics.
//! - `grammar/specification/scalability-model.md`: scalability contract.
//! - `grammar/spec/resources.md`: resource and capability semantics.
//! - `src/classical/optimization/legality.rs`: transformation legality.
//! - `src/classical/optimization/passes.rs`: pass orchestration.
//! - `src/classical/concurrency/data_parallel.rs`: logical data partitions.
//! - `src/classical/concurrency/determinism.rs`: deterministic ordering.
//! - `src/classical/execution/requirements.rs`: execution requirements.
//! - `src/classical/execution/capabilities.rs`: capability assessment.
//! - `src/zuir/`, when selected by the compilation pipeline.
//! - `src/quantum/ir/classical/`: existing canonical classical IR boundary.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - parallelization opportunity categories;
//! - target-independent parallel execution model descriptions;
//! - parallelization candidates and their legality obligations;
//! - explicit, deterministic model-selection policy;
//! - immutable parallelization plans;
//! - the transactional adapter contract for applying a plan;
//! - structured planning and outcome errors.
//!
//! This module does not own:
//!
//! - source syntax, AST, semantic type inference, or source-language meaning;
//! - a competing classical IR, CFG, or ZUIR representation;
//! - dependence, alias, effect, ownership, or concurrency analyses;
//! - proof of semantic equivalence;
//! - task identities, task lifecycle, synchronization, or channels;
//! - physical worker counts, resource discovery, placement, or scheduling;
//! - runtime execution, target lowering, or fallback execution;
//! - numerical reproducibility algorithms.
//!
//! # Legality
//!
//! A parallelization candidate is not legal merely because its iterations
//! appear independent. The responsible analyses must supply evidence for
//! every obligation in the candidate's transformation contract.
//!
//! Missing or unknown evidence fails closed. This module consumes evidence;
//! it does not manufacture proof or execute dependence analysis itself.
//!
//! Reduction candidates additionally require proof that integer behavior,
//! floating-point behavior, and required determinism are preserved.
//! Other transformations can add obligations through `require`.
//!
//! # Scalability and POCO-REAF
//!
//! No universal ceiling is imposed on logical parallelism, operations,
//! partitions, workers, devices, nodes, or supported execution models.
//!
//! This module does not store a universal worker count or assume that a
//! target can execute every planned model. Capability and resource feasibility
//! are separate questions for the execution planner and backend.
//!
//! Finite host representations, available memory, configured operational
//! budgets, and actual target limits remain real constraints. They must not
//! silently become source-language semantic restrictions.
//!
//! # Transactionality
//!
//! Planning never mutates IR. A concrete adapter must validate that a plan
//! applies to the current IR revision, apply it transactionally, verify the
//! result, and leave the original IR unchanged if application fails.
//!
//! A plan is not evidence that the transformation was applied.
//!
//! # Integration
//!
//! - `optimization/mod.rs` must register `pub mod parallelization;`.
//! - `optimization/legality.rs` supplies legality evidence and reports.
//! - `optimization/passes.rs` owns pass orchestration, not IR traversal.
//! - `concurrency/data_parallel.rs` owns logical index domains and partitions.
//! - `concurrency/task.rs` owns task identity and lifecycle.
//! - `concurrency/determinism.rs` owns deterministic ordering contracts.
//! - `execution/requirements.rs` and `execution/capabilities.rs` supply
//!   execution requirements and capability assessment.
//! - A canonical-IR adapter supplies current-revision candidates and analysis
//!   evidence, then applies accepted plans.
//! - The scheduler selects physical execution based on dependencies,
//!   capabilities, resource availability, constraints, and policy.
//!
//! This module deliberately does not depend on unfinished APIs from those
//! modules. A concrete adapter is required before this planner can rewrite IR.

use core::fmt;
use std::error::Error;
use std::collections::BTreeSet;

use super::legality::{
    assess_legality, LegalityError, LegalityEvidence, LegalityProperty,
    LegalityReport, LegalityStatus, ProofEvidence, TransformationContract,
};
use super::passes::PassId;

/// Result type for parallelization planning and validation.
pub type ParallelizationResult<T> = Result<T, ParallelizationError>;

/// Broad category of a potential parallelization transformation.
///
/// These categories describe logical opportunities, not a particular
/// instruction set, thread library, device, or scheduling implementation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum ParallelizationKind {
    /// Distribute independent loop iterations.
    LoopIterations,

    /// Distribute independent elements of a logical data domain.
    DataParallel,

    /// Expose independent tasks or computations.
    TaskParallel,

    /// Overlap independent stages while preserving required dependencies.
    Pipeline,

    /// Parallel reduction with explicitly preserved numeric semantics.
    Reduction,

    /// An adapter-defined parallelization opportunity.
    Extension,
}

/// A logical execution model a target may support.
///
/// These values are capability categories, not promises that any particular
/// target can execute the model. `Extension` permits target-independent
/// integrations to represent additional models without changing this enum.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum ParallelExecutionModel {
    /// Independent logical loop iterations.
    ParallelLoop,

    /// Work partitioned over a logical data domain.
    DataParallel,

    /// Multiple independent logical tasks.
    TaskParallel,

    /// Multiple dependent stages overlap subject to a valid pipeline schedule.
    Pipeline,

    /// A model capable of distributing work across execution locations.
    Distributed,

    /// A model delegated to a suitable accelerator backend.
    Accelerator,

    /// A registered extension-defined model.
    Extension,
}

/// Explicit preference policy for choosing a parallel execution model.
///
/// The default ordering is a configurable heuristic, not a semantic guarantee
/// and not a claim that any model is universally faster.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParallelizationPolicy {
    model_preference: Vec<ParallelExecutionModel>,
    require_parallelization: bool,
}

impl Default for ParallelizationPolicy {
    fn default() -> Self {
        Self {
            model_preference: vec![
                ParallelExecutionModel::DataParallel,
                ParallelExecutionModel::ParallelLoop,
                ParallelExecutionModel::TaskParallel,
                ParallelExecutionModel::Pipeline,
                ParallelExecutionModel::Distributed,
                ParallelExecutionModel::Accelerator,
                ParallelExecutionModel::Extension,
            ],
            require_parallelization: false,
        }
    }
}

impl ParallelizationPolicy {
    /// Creates the default, advisory policy.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Replaces the preference order used during deterministic selection.
    ///
    /// Duplicate entries are ignored during selection. An empty list means
    /// that no model is preferred by this policy.
    pub fn set_model_preference(
        &mut self,
        preference: impl IntoIterator<Item = ParallelExecutionModel>,
    ) {
        self.model_preference = preference.into_iter().collect();
    }

    /// Returns the configured model preference order.
    #[must_use]
    pub fn model_preference(&self) -> &[ParallelExecutionModel] {
        &self.model_preference
    }

    /// Configures whether the caller requires a parallelization plan.
    ///
    /// This cannot override legality or target capability checks.
    pub fn set_require_parallelization(&mut self, required: bool) {
        self.require_parallelization = required;
    }

    /// Returns whether a parallelization plan is required.
    #[must_use]
    pub fn requires_parallelization(&self) -> bool {
        self.require_parallelization
    }
}

/// Target-specific parallel execution capabilities.
///
/// An empty profile means that no model support has been established by this
/// profile. It does not mean that the target is universally incapable of
/// parallel execution.
///
/// This profile describes capabilities only. It does not reserve resources,
/// choose a worker count, or perform target discovery.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ParallelizationTargetProfile {
    supported_models: BTreeSet<ParallelExecutionModel>,
    preferred_model: Option<ParallelExecutionModel>,
}

impl ParallelizationTargetProfile {
    /// Creates a profile without asserted parallel execution capabilities.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a supported parallel execution model.
    pub fn support_model(&mut self, model: ParallelExecutionModel) {
        self.supported_models.insert(model);
    }

    /// Sets an advisory preferred model.
    ///
    /// A preferred model is selected only when it is supported and appears
    /// in the policy's preference order.
    pub fn set_preferred_model(
        &mut self,
        model: Option<ParallelExecutionModel>,
    ) {
        self.preferred_model = model;
    }

    /// Returns the supported execution models in deterministic order.
    #[must_use]
    pub fn supported_models(&self) -> &BTreeSet<ParallelExecutionModel> {
        &self.supported_models
    }

    /// Returns the preferred model, if configured.
    #[must_use]
    pub fn preferred_model(&self) -> Option<ParallelExecutionModel> {
        self.preferred_model
    }

    /// Returns whether this profile establishes support for a model.
    #[must_use]
    pub fn supports_model(&self, model: ParallelExecutionModel) -> bool {
        self.supported_models.contains(&model)
    }
}

/// A potential parallelization transformation and its proof obligations.
///
/// `id` is an adapter-defined stable identifier for the candidate in a
/// particular IR revision. It is not a canonical IR operation ID.
///
/// The adapter must ensure that candidate identity, evidence, input revision,
/// semantic context, and configuration all refer to the same transformation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParallelizationCandidate {
    id: String,
    kind: ParallelizationKind,
    contract: TransformationContract,
    evidence: LegalityEvidence,
}

impl ParallelizationCandidate {
    /// Creates a candidate with conservative default obligations.
    ///
    /// Every candidate requires semantic preservation, type correctness,
    /// effects, memory safety, aliasing, control flow, exception/trap
    /// behavior, concurrency semantics, determinism, and resource semantics.
    ///
    /// Reduction candidates additionally require integer and floating-point
    /// semantics. Extra obligations may be added by an adapter.
    pub fn new(
        id: impl Into<String>,
        kind: ParallelizationKind,
        pass_id: PassId,
    ) -> ParallelizationResult<Self> {
        let id = id.into();

        if id.trim().is_empty() || id.chars().any(char::is_control) {
            return Err(ParallelizationError::InvalidCandidateId { id });
        }

        let mut contract = TransformationContract::new(pass_id);

        contract.require_all([
            LegalityProperty::TypeCorrectness,
            LegalityProperty::Effects,
            LegalityProperty::MemorySafety,
            LegalityProperty::Aliasing,
            LegalityProperty::ControlFlow,
            LegalityProperty::ExceptionsAndTraps,
            LegalityProperty::Concurrency,
            LegalityProperty::Determinism,
            LegalityProperty::ResourceSemantics,
        ]);

        if kind == ParallelizationKind::Reduction {
            contract.require_all([
                LegalityProperty::IntegerSemantics,
                LegalityProperty::FloatingPointSemantics,
            ]);
        }

        Ok(Self {
            id,
            kind,
            contract,
            evidence: LegalityEvidence::new(),
        })
    }

    /// Returns the adapter-defined candidate identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the candidate category.
    #[must_use]
    pub const fn kind(&self) -> ParallelizationKind {
        self.kind
    }

    /// Returns the candidate's transformation contract.
    #[must_use]
    pub fn contract(&self) -> &TransformationContract {
        &self.contract
    }

    /// Returns the supplied legality evidence.
    #[must_use]
    pub fn evidence(&self) -> &LegalityEvidence {
        &self.evidence
    }

    /// Adds an additional required legality property.
    ///
    /// This is intended for adapter-specific proof obligations, such as a
    /// domain-specific dependence or ordering invariant.
    pub fn require(&mut self, property: LegalityProperty) {
        self.contract.require(property);
    }

    /// Records evidence supplied by an authoritative analysis or trusted rule.
    ///
    /// This method records a claim; it does not verify the analysis or prove
    /// the property. Evidence producers must bind the evidence to the current
    /// IR revision, candidate, semantic context, and configuration.
    pub fn record_evidence(
        &mut self,
        property: LegalityProperty,
        evidence: ProofEvidence,
    ) {
        self.evidence.record(property, evidence);
    }

    /// Assesses all declared legality obligations.
    #[must_use]
    pub fn assess_legality(&self) -> LegalityReport {
        assess_legality(&self.contract, &self.evidence)
    }
}

/// An immutable plan for a legal parallelization opportunity.
///
/// The plan expresses a logical execution model only. It does not encode
/// physical worker counts, machine IDs, device placement, partition grain,
/// task IDs, or a scheduling order.
///
/// The concrete adapter and scheduler own those decisions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParallelizationPlan {
    candidate_id: String,
    kind: ParallelizationKind,
    model: ParallelExecutionModel,
    pass_id: PassId,
}

impl ParallelizationPlan {
    /// Returns the candidate identifier to which this plan applies.
    #[must_use]
    pub fn candidate_id(&self) -> &str {
        &self.candidate_id
    }

    /// Returns the opportunity category.
    #[must_use]
    pub const fn kind(&self) -> ParallelizationKind {
        self.kind
    }

    /// Returns the selected logical execution model.
    #[must_use]
    pub const fn model(&self) -> ParallelExecutionModel {
        self.model
    }

    /// Returns the optimization pass identifier.
    #[must_use]
    pub fn pass_id(&self) -> &PassId {
        &self.pass_id
    }
}

/// Result of assessing a parallelization opportunity.
///
/// A non-plan result is not automatically a source-language error. The
/// caller may retain a legal existing implementation, assess another target,
/// or report infeasibility when parallel execution was explicitly required.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParallelizationDecision {
    /// A legal plan was selected for the supplied target profile.
    Planned(ParallelizationPlan),

    /// Legality is disproven or remains unproven.
    NotLegal(LegalityReport),

    /// No supported model was established by the target profile.
    UnsupportedTarget,

    /// The configured policy could not be satisfied by the profile.
    PolicyNotSatisfied,
}

/// Selects a legal parallelization model without modifying the IR.
///
/// Legality is assessed before capability selection. The function returns a
/// plan only if every declared obligation is proven and the target profile
/// establishes support for the selected model.
///
/// A preferred model is considered first only if it is also present in the
/// policy's explicit preference order. Otherwise, the configured order applies.
/// No hash iteration or physical discovery order affects the result.
pub fn plan_parallelization(
    candidate: &ParallelizationCandidate,
    target: &ParallelizationTargetProfile,
    policy: &ParallelizationPolicy,
) -> ParallelizationResult<ParallelizationDecision> {
    let legality = candidate.assess_legality();

    if legality.status() != LegalityStatus::Legal {
        return Ok(ParallelizationDecision::NotLegal(legality));
    }

    let preferred = target.preferred_model();

    let selected = policy
        .model_preference()
        .iter()
        .copied()
        .find(|model| {
            target.supports_model(*model)
                && Some(*model) == preferred
        })
        .or_else(|| {
            policy
                .model_preference()
                .iter()
                .copied()
                .find(|model| target.supports_model(*model))
        });

    match selected {
        Some(model) => Ok(ParallelizationDecision::Planned(
            ParallelizationPlan {
                candidate_id: candidate.id.clone(),
                kind: candidate.kind,
                model,
                pass_id: candidate.contract.pass_id().clone(),
            },
        )),
        None if policy.requires_parallelization() => {
            Ok(ParallelizationDecision::PolicyNotSatisfied)
        }
        None => Ok(ParallelizationDecision::UnsupportedTarget),
    }
}

/// Contract for a canonical-IR adapter that applies an accepted plan.
///
/// Implementations must:
///
/// - operate on the selected canonical IR, not a duplicate IR;
/// - reject a plan if its candidate or input revision is stale;
/// - revalidate the evidence and assumptions on which the plan depends;
/// - stage changes and apply them transactionally;
/// - preserve every obligation required by the transformation contract;
/// - verify the transformed IR before committing;
/// - leave the original IR unchanged if application fails.
///
/// A successful result means the transformation was committed and verified
/// according to the adapter's documented guarantees.
pub trait ParallelizationAdapter {
    /// Adapter-specific error type.
    type Error: Error + Send + Sync + 'static;

    /// Applies a plan transactionally to the current canonical IR.
    fn apply_parallelization_transactionally(
        &mut self,
        plan: &ParallelizationPlan,
    ) -> Result<ParallelizationOutcome, Self::Error>;
}

/// Outcome reported by a concrete IR adapter after applying a plan.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParallelizationOutcome {
    /// Whether the adapter committed an IR change.
    pub changed: bool,

    /// Number of canonical operations changed, as defined by the adapter.
    ///
    /// This is accounting rather than a universal measure of optimization
    /// quality. It must not be interpreted as a worker, task, or device count.
    pub changed_operations: u64,
}

impl ParallelizationOutcome {
    /// Creates an outcome describing no committed change.
    #[must_use]
    pub const fn unchanged() -> Self {
        Self {
            changed: false,
            changed_operations: 0,
        }
    }

    /// Creates an outcome describing a committed change.
    ///
    /// A committed change must report at least one changed operation.
    pub fn changed(
        changed_operations: u64,
    ) -> ParallelizationResult<Self> {
        if changed_operations == 0 {
            return Err(ParallelizationError::InconsistentOutcome {
                changed: true,
                changed_operations,
            });
        }

        Ok(Self {
            changed: true,
            changed_operations,
        })
    }

    /// Validates consistency between the change flag and operation count.
    pub fn validate(&self) -> ParallelizationResult<()> {
        if self.changed != (self.changed_operations > 0) {
            return Err(ParallelizationError::InconsistentOutcome {
                changed: self.changed,
                changed_operations: self.changed_operations,
            });
        }

        Ok(())
    }
}

/// Failures in candidate construction, planning, or adapter outcome validation.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParallelizationError {
    /// A candidate identifier is empty or contains control characters.
    InvalidCandidateId {
        /// Rejected candidate identifier.
        id: String,
    },

    /// The adapter reported a contradictory transformation outcome.
    InconsistentOutcome {
        /// Whether the adapter claimed to change the IR.
        changed: bool,

        /// Number of operations the adapter reported changing.
        changed_operations: u64,
    },

    /// Legality infrastructure returned an error.
    Legality {
        /// Underlying legality error.
        source: LegalityError,
    },
}

impl fmt::Display for ParallelizationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCandidateId { id } => {
                write!(f, "invalid parallelization candidate identifier: {id:?}")
            }
            Self::InconsistentOutcome {
                changed,
                changed_operations,
            } => {
                write!(
                    f,
                    "inconsistent parallelization outcome: \
                     changed={changed}, changed_operations={changed_operations}"
                )
            }
            Self::Legality { source } => {
                write!(f, "parallelization legality failure: {source}")
            }
        }
    }
}

impl Error for ParallelizationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Legality { source } => Some(source),
            _ => None,
        }
    }
}

impl From<LegalityError> for ParallelizationError {
    fn from(source: LegalityError) -> Self {
        Self::Legality { source }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pass_id() -> PassId {
        PassId::new("classical.parallelization")
            .expect("test pass identifier must be valid")
    }

    fn candidate(kind: ParallelizationKind) -> ParallelizationCandidate {
        ParallelizationCandidate::new("candidate-1", kind, pass_id())
            .expect("test candidate must be valid")
    }

    fn legal_candidate(
        kind: ParallelizationKind,
    ) -> ParallelizationCandidate {
        let mut candidate = candidate(kind);

        for property in candidate.contract().obligations() {
            candidate.record_evidence(
                property.clone(),
                ProofEvidence::proven(
                    "established by the test's trusted analysis fixture",
                ),
            );
        }

        candidate
    }

    fn target(
        models: impl IntoIterator<Item = ParallelExecutionModel>,
    ) -> ParallelizationTargetProfile {
        let mut target = ParallelizationTargetProfile::new();

        for model in models {
            target.support_model(model);
        }

        target
    }

    #[test]
    fn missing_evidence_blocks_parallelization() {
        let candidate = candidate(ParallelizationKind::LoopIterations);
        let target = target([ParallelExecutionModel::ParallelLoop]);

        let decision = plan_parallelization(
            &candidate,
            &target,
            &ParallelizationPolicy::new(),
        )
        .expect("planning should return a decision");

        assert!(matches!(
            decision,
            ParallelizationDecision::NotLegal(_)
        ));
    }

    #[test]
    fn fully_proven_candidate_can_be_planned() {
        let candidate = legal_candidate(ParallelizationKind::LoopIterations);
        let target = target([ParallelExecutionModel::ParallelLoop]);

        let decision = plan_parallelization(
            &candidate,
            &target,
            &ParallelizationPolicy::new(),
        )
        .expect("planning should return a decision");

        match decision {
            ParallelizationDecision::Planned(plan) => {
                assert_eq!(plan.candidate_id(), "candidate-1");
                assert_eq!(plan.model(), ParallelExecutionModel::ParallelLoop);
            }
            other => panic!("expected a plan, got {other:?}"),
        }
    }

    #[test]
    fn disproven_concurrency_evidence_blocks_parallelization() {
        let mut candidate = candidate(ParallelizationKind::DataParallel);

        for property in candidate.contract().obligations() {
            candidate.record_evidence(
                property.clone(),
                ProofEvidence::proven("test fixture"),
            );
        }

        candidate.record_evidence(
            LegalityProperty::Concurrency,
            ProofEvidence::disproven(
                "the candidate contains a conflicting dependency",
            ),
        );

        let decision = plan_parallelization(
            &candidate,
            &target([ParallelExecutionModel::DataParallel]),
            &ParallelizationPolicy::new(),
        )
        .expect("planning should return a decision");

        assert!(matches!(
            decision,
            ParallelizationDecision::NotLegal(report)
                if report.status() == LegalityStatus::Illegal
        ));
    }

    #[test]
    fn reduction_requires_numeric_semantics() {
        let candidate = candidate(ParallelizationKind::Reduction);

        assert!(
            candidate.contract().obligations().contains(
                &LegalityProperty::IntegerSemantics
            )
        );
        assert!(
            candidate.contract().obligations().contains(
                &LegalityProperty::FloatingPointSemantics
            )
        );
    }

    #[test]
    fn target_without_supported_models_does_not_produce_a_plan() {
        let candidate = legal_candidate(ParallelizationKind::TaskParallel);

        let decision = plan_parallelization(
            &candidate,
            &ParallelizationTargetProfile::new(),
            &ParallelizationPolicy::new(),
        )
        .expect("planning should return a decision");

        assert_eq!(decision, ParallelizationDecision::UnsupportedTarget);
    }

    #[test]
    fn required_parallelization_is_not_silently_downgraded() {
        let candidate = legal_candidate(ParallelizationKind::TaskParallel);
        let mut policy = ParallelizationPolicy::new();
        policy.set_require_parallelization(true);

        let decision = plan_parallelization(
            &candidate,
            &ParallelizationTargetProfile::new(),
            &policy,
        )
        .expect("planning should return a decision");

        assert_eq!(decision, ParallelizationDecision::PolicyNotSatisfied);
    }

    #[test]
    fn explicit_preference_order_is_respected() {
        let candidate = legal_candidate(ParallelizationKind::DataParallel);
        let target = target([
            ParallelExecutionModel::ParallelLoop,
            ParallelExecutionModel::DataParallel,
        ]);

        let mut policy = ParallelizationPolicy::new();
        policy.set_model_preference([
            ParallelExecutionModel::ParallelLoop,
            ParallelExecutionModel::DataParallel,
        ]);

        let decision = plan_parallelization(&candidate, &target, &policy)
            .expect("planning should return a decision");

        match decision {
            ParallelizationDecision::Planned(plan) => {
                assert_eq!(plan.model(), ParallelExecutionModel::ParallelLoop);
            }
            other => panic!("expected a plan, got {other:?}"),
        }
    }

    #[test]
    fn supported_target_preference_is_used_when_policy_allows_it() {
        let candidate = legal_candidate(ParallelizationKind::DataParallel);
        let mut target = target([
            ParallelExecutionModel::ParallelLoop,
            ParallelExecutionModel::DataParallel,
        ]);
        target.set_preferred_model(Some(ParallelExecutionModel::ParallelLoop));

        let decision = plan_parallelization(
            &candidate,
            &target,
            &ParallelizationPolicy::new(),
        )
        .expect("planning should return a decision");

        match decision {
            ParallelizationDecision::Planned(plan) => {
                assert_eq!(plan.model(), ParallelExecutionModel::ParallelLoop);
            }
            other => panic!("expected a plan, got {other:?}"),
        }
    }

    #[test]
    fn invalid_candidate_identifier_is_rejected() {
        assert!(matches!(
            ParallelizationCandidate::new(
                " \t ",
                ParallelizationKind::LoopIterations,
                pass_id(),
            ),
            Err(ParallelizationError::InvalidCandidateId { .. })
        ));
    }

    #[test]
    fn empty_preference_does_not_invent_a_model() {
        let candidate = legal_candidate(ParallelizationKind::DataParallel);
        let target = target([ParallelExecutionModel::DataParallel]);
        let mut policy = ParallelizationPolicy::new();
        policy.set_model_preference([]);

        let decision = plan_parallelization(&candidate, &target, &policy)
            .expect("planning should return a decision");

        assert_eq!(decision, ParallelizationDecision::UnsupportedTarget);
    }

    #[test]
    fn outcome_validation_rejects_contradictory_accounting() {
        let invalid = ParallelizationOutcome {
            changed: true,
            changed_operations: 0,
        };

        assert!(matches!(
            invalid.validate(),
            Err(ParallelizationError::InconsistentOutcome { .. })
        ));
    }

    #[test]
    fn unchanged_outcome_is_consistent() {
        assert!(ParallelizationOutcome::unchanged().validate().is_ok());
    }

    #[test]
    fn committed_change_requires_a_nonzero_count() {
        assert!(ParallelizationOutcome::changed(1)
            .expect("one changed operation is valid")
            .validate()
            .is_ok());

        assert!(ParallelizationOutcome::changed(0).is_err());
    }

    #[test]
    fn large_model_sets_have_no_fixed_semantic_ceiling() {
        let mut target = ParallelizationTargetProfile::new();

        // Exercise the profile as an extensible capability set. This is a
        // structural test, not a claim about physical hardware capacity.
        target.support_model(ParallelExecutionModel::Extension);
        target.support_model(ParallelExecutionModel::Distributed);
        target.support_model(ParallelExecutionModel::Accelerator);

        assert_eq!(target.supported_models().len(), 3);
    }
}
