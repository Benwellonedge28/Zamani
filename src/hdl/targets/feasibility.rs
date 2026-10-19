
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Target Feasibility
//!
//! Path: `src/hdl/targets/feasibility.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97+.
//! Safety: Safe Rust only.
//!
//! ## Status
//!
//! Conservative target-feasibility integration boundary.
//!
//! This implementation establishes capability-based infeasibility when the
//! canonical evaluator proves that a required capability is incompatible.
//! It does not claim complete feasibility because the current target model
//! does not yet represent target resource capacities and does not provide a
//! complete target-constraint or realization assessment.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/physical-intent.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/semantic-boundaries.md`
//! - `grammar/specification/scalability-model.md`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/frontend/semantic/capabilities/requirements.rs`
//! - `src/frontend/semantic/capabilities/evidence.rs`
//! - `src/frontend/semantic/capabilities/compatibility.rs`
//! - `src/frontend/semantic/resources/model.rs`
//! - `src/frontend/semantic/resources/constraints.rs`
//! - `src/hdl/capabilities.rs`
//! - `src/hdl/resources.rs`
//! - `src/hdl/requirements.rs`
//! - `src/hdl/targets/model.rs`
//! - `src/hdl/targets/capabilities.rs`
//!
//! The grammar defines syntax. The frontend semantic layer owns source
//! meaning. The shared semantic subsystem owns capability and resource
//! semantics. The target model owns target descriptions. The target
//! capability adapter evaluates scoped capability evidence. This module
//! combines that evidence conservatively without duplicating those contracts.
//!
//! ## Owns
//!
//! - A structured overall feasibility status.
//! - Integration of canonical capability compatibility results.
//! - Explicit identification of feasibility dimensions not yet assessed.
//! - A structured error boundary for failed capability assessment.
//! - Conservative decision rules that do not treat missing evidence as proof.
//!
//! ## Does not own
//!
//! - Source parsing, AST construction, or source-level semantic validity.
//! - A second capability identity, requirement model, or evidence evaluator.
//! - A second resource quantity, unit, or constraint evaluator.
//! - Hardware discovery, provider authentication, or authorization.
//! - Resource allocation, synthesis, scheduling, placement, or routing.
//! - Physical timing closure, vendor-specific implementation, or deployment.
//! - Classical IR, quantum IR, or HDLIR.
//!
//! All HDLIR implementation remains under `src/hdl/hdlir/` and is deferred.
//!
//! ## Feasibility semantics
//!
//! `Infeasible` means the canonical capability evaluator has established at
//! least one required capability as incompatible with the requested scope.
//! A single known mandatory incompatibility is sufficient to reject that
//! target for the supplied requirements.
//!
//! `Indeterminate` means no such decisive incompatibility has been proven,
//! but full feasibility has not been established. Missing, unknown, or
//! contradictory capability evidence cannot be promoted to success.
//!
//! `Feasible` is reserved for a future complete evaluator. This implementation
//! never returns it because the current target API does not expose enough
//! information to prove all applicable feasibility dimensions.
//!
//! In particular, `HdlResourceProfile` stores resource requirements and
//! estimates. It is not a target-capacity inventory. An absent resource profile
//! or an existing profile must not be interpreted as proof of capacity.
//!
//! ## Scalability and POCO-REAF
//!
//! There are no fixed maximum counts for targets, requirements, capabilities,
//! evidence records, resource kinds, or hardware units in this module.
//!
//! This module delegates capability evaluation to the canonical evaluator.
//! It does not enumerate supported hardware types or bake hardware capacities
//! into source-level semantics.
//!
//! Real processing remains bounded by available memory, execution time, and
//! caller-defined operational budgets. Literal infinity is not promised.
//! Budget exhaustion must be reported by the owning analysis session and must
//! never be converted into a successful feasibility result.
//!
//! ## Integration contract
//!
//! 1. Semantic analysis supplies canonical `CapabilityRequirements`.
//! 2. The caller supplies an explicit `EvidenceSubject`.
//! 3. A validated `HdlTargetDescription` supplies scoped capability evidence.
//! 4. `targets/capabilities.rs` invokes the canonical compatibility evaluator.
//! 5. This module derives a conservative feasibility status from that report.
//! 6. Resource-capacity, timing, target-constraint, provider-availability, and
//!    physical-realization assessments must be integrated before an overall
//!    positive feasibility claim is permitted.
//!
//! This module must not make the parser or source semantic validator depend
//! on target feasibility.
//!
//! ## Freeze criteria
//!
//! - Canonical capability evaluation is reused.
//! - A proven mandatory capability incompatibility cannot be reported as
//!   feasible.
//! - Missing or contradictory evidence cannot be reported as feasible.
//! - Unassessed dimensions are explicitly exposed to callers.
//! - Resource requirements and estimates are not misrepresented as capacities.
//! - Tests and actual workspace checks pass.
//!
//! Public contract changes require coordinated updates to consumers and tests.

