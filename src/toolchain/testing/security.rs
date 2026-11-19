
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Security Conformance and Integration Tests
//!
//! Path: `src/toolchain/testing/security.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides security conformance and regression tests for the Zamani
//! toolchain's authoritative security subsystem.
//!
//! This module verifies that the actual security authorities preserve
//! their documented behavior across toolchain operations.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Security conformance test scenarios.
//! - Security validation integration tests.
//! - Fail-closed behavior tests.
//! - Rejection and unresolved-evidence tests.
//! - Security extension integration tests.
//! - Security failure propagation tests.
//! - Security regression tests.
//! - Test evidence classification.
//! - Test-suite acceptance assertions.
//!
//! This module does not own:
//!
//! - Authorization policy implementation.
//! - Authentication.
//! - Identity resolution.
//! - Trust-store management.
//! - Cryptographic algorithms.
//! - Hashing algorithms.
//! - Signature generation or verification.
//! - Package archive verification.
//! - Provenance generation.
//! - Security policy parsing.
//! - Source-language semantic analysis.
//! - Runtime security enforcement.
//! - Hardware security implementation.
//! - External process execution.
//!
//! Those responsibilities remain with their canonical authorities.
//!
//! ## Canonical integration
//!
//! Security composition:
//!     crate::toolchain::security
//!
//! Authorization:
//!     crate::toolchain::security::policy
//!
//! Trust:
//!     crate::toolchain::security::trust
//!
//! Signatures:
//!     crate::toolchain::security::signatures
//!
//! Integrity:
//!     crate::toolchain::security::integrity
//!
//! Provenance:
//!     crate::toolchain::security::provenance
//!
//! Aggregate security validation:
//!     crate::toolchain::security::validation
//!
//! Package verification:
//!     crate::toolchain::packages::verification
//!
//! Compiler security:
//!     crate::compiler::security
//!
//! Backend security:
//!     crate::backend::security
//!
//! Hardware security:
//!     crate::zhal::security
//!
//! Runtime security:
//!     crate::zur::security
//!
//! Conformance testing:
//!     crate::toolchain::testing::conformance
//!
//! Integration testing:
//!     crate::toolchain::testing::integration
//!
//! Normative security semantics:
//!     grammar/spec/security.md
//!
//! Security source grammar:
//!     grammar/security/
//!
//! Policy source grammar:
//!     grammar/policies/
//!
//! ## Fundamental testing invariants
//!
//! 1. Tests invoke canonical security authorities.
//! 2. Missing security evidence never establishes permission.
//! 3. An empty security validation request never passes.
//! 4. Rejected evidence never becomes verified.
//! 5. Indeterminate evidence never becomes verified.
//! 6. An unexecuted check never becomes verified.
//! 7. An operational failure never becomes successful verification.
//! 8. An advisory-only check never establishes authorization.
//! 9. A required check must be affirmatively verified.
//! 10. A security extension must return explicit evidence.
//! 11. Merely registering an extension does not establish trust.
//! 12. Test doubles never establish production trust.
//! 13. Integrity, authenticity, trust and authorization remain distinct.
//! 14. Test results cannot replace runtime enforcement.
//! 15. Security failures must remain distinguishable.
//!
//! ## POCO-REAF and scalability
//!
//! Security tests are independent of:
//!
//! - Classical processor architecture.
//! - Quantum processor architecture.
//! - HDL implementation.
//! - FPGA and ASIC vendors.
//! - AI accelerators.
//! - Distributed topology.
//! - Operating system.
//! - Execution environment.
//! - Hardware provider.
//! - Cryptographic provider.
//! - Computational domain.
//!
//! No arbitrary universal maximum is imposed on:
//!
//! - Security checks.
//! - Test scenarios.
//! - Evidence records.
//! - Providers.
//! - Algorithms.
//! - Identities.
//! - Policies.
//! - Packages.
//! - Artifacts.
//! - Targets.
//! - Distributed participants.
//!
//! Operational limits belong to explicit test configuration,
//! provider contracts and available physical resources.
//!
//! Literal infinite testing or resource availability is not promised.
//!
//! ## Test isolation
//!
//! Tests must not:
//!
//! - Read production credentials.
//! - Expose private keys.
//! - Read arbitrary secret files.
//! - Publish packages.
//! - Modify production trust stores.
//! - Change machine security configuration.
//! - Perform unauthorized deployment.
//! - Assume a test provider is trustworthy.
//!
//! External integration tests must use explicitly configured,
//! isolated test environments.
//!
//! ## Determinism
//!
//! Tests must supply explicit inputs and deterministic evidence
//! wherever the underlying security contract permits.
//!
//! Time-dependent trust and revocation tests must use explicit
//! evaluation timestamps.
//!
//! Tests must not silently depend on the system clock, network,
//! current directory or host hardware.
//!
//! ## Production boundary
//!
//! This file is compiled as part of the test configuration.
//!
//! Its test doubles and synthetic evidence must never be exposed
//! as production security authorities.

