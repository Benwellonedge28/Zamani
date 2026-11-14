
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Conformance Testing
//!
//! Path: src/zur/testing/conformance.rs
//!
//! Rust edition: 2021
//! Minimum supported Rust: 1.97
//!
//! PURPOSE
//!
//! Provides the aggregate conformance-testing boundary for ZUR providers.
//!
//! The module combines the existing canonical provider-testing and
//! domain-conformance authorities into one reusable testing interface.
//!
//! It supports:
//!
//! - Individual provider structural conformance.
//! - Multi-provider structural conformance.
//! - Deterministic aggregate reports.
//! - Duplicate provider identity detection.
//! - Operational provider probes.
//! - Aggregate conformance decisions.
//! - Conformance-report integrity validation.
//!
//! OWNERSHIP
//!
//! This module owns:
//!
//! - Conformance-suite orchestration.
//! - Multi-provider report aggregation.
//! - Aggregate conformance summaries.
//! - Deterministic aggregate ordering.
//! - Conformance-suite validation boundaries.
//!
//! This module does NOT own:
//!
//! - Provider identity.
//! - Provider lifecycle.
//! - Provider registration.
//! - Provider discovery.
//! - Provider authentication.
//! - Provider authorization.
//! - Domain identity.
//! - Domain semantics.
//! - Domain descriptors.
//! - Provider execution contracts.
//! - Capability definitions.
//! - Capability evidence authentication.
//! - Capability availability.
//! - Resource requirements.
//! - Resource capacity.
//! - Resource allocation.
//! - Resource reservation.
//! - Execution feasibility.
//! - Hardware selection.
//! - Backend registration.
//! - Code generation.
//! - Compiler optimization.
//! - Classical IR.
//! - Quantum IR.
//! - HDLIR.
//! - AI or tensor IR.
//! - ZUIR.
//! - Hybrid execution planning.
//! - Distributed execution planning.
//! - Scheduling.
//! - Routing.
//! - Physical execution.
//!
//! Existing repository authorities retain these responsibilities.
//!
//! CANONICAL INTEGRATION
//!
//! Provider identity:
//!     crate::stdlib::provider
//!
//! Provider execution contracts:
//!     crate::zur::contracts::provider
//!
//! Domain providers:
//!     crate::zur::domains::provider
//!
//! Structural domain conformance:
//!     crate::zur::domains::conformance
//!
//! Provider invocation probes:
//!     crate::zur::testing::provider_harness
//!
//! Operation contexts:
//!     crate::stdlib::context
//!
//! Operational errors:
//!     crate::zur::error
//!
//! GRAMMAR BOUNDARY
//!
//! This module consumes validated provider and domain contracts.
//!
//! It does not parse grammar, construct AST nodes, perform semantic
//! analysis, or interpret source-language declarations.
//!
//! Relevant specifications:
//!
//! - grammar/specification/domains.md
//! - grammar/specification/semantic-model.md
//! - grammar/specification/execution-model.md
//! - grammar/specification/scalability-model.md
//! - grammar/specification/poco-reaf.md
//! - grammar/specification/compatibility.md
//! - grammar/spec/resources.md
//! - grammar/spec/interoperability.md
//! - grammar/validation/semantic-boundaries.md
//!
//! ARCHITECTURAL INVARIANTS
//!
//! Structural conformance != operational conformance.
//! Provider registration != provider initialization.
//! Provider initialization != provider availability.
//! Provider declaration != capability evidence.
//! Capability evidence != authenticated evidence.
//! Capability availability != resource availability.
//! Requirement satisfaction != feasibility.
//! Feasibility != resource reservation.
//! Resource reservation != execution.
//! Execution submission != successful completion.
//!
//! A passing aggregate structural assessment does not establish
//! successful execution on every device or target.
//!
//! SAFETY
//!
//! Safe Rust only.
//! No unsafe code.
//! No raw pointers.
//! No global mutable state.
//! No fabricated success.
//! No implicit provider initialization.
//! No implicit resource allocation.
//! No implicit hardware selection.
//!
//! SCALABILITY
//!
//! No arbitrary maximum is imposed on:
//!
//! - Providers.
//! - Computational domains.
//! - Provider interfaces.
//! - Provider operations.
//! - Provider probes.
//! - Conformance reports.
//! - Conformance findings.
//!
//! Collections grow subject to available implementation and system
//! resources.
//!
//! Literal infinite computation or storage is not promised.
//!
//! DETERMINISM
//!
//! Aggregate provider reports are ordered by canonical provider identity.
//!
//! Provider descriptors and underlying reports are not modified.
//!
//! INTEGRATION
//!
//! Upstream:
//!
//! - zur/domains/provider.rs
//! - zur/domains/conformance.rs
//! - zur/testing/provider_harness.rs
//! - zur/contracts/provider.rs
//! - stdlib/provider.rs
//! - stdlib/context.rs
//!
//! Downstream:
//!
//! - zur/domains/registry.rs
//! - zur/domains/composition.rs
//! - zur/execution/dispatch.rs
//! - zur/execution/executor.rs
//! - provider integration tests
//! - CI conformance suites