use core::fmt;
use std::error::Error;

use crate::frontend::semantic::capabilities::{
    CapabilityRequirements,
    EvidenceSubject,
};

use crate::hdl::targets::capabilities::{
    assess_target_capabilities,
    HdlTargetCapabilityAssessment,
    HdlTargetCapabilityError,
};
use crate::hdl::targets::model::{
    HdlTargetDescription,
    TargetDescriptionStatus,
    TargetId,
};

/// Stable architectural name of this module.
pub const HDL_TARGET_FEASIBILITY_MODULE_NAME: &str =
    "zamani.hdl.targets.feasibility";

/// Version of this module's in-memory feasibility-report contract.
///
/// This is not the Zamani language version, Rust version, target provider
/// version, or HDLIR schema version.
pub const HDL_TARGET_FEASIBILITY_SCHEMA_VERSION: u16 = 1;

/// Overall feasibility conclusion for a target and a set of requirements.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FeasibilityStatus {
    /// At least one mandatory capability requirement is proven incompatible.
    ///
    /// The target cannot satisfy the supplied requirements in the assessed
    /// scope, regardless of the dimensions that remain unassessed.
    Infeasible,

    /// No decisive incompatibility has been established, but full feasibility
    /// has not been proven.
    ///
    /// This is the conservative result when information is missing,
    /// contradictory, incomplete, or not covered by the current evaluator.
    Indeterminate,

    /// All applicable mandatory requirements and realization conditions have
    /// been established as satisfied.
    ///
    /// Reserved for a complete evaluator. The current implementation never
    /// returns this status because the target model does not yet provide the
    /// complete resource-capacity and realization evidence needed to prove it.
    Feasible,
}

impl FeasibilityStatus {
    /// Return whether the target has been proven infeasible.
    #[must_use]
    pub const fn is_infeasible(self) -> bool {
        matches!(self, Self::Infeasible)
    }

    /// Return whether the overall feasibility decision remains unresolved.
    #[must_use]
    pub const fn is_indeterminate(self) -> bool {
        matches!(self, Self::Indeterminate)
    }

    /// Return whether full feasibility has been proven.
    #[must_use]
    pub const fn is_feasible(self) -> bool {
        matches!(self, Self::Feasible)
    }
}

/// A dimension of feasibility that this implementation cannot yet establish.
///
/// These values are explicit reporting markers, not claims that every
/// dimension necessarily applies to every possible design. Future integration
/// can remove a dimension from the unresolved set only when an authoritative
/// evaluator supplies a sufficient result for that dimension.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UnassessedDimension {
    /// Whether the target's complete required resource capacities are known
    /// and sufficient for the design.
    ResourceCapacity,

    /// Whether all applicable logical timing requirements and constraints
    /// have been evaluated.
    TimingAndConstraints,

    /// Whether target-specific restrictions and implementation constraints
    /// have been evaluated.
    TargetSpecificConstraints,

    /// Whether the target and its capability claims are available and
    /// sufficiently trusted for the requested operation.
    ProviderAvailabilityAndTrust,

    /// Whether a supported realization process can implement the design
    /// while preserving its required semantics.
    PhysicalRealization,
}

impl UnassessedDimension {
    /// Return a stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResourceCapacity => "resource_capacity",
            Self::TimingAndConstraints => "timing_and_constraints",
            Self::TargetSpecificConstraints => "target_specific_constraints",
            Self::ProviderAvailabilityAndTrust => {
                "provider_availability_and_trust"
            }
            Self::PhysicalRealization => "physical_realization",
        }
    }
}