use std::sync::Arc;

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainResult,
};

use crate::toolchain::security::validation::{
    CheckRequirement,
    ExtensionCheck,
    ExtensionEvidence,
    SecurityCheckId,
    SecurityValidationExtension,
    SecurityValidationRequest,
    SecurityValidationReport,
    SecurityValidator,
    ValidationStatus,
};

// ============================================================================
// 1. TEST CONTRACT VERSION
// ============================================================================

/// Security testing contract version.
///
/// Independent of the Zamani language, grammar, compiler, package,
/// provider and runtime versions.
pub const SECURITY_TESTING_API_VERSION: u16 = 1;

/// Stable identity of this test subsystem.
pub const COMPONENT: &str = "toolchain.testing.security";

/// Return the security testing contract version.
#[must_use]
pub const fn api_version() -> u16 {
    SECURITY_TESTING_API_VERSION
}

/// Return the stable security testing component identity.
#[must_use]
pub const fn component_id() -> &'static str {
    COMPONENT
}

// ============================================================================
// 2. SECURITY TEST EXPECTATIONS
// ============================================================================

/// Expected outcome of a security conformance scenario.
///
/// These expectations use the canonical validation status rather
/// than introducing another security status enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityTestExpectation {
    /// The required security check must be verified.
    Verified,

    /// The security authority must reject the evidence.
    Rejected,

    /// The security authority must report unresolved evidence.
    Indeterminate,

    /// No verification must be claimed.
    NotEvaluated,

    /// The operation must fail instead of producing a report.
    OperationalFailure,
}

impl SecurityTestExpectation {
    /// Check whether an actual validation status matches this expectation.
    #[must_use]
    pub const fn matches_status(
        self,
        actual: ValidationStatus,
    ) -> bool {
        match self {
            Self::Verified => {
                matches!(actual, ValidationStatus::Verified)
            }

            Self::Rejected => {
                matches!(actual, ValidationStatus::Rejected)
            }

            Self::Indeterminate => {
                matches!(actual, ValidationStatus::Indeterminate)
            }

            Self::NotEvaluated => {
                matches!(actual, ValidationStatus::NotEvaluated)
            }

            Self::OperationalFailure => false,
        }
    }

    /// Return whether the expected result permits a successful
    /// security validation operation.
    #[must_use]
    pub const fn expects_acceptance(self) -> bool {
        matches!(self, Self::Verified)
    }
}

// ============================================================================
// 3. TEST REPORT ASSERTIONS
// ============================================================================

/// Validate a security report against an expected outcome.
///
/// This function only evaluates the returned report. It does not
/// produce security evidence or modify the security decision.
///
/// # Errors
///
/// Returns an error if the actual report does not satisfy the
/// expected outcome or if its acceptance behavior is inconsistent.
pub fn assert_security_report(
    report: &SecurityValidationReport,
    expected: SecurityTestExpectation,
) -> ToolchainResult<()> {
    let actual = report.status();

    if !expected.matches_status(actual) {
        return Err(ToolchainError::invalid_input(
            format!(
                "security test expectation mismatch: expected \
                 {expected:?}, received {actual:?}"
            ),
        ));
    }

    let should_accept = expected.expects_acceptance();

    if report.can_proceed() != should_accept {
        return Err(ToolchainError::invalid_input(
            "security validation acceptance does not match \
             the expected fail-closed outcome",
        ));
    }

    if expected == SecurityTestExpectation::OperationalFailure {
        return Err(ToolchainError::invalid_input(
            "an operational failure cannot be represented \
             by a successful security validation report",
        ));
    }

    Ok(())
}

