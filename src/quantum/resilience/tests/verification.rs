//! Zamani Quantum Resilience — Verification Integration Tests.
//!
//! Path:
//!     src/quantum/resilience/tests/verification.rs
//!
//! Purpose
//! =======
//!
//! Production-oriented integration tests for the authoritative resilience
//! verification boundary.
//!
//! These tests exercise the public verification contracts rather than private
//! implementation details. They therefore remain useful when the individual
//! verification implementations evolve.
//!
//! Architectural guarantees covered:
//!
//! - verification is fail-closed;
//! - execution success alone never implies acceptance;
//! - semantic equality is required when semantic verification is enabled;
//! - mandatory verification components cannot be silently omitted;
//! - failed verification cannot become accepted through recovery;
//! - recovery/adaptation requires integrity verification under strict policy;
//! - aggregate confidence cannot override a failed verification dimension;
//! - logical and physical qubit identities remain distinct canonical types;
//! - no fixed machine-size assumption exists;
//! - resource cardinality is supplied dynamically;
//! - resource scopes are deterministic and canonicalized;
//! - verification is deterministic for deterministic evidence;
//! - schema identity is stable and non-empty;
//! - component failures are not silently converted into success;
//! - diagnostic/permissive policy does not remove mandatory semantic checks;
//! - simulator, physical, distributed and heterogeneous execution are not
//!   distinguished by provider-specific test logic;
//! - no wall-clock, filesystem, network or environment state is required.
//!
//! Canonical quantum identity
//! ===========================
//!
//! Quantum identity is owned by the canonical IR.
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! This file deliberately defines no resilience-local qubit identifier.
//!
//! Scalability
//! ===========
//!
//! The resilience architecture has no artificial finite quantum-machine
//! ceiling. "Infinity" therefore means that this test suite does not encode a
//! maximum number of qubits, operations, machines, backends or verification
//! observations.
//!
//! The suite verifies scalability by using dynamically supplied identifiers,
//! including `usize::MAX`, and generated collections without embedding a
//! machine-specific constant into the production API.
//!
//! Rust contract
//! =============
//!
//! - Rust 1.97 / 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! Clippy contract
//! ===============
//!
//! The tests intentionally avoid `unwrap`, `expect`, explicit `panic!`,
//! `todo!`, and `unimplemented!` so that the file remains compatible with
//! strict repository linting.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]

use std::sync::Arc;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

use crate::quantum::resilience::errors::{
    ResilienceError,
    ResilienceErrorCode,
    ResilienceResult,
};

use crate::quantum::resilience::verification::{
    ConfidenceVerifier,
    ExecutionIdentity,
    InvariantVerifier,
    ProvenanceVerifier,
    RecoveryIntegrityVerifier,
    ResilienceVerifier,
    ResultVerifier,
    SemanticFingerprint,
    SemanticVerifier,
    StructuralResourceIdentityVerifier,
    VerificationConfidence,
    VerificationDecision,
    VerificationEvidence,
    VerificationPolicy,
    VerificationRequest,
    VerificationResourceScope,
    VerificationStage,
    VerificationState,
    VerificationViolation,
    VerificationViolationKind,
};

// =============================================================================
// Test fixtures
// =============================================================================

fn execution_id(value: &str) -> ResilienceResult<ExecutionIdentity> {
    ExecutionIdentity::new(value)
}

fn fingerprint(value: &str) -> ResilienceResult<SemanticFingerprint> {
    SemanticFingerprint::new(value)
}

fn request() -> ResilienceResult<VerificationRequest> {
    let id = execution_id("verification-test-execution")?;
    let expected = fingerprint("semantic-a")?;
    let candidate = fingerprint("semantic-a")?;

    Ok(
        VerificationRequest::new(
            id,
            expected,
            VerificationResourceScope::default(),
        )
        .with_candidate_semantic_fingerprint(candidate)
        .with_execution_success(),
    )
}

// =============================================================================
// Reusable deterministic verification component
// =============================================================================

#[derive(Debug, Clone, Copy)]
struct FixedPassingVerifier {
    id: &'static str,
    confidence: VerificationConfidence,
}

impl FixedPassingVerifier {
    const fn certain(id: &'static str) -> Self {
        Self {
            id,
            confidence: VerificationConfidence::certain(),
        }
    }

    const fn with_confidence(
        id: &'static str,
        confidence: VerificationConfidence,
    ) -> Self {
        Self {
            id,
            confidence,
        }
    }

