
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! Zamani Classical Computing — Loop Transformation Planning.
//!
//! Path: `src/classical/optimization/loop_transform.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Define target-independent requests, legality obligations, deterministic
//! plans, and an adapter contract for classical loop transformations.
//!
//! # Authority
//!
//! - `grammar/statements/loops.g4`: source-level loop syntax.
//! - `grammar/specification/semantic-model.md`: language semantics.
//! - `grammar/specification/scalability-model.md`: scalability requirements.
//! - `grammar/spec/resources.md`: resource semantics.
//! - `grammar/validation/semantic-boundaries.md`: subsystem ownership.
//! - `src/classical/optimization/passes.rs`: optimization orchestration.
//! - `src/classical/optimization/legality.rs`: legality assessment.
//! - `src/quantum/ir/classical/`: existing canonical classical IR boundary.
//! - `src/zuir/`: universal IR, where selected by the compiler pipeline.
//!
//! # Ownership
//!
//! This module owns loop-transformation requests, their explicit parameters,
//! conservative legality obligations, and target-independent plan records.
//!
//! It does not own source parsing, semantic inference, dependence analysis,
//! alias analysis, canonical IR, target discovery, scheduling, physical
//! resources, or machine-code generation.
//!
//! # Correctness boundary
//!
//! A plan is not a proof. The planner accepts evidence supplied by trusted
//! analyses and rejects missing or inconclusive evidence. A concrete IR
//! adapter must check that the evidence applies to the exact IR revision and
//! transformation, apply changes transactionally, and verify the result.
//!
//! # Scalability
//!
//! This module imposes no universal maximum loop depth, iteration count,
//! unroll factor, tile size, or hardware capacity. Parameters are represented
//! dynamically and tile/unroll factors are validated as nonzero. Actual
//! computation remains bounded by available memory, execution time,
//! representable values, and explicitly configured operational budgets.
//!
//! # Portability
//!
//! A valid loop transformation is not tied to a particular processor. Target
//! lowering and profitability decisions belong to downstream components.
//! A source program's meaning must not change merely because a target lacks
//! a preferred optimization.

use core::fmt;
use std::collections::BTreeSet;
use std::error::Error;
use std::num::NonZeroUsize;

use super::legality::{
    assess_legality, LegalityError, LegalityEvidence, LegalityProperty,
    LegalityReport, LegalityStatus, ProofEvidence, TransformationContract,
};
use super::passes::PassId;

/// Result type for loop-transformation operations.
pub type LoopTransformResult<T> = Result<T, LoopTransformError>;

/// Strategy for handling iterations not covered by a complete unroll group.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum UnrollRemainder {
    /// Preserve remaining iterations in a scalar remainder loop.
    ScalarRemainder,

    /// Permit unrolling only when divisibility by the factor is proven.
    ExactMultipleOnly,
}

/// Requested loop transformation and its explicit parameters.
///
/// These variants describe logical transformations, not machine
/// instructions. The canonical-IR adapter remains responsible for determining
/// whether the requested operation is representable for its IR and input.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum LoopTransformRequest {
    /// Unroll a loop by a nonzero factor.
    Unroll {
        /// Number of logical iterations grouped by the transformation.
        factor: NonZeroUsize,
        /// Handling of any iterations outside complete groups.
        remainder: UnrollRemainder,
    },

    /// Reorder the dimensions of a loop nest.
    ///
    /// Each entry is the original zero-based dimension selected for the
    /// corresponding output position. The adapter must validate the
    /// permutation against the actual loop-nest depth.
    Interchange {
        /// Requested dimension order.
        permutation: Vec<usize>,
    },

    /// Fuse adjacent or otherwise compatible loops.
    ///
    /// Loop compatibility, iteration-space equality, effects, and memory
    /// dependencies must be established by the adapter's analyses.
    Fusion,

    /// Split one loop into multiple loops.
    ///
    /// The adapter must preserve operation ordering and all observable
    /// effects required by the language semantics.
    Fission,

    /// Tile a loop nest using explicit nonzero tile dimensions.
    ///
    /// Tile sizes are optimization parameters, not semantic tensor or
    /// collection limits.
    Tile {
        /// Tile size for each requested dimension.
        tile_sizes: Vec<NonZeroUsize>,
    },

    /// Move a loop-invariant condition outside a loop when proven safe.
    ///
    /// The adapter must account for condition effects, exceptions, and
    /// whether evaluating the condition earlier changes observable behavior.
    Unswitch,
}

