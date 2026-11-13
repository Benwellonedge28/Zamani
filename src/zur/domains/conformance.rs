
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Domain Provider Conformance
//!
//! Path: `src/zur/domains/conformance.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provides a reusable, deterministic assessment of the structural
//! conformance of a ZUR domain provider to the canonical provider,
//! domain-descriptor, and domain-compatibility contracts.
//!
//! This module evaluates:
//!
//! - Provider identity and execution-contract consistency.
//! - Provider execution-contract structural validity.
//! - Domain descriptor structural validity.
//! - Domain identity uniqueness within a provider.
//! - Domain-to-provider interface consistency.
//! - Domain contract-version compatibility.
//!
//! It produces structured findings rather than reducing all results
//! to an unqualified boolean.
//!
//! ## OWNERSHIP
//!
//! This module owns:
//!
//! - Domain-provider structural conformance assessment.
//! - Conformance finding identities and outcomes.
//! - Deterministic provider conformance reports.
//! - Aggregate structural conformance decisions.
//!
//! This module does NOT own:
//!
//! - Domain identity or domain semantics.
//! - Provider identity or provider lifecycle.
//! - Provider registration or discovery.
//! - Provider authentication, authorization, or trust.
//! - Capability definitions.
//! - Capability evidence generation or authentication.
//! - Capability availability.
//! - Resource requirements or resource capacity.
//! - Resource allocation, reservation, or leases.
//! - Feasibility analysis.
//! - Provider selection.
//! - Backend registration or code generation.
//! - Compiler optimization or lowering.
//! - Classical IR, quantum IR, HDLIR, AI/tensor IR, or ZUIR.
//! - Hybrid or distributed execution planning.
//! - Scheduling, routing, or physical execution.
//! - Domain-specific operation test payloads.
//! - Hardware or device conformance.
//!
//! Those responsibilities remain with their canonical authorities.
//!
//! ## CANONICAL AUTHORITIES
//!
//! Domain identity:
//!     frontend::ast::node::domains::domain::Domain
//!
//! Domain descriptors:
//!     zur::domains::descriptor
//!
//! Domain-provider contract:
//!     zur::domains::provider
//!
//! Provider execution contract:
//!     zur::contracts::provider
//!
//! Provider identity and lifecycle:
//!     stdlib::provider
//!
//! Structural domain validation:
//!     zur::domains::validation
//!
//! Domain/provider compatibility:
//!     zur::domains::compatibility
//!
//! Operational errors:
//!     zur::error
//!
//! ## GRAMMAR BOUNDARY
//!
//! This module consumes canonical domain descriptors after source
//! parsing and semantic analysis.
//!
//! It does not parse grammar, interpret source declarations, create
//! domain identities, or define computational-domain semantics.
//!
//! Relevant specifications:
//!
//! - grammar/specification/domains.md
//! - grammar/specification/semantic-model.md
//! - grammar/specification/execution-model.md
//! - grammar/specification/scalability-model.md
//! - grammar/spec/compatibility.md
//! - grammar/spec/resources.md
//!
//! ## IMPORTANT DISTINCTIONS
//!
//! Structural conformance != provider availability.
//! Provider registration != provider initialization.
//! Provider initialization != provider availability.
//! Provider declaration != capability evidence.
//! Capability evidence != authenticated evidence.
//! Capability availability != resource availability.
//! Requirement satisfaction != feasibility.
//! Feasibility != resource reservation.
//! Resource reservation != execution.
//! Submission != successful completion.
//!
//! A passing report from this module establishes only the structural
//! contract properties actually checked by this module.
//!
//! It does not establish that a provider can execute a real workload.
//!
//! ## OPEN-WORLD EXTENSIBILITY
//!
//! Computational domains are represented by the canonical AST Domain
//! type. No closed domain enumeration is introduced here.
//!
//! New computational domains use the same conformance assessment
//! without modifying this module.
//!
//! ## SCALABILITY
//!
//! No arbitrary maximum is imposed on:
//!
//! - Providers.
//! - Domains per provider.
//! - Provider interfaces.
//! - Conformance findings.
//! - Domain identifiers.
//! - Diagnostic lengths.
//!
//! Collections grow according to actual available resources.
//!
//! This module does not promise literal infinite computation or
//! storage. A finite implementation remains subject to actual
//! memory, processing, and platform resources.
//!
//! ## SAFETY
//!
//! Safe Rust only.
//! No unsafe code.
//! No raw pointers.
//! No global mutable state.
//! No filesystem or network access.
//! No hardware discovery.
//! No provider initialization.
//! No resource allocation.
//! No fabricated execution success.
//!
//! ## INTEGRATION
//!
//! Upstream:
//!
//! - zur/domains/provider.rs
//! - zur/domains/descriptor.rs
//! - zur/contracts/provider.rs
//! - stdlib/provider.rs
//!
//! Validation:
//!
//! - zur/domains/validation.rs
//! - zur/domains/compatibility.rs
//!
//! Downstream:
//!
//! - zur/domains/registry.rs
//! - zur/domains/composition.rs
//! - zur/execution/dispatch.rs
//! - zur/execution/executor.rs
//! - provider-specific conformance and integration tests
//!
//! Registry registration must continue to use its own validation
//! authority. This report is not a registration receipt or admission
//! decision.