    fn evidence(&self) -> VerificationEvidence {
        VerificationEvidence::passed(self.confidence)
    }
}

impl SemanticVerifier for FixedPassingVerifier {
    fn id(&self) -> &str {
        self.id
    }

    fn verify_semantics(
        &self,
        _request: &VerificationRequest,
    ) -> ResilienceResult<VerificationEvidence> {
        Ok(self.evidence())
    }
}

impl InvariantVerifier for FixedPassingVerifier {
    fn id(&self) -> &str {
        self.id
    }

    fn verify_invariants(
        &self,
        _request: &VerificationRequest,
    ) -> ResilienceResult<VerificationEvidence> {
        Ok(self.evidence())
    }
}

impl ResultVerifier for FixedPassingVerifier {
    fn id(&self) -> &str {
        self.id
    }

    fn verify_result(
        &self,
        _request: &VerificationRequest,
    ) -> ResilienceResult<VerificationEvidence> {
        Ok(self.evidence())
    }
}

impl ProvenanceVerifier for FixedPassingVerifier {
    fn id(&self) -> &str {
        self.id
    }

    fn verify_provenance(
        &self,
        _request: &VerificationRequest,
    ) -> ResilienceResult<VerificationEvidence> {
        Ok(self.evidence())
    }
}

impl ConfidenceVerifier for FixedPassingVerifier {
    fn id(&self) -> &str {
        self.id
    }

    fn verify_confidence(
        &self,
        _request: &VerificationRequest,
    ) -> ResilienceResult<VerificationEvidence> {
        Ok(self.evidence())
    }
}

impl RecoveryIntegrityVerifier for FixedPassingVerifier {
    fn id(&self) -> &str {
        self.id
    }

    fn verify_recovery_integrity(
        &self,
        _request: &VerificationRequest,
    ) -> ResilienceResult<VerificationEvidence> {
        Ok(self.evidence())
    }
}

// =============================================================================
// Deliberately failing invariant component
// =============================================================================

#[derive(Debug, Clone, Copy)]
struct FailingInvariantVerifier;

impl InvariantVerifier for FailingInvariantVerifier {
    fn id(&self) -> &str {
        "test.invariants.failing"
    }

    fn verify_invariants(
        &self,
        _request: &VerificationRequest,
    ) -> ResilienceResult<VerificationEvidence> {
        Ok(VerificationEvidence::new(
            VerificationState::Failed,
            VerificationConfidence::certain(),
            [VerificationViolation::new(
                VerificationStage::Invariants,
                VerificationViolationKind::InvariantFailure,
                "TEST-VER-INVARIANT-001",
                "test invariant deliberately failed",
            )],
        ))
    }
}

// =============================================================================
// Deliberately erroring semantic component
// =============================================================================

#[derive(Debug, Clone, Copy)]
struct ErroringSemanticVerifier;

impl SemanticVerifier for ErroringSemanticVerifier {
    fn id(&self) -> &str {
        "test.semantic.error"
    }

    fn verify_semantics(
        &self,
        _request: &VerificationRequest,
    ) -> ResilienceResult<VerificationEvidence> {
        Err(ResilienceError::new(
            ResilienceErrorCode::InvalidConfiguration,
            "test semantic verifier failure",
        ))
    }
}

// =============================================================================
// Fully configured production-style verifier
// =============================================================================

fn production_verifier() -> ResilienceVerifier {
    let passing = Arc::new(
        FixedPassingVerifier::certain("test.passing"),
    );

    ResilienceVerifier::strict()
        .with_semantic_verifier(passing.clone())
        .with_invariant_verifier(passing.clone())
        .with_result_verifier(passing.clone())
        .with_provenance_verifier(passing.clone())
        .with_confidence_verifier(passing.clone())
        .with_recovery_integrity_verifier(passing)
}

// =============================================================================
// Public API
// =============================================================================

#[test]
fn verification_public_contract_is_constructible() -> ResilienceResult<()> {
    let id = execution_id("api-contract")?;
    let expected = fingerprint("program")?;
    let candidate = fingerprint("program")?;

    let request = VerificationRequest::new(
        id,
        expected,
        VerificationResourceScope::default(),
    )
    .with_candidate_semantic_fingerprint(candidate)
    .with_execution_success();

    let report = production_verifier().verify(&request)?;

    assert!(report.accepted());
    assert_eq!(
        report.decision(),
        VerificationDecision::Accept
    );

    Ok(())
}