impl LoopTransformRequest {
    /// Returns the kind of transformation requested.
    #[must_use]
    pub const fn kind(&self) -> LoopTransformKind {
        match self {
            Self::Unroll { .. } => LoopTransformKind::Unroll,
            Self::Interchange { .. } => LoopTransformKind::Interchange,
            Self::Fusion => LoopTransformKind::Fusion,
            Self::Fission => LoopTransformKind::Fission,
            Self::Tile { .. } => LoopTransformKind::Tile,
            Self::Unswitch => LoopTransformKind::Unswitch,
        }
    }

    fn validate(&self) -> LoopTransformResult<()> {
        match self {
            Self::Unroll { factor, .. } => {
                if factor.get() < 2 {
                    return Err(LoopTransformError::InvalidParameter {
                        detail: "unroll factor must be at least two".to_owned(),
                    });
                }
            }
            Self::Interchange { permutation } => {
                if permutation.is_empty() {
                    return Err(LoopTransformError::InvalidParameter {
                        detail: "interchange permutation must not be empty"
                            .to_owned(),
                    });
                }

                let unique: BTreeSet<_> = permutation.iter().copied().collect();
                if unique.len() != permutation.len() {
                    return Err(LoopTransformError::InvalidParameter {
                        detail: "interchange permutation contains duplicate dimensions"
                            .to_owned(),
                    });
                }
            }
            Self::Tile { tile_sizes } => {
                if tile_sizes.is_empty() {
                    return Err(LoopTransformError::InvalidParameter {
                        detail: "tiling requires at least one tile dimension"
                            .to_owned(),
                    });
                }
            }
            Self::Fusion | Self::Fission | Self::Unswitch => {}
        }

        Ok(())
    }
}

/// Kind of loop transformation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum LoopTransformKind {
    /// Group multiple logical iterations into one loop body.
    Unroll,
    /// Reorder a loop nest.
    Interchange,
    /// Combine compatible loops.
    Fusion,
    /// Split a loop into multiple loops.
    Fission,
    /// Partition a loop nest into tiles.
    Tile,
    /// Hoist a loop-invariant conditional into control flow outside the loop.
    Unswitch,
}

/// Knowledge established by an authoritative trip-count analysis.
///
/// An unknown count never establishes divisibility. A known zero count is
/// divisible by every nonzero factor, but the adapter must still preserve
/// loop setup, condition evaluation, and other observable behavior.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum LoopTripCount {
    /// No usable trip-count or divisibility fact is available.
    Unknown,

    /// The trip count is proven divisible by the listed factors.
    ExactMultiples(BTreeSet<NonZeroUsize>),

    /// The number of executed iterations is proven to be zero.
    Zero,
}

impl LoopTripCount {
    /// Creates an unknown trip-count description.
    #[must_use]
    pub fn unknown() -> Self {
        Self::Unknown
    }

    /// Creates a description for a proven zero trip count.
    #[must_use]
    pub fn zero() -> Self {
        Self::Zero
    }

    /// Creates a description from proven exact-multiple factors.
    #[must_use]
    pub fn exact_multiples(
        factors: impl IntoIterator<Item = NonZeroUsize>,
    ) -> Self {
        Self::ExactMultiples(factors.into_iter().collect())
    }

    /// Returns whether divisibility by `factor` has been proven.
    #[must_use]
    pub fn is_exact_multiple_of(&self, factor: NonZeroUsize) -> bool {
        match self {
            Self::Unknown => false,
            Self::ExactMultiples(factors) => factors.contains(&factor),
            Self::Zero => true,
        }
    }
}

