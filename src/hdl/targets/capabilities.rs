
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Target Capabilities
//!
//! Path: `src/hdl/targets/capabilities.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Evaluate canonical capability requirements against evidence attached to an
//! HDL target description, for one explicitly selected evidence scope.
//!
//! This module is an integration adapter. It does not define a new capability
//! identity, requirement language, evidence format, or compatibility engine.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/physical-intent.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `src/frontend/semantic/capabilities/model.rs`
//! - `src/frontend/semantic/capabilities/evidence.rs`
//! - `src/frontend/semantic/capabilities/requirements.rs`
//! - `src/frontend/semantic/capabilities/compatibility.rs`
//! - `src/hdl/capabilities.rs`
//! - `src/hdl/targets/model.rs`
//!
//! The grammar defines syntax. The semantic model defines capability meaning.
//! The canonical evidence and compatibility modules define how evidence is
//! represented and evaluated. The target model owns the target description.
//!
//! ## Owns
//!
//! - Target-scoped capability compatibility assessment.
//! - Validation and collection of evidence relevant to an explicit scope.
//! - A structured result that retains target identity and description status.
//! - Errors crossing the target/capability integration boundary.
//!
//! ## Does not own
//!
//! - Capability identity, versions, properties, or source requirements.
//! - A registry of known capabilities or a closed list of hardware features.
//! - Hardware discovery, evidence authentication, or provider trust.
//! - Resource feasibility, physical allocation, device selection, or deployment.
//! - Source parsing, AST construction, or source-level semantic validity.
//! - Synthesis, placement, routing, scheduling, or timing closure.
//! - HDLIR structures, builders, passes, verification, or lowering.
//!
//! All HDLIR implementation remains under `src/hdl/hdlir/` and is deferred.
//!
//! ## Semantics
//!
//! Evidence is evaluated only for the `EvidenceSubject` explicitly supplied by
//! the caller. Evidence for another subject must never satisfy a requirement
//! in the requested scope.
//!
//! `Supported` evidence is a scoped claim, not cryptographic attestation or
//! proof of physical availability. This module does not authenticate claims.
//!
//! Missing, conflicting, incomplete, or unsupported evidence remains subject
//! to the canonical compatibility evaluator's outcome. It is never silently
//! upgraded to success.
//!
//! Capability compatibility is not overall target feasibility. Resource
//! availability, timing constraints, policies, provider trust, and other
//! requirements must be assessed independently.
//!
//! An empty requirement collection produces an empty report. It is explicitly
//! identified as a vacuous assessment and must not be interpreted as proof
//! that the target supports any particular capability.
//!
//! ## Scalability and determinism
//!
//! There are no fixed maximum counts for targets, requirements, capabilities,
//! evidence records, or hardware units in this module.
//!
//! Collections grow subject to available memory and caller-defined operational
//! budgets. Rust collections and execution resources are finite; literal
//! infinity is not promised.
//!
//! Evidence storage and report ordering follow the canonical semantic
//! subsystem. This module introduces no global mutable state, random IDs,
//! target-specific constants, or implicit hardware defaults.
//!
//! Operational-budget exhaustion must be reported by the owning analysis
//! session and must never be interpreted as successful compatibility.
//!
//! ## Integration
//!
//! 1. Semantic analysis produces canonical `CapabilityRequirements`.
//! 2. A target provider supplies a validated `HdlTargetDescription`.
//! 3. The caller selects the evidence scope to assess.
//! 4. This module validates the target and gathers evidence for that scope.
//! 5. The canonical compatibility evaluator creates the report.
//! 6. Target feasibility combines the report with resource and constraint
//!    analysis in a separate downstream step.
//!
//! The source parser and semantic validator must not depend on this module.
//!
//! ## Freeze criteria
//!
//! - Canonical capability and evidence types are reused without duplication.
//! - Evidence is evaluated only in the requested scope.
//! - Invalid target or evidence records return structured errors.
//! - Empty requirements are not presented as positive capability proof.
//! - Target description completeness is retained in the result.
//! - The module is registered by `src/hdl/targets/mod.rs`.
//! - Unit tests and actual workspace checks pass.
//!
//! This file's contract may be revised only to correct a demonstrated defect
//! or evolve a coordinated, versioned integration API.

use core::fmt;
use std::error::Error;

use crate::frontend::semantic::capabilities::{
    evaluate_compatibility,
    CapabilityCompatibilityError,
    CapabilityCompatibilityReport,
    CapabilityEvidenceError,
    CapabilityEvidenceSet,
    CapabilityRequirements,
    EvidenceSubject,
};