// ============================================================================
// IMPORTS
// ============================================================================

use core::fmt;

use crate::stdlib::provider::ProviderId;

use crate::zur::domains::compatibility::{
    check_domain_provider_compatibility,
    DomainCompatibilityStatus,
};

use crate::zur::domains::provider::DomainProvider;

use crate::zur::domains::validation::validate_domain_provider;

// ============================================================================
// COMPONENT IDENTIFICATION
// ============================================================================

const COMPONENT: &str = "zur.domains.conformance";

// ============================================================================
// CONFORMANCE OUTCOMES
// ============================================================================

/// Outcome of one domain-provider conformance check.
///
/// These outcomes distinguish successful evidence from failures and
/// checks for which sufficient evidence was not established.
///
/// An inconclusive or unexecuted check never counts as passing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ConformanceOutcome {
    /// The check completed and its required structural condition passed.
    Passed,

    /// The check completed and established a contract violation.
    Failed,

    /// The check could not establish either conformance or failure.
    Inconclusive,

    /// The check was not executed.
    NotRun,
}

impl ConformanceOutcome {
    /// Returns whether this outcome represents established success.
    #[must_use]
    pub const fn is_passed(self) -> bool {
        matches!(self, Self::Passed)
    }

    /// Returns whether this outcome prevents a conformance claim.
    #[must_use]
    pub const fn prevents_conformance(self) -> bool {
        !self.is_passed()
    }
}

impl fmt::Display for ConformanceOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Inconclusive => "inconclusive",
            Self::NotRun => "not_run",
        })
    }
}

// ============================================================================
// CONFORMANCE FINDINGS
// ============================================================================

/// One independently identifiable domain-provider conformance finding.
///
/// A finding records the actual result of one check. It does not
/// independently establish provider availability or execution success.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConformanceFinding {
    /// Stable, namespaced identity of the check.
    id: String,

    /// Optional canonical domain identity associated with the check.
    domain: Option<String>,

    /// Actual outcome of the check.
    outcome: ConformanceOutcome,

    /// Human-readable explanation of the observed result.
    message: String,
}

impl ConformanceFinding {
    fn new(
        id: impl Into<String>,
        domain: Option<String>,
        outcome: ConformanceOutcome,
        message: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            domain,
            outcome,
            message: message.into(),
        }
    }

    /// Returns the stable finding identity.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the associated canonical domain identity, if applicable.
    #[must_use]
    pub fn domain(&self) -> Option<&str> {
        self.domain.as_deref()
    }

    /// Returns the actual check outcome.
    #[must_use]
    pub const fn outcome(&self) -> ConformanceOutcome {
        self.outcome
    }

    /// Returns the diagnostic explanation.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

// ============================================================================
// AGGREGATE REPORT
// ============================================================================

/// Structural conformance report for one domain provider.
///
/// The report contains the outcomes of the checks actually performed.
/// A report does not claim that untested provider operations, hardware,
/// or runtime behavior conform.
///
/// Findings are ordered deterministically:
///
/// 1. Provider structural validation.
/// 2. Domain compatibility checks in canonical domain-identity order.
///
/// A provider with no declared domains cannot receive a passing report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainProviderConformanceReport {
    provider_id: ProviderId,
    declared_domain_count: usize,
    validated_domain_count: Option<usize>,
    findings: Vec<ConformanceFinding>,
}

