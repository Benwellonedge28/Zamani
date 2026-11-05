
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! Zamani Universal Meta-Compiler — Optimization Decisions.
//!
//! Path:
//!     src/compiler/optimization/decisions.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum version 1.97
//!
//! # Purpose
//!
//! Record and aggregate auditable optimization decisions by combining
//! authoritative planning decisions with optional profitability evidence.
//!
//! # Architectural authorities
//!
//! - `super::planning`: candidate descriptions and planning decisions.
//! - `super::profitability`: profitability assessments.
//! - `super::policies`: optimization policy resolution.
//! - `super::cost_model`: cost-estimation contracts.
//! - `crate::zuir::pass::contracts`: canonical pass identity and version.
//! - `crate::zuir::pass::registry`: canonical pass registration.
//! - `crate::zuir::pass::manager`: canonical pass execution.
//! - `crate::optimizer`: canonical optimization implementation.
//!
//! # Ownership
//!
//! This module owns decision records and their deterministic aggregation.
//!
//! It does not own:
//! - Semantic legality.
//! - Transformation implementation.
//! - Cost estimation.
//! - Profitability calculations.
//! - Policy resolution.
//! - Pass registration.
//! - Pass execution.
//! - IR verification.
//! - Hardware selection.
//! - Resource allocation.
//! - Scheduling or routing.
//!
//! # Decision safety
//!
//! A decision record is evidence about a planning outcome.
//! It is not a semantic-preservation proof.
//! It does not authorize bypassing the canonical pass manager.
//! It does not guarantee that a transformation will execute.
//!
//! Missing evidence is never silently treated as success.
//! Indeterminate profitability is never silently treated as profitable.
//!
//! # Determinism
//!
//! Records are indexed by canonical pass identity in a BTreeMap.
//! Iteration and summaries are deterministic.
//! No wall-clock timestamps or random identifiers are generated.
//!
//! # Scalability
//!
//! No universal limits are imposed on:
//! - Optimization candidates.
//! - Pass identities.
//! - Computing domains.
//! - Decision records.
//! - Optimization objectives.
//! - Resource dimensions.
//!
//! Actual memory, processing, representation and configured operational
//! budgets remain applicable.
//!
//! # Safety
//!
//! Safe Rust only. Unsafe Rust is forbidden.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::zuir::pass::contracts::{
    PassId,
    PassVersion,
};

use super::planning::{
    OptimizationCandidate,
    PlanningDecision,
};

use super::profitability::{
    ProfitabilityAssessment,
    ProfitabilityStatus,
};

// ============================================================================
// API VERSION
// ============================================================================

/// Stable public optimization-decision API version.
pub const OPTIMIZATION_DECISIONS_API_VERSION: u16 = 1;

// ============================================================================
// RESULTS
// ============================================================================

/// Result type for optimization-decision operations.
pub type DecisionResult<T> = Result<T, DecisionError>;

// ============================================================================
// PROFITABILITY EVIDENCE
// ============================================================================

/// Profitability evidence supplied when recording a decision.
///
/// The distinction between `NotRequired` and `NotEvaluated` is intentional.
///
/// `NotRequired` means that the resolved optimization policy does not
/// require a profitability assessment for this candidate.
///
/// `NotEvaluated` means that no assessment has been supplied. It must
/// never be interpreted as evidence that a candidate is profitable.
#[derive(Debug, Clone, PartialEq)]
pub enum ProfitabilityEvidence {
    /// A completed assessment from the canonical profitability evaluator.
    Evaluated(ProfitabilityAssessment),

    /// The applicable resolved policy does not require profitability
    /// evaluation for this decision.
    NotRequired,

    /// Profitability has not been evaluated or evidence is unavailable.
    NotEvaluated,
}

impl ProfitabilityEvidence {
    /// Returns the assessment when one has been supplied.
    #[must_use]
    pub fn assessment(&self) -> Option<&ProfitabilityAssessment> {
        match self {
            Self::Evaluated(assessment) => Some(assessment),
            Self::NotRequired | Self::NotEvaluated => None,
        }
    }

