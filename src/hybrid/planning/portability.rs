#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Hybrid Computing — Portability Assessment
//!
//! Path: `src/hybrid/planning/portability.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Assess whether a validated hybrid computation is portable, conditionally
//! realizable, unsupported, infeasible, indeterminate, or invalid based on
//! existing semantic-validation results, domain-interface declarations, and
//! explicitly supplied resource/capability feasibility evidence.
//!
//! This module reports conclusions from authoritative upstream assessments.
//! It does not independently decide semantic correctness, discover targets,
//! select hardware, allocate resources, lower IR, schedule execution, or run
//! a program.
//!
//! ## Canonical ownership
//!
//! - `domain/interface.rs` owns domain-interface and portability declarations.
//! - `planning/validation.rs` owns plan-validation evidence.
//! - `resources/feasibility.rs` owns resource/capability feasibility decisions.
//! - `config.rs` owns operational budgets.
//! - `errors.rs` owns hybrid errors and machine-readable error codes.
//! - Canonical semantic analysis owns types, effects, requirements and meaning.
//! - Canonical domain IRs own domain-operation semantics.
//! - Compiler, backend and runtime layers own target realization and execution.
//!
//! ## Portability contract
//!
//! A portable source program does not imply that every target can execute it.
//! A `Portable` result requires complete semantic verification, a positive
//! feasibility assessment, and target-independent declarations for every
//! supplied participating interface.
//!
//! `ConditionallyRealizable` means that the computation may be realized only
//! under declared capability, resource, or implementation conditions.
//!
//! `Unsupported` means that the supplied feasibility evidence explicitly
//! identifies an unsupported required capability.
//!
//! `Infeasible` means that the supplied feasibility evidence establishes a
//! binding incompatibility other than an explicitly unsupported capability.
//!
//! `Indeterminate` means that the available evidence cannot establish a
//! definitive conclusion. Missing evidence is never treated as support.
//!
//! `Invalid` means that the supplied feasibility assessment itself is marked
//! invalid. Malformed interface declarations are returned as errors.
//!
//! These statuses describe the supplied assessment scope. They do not prove
//! that evidence is authentic, that a target remains available, or that a
//! later backend/runtime implementation conforms.
//!
//! ## Scalability
//!
//! No fixed number of domains, interfaces, capabilities, graph nodes, devices,
//! or resources is imposed. Work is proportional to the supplied interface
//! and finding inventories, subject to actual process resources.
//!
//! The caller may configure an explicit `PlanningSteps` budget. Budget
//! exhaustion is an operational failure, not a portability conclusion and
//! never changes program semantics.
//!
//! Literal infinite input or execution is not promised by a finite machine.
//!
//! ## Integration
//!
//! Call `HybridPortabilityAssessor::assess` after plan validation and, when a
//! concrete capability/resource scope is being evaluated, after feasibility
//! assessment. The caller supplies the canonical interfaces participating in
//! the assessed computation and the relevant feasibility report.
//!
//! This module deliberately does not construct a feasibility report or infer
//! missing evidence. The compiler pipeline remains responsible for assembling
//! canonical inputs and passing the report to downstream realization layers.

use crate::hybrid::config::{BudgetKind, HybridConfig};
use crate::hybrid::domain::interface::{
    DomainInterface,
    PortabilityLevel,
};
use crate::hybrid::errors::{HybridError, HybridErrorCode};
use crate::hybrid::planning::validation::PlanValidationReport;
use crate::hybrid::resources::feasibility::{
    FeasibilityFindingKind,
    FeasibilityStatus,
    HybridFeasibilityReport,
};

/// Stable identity for the portability-assessment contract.
pub const HYBRID_PORTABILITY_MODULE_NAME: &str =
    "zamani.hybrid.planning.portability";

/// Version of this in-memory report contract.
///
/// This is not the Zamani language version, the domain-interface version,
/// or an interchange-format version.
pub const HYBRID_PORTABILITY_SCHEMA_VERSION: u16 = 1;

