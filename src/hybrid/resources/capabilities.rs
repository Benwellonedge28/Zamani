
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Hybrid Resource Capabilities
//!
//! Path: `src/hybrid/resources/capabilities.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provide the hybrid-composition boundary for consuming capability evidence
//! supplied by semantic analysis, target discovery, deployment manifests, or
//! other authorized upstream providers.
//!
//! This module allows a hybrid computation to evaluate its capability
//! requirements against evidence for one explicitly identified scope.
//!
//! ## Canonical ownership
//!
//! The frontend semantic capability subsystem remains authoritative for:
//!
//! - capability identity;
//! - capability versions and version constraints;
//! - typed capability properties;
//! - capability requirements;
//! - capability evidence and evidence aggregation;
//! - semantic evidence evaluation.
//!
//! This module owns only the hybrid-facing, scope-specific view of that
//! evidence. It does not define another capability ontology.
//!
//! ## Does not own
//!
//! This module does not:
//!
//! - discover hardware;
//! - assert that an unobserved capability exists;
//! - authenticate or authorize an evidence provider;
//! - allocate or reserve resources;
//! - select devices or execution targets;
//! - decide resource sufficiency;
//! - implement target feasibility or placement;
//! - implement backend dispatch or runtime execution;
//! - define a second semantic capability model;
//! - define a second quantum IR or universal IR.
//!
//! ## Authority
//!
//! - `grammar/Zamani.g4`
//! - `grammar/hybrid/hybrid.g4`
//! - `grammar/hybrid/hybrid-resources.g4`
//! - `grammar/hybrid/README.md`
//! - `grammar/spec/hybrid.md`
//! - `src/frontend/semantic/capabilities/model.rs`
//! - `src/frontend/semantic/capabilities/evidence.rs`
//! - `src/frontend/semantic/capabilities/compatibility.rs`
//! - `src/hybrid/resources/requirements.rs`
//! - `src/hybrid/domain/interface.rs`
//! - `src/hybrid/config.rs`
//!
//! The grammar describes source-level intent. The semantic subsystem owns
//! capability meaning. Providers supply scoped evidence. Downstream planning
//! determines whether a target can realize the complete computation.
//!
//! ## Scalability and POCO-REAF
//!
//! Capability identities are open-world: this module does not enumerate
//! CPUs, GPUs, FPGAs, QPUs, accelerators, or future computational domains.
//!
//! There is no universal maximum number of capabilities or evidence records.
//! Collection growth is constrained by actual memory and any explicit
//! operational policy enforced by an owning caller.
//!
//! No finite implementation can process literally infinite input. This
//! module introduces no artificial hardware-capacity ceiling and does not
//! turn operational exhaustion into a claim that a capability is absent.
//!
//! ## Evidence invariants
//!
//! - Evidence applies only to its declared subject.
//! - Evidence for different subjects is never combined.
//! - Unsupported evidence is not silently ignored.
//! - Unknown or incomplete evidence is not treated as support.
//! - Contradictory evidence remains observable.
//! - Exact duplicate records follow the canonical evidence-set policy.
//! - Evaluation results are not proof of authenticity or authorization.
//! - A supported capability does not establish sufficient resource capacity.
//!
//! ## Integration
//!
//! `frontend::semantic::capabilities::evidence` owns the canonical evidence
//! records and evidence evaluation.
//!
//! `frontend::semantic::capabilities::model` owns the canonical capability
//! requirements.
//!
//! The hybrid compiler adapter may pass the resulting evidence evaluation
//! into downstream compatibility and feasibility analysis. It must not
//! interpret missing evidence as successful feasibility.
//!
//! `resources/mod.rs` must declare this module. That sibling module is not
//! currently present in the inspected repository and must be added as part
//! of integration.
//!
//! No hardware, backend, quantum IR, ZUIR, or runtime dependency is needed
//! here.

use core::fmt;
use std::error::Error;

use crate::frontend::semantic::capabilities::evidence::{
    CapabilityEvidence,
    CapabilityEvidenceError,
    CapabilityEvidenceResult,
    CapabilityEvidenceSet,
    EvidenceSubject,
};
use crate::frontend::semantic::capabilities::model::CapabilityRequirement;

/// Re-export the canonical semantic capability identity.
///
/// This is a re-export, not a hybrid-specific capability type.
pub use crate::frontend::semantic::capabilities::model::CapabilityId;

/// Re-export the canonical semantic capability requirement.
///
/// Requirements describe program intent; they do not prove target support.
pub use crate::frontend::semantic::capabilities::model::CapabilityRequirement as SemanticCapabilityRequirement;

/// Re-export the canonical semantic evidence status.
pub use crate::frontend::semantic::capabilities::evidence::CapabilityEvidenceStatus;

/// Stable architectural identity for this module.
pub const HYBRID_CAPABILITIES_MODULE_NAME: &str =
    "zamani.hybrid.resources.capabilities";

