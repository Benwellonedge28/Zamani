
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani AI Security Integration Tests
//!
//! Path: src/ai/tests/security.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! These tests verify the integration contracts of the existing
//! AI security subsystem.
//!
//! They deliberately reuse the canonical implementations instead
//! of creating duplicate security infrastructure.
//!
//! ## Canonical authorities
//!
//! - ai::security::identity
//! - ai::security::authorization
//! - ai::security::policies
//! - ai::security::isolation
//! - ai::security::integrity
//! - ai::security::audit
//! - ai::agents::policies
//! - ai::ids
//! - backend::artifact
//! - backend::ids
//! - distributed::security
//! - frontend::semantic
//!
//! ## Grammar authorities
//!
//! - grammar/ai/ai.g4
//! - grammar/security/security.g4
//! - grammar/security/authorization.g4
//! - grammar/security/audit.g4
//! - grammar/security/sandbox.g4
//! - grammar/security/security-constraints.g4
//! - grammar/security/identifiers.g4
//! - grammar/security/policies.g4
//!
//! ## Testing boundary
//!
//! These tests verify the AI security domain and its integration
//! contracts. They do not pretend to test a complete frontend,
//! distributed authentication provider, operating-system sandbox,
//! or production audit database.
//!
//! Those require their respective integration environments.

use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};

// ============================================================================
// CANONICAL AI IDENTIFIERS
// ============================================================================

use crate::ai::ids::{
    AiAgentId,
    AiArtifactId,
    AiDatasetId,
    AiModelId,
    AiPipelineId,
    AiTrainingRunId,
};

// ============================================================================
// AI SECURITY IDENTITY
// ============================================================================

use crate::ai::security::identity::{
    AiEntityRef,
    AiExecutionRef,
    AiIdentityRef,
    AiPrincipalRef,
    AiPrincipalSubject,
    ExternalSubjectId,
    IdentityAuthority,
};

// ============================================================================
// AI AUTHORIZATION
// ============================================================================

use crate::ai::security::authorization::{
    AiAuthorizationAction,
    AiAuthorizationRequest,
};

// ============================================================================
// AI POLICY INTEGRATION
// ============================================================================

use crate::ai::agents::policies::AgentPolicyReference;

use crate::ai::security::policies::{
    AiSecurityPolicyBinding,
    AiSecurityPolicyBindingId,
    AiSecurityPolicyScope,
    AiSecurityPolicySet,
    AiSecurityPolicySubject,
    AiSecurityPolicyTarget,
};

// ============================================================================
// AI ISOLATION
// ============================================================================

use crate::ai::security::isolation::{
    AiIsolationBoundary,
    AiIsolationBoundaryId,
    AiIsolationControlId,
    AiIsolationDisposition,
    AiIsolationPlan,
    AiIsolationRequirement,
    AiIsolationSubject,
};

// ============================================================================
// AI INTEGRITY
// ============================================================================

use crate::ai::security::integrity::{
    refresh_artifact_digest,
    verify_artifact,
    AiContentDigest,
    AiIntegrityError,
    AiIntegrityPolicy,
    AiIntegrityService,
};

// ============================================================================
// BACKEND ARTIFACTS
// ============================================================================

use crate::backend::artifact::{
    ArtifactPayload,
    OutputArtifact,
};

use crate::backend::ids::{
    ArtifactId,
    IdNamespace,
};

// ============================================================================
// AI AUDIT
// ============================================================================

use crate::ai::security::audit::{
    AiAuditCategory,
    AiAuditDecision,
    AiAuditDurability,
    AiAuditError,
    AiAuditEvent,
    AiAuditEventId,
    AiAuditOutcome,
    AiAuditReceipt,
    AiAuditService,
    AiAuditSink,
    AiAuditTimestamp,
    AiAuditFieldName,
    AiAuditFieldValue,
};

// ============================================================================
// SHARED TEST FIXTURES
// ============================================================================

fn authority() -> IdentityAuthority {
    IdentityAuthority::new("zamani::security::tests")
        .expect("valid identity authority")
}

