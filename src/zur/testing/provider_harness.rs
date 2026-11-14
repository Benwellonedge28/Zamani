
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Provider Integration Harness
//!
//! Path: src/zur/testing/provider_harness.rs
//!
//! Rust edition: 2021
//! Minimum supported Rust: 1.97
//!
//! PURPOSE
//!
//! Provides a reusable provider integration harness for independently
//! implemented Zamani Universal Runtime providers.
//!
//! The harness combines:
//!
//! 1. Canonical structural domain-provider conformance.
//! 2. Explicitly requested operational invocation probes.
//! 3. Actual response validation.
//! 4. Cooperative cancellation.
//! 5. Structured, deterministic test reporting.
//!
//! The harness is designed to support providers implementing any
//! computational domain without introducing a closed domain inventory.
//!
//! OWNERSHIP
//!
//! This module owns:
//!
//! - Provider integration probe contracts.
//! - Invocation-probe execution.
//! - Invocation-probe result classification.
//! - Provider integration test reports.
//! - Probe identity validation.
//!
//! This module does NOT own:
//!
//! - Provider identity.
//! - Provider descriptors.
//! - Provider lifecycle.
//! - Provider registration.
//! - Provider discovery.
//! - Provider authentication.
//! - Provider authorization.
//! - Domain identity.
//! - Domain semantics.
//! - Domain descriptors.
//! - Structural domain-provider conformance rules.
//! - Capability definitions.
//! - Capability evidence authentication.
//! - Capability availability.
//! - Resource requirements.
//! - Resource capacity.
//! - Resource allocation.
//! - Resource reservation.
//! - Feasibility.
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
//! - Device drivers.
//! - Hardware abstraction.
//! - Production execution orchestration.
//!
//! Existing repository authorities retain these responsibilities.
//!
//! CANONICAL INTEGRATION
//!
//! Provider identity and lifecycle:
//!     crate::stdlib::provider
//!
//! Provider execution contract:
//!     crate::zur::contracts::provider
//!
//! Domain provider:
//!     crate::zur::domains::provider
//!
//! Domain structural conformance:
//!     crate::zur::domains::conformance
//!
//! Operation context and cancellation:
//!     crate::stdlib::context
//!
//! Operational errors:
//!     crate::zur::error
//!
//! Execution results:
//!     crate::zur::execution::result
//!
//! GRAMMAR BOUNDARY
//!
//! This module does not parse source code or grammar productions.
//!
//! It tests implementations of already-established provider contracts.
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
//! - grammar/spec/semantics.md
//! - grammar/validation/semantic-boundaries.md
//!
//! ARCHITECTURAL INVARIANTS
//!
//! Structural conformance != operational conformance.
//! Provider registration != provider initialization.
//! Provider initialization != provider availability.
//! Provider availability != capability availability.
//! Capability availability != resource reservation.
//! Feasibility != resource reservation.
//! Resource reservation != execution.
//! Submission != successful completion.
//! Successful invocation != universal provider correctness.
//! A passing probe != proof of untested operations.
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
//! No implicit provider selection.
//! No arbitrary hardware ceilings.
//! No arbitrary computational-domain ceilings.
//!
//! SCALABILITY
//!
//! The harness imposes no fixed maximum on:
//!
//! - Providers.
//! - Computational domains.
//! - Provider interfaces.
//! - Operations.
//! - Probe definitions.
//! - Test findings.
//! - Devices.
//! - Execution sessions.
//! - Resource quantities.
//!
//! Collections grow subject to actual allocation capacity and available
//! system resources.
//!
//! Literal infinite physical computation or storage is not promised.
//!
//! INTEGRATION
//!
//! Upstream:
//!
//! - zur/contracts/provider.rs
//! - zur/domains/provider.rs
//! - zur/domains/descriptor.rs
//! - zur/domains/conformance.rs
//! - stdlib/provider.rs
//! - stdlib/context.rs
//! - zur/error.rs
//!
//! Downstream:
//!
//! - zur/domains/conformance.rs
//! - zur/domains/registry.rs
//! - zur/domains/discovery.rs
//! - zur/domains/validation.rs
//! - zur/domains/compatibility.rs
//! - zur/domains/composition.rs
//! - zur/execution/dispatch.rs
//! - zur/execution/executor.rs
//! - zur/testing/conformance.rs
//! - CI and integration-test suites

