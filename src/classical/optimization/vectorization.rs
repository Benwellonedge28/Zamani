
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! Zamani Classical Computing — Vectorization Planning.
//!
//! Path: `src/classical/optimization/vectorization.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Define a target-independent contract for identifying, assessing, and
//! planning legal vectorization opportunities in classical computations.
//!
//! This module separates:
//!
//! - vectorization opportunities from physical vector instructions;
//! - transformation legality from target feasibility;
//! - target capability descriptions from source-language semantics;
//! - planning from canonical-IR mutation;
//! - semantic vector length from physical SIMD lane count;
//! - correctness requirements from optional profitability estimates.
//!
//! # Authority
//!
//! This module follows the repository's applicable contracts:
//!
//! - `grammar/Zamani.g4` — root grammar composition;
//! - `grammar/classical/vector.g4` — source-level vector syntax;
//! - `grammar/specification/semantic-model.md` — language meaning;
//! - `grammar/specification/scalability-model.md` — scalability;
//! - `grammar/spec/resources.md` — resource and capability semantics;
//! - `grammar/validation/semantic-boundaries.md` — ownership boundaries;
//! - `src/classical/optimization/legality.rs` — legality assessment;
//! - `src/classical/optimization/passes.rs` — optimization orchestration;
//! - `src/zuir/`, when selected by the compilation pipeline;
//! - `src/quantum/ir/classical/` — existing canonical classical IR boundary.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - Vectorization candidate and policy descriptions.
//! - Target vectorization capability descriptions.
//! - Deterministic selection of a vectorization plan.
//! - Vectorization-specific legality obligations.
//! - The contract for applying a plan through an IR-specific adapter.
//!
//! This module does not own:
//!
//! - Source syntax, AST nodes, semantic type inference, or source semantics.
//! - A competing classical IR or ZUIR representation.
//! - Alias analysis, dependence analysis, effect analysis, or numerical proofs.
//! - Physical vector registers, instruction selection, or machine code.
//! - Hardware discovery, device allocation, placement, or scheduling.
//! - Runtime execution or an assumption that a target supports vectorization.
//!
//! # Critical integration limitation
//!
//! `passes::OptimizationTarget` currently exposes verification but not
//! instruction traversal or mutation. Consequently, this file does not
//! pretend that an opaque `OptimizationTarget` can be rewritten.
//!
//! A concrete IR adapter must supply trustworthy analysis evidence, invoke
//! this planner, apply an accepted plan transactionally, and verify the
//! resulting canonical IR. The adapter must not claim that a plan was
//! applied when it only assessed an opportunity.
//!
//! # POCO-REAF and scalability
//!
//! No universal SIMD width, vector length, tensor rank, operation count,
//! device count, or hardware capacity is imposed here.
//!
//! A fixed lane count is permitted only as a target-specific capability or
//! an explicit request. Scalable-vector support is represented separately.
//! The logical length of a source vector never implies the number of
//! physical lanes.
//!
//! A plan is not a guarantee of successful execution. Actual feasibility
//! depends on target capabilities, available resources, backend constraints,
//! and the program's declared semantic requirements.

use core::fmt;
use std::collections::BTreeSet;
use std::error::Error;
use std::num::NonZeroUsize;

use super::legality::{
    assess_legality, LegalityError, LegalityEvidence, LegalityProperty,
    LegalityReport, LegalityStatus, ProofEvidence, TransformationContract,
};
use super::passes::PassId;

/// Result type for vectorization planning operations.
pub type VectorizationResult<T> = Result<T, VectorizationError>;

/// Broad kind of vectorization opportunity.
///
/// These categories describe the optimization opportunity, not a particular
/// machine instruction or source-language construct. An IR adapter decides
/// which canonical operations can be represented by each category.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum VectorizationKind {
    /// Independent element-wise operations.
    ElementWise,

    /// A loop whose iterations may be executed in vector form.
    Loop,

    /// A reduction over multiple logical values.
    ///
    /// Reassociation is not automatically legal. Floating-point, overflow,
    /// ordering, and reproducibility contracts must be respected.
    Reduction,

    /// A copy or transfer over independent elements.
    MemoryTransfer,

    /// An operation over a collection, shape, or aggregate.
    Aggregate,

    /// An extension-defined opportunity described by an adapter.
    Extension,
}