/// Version of this module's in-memory scope-view contract.
///
/// This is not the Zamani language version, semantic-model version, or
/// serialized artifact version.
pub const HYBRID_CAPABILITIES_SCHEMA_VERSION: u16 = 1;

/// Errors returned while constructing or updating a hybrid capability view.
///
/// These errors describe invalid evidence integration. They do not claim
/// that a target is feasible or infeasible.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum HybridCapabilityError {
    /// The supplied evidence is structurally invalid.
    InvalidEvidence(CapabilityEvidenceError),

    /// Evidence belongs to a different subject than this view.
    ///
    /// Evidence must not be moved across scopes implicitly.
    SubjectMismatch {
        /// Subject declared by this view.
        expected: String,

        /// Subject attached to the supplied evidence.
        actual: String,
    },
}

impl fmt::Display for HybridCapabilityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidEvidence(error) => {
                write!(formatter, "invalid hybrid capability evidence: {error}")
            }
            Self::SubjectMismatch { expected, actual } => write!(
                formatter,
                "capability evidence subject mismatch: expected '{expected}', \
                 received '{actual}'"
            ),
        }
    }
}

impl Error for HybridCapabilityError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidEvidence(error) => Some(error),
            Self::SubjectMismatch { .. } => None,
        }
    }
}

impl From<CapabilityEvidenceError> for HybridCapabilityError {
    fn from(error: CapabilityEvidenceError) -> Self {
        Self::InvalidEvidence(error)
    }
}

/// A capability-evidence view for one explicit subject.
///
/// A subject is the scope against which capability claims are evaluated. Its
/// exact meaning is determined by the caller and provider contract; it might
/// identify a target description, deployment environment, simulator profile,
/// or another explicitly defined capability scope.
///
/// This structure does not itself discover or represent physical devices.
/// Its evidence is descriptive and is not automatically trusted.
///
/// All records in the view must refer to the same subject. Evidence for
/// another subject is rejected instead of silently mixed into this view.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HybridCapabilityView {
    subject: EvidenceSubject,
    evidence: CapabilityEvidenceSet,
}

impl HybridCapabilityView {
    /// Create an empty view for an explicitly identified subject.
    ///
    /// An empty view means that no evidence has been supplied to this view.
    /// It does not mean that the subject supports no capabilities.
    #[must_use]
    pub fn new(subject: EvidenceSubject) -> Self {
        Self {
            subject,
            evidence: CapabilityEvidenceSet::new(),
        }
    }

    /// Return the subject to which this view applies.
    #[must_use]
    pub const fn subject(&self) -> &EvidenceSubject {
        &self.subject
    }

    /// Borrow the canonical evidence set.
    ///
    /// The returned set retains the semantic subsystem's deterministic
    /// ordering and conflict-preservation behavior.
    #[must_use]
    pub const fn evidence(&self) -> &CapabilityEvidenceSet {
        &self.evidence
    }

    /// Return the number of distinct evidence records in this view.
    #[must_use]
    pub fn len(&self) -> usize {
        self.evidence.len()
    }