// =============================================================================
// Policy
// =============================================================================

#[test]
fn strict_policy_enables_every_authoritative_dimension() {
    let policy = VerificationPolicy::strict();

    assert!(policy.execution());
    assert!(policy.resource_identity());
    assert!(policy.invariants());
    assert!(policy.semantic());
    assert!(policy.result());
    assert!(policy.provenance());
    assert!(policy.confidence());
    assert!(policy.recovery_integrity());
}

#[test]
fn default_policy_is_strict() {
    assert_eq!(
        VerificationPolicy::default(),
        VerificationPolicy::strict()
    );
}

#[test]
fn all_policy_is_equivalent_to_strict_policy() {
    assert_eq!(
        VerificationPolicy::all(),
        VerificationPolicy::strict()
    );
}

#[test]
fn permissive_policy_only_disables_explicit_optional_dimensions() {
    let policy = VerificationPolicy::permissive();

    assert!(policy.execution());
    assert!(policy.resource_identity());
    assert!(policy.invariants());
    assert!(policy.semantic());
    assert!(policy.result());

    assert!(!policy.provenance());
    assert!(!policy.confidence());
    assert!(!policy.recovery_integrity());
}

// =============================================================================
// Schema
// =============================================================================

#[test]
fn schema_identity_is_stable_and_versioned() {
    assert!(
        !crate::quantum::resilience::verification::
            RESILIENCE_VERIFIER_SCHEMA_ID
            .is_empty()
    );

    assert!(
        crate::quantum::resilience::verification::
            RESILIENCE_VERIFIER_SCHEMA_VERSION
            > 0
    );
}

#[test]
fn stage_names_are_non_empty_and_unique() {
    let stages = [
        VerificationStage::Execution,
        VerificationStage::ResourceIdentity,
        VerificationStage::Invariants,
        VerificationStage::Semantic,
        VerificationStage::Result,
        VerificationStage::Provenance,
        VerificationStage::Confidence,
        VerificationStage::RecoveryIntegrity,
        VerificationStage::Acceptance,
    ];

    for stage in stages {
        assert!(!stage.as_str().is_empty());
    }

    for (index, left) in stages.iter().enumerate() {
        for right in stages.iter().skip(index + 1) {
            assert_ne!(left.as_str(), right.as_str());
        }
    }
}

// =============================================================================
// Identity validation
// =============================================================================

#[test]
fn empty_execution_identity_is_rejected() {
    assert!(execution_id("").is_err());
}

#[test]
fn empty_semantic_fingerprint_is_rejected() {
    assert!(fingerprint("").is_err());
}

#[test]
fn execution_identity_is_opaque_and_deterministic() -> ResilienceResult<()> {
    let first = execution_id("same-execution")?;
    let second = execution_id("same-execution")?;
    let different = execution_id("different-execution")?;

    assert_eq!(first, second);
    assert_ne!(first, different);
    assert_eq!(first.as_str(), "same-execution");

    Ok(())
}

#[test]
fn semantic_fingerprint_is_opaque_and_deterministic() -> ResilienceResult<()> {
    let first = fingerprint("same-program")?;
    let second = fingerprint("same-program")?;
    let different = fingerprint("different-program")?;

    assert_eq!(first, second);
    assert_ne!(first, different);
    assert_eq!(first.as_str(), "same-program");

    Ok(())
}

// =============================================================================
// Canonical qubit identity
// =============================================================================

#[test]
fn canonical_logical_and_physical_qubit_types_are_distinct() {
    let logical = QubitId::new(7);
    let physical = PhysicalQubitId::new(7);

    assert_eq!(logical.index(), physical.index());

    assert_ne!(
        std::any::type_name::<QubitId>(),
        std::any::type_name::<PhysicalQubitId>()
    );
}

#[test]
fn canonical_qubit_ids_support_maximum_host_identifier() {
    let logical = QubitId::new(usize::MAX);
    let physical = PhysicalQubitId::new(usize::MAX);

    assert_eq!(logical.index(), usize::MAX);
    assert_eq!(physical.index(), usize::MAX);

    assert!(logical.checked_next().is_none());
    assert!(physical.checked_next().is_none());
}

// =============================================================================
// Resource scope
// =============================================================================