use crate::hdl::capabilities::HdlCapabilityEvidence;
use crate::hdl::targets::model::{
    HdlTargetDescription,
    HdlTargetModelError,
    TargetDescriptionStatus,
    TargetId,
};

/// Stable architectural name for this module.
pub const HDL_TARGET_CAPABILITIES_MODULE_NAME: &str =
    "zamani.hdl.targets.capabilities";

/// Version of this module's in-memory assessment contract.
///
/// This is not the Zamani language version, target provider version, or
/// capability-evidence schema version.
pub const HDL_TARGET_CAPABILITIES_SCHEMA_VERSION: u16 = 1;

/// A capability-compatibility assessment for one target and one evidence scope.
///
/// The report preserves the canonical compatibility outcomes. It does not
/// independently establish that the target exists, that evidence is trusted,
/// that resources are sufficient, or that a design can be realized.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlTargetCapabilityAssessment {
    target_id: TargetId,
    target_status: TargetDescriptionStatus,
    subject: EvidenceSubject,
    report: CapabilityCompatibilityReport,
}

impl HdlTargetCapabilityAssessment {
    /// Return the identity of the target description that was assessed.
    #[must_use]
    pub fn target_id(&self) -> &TargetId {
        &self.target_id
    }

    /// Return the target description's declared completeness status.
    ///
    /// A partial or unknown target description remains partial or unknown
    /// even if the supplied evidence satisfies all evaluated requirements.
    #[must_use]
    pub const fn target_status(&self) -> TargetDescriptionStatus {
        self.target_status
    }

    /// Return the exact evidence scope that was evaluated.
    #[must_use]
    pub fn subject(&self) -> &EvidenceSubject {
        &self.subject
    }

    /// Return the canonical compatibility report.
    ///
    /// Inspect its individual records and summary. An empty report means
    /// there were no requirements to evaluate; it is not capability proof.
    #[must_use]
    pub fn report(&self) -> &CapabilityCompatibilityReport {
        &self.report
    }

    /// Return whether the assessment contained no requirements.
    ///
    /// `true` means the assessment was vacuous, not that a capability was
    /// established or that the target is feasible.
    #[must_use]
    pub fn is_vacuous(&self) -> bool {
        self.report.is_empty()
    }

    /// Return the number of capability requirements evaluated.
    #[must_use]
    pub fn len(&self) -> usize {
        self.report.len()
    }

    /// Return whether no capability requirements were evaluated.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.report.is_empty()
    }
}

/// Errors raised at the target-capability integration boundary.
///
/// The underlying canonical errors are preserved rather than flattened into
/// strings. This allows callers to distinguish invalid target descriptions,
/// invalid evidence, and compatibility evaluation failures.
#[derive(Debug)]
#[non_exhaustive]
pub enum HdlTargetCapabilityError {
    /// The target description violated its local invariants.
    InvalidTarget(HdlTargetModelError),

    /// A capability-evidence record violated its canonical invariants.
    InvalidEvidence(CapabilityEvidenceError),

    /// The canonical compatibility evaluator failed.
    Compatibility(CapabilityCompatibilityError),
}

impl fmt::Display for HdlTargetCapabilityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTarget(error) => {
                write!(formatter, "invalid HDL target description: {error}")
            }
            Self::InvalidEvidence(error) => {
                write!(formatter, "invalid HDL target capability evidence: {error}")
            }
            Self::Compatibility(error) => {
                write!(formatter, "HDL target capability evaluation failed: {error}")
            }
        }
    }
}

impl Error for HdlTargetCapabilityError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidTarget(error) => Some(error),
            Self::InvalidEvidence(error) => Some(error),
            Self::Compatibility(error) => Some(error),
        }
    }
}