use core::fmt;

use std::collections::BTreeSet;

use crate::stdlib::context::OperationContext;

use crate::stdlib::provider::{
    ProviderId,
    ProviderOperationId,
};

use crate::zur::contracts::provider::{
    ProviderOperationRequest,
    ProviderOperationResponse,
    ProviderPayload,
};

use crate::zur::domains::conformance::{
    assess_domain_provider,
    DomainProviderConformanceReport,
};

use crate::zur::domains::provider::DomainProvider;

use crate::zur::error::{
    ZurError,
    ZurResult,
};

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

const COMPONENT: &str = "zur.testing.provider_harness";

// ============================================================================
// PROBE IDENTITY
// ============================================================================

/// Validates a provider integration probe identity.
///
/// Probe identifiers use open-world namespaced identities.
///
/// Supported characters:
///
/// - ASCII letters.
/// - ASCII digits.
/// - Underscores.
/// - Hyphens.
///
/// Namespace segments are separated by dots.
///
/// No arbitrary identifier-length ceiling is imposed.
///
/// # Errors
///
/// Returns an error for an empty identifier, surrounding whitespace,
/// empty namespace segments, or unsupported characters.
pub fn validate_probe_id(value: &str) -> ZurResult<()> {
    if value.is_empty() {
        return Err(
            ZurError::invalid_input(
                "provider probe identity cannot be empty",
            )
            .with_component(COMPONENT)
            .with_operation("validate_probe_id"),
        );
    }

    if value.trim() != value {
        return Err(
            ZurError::invalid_input(
                "provider probe identity cannot contain \
                 surrounding whitespace",
            )
            .with_component(COMPONENT)
            .with_operation("validate_probe_id"),
        );
    }

    for segment in value.split('.') {
        if segment.is_empty() {
            return Err(
                ZurError::invalid_input(
                    "provider probe identity contains \
                     an empty namespace segment",
                )
                .with_component(COMPONENT)
                .with_operation("validate_probe_id"),
            );
        }

        if !segment.bytes().all(|byte| {
            byte.is_ascii_alphanumeric()
                || byte == b'_'
                || byte == b'-'
        }) {
            return Err(
                ZurError::invalid_input(
                    "provider probe identity contains \
                     unsupported characters",
                )
                .with_component(COMPONENT)
                .with_operation("validate_probe_id"),
            );
        }
    }

    Ok(())
}

// ============================================================================
// PROBE OUTCOMES
// ============================================================================

/// Actual outcome of a provider integration probe.
///
/// A non-passing outcome must never be interpreted as successful
/// provider conformance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ProbeOutcome {
    /// The probe executed and its required assertions passed.
    Passed,

    /// The probe executed and established a failure.
    Failed,

    /// Available evidence was insufficient to establish success
    /// or failure.
    Inconclusive,

    /// The probe was not executed.
    NotRun,
}

impl ProbeOutcome {
    /// Returns whether the probe established success.
    #[must_use]
    pub const fn is_passed(self) -> bool {
        matches!(self, Self::Passed)
    }

    /// Returns whether the probe established failure.
    #[must_use]
    pub const fn is_failed(self) -> bool {
        matches!(self, Self::Failed)
    }

    /// Returns whether the outcome is inconclusive.
    #[must_use]
    pub const fn is_inconclusive(self) -> bool {
        matches!(self, Self::Inconclusive)
    }

    /// Returns whether the probe was not executed.
    #[must_use]
    pub const fn was_not_run(self) -> bool {
        matches!(self, Self::NotRun)
    }

    /// Returns whether the outcome permits a successful conformance claim.
    #[must_use]
    pub const fn prevents_conformance(self) -> bool {
        !self.is_passed()
    }
}