#[test]
fn resource_scope_is_deterministic_and_deduplicated() {
    let scope = VerificationResourceScope::new(
        [
            QubitId::new(9),
            QubitId::new(2),
            QubitId::new(9),
            QubitId::new(0),
        ],
        [
            PhysicalQubitId::new(42),
            PhysicalQubitId::new(3),
            PhysicalQubitId::new(42),
        ],
    );

    assert_eq!(
        scope.logical_qubits(),
        &[
            QubitId::new(0),
            QubitId::new(2),
            QubitId::new(9)
        ]
    );

    assert_eq!(
        scope.physical_qubits(),
        &[
            PhysicalQubitId::new(3),
            PhysicalQubitId::new(42)
        ]
    );
}

#[test]
fn resource_scope_does_not_assume_logical_and_physical_cardinality_match() {
    let scope = VerificationResourceScope::new(
        [
            QubitId::new(0),
            QubitId::new(1),
            QubitId::new(2),
        ],
        [PhysicalQubitId::new(100)],
    );

    assert_eq!(scope.logical_qubits().len(), 3);
    assert_eq!(scope.physical_qubits().len(), 1);
}

#[test]
fn empty_physical_scope_is_valid() {
    let scope = VerificationResourceScope::new(
        [QubitId::new(0)],
        std::iter::empty(),
    );

    assert_eq!(
        scope.logical_qubits(),
        &[QubitId::new(0)]
    );

    assert!(scope.physical_qubits().is_empty());
}

#[test]
fn empty_logical_scope_is_valid() {
    let scope = VerificationResourceScope::new(
        std::iter::empty(),
        [PhysicalQubitId::new(0)],
    );

    assert!(scope.logical_qubits().is_empty());

    assert_eq!(
        scope.physical_qubits(),
        &[PhysicalQubitId::new(0)]
    );
}

// =============================================================================
// Fail-closed execution
// =============================================================================

#[test]
fn execution_success_is_required_by_strict_policy() -> ResilienceResult<()> {
    let verifier = production_verifier();

    let request = VerificationRequest::new(
        execution_id("execution-failed")?,
        fingerprint("semantic-a")?,
        VerificationResourceScope::default(),
    )
    .with_candidate_semantic_fingerprint(
        fingerprint("semantic-a")?,
    );

    let report = verifier.verify(&request)?;

    assert!(!report.accepted());

    assert_eq!(
        report.decision(),
        VerificationDecision::Reject
    );

    assert!(
        report.violations().iter().any(|violation| {
            violation.kind()
                == VerificationViolationKind::ExecutionFailure
        })
    );

    Ok(())
}

#[test]
fn backend_success_without_verification_evidence_is_not_accepted()
-> ResilienceResult<()> {
    let report = ResilienceVerifier::strict()
        .verify(&request()?)?;

    assert!(!report.accepted());

    assert!(
        report.violations().iter().any(|violation| {
            violation.kind()
                == VerificationViolationKind::MissingEvidence
        })
    );

    Ok(())
}

// =============================================================================
// Semantic verification
// =============================================================================

#[test]
fn semantic_mismatch_is_rejected() -> ResilienceResult<()> {
    let verifier = production_verifier();

    let request = VerificationRequest::new(
        execution_id("semantic-mismatch")?,
        fingerprint("expected")?,
        VerificationResourceScope::default(),
    )
    .with_candidate_semantic_fingerprint(
        fingerprint("candidate")?,
    )
    .with_execution_success();

    let report = verifier.verify(&request)?;

    assert!(!report.accepted());

    assert!(
        report.violations().iter().any(|violation| {
            violation.kind()
                == VerificationViolationKind::SemanticMismatch
        })
    );

    Ok(())
}

#[test]
fn missing_candidate_semantics_is_rejected() -> ResilienceResult<()> {
    let request = VerificationRequest::new(
        execution_id("missing-candidate")?,
        fingerprint("expected")?,
        VerificationResourceScope::default(),
    )
    .with_execution_success();

    let result = production_verifier().verify(&request);

    assert!(result.is_err());

    Ok(())
}

#[test]
fn equal_semantic_fingerprints_do_not_replace_real_semantic_verification()
-> ResilienceResult<()> {
    let passing = Arc::new(
        FixedPassingVerifier::certain("test.semantic")
    );

    let verifier = ResilienceVerifier::strict()
        .with_semantic_verifier(passing);

    let result = verifier.verify(&request()?);

    assert!(result.is_err());

    Ok(())
}

// =============================================================================
// Invariant verification
// =============================================================================