/// Candidate loop transformation and its analysis evidence.
///
/// Candidate identifiers are adapter-defined and must be stable for the
/// relevant input revision. Evidence must refer to the same candidate,
/// transformation request, IR revision, and configuration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoopTransformCandidate {
    id: String,
    request: LoopTransformRequest,
    contract: TransformationContract,
    evidence: LegalityEvidence,
    trip_count: LoopTripCount,
}

impl LoopTransformCandidate {
    /// Creates a candidate and installs its conservative proof obligations.
    ///
    /// All transformations require semantic preservation, type correctness,
    /// effects, memory safety, aliasing, control flow, and exception/trap
    /// preservation. Dependence-sensitive transformations also require the
    /// extension property `loop-dependence-preservation`.
    pub fn new(
        id: impl Into<String>,
        request: LoopTransformRequest,
        pass_id: PassId,
    ) -> LoopTransformResult<Self> {
        let id = id.into();

        if id.trim().is_empty() || id.chars().any(char::is_control) {
            return Err(LoopTransformError::InvalidCandidateId { id });
        }

        request.validate()?;

        let mut contract = TransformationContract::new(pass_id);
        contract.require_all([
            LegalityProperty::TypeCorrectness,
            LegalityProperty::Effects,
            LegalityProperty::MemorySafety,
            LegalityProperty::Aliasing,
            LegalityProperty::ControlFlow,
            LegalityProperty::ExceptionsAndTraps,
        ]);

        match request.kind() {
            LoopTransformKind::Unroll => {
                // Unrolling must preserve integer behavior and any required
                // floating-point evaluation guarantees.
                contract.require_all([
                    LegalityProperty::IntegerSemantics,
                    LegalityProperty::FloatingPointSemantics,
                    LegalityProperty::Determinism,
                ]);
            }
            LoopTransformKind::Interchange
            | LoopTransformKind::Fusion
            | LoopTransformKind::Fission
            | LoopTransformKind::Tile => {
                contract.require(
                    LegalityProperty::custom("loop-dependence-preservation")?,
                );
            }
            LoopTransformKind::Unswitch => {
                contract.require(LegalityProperty::Determinism);
            }
        }

        Ok(Self {
            id,
            request,
            contract,
            evidence: LegalityEvidence::new(),
            trip_count: LoopTripCount::Unknown,
        })
    }

    /// Returns the candidate's adapter-defined identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the requested transformation.
    #[must_use]
    pub fn request(&self) -> &LoopTransformRequest {
        &self.request
    }

    /// Returns the transformation kind.
    #[must_use]
    pub const fn kind(&self) -> LoopTransformKind {
        self.request.kind()
    }

    /// Returns the legality contract.
    #[must_use]
    pub fn contract(&self) -> &TransformationContract {
        &self.contract
    }

    /// Returns the supplied analysis evidence.
    #[must_use]
    pub fn evidence(&self) -> &LegalityEvidence {
        &self.evidence
    }

    /// Returns the current trip-count knowledge.
    #[must_use]
    pub fn trip_count(&self) -> &LoopTripCount {
        &self.trip_count
    }

    /// Adds a required legality property.
    ///
    /// This supports additional obligations introduced by a specific IR,
    /// semantic profile, or language extension.
    pub fn require(&mut self, property: LegalityProperty) {
        self.contract.require(property);
    }

    /// Records evidence from a trusted analysis or sound transformation rule.
    ///
    /// This method records evidence; it does not prove the property. The
    /// caller is responsible for evidence provenance and revision matching.
    pub fn record_evidence(
        &mut self,
        property: LegalityProperty,
        evidence: ProofEvidence,
    ) {
        self.evidence.record(property, evidence);
    }

    /// Supplies trip-count knowledge from an authoritative analysis.
    pub fn set_trip_count(&mut self, trip_count: LoopTripCount) {
        self.trip_count = trip_count;
    }