/// Vector execution model supported by a target.
///
/// This enum deliberately describes broad execution models. It does not
/// encode a vendor instruction set or a universal list of hardware types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum VectorizationMode {
    /// A fixed number of logical lanes per vector operation.
    FixedWidth,

    /// A scalable vector model whose physical width is determined downstream.
    Scalable,

    /// A vector model capable of predicating or masking individual lanes.
    Predicated,
}

/// Strategy for handling elements not covered by a complete vector group.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum TailStrategy {
    /// Process remaining elements using a scalar epilogue.
    ScalarRemainder,

    /// Process remaining elements using masked or predicated lanes.
    Masked,

    /// Vectorize only when the trip count is known to be an exact multiple
    /// of the selected fixed lane count.
    ExactMultipleOnly,
}

/// Knowledge about the trip count of a candidate loop or sequence.
///
/// `Unknown` does not imply divisibility. A target adapter must not infer
/// divisibility from a missing trip-count analysis.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum TripCountKnowledge {
    /// The trip count or relevant divisibility relationship is not known.
    Unknown,

    /// The trip count is known to be divisible by these fixed lane counts.
    ///
    /// The set contains only widths proven by an authoritative analysis.
    ExactMultiples(BTreeSet<NonZeroUsize>),

    /// The trip count is known to be zero.
    Zero,
}

impl TripCountKnowledge {
    /// Creates an unknown trip-count description.
    #[must_use]
    pub fn unknown() -> Self {
        Self::Unknown
    }

    /// Creates a description for a known zero trip count.
    #[must_use]
    pub fn zero() -> Self {
        Self::Zero
    }

    /// Creates a description from proven exact-multiple lane counts.
    #[must_use]
    pub fn exact_multiples(
        widths: impl IntoIterator<Item = NonZeroUsize>,
    ) -> Self {
        Self::ExactMultiples(widths.into_iter().collect())
    }

    /// Returns whether the trip count is proven divisible by `width`.
    #[must_use]
    pub fn is_exact_multiple_of(&self, width: NonZeroUsize) -> bool {
        match self {
            Self::Unknown => false,
            Self::ExactMultiples(widths) => widths.contains(&width),
            Self::Zero => true,
        }
    }
}

/// Target-specific capabilities relevant to vectorization.
///
/// These capabilities describe a candidate target or lowering environment.
/// They do not alter the source program's meaning and must be obtained from
/// the responsible target/backend subsystem.
///
/// An empty capability set means no support has been established; it does
/// not mean that a target is universally incapable of vectorization.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VectorizationTargetProfile {
    modes: BTreeSet<VectorizationMode>,
    fixed_widths: BTreeSet<NonZeroUsize>,
    tail_strategies: BTreeSet<TailStrategy>,
    preferred_fixed_width: Option<NonZeroUsize>,
}

impl VectorizationTargetProfile {
    /// Creates an empty profile with no asserted vectorization capabilities.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a supported vector execution mode.
    pub fn support_mode(&mut self, mode: VectorizationMode) {
        self.modes.insert(mode);
    }

    /// Adds a supported fixed lane count.
    ///
    /// The lane count must be nonzero. No universal upper bound is imposed.
    pub fn support_fixed_width(&mut self, width: NonZeroUsize) {
        self.fixed_widths.insert(width);
        self.modes.insert(VectorizationMode::FixedWidth);
    }

    /// Adds a supported tail-handling strategy.
    pub fn support_tail_strategy(&mut self, strategy: TailStrategy) {
        self.tail_strategies.insert(strategy);
    }

    /// Sets the preferred fixed lane count.
    ///
    /// The preference is advisory: planning uses it only if that width is
    /// included in the profile's supported fixed widths.
    pub fn set_preferred_fixed_width(&mut self, width: Option<NonZeroUsize>) {
        self.preferred_fixed_width = width;
    }

    /// Returns supported vector execution modes.
    #[must_use]
    pub fn modes(&self) -> &BTreeSet<VectorizationMode> {
        &self.modes
    }

    /// Returns supported fixed lane counts.
    #[must_use]
    pub fn fixed_widths(&self) -> &BTreeSet<NonZeroUsize> {
        &self.fixed_widths
    }

    /// Returns supported tail-handling strategies.
    #[must_use]
    pub fn tail_strategies(&self) -> &BTreeSet<TailStrategy> {
        &self.tail_strategies
    }

    /// Returns the preferred fixed lane count, if configured.
    #[must_use]
    pub fn preferred_fixed_width(&self) -> Option<NonZeroUsize> {
        self.preferred_fixed_width
    }