#[test]
fn failed_invariant_cannot_be_hidden_by_passing_components()
-> ResilienceResult<()> {
    let passing = Arc::new(
        FixedPassingVerifier::certain("test.passing")
    );

    let verifier = ResilienceVerifier::strict()
        .with_semantic_verifier(passing.clone())
        .with_invariant_verifier(
            Arc::new(FailingInvariantVerifier)
        )
        .with_result_verifier(passing.clone())
        .with_provenance_verifier(passing.clone())
        .with_confidence_verifier(passing.clone())
        .with_recovery_integrity_verifier(passing);

    let report = verifier.verify(&request()?)?;

    assert!(!report.accepted());

    assert!(
        report.violations().iter().any(|violation| {
            violation.kind()
                == VerificationViolationKind::InvariantFailure
        })
    );

    Ok(())
}

// =============================================================================
// Recovery/adaptation integrity
// =============================================================================

#[test]
fn recovery_requires_integrity_verification_under_strict_policy()
-> ResilienceResult<()> {
    let passing = Arc::new(
        FixedPassingVerifier::certain("test.passing")
    );

    let verifier = ResilienceVerifier::strict()
        .with_semantic_verifier(passing.clone())
        .with_invariant_verifier(passing.clone())
        .with_result_verifier(passing.clone())
        .with_provenance_verifier(passing.clone())
        .with_confidence_verifier(passing);

    let request =
        request()?.with_recovery_or_adaptation();

    let report = verifier.verify(&request)?;

    assert!(!report.accepted());

    assert!(
        report.violations().iter().any(|violation| {
            violation.kind()
                == VerificationViolationKind::MissingEvidence
        })
    );

    Ok(())
}

#[test]
fn verified_recovery_can_be_accepted() -> ResilienceResult<()> {
    let request =
        request()?.with_recovery_or_adaptation();

    let report =
        production_verifier().verify(&request)?;

    assert_eq!(
        report.decision(),
        VerificationDecision::Accept
    );

    assert!(report.accepted());
    assert!(report.violations().is_empty());

    Ok(())
}

#[test]
fn failed_recovered_execution_requires_another_cycle()
-> ResilienceResult<()> {
    let passing = Arc::new(
        FixedPassingVerifier::certain("test.passing")
    );

    let verifier = ResilienceVerifier::strict()
        .with_semantic_verifier(passing.clone())
        .with_invariant_verifier(
            Arc::new(FailingInvariantVerifier)
        )
        .with_result_verifier(passing.clone())
        .with_provenance_verifier(passing.clone())
        .with_confidence_verifier(passing.clone())
        .with_recovery_integrity_verifier(passing);

    let request =
        request()?.with_recovery_or_adaptation();

    let report = verifier.verify(&request)?;

    assert_eq!(
        report.decision(),
        VerificationDecision::Repeat
    );

    assert!(!report.accepted());

    Ok(())
}

// =============================================================================
// Confidence
// =============================================================================

#[test]
fn verification_confidence_has_exact_integer_ordering() {
    let low =
        VerificationConfidence::from_basis_points(9_000);

    let high =
        VerificationConfidence::from_basis_points(9_500);

    assert!(low.is_some());
    assert!(high.is_some());

    if let (Some(low), Some(high)) = (low, high) {
        assert!(high > low);
        assert!(high.meets(low));
        assert!(!low.meets(high));
    }
}

#[test]
fn confidence_above_required_threshold_can_accept()
-> ResilienceResult<()> {
    let confidence =
        VerificationConfidence::from_basis_points(9_000);

    let required =
        VerificationConfidence::from_basis_points(8_000);

    let (Some(confidence), Some(required)) =
        (confidence, required)
    else {
        return Err(ResilienceError::new(
            ResilienceErrorCode::InvalidConfiguration,
            "test confidence values could not be constructed",
        ));
    };

    let passing = Arc::new(
        FixedPassingVerifier::with_confidence(
            "test.9000",
            confidence,
        )
    );

    let verifier = ResilienceVerifier::strict()
        .with_required_confidence(required)
        .with_semantic_verifier(passing.clone())
        .with_invariant_verifier(passing.clone())
        .with_result_verifier(passing.clone())
        .with_provenance_verifier(passing.clone())
        .with_confidence_verifier(passing.clone())
        .with_recovery_integrity_verifier(passing);

    let report = verifier.verify(&request()?)?;

    assert_eq!(
        report.decision(),
        VerificationDecision::Accept
    );

    assert!(report.accepted());

    Ok(())
}