    /// Assesses the declared legality obligations.
    #[must_use]
    pub fn assess_legality(&self) -> LegalityReport {
        assess_legality(&self.contract, &self.evidence)
    }
}

/// A validated loop-transformation plan.
///
/// The plan does not contain IR nodes or target-specific machine operations.
/// The adapter must validate it against the current IR revision before use.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoopTransformPlan {
    candidate_id: String,
    request: LoopTransformRequest,
    pass_id: PassId,
}

impl LoopTransformPlan {
    /// Returns the candidate identifier.
    #[must_use]
    pub fn candidate_id(&self) -> &str {
        &self.candidate_id
    }

    /// Returns the accepted request and parameters.
    #[must_use]
    pub fn request(&self) -> &LoopTransformRequest {
        &self.request
    }

    /// Returns the transformation kind.
    #[must_use]
    pub const fn kind(&self) -> LoopTransformKind {
        self.request.kind()
    }

    /// Returns the associated optimization pass identifier.
    #[must_use]
    pub fn pass_id(&self) -> &PassId {
        &self.pass_id
    }
}

/// Outcome of assessing a loop-transformation candidate.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum LoopTransformDecision {
    /// All obligations were proven and a plan was produced.
    Planned(LoopTransformPlan),

    /// At least one required obligation was disproven or remains unproven.
    NotLegal(LegalityReport),
}

/// Creates a plan only when all required legality properties are proven.
///
/// This function does not run dependence analysis or mutate IR. It consumes
/// evidence from trusted producers and blocks a request when required
/// evidence is missing, unknown, or disproven.
///
/// Exact-multiple unrolling additionally requires proven divisibility when
/// `UnrollRemainder::ExactMultipleOnly` is selected.
pub fn plan_loop_transform(
    candidate: &LoopTransformCandidate,
) -> LoopTransformResult<LoopTransformDecision> {
    let legality = candidate.assess_legality();

    if legality.status() != LegalityStatus::Legal {
        return Ok(LoopTransformDecision::NotLegal(legality));
    }

    if let LoopTransformRequest::Unroll {
        factor,
        remainder: UnrollRemainder::ExactMultipleOnly,
    } = &candidate.request
    {
        if !candidate.trip_count.is_exact_multiple_of(*factor) {
            return Err(LoopTransformError::UnprovenTripCount {
                factor: factor.get(),
            });
        }
    }

    Ok(LoopTransformDecision::Planned(LoopTransformPlan {
        candidate_id: candidate.id.clone(),
        request: candidate.request.clone(),
        pass_id: candidate.contract.pass_id().clone(),
    }))
}

/// Adapter contract for applying plans to an existing canonical IR.
///
/// Implementations must:
///
/// - validate the plan against the current IR and input revision;
/// - obtain sound analyses for loop structure and dependencies;
/// - stage the transformation before committing it;
/// - preserve the source language's observable semantics;
/// - verify the resulting canonical IR before commit;
/// - leave the original IR unchanged if application fails.
///
/// This trait deliberately does not define a second IR or expose assumptions
/// about the selected IR's internal representation.
pub trait LoopTransformAdapter {
    /// Adapter-specific failure type.
    type Error: Error + Send + Sync + 'static;

    /// Applies the plan transactionally and verifies the resulting IR.
    ///
    /// Returning `Ok` means the adapter committed the change and its required
    /// post-transformation verification succeeded.
    fn apply_transactionally(
        &mut self,
        plan: &LoopTransformPlan,
    ) -> Result<LoopTransformOutcome, Self::Error>;
}

/// Result reported by a concrete canonical-IR adapter.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LoopTransformOutcome {
    /// Number of transformation units committed.
    ///
    /// The adapter must document the unit it counts. This value is accounting
    /// information, not a cross-pass measure of optimization quality.
    pub changes: u64,

    /// Analysis names invalidated by the committed change.
    ///
    /// These names are interpreted by the repository's analysis manager, if
    /// one exists; this module does not define a duplicate analysis registry.
    pub invalidated_analyses: BTreeSet<String>,
}