/// Overall portability conclusion for the supplied assessment scope.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum PortabilityStatus {
    /// Semantic verification and supplied feasibility evidence establish
    /// feasibility, and every participating interface declares itself
    /// target-independent.
    Portable,

    /// Realization depends on capabilities, resources, implementation
    /// mappings, or other explicit conditions.
    ConditionallyRealizable,

    /// The supplied evidence explicitly identifies an unsupported required
    /// capability.
    Unsupported,

    /// The supplied evidence establishes a binding incompatibility that is
    /// not classified as an unsupported required capability.
    Infeasible,

    /// Available evidence is insufficient to establish a definitive result.
    Indeterminate,

    /// The supplied feasibility assessment is invalid.
    Invalid,
}

/// Machine-readable reason for a portability finding.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum PortabilityFindingKind {
    /// The plan has not received complete semantic verification.
    SemanticVerificationIncomplete,

    /// No resource/capability feasibility assessment was supplied.
    FeasibilityEvidenceMissing,

    /// The supplied feasibility assessment is indeterminate.
    FeasibilityIndeterminate,

    /// The supplied feasibility assessment is invalid.
    FeasibilityInvalid,

    /// A required capability is explicitly unsupported.
    RequiredCapabilityUnsupported,

    /// A binding resource or constraint condition is violated.
    BindingConstraintUnsatisfied,

    /// At least one interface declares capability-conditional portability.
    CapabilityConditionalInterface,

    /// At least one interface declares implementation-specific portability.
    ImplementationSpecificInterface,

    /// The supplied evidence establishes feasibility, but the interfaces
    /// are not all declared target-independent.
    ConditionalRealizationRequired,

    /// All conditions for a portable result have been established.
    PortabilityEstablished,
}

/// One structured explanation for a portability conclusion.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PortabilityFinding {
    kind: PortabilityFindingKind,
    subject: Option<String>,
    message: String,
}

impl PortabilityFinding {
    fn new(
        kind: PortabilityFindingKind,
        subject: Option<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            subject,
            message: message.into(),
        }
    }

    /// Return the finding's machine-readable category.
    #[must_use]
    pub const fn kind(&self) -> PortabilityFindingKind {
        self.kind
    }

    /// Return the relevant domain, capability, or evidence subject, if any.
    #[must_use]
    pub fn subject(&self) -> Option<&str> {
        self.subject.as_deref()
    }

    /// Return the human-readable explanation.
    ///
    /// Programmatic decisions should use the finding kind and report status,
    /// not parse this message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Result of assessing portability for one supplied hybrid-computation scope.
///
/// This is an assessment record, not an executable plan and not a target
/// selection. The private fields ensure that ordinary callers obtain reports
/// through the assessor rather than assembling arbitrary status combinations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PortabilityReport {
    status: PortabilityStatus,
    graph_id: crate::hybrid::ids::HybridGraphId,
    interface_count: u128,
    semantic_verification_complete: bool,
    feasibility_status: Option<FeasibilityStatus>,
    findings: Vec<PortabilityFinding>,
}

impl PortabilityReport {
    /// Return the overall portability conclusion.
    #[must_use]
    pub const fn status(&self) -> PortabilityStatus {
        self.status
    }

    /// Return the graph identity covered by the plan-validation report.
    #[must_use]
    pub const fn graph_id(&self) -> crate::hybrid::ids::HybridGraphId {
        self.graph_id
    }

    /// Return the number of participating interfaces assessed.
    #[must_use]
    pub const fn interface_count(&self) -> u128 {
        self.interface_count
    }

    /// Return whether the plan has complete semantic verification.
    #[must_use]
    pub const fn semantic_verification_complete(&self) -> bool {
        self.semantic_verification_complete
    }

    /// Return the feasibility status supplied to this assessment, if present.
    #[must_use]
    pub const fn feasibility_status(&self) -> Option<FeasibilityStatus> {
        self.feasibility_status
    }

    /// Return findings in deterministic assessment order.
    #[must_use]
    pub fn findings(&self) -> &[PortabilityFinding] {
        &self.findings
    }

    /// Return whether the supplied evidence establishes target-independent
    /// portability for this assessment scope.
    #[must_use]
    pub const fn is_portable(&self) -> bool {
        matches!(self.status, PortabilityStatus::Portable)
    }

    /// Return whether realization requires additional declared conditions.
    #[must_use]
    pub const fn is_conditionally_realizable(&self) -> bool {
        matches!(
            self.status,
            PortabilityStatus::ConditionallyRealizable
        )
    }