#[test]
fn insufficient_aggregate_confidence_is_not_accepted()
-> ResilienceResult<()> {
    let confidence =
        VerificationConfidence::from_basis_points(9_000);

    let required =
        VerificationConfidence::from_basis_points(9_500);

    let (Some(confidence), Some(required)) =
        (confidence, required)
    else {
        return Err(ResilienceError::new(
            ResilienceErrorCode::InvalidConfiguration,
            "test confidence values could not be constructed",
        ));
    };

    let passing = Arc::new(
        FixedPassingVerifier::with_confidence(
            "test.9000",
            confidence,
        )
    );

    let verifier = ResilienceVerifier::strict()
        .with_required_confidence(required)
        .with_semantic_verifier(passing.clone())
        .with_invariant_verifier(passing.clone())
        .with_result_verifier(passing.clone())
        .with_provenance_verifier(passing.clone())
        .with_confidence_verifier(passing.clone())
        .with_recovery_integrity_verifier(passing);

    let report = verifier.verify(&request()?)?;

    assert!(!report.accepted());

    assert_eq!(
        report.decision(),
        VerificationDecision::Escalate
    );

    assert!(
        report.violations().iter().any(|violation| {
            violation.kind()
                == VerificationViolationKind::ConfidenceInsufficient
        })
    );

    Ok(())
}

#[test]
fn zero_confidence_cannot_satisfy_positive_requirement()
-> ResilienceResult<()> {
    let zero = VerificationConfidence::none();

    let required =
        VerificationConfidence::from_basis_points(1);

    let Some(required) = required else {
        return Err(ResilienceError::new(
            ResilienceErrorCode::InvalidConfiguration,
            "test confidence threshold could not be constructed",
        ));
    };

    assert!(!zero.meets(required));

    Ok(())
}

// =============================================================================
// Component failure propagation
// =============================================================================

#[test]
fn component_errors_are_propagated_not_hidden()
-> ResilienceResult<()> {
    let passing = Arc::new(
        FixedPassingVerifier::certain("test.passing")
    );

    let verifier = ResilienceVerifier::strict()
        .with_semantic_verifier(
            Arc::new(ErroringSemanticVerifier)
        )
        .with_invariant_verifier(passing.clone())
        .with_result_verifier(passing.clone())
        .with_provenance_verifier(passing.clone())
        .with_confidence_verifier(passing.clone())
        .with_recovery_integrity_verifier(passing);

    let result =
        verifier.verify(&request()?);

    assert!(result.is_err());

    if let Err(error) = result {
        assert_eq!(
            error.code(),
            ResilienceErrorCode::InvalidConfiguration
        );
    }

    Ok(())
}

// =============================================================================
// Resource identity
// =============================================================================

#[test]
fn structural_resource_identity_verifier_is_provider_neutral()
-> ResilienceResult<()> {
    let scope = VerificationResourceScope::new(
        [
            QubitId::new(0),
            QubitId::new(1),
            QubitId::new(2),
        ],
        [
            PhysicalQubitId::new(17),
            PhysicalQubitId::new(21),
        ],
    );

    let request = VerificationRequest::new(
        execution_id("resource-identity")?,
        fingerprint("resource-program")?,
        scope,
    )
    .with_candidate_semantic_fingerprint(
        fingerprint("resource-program")?
    )
    .with_execution_success();

    let verifier =
        StructuralResourceIdentityVerifier::new();

    let evidence =
        verifier.verify_resource_identity(&request)?;

    assert_eq!(
        evidence.state(),
        VerificationState::Passed
    );

    assert_eq!(
        evidence.confidence(),
        VerificationConfidence::certain()
    );

    Ok(())
}

#[test]
fn replacing_resource_identity_verifier_uses_public_contract()
-> ResilienceResult<()> {
    let verifier =
        production_verifier()
            .with_resource_identity_verifier(
                Arc::new(
                    StructuralResourceIdentityVerifier::new()
                )
            );

    let report =
        verifier.verify(&request()?)?;

    assert!(report.accepted());

    assert!(
        report.stages().iter().any(|stage| {
            stage.stage()
                == VerificationStage::ResourceIdentity
                && stage.component_id()
                    == "zamani.resilience.verification.resource_identity.structural"
        })
    );

    Ok(())
}

// =============================================================================
// Policy safety
// =============================================================================