    /// Returns whether this profile supports the supplied mode.
    #[must_use]
    pub fn supports_mode(&self, mode: VectorizationMode) -> bool {
        self.modes.contains(&mode)
    }
}

/// Policy controlling the planner's deterministic choices.
///
/// The preference order is explicit and configurable. It is not a claim
/// that one vector model is universally faster than another.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VectorizationPolicy {
    mode_preference: Vec<VectorizationMode>,
    tail_preference: Vec<TailStrategy>,
    requested_fixed_width: Option<NonZeroUsize>,
    require_vectorization: bool,
}

impl Default for VectorizationPolicy {
    fn default() -> Self {
        Self {
            mode_preference: vec![
                VectorizationMode::Scalable,
                VectorizationMode::Predicated,
                VectorizationMode::FixedWidth,
            ],
            tail_preference: vec![
                TailStrategy::Masked,
                TailStrategy::ScalarRemainder,
                TailStrategy::ExactMultipleOnly,
            ],
            requested_fixed_width: None,
            require_vectorization: false,
        }
    }
}

impl VectorizationPolicy {
    /// Creates the default policy.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Replaces the mode preference order.
    ///
    /// Duplicate entries are harmless but unnecessary. An empty preference
    /// list means that no mode is selected.
    pub fn set_mode_preference(
        &mut self,
        preference: impl IntoIterator<Item = VectorizationMode>,
    ) {
        self.mode_preference = preference.into_iter().collect();
    }

    /// Replaces the tail-strategy preference order.
    pub fn set_tail_preference(
        &mut self,
        preference: impl IntoIterator<Item = TailStrategy>,
    ) {
        self.tail_preference = preference.into_iter().collect();
    }

    /// Requests a particular fixed lane count.
    ///
    /// A request is not a capability. Planning fails to produce a fixed-width
    /// plan if the target profile does not support the requested width.
    pub fn set_requested_fixed_width(
        &mut self,
        width: Option<NonZeroUsize>,
    ) {
        self.requested_fixed_width = width;
    }

    /// Configures whether failure to produce a vectorization plan is an error.
    ///
    /// This does not override legality or target-capability checks.
    pub fn set_require_vectorization(&mut self, required: bool) {
        self.require_vectorization = required;
    }

    /// Returns the configured mode preference order.
    #[must_use]
    pub fn mode_preference(&self) -> &[VectorizationMode] {
        &self.mode_preference
    }

    /// Returns the configured tail-strategy preference order.
    #[must_use]
    pub fn tail_preference(&self) -> &[TailStrategy] {
        &self.tail_preference
    }

    /// Returns the requested fixed lane count, if any.
    #[must_use]
    pub fn requested_fixed_width(&self) -> Option<NonZeroUsize> {
        self.requested_fixed_width
    }

    /// Returns whether vectorization is required by this policy.
    #[must_use]
    pub fn requires_vectorization(&self) -> bool {
        self.require_vectorization
    }
}

/// A candidate transformation and its declared legality obligations.
///
/// The candidate identifier is an adapter-defined stable identifier for the
/// current IR revision. It is not an IR node or a replacement for canonical
/// operation identity.
///
/// The adapter must ensure that the identifier and evidence refer to the
/// same input revision, transformation, and configuration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VectorizationCandidate {
    id: String,
    kind: VectorizationKind,
    contract: TransformationContract,
    evidence: LegalityEvidence,
    trip_count: TripCountKnowledge,
}

impl VectorizationCandidate {
    /// Creates a candidate with conservative default obligations.
    ///
    /// The contract always requires semantic preservation. It additionally
    /// requires type correctness, effects, memory safety, aliasing, and
    /// control-flow preservation. Reduction candidates also require explicit
    /// numerical and determinism evidence.
    pub fn new(
        id: impl Into<String>,
        kind: VectorizationKind,
        pass_id: PassId,
    ) -> VectorizationResult<Self> {
        let id = id.into();

        if id.trim().is_empty() || id.chars().any(char::is_control) {
            return Err(VectorizationError::InvalidCandidateId { id });
        }

        let mut contract = TransformationContract::new(pass_id);
        contract.require_all([
            LegalityProperty::TypeCorrectness,
            LegalityProperty::Effects,
            LegalityProperty::MemorySafety,
            LegalityProperty::Aliasing,
            LegalityProperty::ControlFlow,
        ]);

        if kind == VectorizationKind::Reduction {
            contract.require_all([
                LegalityProperty::IntegerSemantics,
                LegalityProperty::FloatingPointSemantics,
                LegalityProperty::Determinism,
            ]);
        }

        Ok(Self {
            id,
            kind,
            contract,
            evidence: LegalityEvidence::new(),
            trip_count: TripCountKnowledge::Unknown,
        })
    }