    /// Returns whether profitability was explicitly evaluated.
    #[must_use]
    pub const fn was_evaluated(&self) -> bool {
        matches!(self, Self::Evaluated(_))
    }

    /// Returns whether profitability was explicitly declared unnecessary.
    #[must_use]
    pub const fn was_not_required(&self) -> bool {
        matches!(self, Self::NotRequired)
    }
}

// ============================================================================
// DECISION DISPOSITION
// ============================================================================

/// Final planning disposition for an optimization candidate.
///
/// `EligibleForPassManager` means that the planning and supplied
/// profitability gates have been satisfied.
///
/// It does not execute the pass or bypass the canonical pass manager's
/// own validation, verification, budget and cancellation requirements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DecisionDisposition {
    /// The candidate failed an authoritative planning or profitability
    /// requirement.
    Rejected,

    /// Evidence is missing or inconclusive. Further analysis is required.
    Deferred,

    /// The candidate may be submitted to the canonical pass manager
    /// for its independent execution checks.
    EligibleForPassManager,
}

impl DecisionDisposition {
    /// Returns whether the candidate may proceed to pass-manager checks.
    ///
    /// This is not permission to execute a transformation directly.
    #[must_use]
    pub const fn is_eligible_for_pass_manager(self) -> bool {
        matches!(self, Self::EligibleForPassManager)
    }

    /// Returns whether the candidate was rejected.
    #[must_use]
    pub const fn is_rejected(self) -> bool {
        matches!(self, Self::Rejected)
    }

    /// Returns whether additional evidence is needed.
    #[must_use]
    pub const fn is_deferred(self) -> bool {
        matches!(self, Self::Deferred)
    }
}

// ============================================================================
// DECISION REASON
// ============================================================================

/// Machine-readable explanation for a decision disposition.
///
/// Detailed planning reasons remain available through `PlanningDecision`.
/// Detailed numerical evidence remains available through
/// `ProfitabilityAssessment`.
///
/// This enum provides a stable high-level reason without duplicating
/// either subsystem's detailed error model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DecisionReason {
    /// Planning accepted the candidate and profitability requirements
    /// were satisfied or explicitly not required.
    Eligible,

    /// The canonical planner rejected the candidate.
    PlanningRejected,

    /// Profitability evidence shows that the candidate does not satisfy
    /// the configured profitability policy.
    NotProfitable,

    /// Profitability evidence is inconclusive.
    ProfitabilityIndeterminate,

    /// Profitability was required but has not been evaluated.
    ProfitabilityNotEvaluated,

    /// Profitability was explicitly not required by the resolved policy.
    ProfitabilityNotRequired,
}

impl DecisionReason {
    /// Returns a stable machine-readable reason code.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::Eligible => "optimization.decision.eligible",
            Self::PlanningRejected => "optimization.decision.planning_rejected",
            Self::NotProfitable => "optimization.decision.not_profitable",
            Self::ProfitabilityIndeterminate => {
                "optimization.decision.profitability_indeterminate"
            }
            Self::ProfitabilityNotEvaluated => {
                "optimization.decision.profitability_not_evaluated"
            }
            Self::ProfitabilityNotRequired => {
                "optimization.decision.profitability_not_required"
            }
        }
    }
}

// ============================================================================
// DECISION RECORD
// ============================================================================

/// Auditable decision record for one canonical optimization pass.
///
/// The record retains the original planning and profitability evidence.
/// It does not replace either source of evidence.
///
/// Fields are private so that the disposition cannot be changed without
/// recomputing it from the authoritative evidence.
#[derive(Debug, Clone, PartialEq)]
pub struct OptimizationDecisionRecord {
    pass_id: PassId,
    pass_version: PassVersion,
    planning_decision: PlanningDecision,
    profitability: ProfitabilityEvidence,
    disposition: DecisionDisposition,
    reason: DecisionReason,
}