impl DomainProviderConformanceReport {
    fn new(
        provider_id: ProviderId,
        declared_domain_count: usize,
    ) -> Self {
        Self {
            provider_id,
            declared_domain_count,
            validated_domain_count: None,
            findings: Vec::new(),
        }
    }

    fn push(&mut self, finding: ConformanceFinding) {
        self.findings.push(finding);
    }

    fn set_validated_domain_count(&mut self, count: usize) {
        self.validated_domain_count = Some(count);
    }

    /// Returns the canonical provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Returns the number of domain descriptors declared by the provider.
    #[must_use]
    pub const fn declared_domain_count(&self) -> usize {
        self.declared_domain_count
    }

    /// Returns the number of domains accepted by canonical structural
    /// validation.
    ///
    /// `None` means provider/domain structural validation did not
    /// complete successfully.
    #[must_use]
    pub const fn validated_domain_count(&self) -> Option<usize> {
        self.validated_domain_count
    }

    /// Returns all findings in deterministic report order.
    #[must_use]
    pub fn findings(&self) -> &[ConformanceFinding] {
        &self.findings
    }

    /// Returns the number of findings.
    #[must_use]
    pub fn finding_count(&self) -> usize {
        self.findings.len()
    }

    /// Returns the number of passing findings.
    #[must_use]
    pub fn passed_count(&self) -> usize {
        self.findings
            .iter()
            .filter(|finding| finding.outcome.is_passed())
            .count()
    }

    /// Returns the number of failed findings.
    #[must_use]
    pub fn failed_count(&self) -> usize {
        self.findings
            .iter()
            .filter(|finding| {
                finding.outcome == ConformanceOutcome::Failed
            })
            .count()
    }

    /// Returns the number of inconclusive findings.
    #[must_use]
    pub fn inconclusive_count(&self) -> usize {
        self.findings
            .iter()
            .filter(|finding| {
                finding.outcome == ConformanceOutcome::Inconclusive
            })
            .count()
    }

    /// Returns the number of checks that were not run.
    #[must_use]
    pub fn not_run_count(&self) -> usize {
        self.findings
            .iter()
            .filter(|finding| {
                finding.outcome == ConformanceOutcome::NotRun
            })
            .count()
    }

    /// Returns whether every recorded check passed and at least one
    /// domain was structurally validated.
    ///
    /// This means structural domain-provider conformance only. It does
    /// not establish operational, hardware, or workload conformance.
    #[must_use]
    pub fn is_conformant(&self) -> bool {
        self.declared_domain_count > 0
            && self.validated_domain_count
                == Some(self.declared_domain_count)
            && !self.findings.is_empty()
            && self
                .findings
                .iter()
                .all(|finding| finding.outcome.is_passed())
    }

    /// Returns whether the report contains any non-passing finding.
    #[must_use]
    pub fn has_blocking_findings(&self) -> bool {
        self.findings
            .iter()
            .any(|finding| finding.outcome.prevents_conformance())
    }
}

impl fmt::Display for DomainProviderConformanceReport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "domain provider {}: {} of {} checks passed; \
             {} failed; {} inconclusive; {} not run",
            self.provider_id,
            self.passed_count(),
            self.finding_count(),
            self.failed_count(),
            self.inconclusive_count(),
            self.not_run_count(),
        )
    }
}

// ============================================================================
// PUBLIC CONFORMANCE ASSESSMENT
// ============================================================================