fn model_identity(value: u128) -> AiIdentityRef {
    AiIdentityRef::new(
        authority(),
        AiEntityRef::Model(
            AiModelId::new(value)
                .expect("nonzero model identity"),
        ),
    )
}

fn agent_identity(value: u128) -> AiIdentityRef {
    AiIdentityRef::new(
        authority(),
        AiEntityRef::Agent(
            AiAgentId::new(value)
                .expect("nonzero agent identity"),
        ),
    )
}

fn principal() -> AiPrincipalRef {
    AiPrincipalRef::agent(
        authority(),
        AiAgentId::new(1)
            .expect("valid agent identity"),
    )
}

fn execution_identity() -> AiExecutionRef {
    AiExecutionRef::new(
        authority(),
        ExternalSubjectId::new(
            "execution::security-test",
        )
        .expect("valid execution identity"),
    )
}

fn authorization_request() -> AiAuthorizationRequest {
    AiAuthorizationRequest::new(
        AiAuthorizationAction::new(
            "ai.model.infer",
        )
        .expect("valid AI authorization action"),
        model_identity(42),
    )
}

fn policy_reference() -> AgentPolicyReference {
    AgentPolicyReference::new(
        "zamani.ai.security",
        "model_access",
    )
    .expect("valid canonical policy reference")
}

fn policy_binding(
    id: &str,
) -> AiSecurityPolicyBinding {
    let scope = AiSecurityPolicyScope::new(
        AiSecurityPolicySubject::Principal(
            principal(),
        ),
        AiSecurityPolicyTarget::action_on_resource(
            "ai.model.infer",
            model_identity(42),
        )
        .expect("valid policy target"),
    )
    .expect("valid policy scope");

    AiSecurityPolicyBinding::new(
        AiSecurityPolicyBindingId::new(id)
            .expect("valid binding identity"),
        policy_reference(),
        scope,
    )
    .expect("valid AI policy binding")
}

fn isolation_boundary(
    id: &str,
) -> AiIsolationBoundary {
    let mut boundary = AiIsolationBoundary::new(
        AiIsolationBoundaryId::new(id)
            .expect("valid boundary identity"),
        AiIsolationSubject::Entity(
            model_identity(42),
        ),
    );

    boundary.add_requirement(
        AiIsolationRequirement::new(
            AiIsolationControlId::new(
                "memory.domain",
            )
            .expect("valid isolation control"),
            AiIsolationDisposition::Required,
        ),
    );

    boundary
}

// ============================================================================
// 1. IDENTITY SECURITY
// ============================================================================

#[test]
fn identity_is_not_authentication() {
    let identity = model_identity(42);

    assert_eq!(identity.local_id(), 42);

    // An identity identifies an entity.
    // It does not establish an authenticated session.
    //
    // Authentication remains owned by distributed/security.
    assert_eq!(
        identity.authority().as_str(),
        "zamani::security::tests",
    );
}

#[test]
fn agent_identity_is_distinct_from_model_identity() {
    let model = model_identity(7);
    let agent = agent_identity(7);

    assert_ne!(model, agent);

    assert_ne!(
        model.category(),
        agent.category(),
    );
}

#[test]
fn identity_authority_is_part_of_identity_equality() {
    let first = AiIdentityRef::new(
        IdentityAuthority::new("organization::one")
            .unwrap(),
        AiEntityRef::Model(
            AiModelId::new(7).unwrap(),
        ),
    );

    let second = AiIdentityRef::new(
        IdentityAuthority::new("organization::two")
            .unwrap(),
        AiEntityRef::Model(
            AiModelId::new(7).unwrap(),
        ),
    );

    assert_ne!(first, second);
}

#[test]
fn execution_identity_is_distinct_from_agent_identity() {
    let execution = execution_identity();

    let agent = principal();

    assert!(agent.is_agent());

    assert!(!agent.is_execution());

    assert_eq!(
        execution.subject().as_str(),
        "execution::security-test",
    );
}

#[test]
fn identity_deserialization_rejects_invalid_authorities() {
    let result = serde_json::from_str::<IdentityAuthority>(
        "\" \"",
    );

    assert!(result.is_err());
}

