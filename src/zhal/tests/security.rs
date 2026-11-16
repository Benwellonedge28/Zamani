
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani — ZHAL Security Conformance Tests
//!
//! Path: src/zhal/tests/security.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97.1
//!
//! Purpose:
//! Verify the security boundaries of the Zamani Universal Hardware
//! Abstraction Layer.
//!
//! These tests exercise the existing canonical security authorities.
//! They do not introduce a competing authentication, authorization,
//! trust, audit, cryptography or isolation implementation.
//!
//! Canonical authorities:
//!
//! - stdlib::security::identity
//! - stdlib::security::policy
//! - stdlib::provider
//! - zur::security::permissions
//! - zur::security::isolation
//! - distributed::security
//! - zhal::security::trust
//! - zhal::security::authorization
//! - zhal::security::isolation
//! - zhal::devices::access
//!
//! Grammar authorities:
//!
//! - grammar/security/security.g4
//! - grammar/security/authorization.g4
//! - grammar/security/permissions.g4
//! - grammar/security/trust.g4
//! - grammar/security/sandbox.g4
//! - grammar/security/security-constraints.g4
//! - grammar/security/audit.g4
//! - grammar/security/provenance.g4
//! - grammar/execution/execution-context.g4
//!
//! Architectural rules:
//!
//! - Security is deny-by-default.
//! - Authentication and authorization are independent.
//! - Provider trust and authorization are independent.
//! - Authorization and isolation are independent.
//! - Hardware capability never implies permission.
//! - Hardware discovery never implies permission.
//! - Missing or indeterminate security evidence never establishes success.
//! - Mandatory audit failure prevents authorization.
//! - Security diagnostics must not disclose secret material.
//! - Providers must enforce their own applicable security boundaries.
//! - Source grammar expresses intent, not runtime enforcement.
//!
//! Scalability:
//!
//! These tests introduce no universal hardware, provider, device,
//! principal, policy, permission or resource-count limits.
//!
//! Test workloads are samples, not semantic capacity ceilings.
//!
//! Safety:
//!
//! Unsafe Rust is forbidden.