/// Assesses the structural conformance of a domain provider.
///
/// The function invokes the canonical domain-provider validation
/// authority and the canonical domain/provider compatibility authority.
///
/// It does not initialize the provider or invoke its operations.
///
/// The provider must already expose stable descriptors and a stable
/// execution contract, as required by `DomainProvider`.
///
/// # Checks
///
/// The assessment performs:
///
/// 1. Canonical provider/domain structural validation.
/// 2. Domain/provider interface and contract-version compatibility
///    for every declared domain descriptor.
///
/// The compatibility checks are performed independently for each
/// descriptor so that the report identifies the domain responsible
/// for an incompatibility.
///
/// # Important limitations
///
/// A passing result does not establish:
///
/// - provider authenticity or trust;
/// - provider initialization or readiness;
/// - current provider availability;
/// - capability-evidence authenticity or freshness;
/// - current capability availability;
/// - resource availability or reservation;
/// - execution feasibility;
/// - successful operation execution;
/// - hardware or device conformance.
///
/// Those checks must be performed by the relevant authorities and
/// by provider-specific integration tests.
///
/// # Determinism
///
/// Domain findings are sorted by canonical qualified domain identity.
/// Provider identity is taken from the canonical provider descriptor.
///
/// # Scalability
///
/// The function imposes no arbitrary maximum on the number of domains
/// or findings. Processing and storage remain subject to available
/// resources.
///
/// # Arguments
///
/// * `provider` - The provider whose structural contract is assessed.
///
/// # Returns
///
/// A report containing every structural and compatibility finding
/// that could be evaluated.
#[must_use]
pub fn assess_domain_provider(
    provider: &dyn DomainProvider,
) -> DomainProviderConformanceReport {
    let provider_id = provider.descriptor().id().clone();

    let domains = provider.domain_descriptors();

    let mut report = DomainProviderConformanceReport::new(
        provider_id,
        domains.len(),
    );

    // ------------------------------------------------------------------------
    // 1. Canonical structural validation
    // ------------------------------------------------------------------------
    //
    // The validation module remains the authoritative implementation.
    // No local duplicate validation rules are introduced here.
    //
    // No evidence resolver is supplied. Therefore this check validates
    // the provider and descriptors structurally without pretending that
    // external capability evidence has been authenticated or refreshed.

    match validate_domain_provider(
        provider.descriptor(),
        provider.execution_contract(),
        domains,
        None,
    ) {
        Ok(validation) => {
            report.set_validated_domain_count(
                validation.domain_count(),
            );

            report.push(ConformanceFinding::new(
                "provider.structural_validation",
                None,
                ConformanceOutcome::Passed,
                format!(
                    "canonical validation accepted {} domain descriptor(s) \
                     for provider {}",
                    validation.domain_count(),
                    validation.provider_id(),
                ),
            ));
        }

        Err(error) => {
            report.push(ConformanceFinding::new(
                "provider.structural_validation",
                None,
                ConformanceOutcome::Failed,
                format!(
                    "canonical provider/domain validation failed: {error}"
                ),
            ));
        }
    }

    // ------------------------------------------------------------------------
    // 2. Deterministic domain ordering
    // ------------------------------------------------------------------------
    //
    // DomainProvider exposes a stable slice, but the provider's original
    // insertion order is not required to be the canonical report order.
    //
    // Sort references only. The provider's descriptors are not modified.

    let mut ordered_domains = domains.iter().collect::<Vec<_>>();

    ordered_domains.sort_by(|left, right| {
        left.domain()
            .qualified_name()
            .cmp(&right.domain().qualified_name())
    });

    // ------------------------------------------------------------------------
    // 3. Per-domain compatibility checks
    // ------------------------------------------------------------------------
    //
    // The compatibility module owns exact interface identity and
    // accepted domain-contract-version evaluation.
    //
    // Compatibility is not feasibility, resource availability,
    // reservation, provider selection, or execution.

    for domain_descriptor in ordered_domains {
        let domain_identity =
            domain_descriptor.domain().qualified_name().to_string();

        let finding_id = format!(
            "domain.compatibility.{domain_identity}"
        );

        match check_domain_provider_compatibility(
            provider.descriptor(),
            provider.execution_contract(),
            domain_descriptor,
        ) {
            Ok(compatibility) => {
                let outcome = match compatibility.status() {
                    DomainCompatibilityStatus::Compatible => {
                        ConformanceOutcome::Passed
                    }

                    DomainCompatibilityStatus::Incompatible => {
                        ConformanceOutcome::Failed
                    }
                };

                let message = format!(
                    "domain {} is {} with provider {}; \
                     {} compatible interface(s)",
                    domain_identity,
                    compatibility.status(),
                    compatibility.provider_id(),
                    compatibility.compatible_interface_count(),
                );

                report.push(ConformanceFinding::new(
                    finding_id,
                    Some(domain_identity),
                    outcome,
                    message,
                ));
            }

            Err(error) => {
                report.push(ConformanceFinding::new(
                    finding_id,
                    Some(domain_identity.clone()),
                    ConformanceOutcome::Failed,
                    format!(
                        "domain compatibility evaluation failed for \
                         {domain_identity}: {error}"
                    ),
                ));
            }
        }
    }

    report
}