use std::collections::BTreeSet;

use crate::stdlib::context::OperationContext;

use crate::zur::domains::conformance::{
    assess_domain_provider,
    validate_conformance_report,
    DomainProviderConformanceReport,
};

use crate::zur::domains::provider::DomainProvider;

use crate::zur::error::{
    ZurError,
    ZurResult,
};

use crate::zur::testing::provider_harness::{
    ProviderHarness,
    ProviderProbe,
    ProviderProbeReport,
};

// ============================================================================
// COMPONENT IDENTIFICATION
// ============================================================================

const COMPONENT: &str = "zur.testing.conformance";

// ============================================================================
// AGGREGATE REPORT
// ============================================================================

/// Aggregate structural conformance report for multiple providers.
///
/// Each individual report is produced by the canonical
/// `zur::domains::conformance` implementation.
///
/// This type does not duplicate domain-level conformance rules.
///
/// Reports are ordered deterministically by provider identity.
///
/// An empty report collection is not considered conformant.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ConformanceSuiteReport {

    providers: Vec<DomainProviderConformanceReport>,
}

impl ConformanceSuiteReport {

    /// Creates an empty conformance-suite report.
    ///
    /// An empty report is not a successful conformance assessment.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns all individual provider reports.
    ///
    /// Reports are ordered by canonical provider identity.
    #[must_use]
    pub fn providers(
        &self,
    ) -> &[DomainProviderConformanceReport] {
        &self.providers
    }

    /// Returns the number of assessed providers.
    #[must_use]
    pub fn provider_count(&self) -> usize {
        self.providers.len()
    }

    /// Returns the number of structurally conformant providers.
    #[must_use]
    pub fn conformant_provider_count(&self) -> usize {
        self.providers
            .iter()
            .filter(|report| report.is_conformant())
            .count()
    }

    /// Returns the number of providers that did not establish
    /// structural conformance.
    #[must_use]
    pub fn non_conformant_provider_count(&self) -> usize {
        self.providers
            .iter()
            .filter(|report| !report.is_conformant())
            .count()
    }

    /// Returns the total number of structural findings.
    #[must_use]
    pub fn finding_count(&self) -> usize {
        self.providers
            .iter()
            .map(|report| report.finding_count())
            .sum()
    }

    /// Returns the total number of passing findings.
    #[must_use]
    pub fn passed_count(&self) -> usize {
        self.providers
            .iter()
            .map(|report| report.passed_count())
            .sum()
    }

    /// Returns the total number of failed findings.
    #[must_use]
    pub fn failed_count(&self) -> usize {
        self.providers
            .iter()
            .map(|report| report.failed_count())
            .sum()
    }

    /// Returns the total number of inconclusive findings.
    #[must_use]
    pub fn inconclusive_count(&self) -> usize {
        self.providers
            .iter()
            .map(|report| report.inconclusive_count())
            .sum()
    }

    /// Returns the total number of checks that were not executed.
    #[must_use]
    pub fn not_run_count(&self) -> usize {
        self.providers
            .iter()
            .map(|report| report.not_run_count())
            .sum()
    }

    /// Returns whether every assessed provider established
    /// structural conformance.
    ///
    /// An empty collection never establishes conformance.
    ///
    /// This result does not establish hardware availability,
    /// resource feasibility, reservation, or successful execution.
    #[must_use]
    pub fn is_conformant(&self) -> bool {

        !self.providers.is_empty()
            && self.providers
                .iter()
                .all(|report| report.is_conformant())
    }

    /// Returns whether any provider failed to establish
    /// structural conformance.
    #[must_use]
    pub fn has_non_conformant_providers(&self) -> bool {

        self.providers
            .iter()
            .any(|report| !report.is_conformant())
    }

    /// Returns whether the aggregate contains any non-passing finding.
    #[must_use]
    pub fn has_blocking_findings(&self) -> bool {

        self.providers
            .iter()
            .any(|report| report.has_blocking_findings())
    }

    /// Validates every underlying structural conformance report.
    ///
    /// This operation checks report integrity.
    ///
    /// It does not rerun provider validation or provider operations.
    ///
    /// # Errors
    ///
    /// Returns the canonical structured error when an individual
    /// report violates its internal consistency requirements.
    pub fn validate(&self) -> ZurResult<()> {

        for report in &self.providers {

            validate_conformance_report(report)
                .map_err(|error| {
                    error
                        .with_component(COMPONENT)
                        .with_operation("validate_aggregate_report")
                        .with_provider(
                            report.provider_id().to_string(),
                        )
                })?;
        }

        Ok(())
    }
}