impl LoopTransformOutcome {
    /// Creates an outcome with no reported changes.
    #[must_use]
    pub fn unchanged() -> Self {
        Self::default()
    }

    /// Creates an outcome reporting committed changes.
    #[must_use]
    pub fn changed(changes: u64) -> LoopTransformResult<Self> {
        if changes == 0 {
            return Err(LoopTransformError::InvalidOutcome {
                changes,
            });
        }

        Ok(Self {
            changes,
            invalidated_analyses: BTreeSet::new(),
        })
    }

    /// Returns whether any transformation units were committed.
    #[must_use]
    pub fn has_changes(&self) -> bool {
        self.changes > 0
    }

    /// Validates that the outcome reports a consistent change count.
    pub fn validate(&self) -> LoopTransformResult<()> {
        // Zero changes is valid and means that no transformation was applied.
        // Positive counts represent committed changes.
        Ok(())
    }
}

/// Errors in candidate creation, request validation, or planning.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum LoopTransformError {
    /// Candidate identifier is empty or contains control characters.
    InvalidCandidateId {
        /// Rejected identifier.
        id: String,
    },

    /// A request contains invalid or contradictory parameters.
    InvalidParameter {
        /// Explanation of the invalid parameter.
        detail: String,
    },

    /// Exact-multiple unrolling was requested without proof of divisibility.
    UnprovenTripCount {
        /// Requested unroll factor.
        factor: usize,
    },

    /// An adapter reported a change count of zero for a committed change.
    InvalidOutcome {
        /// Reported number of transformation units.
        changes: u64,
    },

    /// A legality-contract property could not be constructed.
    Legality {
        /// Underlying legality error.
        source: LegalityError,
    },
}

impl fmt::Display for LoopTransformError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCandidateId { id } => {
                write!(f, "invalid loop-transform candidate identifier: {id:?}")
            }
            Self::InvalidParameter { detail } => {
                write!(f, "invalid loop-transform parameter: {detail}")
            }
            Self::UnprovenTripCount { factor } => {
                write!(
                    f,
                    "cannot use exact-multiple unrolling: divisibility by \
                     factor {factor} has not been proven"
                )
            }
            Self::InvalidOutcome { changes } => {
                write!(
                    f,
                    "invalid loop-transform outcome: committed changes \
                     must be nonzero, got {changes}"
                )
            }
            Self::Legality { source } => {
                write!(f, "loop-transform legality error: {source}")
            }
        }
    }
}

impl Error for LoopTransformError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Legality { source } => Some(source),
            _ => None,
        }
    }
}