    /// Returns the candidate's stable adapter-defined identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the opportunity category.
    #[must_use]
    pub const fn kind(&self) -> VectorizationKind {
        self.kind
    }

    /// Returns the transformation contract.
    #[must_use]
    pub fn contract(&self) -> &TransformationContract {
        &self.contract
    }

    /// Returns the evidence currently supplied for this candidate.
    #[must_use]
    pub fn evidence(&self) -> &LegalityEvidence {
        &self.evidence
    }

    /// Returns the trip-count knowledge supplied by analysis.
    #[must_use]
    pub fn trip_count(&self) -> &TripCountKnowledge {
        &self.trip_count
    }

    /// Adds a required legality property.
    ///
    /// Use this for additional obligations identified by the canonical
    /// semantic model or the particular IR transformation.
    pub fn require(&mut self, property: LegalityProperty) {
        self.contract.require(property);
    }

    /// Records evidence from an authoritative analysis or trusted rule.
    ///
    /// This method does not prove the property itself. The caller is
    /// responsible for supplying evidence that applies to this candidate,
    /// input IR revision, and configuration.
    pub fn record_evidence(
        &mut self,
        property: LegalityProperty,
        evidence: ProofEvidence,
    ) {
        self.evidence.record(property, evidence);
    }

    /// Supplies the result of trip-count analysis.
    pub fn set_trip_count(&mut self, knowledge: TripCountKnowledge) {
        self.trip_count = knowledge;
    }

    /// Assesses the candidate's declared legality obligations.
    #[must_use]
    pub fn assess_legality(&self) -> LegalityReport {
        assess_legality(&self.contract, &self.evidence)
    }
}

/// A selected plan for a legal vectorization opportunity.
///
/// `fixed_width` is populated only for fixed-width execution. Scalable and
/// predicated execution may still require backend-specific width selection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VectorizationPlan {
    candidate_id: String,
    kind: VectorizationKind,
    mode: VectorizationMode,
    fixed_width: Option<NonZeroUsize>,
    tail_strategy: Option<TailStrategy>,
    pass_id: PassId,
}

impl VectorizationPlan {
    /// Returns the candidate identifier to which this plan applies.
    #[must_use]
    pub fn candidate_id(&self) -> &str {
        &self.candidate_id
    }

    /// Returns the opportunity category.
    #[must_use]
    pub const fn kind(&self) -> VectorizationKind {
        self.kind
    }

    /// Returns the selected vector execution model.
    #[must_use]
    pub const fn mode(&self) -> VectorizationMode {
        self.mode
    }

    /// Returns the fixed lane count when the selected model requires one.
    #[must_use]
    pub fn fixed_width(&self) -> Option<NonZeroUsize> {
        self.fixed_width
    }

    /// Returns the selected tail strategy, if required by this plan.
    #[must_use]
    pub fn tail_strategy(&self) -> Option<TailStrategy> {
        self.tail_strategy
    }

    /// Returns the optimization pass identifier.
    #[must_use]
    pub fn pass_id(&self) -> &PassId {
        &self.pass_id
    }
}

/// Outcome of assessing a vectorization opportunity.
///
/// An unsupported target or an unproven transformation is not the same as
/// an invalid source program. The caller decides whether to keep scalar
/// execution, try another target, use an explicitly valid fallback, or
/// report that the requested realization is infeasible.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum VectorizationDecision {
    /// A legal plan was selected for the supplied target profile.
    Planned(VectorizationPlan),

    /// The transformation is illegal or its legality remains unproven.
    NotLegal(LegalityReport),

    /// The profile does not establish a usable vectorization mode.
    UnsupportedTarget,

    /// The available target modes cannot satisfy the selected policy.
    PolicyNotSatisfied,
}