// ============================================================================
// CONFORMANCE SUITE
// ============================================================================

/// Reusable ZUR provider conformance suite.
///
/// This is an orchestration facade over existing canonical
/// conformance and provider-testing implementations.
///
/// It does not own a provider registry.
///
/// Providers are explicitly supplied by the caller.
///
/// It does not initialize, shut down, authenticate, select,
/// or allocate resources for providers.
#[derive(Debug, Default, Clone, Copy)]
pub struct ConformanceSuite {

    harness: ProviderHarness,
}

impl ConformanceSuite {

    /// Creates a conformance suite.
    #[must_use]
    pub const fn new() -> Self {

        Self {
            harness: ProviderHarness::new(),
        }
    }

    // ------------------------------------------------------------------------
    // INDIVIDUAL STRUCTURAL ASSESSMENT
    // ------------------------------------------------------------------------

    /// Assesses one provider's structural domain conformance.
    ///
    /// The canonical domain conformance implementation remains
    /// authoritative.
    ///
    /// The returned report distinguishes passing, failing,
    /// inconclusive, and unexecuted findings.
    ///
    /// A successful function return does not necessarily mean
    /// that the provider itself is conformant.
    ///
    /// Inspect `DomainProviderConformanceReport::is_conformant`
    /// to determine the actual assessment result.
    ///
    /// # Errors
    ///
    /// Returns an error only when the generated report fails
    /// canonical report-integrity validation.
    ///
    /// Structural incompatibility is represented in the report.
    #[must_use = "the report contains the actual conformance findings"]
    pub fn assess_provider(
        &self,
        provider: &dyn DomainProvider,
    ) -> ZurResult<DomainProviderConformanceReport> {

        let report = self.harness.assess_structure(provider);

        validate_conformance_report(&report)
            .map_err(|error| {
                error
                    .with_component(COMPONENT)
                    .with_operation("assess_provider")
                    .with_provider(
                        report.provider_id().to_string(),
                    )
            })?;

        Ok(report)
    }

    // ------------------------------------------------------------------------
    // MULTI-PROVIDER STRUCTURAL ASSESSMENT
    // ------------------------------------------------------------------------

    /// Assesses multiple explicitly supplied providers.
    ///
    /// Every provider is assessed using the canonical
    /// domain-provider conformance implementation.
    ///
    /// The resulting reports are sorted by canonical provider identity.
    ///
    /// Duplicate provider identities within the supplied assessment
    /// are rejected because an aggregate must not ambiguously represent
    /// two implementations under the same canonical identity.
    ///
    /// The input collection is not modified.
    ///
    /// This method does not register providers or initialize them.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - the same canonical provider identity occurs more than once;
    /// - an individual generated report fails integrity validation.
    ///
    /// Structural incompatibilities are preserved as report findings.
    pub fn assess_providers<'a, I>(
        &self,
        providers: I,
    ) -> ZurResult<ConformanceSuiteReport>
    where
        I: IntoIterator<Item = &'a dyn DomainProvider>,
    {

        let mut reports =
            Vec::<DomainProviderConformanceReport>::new();

        let mut identities =
            BTreeSet::<String>::new();

        for provider in providers {

            let report = self.assess_provider(provider)?;

            let identity =
                report.provider_id().to_string();

            if !identities.insert(identity.clone()) {

                return Err(
                    ZurError::invalid_input(
                        format!(
                            "duplicate provider identity in \
                             conformance assessment: {identity}"
                        ),
                    )
                    .with_component(COMPONENT)
                    .with_operation("assess_providers")
                    .with_provider(identity),
                );
            }

            reports.push(report);
        }

        // Deterministic report ordering.
        //
        // The canonical ProviderId remains authoritative.
        // Its display representation is used only as the ordering key.

        reports.sort_by(|left, right| {
            left.provider_id()
                .to_string()
                .cmp(&right.provider_id().to_string())
        });

        let aggregate = ConformanceSuiteReport {
            providers: reports,
        };

        aggregate.validate()?;

        Ok(aggregate)
    }

    // ------------------------------------------------------------------------
    // OPERATIONAL PROVIDER CONFORMANCE
    // ------------------------------------------------------------------------

    /// Executes an explicitly supplied operational provider probe.
    ///
    /// Delegates execution to the canonical ProviderHarness.
    ///
    /// The provider harness:
    ///
    /// 1. Assesses structural provider conformance.
    /// 2. Validates the probe identity.
    /// 3. Verifies the declared operation.
    /// 4. Checks cancellation.
    /// 5. Invokes the actual provider operation.
    /// 6. Validates the actual response.
    /// 7. Records the observed result.
    ///
    /// The operation context is supplied by the caller.
    ///
    /// The caller is responsible for selecting a safe test operation
    /// and providing an appropriate asynchronous execution environment.
    ///
    /// No provider is initialized automatically.
    ///
    /// No resource is implicitly allocated or reserved.
    ///
    /// A successful probe establishes only that the specified
    /// operation satisfied the supplied assertions under the
    /// observed conditions.
    ///
    /// It does not prove universal provider correctness.
    pub async fn run_provider_probe(
        &self,
        provider: &dyn DomainProvider,
        probe: &dyn ProviderProbe,
        context: &OperationContext,
    ) -> ProviderProbeReport {

        self.harness
            .run_probe(provider, probe, context)
            .await
    }

    // ------------------------------------------------------------------------
    // STRUCTURAL ASSESSMENT AND REPORT INTEGRITY
    // ------------------------------------------------------------------------

    /// Assesses the structural conformance of one provider and
    /// independently verifies the resulting report.
    ///
    /// This is a convenience entry point for CI and integration tests.
    ///
    /// The result remains a structural assessment only.
    ///
    /// # Errors
    ///
    /// Returns an error if report integrity cannot be established.
    pub fn verify_provider(
        &self,
        provider: &dyn DomainProvider,
    ) -> ZurResult<DomainProviderConformanceReport> {

        let report = assess_domain_provider(provider);

        validate_conformance_report(&report)
            .map_err(|error| {
                error
                    .with_component(COMPONENT)
                    .with_operation("verify_provider")
                    .with_provider(
                        report.provider_id().to_string(),
                    )
            })?;

        Ok(report)
    }
}