#[test]
fn permissive_policy_does_not_disable_core_semantic_correctness()
-> ResilienceResult<()> {
    let passing = Arc::new(
        FixedPassingVerifier::certain("test.passing")
    );

    let verifier =
        ResilienceVerifier::with_policy(
            VerificationPolicy::permissive()
        )
        .with_semantic_verifier(passing.clone())
        .with_invariant_verifier(passing.clone())
        .with_result_verifier(passing);

    let report =
        verifier.verify(&request()?)?;

    assert!(report.accepted());

    assert_eq!(
        report.decision(),
        VerificationDecision::Accept
    );

    Ok(())
}

#[test]
fn permissive_policy_does_not_accept_semantic_mismatch()
-> ResilienceResult<()> {
    let passing = Arc::new(
        FixedPassingVerifier::certain("test.passing")
    );

    let verifier =
        ResilienceVerifier::with_policy(
            VerificationPolicy::permissive()
        )
        .with_semantic_verifier(passing.clone())
        .with_invariant_verifier(passing.clone())
        .with_result_verifier(passing);

    let request = VerificationRequest::new(
        execution_id("permissive-mismatch")?,
        fingerprint("expected")?,
        VerificationResourceScope::default(),
    )
    .with_candidate_semantic_fingerprint(
        fingerprint("different")?
    )
    .with_execution_success();

    let report =
        verifier.verify(&request)?;

    assert!(!report.accepted());

    assert!(
        report.violations().iter().any(|violation| {
            violation.kind()
                == VerificationViolationKind::SemanticMismatch
        })
    );

    Ok(())
}

// =============================================================================
// Acceptance semantics
// =============================================================================

#[test]
fn every_non_accepting_decision_blocks_acceptance() {
    let decisions = [
        VerificationDecision::Repeat,
        VerificationDecision::Escalate,
        VerificationDecision::Reject,
    ];

    for decision in decisions {
        assert!(decision.blocks_acceptance());
        assert!(!decision.is_accepted());
    }
}

#[test]
fn only_explicit_acceptance_decisions_are_accepted() {
    assert!(VerificationDecision::Accept.is_accepted());
    assert!(
        VerificationDecision::DegradedAccept.is_accepted()
    );

    assert!(!VerificationDecision::Repeat.is_accepted());
    assert!(!VerificationDecision::Escalate.is_accepted());
    assert!(!VerificationDecision::Reject.is_accepted());
}

#[test]
fn accepted_report_has_no_violations()
-> ResilienceResult<()> {
    let report =
        production_verifier().verify(&request()?)?;

    assert_eq!(
        report.decision(),
        VerificationDecision::Accept
    );

    assert_eq!(
        report.overall_state(),
        VerificationState::Passed
    );

    assert!(report.accepted());
    assert!(report.violations().is_empty());

    Ok(())
}

#[test]
fn accepted_report_has_certain_confidence_when_all_components_are_certain()
-> ResilienceResult<()> {
    let report =
        production_verifier().verify(&request()?)?;

    assert_eq!(
        report.overall_confidence(),
        VerificationConfidence::certain()
    );

    Ok(())
}

// =============================================================================
// Stage composition
// =============================================================================

#[test]
fn report_contains_all_required_strict_stages()
-> ResilienceResult<()> {
    let report =
        production_verifier().verify(&request()?)?;

    let expected = [
        VerificationStage::Execution,
        VerificationStage::ResourceIdentity,
        VerificationStage::Invariants,
        VerificationStage::Semantic,
        VerificationStage::Result,
        VerificationStage::Provenance,
        VerificationStage::Confidence,
        VerificationStage::RecoveryIntegrity,
    ];

    for stage in expected {
        assert!(
            report.stages().iter().any(|actual| {
                actual.stage() == stage
            })
        );
    }

    Ok(())
}

#[test]
fn successful_stage_components_report_their_component_ids()
-> ResilienceResult<()> {
    let report =
        production_verifier().verify(&request()?)?;

    for stage in report.stages() {
        if stage.stage() != VerificationStage::Execution
            && stage.stage()
                != VerificationStage::ResourceIdentity
        {
            assert!(!stage.component_id().is_empty());
        }
    }

    Ok(())
}

// =============================================================================
// Determinism
// =============================================================================

#[test]
fn identical_deterministic_inputs_produce_identical_reports()
-> ResilienceResult<()> {
    let verifier = production_verifier();
    let request = request()?;

    let first = verifier.verify(&request)?;
    let second = verifier.verify(&request)?;

    assert_eq!(first, second);

    Ok(())
}