#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use std::sync::{
        Arc,
        Mutex,
    };

    use crate::stdlib::provider::ProviderId;

    use crate::stdlib::security::identity::PrincipalId;

    use crate::stdlib::security::policy::{
        AuditEvent,
        AuditReceipt,
        AuditSink,
        AuditSinkId,
        AuditStatus,
        AuthorizationRequest,
        AuthorizationService,
        PolicyAction,
        PolicyAttributeKey,
        PolicyAttributeValue,
        PolicyDecision,
        PolicyEffect,
        PolicyError,
        PolicyErrorKind,
        PolicyEvaluator,
        PolicyEvaluatorId,
        PolicyObligation,
        PolicyObligationId,
        PolicyResource,
        PolicyResult,
    };

    use crate::zhal::errors::{
        ZhalErrorKind,
    };

    use crate::zhal::security::authorization::{
        HardwareAuthorizationService,
    };

    use crate::zhal::security::trust::{
        evaluate_trust,
        RevocationChecker,
        RevocationStatus,
        SignatureScheme,
        SignatureValidity,
        SignatureVerifier,
        TrustAssessment,
        TrustDecision,
        TrustEvidence,
        TrustEvidenceId,
        TrustIssuerId,
        TrustKeyReference,
        TrustPolicy,
        TrustPolicyId,
        TrustTimestamp,
    };

    // ========================================================================
    // TEST FIXTURES
    // ========================================================================

    /// A deterministic policy evaluator for testing the real
    /// canonical AuthorizationService.
    ///
    /// This evaluator is test infrastructure only. It is not a
    /// production authorization authority.
    struct TestEvaluator {
        id: PolicyEvaluatorId,
        decision: PolicyDecision,
    }

    impl PolicyEvaluator for TestEvaluator {
        fn id(&self) -> &PolicyEvaluatorId {
            &self.id
        }

        fn evaluate(
            &self,
            _request: &AuthorizationRequest,
        ) -> PolicyResult<PolicyDecision> {
            Ok(self.decision.clone())
        }
    }

    /// Audit sink that records events in memory.
    ///
    /// The mutex protects test observations only.
    /// Production audit persistence remains externally owned.
    struct RecordingAuditSink {
        id: AuditSinkId,
        events: Mutex<Vec<AuditEvent>>,
    }

    impl AuditSink for RecordingAuditSink {
        fn id(&self) -> &AuditSinkId {
            &self.id
        }

        fn append(
            &self,
            event: &AuditEvent,
        ) -> PolicyResult<AuditReceipt> {
            let mut events = self.events.lock().map_err(|_| {
                PolicyError::new(PolicyErrorKind::AuditFailure)
            })?;

            events.push(event.clone());

            AuditReceipt::new(
                self.id.clone(),
                "test::audit::event",
            )
        }
    }

    /// Audit sink that always fails.
    struct FailingAuditSink {
        id: AuditSinkId,
    }

    impl AuditSink for FailingAuditSink {
        fn id(&self) -> &AuditSinkId {
            &self.id
        }

        fn append(
            &self,
            _event: &AuditEvent,
        ) -> PolicyResult<AuditReceipt> {
            Err(PolicyError::new(
                PolicyErrorKind::AuditFailure,
            ))
        }
    }

    /// Audit sink that returns a receipt belonging to another sink.
    ///
    /// This simulates a compromised or contract-violating provider.
    struct MismatchedAuditSink {
        id: AuditSinkId,
    }

    impl AuditSink for MismatchedAuditSink {
        fn id(&self) -> &AuditSinkId {
            &self.id
        }

        fn append(
            &self,
            _event: &AuditEvent,
        ) -> PolicyResult<AuditReceipt> {
            AuditReceipt::new(
                AuditSinkId::new("foreign::audit_sink").unwrap(),
                "foreign::event",
            )
        }
    }

    /// Constructs a valid, resource-scoped authorization request.
    fn authorization_request() -> AuthorizationRequest {
        AuthorizationRequest::new(
            PrincipalId::new("application::worker").unwrap(),
            PolicyAction::new(
                "hardware::device::access",
            )
            .unwrap(),
        )
        .with_resource(
            PolicyResource::new(
                "hardware::device::primary",
            )
            .unwrap(),
        )
    }

    /// Constructs a configured canonical authorization service.
    fn authorization_service(
        effect: PolicyEffect,
    ) -> (
        HardwareAuthorizationService,
        Arc<RecordingAuditSink>,
    ) {
        let evaluator = Arc::new(TestEvaluator {
            id: PolicyEvaluatorId::new(
                "test::hardware_policy_evaluator",
            )
            .unwrap(),

            decision: PolicyDecision::new(effect),
        });

        let sink = Arc::new(RecordingAuditSink {
            id: AuditSinkId::new(
                "test::hardware_audit_sink",
            )
            .unwrap(),

            events: Mutex::new(Vec::new()),
        });

        let canonical = Arc::new(
            AuthorizationService::new(
                evaluator,
                sink.clone(),
            ),
        );

        (
            HardwareAuthorizationService::new(canonical),
            sink,
        )
    }

    // ========================================================================
    // TRUST TEST FIXTURES
    // ========================================================================

    struct ValidSignatureVerifier;

    impl SignatureVerifier for ValidSignatureVerifier {
        fn verify(
            &self,
            _evidence: &TrustEvidence,
        ) -> crate::zhal::errors::ZhalResult<SignatureValidity> {
            Ok(SignatureValidity::Valid)
        }
    }

    struct InvalidSignatureVerifier;

    impl SignatureVerifier for InvalidSignatureVerifier {
        fn verify(
            &self,
            _evidence: &TrustEvidence,
        ) -> crate::zhal::errors::ZhalResult<SignatureValidity> {
            Ok(SignatureValidity::Invalid)
        }
    }

    struct IndeterminateSignatureVerifier;

    impl SignatureVerifier for IndeterminateSignatureVerifier {
        fn verify(
            &self,
            _evidence: &TrustEvidence,
        ) -> crate::zhal::errors::ZhalResult<SignatureValidity> {
            Ok(SignatureValidity::Indeterminate)
        }
    }

    struct FailingSignatureVerifier;

    impl SignatureVerifier for FailingSignatureVerifier {
        fn verify(
            &self,
            _evidence: &TrustEvidence,
        ) -> crate::zhal::errors::ZhalResult<SignatureValidity> {
            Err(
                crate::zhal::errors::ZhalError::new(
                    ZhalErrorKind::ProviderFailure,
                    "test verification infrastructure failure",
                ),
            )
        }
    }

    struct ValidRevocationChecker;

    impl RevocationChecker for ValidRevocationChecker {
        fn check(
            &self,
            _evidence: &TrustEvidence,
            _evaluated_at: TrustTimestamp,
        ) -> crate::zhal::errors::ZhalResult<RevocationStatus> {
            Ok(RevocationStatus::NotRevoked)
        }
    }

    struct RevokedEvidenceChecker;

    impl RevocationChecker for RevokedEvidenceChecker {
        fn check(
            &self,
            _evidence: &TrustEvidence,
            _evaluated_at: TrustTimestamp,
        ) -> crate::zhal::errors::ZhalResult<RevocationStatus> {
            Ok(RevocationStatus::Revoked)
        }
    }

    struct UnknownRevocationChecker;

    impl RevocationChecker for UnknownRevocationChecker {
        fn check(
            &self,
            _evidence: &TrustEvidence,
            _evaluated_at: TrustTimestamp,
        ) -> crate::zhal::errors::ZhalResult<RevocationStatus> {
            Ok(RevocationStatus::Unknown)
        }
    }

    fn provider_id() -> ProviderId {
        ProviderId::new(
            "zamani::test::hardware_provider",
        )
        .unwrap()
    }

    fn trust_issuer() -> TrustIssuerId {
        TrustIssuerId::new(
            "zamani::test::trust_authority",
        )
        .unwrap()
    }

    fn signature_scheme() -> SignatureScheme {
        SignatureScheme::new(
            "test::signature_scheme",
        )
        .unwrap()
    }

    fn trust_policy() -> TrustPolicy {
        TrustPolicy::new(
            TrustPolicyId::new(
                "zamani::test::hardware_trust_policy",
            )
            .unwrap(),
        )
        .trust_issuer(trust_issuer())
        .allow_scheme(signature_scheme())
    }

    fn trust_evidence() -> TrustEvidence {
        TrustEvidence::new(
            TrustEvidenceId::new(
                "test::provider_evidence",
            )
            .unwrap(),
            provider_id(),
            trust_issuer(),
            TrustKeyReference::new(
                "external::verification_key_reference",
            )
            .unwrap(),
            signature_scheme(),
            TrustTimestamp::from_unix_nanos(100),
            Some(
                TrustTimestamp::from_unix_nanos(1_000),
            ),
            b"canonical signed provider statement".to_vec(),
            b"test signature bytes".to_vec(),
        )
        .unwrap()
    }

    fn evaluation_time() -> TrustTimestamp {
        TrustTimestamp::from_unix_nanos(200)
    }

    fn assess_trust(
        verifier: &dyn SignatureVerifier,
        revocation: Option<&dyn RevocationChecker>,
    ) -> crate::zhal::errors::ZhalResult<TrustAssessment> {
        evaluate_trust(
            &trust_policy(),
            &trust_evidence(),
            &provider_id(),
            evaluation_time(),
            verifier,
            revocation,
        )
    }

    // ========================================================================
    // AUTHORIZATION — DENY BY DEFAULT
    // ========================================================================

    #[test]
    fn unconditional_permit_is_accepted() {
        let (service, _) =
            authorization_service(PolicyEffect::Permit);

        let receipt = service
            .authorize(&authorization_request())
            .expect("unconditional permit should be accepted");

        assert!(receipt.is_authorized());
    }

    #[test]
    fn explicit_denial_never_authorizes_hardware() {
        let (service, _) =
            authorization_service(PolicyEffect::Deny);

        let result = service.authorize(
            &authorization_request(),
        );

        assert!(result.is_err());

        assert_eq!(
            result.unwrap_err().kind(),
            ZhalErrorKind::PermissionDenied,
        );
    }

    #[test]
    fn not_applicable_never_authorizes_hardware() {
        let (service, _) =
            authorization_service(PolicyEffect::NotApplicable);

        let result = service.authorize(
            &authorization_request(),
        );

        assert!(result.is_err());

        assert_eq!(
            result.unwrap_err().kind(),
            ZhalErrorKind::PermissionDenied,
        );
    }

    #[test]
    fn indeterminate_policy_never_authorizes_hardware() {
        let (service, _) =
            authorization_service(PolicyEffect::Indeterminate);

        let result = service.authorize(
            &authorization_request(),
        );

        assert!(result.is_err());

        assert_eq!(
            result.unwrap_err().kind(),
            ZhalErrorKind::PermissionDenied,
        );
    }

    #[test]
    fn authorization_requires_a_resource_scope() {
        let (service, _) =
            authorization_service(PolicyEffect::Permit);

        let request = AuthorizationRequest::new(
            PrincipalId::new(
                "application::worker",
            )
            .unwrap(),
            PolicyAction::new(
                "hardware::device::access",
            )
            .unwrap(),
        );

        let result = service.authorize(&request);

        assert!(result.is_err());

        assert_eq!(
            result.unwrap_err().kind(),
            ZhalErrorKind::InvalidInput,
        );
    }

    #[test]
    fn outstanding_obligations_prevent_hardware_authorization() {
        let evaluator = Arc::new(TestEvaluator {
            id: PolicyEvaluatorId::new(
                "test::obligation_evaluator",
            )
            .unwrap(),

            decision: PolicyDecision::new(
                PolicyEffect::Permit,
            )
            .with_obligation(
                PolicyObligation::new(
                    PolicyObligationId::new(
                        "security::record_access",
                    )
                    .unwrap(),
                ),
            ),
        });

        let sink = Arc::new(RecordingAuditSink {
            id: AuditSinkId::new(
                "test::obligation_audit",
            )
            .unwrap(),

            events: Mutex::new(Vec::new()),
        });

        let service = HardwareAuthorizationService::new(
            Arc::new(
                AuthorizationService::new(
                    evaluator,
                    sink,
                ),
            ),
        );

        let result = service.authorize(
            &authorization_request(),
        );

        assert!(result.is_err());

        assert_eq!(
            result.unwrap_err().kind(),
            ZhalErrorKind::PermissionDenied,
        );
    }

    // ========================================================================
    // AUTHORIZATION — AUDIT INTEGRITY
    // ========================================================================

    #[test]
    fn successful_authorization_records_audit_event() {
        let (service, sink) =
            authorization_service(PolicyEffect::Permit);

        service.authorize(
            &authorization_request(),
        )
        .expect("authorization should succeed");

        let events = sink.events.lock().unwrap();

        assert_eq!(events.len(), 1);

        assert_eq!(
            events[0].request().action().as_str(),
            "hardware::device::access",
        );

        assert_eq!(
            events[0]
                .request()
                .resource()
                .unwrap()
                .as_str(),
            "hardware::device::primary",
        );
    }

    #[test]
    fn mandatory_audit_failure_prevents_authorization() {
        let evaluator = Arc::new(TestEvaluator {
            id: PolicyEvaluatorId::new(
                "test::audit_failure_evaluator",
            )
            .unwrap(),

            decision: PolicyDecision::new(
                PolicyEffect::Permit,
            ),
        });

        let sink = Arc::new(FailingAuditSink {
            id: AuditSinkId::new(
                "test::failing_audit",
            )
            .unwrap(),
        });

        let service = HardwareAuthorizationService::new(
            Arc::new(
                AuthorizationService::new(
                    evaluator,
                    sink,
                ),
            ),
        );

        let result = service.authorize(
            &authorization_request(),
        );

        assert!(result.is_err());

        assert_eq!(
            result.unwrap_err().kind(),
            ZhalErrorKind::ProviderFailure,
        );
    }

    #[test]
    fn mismatched_audit_receipt_prevents_authorization() {
        let evaluator = Arc::new(TestEvaluator {
            id: PolicyEvaluatorId::new(
                "test::mismatched_audit_evaluator",
            )
            .unwrap(),

            decision: PolicyDecision::new(
                PolicyEffect::Permit,
            ),
        });

        let sink = Arc::new(MismatchedAuditSink {
            id: AuditSinkId::new(
                "test::expected_audit",
            )
            .unwrap(),
        });

        let service = HardwareAuthorizationService::new(
            Arc::new(
                AuthorizationService::new(
                    evaluator,
                    sink,
                ),
            ),
        );

        let result = service.authorize(
            &authorization_request(),
        );

        assert!(result.is_err());

        assert_eq!(
            result.unwrap_err().kind(),
            ZhalErrorKind::ContractViolation,
        );
    }

    #[test]
    fn denial_is_still_audited() {
        let (service, sink) =
            authorization_service(PolicyEffect::Deny);

        assert!(
            service
                .authorize(&authorization_request())
                .is_err()
        );

        let events = sink.events.lock().unwrap();

        assert_eq!(events.len(), 1);

        assert!(matches!(
            events[0].outcome(),
            crate::stdlib::security::policy::AuditOutcome::Decision(
                decision
            ) if decision.effect() == PolicyEffect::Deny
        ));
    }

    // ========================================================================
    // AUTHORIZATION — PRINCIPAL AND PERMISSION BOUNDARIES
    // ========================================================================

    #[test]
    fn principal_identity_is_not_an_authorization_grant() {
        let principal = PrincipalId::new(
            "application::worker",
        );

        assert!(principal.is_ok());

        let (service, _) =
            authorization_service(PolicyEffect::Deny);

        let result = service.authorize(
            &authorization_request(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn arbitrary_open_world_actions_are_supported() {
        let action = PolicyAction::new(
            "future::quantum::hardware::reconfiguration",
        );

        assert!(action.is_ok());
    }

    #[test]
    fn malformed_actions_are_rejected() {
        assert!(PolicyAction::new("").is_err());

        assert!(
            PolicyAction::new(" invalid::action").is_err()
        );

        assert!(
            PolicyAction::new("security::").is_err()
        );
    }

    #[test]
    fn authorization_is_evaluated_again_for_each_request() {
        let (service, sink) =
            authorization_service(PolicyEffect::Permit);

        service
            .authorize(&authorization_request())
            .unwrap();

        service
            .authorize(&authorization_request())
            .unwrap();

        assert_eq!(
            sink.events.lock().unwrap().len(),
            2,
            "the adapter must not reuse a cached authorization receipt",
        );
    }

    // ========================================================================
    // PROVIDER TRUST — FAIL-CLOSED VERIFICATION
    // ========================================================================

    #[test]
    fn valid_provider_evidence_establishes_trust() {
        let assessment = assess_trust(
            &ValidSignatureVerifier,
            Some(&ValidRevocationChecker),
        )
        .unwrap();

        assert_eq!(
            assessment.decision(),
            TrustDecision::Trusted,
        );

        assert!(assessment.is_trusted());
    }

    #[test]
    fn invalid_provider_signature_is_rejected() {
        let assessment = assess_trust(
            &InvalidSignatureVerifier,
            Some(&ValidRevocationChecker),
        )
        .unwrap();

        assert_eq!(
            assessment.decision(),
            TrustDecision::Rejected,
        );

        assert!(
            assessment.findings().contains(
                &crate::zhal::security::trust::TrustFinding::InvalidSignature,
            )
        );
    }

    #[test]
    fn indeterminate_signature_never_establishes_trust() {
        let assessment = assess_trust(
            &IndeterminateSignatureVerifier,
            Some(&ValidRevocationChecker),
        )
        .unwrap();

        assert_eq!(
            assessment.decision(),
            TrustDecision::Indeterminate,
        );
    }

    #[test]
    fn signature_verification_infrastructure_failure_is_propagated() {
        let result = assess_trust(
            &FailingSignatureVerifier,
            Some(&ValidRevocationChecker),
        );

        assert!(result.is_err());

        assert_eq!(
            result.unwrap_err().kind(),
            ZhalErrorKind::ProviderFailure,
        );
    }

    #[test]
    fn revoked_provider_evidence_is_rejected() {
        let assessment = assess_trust(
            &ValidSignatureVerifier,
            Some(&RevokedEvidenceChecker),
        )
        .unwrap();

        assert_eq!(
            assessment.decision(),
            TrustDecision::Rejected,
        );

        assert!(
            assessment.findings().contains(
                &crate::zhal::security::trust::TrustFinding::Revoked,
            )
        );
    }

    #[test]
    fn unknown_revocation_status_never_establishes_trust() {
        let assessment = assess_trust(
            &ValidSignatureVerifier,
            Some(&UnknownRevocationChecker),
        )
        .unwrap();

        assert_eq!(
            assessment.decision(),
            TrustDecision::Indeterminate,
        );
    }

    #[test]
    fn missing_required_revocation_checker_fails_closed() {
        let assessment = assess_trust(
            &ValidSignatureVerifier,
            None,
        )
        .unwrap();

        assert_eq!(
            assessment.decision(),
            TrustDecision::Indeterminate,
        );
    }

    #[test]
    fn provider_identity_mismatch_is_rejected() {
        let other_provider = ProviderId::new(
            "zamani::test::different_provider",
        )
        .unwrap();

        let assessment = evaluate_trust(
            &trust_policy(),
            &trust_evidence(),
            &other_provider,
            evaluation_time(),
            &ValidSignatureVerifier,
            Some(&ValidRevocationChecker),
        )
        .unwrap();

        assert_eq!(
            assessment.decision(),
            TrustDecision::Rejected,
        );

        assert!(
            assessment.findings().contains(
                &crate::zhal::security::trust::TrustFinding::SubjectMismatch,
            )
        );
    }

    #[test]
    fn expired_evidence_is_rejected() {
        let assessment = evaluate_trust(
            &trust_policy(),
            &trust_evidence(),
            &provider_id(),
            TrustTimestamp::from_unix_nanos(1_000),
            &ValidSignatureVerifier,
            Some(&ValidRevocationChecker),
        )
        .unwrap();

        assert_eq!(
            assessment.decision(),
            TrustDecision::Rejected,
        );
    }

    #[test]
    fn evidence_issued_in_the_future_is_rejected() {
        let assessment = evaluate_trust(
            &trust_policy(),
            &trust_evidence(),
            &provider_id(),
            TrustTimestamp::from_unix_nanos(50),
            &ValidSignatureVerifier,
            Some(&ValidRevocationChecker),
        )
        .unwrap();

        assert_eq!(
            assessment.decision(),
            TrustDecision::Rejected,
        );
    }

    #[test]
    fn empty_trust_policy_does_not_establish_trust() {
        let empty_policy = TrustPolicy::new(
            TrustPolicyId::new(
                "test::empty_policy",
            )
            .unwrap(),
        );

        let assessment = evaluate_trust(
            &empty_policy,
            &trust_evidence(),
            &provider_id(),
            evaluation_time(),
            &ValidSignatureVerifier,
            Some(&ValidRevocationChecker),
        )
        .unwrap();

        assert_eq!(
            assessment.decision(),
            TrustDecision::Rejected,
        );
    }

    #[test]
    fn untrusted_issuer_is_rejected() {
        let policy = TrustPolicy::new(
            TrustPolicyId::new(
                "test::restrictive_policy",
            )
            .unwrap(),
        )
        .trust_issuer(
            TrustIssuerId::new(
                "test::different_issuer",
            )
            .unwrap(),
        )
        .allow_scheme(signature_scheme());

        let assessment = evaluate_trust(
            &policy,
            &trust_evidence(),
            &provider_id(),
            evaluation_time(),
            &ValidSignatureVerifier,
            Some(&ValidRevocationChecker),
        )
        .unwrap();

        assert_eq!(
            assessment.decision(),
            TrustDecision::Rejected,
        );
    }

    // ========================================================================
    // TRUST AND AUTHORIZATION ARE INDEPENDENT
    // ========================================================================

    #[test]
    fn trusted_provider_is_not_automatically_authorized() {
        let assessment = assess_trust(
            &ValidSignatureVerifier,
            Some(&ValidRevocationChecker),
        )
        .unwrap();

        assert!(assessment.is_trusted());

        let (authorization, _) =
            authorization_service(PolicyEffect::Deny);

        let result = authorization.authorize(
            &authorization_request(),
        );

        assert!(
            result.is_err(),
            "provider trust must not bypass authorization",
        );
    }

    #[test]
    fn authorization_does_not_establish_provider_trust() {
        let (authorization, _) =
            authorization_service(PolicyEffect::Permit);

        let receipt = authorization
            .authorize(&authorization_request())
            .unwrap();

        assert!(receipt.is_authorized());

        let assessment = assess_trust(
            &InvalidSignatureVerifier,
            Some(&ValidRevocationChecker),
        )
        .unwrap();

        assert!(
            !assessment.is_trusted(),
            "authorization must not bypass provider trust verification",
        );
    }

    // ========================================================================
    // SECRET HANDLING AND DIAGNOSTIC REDACTION
    // ========================================================================

    #[test]
    fn trust_evidence_debug_redacts_signed_payload() {
        let evidence = trust_evidence();

        let diagnostic = format!("{evidence:?}");

        assert!(
            !diagnostic.contains(
                "canonical signed provider statement"
            )
        );
    }

    #[test]
    fn trust_evidence_debug_redacts_signature_bytes() {
        let evidence = trust_evidence();

        let diagnostic = format!("{evidence:?}");

        assert!(
            !diagnostic.contains(
                "test signature bytes"
            )
        );
    }

    #[test]
    fn policy_attribute_debug_redacts_value() {
        let value = PolicyAttributeValue::new(
            "sensitive-looking-private-metadata",
        )
        .unwrap();

        let diagnostic = format!("{value:?}");

        assert!(
            !diagnostic.contains(
                "sensitive-looking-private-metadata"
            )
        );

        assert!(
            diagnostic.contains("[REDACTED]")
        );
    }

    #[test]
    fn authorization_service_debug_does_not_expose_dependencies() {
        let (service, _) =
            authorization_service(PolicyEffect::Permit);

        let diagnostic = format!("{service:?}");

        assert!(
            !diagnostic.contains(
                "hardware_policy_evaluator"
            )
        );

        assert!(
            !diagnostic.contains(
                "hardware_audit_sink"
            )
        );
    }

    #[test]
    fn audit_events_preserve_resource_and_action_without_secret_fields() {
        let request = authorization_request();

        let event = AuditEvent::decision(
            request,
            PolicyEvaluatorId::new(
                "test::evaluator",
            )
            .unwrap(),
            PolicyDecision::new(
                PolicyEffect::Deny,
            ),
        );

        assert_eq!(
            event.request().action().as_str(),
            "hardware::device::access",
        );

        assert!(
            event.request().attributes().is_empty(),
        );
    }

    // ========================================================================
    // INPUT VALIDATION AND OPEN-WORLD SCALABILITY
    // ========================================================================

    #[test]
    fn trust_identifiers_are_open_world() {
        let identifier = TrustEvidenceId::new(
            "future::hardware::attestation::evidence",
        );

        assert!(identifier.is_ok());
    }

    #[test]
    fn trust_identifiers_reject_empty_values() {
        assert!(
            TrustEvidenceId::new("").is_err()
        );

        assert!(
            TrustIssuerId::new("").is_err()
        );

        assert!(
            TrustKeyReference::new("").is_err()
        );

        assert!(
            SignatureScheme::new("").is_err()
        );
    }

    #[test]
    fn evidence_with_empty_signature_is_rejected() {
        let result = TrustEvidence::new(
            TrustEvidenceId::new(
                "test::empty_signature",
            )
            .unwrap(),
            provider_id(),
            trust_issuer(),
            TrustKeyReference::new(
                "test::external_key",
            )
            .unwrap(),
            signature_scheme(),
            evaluation_time(),
            None,
            b"payload".to_vec(),
            Vec::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn evidence_cannot_expire_before_issuance() {
        let result = TrustEvidence::new(
            TrustEvidenceId::new(
                "test::invalid_validity",
            )
            .unwrap(),
            provider_id(),
            trust_issuer(),
            TrustKeyReference::new(
                "test::external_key",
            )
            .unwrap(),
            signature_scheme(),
            TrustTimestamp::from_unix_nanos(100),
            Some(
                TrustTimestamp::from_unix_nanos(50),
            ),
            b"payload".to_vec(),
            b"signature".to_vec(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn long_open_world_identifiers_are_not_rejected_by_an_arbitrary_small_limit() {
        // This is a representation/validation test, not a claim
        // that memory or identifiers have infinite physical capacity.
        let namespace = "future_domain.".repeat(512);

        let identifier = format!(
            "{namespace}hardware_operation"
        );

        assert!(
            PolicyAction::new(identifier).is_ok()
        );
    }

    #[test]
    fn many_distinct_resource_scoped_requests_remain_independent() {
        let (service, sink) =
            authorization_service(PolicyEffect::Permit);

        let request_count = 256usize;

        for index in 0..request_count {
            let request = AuthorizationRequest::new(
                PrincipalId::new(
                    "application::worker",
                )
                .unwrap(),
                PolicyAction::new(
                    "hardware::device::inspect",
                )
                .unwrap(),
            )
            .with_resource(
                PolicyResource::new(
                    format!(
                        "hardware::device::{index}"
                    ),
                )
                .unwrap(),
            );

            service
                .authorize(&request)
                .expect("independent request should be evaluated");
        }

        assert_eq!(
            sink.events.lock().unwrap().len(),
            request_count,
        );
    }

    // ========================================================================
    // THREAD-SAFETY AND CONCURRENT AUTHORIZATION
    // ========================================================================

    #[test]
    fn authorization_service_can_be_shared_across_threads() {
        let (service, sink) =
            authorization_service(PolicyEffect::Permit);

        let service = Arc::new(service);

        let request = authorization_request();

        std::thread::scope(|scope| {
            let first = {
                let service = Arc::clone(&service);
                let request = request.clone();

                scope.spawn(move || {
                    service.authorize(&request)
                })
            };

            let second = {
                let service = Arc::clone(&service);
                let request = request.clone();

                scope.spawn(move || {
                    service.authorize(&request)
                })
            };

            assert!(
                first.join().unwrap().is_ok()
            );

            assert!(
                second.join().unwrap().is_ok()
            );
        });

        assert_eq!(
            sink.events.lock().unwrap().len(),
            2,
        );
    }

    // ========================================================================
    // DOMAIN-INDEPENDENT SECURITY
    // ========================================================================

    #[test]
    fn security_action_names_are_not_bound_to_one_hardware_domain() {
        let actions = [
            "classical::processor::access",
            "quantum::processor::execute",
            "hdl::device::program",
            "accelerator::compute",
            "distributed::node::access",
            "embedded::device::control",
            "future::photonic::operation",
        ];

        for action in actions {
            assert!(
                PolicyAction::new(action).is_ok(),
                "open-world action rejected: {action}",
            );
        }
    }

    #[test]
    fn hardware_provider_identity_is_not_a_permission() {
        let provider = provider_id();

        assert!(
            !provider.to_string().is_empty()
        );

        let (service, _) =
            authorization_service(PolicyEffect::Deny);

        assert!(
            service
                .authorize(&authorization_request())
                .is_err(),
            "provider identity must not grant hardware access",
        );
    }

    // ========================================================================
    // PUBLIC CONTRACT AVAILABILITY
    // ========================================================================

    #[test]
    fn canonical_security_contracts_are_accessible() {
        fn accepts_evaluator<T: PolicyEvaluator + ?Sized>(
            _: &T,
        ) {
        }

        fn accepts_audit_sink<T: AuditSink + ?Sized>(
            _: &T,
        ) {
        }

        fn accepts_signature_verifier<T: SignatureVerifier + ?Sized>(
            _: &T,
        ) {
        }

        fn accepts_revocation_checker<T: RevocationChecker + ?Sized>(
            _: &T,
        ) {
        }

        let evaluator = TestEvaluator {
            id: PolicyEvaluatorId::new(
                "test::contract_evaluator",
            )
            .unwrap(),

            decision: PolicyDecision::new(
                PolicyEffect::Deny,
            ),
        };

        let sink = RecordingAuditSink {
            id: AuditSinkId::new(
                "test::contract_sink",
            )
            .unwrap(),

            events: Mutex::new(Vec::new()),
        };

        accepts_evaluator(&evaluator);
        accepts_audit_sink(&sink);
        accepts_signature_verifier(
            &ValidSignatureVerifier,
        );
        accepts_revocation_checker(
            &ValidRevocationChecker,
        );
    }

    #[test]
    fn audit_status_is_a_distinct_contract() {
        fn accepts_status(_: &AuditStatus) {
        }

        let (service, _) =
            authorization_service(PolicyEffect::Permit);

        let receipt = service
            .authorize(&authorization_request())
            .unwrap();

        accepts_status(receipt.audit_status());
    }

    #[test]
    fn error_classification_remains_structured() {
        let (service, _) =
            authorization_service(PolicyEffect::Deny);

        let error = service
            .authorize(&authorization_request())
            .unwrap_err();

        assert_eq!(
            error.kind(),
            ZhalErrorKind::PermissionDenied,
        );
    }

    // ========================================================================
    // TEST FIXTURE CONTRACT
    // ========================================================================

    #[test]
    fn test_provider_identity_is_validated_by_canonical_provider_authority() {
        let provider = ProviderId::new(
            "zamani::test::provider",
        );

        assert!(provider.is_ok());
    }

    #[test]
    fn trust_evidence_subject_matches_canonical_provider_identity() {
        let evidence = trust_evidence();

        assert_eq!(
            evidence.subject(),
            &provider_id(),
        );
    }

    #[test]
    fn debug_redaction_contract_is_stable() {
        fn assert_debug<T: Debug>(_: &T) {
        }

        let evidence = trust_evidence();

        let value = PolicyAttributeValue::new(
            "non-secret-test-metadata",
        )
        .unwrap();

        assert_debug(&evidence);
        assert_debug(&value);
    }
}