    /// Return whether the supplied evidence establishes infeasibility.
    #[must_use]
    pub const fn is_infeasible(&self) -> bool {
        matches!(
            self.status,
            PortabilityStatus::Infeasible | PortabilityStatus::Unsupported
        )
    }

    /// Return whether additional evidence or correction is required.
    #[must_use]
    pub const fn needs_more_evidence(&self) -> bool {
        matches!(
            self.status,
            PortabilityStatus::Indeterminate | PortabilityStatus::Invalid
        )
    }
}

/// Stateless portability assessor.
///
/// All inputs are explicitly supplied. The assessor performs no I/O,
/// hardware discovery, target selection, allocation, or runtime actions.
#[derive(Clone, Copy, Debug, Default)]
pub struct HybridPortabilityAssessor;

impl HybridPortabilityAssessor {
    /// Construct a stateless portability assessor.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Assess portability using canonical plan-validation evidence, domain
    /// interfaces, and an optional resource/capability feasibility report.
    ///
    /// # Arguments
    ///
    /// - `validation`: successful validation evidence for the abstract plan.
    /// - `interfaces`: canonical interfaces participating in the assessed
    ///   computation. Their order is preserved in diagnostic production.
    /// - `feasibility`: the result of assessing the relevant resource and
    ///   capability scope. `None` means that feasibility has not been
    ///   established.
    /// - `config`: explicit operational policy, including an optional
    ///   planning-work budget.
    ///
    /// # Errors
    ///
    /// Returns a structured hybrid error if an interface is invalid, work
    /// accounting overflows, or the configured planning-work budget is
    /// exhausted.
    ///
    /// A missing feasibility report, incomplete semantic verification, or
    /// indeterminate feasibility is represented in the returned report;
    /// none is silently upgraded to success.
    pub fn assess(
        &self,
        validation: &PlanValidationReport,
        interfaces: &[DomainInterface],
        feasibility: Option<&HybridFeasibilityReport>,
        config: &HybridConfig,
    ) -> Result<PortabilityReport, HybridError> {
        let interface_count = interfaces.len() as u128;
        let feasibility_findings = feasibility.map_or(0_u128, |report| {
            report.findings().len() as u128
        });

        // Every interface and every supplied feasibility finding is inspected
        // at most once by this assessor. Checked arithmetic prevents a
        // wrapped work estimate from bypassing an explicit budget.
        let estimated_work = interface_count
            .checked_add(feasibility_findings)
            .and_then(|work| work.checked_add(1))
            .ok_or_else(|| {
                HybridError::new(
                    HybridErrorCode::InvalidRepresentation,
                    "portability assessment work accounting overflowed",
                )
                .in_component("hybrid portability assessment")
            })?;

        config
            .check_budget(BudgetKind::PlanningSteps, estimated_work)
            .map_err(|error| error.in_component("hybrid portability assessment"))?;

        for interface in interfaces {
            interface
                .validate()
                .map_err(|error| error.in_component("hybrid portability assessment"))?;
        }

        let semantic_complete = validation.is_fully_semantically_verified();
        let mut findings = Vec::new();
        let mut has_conditional_interface = false;
        let mut has_implementation_specific_interface = false;

        if !semantic_complete {
            findings.push(PortabilityFinding::new(
                PortabilityFindingKind::SemanticVerificationIncomplete,
                None,
                "the hybrid plan has not established complete semantic verification",
            ));
        }

        for interface in interfaces {
            match interface.portability().level() {
                PortabilityLevel::TargetIndependent => {}
                PortabilityLevel::CapabilityConditional => {
                    has_conditional_interface = true;
                    findings.push(PortabilityFinding::new(
                        PortabilityFindingKind::CapabilityConditionalInterface,
                        Some(interface.domain().as_str().to_owned()),
                        "the domain interface requires capability or resource conditions for realization",
                    ));
                }
                PortabilityLevel::ImplementationSpecific => {
                    has_implementation_specific_interface = true;
                    findings.push(PortabilityFinding::new(
                        PortabilityFindingKind::ImplementationSpecificInterface,
                        Some(interface.domain().as_str().to_owned()),
                        "the domain interface declares an implementation-specific contract; an appropriate implementation mapping must preserve semantics",
                    ));
                }
            }
        }

        let feasibility_status = feasibility.map(HybridFeasibilityReport::status);

        let status = match feasibility {
            Some(report) => match report.status() {
                FeasibilityStatus::Invalid => {
                    findings.push(PortabilityFinding::new(
                        PortabilityFindingKind::FeasibilityInvalid,
                        Some(report.subject().to_owned()),
                        "the supplied feasibility evidence is invalid and cannot establish portability",
                    ));
                    PortabilityStatus::Invalid
                }

                FeasibilityStatus::Infeasible => {
                    // Unsupported required capabilities are more specific
                    // than general resource/constraint infeasibility. Keep
                    // that distinction visible to callers.
                    if report.findings().iter().any(|finding| {
                        finding.kind()
                            == FeasibilityFindingKind::RequiredCapabilityUnsupported
                    }) {
                        for finding in report.findings().iter().filter(|finding| {
                            finding.kind()
                                == FeasibilityFindingKind::RequiredCapabilityUnsupported
                        }) {
                            findings.push(PortabilityFinding::new(
                                PortabilityFindingKind::RequiredCapabilityUnsupported,
                                finding.subject().map(str::to_owned),
                                finding.message(),
                            ));
                        }

                        PortabilityStatus::Unsupported
                    } else {
                        for finding in report.findings().iter().filter(|finding| {
                            finding.kind()
                                == FeasibilityFindingKind::BindingResourceViolation
                        }) {
                            findings.push(PortabilityFinding::new(
                                PortabilityFindingKind::BindingConstraintUnsatisfied,
                                finding.subject().map(str::to_owned),
                                finding.message(),
                            ));
                        }

                        PortabilityStatus::Infeasible
                    }
                }

                FeasibilityStatus::Indeterminate => {
                    findings.push(PortabilityFinding::new(
                        PortabilityFindingKind::FeasibilityIndeterminate,
                        Some(report.subject().to_owned()),
                        "available resource/capability evidence is insufficient to establish feasibility",
                    ));
                    PortabilityStatus::Indeterminate
                }

                FeasibilityStatus::Feasible => {
                    if !semantic_complete {
                        PortabilityStatus::Indeterminate
                    } else if has_conditional_interface
                        || has_implementation_specific_interface
                    {
                        findings.push(PortabilityFinding::new(
                            PortabilityFindingKind::ConditionalRealizationRequired,
                            None,
                            "feasibility is established for the supplied evidence scope, but at least one interface is conditional or implementation-specific",
                        ));
                        PortabilityStatus::ConditionallyRealizable
                    } else {
                        findings.push(PortabilityFinding::new(
                            PortabilityFindingKind::PortabilityEstablished,
                            None,
                            "semantic verification, supplied feasibility evidence, and target-independent interface declarations establish portability for this assessment scope",
                        ));
                        PortabilityStatus::Portable
                    }
                }
            },

            None => {
                findings.push(PortabilityFinding::new(
                    PortabilityFindingKind::FeasibilityEvidenceMissing,
                    None,
                    "no resource/capability feasibility report was supplied; target-independent declarations alone do not prove realizability",
                ));
                PortabilityStatus::Indeterminate
            }
        };

        Ok(PortabilityReport {
            status,
            graph_id: validation.graph_id,
            interface_count,
            semantic_verification_complete: semantic_complete,
            feasibility_status,
            findings,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_helpers_distinguish_portable_from_unproven() {
        let portable = PortabilityStatus::Portable;
        let conditional = PortabilityStatus::ConditionallyRealizable;
        let unsupported = PortabilityStatus::Unsupported;
        let infeasible = PortabilityStatus::Infeasible;
        let indeterminate = PortabilityStatus::Indeterminate;
        let invalid = PortabilityStatus::Invalid;

        assert_ne!(portable, conditional);
        assert_ne!(unsupported, infeasible);
        assert_ne!(indeterminate, portable);
        assert_ne!(invalid, portable);
    }

    #[test]
    fn finding_kind_is_machine_readable() {
        let finding = PortabilityFinding::new(
            PortabilityFindingKind::FeasibilityEvidenceMissing,
            None,
            "feasibility has not been established",
        );

        assert_eq!(
            finding.kind(),
            PortabilityFindingKind::FeasibilityEvidenceMissing
        );
        assert_eq!(finding.subject(), None);
        assert!(!finding.message().is_empty());
    }
}