/// Assess target capabilities against canonical semantic requirements.
///
/// Only evidence whose subject exactly matches `subject` is included in the
/// evaluation. Evidence for other subjects cannot satisfy requirements in
/// this scope.
///
/// The target is validated before evidence is used. Relevant evidence is
/// inserted into the canonical evidence set, which validates records and
/// preserves deterministic, conflict-aware semantics. Compatibility is then
/// evaluated by the existing shared semantic evaluator.
///
/// This function does not perform target discovery, authenticate evidence,
/// allocate resources, or establish overall design feasibility.
///
/// # Errors
///
/// Returns an error if the target description is invalid, if relevant
/// evidence violates canonical invariants, or if compatibility evaluation
/// fails.
///
/// # Vacuous assessments
///
/// An empty `requirements` collection returns an empty report. Callers must
/// inspect [`HdlTargetCapabilityAssessment::is_vacuous`] rather than treating
/// that result as evidence that the target supports any capability.
pub fn assess_target_capabilities(
    target: &HdlTargetDescription,
    requirements: &CapabilityRequirements,
    subject: &EvidenceSubject,
) -> Result<HdlTargetCapabilityAssessment, HdlTargetCapabilityError> {
    target
        .validate()
        .map_err(HdlTargetCapabilityError::InvalidTarget)?;

    let evidence = scoped_evidence(target.capability_evidence(), subject)?;

    let report = evaluate_compatibility(requirements, subject, &evidence)
        .map_err(HdlTargetCapabilityError::Compatibility)?;

    Ok(HdlTargetCapabilityAssessment {
        target_id: target.id().clone(),
        target_status: target.status(),
        subject: subject.clone(),
        report,
    })
}

/// Build a validated canonical evidence set for one explicit subject.
///
/// The input order does not define compatibility precedence. The canonical
/// set and evaluator preserve their own deterministic behavior and do not
/// silently discard conflicting claims from different origins.
fn scoped_evidence(
    records: &[HdlCapabilityEvidence],
    subject: &EvidenceSubject,
) -> Result<CapabilityEvidenceSet, HdlTargetCapabilityError> {
    let mut evidence = CapabilityEvidenceSet::new();

    for record in records {
        if record.subject() != subject {
            continue;
        }

        evidence
            .insert(record.clone())
            .map_err(HdlTargetCapabilityError::InvalidEvidence)?;
    }

    Ok(evidence)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hdl::targets::model::TargetId;

    fn target() -> HdlTargetDescription {
        HdlTargetDescription::new(
            TargetId::new("test.target").expect("valid target ID"),
            "Test target",
            "test.simulator",
        )
        .expect("valid target description")
    }

    #[test]
    fn empty_requirements_are_explicitly_vacuous() {
        let target = target();
        let requirements = CapabilityRequirements::new();
        let subject = EvidenceSubject::new("test.profile")
            .expect("valid evidence subject");

        let assessment = assess_target_capabilities(
            &target,
            &requirements,
            &subject,
        )
        .expect("an empty assessment should be representable");

        assert!(assessment.is_vacuous());
        assert!(assessment.is_empty());
        assert_eq!(assessment.len(), 0);
        assert_eq!(assessment.target_id().as_str(), "test.target");
        assert_eq!(assessment.subject(), &subject);
        assert_eq!(
            assessment.target_status(),
            TargetDescriptionStatus::Partial
        );
    }

    #[test]
    fn assessment_preserves_target_completeness_status() {
        let mut target = target();
        target.set_status(TargetDescriptionStatus::Unknown);

        let requirements = CapabilityRequirements::new();
        let subject = EvidenceSubject::new("test.profile")
            .expect("valid evidence subject");

        let assessment = assess_target_capabilities(
            &target,
            &requirements,
            &subject,
        )
        .expect("an empty assessment should be representable");

        assert_eq!(
            assessment.target_status(),
            TargetDescriptionStatus::Unknown
        );
    }

    #[test]
    fn target_scope_filter_does_not_import_other_subjects() {
        use crate::frontend::semantic::capabilities::{
            CapabilityEvidence,
            CapabilityEvidenceStatus,
            CapabilityId,
            CapabilityVersion,
            EvidenceOrigin,
        };

        let capability = CapabilityId::new(
            "zamani.hdl",
            "combinational_logic",
        )
        .expect("valid capability ID");

        let other_subject = EvidenceSubject::new("another.profile")
            .expect("valid evidence subject");

        let origin = EvidenceOrigin::new(
            "test",
            "unit-test",
            None,
        )
        .expect("valid evidence origin");

        let record = CapabilityEvidence::new(
            capability,
            other_subject.clone(),
            CapabilityEvidenceStatus::Supported,
            None::<CapabilityVersion>,
            Default::default(),
            origin,
        )
        .expect("valid evidence");

        let evidence = scoped_evidence(
            &[record],
            &EvidenceSubject::new("requested.profile")
                .expect("valid requested subject"),
        )
        .expect("valid scoped evidence");

        assert!(evidence.is_empty());
        assert!(evidence.iter().all(|item| item.subject() != &other_subject));
    }
}