/// Validate that a security report contains at least one mandatory
/// check and that every mandatory check is verified.
///
/// Advisory checks cannot substitute for mandatory checks.
///
/// # Errors
///
/// Returns an error when mandatory verification is missing,
/// incomplete or rejected.
pub fn assert_required_checks_verified(
    report: &SecurityValidationReport,
) -> ToolchainResult<()> {
    if report.check_count() == 0 {
        return Err(ToolchainError::invalid_input(
            "security report contains no evaluated checks",
        ));
    }

    if !report.required_checks_verified() {
        return Err(ToolchainError::invalid_input(
            "one or more mandatory security checks were \
             not affirmatively verified",
        ));
    }

    if report.rejected_count() != 0 {
        return Err(ToolchainError::invalid_input(
            "security report contains rejected checks",
        ));
    }

    if report.has_unresolved_checks() {
        return Err(ToolchainError::invalid_input(
            "security report contains unresolved checks",
        ));
    }

    if !report.can_proceed() {
        return Err(ToolchainError::invalid_input(
            "security report does not permit proceeding",
        ));
    }

    Ok(())
}

// ============================================================================
// 4. CANONICAL SECURITY VALIDATION HARNESS
// ============================================================================

/// Execute a security validation request using the canonical
/// toolchain security validator.
///
/// No alternate policy engine, trust evaluator or cryptographic
/// implementation is introduced.
///
/// # Errors
///
/// Returns the original toolchain validation error when the
/// authoritative validator cannot complete the request.
pub fn execute_security_validation(
    request: SecurityValidationRequest<'_>,
) -> ToolchainResult<SecurityValidationReport> {
    SecurityValidator::new().validate(request)
}

/// Execute a security validation request and check its expected outcome.
///
/// This function deliberately separates:
///
/// - Validation execution.
/// - Validation outcome.
/// - Test expectation.
///
/// A rejected report is a completed validation result, not an
/// infrastructure failure.
///
/// # Errors
///
/// Returns an error if validation fails operationally or if the
/// completed report violates the test expectation.
pub fn execute_and_assert_security_validation(
    request: SecurityValidationRequest<'_>,
    expected: SecurityTestExpectation,
) -> ToolchainResult<SecurityValidationReport> {
    let report = execute_security_validation(request)?;

    assert_security_report(&report, expected)?;

    Ok(report)
}

// ============================================================================
// 5. SECURITY EXTENSION TEST SUPPORT
// ============================================================================

/// Synthetic security extension for isolated conformance tests.
///
/// This type must never be registered as a production security
/// authority.
///
/// Its evidence is deliberately synthetic and is only suitable
/// for testing the aggregate validation coordinator.
#[derive(Debug, Clone)]
pub struct TestSecurityExtension {
    id: Arc<str>,
    authority: Arc<str>,
    evidence_reference: Arc<str>,
    status: ValidationStatus,
    message: Arc<str>,
    failure: bool,
}

impl TestSecurityExtension {
    /// Construct an isolated synthetic security extension.
    ///
    /// The identity, authority and evidence reference must be
    /// nonempty and must not contain control characters.
    ///
    /// This constructor does not establish trust in the extension.
    ///
    /// # Errors
    ///
    /// Returns an error when an identity or evidence field is invalid.
    pub fn new(
        id: impl Into<Arc<str>>,
        authority: impl Into<Arc<str>>,
        evidence_reference: impl Into<Arc<str>>,
        status: ValidationStatus,
        message: impl Into<Arc<str>>,
    ) -> ToolchainResult<Self> {
        let id = id.into();
        let authority = authority.into();
        let evidence_reference = evidence_reference.into();
        let message = message.into();

        validate_test_identity(&id, "extension identity")?;
        validate_test_identity(&authority, "extension authority")?;
        validate_test_identity(
            &evidence_reference,
            "evidence reference",
        )?;
        validate_test_identity(&message, "evidence message")?;

        Ok(Self {
            id,
            authority,
            evidence_reference,
            status,
            message,
            failure: false,
        })
    }