/// Selects a vectorization plan without modifying IR.
///
/// Legality is assessed before target capability selection. A plan is
/// produced only when all declared obligations are proven and the supplied
/// target profile supports the selected execution model.
///
/// This function does not run alias analysis, dependence analysis, or
/// numerical proofs. It consumes evidence from authoritative producers.
pub fn plan_vectorization(
    candidate: &VectorizationCandidate,
    target: &VectorizationTargetProfile,
    policy: &VectorizationPolicy,
) -> VectorizationResult<VectorizationDecision> {
    let legality = candidate.assess_legality();

    if legality.status() != LegalityStatus::Legal {
        return Ok(VectorizationDecision::NotLegal(legality));
    }

    let mut selected_mode = None;

    for mode in &policy.mode_preference {
        if target.supports_mode(*mode) {
            if *mode == VectorizationMode::FixedWidth
                && target.fixed_widths().is_empty()
            {
                continue;
            }

            selected_mode = Some(*mode);
            break;
        }
    }

    let Some(mode) = selected_mode else {
        if policy.requires_vectorization() {
            return Ok(VectorizationDecision::PolicyNotSatisfied);
        }

        return Ok(VectorizationDecision::UnsupportedTarget);
    };

    let fixed_width = if mode == VectorizationMode::FixedWidth {
        let requested = policy.requested_fixed_width();

        if let Some(width) = requested {
            if !target.fixed_widths().contains(&width) {
                return Ok(VectorizationDecision::PolicyNotSatisfied);
            }

            Some(width)
        } else if let Some(preferred) = target.preferred_fixed_width() {
            if target.fixed_widths().contains(&preferred) {
                Some(preferred)
            } else {
                target.fixed_widths().iter().next().copied()
            }
        } else {
            target.fixed_widths().iter().next().copied()
        }
    } else {
        None
    };

    if mode == VectorizationMode::FixedWidth && fixed_width.is_none() {
        return Ok(VectorizationDecision::UnsupportedTarget);
    }

    let tail_strategy = if mode == VectorizationMode::Scalable {
        None
    } else {
        select_tail_strategy(candidate, target, policy, fixed_width)?
    };

    if mode != VectorizationMode::Scalable && tail_strategy.is_none() {
        if policy.requires_vectorization() {
            return Ok(VectorizationDecision::PolicyNotSatisfied);
        }

        return Ok(VectorizationDecision::UnsupportedTarget);
    }

    Ok(VectorizationDecision::Planned(VectorizationPlan {
        candidate_id: candidate.id.clone(),
        kind: candidate.kind,
        mode,
        fixed_width,
        tail_strategy,
        pass_id: candidate.contract.pass_id().clone(),
    }))
}

fn select_tail_strategy(
    candidate: &VectorizationCandidate,
    target: &VectorizationTargetProfile,
    policy: &VectorizationPolicy,
    fixed_width: Option<NonZeroUsize>,
) -> VectorizationResult<Option<TailStrategy>> {
    for strategy in &policy.tail_preference {
        if !target.tail_strategies().contains(strategy) {
            continue;
        }

        match strategy {
            TailStrategy::ExactMultipleOnly => {
                let Some(width) = fixed_width else {
                    continue;
                };

                if candidate.trip_count.is_exact_multiple_of(width) {
                    return Ok(Some(*strategy));
                }
            }
            TailStrategy::ScalarRemainder | TailStrategy::Masked => {
                return Ok(Some(*strategy));
            }
        }
    }

    Ok(None)
}

/// Contract for a target-independent adapter that applies an accepted plan.
///
/// Implementations must:
///
/// - operate on the selected canonical IR rather than creating a competing IR;
/// - confirm that the plan still refers to the current input revision;
/// - apply the transformation transactionally;
/// - preserve all obligations proven during planning;
/// - verify the transformed IR before committing;
/// - leave the original IR unchanged if application fails.
///
/// This trait intentionally does not prescribe a specific IR representation.
pub trait VectorizationAdapter {
    /// Adapter-specific error type.
    type Error: Error + Send + Sync + 'static;

    /// Applies the plan transactionally to the current canonical IR.
    ///
    /// Implementations must reject stale plans and must not silently change
    /// program meaning. Returning `Ok` means the transformation was committed
    /// and the adapter's required verification succeeded.
    fn apply_vectorization_transactionally(
        &mut self,
        plan: &VectorizationPlan,
    ) -> Result<VectorizationOutcome, Self::Error>;
}

/// Report from a concrete IR adapter after applying a plan.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VectorizationOutcome {
    /// Number of canonical operations changed by the adapter.
    ///
    /// This is accounting, not a universal measure of optimization quality.
    pub changed_operations: u64,

    /// Whether the adapter actually committed a change.
    pub changed: bool,
}