impl OptimizationDecisionRecord {
    /// Creates a decision record from canonical evidence.
    ///
    /// The profitability assessment, when present, must refer to the
    /// same pass identity as this record.
    ///
    /// The final disposition is derived deterministically:
    ///
    /// - Rejected planning always rejects the candidate.
    /// - Accepted planning plus profitable evidence is eligible.
    /// - Accepted planning plus explicitly unnecessary profitability
    ///   evidence is eligible.
    /// - Accepted planning plus missing profitability evidence is deferred.
    /// - Accepted planning plus indeterminate profitability is deferred.
    /// - Accepted planning plus unprofitable evidence is rejected.
    pub fn new(
        pass_id: PassId,
        pass_version: PassVersion,
        planning_decision: PlanningDecision,
        profitability: ProfitabilityEvidence,
    ) -> DecisionResult<Self> {
        validate_profitability_identity(&pass_id, &profitability)?;

        let (disposition, reason) =
            classify_decision(&planning_decision, &profitability);

        Ok(Self {
            pass_id,
            pass_version,
            planning_decision,
            profitability,
            disposition,
            reason,
        })
    }

    /// Creates a record directly from an existing planning candidate.
    ///
    /// The candidate supplies the canonical pass identity and version.
    /// The caller supplies the result of the canonical planner and,
    /// where required, the canonical profitability evaluator.
    pub fn from_candidate(
        candidate: &OptimizationCandidate,
        planning_decision: PlanningDecision,
        profitability: ProfitabilityEvidence,
    ) -> DecisionResult<Self> {
        candidate
            .validate()
            .map_err(|error| DecisionError::InvalidCandidate {
                pass_id: candidate.pass_id().to_owned(),
                reason: error.to_string(),
            })?;

        Self::new(
            candidate.descriptor.id.clone(),
            candidate.descriptor.version,
            planning_decision,
            profitability,
        )
    }

    /// Returns the canonical pass identity.
    #[must_use]
    pub fn pass_id(&self) -> &PassId {
        &self.pass_id
    }

    /// Returns the canonical pass version.
    #[must_use]
    pub const fn pass_version(&self) -> PassVersion {
        self.pass_version
    }

    /// Returns the original planning decision.
    #[must_use]
    pub fn planning_decision(&self) -> &PlanningDecision {
        &self.planning_decision
    }

    /// Returns the profitability evidence.
    #[must_use]
    pub fn profitability(&self) -> &ProfitabilityEvidence {
        &self.profitability
    }

    /// Returns the derived disposition.
    #[must_use]
    pub const fn disposition(&self) -> DecisionDisposition {
        self.disposition
    }

    /// Returns the high-level decision reason.
    #[must_use]
    pub const fn reason(&self) -> DecisionReason {
        self.reason
    }

    /// Returns the stable machine-readable reason code.
    #[must_use]
    pub const fn reason_code(&self) -> &'static str {
        self.reason.code()
    }

    /// Returns whether the candidate may proceed to pass-manager checks.
    #[must_use]
    pub const fn is_eligible_for_pass_manager(&self) -> bool {
        self.disposition.is_eligible_for_pass_manager()
    }
}

// ============================================================================
// DECISION CLASSIFICATION
// ============================================================================

/// Derives a disposition from authoritative planning and profitability
/// evidence.
///
/// This function is deliberately conservative. Missing evidence cannot
/// create eligibility.
fn classify_decision(
    planning: &PlanningDecision,
    profitability: &ProfitabilityEvidence,
) -> (DecisionDisposition, DecisionReason) {
    if !matches!(planning, PlanningDecision::Accepted) {
        return (
            DecisionDisposition::Rejected,
            DecisionReason::PlanningRejected,
        );
    }

    match profitability {
        ProfitabilityEvidence::NotRequired => (
            DecisionDisposition::EligibleForPassManager,
            DecisionReason::ProfitabilityNotRequired,
        ),

        ProfitabilityEvidence::NotEvaluated => (
            DecisionDisposition::Deferred,
            DecisionReason::ProfitabilityNotEvaluated,
        ),

        ProfitabilityEvidence::Evaluated(assessment) => {
            match assessment.status {
                ProfitabilityStatus::Profitable => (
                    DecisionDisposition::EligibleForPassManager,
                    DecisionReason::Eligible,
                ),

                ProfitabilityStatus::NotProfitable => (
                    DecisionDisposition::Rejected,
                    DecisionReason::NotProfitable,
                ),

                ProfitabilityStatus::Indeterminate => (
                    DecisionDisposition::Deferred,
                    DecisionReason::ProfitabilityIndeterminate,
                ),
            }
        }
    }
}