impl fmt::Display for ProbeOutcome {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Inconclusive => "inconclusive",
            Self::NotRun => "not_run",
        })
    }
}

// ============================================================================
// PROVIDER PROBE CONTRACT
// ============================================================================

/// An explicitly supplied provider invocation test.
///
/// The probe describes one operation that the caller has deliberately
/// selected for integration testing.
///
/// The harness does not invent operations or select them from a provider.
///
/// The probe implementation owns:
///
/// - Its operation identity.
/// - Its typed request payload.
/// - The safety of the test operation.
/// - The expected response contract.
/// - The response assertions.
///
/// IMPORTANT
///
/// A probe may cause externally observable effects.
///
/// The caller must ensure that its operation is safe and appropriate
/// for the target test environment.
///
/// The harness does not assume that arbitrary provider operations
/// are read-only, idempotent, reversible, or harmless.
///
/// Probe implementations must be deterministic where the tested
/// operation contract requires deterministic behavior.
///
/// They must not claim success without validating the actual response.
pub trait ProviderProbe: Send + Sync {

    /// Returns the stable, namespaced identity of this probe.
    fn id(&self) -> &str;

    /// Returns the canonical provider operation being tested.
    fn operation(&self) -> &ProviderOperationId;

    /// Returns the typed request payload.
    fn payload(&self) -> &dyn ProviderPayload;

    /// Validates the actual response produced by the provider.
    ///
    /// Returning `Ok(())` establishes that this probe's declared
    /// response assertions passed.
    ///
    /// Returning an error establishes that the response did not
    /// satisfy the probe's requirements.
    fn validate_response(
        &self,
        response: &ProviderOperationResponse,
    ) -> ZurResult<()>;
}

// ============================================================================
// PROBE FINDING
// ============================================================================

/// One provider integration test finding.
///
/// The finding records an observed outcome rather than an assumed result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderProbeFinding {

    id: String,

    operation: String,

    outcome: ProbeOutcome,

    message: String,
}

impl ProviderProbeFinding {

    /// Creates a finding from an actual probe observation.
    ///
    /// The probe identity is validated before construction.
    pub fn new(
        id: impl Into<String>,
        operation: impl Into<String>,
        outcome: ProbeOutcome,
        message: impl Into<String>,
    ) -> ZurResult<Self> {

        let id = id.into();

        validate_probe_id(&id)?;

        let operation = operation.into();
        let message = message.into();

        if operation.trim().is_empty() {
            return Err(
                ZurError::invalid_input(
                    "probe operation identity cannot be empty",
                )
                .with_component(COMPONENT)
                .with_operation("finding.create"),
            );
        }

        if message.chars().any(char::is_control) {
            return Err(
                ZurError::invalid_input(
                    "probe finding message contains control characters",
                )
                .with_component(COMPONENT)
                .with_operation("finding.create"),
            );
        }

        Ok(Self {
            id,
            operation,
            outcome,
            message,
        })
    }

    /// Returns the probe identity.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the tested operation identity.
    #[must_use]
    pub fn operation(&self) -> &str {
        &self.operation
    }

    /// Returns the actual probe outcome.
    #[must_use]
    pub const fn outcome(&self) -> ProbeOutcome {
        self.outcome
    }

    /// Returns the diagnostic message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

// ============================================================================
// PROVIDER PROBE REPORT
// ============================================================================

/// Report for one provider integration probe.
///
/// The structural conformance report is retained separately from the
/// operational probe result.
///
/// This prevents a passing structural assessment from being mistaken
/// for proof that the provider can execute operations successfully.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderProbeReport {

    provider_id: ProviderId,

    structural: DomainProviderConformanceReport,

    finding: ProviderProbeFinding,
}

impl ProviderProbeReport {

    /// Constructs a provider probe report.
    fn new(
        provider_id: ProviderId,
        structural: DomainProviderConformanceReport,
        finding: ProviderProbeFinding,
    ) -> Self {

        Self {
            provider_id,
            structural,
            finding,
        }
    }