impl VectorizationOutcome {
    /// Creates an outcome describing no committed change.
    #[must_use]
    pub const fn unchanged() -> Self {
        Self {
            changed_operations: 0,
            changed: false,
        }
    }

    /// Creates an outcome describing a committed change.
    ///
    /// A committed change must report at least one changed operation.
    pub fn changed(changed_operations: u64) -> VectorizationResult<Self> {
        if changed_operations == 0 {
            return Err(VectorizationError::InconsistentOutcome {
                changed: true,
                changed_operations,
            });
        }

        Ok(Self {
            changed_operations,
            changed: true,
        })
    }

    /// Validates consistency between the change flag and operation count.
    pub fn validate(&self) -> VectorizationResult<()> {
        if self.changed != (self.changed_operations > 0) {
            return Err(VectorizationError::InconsistentOutcome {
                changed: self.changed,
                changed_operations: self.changed_operations,
            });
        }

        Ok(())
    }
}

/// Failures in vectorization candidate construction or plan validation.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum VectorizationError {
    /// A candidate identifier is empty or contains control characters.
    InvalidCandidateId {
        /// Rejected identifier.
        id: String,
    },

    /// The adapter reported a contradictory transformation outcome.
    InconsistentOutcome {
        /// Whether the adapter claimed to change the IR.
        changed: bool,

        /// Number of operations the adapter reported changing.
        changed_operations: u64,
    },

    /// Legality assessment failed.
    Legality {
        /// Underlying legality error.
        source: LegalityError,
    },
}

impl fmt::Display for VectorizationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCandidateId { id } => {
                write!(f, "invalid vectorization candidate identifier: {id:?}")
            }
            Self::InconsistentOutcome {
                changed,
                changed_operations,
            } => {
                write!(
                    f,
                    "inconsistent vectorization outcome: changed={changed}, \
                     changed_operations={changed_operations}"
                )
            }
            Self::Legality { source } => {
                write!(f, "vectorization legality failure: {source}")
            }
        }
    }
}

impl Error for VectorizationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Legality { source } => Some(source),
            _ => None,
        }
    }
}