impl fmt::Display for UnassessedDimension {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A conservative feasibility report for one target and capability scope.
///
/// The embedded capability assessment preserves the canonical report,
/// including per-requirement outcomes and source provenance.
///
/// `unassessed_dimensions` lists areas that this implementation cannot prove.
/// A nonempty list means the overall status cannot be `Feasible`. The current
/// implementation deliberately leaves these dimensions unresolved because
/// their complete evidence contracts are not yet available at this boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlTargetFeasibilityReport {
    target_id: TargetId,
    target_status: TargetDescriptionStatus,
    status: FeasibilityStatus,
    capability_assessment: HdlTargetCapabilityAssessment,
    unassessed_dimensions: Vec<UnassessedDimension>,
}

impl HdlTargetFeasibilityReport {
    /// Return the identity of the assessed target description.
    #[must_use]
    pub fn target_id(&self) -> &TargetId {
        &self.target_id
    }

    /// Return the target description's declared completeness status.
    ///
    /// `Complete` is a declaration made by the provider or caller. It is not
    /// independent authentication or proof of physical availability.
    #[must_use]
    pub const fn target_status(&self) -> TargetDescriptionStatus {
        self.target_status
    }

    /// Return the overall feasibility conclusion.
    #[must_use]
    pub const fn status(&self) -> FeasibilityStatus {
        self.status
    }

    /// Return the canonical capability assessment.
    ///
    /// Inspect the per-requirement results and summary for the detailed
    /// evidence behind the capability portion of the decision.
    #[must_use]
    pub fn capability_assessment(&self) -> &HdlTargetCapabilityAssessment {
        &self.capability_assessment
    }

    /// Return dimensions that remain unassessed.
    ///
    /// The returned slice has deterministic order and contains no duplicate
    /// entries.
    #[must_use]
    pub fn unassessed_dimensions(&self) -> &[UnassessedDimension] {
        &self.unassessed_dimensions
    }

    /// Return whether capability requirements were vacuous.
    ///
    /// A vacuous capability assessment is not proof that a design is feasible.
    #[must_use]
    pub fn has_vacuous_capability_assessment(&self) -> bool {
        self.capability_assessment.is_vacuous()
    }

    /// Return whether full feasibility has been established.
    ///
    /// This remains `false` for all reports produced by the current
    /// implementation.
    #[must_use]
    pub const fn is_feasible(&self) -> bool {
        self.status.is_feasible()
    }

    /// Return whether the target has been proven infeasible.
    #[must_use]
    pub const fn is_infeasible(&self) -> bool {
        self.status.is_infeasible()
    }

    /// Return whether more evidence or analysis is required.
    #[must_use]
    pub const fn is_indeterminate(&self) -> bool {
        self.status.is_indeterminate()
    }
}

/// Errors raised while preparing a target-feasibility report.
///
/// These are failures in the input description or capability-evaluation
/// process, not feasibility outcomes. A malformed evidence record must not
/// be converted into `Infeasible` or `Feasible`.
#[derive(Debug)]
#[non_exhaustive]
pub enum HdlTargetFeasibilityError {
    /// Target validation, scoped evidence collection, or capability
    /// compatibility evaluation failed.
    CapabilityAssessment(HdlTargetCapabilityError),
}

impl fmt::Display for HdlTargetFeasibilityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CapabilityAssessment(error) => {
                write!(
                    formatter,
                    "cannot assess HDL target feasibility: {error}"
                )
            }
        }
    }
}

impl Error for HdlTargetFeasibilityError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::CapabilityAssessment(error) => Some(error),
        }
    }
}