// ============================================================================
// REPORT VALIDATION
// ============================================================================

/// Validates the internal consistency of a conformance report.
///
/// This is a defensive check for consumers that persist, transform,
/// or exchange reports.
///
/// It does not rerun provider validation or compatibility evaluation.
///
/// # Errors
///
/// Returns a structured error if:
///
/// - the report has no findings;
/// - a finding identity is duplicated;
/// - a finding identity is empty;
/// - a finding diagnostic is empty;
/// - a report claims validated domains without a passing structural
///   validation finding;
/// - a report claims conformance while containing a non-passing finding.
pub fn validate_conformance_report(
    report: &DomainProviderConformanceReport,
) -> crate::zur::error::ZurResult<()> {
    use std::collections::BTreeSet;

    use crate::zur::error::ZurError;

    if report.findings.is_empty() {
        return Err(
            ZurError::invalid_data(
                "domain-provider conformance report contains no findings",
            )
            .with_component(COMPONENT)
            .with_operation("validate_conformance_report")
            .with_provider(report.provider_id.to_string()),
        );
    }

    let mut finding_ids = BTreeSet::new();

    for finding in &report.findings {
        if finding.id.trim().is_empty() {
            return Err(
                ZurError::invalid_data(
                    "conformance finding contains an empty identity",
                )
                .with_component(COMPONENT)
                .with_operation("validate_conformance_report")
                .with_provider(report.provider_id.to_string()),
            );
        }

        if !finding_ids.insert(finding.id.as_str()) {
            return Err(
                ZurError::invalid_data(
                    format!(
                        "duplicate conformance finding identity: {}",
                        finding.id,
                    ),
                )
                .with_component(COMPONENT)
                .with_operation("validate_conformance_report")
                .with_provider(report.provider_id.to_string()),
            );
        }

        if finding.message.trim().is_empty() {
            return Err(
                ZurError::invalid_data(
                    format!(
                        "conformance finding {} has no diagnostic message",
                        finding.id,
                    ),
                )
                .with_component(COMPONENT)
                .with_operation("validate_conformance_report")
                .with_provider(report.provider_id.to_string()),
            );
        }
    }

    let structural_validation_passed =
        report.findings.iter().any(|finding| {
            finding.id == "provider.structural_validation"
                && finding.outcome == ConformanceOutcome::Passed
        });

    if report.validated_domain_count.is_some()
        && !structural_validation_passed
    {
        return Err(
            ZurError::invariant_violation(
                "report contains a validated-domain count without \
                 a passing structural-validation finding",
            )
            .with_component(COMPONENT)
            .with_operation("validate_conformance_report")
            .with_provider(report.provider_id.to_string()),
        );
    }

    if report.is_conformant()
        && report
            .findings
            .iter()
            .any(|finding| !finding.outcome.is_passed())
    {
        return Err(
            ZurError::invariant_violation(
                "report claims conformance while containing \
                 a non-passing finding",
            )
            .with_component(COMPONENT)
            .with_operation("validate_conformance_report")
            .with_provider(report.provider_id.to_string()),
        );
    }

    Ok(())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_passed_is_a_passing_outcome() {
        assert!(ConformanceOutcome::Passed.is_passed());

        assert!(!ConformanceOutcome::Failed.is_passed());
        assert!(!ConformanceOutcome::Inconclusive.is_passed());
        assert!(!ConformanceOutcome::NotRun.is_passed());
    }

    #[test]
    fn every_non_passed_outcome_blocks_conformance() {
        assert!(!ConformanceOutcome::Passed.prevents_conformance());

        assert!(ConformanceOutcome::Failed.prevents_conformance());
        assert!(ConformanceOutcome::Inconclusive.prevents_conformance());
        assert!(ConformanceOutcome::NotRun.prevents_conformance());
    }

    #[test]
    fn outcomes_have_stable_display_values() {
        assert_eq!(
            ConformanceOutcome::Passed.to_string(),
            "passed",
        );

        assert_eq!(
            ConformanceOutcome::Failed.to_string(),
            "failed",
        );

        assert_eq!(
            ConformanceOutcome::Inconclusive.to_string(),
            "inconclusive",
        );

        assert_eq!(
            ConformanceOutcome::NotRun.to_string(),
            "not_run",
        );
    }
}