impl From<LegalityError> for VectorizationError {
    fn from(source: LegalityError) -> Self {
        Self::Legality { source }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pass_id() -> PassId {
        PassId::new("classical.vectorization")
            .expect("test pass identifier must be valid")
    }

    fn legal_candidate(kind: VectorizationKind) -> VectorizationCandidate {
        let mut candidate =
            VectorizationCandidate::new("candidate-1", kind, pass_id())
                .expect("test candidate must be valid");

        for property in candidate.contract().obligations() {
            candidate.record_evidence(
                property.clone(),
                ProofEvidence::proven("established by test analysis"),
            );
        }

        candidate
    }

    fn fixed_target(width: usize) -> VectorizationTargetProfile {
        let mut target = VectorizationTargetProfile::new();
        let width = NonZeroUsize::new(width).expect("width must be nonzero");

        target.support_fixed_width(width);
        target.support_tail_strategy(TailStrategy::ScalarRemainder);
        target.set_preferred_fixed_width(Some(width));
        target
    }

    #[test]
    fn missing_legality_evidence_blocks_vectorization() {
        let candidate =
            VectorizationCandidate::new("candidate-1", VectorizationKind::Loop, pass_id())
                .expect("valid candidate");

        let decision = plan_vectorization(
            &candidate,
            &fixed_target(4),
            &VectorizationPolicy::new(),
        )
        .expect("assessment itself should succeed");

        assert!(matches!(
            decision,
            VectorizationDecision::NotLegal(_)
        ));
    }

    #[test]
    fn proven_candidate_can_produce_a_fixed_width_plan() {
        let candidate = legal_candidate(VectorizationKind::ElementWise);
        let decision = plan_vectorization(
            &candidate,
            &fixed_target(4),
            &VectorizationPolicy::new(),
        )
        .expect("planning should succeed");

        let VectorizationDecision::Planned(plan) = decision else {
            panic!("expected a plan");
        };

        assert_eq!(plan.mode(), VectorizationMode::FixedWidth);
        assert_eq!(plan.fixed_width().map(NonZeroUsize::get), Some(4));
        assert_eq!(plan.tail_strategy(), Some(TailStrategy::ScalarRemainder));
    }

    #[test]
    fn requested_width_must_be_supported_by_target() {
        let candidate = legal_candidate(VectorizationKind::Loop);
        let mut policy = VectorizationPolicy::new();
        policy.set_requested_fixed_width(NonZeroUsize::new(8));
        policy.set_mode_preference([VectorizationMode::FixedWidth]);

        let decision = plan_vectorization(
            &candidate,
            &fixed_target(4),
            &policy,
        )
        .expect("planning should succeed");

        assert_eq!(decision, VectorizationDecision::PolicyNotSatisfied);
    }

    #[test]
    fn exact_multiple_tail_requires_proven_divisibility() {
        let mut candidate = legal_candidate(VectorizationKind::Loop);
        candidate.set_trip_count(TripCountKnowledge::unknown());

        let mut target = fixed_target(4);
        target.support_tail_strategy(TailStrategy::ExactMultipleOnly);

        let mut policy = VectorizationPolicy::new();
        policy.set_mode_preference([VectorizationMode::FixedWidth]);
        policy.set_tail_preference([TailStrategy::ExactMultipleOnly]);

        let decision = plan_vectorization(&candidate, &target, &policy)
            .expect("planning should succeed");

        assert_eq!(decision, VectorizationDecision::UnsupportedTarget);
    }

    #[test]
    fn exact_multiple_tail_accepts_proven_divisibility() {
        let width = NonZeroUsize::new(4).expect("nonzero width");
        let mut candidate = legal_candidate(VectorizationKind::Loop);
        candidate.set_trip_count(TripCountKnowledge::exact_multiples([width]));

        let mut target = fixed_target(4);
        target.support_tail_strategy(TailStrategy::ExactMultipleOnly);

        let mut policy = VectorizationPolicy::new();
        policy.set_mode_preference([VectorizationMode::FixedWidth]);
        policy.set_tail_preference([TailStrategy::ExactMultipleOnly]);

        let decision = plan_vectorization(&candidate, &target, &policy)
            .expect("planning should succeed");

        assert!(matches!(
            decision,
            VectorizationDecision::Planned(_)
        ));
    }

    #[test]
    fn scalable_mode_does_not_require_a_fixed_width() {
        let candidate = legal_candidate(VectorizationKind::Aggregate);
        let mut target = VectorizationTargetProfile::new();
        target.support_mode(VectorizationMode::Scalable);

        let mut policy = VectorizationPolicy::new();
        policy.set_mode_preference([VectorizationMode::Scalable]);

        let decision = plan_vectorization(&candidate, &target, &policy)
            .expect("planning should succeed");

        let VectorizationDecision::Planned(plan) = decision else {
            panic!("expected scalable plan");
        };

        assert_eq!(plan.mode(), VectorizationMode::Scalable);
        assert_eq!(plan.fixed_width(), None);
        assert_eq!(plan.tail_strategy(), None);
    }

    #[test]
    fn reduction_requires_numerical_and_determinism_evidence() {
        let candidate =
            VectorizationCandidate::new("reduction", VectorizationKind::Reduction, pass_id())
                .expect("valid candidate");

        assert!(candidate
            .contract()
            .obligations()
            .contains(&LegalityProperty::FloatingPointSemantics));
        assert!(candidate
            .contract()
            .obligations()
            .contains(&LegalityProperty::IntegerSemantics));
        assert!(candidate
            .contract()
            .obligations()
            .contains(&LegalityProperty::Determinism));
    }

    #[test]
    fn unsupported_vectorization_does_not_mean_invalid_source() {
        let candidate = legal_candidate(VectorizationKind::Loop);
        let target = VectorizationTargetProfile::new();

        let decision = plan_vectorization(
            &candidate,
            &target,
            &VectorizationPolicy::new(),
        )
        .expect("planning should succeed");

        assert_eq!(decision, VectorizationDecision::UnsupportedTarget);
    }

    #[test]
    fn outcome_rejects_inconsistent_change_counts() {
        assert!(VectorizationOutcome::changed(0).is_err());

        let invalid = VectorizationOutcome {
            changed: false,
            changed_operations: 1,
        };

        assert!(invalid.validate().is_err());
    }

    #[test]
    fn target_profile_has_no_universal_width_ceiling() {
        let width = NonZeroUsize::new(1024).expect("nonzero width");
        let mut target = VectorizationTargetProfile::new();

        target.support_fixed_width(width);

        assert!(target.fixed_widths().contains(&width));
    }
}