#[test]
fn external_subject_deserialization_rejects_empty_subjects() {
    let result = serde_json::from_str::<ExternalSubjectId>(
        "\"\"",
    );

    assert!(result.is_err());
}

#[test]
fn identity_serialization_preserves_authority_and_category() {
    let original = model_identity(u128::MAX);

    let serialized = serde_json::to_string(&original)
        .expect("identity serialization");

    let restored: AiIdentityRef =
        serde_json::from_str(&serialized)
            .expect("identity deserialization");

    assert_eq!(restored, original);
    assert_eq!(restored.local_id(), u128::MAX);
}

// ============================================================================
// 2. AUTHORIZATION SECURITY
// ============================================================================

#[test]
fn authorization_actions_require_ai_namespace() {
    assert!(
        AiAuthorizationAction::new(
            "ai.model.infer",
        )
        .is_ok()
    );

    assert!(
        AiAuthorizationAction::new(
            "ai.future.quantum_ml.execute",
        )
        .is_ok()
    );

    assert!(
        AiAuthorizationAction::new(
            "system.model.infer",
        )
        .is_err()
    );
}

#[test]
fn authorization_rejects_empty_action_segments() {
    for invalid in [
        "",
        "ai.",
        "ai..model",
        "ai.model.",
        "ai model infer",
    ] {
        assert!(
            AiAuthorizationAction::new(invalid).is_err(),
            "unexpected acceptance of {invalid:?}",
        );
    }
}

#[test]
fn authorization_requests_preserve_resource_identity() {
    let request = authorization_request();

    assert_eq!(
        request.action().as_str(),
        "ai.model.infer",
    );

    assert_eq!(
        request.resource().local_id(),
        42,
    );

    assert_eq!(
        request.resource().category(),
        crate::ai::security::identity::AiIdentityCategory::Model,
    );
}

#[test]
fn authorization_request_does_not_manufacture_authenticated_subject() {
    let request = authorization_request();

    assert_eq!(
        request.action().as_str(),
        "ai.model.infer",
    );

    // The request carries an action and resource.
    //
    // It does not contain an authenticated principal.
    // The distributed authentication service supplies
    // the authenticated subject.
}

#[test]
fn future_ai_authorization_domains_remain_open_world() {
    let actions = [
        "ai.neuromorphic.execute",
        "ai.quantum_ml.train",
        "ai.swarm.coordinate",
        "ai.probabilistic.infer",
        "ai.future_computing.execute",
    ];

    for action in actions {
        assert!(
            AiAuthorizationAction::new(action).is_ok(),
            "valid future AI action rejected: {action}",
        );
    }
}

// ============================================================================
// 3. POLICY SECURITY
// ============================================================================

#[test]
fn policy_bindings_require_explicit_subject_and_target() {
    let binding = policy_binding("model_access");

    assert!(
        binding.validate().is_ok()
    );

    assert_eq!(
        binding.id().as_str(),
        "model_access",
    );
}

#[test]
fn policy_binding_does_not_grant_permission() {
    let binding = policy_binding("declarative_policy");

    assert!(
        binding.validate().is_ok()
    );

    // A binding merely references a canonical policy.
    // Actual authorization requires distributed policy evaluation.
}

#[test]
fn broad_policy_subject_must_be_explicit() {
    let scope = AiSecurityPolicyScope::new(
        AiSecurityPolicySubject::AllPrincipals,
        AiSecurityPolicyTarget::all_ai_operations(),
    );

    assert!(scope.is_ok());
}

#[test]
fn policy_target_rejects_non_ai_actions() {
    assert!(
        AiSecurityPolicyTarget::action(
            "system.execute",
        )
        .is_err()
    );

    assert!(
        AiSecurityPolicyTarget::action(
            "ai.model.infer",
        )
        .is_ok()
    );
}

#[test]
fn policy_set_rejects_duplicate_binding_identifiers() {
    let mut policies = AiSecurityPolicySet::new();

    policies
        .attach(policy_binding("same_binding"))
        .expect("first binding");

    let result = policies.attach(
        policy_binding("same_binding"),
    );

    assert!(result.is_err());

    assert_eq!(policies.len(), 1);
}