    /// Return whether this view contains any evidence records.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.evidence.is_empty()
    }

    /// Iterate over evidence records in canonical deterministic order.
    pub fn iter(&self) -> impl Iterator<Item = &CapabilityEvidence> {
        self.evidence.iter()
    }

    /// Insert one evidence record after validating its scope and structure.
    ///
    /// Returns `true` when a new distinct record was inserted. An exact
    /// duplicate follows the canonical evidence-set behavior and returns
    /// `false`.
    ///
    /// A record for another subject is rejected. The existing view remains
    /// unchanged on failure.
    pub fn insert(
        &mut self,
        evidence: CapabilityEvidence,
    ) -> Result<bool, HybridCapabilityError> {
        self.ensure_subject(evidence.subject())?;
        evidence.validate()?;

        self.evidence
            .insert(evidence)
            .map_err(HybridCapabilityError::InvalidEvidence)
    }

    /// Add multiple records transactionally.
    ///
    /// Every record must belong to this view's subject and pass canonical
    /// evidence validation. If any record fails, the original evidence set
    /// remains unchanged.
    ///
    /// Exact duplicates are handled by the canonical evidence-set policy.
    pub fn extend<I>(
        &mut self,
        records: I,
    ) -> Result<(), HybridCapabilityError>
    where
        I: IntoIterator<Item = CapabilityEvidence>,
    {
        // Work on a candidate so errors cannot partially update this view.
        let mut candidate = self.evidence.clone();

        for record in records {
            self.ensure_subject(record.subject())?;
            record.validate()?;
            candidate
                .insert(record)
                .map_err(HybridCapabilityError::InvalidEvidence)?;
        }

        self.evidence = candidate;
        Ok(())
    }

    /// Evaluate one canonical semantic requirement against this subject's
    /// evidence.
    ///
    /// The result distinguishes satisfaction, lack of evidence, non-support,
    /// indeterminate evidence, and conflicting evidence according to the
    /// canonical semantic evidence subsystem.
    ///
    /// This method does not check resource quantities, allocation, scheduling,
    /// authorization, or complete target feasibility.
    #[must_use]
    pub fn evaluate(
        &self,
        requirement: &CapabilityRequirement,
    ) -> CapabilityEvidenceResult {
        self.evidence.evaluate(&self.subject, requirement)
    }

    /// Validate this view and every contained record.
    ///
    /// This check is useful at trust boundaries, including after an external
    /// representation has been converted into in-memory values.
    pub fn validate(&self) -> Result<(), HybridCapabilityError> {
        self.evidence
            .validate()
            .map_err(HybridCapabilityError::InvalidEvidence)?;

        for record in self.evidence.iter() {
            self.ensure_subject(record.subject())?;
        }

        Ok(())
    }

    fn ensure_subject(
        &self,
        actual: &EvidenceSubject,
    ) -> Result<(), HybridCapabilityError> {
        if actual == &self.subject {
            return Ok(());
        }

        Err(HybridCapabilityError::SubjectMismatch {
            expected: self.subject.as_str().to_owned(),
            actual: actual.as_str().to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeMap;

    fn subject(value: &str) -> EvidenceSubject {
        EvidenceSubject::new(value).expect("valid test subject")
    }

    fn capability() -> CapabilityId {
        CapabilityId::new("zamani.quantum", "measurement")
            .expect("valid test capability")
    }

    fn evidence(
        scope: &str,
        status: CapabilityEvidenceStatus,
    ) -> CapabilityEvidence {
        let origin = crate::frontend::semantic::capabilities::evidence::EvidenceOrigin::new(
            "test",
            "hybrid-capabilities-tests",
            None,
        )
        .expect("valid test evidence origin");

        CapabilityEvidence::new(
            capability(),
            subject(scope),
            status,
            None,
            BTreeMap::new(),
            origin,
        )
        .expect("valid test evidence")
    }

    #[test]
    fn empty_view_does_not_claim_capability_support() {
        let view = HybridCapabilityView::new(subject("test-target"));
        let requirement = CapabilityRequirement::new(capability());

        assert!(view.is_empty());
        assert_eq!(
            view.evaluate(&requirement),
            CapabilityEvidenceResult::NoEvidence
        );
    }

    #[test]
    fn matching_supported_evidence_satisfies_requirement() {
        let mut view = HybridCapabilityView::new(subject("test-target"));
        view.insert(evidence(
            "test-target",
            CapabilityEvidenceStatus::Supported,
        ))
        .expect("matching evidence should be accepted");

        let requirement = CapabilityRequirement::new(capability());

        assert_eq!(
            view.evaluate(&requirement),
            CapabilityEvidenceResult::Satisfied
        );
        assert!(view.validate().is_ok());
    }

    #[test]
    fn evidence_for_another_subject_is_rejected() {
        let mut view = HybridCapabilityView::new(subject("target-a"));
        let error = view
            .insert(evidence(
                "target-b",
                CapabilityEvidenceStatus::Supported,
            ))
            .expect_err("cross-scope evidence must be rejected");

        assert!(matches!(
            error,
            HybridCapabilityError::SubjectMismatch { .. }
        ));
        assert!(view.is_empty());
    }

    #[test]
    fn contradictory_evidence_is_not_silently_overwritten() {
        let mut view = HybridCapabilityView::new(subject("test-target"));

        view.insert(evidence(
            "test-target",
            CapabilityEvidenceStatus::Supported,
        ))
        .expect("supported evidence should be accepted");

        view.insert(evidence(
            "test-target",
            CapabilityEvidenceStatus::Unsupported,
        ))
        .expect("contradictory evidence should be retained");

        let requirement = CapabilityRequirement::new(capability());

        assert_eq!(
            view.evaluate(&requirement),
            CapabilityEvidenceResult::ConflictingEvidence
        );
        assert_eq!(view.len(), 2);
    }

    #[test]
    fn extending_with_wrong_scope_is_transactional() {
        let mut view = HybridCapabilityView::new(subject("target-a"));
        view.insert(evidence(
            "target-a",
            CapabilityEvidenceStatus::Supported,
        ))
        .expect("initial evidence should be accepted");

        let before = view.clone();

        let result = view.extend([
            evidence("target-a", CapabilityEvidenceStatus::Unknown),
            evidence("target-b", CapabilityEvidenceStatus::Supported),
        ]);

        assert!(result.is_err());
        assert_eq!(view, before);
    }

    #[test]
    fn exact_duplicate_evidence_is_idempotent() {
        let mut view = HybridCapabilityView::new(subject("test-target"));
        let record = evidence("test-target", CapabilityEvidenceStatus::Supported);

        assert!(view.insert(record.clone()).expect("first insert succeeds"));
        assert!(!view.insert(record).expect("duplicate insert succeeds"));
        assert_eq!(view.len(), 1);
    }
}