    /// Returns the canonical provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Returns the canonical structural conformance report.
    #[must_use]
    pub fn structural_report(
        &self,
    ) -> &DomainProviderConformanceReport {
        &self.structural
    }

    /// Returns the operational probe finding.
    #[must_use]
    pub fn finding(&self) -> &ProviderProbeFinding {
        &self.finding
    }

    /// Returns whether structural conformance and the operational
    /// invocation probe both passed.
    ///
    /// This does not prove that untested provider operations conform.
    #[must_use]
    pub fn is_conformant(&self) -> bool {
        self.structural.is_conformant()
            && self.finding.outcome().is_passed()
    }
}

// ============================================================================
// PROVIDER HARNESS
// ============================================================================

/// Reusable provider integration harness.
///
/// This type contains no global state, provider registry, executor,
/// hardware selector, scheduler, or resource manager.
///
/// It delegates structural validation to the canonical ZUR domain
/// conformance implementation.
///
/// Operational tests are explicitly supplied by callers.
///
/// No provider is initialized or shut down automatically.
#[derive(Debug, Default, Clone, Copy)]
pub struct ProviderHarness;

impl ProviderHarness {

    /// Creates a provider integration harness.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    // ------------------------------------------------------------------------
    // STRUCTURAL CONFORMANCE
    // ------------------------------------------------------------------------

    /// Assesses canonical structural domain-provider conformance.
    ///
    /// This delegates to:
    ///
    /// `zur::domains::conformance::assess_domain_provider`
    ///
    /// The existing canonical validation and compatibility rules
    /// remain authoritative.
    ///
    /// This operation does not initialize the provider, inspect
    /// physical hardware, reserve resources, or execute operations.
    ///
    /// A passing report establishes structural conformance only.
    #[must_use]
    pub fn assess_structure(
        &self,
        provider: &dyn DomainProvider,
    ) -> DomainProviderConformanceReport {

        assess_domain_provider(provider)
    }

    // ------------------------------------------------------------------------
    // OPERATIONAL INTEGRATION PROBE
    // ------------------------------------------------------------------------