#[test]
fn policy_set_preserves_deterministic_order() {
    let mut policies = AiSecurityPolicySet::new();

    policies.attach(policy_binding("zeta")).unwrap();
    policies.attach(policy_binding("alpha")).unwrap();
    policies.attach(policy_binding("middle")).unwrap();

    let ids: Vec<&str> = policies
        .iter()
        .map(|binding| binding.id().as_str())
        .collect();

    assert_eq!(
        ids,
        vec!["alpha", "middle", "zeta"],
    );
}

#[test]
fn policy_set_serialization_preserves_bindings() {
    let mut original = AiSecurityPolicySet::new();

    original.attach(
        policy_binding("first"),
    ).unwrap();

    original.attach(
        policy_binding("second"),
    ).unwrap();

    let encoded = serde_json::to_string(&original)
        .expect("policy serialization");

    let restored: AiSecurityPolicySet =
        serde_json::from_str(&encoded)
            .expect("policy deserialization");

    assert_eq!(restored, original);
    assert_eq!(restored.len(), 2);

    assert!(restored.validate().is_ok());
}

#[test]
fn malformed_policy_binding_identifiers_are_rejected() {
    let invalid = r#"{
        "bindings": {
            "invalid": {
                "id": " ",
                "policy": {
                    "namespace": "zamani.ai",
                    "name": "model_access"
                },
                "scope": {
                    "subject": "all_principals",
                    "target": "all_ai_operations"
                },
                "condition": null,
                "arguments": {},
                "metadata": {}
            }
        }
    }"#;

    let result = serde_json::from_str::<AiSecurityPolicySet>(
        invalid,
    );

    assert!(result.is_err());
}

// ============================================================================
// 4. ISOLATION SECURITY
// ============================================================================

#[test]
fn isolation_requirements_preserve_explicit_dispositions() {
    let mut boundary = isolation_boundary("model_boundary");

    boundary.add_requirement(
        AiIsolationRequirement::new(
            AiIsolationControlId::new(
                "network.external_access",
            )
            .unwrap(),
            AiIsolationDisposition::Prohibited,
        ),
    );

    boundary.add_requirement(
        AiIsolationRequirement::new(
            AiIsolationControlId::new(
                "memory.domain",
            )
            .unwrap(),
            AiIsolationDisposition::Preferred,
        ),
    );

    assert_eq!(boundary.requirements().len(), 3);

    assert_eq!(
        boundary.requirements()[0].disposition(),
        AiIsolationDisposition::Required,
    );

    assert_eq!(
        boundary.requirements()[1].disposition(),
        AiIsolationDisposition::Prohibited,
    );

    assert_eq!(
        boundary.requirements()[2].disposition(),
        AiIsolationDisposition::Preferred,
    );
}

#[test]
fn isolation_plan_rejects_duplicate_boundaries() {
    let mut plan = AiIsolationPlan::new();

    plan.insert(
        isolation_boundary("boundary_one"),
    )
    .expect("first boundary");

    assert!(
        plan.insert(
            isolation_boundary("boundary_one"),
        )
        .is_err()
    );

    assert_eq!(plan.len(), 1);
}

#[test]
fn isolation_plan_preserves_deterministic_order() {
    let mut plan = AiIsolationPlan::new();

    plan.insert(isolation_boundary("zeta")).unwrap();
    plan.insert(isolation_boundary("alpha")).unwrap();
    plan.insert(isolation_boundary("middle")).unwrap();

    let ids: Vec<&str> = plan
        .iter()
        .map(|boundary| boundary.id().as_str())
        .collect();

    assert_eq!(
        ids,
        vec!["alpha", "middle", "zeta"],
    );
}

#[test]
fn isolation_plan_supports_execution_subjects() {
    let boundary = AiIsolationBoundary::new(
        AiIsolationBoundaryId::new(
            "execution_isolation",
        )
        .unwrap(),
        AiIsolationSubject::Execution(
            execution_identity(),
        ),
    );

    assert!(boundary.validate().is_ok());

    assert!(matches!(
        boundary.subject(),
        AiIsolationSubject::Execution(_),
    ));
}