/// Assess a target against canonical capability requirements.
///
/// The caller must supply the evidence scope explicitly. Evidence belonging
/// to another scope is not used to satisfy these requirements.
///
/// The result is deliberately conservative:
///
/// - A proven incompatible mandatory capability yields `Infeasible`.
/// - Missing, conflicting, or indeterminate capability evidence yields
///   `Indeterminate`.
/// - A capability assessment with every nonempty requirement satisfied still
///   yields `Indeterminate` because resource capacity, timing, target
///   constraints, provider availability/trust, and physical realization are
///   not fully evaluated by the current repository contracts.
///
/// The current target model's resource profile is not a capacity inventory:
/// it holds resource requirements and estimates. This function therefore
/// does not compare those entries against fictional or inferred capacities.
///
/// This function does not validate source-language semantics, allocate
/// hardware, invoke a synthesis provider, or perform HDLIR lowering.
///
/// # Errors
///
/// Returns an error if the target description or relevant capability
/// evidence fails validation, or if the canonical capability evaluator fails.
pub fn assess_target_feasibility(
    target: &HdlTargetDescription,
    requirements: &CapabilityRequirements,
    subject: &EvidenceSubject,
) -> Result<HdlTargetFeasibilityReport, HdlTargetFeasibilityError> {
    let capability_assessment =
        assess_target_capabilities(target, requirements, subject)
            .map_err(HdlTargetFeasibilityError::CapabilityAssessment)?;

    let summary = capability_assessment.report().summary();

    // A single proven incompatibility in a mandatory capability requirement
    // is sufficient to establish that this target cannot meet the supplied
    // requirement set. Unresolved requirements do not erase this fact.
    let status = if summary.has_incompatibilities() {
        FeasibilityStatus::Infeasible
    } else {
        // The repository currently has no target-capacity evidence contract
        // that can establish complete overall feasibility. In particular,
        // HdlResourceProfile is not a target inventory.
        FeasibilityStatus::Indeterminate
    };

    let unassessed_dimensions = vec![
        UnassessedDimension::ResourceCapacity,
        UnassessedDimension::TimingAndConstraints,
        UnassessedDimension::TargetSpecificConstraints,
        UnassessedDimension::ProviderAvailabilityAndTrust,
        UnassessedDimension::PhysicalRealization,
    ];

    Ok(HdlTargetFeasibilityReport {
        target_id: capability_assessment.target_id().clone(),
        target_status: capability_assessment.target_status(),
        status,
        capability_assessment,
        unassessed_dimensions,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::semantic::capabilities::{
        CapabilityRequirements,
        EvidenceSubject,
    };
    use crate::hdl::targets::model::{
        HdlTargetDescription,
        TargetDescriptionStatus,
        TargetId,
    };

    fn target() -> HdlTargetDescription {
        HdlTargetDescription::new(
            TargetId::new("test.target").expect("valid target ID"),
            "Test target",
            "test.simulator",
        )
        .expect("valid target description")
    }

    fn subject() -> EvidenceSubject {
        EvidenceSubject::new("test.target.profile")
            .expect("valid evidence subject")
    }

    #[test]
    fn empty_capability_requirements_do_not_prove_feasibility() {
        let target = target();
        let requirements = CapabilityRequirements::new();

        let report = assess_target_feasibility(
            &target,
            &requirements,
            &subject(),
        )
        .expect("empty capability assessment is valid");

        assert_eq!(report.target_id().as_str(), "test.target");
        assert_eq!(
            report.status(),
            FeasibilityStatus::Indeterminate
        );
        assert!(!report.is_feasible());
        assert!(!report.is_infeasible());
        assert!(report.is_indeterminate());
        assert!(report.has_vacuous_capability_assessment());
        assert!(!report.unassessed_dimensions().is_empty());
    }

    #[test]
    fn declared_target_status_is_preserved() {
        let mut target = target();
        target.set_status(TargetDescriptionStatus::Unknown);

        let report = assess_target_feasibility(
            &target,
            &CapabilityRequirements::new(),
            &subject(),
        )
        .expect("target description is valid");

        assert_eq!(
            report.target_status(),
            TargetDescriptionStatus::Unknown
        );
    }

    #[test]
    fn unassessed_dimensions_have_stable_identifiers() {
        assert_eq!(
            UnassessedDimension::ResourceCapacity.as_str(),
            "resource_capacity"
        );
        assert_eq!(
            UnassessedDimension::PhysicalRealization.as_str(),
            "physical_realization"
        );
    }

    #[test]
    fn feasibility_status_predicates_are_consistent() {
        assert!(FeasibilityStatus::Infeasible.is_infeasible());
        assert!(!FeasibilityStatus::Infeasible.is_feasible());

        assert!(FeasibilityStatus::Indeterminate.is_indeterminate());
        assert!(!FeasibilityStatus::Indeterminate.is_feasible());

        assert!(FeasibilityStatus::Feasible.is_feasible());
        assert!(!FeasibilityStatus::Feasible.is_indeterminate());
    }

    #[test]
    fn report_contract_has_a_stable_name_and_version() {
        assert_eq!(
            HDL_TARGET_FEASIBILITY_MODULE_NAME,
            "zamani.hdl.targets.feasibility"
        );
        assert_eq!(HDL_TARGET_FEASIBILITY_SCHEMA_VERSION, 1);
    }
}