/// Ensures profitability evidence belongs to the same pass as the record.
fn validate_profitability_identity(
    pass_id: &PassId,
    profitability: &ProfitabilityEvidence,
) -> DecisionResult<()> {
    if let ProfitabilityEvidence::Evaluated(assessment) = profitability {
        if assessment.pass_id != pass_id.as_str() {
            return Err(DecisionError::PassIdentityMismatch {
                record_pass: pass_id.as_str().to_owned(),
                assessment_pass: assessment.pass_id.clone(),
            });
        }
    }

    Ok(())
}

// ============================================================================
// DECISION SUMMARY
// ============================================================================

/// Deterministic summary of a decision ledger.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DecisionSummary {
    /// Total number of recorded decisions.
    pub total: usize,

    /// Number of rejected candidates.
    pub rejected: usize,

    /// Number of deferred candidates.
    pub deferred: usize,

    /// Number of candidates eligible for pass-manager checks.
    pub eligible_for_pass_manager: usize,
}

impl DecisionSummary {
    /// Returns whether the ledger contains no decisions.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.total == 0
    }

    /// Returns whether every recorded candidate was rejected.
    #[must_use]
    pub const fn all_rejected(self) -> bool {
        self.total > 0 && self.rejected == self.total
    }

    /// Returns whether every recorded candidate is eligible for
    /// pass-manager checks.
    #[must_use]
    pub const fn all_eligible(self) -> bool {
        self.total > 0 && self.eligible_for_pass_manager == self.total
    }
}

// ============================================================================
// DECISION LEDGER
// ============================================================================

/// Deterministic collection of optimization decision records.
///
/// Pass identity is unique within a ledger. Duplicate records are rejected
/// rather than silently overwriting previous evidence.
///
/// The ledger is not a persistent database and does not claim to provide
/// durable storage. Persistence and provenance serialization belong to
/// their established repository subsystems.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OptimizationDecisionLedger {
    records: BTreeMap<PassId, OptimizationDecisionRecord>,
}

impl OptimizationDecisionLedger {
    /// Creates an empty decision ledger.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Records a decision.
    ///
    /// Duplicate pass identities are rejected.
    pub fn record(
        &mut self,
        record: OptimizationDecisionRecord,
    ) -> DecisionResult<()> {
        let pass_id = record.pass_id.clone();

        if self.records.contains_key(&pass_id) {
            return Err(DecisionError::DuplicateDecision {
                pass_id: pass_id.as_str().to_owned(),
            });
        }

        self.records.insert(pass_id, record);

        Ok(())
    }

    /// Replaces an existing decision explicitly.
    ///
    /// This operation is useful when new authoritative evidence becomes
    /// available and a previous deferred decision must be recomputed.
    ///
    /// It does not silently create a new entry.
    pub fn replace(
        &mut self,
        record: OptimizationDecisionRecord,
    ) -> DecisionResult<OptimizationDecisionRecord> {
        let pass_id = record.pass_id.clone();

        if !self.records.contains_key(&pass_id) {
            return Err(DecisionError::DecisionNotFound {
                pass_id: pass_id.as_str().to_owned(),
            });
        }

        match self.records.insert(pass_id, record) {
            Some(previous) => Ok(previous),
            None => Err(DecisionError::DecisionNotFound {
                pass_id: "record disappeared during replacement".to_owned(),
            }),
        }
    }

    /// Retrieves a decision by canonical pass identity.
    #[must_use]
    pub fn get(
        &self,
        pass_id: &PassId,
    ) -> Option<&OptimizationDecisionRecord> {
        self.records.get(pass_id)
    }