#[test]
fn isolation_plan_serialization_preserves_requirements() {
    let mut plan = AiIsolationPlan::new();

    plan.insert(
        isolation_boundary("serialized_boundary"),
    )
    .unwrap();

    let encoded = serde_json::to_string(&plan)
        .expect("isolation serialization");

    let restored: AiIsolationPlan =
        serde_json::from_str(&encoded)
            .expect("isolation deserialization");

    assert_eq!(restored, plan);

    assert!(restored.validate().is_ok());
}

#[test]
fn isolation_does_not_claim_actual_enforcement() {
    let boundary = isolation_boundary("declarative_only");

    assert!(boundary.validate().is_ok());

    // Structural validity is not evidence of an operating-system
    // sandbox, container, memory isolation or runtime enforcement.
}

// ============================================================================
// 5. SCALABILITY AND OPEN-WORLD SECURITY
// ============================================================================

#[test]
fn isolation_collections_support_dynamic_growth() {
    let mut plan = AiIsolationPlan::new();

    // This is a test workload, not an architectural maximum.
    for index in 1_u128..=512 {
        plan.insert(
            isolation_boundary(
                &format!("boundary_{index}"),
            ),
        )
        .expect("unique isolation boundary");
    }

    assert_eq!(plan.len(), 512);

    assert!(plan.validate().is_ok());
}

#[test]
fn policy_collections_support_dynamic_growth() {
    let mut policies = AiSecurityPolicySet::new();

    for index in 1_u128..=512 {
        policies
            .attach(policy_binding(
                &format!("binding_{index}"),
            ))
            .expect("unique policy binding");
    }

    assert_eq!(policies.len(), 512);

    assert!(policies.validate().is_ok());
}

#[test]
fn security_identifiers_do_not_impose_arbitrary_length_limits() {
    let long_identifier = "future_security_domain".repeat(1024);

    assert!(
        AiIsolationBoundaryId::new(
            long_identifier.clone(),
        )
        .is_ok()
    );

    assert!(
        AiIsolationControlId::new(
            long_identifier,
        )
        .is_ok()
    );
}

#[test]
fn maximum_ai_entity_identifier_is_preserved() {
    let identity = model_identity(u128::MAX);

    assert_eq!(
        identity.local_id(),
        u128::MAX,
    );
}

#[test]
fn future_isolation_controls_are_supported() {
    let controls = [
        "future.quantum.execution.isolation",
        "neuromorphic.memory.protection",
        "distributed.model.tenant_boundary",
        "future.accelerator.shared_state",
    ];

    for control in controls {
        assert!(
            AiIsolationControlId::new(control).is_ok()
        );
    }
}

#[test]
fn security_does_not_encode_hardware_identity() {
    let request = authorization_request();

    let serialized = serde_json::to_string(&request)
        .expect("request serialization");

    assert!(
        !serialized.contains("gpu_0")
    );

    assert!(
        !serialized.contains("cpu_0")
    );

    assert!(
        !serialized.contains("qpu_0")
    );
}

// ============================================================================
// 6. ARTIFACT INTEGRITY
// ============================================================================

fn artifact() -> OutputArtifact {
    let namespace = IdNamespace::new(
        "zamani.security.tests",
    )
    .expect("valid backend identifier namespace");

    let id = ArtifactId::new(namespace, 1)
        .expect("valid artifact identity");

    OutputArtifact::new(
        id,
        "target-independent",
        "ai-model",
        ArtifactPayload::binary(
            b"canonical-ai-artifact-payload".to_vec(),
        ),
    )
    .expect("valid backend artifact")
}

#[test]
fn strict_integrity_policy_requires_digest_and_content_matching() {
    let policy = AiIntegrityPolicy::strict();

    assert!(policy.validate().is_ok());

    assert!(policy.requires_recorded_digest());

    assert!(policy.requires_content_match());
}

#[test]
fn artifact_integrity_requires_a_recorded_digest() {
    let value = artifact();

    let result = verify_artifact(&value);

    assert!(
        matches!(
            result,
            Err(AiIntegrityError::MissingDigest)
        )
    );
}