    /// Executes one explicitly supplied provider integration probe.
    ///
    /// The harness:
    ///
    /// 1. Assesses canonical structural conformance.
    /// 2. Rejects operational testing when structural conformance fails.
    /// 3. Validates the probe identity.
    /// 4. Confirms the requested operation is declared.
    /// 5. Checks cooperative cancellation.
    /// 6. Invokes the actual provider operation.
    /// 7. Validates the actual returned response.
    /// 8. Records the actual result.
    ///
    /// The operation is invoked synchronously through the provider's
    /// asynchronous execution contract.
    ///
    /// The caller supplies the execution context and is responsible
    /// for driving this future with an appropriate executor.
    ///
    /// This method does not initialize or shut down the provider.
    ///
    /// It does not reserve resources or guarantee that the provider
    /// remains available after the observation.
    ///
    /// # Important
    ///
    /// A successful probe establishes only that the supplied operation
    /// passed the supplied assertions under the observed conditions.
    ///
    /// It does not establish universal provider correctness.
    ///
    /// # Errors
    ///
    /// The method returns a report for provider or probe failures.
    ///
    /// A failed operation is preserved as a failed finding rather
    /// than being converted into successful completion.
    pub async fn run_probe(
        &self,
        provider: &dyn DomainProvider,
        probe: &dyn ProviderProbe,
        context: &OperationContext,
    ) -> ProviderProbeReport {

        let provider_id = provider.descriptor().id().clone();

        let structural = self.assess_structure(provider);

        let operation = probe.operation().to_string();

        // Structural conformance is a prerequisite for invoking
        // a provider through this harness.
        //
        // An invalid provider must not receive an operational
        // conformance claim.

        if !structural.is_conformant() {

            return Self::report(
                provider_id,
                structural,
                probe,
                ProbeOutcome::NotRun,
                "operational probe was not executed because \
                 structural provider conformance failed",
            );
        }

        // Validate the probe identity before invoking anything.

        if let Err(error) = validate_probe_id(probe.id()) {

            return Self::report(
                provider_id,
                structural,
                probe,
                ProbeOutcome::Failed,
                &format!(
                    "invalid provider probe identity: {error}"
                ),
            );
        }

        // Ensure that the requested operation is part of the
        // provider's canonical descriptor.
        //
        // A probe cannot invent a provider operation.

        if provider
            .descriptor()
            .operation(probe.operation())
            .is_none()
        {

            return Self::report(
                provider_id,
                structural,
                probe,
                ProbeOutcome::Failed,
                &format!(
                    "provider does not declare operation {operation}"
                ),
            );
        }

        // Cancellation must be observed before execution.

        if let Err(error) = context.check_cancelled() {

            return Self::report(
                provider_id,
                structural,
                probe,
                ProbeOutcome::NotRun,
                &format!(
                    "provider probe was cancelled before invocation: {error}"
                ),
            );
        }

        // Construct the canonical typed provider request.
        //
        // No duplicate operation or payload model is introduced.

        let request = ProviderOperationRequest::new(
            probe.operation(),
            probe.payload(),
        );

        // Execute the real provider operation.
        //
        // ExecutionProvider owns invocation behavior.
        // The harness only coordinates the test.

        let response = match provider
            .invoke(request, context)
            .await
        {

            Ok(response) => response,

            Err(error) => {

                return Self::report(
                    provider_id,
                    structural,
                    probe,
                    ProbeOutcome::Failed,
                    &format!(
                        "provider invocation failed: {error}"
                    ),
                );
            }
        };

        // Check cancellation again before accepting the probe result.
        //
        // A response returned after cancellation is not automatically
        // considered a successful conformance result.

        if let Err(error) = context.check_cancelled() {

            return Self::report(
                provider_id,
                structural,
                probe,
                ProbeOutcome::Inconclusive,
                &format!(
                    "provider returned a response, but the probe \
                     context was cancelled before validation: {error}"
                ),
            );
        }

        // Validate the actual response through the caller's
        // domain-specific assertions.

        match probe.validate_response(&response) {

            Ok(()) => Self::report(
                provider_id,
                structural,
                probe,
                ProbeOutcome::Passed,
                "provider invocation completed and all supplied \
                 response assertions passed",
            ),

            Err(error) => Self::report(
                provider_id,
                structural,
                probe,
                ProbeOutcome::Failed,
                &format!(
                    "provider response validation failed: {error}"
                ),
            ),
        }
    }

    // ------------------------------------------------------------------------
    // REPORT CONSTRUCTION
    // ------------------------------------------------------------------------