    /// Returns the number of recorded decisions.
    #[must_use]
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Returns whether the ledger is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Iterates over records in canonical pass-ID order.
    ///
    /// This ordering is deterministic and independent of insertion order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&PassId, &OptimizationDecisionRecord)> {
        self.records.iter()
    }

    /// Returns a deterministic summary of all decisions.
    #[must_use]
    pub fn summary(&self) -> DecisionSummary {
        let mut summary = DecisionSummary::default();

        for record in self.records.values() {
            summary.total += 1;

            match record.disposition {
                DecisionDisposition::Rejected => {
                    summary.rejected += 1;
                }

                DecisionDisposition::Deferred => {
                    summary.deferred += 1;
                }

                DecisionDisposition::EligibleForPassManager => {
                    summary.eligible_for_pass_manager += 1;
                }
            }
        }

        summary
    }

    /// Removes a decision by canonical pass identity.
    ///
    /// The removed record is returned so the caller can retain or archive
    /// the evidence through its provenance or persistence subsystem.
    pub fn remove(
        &mut self,
        pass_id: &PassId,
    ) -> Option<OptimizationDecisionRecord> {
        self.records.remove(pass_id)
    }

    /// Clears all decisions from this in-memory ledger.
    ///
    /// This does not delete externally persisted provenance records.
    pub fn clear(&mut self) {
        self.records.clear();
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by optimization-decision recording and aggregation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecisionError {
    /// Candidate metadata failed its canonical planning validation.
    InvalidCandidate {
        /// Stable pass identity.
        pass_id: String,

        /// Underlying validation failure.
        reason: String,
    },

    /// Profitability evidence belongs to another pass.
    PassIdentityMismatch {
        /// Identity expected by the decision record.
        record_pass: String,

        /// Identity found in the profitability assessment.
        assessment_pass: String,
    },

    /// A decision already exists for the pass identity.
    DuplicateDecision {
        /// Duplicated canonical pass identity.
        pass_id: String,
    },

    /// An explicit replacement was requested for a missing decision.
    DecisionNotFound {
        /// Missing canonical pass identity.
        pass_id: String,
    },
}

impl fmt::Display for DecisionError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidCandidate {
                pass_id,
                reason,
            } => write!(
                formatter,
                "optimization candidate {pass_id:?} is invalid: {reason}"
            ),

            Self::PassIdentityMismatch {
                record_pass,
                assessment_pass,
            } => write!(
                formatter,
                "profitability evidence belongs to pass \
                 {assessment_pass:?}, not decision pass {record_pass:?}"
            ),

            Self::DuplicateDecision { pass_id } => write!(
                formatter,
                "an optimization decision already exists for pass {pass_id:?}"
            ),

            Self::DecisionNotFound { pass_id } => write!(
                formatter,
                "no optimization decision exists for pass {pass_id:?}"
            ),
        }
    }
}