#[test]
fn canonical_artifact_digest_is_verified() {
    let mut value = artifact();

    let evidence = refresh_artifact_digest(
        &mut value,
    )
    .expect("canonical artifact integrity verification");

    assert!(evidence.is_verified());

    assert_eq!(
        evidence.algorithm(),
        "sha256",
    );

    assert_eq!(
        evidence.payload_size_bytes(),
        value.size_bytes() as u64,
    );
}

#[test]
fn artifact_tampering_is_detected() {
    let mut value = artifact();

    refresh_artifact_digest(
        &mut value,
    )
    .expect("initial digest");

    // Simulate modification after the original digest was recorded.
    value.payload = ArtifactPayload::binary(
        b"tampered-artifact-payload".to_vec(),
    );

    assert!(
        verify_artifact(&value).is_err()
    );
}

#[test]
fn identical_artifacts_produce_identical_digests() {
    let mut first = artifact();
    let mut second = artifact();

    let first_evidence = refresh_artifact_digest(
        &mut first,
    )
    .unwrap();

    let second_evidence = refresh_artifact_digest(
        &mut second,
    )
    .unwrap();

    assert_eq!(
        first_evidence.digest(),
        second_evidence.digest(),
    );
}

#[test]
fn different_payloads_produce_different_digests() {
    let mut first = artifact();
    let mut second = artifact();

    refresh_artifact_digest(&mut first).unwrap();

    second.payload = ArtifactPayload::binary(
        b"different-payload".to_vec(),
    );

    refresh_artifact_digest(&mut second).unwrap();

    assert_ne!(
        first.content_digest,
        second.content_digest,
    );
}

#[test]
fn malformed_integrity_digests_are_rejected() {
    for digest in [
        "sha256:invalid",
        "sha256:ABC",
        "sha256:",
        "md5:00000000000000000000000000000000",
    ] {
        assert!(
            AiContentDigest::new(digest).is_err(),
            "malformed digest accepted: {digest}",
        );
    }
}

#[test]
fn integrity_verification_does_not_claim_authenticity() {
    let mut value = artifact();

    let evidence = refresh_artifact_digest(
        &mut value,
    )
    .unwrap();

    assert!(evidence.is_verified());

    // Digest verification establishes content matching.
    // It does not establish who created the artifact,
    // whether the producer is trusted, or whether execution is authorized.
}

// ============================================================================
// 7. SECURITY AUDIT TEST INFRASTRUCTURE
// ============================================================================

#[derive(Default)]
struct MemoryAuditSink {
    events: Mutex<Vec<AiAuditEvent>>,
    durability: Option<AiAuditDurability>,
    fail: bool,
}

impl AiAuditSink for MemoryAuditSink {
    fn append(
        &self,
        event: &AiAuditEvent,
    ) -> Result<AiAuditReceipt, AiAuditError> {
        if self.fail {
            return Err(
                AiAuditError::StorageFailure {
                    message: "simulated audit persistence failure".to_owned(),
                },
            );
        }

        let mut events = self.events
            .lock()
            .map_err(|_| {
                AiAuditError::StorageFailure {
                    message: "audit test mutex poisoned".to_owned(),
                }
            })?;

        events.push(event.clone());

        AiAuditReceipt::new(
            event.event_id().clone(),
            "test.audit.storage",
            self.durability.unwrap_or(
                AiAuditDurability::Accepted,
            ),
        )
    }
}

fn audit_event() -> AiAuditEvent {
    AiAuditEvent::new(
        AiAuditEventId::new("audit_security_event_1")
            .expect("valid event identifier"),
        AiAuditTimestamp::new(
            1_800_000_000,
            0,
        )
        .expect("valid explicit timestamp"),
        AiAuditCategory::Authorization,
        "ai.model.infer",
        principal(),
        AiAuditDecision::Deny,
        AiAuditOutcome::Rejected,
    )
    .expect("valid audit event")
}

// ============================================================================
// 8. AUDIT SECURITY
// ============================================================================

#[test]
fn audit_event_preserves_denied_authorization() {
    let event = audit_event();

    assert_eq!(
        event.decision(),
        AiAuditDecision::Deny,
    );

    assert_eq!(
        event.outcome(),
        AiAuditOutcome::Rejected,
    );
}