// ============================================================================
// CONVENIENCE API
// ============================================================================

/// Assesses one provider using the canonical ZUR conformance authorities.
///
/// This function is useful when a caller does not need to retain
/// a ConformanceSuite instance.
///
/// # Errors
///
/// Returns an error if report integrity validation fails.
pub fn assess_provider_conformance(
    provider: &dyn DomainProvider,
) -> ZurResult<DomainProviderConformanceReport> {

    ConformanceSuite::new()
        .assess_provider(provider)
}

/// Assesses multiple explicitly supplied providers.
///
/// Results are ordered by canonical provider identity.
///
/// An empty input produces an empty, non-conformant report.
///
/// # Errors
///
/// Returns an error for duplicate provider identities or
/// invalid generated reports.
pub fn assess_provider_conformances<'a, I>(
    providers: I,
) -> ZurResult<ConformanceSuiteReport>
where
    I: IntoIterator<Item = &'a dyn DomainProvider>,
{

    ConformanceSuite::new()
        .assess_providers(providers)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn empty_suite_is_not_conformant() {

        let report = ConformanceSuiteReport::new();

        assert_eq!(report.provider_count(), 0);

        assert_eq!(
            report.conformant_provider_count(),
            0,
        );

        assert_eq!(
            report.non_conformant_provider_count(),
            0,
        );

        assert!(!report.is_conformant());
    }

    #[test]
    fn empty_suite_has_no_findings() {

        let report = ConformanceSuiteReport::new();

        assert_eq!(report.finding_count(), 0);
        assert_eq!(report.passed_count(), 0);
        assert_eq!(report.failed_count(), 0);
        assert_eq!(report.inconclusive_count(), 0);
        assert_eq!(report.not_run_count(), 0);
    }

    #[test]
    fn empty_suite_report_is_internally_valid() {

        let report = ConformanceSuiteReport::new();

        assert!(report.validate().is_ok());
    }

    #[test]
    fn empty_provider_assessment_does_not_claim_success() {

        let suite = ConformanceSuite::new();

        let report = suite
            .assess_providers(
                std::iter::empty::<&dyn DomainProvider>(),
            )
            .expect("empty assessment should be valid");

        assert_eq!(report.provider_count(), 0);

        assert!(!report.is_conformant());
    }

    #[test]
    fn suite_construction_is_independent() {

        let first = ConformanceSuite::new();
        let second = ConformanceSuite::new();

        let first_report = ConformanceSuiteReport::new();
        let second_report = ConformanceSuiteReport::new();

        assert_eq!(
            first_report.provider_count(),
            second_report.provider_count(),
        );

        let _ = (first, second);
    }

    #[test]
    fn empty_report_does_not_report_blocking_findings() {

        let report = ConformanceSuiteReport::new();

        assert!(!report.has_blocking_findings());

        assert!(!report.has_non_conformant_providers());
    }
}