    fn report(
        provider_id: ProviderId,
        structural: DomainProviderConformanceReport,
        probe: &dyn ProviderProbe,
        outcome: ProbeOutcome,
        message: &str,
    ) -> ProviderProbeReport {

        let operation = probe.operation().to_string();

        // If the supplied probe identity is malformed, preserve
        // a safe diagnostic identity rather than claiming that
        // the original identity passed validation.

        let id = if validate_probe_id(probe.id()).is_ok() {
            probe.id().to_owned()
        } else {
            "zur.testing.invalid_probe_identity".to_owned()
        };

        let finding = ProviderProbeFinding::new(
            id,
            operation,
            outcome,
            message,
        );

        match finding {

            Ok(finding) => ProviderProbeReport::new(
                provider_id,
                structural,
                finding,
            ),

            // This is a defensive fallback for a malformed finding.
            // It must never convert an invalid finding into success.

            Err(error) => {

                let finding = ProviderProbeFinding {
                    id: "zur.testing.report_construction".to_owned(),
                    operation: "unknown".to_owned(),
                    outcome: ProbeOutcome::Failed,
                    message: format!(
                        "provider report construction failed: {error}"
                    ),
                };

                ProviderProbeReport::new(
                    provider_id,
                    structural,
                    finding,
                )
            }
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn valid_namespaced_probe_identity_is_accepted() {

        assert!(
            validate_probe_id(
                "zamani.quantum.integration.measurement"
            )
            .is_ok()
        );
    }

    #[test]
    fn future_domain_probe_identity_is_accepted() {

        assert!(
            validate_probe_id(
                "future.photonics.optical_execution"
            )
            .is_ok()
        );
    }

    #[test]
    fn deeply_namespaced_probe_identity_is_accepted() {

        let identity = format!(
            "{}.operation",
            "future".repeat(100)
        );

        assert!(
            validate_probe_id(&identity).is_ok()
        );
    }

    #[test]
    fn empty_probe_identity_is_rejected() {

        assert!(
            validate_probe_id("").is_err()
        );
    }

    #[test]
    fn whitespace_probe_identity_is_rejected() {

        assert!(
            validate_probe_id(
                " zamani.classical.execution"
            )
            .is_err()
        );
    }

    #[test]
    fn empty_namespace_segment_is_rejected() {

        assert!(
            validate_probe_id(
                "zamani..quantum.execution"
            )
            .is_err()
        );
    }

    #[test]
    fn invalid_probe_characters_are_rejected() {

        assert!(
            validate_probe_id(
                "zamani.quantum.execution!"
            )
            .is_err()
        );
    }

    #[test]
    fn probe_outcome_only_passes_when_explicitly_passed() {

        assert!(
            ProbeOutcome::Passed.is_passed()
        );

        assert!(
            !ProbeOutcome::Failed.is_passed()
        );

        assert!(
            !ProbeOutcome::Inconclusive.is_passed()
        );

        assert!(
            !ProbeOutcome::NotRun.is_passed()
        );
    }

    #[test]
    fn inconclusive_outcome_prevents_conformance() {

        assert!(
            ProbeOutcome::Inconclusive
                .prevents_conformance()
        );
    }

    #[test]
    fn not_run_outcome_is_not_success() {

        assert!(
            ProbeOutcome::NotRun.was_not_run()
        );

        assert!(
            !ProbeOutcome::NotRun.is_passed()
        );
    }

    #[test]
    fn finding_rejects_empty_operation_identity() {

        let result = ProviderProbeFinding::new(
            "zamani.classical.test",
            " ",
            ProbeOutcome::Passed,
            "test",
        );

        assert!(result.is_err());
    }

    #[test]
    fn finding_preserves_failed_outcome() {

        let finding = ProviderProbeFinding::new(
            "zamani.classical.execution",
            "zamani.classical.execute",
            ProbeOutcome::Failed,
            "provider returned an invalid response",
        )
        .expect("finding must be valid");

        assert_eq!(
            finding.outcome(),
            ProbeOutcome::Failed
        );

        assert!(
            finding.outcome().prevents_conformance()
        );
    }

    #[test]
    fn finding_preserves_inconclusive_outcome() {

        let finding = ProviderProbeFinding::new(
            "zamani.distributed.execution",
            "zamani.distributed.execute",
            ProbeOutcome::Inconclusive,
            "execution result could not be established",
        )
        .expect("finding must be valid");

        assert_eq!(
            finding.outcome(),
            ProbeOutcome::Inconclusive
        );
    }

    #[test]
    fn finding_rejects_control_characters_in_message() {

        let result = ProviderProbeFinding::new(
            "zamani.classical.execution",
            "zamani.classical.execute",
            ProbeOutcome::Failed,
            "invalid\nmessage",
        );

        assert!(result.is_err());
    }

    #[test]
    fn outcome_display_is_stable() {

        assert_eq!(
            ProbeOutcome::Passed.to_string(),
            "passed"
        );

        assert_eq!(
            ProbeOutcome::Failed.to_string(),
            "failed"
        );

        assert_eq!(
            ProbeOutcome::Inconclusive.to_string(),
            "inconclusive"
        );

        assert_eq!(
            ProbeOutcome::NotRun.to_string(),
            "not_run"
        );
    }
}