#[test]
fn audit_event_preserves_identity_and_resource_separately() {
    let event = audit_event()
        .with_resource(model_identity(42));

    assert_eq!(
        event.resource().unwrap().local_id(),
        42,
    );

    assert_eq!(
        event.principal(),
        &principal(),
    );
}

#[test]
fn audit_event_rejects_invalid_action_whitespace() {
    let result = AiAuditEvent::new(
        AiAuditEventId::new("invalid_action_event").unwrap(),
        AiAuditTimestamp::new(100, 0).unwrap(),
        AiAuditCategory::Model,
        "ai.model infer",
        principal(),
        AiAuditDecision::NotEvaluated,
        AiAuditOutcome::Unknown,
    );

    assert!(result.is_err());
}

#[test]
fn audit_metadata_rejects_secret_bearing_field_names() {
    for field in [
        "password",
        "access_token",
        "refresh_token",
        "private_key",
        "client_secret",
        "authorization_header",
        "api_key",
        "recovery_secret",
    ] {
        assert!(
            AiAuditFieldName::new(field).is_err(),
            "sensitive field accepted: {field}",
        );
    }
}

#[test]
fn audit_metadata_allows_open_world_descriptive_fields() {
    for field in [
        "execution.domain",
        "ai.model.version",
        "quantum.execution.mode",
        "future.extension.identifier",
    ] {
        assert!(
            AiAuditFieldName::new(field).is_ok()
        );
    }
}

#[test]
fn audit_metadata_rejects_control_characters() {
    assert!(
        AiAuditFieldValue::new(
            "invalid\nmetadata",
        )
        .is_err()
    );
}

#[test]
fn audit_event_does_not_implicitly_grant_permission() {
    let event = audit_event();

    assert_eq!(
        event.decision(),
        AiAuditDecision::Deny,
    );

    // An audit event records a decision.
    // It cannot modify that decision or create permission.
}

#[test]
fn audit_event_serialization_preserves_security_decision() {
    let original = audit_event();

    let encoded = serde_json::to_string(&original)
        .expect("audit event serialization");

    assert!(
        encoded.contains("deny")
    );

    assert!(
        encoded.contains("rejected")
    );

    let value: serde_json::Value =
        serde_json::from_str(&encoded)
            .expect("valid audit JSON");

    assert!(
        value.get("schema_version").is_some()
    );
}

// ============================================================================
// 9. AUDIT PERSISTENCE AND FAILURE HANDLING
// ============================================================================

#[test]
fn audit_service_records_events() {
    let sink = Arc::new(MemoryAuditSink {
        durability: Some(AiAuditDurability::Durable),
        ..Default::default()
    });

    let service = AiAuditService::new(
        sink.clone(),
    );

    let receipt = service
        .record(&audit_event())
        .expect("audit append");

    assert_eq!(
        receipt.durability(),
        AiAuditDurability::Durable,
    );

    let events = sink.events.lock().unwrap();

    assert_eq!(events.len(), 1);

    assert_eq!(
        events[0].decision(),
        AiAuditDecision::Deny,
    );
}

#[test]
fn durable_audit_requires_durable_acknowledgement() {
    let sink = Arc::new(MemoryAuditSink {
        durability: Some(AiAuditDurability::Durable),
        ..Default::default()
    });

    let service = AiAuditService::new(sink);

    let result = service.record_durable(
        &audit_event(),
    );

    assert!(result.is_ok());
}

#[test]
fn replicated_audit_satisfies_durable_requirement() {
    let sink = Arc::new(MemoryAuditSink {
        durability: Some(AiAuditDurability::Replicated),
        ..Default::default()
    });

    let service = AiAuditService::new(sink);

    assert!(
        service.record_durable(
            &audit_event(),
        ).is_ok()
    );
}

#[test]
fn accepted_audit_does_not_claim_durable_persistence() {
    let sink = Arc::new(MemoryAuditSink {
        durability: Some(AiAuditDurability::Accepted),
        ..Default::default()
    });

    let service = AiAuditService::new(sink);

    assert_eq!(
        service.record_durable(&audit_event()),
        Err(AiAuditError::DurabilityNotSatisfied),
    );
}