impl From<LegalityError> for LoopTransformError {
    fn from(source: LegalityError) -> Self {
        Self::Legality { source }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pass_id() -> PassId {
        PassId::new("classical.loop-transform")
            .expect("test pass identifier must be valid")
    }

    fn candidate(request: LoopTransformRequest) -> LoopTransformCandidate {
        LoopTransformCandidate::new("loop-candidate", request, pass_id())
            .expect("test candidate must be valid")
    }

    fn prove_all(candidate: &mut LoopTransformCandidate) {
        let obligations: Vec<_> =
            candidate.contract().obligations().iter().cloned().collect();

        for property in obligations {
            candidate.record_evidence(
                property,
                ProofEvidence::proven("established by test analysis"),
            );
        }
    }

    #[test]
    fn missing_evidence_blocks_a_loop_transform() {
        let candidate = candidate(LoopTransformRequest::Fusion);
        let decision =
            plan_loop_transform(&candidate).expect("assessment should succeed");

        assert!(matches!(
            decision,
            LoopTransformDecision::NotLegal(_)
        ));
    }

    #[test]
    fn fully_proven_unroll_produces_a_plan() {
        let mut candidate = candidate(LoopTransformRequest::Unroll {
            factor: NonZeroUsize::new(4).expect("nonzero factor"),
            remainder: UnrollRemainder::ScalarRemainder,
        });
        prove_all(&mut candidate);

        let decision =
            plan_loop_transform(&candidate).expect("planning should succeed");

        assert!(matches!(
            decision,
            LoopTransformDecision::Planned(_)
        ));
    }

    #[test]
    fn exact_multiple_unroll_requires_divisibility_evidence() {
        let factor = NonZeroUsize::new(4).expect("nonzero factor");
        let mut candidate = candidate(LoopTransformRequest::Unroll {
            factor,
            remainder: UnrollRemainder::ExactMultipleOnly,
        });
        prove_all(&mut candidate);

        let result = plan_loop_transform(&candidate);
        assert!(matches!(
            result,
            Err(LoopTransformError::UnprovenTripCount { factor: 4 })
        ));
    }

    #[test]
    fn exact_multiple_unroll_accepts_proven_divisibility() {
        let factor = NonZeroUsize::new(4).expect("nonzero factor");
        let mut candidate = candidate(LoopTransformRequest::Unroll {
            factor,
            remainder: UnrollRemainder::ExactMultipleOnly,
        });
        prove_all(&mut candidate);
        candidate.set_trip_count(LoopTripCount::exact_multiples([factor]));

        let decision =
            plan_loop_transform(&candidate).expect("planning should succeed");

        assert!(matches!(
            decision,
            LoopTransformDecision::Planned(_)
        ));
    }

    #[test]
    fn interchange_rejects_duplicate_dimensions() {
        let result = LoopTransformCandidate::new(
            "bad-interchange",
            LoopTransformRequest::Interchange {
                permutation: vec![0, 1, 0],
            },
            pass_id(),
        );

        assert!(matches!(
            result,
            Err(LoopTransformError::InvalidParameter { .. })
        ));
    }

    #[test]
    fn tiling_rejects_empty_tile_sizes() {
        let result = LoopTransformCandidate::new(
            "bad-tile",
            LoopTransformRequest::Tile {
                tile_sizes: Vec::new(),
            },
            pass_id(),
        );

        assert!(matches!(
            result,
            Err(LoopTransformError::InvalidParameter { .. })
        ));
    }

    #[test]
    fn unroll_factor_must_be_at_least_two() {
        let result = LoopTransformCandidate::new(
            "no-op-unroll",
            LoopTransformRequest::Unroll {
                factor: NonZeroUsize::new(1).expect("nonzero factor"),
                remainder: UnrollRemainder::ScalarRemainder,
            },
            pass_id(),
        );

        assert!(matches!(
            result,
            Err(LoopTransformError::InvalidParameter { .. })
        ));
    }

    #[test]
    fn dependence_sensitive_transforms_require_dependence_evidence() {
        let candidate = candidate(LoopTransformRequest::Fusion);
        let property = LegalityProperty::custom("loop-dependence-preservation")
            .expect("valid extension property");

        assert!(candidate.contract().obligations().contains(&property));
    }

    #[test]
    fn very_large_tile_sizes_are_not_rejected_by_a_universal_ceiling() {
        let candidate = LoopTransformRequest::Tile {
            tile_sizes: vec![
                NonZeroUsize::new(usize::MAX).expect("usize::MAX is nonzero"),
            ],
        };

        assert!(LoopTransformCandidate::new(
            "large-tile",
            candidate,
            pass_id(),
        )
        .is_ok());
    }

    #[test]
    fn zero_trip_count_is_divisible_by_nonzero_factor() {
        let factor = NonZeroUsize::new(7).expect("nonzero factor");
        assert!(LoopTripCount::zero().is_exact_multiple_of(factor));
    }

    #[test]
    fn unchanged_outcome_is_valid() {
        let outcome = LoopTransformOutcome::unchanged();
        assert!(!outcome.has_changes());
        assert!(outcome.validate().is_ok());
    }

    #[test]
    fn committed_change_count_must_be_nonzero() {
        assert!(LoopTransformOutcome::changed(0).is_err());
        assert!(LoopTransformOutcome::changed(1)
            .expect("positive change count")
            .has_changes());
    }
}