impl Error for DecisionError {}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler::optimization::cost_model::{
        MetricId,
        Quantity,
        UnitId,
    };

    fn pass_id(value: &str) -> PassId {
        PassId::new(value).expect("valid canonical pass ID")
    }

    fn version() -> PassVersion {
        PassVersion::new(1, 0, 0)
    }

    fn assessment(
        pass: &str,
        status: ProfitabilityStatus,
    ) -> ProfitabilityAssessment {
        ProfitabilityAssessment {
            pass_id: pass.to_owned(),
            metric: MetricId::new("execution.latency")
                .expect("valid metric"),
            unit: UnitId::new("seconds")
                .expect("valid unit"),
            direction: super::super::profitability::ObjectiveDirection::Minimize,
            baseline_expected: Quantity::new(10.0)
                .expect("valid quantity"),
            candidate_expected: Quantity::new(8.0)
                .expect("valid quantity"),
            expected_improvement:
                super::super::profitability::Improvement::new_for_test(2.0),
            conservative_improvement:
                super::super::profitability::Improvement::new_for_test(1.0),
            minimum_improvement: Quantity::zero(),
            uncertainty_mode:
                super::super::profitability::UncertaintyMode::Conservative,
            status,
        }
    }

    #[test]
    fn accepted_and_profitable_candidate_is_eligible() {
        let record = OptimizationDecisionRecord::new(
            pass_id("classical.optimize"),
            version(),
            PlanningDecision::Accepted,
            ProfitabilityEvidence::Evaluated(assessment(
                "classical.optimize",
                ProfitabilityStatus::Profitable,
            )),
        )
        .expect("valid record");

        assert_eq!(
            record.disposition(),
            DecisionDisposition::EligibleForPassManager
        );

        assert_eq!(record.reason(), DecisionReason::Eligible);
    }

    #[test]
    fn accepted_candidate_with_not_required_profitability_is_eligible() {
        let record = OptimizationDecisionRecord::new(
            pass_id("classical.optimize"),
            version(),
            PlanningDecision::Accepted,
            ProfitabilityEvidence::NotRequired,
        )
        .expect("valid record");

        assert_eq!(
            record.disposition(),
            DecisionDisposition::EligibleForPassManager
        );

        assert_eq!(
            record.reason(),
            DecisionReason::ProfitabilityNotRequired
        );
    }

    #[test]
    fn missing_profitability_evidence_is_deferred() {
        let record = OptimizationDecisionRecord::new(
            pass_id("classical.optimize"),
            version(),
            PlanningDecision::Accepted,
            ProfitabilityEvidence::NotEvaluated,
        )
        .expect("valid record");

        assert_eq!(
            record.disposition(),
            DecisionDisposition::Deferred
        );

        assert_eq!(
            record.reason(),
            DecisionReason::ProfitabilityNotEvaluated
        );
    }

    #[test]
    fn indeterminate_profitability_is_deferred() {
        let record = OptimizationDecisionRecord::new(
            pass_id("classical.optimize"),
            version(),
            PlanningDecision::Accepted,
            ProfitabilityEvidence::Evaluated(assessment(
                "classical.optimize",
                ProfitabilityStatus::Indeterminate,
            )),
        )
        .expect("valid record");

        assert_eq!(
            record.disposition(),
            DecisionDisposition::Deferred
        );
    }

    #[test]
    fn unprofitable_candidate_is_rejected() {
        let record = OptimizationDecisionRecord::new(
            pass_id("classical.optimize"),
            version(),
            PlanningDecision::Accepted,
            ProfitabilityEvidence::Evaluated(assessment(
                "classical.optimize",
                ProfitabilityStatus::NotProfitable,
            )),
        )
        .expect("valid record");

        assert_eq!(
            record.disposition(),
            DecisionDisposition::Rejected
        );
    }

    #[test]
    fn planning_rejection_cannot_be_overridden_by_profitability() {
        let record = OptimizationDecisionRecord::new(
            pass_id("classical.optimize"),
            version(),
            PlanningDecision::UnverifiedInput,
            ProfitabilityEvidence::Evaluated(assessment(
                "classical.optimize",
                ProfitabilityStatus::Profitable,
            )),
        )
        .expect("valid record");

        assert_eq!(
            record.disposition(),
            DecisionDisposition::Rejected
        );

        assert_eq!(
            record.reason(),
            DecisionReason::PlanningRejected
        );
    }

    #[test]
    fn mismatched_profitability_identity_is_rejected() {
        let result = OptimizationDecisionRecord::new(
            pass_id("classical.optimize"),
            version(),
            PlanningDecision::Accepted,
            ProfitabilityEvidence::Evaluated(assessment(
                "quantum.optimize",
                ProfitabilityStatus::Profitable,
            )),
        );

        assert!(matches!(
            result,
            Err(DecisionError::PassIdentityMismatch { .. })
        ));
    }

    #[test]
    fn ledger_rejects_duplicate_pass_decisions() {
        let record = OptimizationDecisionRecord::new(
            pass_id("classical.optimize"),
            version(),
            PlanningDecision::Accepted,
            ProfitabilityEvidence::NotRequired,
        )
        .expect("valid record");

        let duplicate = record.clone();

        let mut ledger = OptimizationDecisionLedger::new();

        ledger.record(record).expect("first record");

        assert!(matches!(
            ledger.record(duplicate),
            Err(DecisionError::DuplicateDecision { .. })
        ));

        assert_eq!(ledger.len(), 1);
    }

    #[test]
    fn ledger_replacement_requires_existing_record() {
        let record = OptimizationDecisionRecord::new(
            pass_id("classical.optimize"),
            version(),
            PlanningDecision::Accepted,
            ProfitabilityEvidence::NotRequired,
        )
        .expect("valid record");

        let mut ledger = OptimizationDecisionLedger::new();

        assert!(matches!(
            ledger.replace(record),
            Err(DecisionError::DecisionNotFound { .. })
        ));
    }

    #[test]
    fn ledger_replacement_returns_previous_evidence() {
        let initial = OptimizationDecisionRecord::new(
            pass_id("classical.optimize"),
            version(),
            PlanningDecision::Accepted,
            ProfitabilityEvidence::NotEvaluated,
        )
        .expect("valid record");

        let updated = OptimizationDecisionRecord::new(
            pass_id("classical.optimize"),
            version(),
            PlanningDecision::Accepted,
            ProfitabilityEvidence::NotRequired,
        )
        .expect("valid record");

        let mut ledger = OptimizationDecisionLedger::new();

        ledger.record(initial).expect("initial record");

        let previous = ledger
            .replace(updated)
            .expect("replacement");

        assert_eq!(
            previous.disposition(),
            DecisionDisposition::Deferred
        );

        assert_eq!(
            ledger
                .get(&pass_id("classical.optimize"))
                .expect("updated record")
                .disposition(),
            DecisionDisposition::EligibleForPassManager
        );
    }

    #[test]
    fn ledger_summary_counts_all_dispositions() {
        let mut ledger = OptimizationDecisionLedger::new();

        let records = [
            (
                "classical.accepted",
                PlanningDecision::Accepted,
                ProfitabilityEvidence::NotRequired,
            ),
            (
                "classical.deferred",
                PlanningDecision::Accepted,
                ProfitabilityEvidence::NotEvaluated,
            ),
            (
                "classical.rejected",
                PlanningDecision::UnverifiedInput,
                ProfitabilityEvidence::NotRequired,
            ),
        ];

        for (id, planning, profitability) in records {
            let record = OptimizationDecisionRecord::new(
                pass_id(id),
                version(),
                planning,
                profitability,
            )
            .expect("valid record");

            ledger.record(record).expect("unique record");
        }

        let summary = ledger.summary();

        assert_eq!(summary.total, 3);
        assert_eq!(summary.eligible_for_pass_manager, 1);
        assert_eq!(summary.deferred, 1);
        assert_eq!(summary.rejected, 1);
    }

    #[test]
    fn ledger_iteration_is_sorted_by_pass_identity() {
        let mut ledger = OptimizationDecisionLedger::new();

        for id in [
            "quantum.optimize",
            "classical.optimize",
            "tensor.optimize",
        ] {
            let record = OptimizationDecisionRecord::new(
                pass_id(id),
                version(),
                PlanningDecision::Accepted,
                ProfitabilityEvidence::NotRequired,
            )
            .expect("valid record");

            ledger.record(record).expect("unique record");
        }

        let ids: Vec<&str> = ledger
            .iter()
            .map(|(id, _)| id.as_str())
            .collect();

        assert_eq!(
            ids,
            vec![
                "classical.optimize",
                "quantum.optimize",
                "tensor.optimize",
            ]
        );
    }

    #[test]
    fn reason_codes_are_stable_and_machine_readable() {
        assert_eq!(
            DecisionReason::Eligible.code(),
            "optimization.decision.eligible"
        );

        assert_eq!(
            DecisionReason::PlanningRejected.code(),
            "optimization.decision.planning_rejected"
        );
    }

    #[test]
    fn empty_ledger_has_empty_summary() {
        let ledger = OptimizationDecisionLedger::new();
        let summary = ledger.summary();

        assert!(summary.is_empty());
        assert_eq!(summary.total, 0);
    }

    #[test]
    fn ledger_removal_returns_the_original_record() {
        let record = OptimizationDecisionRecord::new(
            pass_id("classical.optimize"),
            version(),
            PlanningDecision::Accepted,
            ProfitabilityEvidence::NotRequired,
        )
        .expect("valid record");

        let mut ledger = OptimizationDecisionLedger::new();

        ledger.record(record).expect("record");

        let removed = ledger
            .remove(&pass_id("classical.optimize"))
            .expect("removed record");

        assert_eq!(
            removed.disposition(),
            DecisionDisposition::EligibleForPassManager
        );

        assert!(ledger.is_empty());
    }
}