#[test]
fn audit_storage_failure_is_propagated() {
    let sink = Arc::new(MemoryAuditSink {
        durability: Some(AiAuditDurability::Durable),
        fail: true,
        ..Default::default()
    });

    let service = AiAuditService::new(sink);

    assert!(
        matches!(
            service.record(&audit_event()),
            Err(AiAuditError::StorageFailure { .. })
        )
    );
}

#[test]
fn audit_failure_never_becomes_successful_persistence() {
    let sink = Arc::new(MemoryAuditSink {
        fail: true,
        ..Default::default()
    });

    let service = AiAuditService::new(sink);

    let result = service.record_durable(
        &audit_event(),
    );

    assert!(result.is_err());
}

// ============================================================================
// 10. END-TO-END AI SECURITY CONTRACT
// ============================================================================

#[test]
fn ai_security_domains_preserve_independent_ownership() {
    let identity = model_identity(42);

    let request = AiAuthorizationRequest::new(
        AiAuthorizationAction::new(
            "ai.model.infer",
        )
        .unwrap(),
        identity.clone(),
    );

    let binding = policy_binding("integration_binding");

    let boundary = AiIsolationBoundary::new(
        AiIsolationBoundaryId::new(
            "integration_boundary",
        )
        .unwrap(),
        AiIsolationSubject::Entity(
            identity.clone(),
        ),
    );

    let event = audit_event()
        .with_resource(identity);

    assert_eq!(
        request.action().as_str(),
        "ai.model.infer",
    );

    assert!(
        binding.validate().is_ok()
    );

    assert!(
        boundary.validate().is_ok()
    );

    assert_eq!(
        event.decision(),
        AiAuditDecision::Deny,
    );

    // These are independent security descriptions.
    //
    // None replaces the canonical distributed policy engine,
    // semantic validation, authentication, or runtime enforcement.
}

#[test]
fn security_serialization_does_not_erase_denial() {
    let event = audit_event();

    let encoded = serde_json::to_string(&event)
        .unwrap();

    let decoded: serde_json::Value =
        serde_json::from_str(&encoded)
            .unwrap();

    assert_eq!(
        decoded["decision"],
        "deny",
    );

    assert_eq!(
        decoded["outcome"],
        "rejected",
    );
}

#[test]
fn security_collections_have_stable_iteration() {
    let mut identities = BTreeSet::new();

    identities.insert(
        AiIsolationBoundaryId::new("zeta").unwrap(),
    );

    identities.insert(
        AiIsolationBoundaryId::new("alpha").unwrap(),
    );

    identities.insert(
        AiIsolationBoundaryId::new("middle").unwrap(),
    );

    let ordered: Vec<&str> = identities
        .iter()
        .map(|id| id.as_str())
        .collect();

    assert_eq!(
        ordered,
        vec!["alpha", "middle", "zeta"],
    );
}

// ============================================================================
// 11. AI SECURITY IDENTIFIER CATEGORY COVERAGE
// ============================================================================

#[test]
fn all_canonical_ai_entity_categories_remain_supported() {
    let authority = authority();

    let entities = [
        AiEntityRef::Model(
            AiModelId::new(1).unwrap(),
        ),
        AiEntityRef::Agent(
            AiAgentId::new(2).unwrap(),
        ),
        AiEntityRef::Dataset(
            AiDatasetId::new(3).unwrap(),
        ),
        AiEntityRef::TrainingRun(
            AiTrainingRunId::new(4).unwrap(),
        ),
        AiEntityRef::Pipeline(
            AiPipelineId::new(5).unwrap(),
        ),
        AiEntityRef::Artifact(
            AiArtifactId::new(6).unwrap(),
        ),
    ];

    for entity in entities {
        let reference = AiIdentityRef::new(
            authority.clone(),
            entity,
        );

        assert!(
            !reference.authority().as_str().is_empty()
        );

        assert_ne!(
            reference.local_id(),
            0,
        );
    }
}