    /// Construct an extension that simulates an operational failure.
    ///
    /// The failure must propagate as an error, never as successful
    /// verification.
    ///
    /// # Errors
    ///
    /// Returns an error when the extension identity is invalid.
    pub fn failing(
        id: impl Into<Arc<str>>,
    ) -> ToolchainResult<Self> {
        let id = id.into();

        validate_test_identity(&id, "extension identity")?;

        Ok(Self {
            id,
            authority: Arc::from("test.failure"),
            evidence_reference: Arc::from("test.failure.evidence"),
            status: ValidationStatus::Indeterminate,
            message: Arc::from("synthetic operational failure"),
            failure: true,
        })
    }

    /// Return the synthetic extension identity.
    #[must_use]
    pub fn id_value(&self) -> &str {
        &self.id
    }

    /// Return whether this extension simulates an operational failure.
    #[must_use]
    pub const fn is_failing(&self) -> bool {
        self.failure
    }
}

impl SecurityValidationExtension for TestSecurityExtension {
    fn id(&self) -> &str {
        &self.id
    }

    fn validate(
        &self,
        _operation: &str,
    ) -> ToolchainResult<ExtensionEvidence> {
        if self.failure {
            return Err(ToolchainError::invalid_input(
                "synthetic security extension failure",
            ));
        }

        ExtensionEvidence::new(
            Arc::clone(&self.authority),
            Arc::clone(&self.evidence_reference),
            self.status,
            Arc::clone(&self.message),
        )
    }
}

// ============================================================================
// 6. VALIDATION INPUT HELPERS
// ============================================================================

fn validate_test_identity(
    value: &str,
    field: &'static str,
) -> ToolchainResult<()> {
    if value.trim().is_empty() {
        return Err(ToolchainError::invalid_input(
            format!("{field} must not be empty"),
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(ToolchainError::invalid_input(
            format!("{field} contains control characters"),
        ));
    }

    Ok(())
}

fn extension_request<'a>(
    extension: &'a dyn SecurityValidationExtension,
    requirement: CheckRequirement,
) -> ToolchainResult<SecurityValidationRequest<'a>> {
    let check_id = SecurityCheckId::new(
        "testing.security.extension",
    )?;

    let check = ExtensionCheck::new(
        check_id,
        requirement,
        extension,
    );

    SecurityValidationRequest::new(
        "testing.security.validate",
    )
    .map(|request| request.with_extension(check))
}