#[test]
fn repeated_verification_does_not_depend_on_wall_clock_progress()
-> ResilienceResult<()> {
    let verifier = production_verifier();
    let request = request()?;

    let first = verifier.verify(&request)?;
    let second = verifier.verify(&request)?;

    assert_eq!(
        first.decision(),
        second.decision()
    );

    assert_eq!(
        first.overall_state(),
        second.overall_state()
    );

    assert_eq!(
        first.overall_confidence(),
        second.overall_confidence()
    );

    assert_eq!(
        first.violations(),
        second.violations()
    );

    Ok(())
}

// =============================================================================
// Large identifier / scalability checks
// =============================================================================

#[test]
fn verification_accepts_maximum_logical_and_physical_identifiers()
-> ResilienceResult<()> {
    let scope = VerificationResourceScope::new(
        [QubitId::new(usize::MAX)],
        [PhysicalQubitId::new(usize::MAX)],
    );

    let request = VerificationRequest::new(
        execution_id("maximum-resource-identifier")?,
        fingerprint("semantic-a")?,
        scope,
    )
    .with_candidate_semantic_fingerprint(
        fingerprint("semantic-a")?
    )
    .with_execution_success();

    let report =
        production_verifier().verify(&request)?;

    assert!(report.accepted());

    Ok(())
}

#[test]
fn verification_resource_scope_scales_without_machine_specific_limits() {
    let logical =
        (0usize..1024).rev().map(QubitId::new);

    let physical =
        (0usize..1024)
            .rev()
            .map(PhysicalQubitId::new);

    let scope =
        VerificationResourceScope::new(
            logical,
            physical,
        );

    assert_eq!(
        scope.logical_qubits().first(),
        Some(&QubitId::new(0))
    );

    assert_eq!(
        scope.logical_qubits().last(),
        Some(&QubitId::new(1023))
    );

    assert_eq!(
        scope.physical_qubits().first(),
        Some(&PhysicalQubitId::new(0))
    );

    assert_eq!(
        scope.physical_qubits().last(),
        Some(&PhysicalQubitId::new(1023))
    );

    assert_eq!(
        scope.logical_qubits().len(),
        1024
    );

    assert_eq!(
        scope.physical_qubits().len(),
        1024
    );
}

// =============================================================================
// Evidence normalization
// =============================================================================

#[test]
fn passing_evidence_with_violations_is_normalized_to_failure() {
    let evidence = VerificationEvidence::new(
        VerificationState::Passed,
        VerificationConfidence::certain(),
        [VerificationViolation::new(
            VerificationStage::Result,
            VerificationViolationKind::ResultInvalid,
            "TEST-VER-RESULT-001",
            "deliberately invalid result evidence",
        )],
    );

    assert_eq!(
        evidence.state(),
        VerificationState::Failed
    );

    assert!(!evidence.is_successful());
    assert_eq!(
        evidence.violations().len(),
        1
    );
}

#[test]
fn not_required_evidence_is_successful_for_disabled_dimension() {
    let evidence =
        VerificationEvidence::not_required();

    assert_eq!(
        evidence.state(),
        VerificationState::NotRequired
    );

    assert!(evidence.is_successful());
    assert!(evidence.violations().is_empty());
}

#[test]
fn indeterminate_evidence_is_not_successful() {
    let evidence =
        VerificationEvidence::indeterminate(
            VerificationConfidence::none(),
            Arc::<[VerificationViolation]>::from([]),
        );

    assert_eq!(
        evidence.state(),
        VerificationState::Indeterminate
    );

    assert!(!evidence.is_successful());
}

// =============================================================================
// Verification-state semantics
// =============================================================================

#[test]
fn passed_state_is_successful() {
    assert!(
        VerificationState::Passed.is_successful()
    );

    assert!(
        !VerificationState::Passed.blocks_acceptance()
    );
}

#[test]
fn not_required_state_is_successful_for_its_dimension() {
    assert!(
        VerificationState::NotRequired
            .is_successful()
    );

    assert!(
        !VerificationState::NotRequired
            .blocks_acceptance()
    );
}

#[test]
fn failed_state_blocks_acceptance() {
    assert!(
        VerificationState::Failed.blocks_acceptance()
    );

    assert!(
        !VerificationState::Failed.is_successful()
    );
}

#[test]
fn indeterminate_state_blocks_acceptance() {
    assert!(
        VerificationState::Indeterminate
            .blocks_acceptance()
    );

    assert!(
        !VerificationState::Indeterminate
            .is_successful()
    );
}