// ============================================================================
// 7. TESTS — CONTRACT AND VERSIONING
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------------------------------
    // Test helpers
    // ------------------------------------------------------------------------

    fn extension_with_status(
        status: ValidationStatus,
    ) -> TestSecurityExtension {
        TestSecurityExtension::new(
            "test.security.extension",
            "test.security.authority",
            "test.security.evidence",
            status,
            "synthetic security test evidence",
        )
        .expect("valid synthetic extension")
    }

    fn execute_extension(
        status: ValidationStatus,
        requirement: CheckRequirement,
    ) -> ToolchainResult<SecurityValidationReport> {
        let extension = extension_with_status(status);
        let request = extension_request(
            &extension,
            requirement,
        )?;

        execute_security_validation(request)
    }

    // ------------------------------------------------------------------------
    // Versioning
    // ------------------------------------------------------------------------

    #[test]
    fn security_testing_api_version_is_stable() {
        assert_eq!(
            api_version(),
            SECURITY_TESTING_API_VERSION,
        );
    }

    #[test]
    fn security_testing_component_identity_is_stable() {
        assert_eq!(
            component_id(),
            "toolchain.testing.security",
        );
    }

    // ------------------------------------------------------------------------
    // Expected outcome classification
    // ------------------------------------------------------------------------

    #[test]
    fn verified_status_matches_verified_expectation() {
        assert!(
            SecurityTestExpectation::Verified.matches_status(
                ValidationStatus::Verified,
            )
        );
    }

    #[test]
    fn rejected_status_matches_rejected_expectation() {
        assert!(
            SecurityTestExpectation::Rejected.matches_status(
                ValidationStatus::Rejected,
            )
        );
    }

    #[test]
    fn indeterminate_status_is_not_verified() {
        assert!(
            SecurityTestExpectation::Indeterminate.matches_status(
                ValidationStatus::Indeterminate,
            )
        );

        assert!(
            !SecurityTestExpectation::Verified.matches_status(
                ValidationStatus::Indeterminate,
            )
        );
    }

    #[test]
    fn not_evaluated_status_is_not_verified() {
        assert!(
            SecurityTestExpectation::NotEvaluated.matches_status(
                ValidationStatus::NotEvaluated,
            )
        );

        assert!(
            !SecurityTestExpectation::Verified.matches_status(
                ValidationStatus::NotEvaluated,
            )
        );
    }

    #[test]
    fn operational_failure_is_not_a_validation_status() {
        assert!(
            !SecurityTestExpectation::OperationalFailure.matches_status(
                ValidationStatus::Verified,
            )
        );
    }

    // ------------------------------------------------------------------------
    // Empty request — default deny
    // ------------------------------------------------------------------------

    #[test]
    fn empty_request_never_establishes_security_acceptance() {
        let request = SecurityValidationRequest::new(
            "testing.security.empty",
        )
        .expect("valid operation identity");

        assert!(request.is_empty());
        assert_eq!(request.check_count(), 0);

        let report = execute_security_validation(request)
            .expect("empty validation should produce a report");

        assert_eq!(
            report.status(),
            ValidationStatus::NotEvaluated,
        );

        assert!(!report.can_proceed());
        assert!(!report.required_checks_verified());
        assert_eq!(report.check_count(), 0);
    }

    #[test]
    fn empty_report_fails_required_check_assertion() {
        let request = SecurityValidationRequest::new(
            "testing.security.empty",
        )
        .expect("valid operation identity");

        let report = execute_security_validation(request)
            .expect("validation should complete");

        assert!(
            assert_required_checks_verified(&report).is_err()
        );
    }

    #[test]
    fn empty_report_does_not_pass_verified_expectation() {
        let request = SecurityValidationRequest::new(
            "testing.security.empty",
        )
        .expect("valid operation identity");

        let report = execute_security_validation(request)
            .expect("validation should complete");

        assert!(
            assert_security_report(
                &report,
                SecurityTestExpectation::Verified,
            )
            .is_err()
        );
    }

    // ------------------------------------------------------------------------
    // Mandatory verified evidence
    // ------------------------------------------------------------------------

    #[test]
    fn required_verified_extension_allows_validation() {
        let report = execute_extension(
            ValidationStatus::Verified,
            CheckRequirement::Required,
        )
        .expect("extension validation should complete");

        assert_eq!(
            report.status(),
            ValidationStatus::Verified,
        );

        assert_eq!(report.check_count(), 1);
        assert_eq!(report.verified_count(), 1);
        assert_eq!(report.rejected_count(), 0);
        assert_eq!(report.unresolved_count(), 0);

        assert!(report.required_checks_verified());
        assert!(report.can_proceed());

        assert_required_checks_verified(&report)
            .expect("mandatory evidence should be verified");
    }

    #[test]
    fn verified_expectation_accepts_verified_report() {
        let report = execute_extension(
            ValidationStatus::Verified,
            CheckRequirement::Required,
        )
        .expect("validation should complete");

        assert_security_report(
            &report,
            SecurityTestExpectation::Verified,
        )
        .expect("verified report should satisfy expectation");
    }

    // ------------------------------------------------------------------------
    // Rejected evidence
    // ------------------------------------------------------------------------

    #[test]
    fn rejected_extension_blocks_required_operation() {
        let report = execute_extension(
            ValidationStatus::Rejected,
            CheckRequirement::Required,
        )
        .expect("rejection is a completed validation result");

        assert_eq!(
            report.status(),
            ValidationStatus::Rejected,
        );

        assert_eq!(report.rejected_count(), 1);
        assert!(!report.can_proceed());
        assert!(!report.required_checks_verified());

        assert_security_report(
            &report,
            SecurityTestExpectation::Rejected,
        )
        .expect("rejection should match expectation");
    }

    #[test]
    fn rejected_evidence_never_passes_verified_assertion() {
        let report = execute_extension(
            ValidationStatus::Rejected,
            CheckRequirement::Required,
        )
        .expect("validation should complete");

        assert!(
            assert_required_checks_verified(&report).is_err()
        );
    }

    // ------------------------------------------------------------------------
    // Indeterminate evidence
    // ------------------------------------------------------------------------

    #[test]
    fn indeterminate_extension_does_not_allow_operation() {
        let report = execute_extension(
            ValidationStatus::Indeterminate,
            CheckRequirement::Required,
        )
        .expect("indeterminate result should be reportable");

        assert_eq!(
            report.status(),
            ValidationStatus::Indeterminate,
        );

        assert_eq!(report.unresolved_count(), 1);
        assert!(!report.can_proceed());
        assert!(!report.required_checks_verified());

        assert_security_report(
            &report,
            SecurityTestExpectation::Indeterminate,
        )
        .expect("indeterminate result should be preserved");
    }

    #[test]
    fn unresolved_evidence_never_passes_required_check_assertion() {
        let report = execute_extension(
            ValidationStatus::Indeterminate,
            CheckRequirement::Required,
        )
        .expect("validation should complete");

        assert!(
            assert_required_checks_verified(&report).is_err()
        );
    }

    // ------------------------------------------------------------------------
    // Not evaluated evidence
    // ------------------------------------------------------------------------

    #[test]
    fn not_evaluated_extension_does_not_allow_operation() {
        let report = execute_extension(
            ValidationStatus::NotEvaluated,
            CheckRequirement::Required,
        )
        .expect("not-evaluated result should be reportable");

        assert_eq!(
            report.status(),
            ValidationStatus::NotEvaluated,
        );

        assert_eq!(report.unresolved_count(), 1);
        assert!(!report.can_proceed());
    }

    // ------------------------------------------------------------------------
    // Advisory checks
    // ------------------------------------------------------------------------

    #[test]
    fn advisory_only_verified_check_does_not_authorize_operation() {
        let report = execute_extension(
            ValidationStatus::Verified,
            CheckRequirement::Advisory,
        )
        .expect("advisory validation should complete");

        assert_eq!(
            report.status(),
            ValidationStatus::Indeterminate,
        );

        assert!(!report.required_checks_verified());
        assert!(!report.can_proceed());
    }

    #[test]
    fn mandatory_verified_check_with_advisory_verified_check_passes() {
        let required = extension_with_status(
            ValidationStatus::Verified,
        );

        let advisory = TestSecurityExtension::new(
            "test.security.advisory",
            "test.security.advisory.authority",
            "test.security.advisory.evidence",
            ValidationStatus::Verified,
            "advisory evidence",
        )
        .expect("valid advisory extension");

        let required_check = ExtensionCheck::new(
            SecurityCheckId::new(
                "testing.security.required",
            )
            .expect("valid required check ID"),
            CheckRequirement::Required,
            &required,
        );

        let advisory_check = ExtensionCheck::new(
            SecurityCheckId::new(
                "testing.security.advisory",
            )
            .expect("valid advisory check ID"),
            CheckRequirement::Advisory,
            &advisory,
        );

        let request = SecurityValidationRequest::new(
            "testing.security.required_and_advisory",
        )
        .expect("valid operation")
        .with_extension(required_check)
        .with_extension(advisory_check);

        let report = execute_security_validation(request)
            .expect("both extensions should complete");

        assert_eq!(report.check_count(), 2);
        assert_eq!(
            report.status(),
            ValidationStatus::Verified,
        );

        assert!(report.can_proceed());
        assert_required_checks_verified(&report)
            .expect("mandatory check should be satisfied");
    }

    #[test]
    fn advisory_rejection_still_blocks_operation() {
        let required = extension_with_status(
            ValidationStatus::Verified,
        );

        let advisory = TestSecurityExtension::new(
            "test.security.advisory",
            "test.security.advisory.authority",
            "test.security.advisory.evidence",
            ValidationStatus::Rejected,
            "advisory rejection",
        )
        .expect("valid advisory extension");

        let request = SecurityValidationRequest::new(
            "testing.security.advisory_rejection",
        )
        .expect("valid operation")
        .with_extension(
            ExtensionCheck::new(
                SecurityCheckId::new(
                    "testing.security.required",
                )
                .expect("valid ID"),
                CheckRequirement::Required,
                &required,
            ),
        )
        .with_extension(
            ExtensionCheck::new(
                SecurityCheckId::new(
                    "testing.security.advisory",
                )
                .expect("valid ID"),
                CheckRequirement::Advisory,
                &advisory,
            ),
        );

        let report = execute_security_validation(request)
            .expect("validation should complete");

        assert_eq!(
            report.status(),
            ValidationStatus::Rejected,
        );

        assert!(!report.can_proceed());
    }

    // ------------------------------------------------------------------------
    // Operational failure propagation
    // ------------------------------------------------------------------------

    #[test]
    fn extension_operational_failure_propagates_as_error() {
        let extension = TestSecurityExtension::failing(
            "test.security.failure",
        )
        .expect("valid extension identity");

        assert!(extension.is_failing());

        let request = extension_request(
            &extension,
            CheckRequirement::Required,
        )
        .expect("valid request");

        let result = execute_security_validation(request);

        assert!(result.is_err());
    }

    #[test]
    fn operational_failure_cannot_be_asserted_as_success() {
        let extension = TestSecurityExtension::failing(
            "test.security.failure",
        )
        .expect("valid extension identity");

        let request = extension_request(
            &extension,
            CheckRequirement::Required,
        )
        .expect("valid request");

        assert!(
            execute_and_assert_security_validation(
                request,
                SecurityTestExpectation::Verified,
            )
            .is_err()
        );
    }

    // ------------------------------------------------------------------------
    // Input validation
    // ------------------------------------------------------------------------

    #[test]
    fn empty_extension_identity_is_rejected() {
        assert!(
            TestSecurityExtension::new(
                "",
                "test.authority",
                "test.evidence",
                ValidationStatus::Verified,
                "test",
            )
            .is_err()
        );
    }

    #[test]
    fn empty_authority_identity_is_rejected() {
        assert!(
            TestSecurityExtension::new(
                "test.extension",
                "",
                "test.evidence",
                ValidationStatus::Verified,
                "test",
            )
            .is_err()
        );
    }

    #[test]
    fn empty_evidence_reference_is_rejected() {
        assert!(
            TestSecurityExtension::new(
                "test.extension",
                "test.authority",
                "",
                ValidationStatus::Verified,
                "test",
            )
            .is_err()
        );
    }

    #[test]
    fn empty_evidence_message_is_rejected() {
        assert!(
            TestSecurityExtension::new(
                "test.extension",
                "test.authority",
                "test.evidence",
                ValidationStatus::Verified,
                "",
            )
            .is_err()
        );
    }

    #[test]
    fn control_characters_in_extension_identity_are_rejected() {
        assert!(
            TestSecurityExtension::new(
                "test\nextension",
                "test.authority",
                "test.evidence",
                ValidationStatus::Verified,
                "test",
            )
            .is_err()
        );
    }

    #[test]
    fn invalid_operation_identity_is_rejected() {
        assert!(
            SecurityValidationRequest::new("").is_err()
        );
    }

    // ------------------------------------------------------------------------
    // Security report invariants
    // ------------------------------------------------------------------------

    #[test]
    fn verified_report_has_consistent_counts() {
        let report = execute_extension(
            ValidationStatus::Verified,
            CheckRequirement::Required,
        )
        .expect("validation should complete");

        assert_eq!(report.check_count(), 1);
        assert_eq!(report.verified_count(), 1);
        assert_eq!(report.rejected_count(), 0);
        assert_eq!(report.unresolved_count(), 0);

        assert!(report.is_complete());
        assert!(report.required_checks_verified());
        assert!(report.can_proceed());
    }

    #[test]
    fn rejected_report_has_consistent_counts() {
        let report = execute_extension(
            ValidationStatus::Rejected,
            CheckRequirement::Required,
        )
        .expect("validation should complete");

        assert_eq!(report.check_count(), 1);
        assert_eq!(report.verified_count(), 0);
        assert_eq!(report.rejected_count(), 1);

        assert!(report.is_complete());
        assert!(!report.can_proceed());
    }

    #[test]
    fn indeterminate_report_is_not_complete() {
        let report = execute_extension(
            ValidationStatus::Indeterminate,
            CheckRequirement::Required,
        )
        .expect("validation should complete");

        assert!(!report.is_complete());
        assert!(report.has_unresolved_checks());
        assert!(!report.can_proceed());
    }
}
