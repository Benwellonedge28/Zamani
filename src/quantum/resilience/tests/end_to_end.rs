//! Zamani Quantum Resilience — End-to-End Integration Tests
//!
//! Path:
//!     src/quantum/resilience/tests/end_to_end.rs
//!
//! Purpose:
//!     Production-oriented end-to-end contract tests for the complete
//!     quantum-resilience orchestration boundary.
//!
//! This file tests the real public resilience controller against an explicit
//! in-test orchestration implementation. It intentionally does not duplicate
//! production resilience algorithms. Instead, it verifies that the controller
//! correctly composes:
//!
//!     ResilienceRequest
//!          |
//!          v
//!     ResilienceController
//!          |
//!          +--> observe
//!          +--> detect
//!          +--> diagnose
//!          +--> policy
//!          +--> plan
//!          +--> adapt
//!          +--> recover
//!          +--> mitigate
//!          +--> verify
//!          +--> decide
//!          +--> response
//!
//! Architectural guarantees tested here:
//!
//! - canonical `quantum::ir::qubit::QubitId` is used;
//! - canonical `quantum::ir::program::QuantumProgram` is used;
//! - no resilience-specific qubit identity exists;
//! - no provider-specific behavior exists;
//! - no fixed quantum-machine size exists;
//! - no fixed retry count exists;
//! - optional stages are driven by the plan;
//! - verification is mandatory;
//! - lower-level errors are propagated;
//! - lifecycle ordering is deterministic;
//! - repeated identical inputs produce identical observable outcomes;
//! - large dynamically sized logical-qubit scopes are supported;
//! - the controller does not own or retain execution state;
//! - the test does not use unsafe Rust;
//! - the test uses only stable Rust facilities compatible with Rust 1.97/1.97.1.
//!
//! Important scope:
//!
//! This is an orchestration integration test. It does not pretend that a mock
//! can prove correctness of every hardware provider, QEC implementation,
//! routing algorithm, scheduler, optimizer, simulator, or backend. Those
//! systems require their own integration suites. This file verifies that the
//! resilience controller can safely compose those contracts when supplied.
//!
//! Integration:
//!
//! `src/quantum/resilience/tests/mod.rs` should declare this file only when
//! the file physically exists:
//!
//!     mod end_to_end;
//!
//! If the resilience test module is public, `pub mod end_to_end;` may be used.
//!
//! The test implementation deliberately imports through canonical public
//! resilience/IR paths rather than reaching into implementation-only details.
//!
//! Rust:
//!
//! - Rust 1.97 / 1.97.1
//! - Rust 2021 edition
//! - stable Rust
//! - no unsafe
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::cell::RefCell;
use std::sync::Arc;

use crate::quantum::ir::program::QuantumProgram;
use crate::quantum::ir::qubit::QubitId;
use crate::quantum::resilience::api::{
    ResilienceController,
    ResilienceControllerResult,
    ResilienceDecision,
    ResilienceOrchestration,
    ResilienceRequest,
    ResilienceRequestId,
    ResilienceRequestOptions,
    ResilienceScope,
    SemanticGuarantee,
    DeterministicSeed,
    ResilienceExecutionMode,
    ResourcePreference,
    ResilienceCycleId,
};
use crate::quantum::resilience::errors::{
    ResilienceError,
    ResilienceErrorCode,
};

// =============================================================================
// Test-domain types
// =============================================================================

/// Lifecycle stages observed by the end-to-end orchestration test.
///
/// This is test instrumentation only. It does not replace the production
/// resilience phase model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Stage {
    Observe,
    Detect,
    Diagnose,
    Policy,
    Plan,
    Adapt,
    Recover,
    Mitigate,
    Verify,
    Decide,
    BuildResponse,
}

/// Test plan describing which optional execution stages are required.
///
/// Production planning remains owned by the resilience planning subsystem.
/// This type exists only so the controller's conditional-stage contract can be
/// exercised deterministically.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TestPlan {
    requires_adaptation: bool,
    requires_recovery: bool,
    requires_mitigation: bool,
}

/// Test verification result.
///
/// A verification result explicitly records whether semantic verification
/// succeeded. The controller never interprets this value itself; the test
/// orchestration implementation converts it into the final decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TestVerification {
    accepted: bool,
    degraded: bool,
}

/// Test response representing the final output of the complete lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
struct TestResponse {
    request_id: String,
    cycle_id: u64,
    verified: bool,
    degraded: bool,
    adaptation_performed: bool,
    recovery_performed: bool,
    mitigation_performed: bool,
}

/// Test observation.
///
/// The production telemetry subsystem owns real observations. This structure
/// merely carries deterministic test evidence through the complete lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TestObservation {
    resource_count: usize,
    affected_qubit_count: usize,
}

/// Test detection result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TestDetection {
    fault_detected: bool,
}

/// Test diagnosis result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TestDiagnosis {
    recoverable: bool,
}

/// Test policy result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TestPolicy {
    allow_adaptation: bool,
    allow_recovery: bool,
    allow_mitigation: bool,
}

// =============================================================================
// Failure injection
// =============================================================================

/// Lifecycle stage at which the test orchestration should intentionally fail.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FailurePoint {
    None,
    Observe,
    Detect,
    Diagnose,
    Policy,
    Plan,
    Adapt,
    Recover,
    Mitigate,
    Verify,
    Decide,
    BuildResponse,
}

impl FailurePoint {
    const fn matches(self, stage: FailurePoint) -> bool {
        matches!(
            (self, stage),
            (Self::Observe, FailurePoint::Observe)
                | (Self::Detect, FailurePoint::Detect)
                | (Self::Diagnose, FailurePoint::Diagnose)
                | (Self::Policy, FailurePoint::Policy)
                | (Self::Plan, FailurePoint::Plan)
                | (Self::Adapt, FailurePoint::Adapt)
                | (Self::Recover, FailurePoint::Recover)
                | (Self::Mitigate, FailurePoint::Mitigate)
                | (Self::Verify, FailurePoint::Verify)
                | (Self::Decide, FailurePoint::Decide)
                | (Self::BuildResponse, FailurePoint::BuildResponse)
        )
    }
}

// =============================================================================
// End-to-end orchestration implementation
// =============================================================================

/// Deterministic orchestration implementation used by the integration tests.
///
/// It intentionally models the dependency boundary rather than implementing
/// resilience algorithms. Every method records its invocation and receives
/// the complete preceding lifecycle state where the real controller requires
/// it.
///
/// Interior mutability is test-local only. No production global mutable state
/// is used.
#[derive(Debug)]
struct EndToEndOrchestration {
    cycle_id: ResilienceCycleId,
    failure: FailurePoint,
    plan: TestPlan,
    events: RefCell<Vec<Stage>>,
}

impl EndToEndOrchestration {
    fn new(cycle_id: u64, plan: TestPlan) -> Self {
        Self {
            cycle_id: ResilienceCycleId::new(cycle_id),
            failure: FailurePoint::None,
            plan,
            events: RefCell::new(Vec::new()),
        }
    }

    fn failing_at(
        cycle_id: u64,
        plan: TestPlan,
        failure: FailurePoint,
    ) -> Self {
        Self {
            cycle_id: ResilienceCycleId::new(cycle_id),
            failure,
            plan,
            events: RefCell::new(Vec::new()),
        }
    }

    fn record(&self, stage: Stage) -> Result<(), ResilienceError> {
        self.events.borrow_mut().push(stage);

        if self.failure.matches(match stage {
            Stage::Observe => FailurePoint::Observe,
            Stage::Detect => FailurePoint::Detect,
            Stage::Diagnose => FailurePoint::Diagnose,
            Stage::Policy => FailurePoint::Policy,
            Stage::Plan => FailurePoint::Plan,
            Stage::Adapt => FailurePoint::Adapt,
            Stage::Recover => FailurePoint::Recover,
            Stage::Mitigate => FailurePoint::Mitigate,
            Stage::Verify => FailurePoint::Verify,
            Stage::Decide => FailurePoint::Decide,
            Stage::BuildResponse => FailurePoint::BuildResponse,
        }) {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidArgument,
                format!("injected end-to-end failure at {stage:?}"),
            ));
        }

        Ok(())
    }

    fn events(&self) -> Vec<Stage> {
        self.events.borrow().clone()
    }
}

impl ResilienceOrchestration for EndToEndOrchestration {
    type Request = ResilienceRequest;

    type Response = TestResponse;

    type Observation = TestObservation;

    type Detection = TestDetection;

    type Diagnosis = TestDiagnosis;

    type PolicyDecision = TestPolicy;

    type Plan = TestPlan;

    type Adaptation = bool;

    type Recovery = bool;

    type Mitigation = bool;

    type Verification = TestVerification;

    fn cycle_id(
        &self,
        _request: &Self::Request,
    ) -> Result<ResilienceCycleId, ResilienceError> {
        self.record(Stage::Observe)?;
        Ok(self.cycle_id)
    }

    fn observe(
        &self,
        request: &Self::Request,
    ) -> Result<Self::Observation, ResilienceError> {
        //
        // `cycle_id()` is the controller's identifier lookup and is therefore
        // intentionally separate from the actual observation lifecycle event.
        //
        // The controller calls cycle_id before observe. The event generated
        // there is test instrumentation and is not counted as a lifecycle
        // stage by the production controller summary.
        //
        // Do not record another artificial phase here.
        //
        let affected_qubit_count = request
            .logical_qubits()
            .map(<[QubitId]>::len)
            .unwrap_or(0);

        Ok(TestObservation {
            resource_count: affected_qubit_count,
            affected_qubit_count,
        })
    }

    fn detect(
        &self,
        _request: &Self::Request,
        observation: &Self::Observation,
    ) -> Result<Self::Detection, ResilienceError> {
        self.record(Stage::Detect)?;

        Ok(TestDetection {
            fault_detected: observation.affected_qubit_count > 0,
        })
    }

    fn diagnose(
        &self,
        _request: &Self::Request,
        _observation: &Self::Observation,
        detection: &Self::Detection,
    ) -> Result<Self::Diagnosis, ResilienceError> {
        self.record(Stage::Diagnose)?;

        Ok(TestDiagnosis {
            recoverable: !detection.fault_detected || true,
        })
    }

    fn evaluate_policy(
        &self,
        request: &Self::Request,
        _observation: &Self::Observation,
        _detection: &Self::Detection,
        diagnosis: &Self::Diagnosis,
    ) -> Result<Self::PolicyDecision, ResilienceError> {
        self.record(Stage::Policy)?;

        let adaptation = request.adaptation_permissions();

        let recovery = request.recovery_permissions();

        let mitigation_allowed =
            !matches!(
                request.mitigation_permission().as_str(),
                "forbidden"
            );

        Ok(TestPolicy {
            allow_adaptation: adaptation.remapping_allowed()
                && adaptation.rerouting_allowed()
                && adaptation.rescheduling_allowed()
                && adaptation.recompilation_allowed()
                && adaptation.reoptimization_allowed(),

            allow_recovery: diagnosis.recoverable
                && recovery.retry_allowed()
                && recovery.restart_allowed()
                && recovery.resume_allowed()
                && recovery.rollback_allowed()
                && recovery.migration_allowed()
                && recovery.compensation_allowed(),

            allow_mitigation: mitigation_allowed,
        })
    }

    fn plan(
        &self,
        _request: &Self::Request,
        _observation: &Self::Observation,
        _detection: &Self::Detection,
        _diagnosis: &Self::Diagnosis,
        policy: &Self::PolicyDecision,
    ) -> Result<Self::Plan, ResilienceError> {
        self.record(Stage::Plan)?;

        Ok(TestPlan {
            requires_adaptation:
                self.plan.requires_adaptation && policy.allow_adaptation,

            requires_recovery:
                self.plan.requires_recovery && policy.allow_recovery,

            requires_mitigation:
                self.plan.requires_mitigation && policy.allow_mitigation,
        })
    }

    fn requires_adaptation(&self, plan: &Self::Plan) -> bool {
        plan.requires_adaptation
    }

    fn adapt(
        &self,
        _request: &Self::Request,
        _observation: &Self::Observation,
        _diagnosis: &Self::Diagnosis,
        plan: &Self::Plan,
    ) -> Result<Self::Adaptation, ResilienceError> {
        self.record(Stage::Adapt)?;

        Ok(plan.requires_adaptation)
    }

    fn requires_recovery(&self, plan: &Self::Plan) -> bool {
        plan.requires_recovery
    }

    fn recover(
        &self,
        _request: &Self::Request,
        _observation: &Self::Observation,
        _diagnosis: &Self::Diagnosis,
        plan: &Self::Plan,
        adaptation: Option<&Self::Adaptation>,
    ) -> Result<Self::Recovery, ResilienceError> {
        self.record(Stage::Recover)?;

        //
        // Recovery receives adaptation output when adaptation was requested.
        // This verifies the controller's dependency ordering.
        //
        if plan.requires_adaptation && adaptation != Some(&true) {
            return Err(ResilienceError::invalid_argument(
                "recovery received an invalid adaptation dependency",
            ));
        }

        Ok(plan.requires_recovery)
    }

    fn requires_mitigation(&self, plan: &Self::Plan) -> bool {
        plan.requires_mitigation
    }

    fn mitigate(
        &self,
        _request: &Self::Request,
        _observation: &Self::Observation,
        _diagnosis: &Self::Diagnosis,
        plan: &Self::Plan,
        adaptation: Option<&Self::Adaptation>,
        recovery: Option<&Self::Recovery>,
    ) -> Result<Self::Mitigation, ResilienceError> {
        self.record(Stage::Mitigate)?;

        //
        // Mitigation must see the outputs of the preceding optional stages.
        //
        if plan.requires_adaptation && adaptation != Some(&true) {
            return Err(ResilienceError::invalid_argument(
                "mitigation received an invalid adaptation dependency",
            ));
        }

        if plan.requires_recovery && recovery != Some(&true) {
            return Err(ResilienceError::invalid_argument(
                "mitigation received an invalid recovery dependency",
            ));
        }

        Ok(plan.requires_mitigation)
    }

    fn verify(
        &self,
        request: &Self::Request,
        _observation: &Self::Observation,
        _detection: &Self::Detection,
        _diagnosis: &Self::Diagnosis,
        _policy: &Self::PolicyDecision,
        plan: &Self::Plan,
        adaptation: Option<&Self::Adaptation>,
        recovery: Option<&Self::Recovery>,
        mitigation: Option<&Self::Mitigation>,
    ) -> Result<Self::Verification, ResilienceError> {
        self.record(Stage::Verify)?;

        //
        // Verification is mandatory. The controller guarantees this method
        // is called even when all optional stages are disabled.
        //
        let adaptation_valid =
            !plan.requires_adaptation || adaptation == Some(&true);

        let recovery_valid =
            !plan.requires_recovery || recovery == Some(&true);

        let mitigation_valid =
            !plan.requires_mitigation || mitigation == Some(&true);

        let scope_valid = request.validate().is_ok();

        Ok(TestVerification {
            accepted: adaptation_valid
                && recovery_valid
                && mitigation_valid
                && scope_valid,

            degraded: false,
        })
    }

    fn build_response(
        &self,
        request: &Self::Request,
        _observation: &Self::Observation,
        _detection: &Self::Detection,
        _diagnosis: &Self::Diagnosis,
        _policy: &Self::PolicyDecision,
        plan: &Self::Plan,
        adaptation: Option<&Self::Adaptation>,
        recovery: Option<&Self::Recovery>,
        mitigation: Option<&Self::Mitigation>,
        verification: &Self::Verification,
    ) -> Result<Self::Response, ResilienceError> {
        self.record(Stage::BuildResponse)?;

        Ok(TestResponse {
            request_id: request.request_id().as_str().to_owned(),
            cycle_id: self.cycle_id.get(),
            verified: verification.accepted,
            degraded: verification.degraded,
            adaptation_performed:
                plan.requires_adaptation && adaptation == Some(&true),
            recovery_performed:
                plan.requires_recovery && recovery == Some(&true),
            mitigation_performed:
                plan.requires_mitigation && mitigation == Some(&true),
        })
    }

    fn decision(
        &self,
        request: &Self::Request,
        verification: &Self::Verification,
    ) -> Result<ResilienceDecision, ResilienceError> {
        self.record(Stage::Decide)?;

        if verification.accepted {
            if verification.degraded {
                if request.degraded_acceptance_allowed() {
                    return Ok(ResilienceDecision::DegradedAccept);
                }

                return Ok(ResilienceDecision::Reject);
            }

            return Ok(ResilienceDecision::Accept);
        }

        if request.escalation_allowed() {
            return Ok(ResilienceDecision::Escalate);
        }

        Ok(ResilienceDecision::Reject)
    }
}

// =============================================================================
// Request/program construction helpers
// =============================================================================

/// Creates the smallest real canonical Zamani quantum program suitable for
/// exercising the resilience request contract.
///
/// The test intentionally constructs the actual IR type instead of creating a
/// fake program abstraction.
fn test_program(program_id: u64, root_region_id: u64) -> Arc<QuantumProgram> {
    let program = QuantumProgram::new(
        crate::quantum::ir::program::ProgramId::new(program_id),
        crate::quantum::ir::program::RegionId::new(root_region_id),
    )
    .expect("canonical QuantumProgram construction must succeed");

    Arc::new(program)
}

/// Creates a real resilience request over the canonical Zamani IR.
///
/// Logical qubits are always created through:
///
///     crate::quantum::ir::qubit::QubitId
///
/// No resilience-specific qubit identity is used.
fn test_request(
    request_id: &str,
    program_id: u64,
    root_region_id: u64,
    logical_qubit_count: usize,
) -> ResilienceRequest {
    let program = test_program(program_id, root_region_id);

    let request_id = ResilienceRequestId::new(request_id)
        .expect("test request ID must be valid");

    let logical_qubits = (0..logical_qubit_count)
        .map(QubitId::new)
        .collect::<Vec<_>>();

    let scope = ResilienceScope::logical_qubits(logical_qubits);

    let options = ResilienceRequestOptions::new()
        .with_execution_mode(ResilienceExecutionMode::StrictDeterministic)
        .with_semantic_guarantee(SemanticGuarantee::Strict)
        .with_resource_preference(ResourcePreference::CorrectnessFirst)
        .with_deterministic_seed(Some(DeterministicSeed::new(0)));

    ResilienceRequest::from_parts(
        request_id,
        program,
        scope,
        options,
    )
    .expect("test resilience request must satisfy local invariants")
}

// =============================================================================
// Controller invocation helper
// =============================================================================

fn execute(
    orchestration: &EndToEndOrchestration,
    request: &ResilienceRequest,
) -> ResilienceControllerResult<TestResponse> {
    ResilienceController::new()
        .execute(orchestration, request)
        .expect("end-to-end resilience lifecycle should succeed")
}

// =============================================================================
// End-to-end happy path
// =============================================================================

#[test]
fn complete_lifecycle_executes_in_canonical_order() {
    let request = test_request(
        "e2e-complete-lifecycle",
        1,
        1,
        2,
    );

    let orchestration = EndToEndOrchestration::new(
        100,
        TestPlan {
            requires_adaptation: true,
            requires_recovery: true,
            requires_mitigation: true,
        },
    );

    let result = execute(&orchestration, &request);

    assert_eq!(
        result.decision(),
        ResilienceDecision::Accept
    );

    assert_eq!(
        result.cycle_id(),
        ResilienceCycleId::new(100)
    );

    assert_eq!(
        result.response(),
        &TestResponse {
            request_id: "e2e-complete-lifecycle".to_owned(),
            cycle_id: 100,
            verified: true,
            degraded: false,
            adaptation_performed: true,
            recovery_performed: true,
            mitigation_performed: true,
        }
    );

    //
    // The controller's own lifecycle ordering is:
    //
    // observe
    // detect
    // diagnose
    // policy
    // plan
    // adapt
    // recover
    // mitigate
    // verify
    // decide
    //
    // `BuildResponse` is a response-construction callback and is intentionally
    // not a resilience phase.
    //
    assert_eq!(
        orchestration.events(),
        vec![
            Stage::Observe,
            Stage::Detect,
            Stage::Diagnose,
            Stage::Policy,
            Stage::Plan,
            Stage::Adapt,
            Stage::Recover,
            Stage::Mitigate,
            Stage::Verify,
            Stage::Decide,
            Stage::BuildResponse,
        ]
    );

    let summary = result.summary();

    assert_eq!(summary.phases_completed(), 10);
    assert_eq!(summary.adaptations_requested(), 1);
    assert_eq!(summary.recoveries_requested(), 1);
    assert_eq!(summary.mitigations_requested(), 1);
    assert_eq!(summary.verifications_performed(), 1);
}

// =============================================================================
// Minimal lifecycle
// =============================================================================

#[test]
fn lifecycle_without_optional_operations_still_verifies() {
    let request = test_request(
        "e2e-minimal-lifecycle",
        2,
        2,
        1,
    );

    let orchestration = EndToEndOrchestration::new(
        200,
        TestPlan {
            requires_adaptation: false,
            requires_recovery: false,
            requires_mitigation: false,
        },
    );

    let result = execute(&orchestration, &request);

    assert_eq!(
        result.decision(),
        ResilienceDecision::Accept
    );

    assert_eq!(
        result.summary().phases_completed(),
        7
    );

    assert_eq!(
        result.summary().adaptations_requested(),
        0
    );

    assert_eq!(
        result.summary().recoveries_requested(),
        0
    );

    assert_eq!(
        result.summary().mitigations_requested(),
        0
    );

    assert_eq!(
        result.summary().verifications_performed(),
        1
    );

    assert_eq!(
        orchestration.events(),
        vec![
            Stage::Observe,
            Stage::Detect,
            Stage::Diagnose,
            Stage::Policy,
            Stage::Plan,
            Stage::Verify,
            Stage::Decide,
            Stage::BuildResponse,
        ]
    );

    assert!(
        result.response().verified,
        "successful execution must still pass verification"
    );
}

// =============================================================================
// Canonical QubitId integration
// =============================================================================

#[test]
fn request_scope_uses_canonical_ir_qubit_identity() {
    let request = test_request(
        "e2e-canonical-qubit",
        3,
        3,
        4,
    );

    let qubits = request
        .logical_qubits()
        .expect("logical scope must be present");

    assert_eq!(qubits.len(), 4);

    assert_eq!(qubits[0], QubitId::new(0));
    assert_eq!(qubits[1], QubitId::new(1));
    assert_eq!(qubits[2], QubitId::new(2));
    assert_eq!(qubits[3], QubitId::new(3));

    //
    // Compile-time type ownership is provided by the function signature:
    //
    //     fn accepts_canonical_qubit(_: QubitId)
    //
    // This prevents a resilience-local qubit ID from silently replacing the
    // canonical IR identity.
    //
    fn accepts_canonical_qubit(_qubit: QubitId) {}

    for qubit in qubits {
        accepts_canonical_qubit(*qubit);
    }
}

// =============================================================================
// Dynamic scalability
// =============================================================================

#[test]
fn logical_scope_scales_without_a_resilience_machine_size_constant() {
    //
    // This test deliberately derives the scope from a runtime test parameter.
    // The resilience implementation must not contain an architectural
    // MAX_QUBITS constant.
    //
    let sizes = [
        0_usize,
        1_usize,
        2_usize,
        8_usize,
        64_usize,
        257_usize,
        1024_usize,
    ];

    for (case_index, size) in sizes.into_iter().enumerate() {
        let request = test_request(
            &format!("e2e-scale-{size}"),
            (case_index as u64).saturating_add(10),
            (case_index as u64).saturating_add(20),
            size,
        );

        let orchestration = EndToEndOrchestration::new(
            (case_index as u64).saturating_add(1000),
            TestPlan {
                requires_adaptation: false,
                requires_recovery: false,
                requires_mitigation: false,
            },
        );

        let result = execute(&orchestration, &request);

        assert_eq!(
            result.decision(),
            ResilienceDecision::Accept,
            "scope size {size} must remain executable"
        );

        assert_eq!(
            request
                .logical_qubits()
                .map(<[QubitId]>::len)
                .unwrap_or(0),
            size
        );
    }
}

// =============================================================================
// Larger dynamic scope
// =============================================================================

#[test]
fn larger_dynamic_scope_preserves_order_and_identity() {
    let size = 4096_usize;

    let request = test_request(
        "e2e-large-scope",
        50,
        50,
        size,
    );

    let qubits = request
        .logical_qubits()
        .expect("large logical scope must exist");

    assert_eq!(qubits.len(), size);

    for (index, qubit) in qubits.iter().enumerate() {
        assert_eq!(
            *qubit,
            QubitId::new(index),
            "logical qubit ordering must remain deterministic"
        );
    }

    let orchestration = EndToEndOrchestration::new(
        5000,
        TestPlan {
            requires_adaptation: true,
            requires_recovery: false,
            requires_mitigation: false,
        },
    );

    let result = execute(&orchestration, &request);

    assert_eq!(
        result.decision(),
        ResilienceDecision::Accept
    );

    assert_eq!(
        result.summary().adaptations_requested(),
        1
    );

    assert_eq!(
        result.summary().verifications_performed(),
        1
    );
}

// =============================================================================
// Determinism
// =============================================================================

#[test]
fn identical_inputs_produce_identical_controller_results() {
    let request_a = test_request(
        "e2e-deterministic",
        60,
        60,
        16,
    );

    let request_b = test_request(
        "e2e-deterministic",
        60,
        60,
        16,
    );

    assert_eq!(
        request_a.request_id(),
        request_b.request_id()
    );

    assert_eq!(
        request_a.logical_qubits(),
        request_b.logical_qubits()
    );

    assert_eq!(
        request_a.deterministic_seed(),
        request_b.deterministic_seed()
    );

    let orchestration_a = EndToEndOrchestration::new(
        6000,
        TestPlan {
            requires_adaptation: true,
            requires_recovery: true,
            requires_mitigation: true,
        },
    );

    let orchestration_b = EndToEndOrchestration::new(
        6000,
        TestPlan {
            requires_adaptation: true,
            requires_recovery: true,
            requires_mitigation: true,
        },
    );

    let result_a = execute(
        &orchestration_a,
        &request_a,
    );

    let result_b = execute(
        &orchestration_b,
        &request_b,
    );

    assert_eq!(
        result_a.decision(),
        result_b.decision()
    );

    assert_eq!(
        result_a.cycle_id(),
        result_b.cycle_id()
    );

    assert_eq!(
        result_a.summary(),
        result_b.summary()
    );

    assert_eq!(
        result_a.response(),
        result_b.response()
    );

    assert_eq!(
        orchestration_a.events(),
        orchestration_b.events()
    );
}

// =============================================================================
// Verification is mandatory
// =============================================================================

#[test]
fn verification_is_mandatory_when_no_recovery_is_needed() {
    let request = test_request(
        "e2e-verification-required",
        70,
        70,
        1,
    );

    let orchestration = EndToEndOrchestration::new(
        7000,
        TestPlan {
            requires_adaptation: false,
            requires_recovery: false,
            requires_mitigation: false,
        },
    );

    let result = execute(&orchestration, &request);

    assert_eq!(
        result.summary().verifications_performed(),
        1
    );

    assert!(
        orchestration
            .events()
            .contains(&Stage::Verify),
        "verification must occur even on an apparently healthy execution"
    );

    assert_eq!(
        result.decision(),
        ResilienceDecision::Accept
    );
}

// =============================================================================
// Adaptation dependency
// =============================================================================

#[test]
fn adaptation_precedes_recovery() {
    let request = test_request(
        "e2e-adaptation-before-recovery",
        80,
        80,
        2,
    );

    let orchestration = EndToEndOrchestration::new(
        8000,
        TestPlan {
            requires_adaptation: true,
            requires_recovery: true,
            requires_mitigation: false,
        },
    );

    let result = execute(&orchestration, &request);

    assert_eq!(
        result.decision(),
        ResilienceDecision::Accept
    );

    let events = orchestration.events();

    let adaptation_index = events
        .iter()
        .position(|stage| *stage == Stage::Adapt)
        .expect("adaptation stage must exist");

    let recovery_index = events
        .iter()
        .position(|stage| *stage == Stage::Recover)
        .expect("recovery stage must exist");

    assert!(
        adaptation_index < recovery_index,
        "adaptation must precede recovery"
    );
}

// =============================================================================
// Recovery dependency
// =============================================================================

#[test]
fn recovery_precedes_mitigation() {
    let request = test_request(
        "e2e-recovery-before-mitigation",
        90,
        90,
        2,
    );

    let orchestration = EndToEndOrchestration::new(
        9000,
        TestPlan {
            requires_adaptation: false,
            requires_recovery: true,
            requires_mitigation: true,
        },
    );

    let result = execute(&orchestration, &request);

    assert_eq!(
        result.decision(),
        ResilienceDecision::Accept
    );

    let events = orchestration.events();

    let recovery_index = events
        .iter()
        .position(|stage| *stage == Stage::Recover)
        .expect("recovery stage must exist");

    let mitigation_index = events
        .iter()
        .position(|stage| *stage == Stage::Mitigate)
        .expect("mitigation stage must exist");

    assert!(
        recovery_index < mitigation_index,
        "recovery must precede mitigation"
    );
}

// =============================================================================
// Verification precedes decision
// =============================================================================

#[test]
fn verification_precedes_final_decision() {
    let request = test_request(
        "e2e-verification-before-decision",
        100,
        100,
        2,
    );

    let orchestration = EndToEndOrchestration::new(
        10000,
        TestPlan {
            requires_adaptation: true,
            requires_recovery: true,
            requires_mitigation: true,
        },
    );

    let result = execute(&orchestration, &request);

    assert_eq!(
        result.decision(),
        ResilienceDecision::Accept
    );

    let events = orchestration.events();

    let verify_index = events
        .iter()
        .position(|stage| *stage == Stage::Verify)
        .expect("verification must exist");

    let decide_index = events
        .iter()
        .position(|stage| *stage == Stage::Decide)
        .expect("decision must exist");

    assert!(
        verify_index < decide_index,
        "final acceptance decision must follow verification"
    );
}

// =============================================================================
// Failure propagation
// =============================================================================

#[test]
fn observe_failure_is_propagated_without_silent_success() {
    let request = test_request(
        "e2e-failure-observe",
        110,
        110,
        1,
    );

    let orchestration = EndToEndOrchestration::failing_at(
        11000,
        TestPlan {
            requires_adaptation: false,
            requires_recovery: false,
            requires_mitigation: false,
        },
        FailurePoint::Observe,
    );

    let result = ResilienceController::new()
        .execute(&orchestration, &request);

    assert!(
        result.is_err(),
        "observation failure must never become success"
    );
}

#[test]
fn detection_failure_is_propagated_without_entering_later_stages() {
    let request = test_request(
        "e2e-failure-detection",
        120,
        120,
        1,
    );

    let orchestration = EndToEndOrchestration::failing_at(
        12000,
        TestPlan {
            requires_adaptation: false,
            requires_recovery: false,
            requires_mitigation: false,
        },
        FailurePoint::Detect,
    );

    let result = ResilienceController::new()
        .execute(&orchestration, &request);

    assert!(
        result.is_err(),
        "detection failure must be propagated"
    );

    let events = orchestration.events();

    assert!(events.contains(&Stage::Detect));
    assert!(!events.contains(&Stage::Diagnose));
    assert!(!events.contains(&Stage::Verify));
    assert!(!events.contains(&Stage::Decide));
}

#[test]
fn diagnosis_failure_stops_the_lifecycle() {
    let request = test_request(
        "e2e-failure-diagnosis",
        130,
        130,
        2,
    );

    let orchestration = EndToEndOrchestration::failing_at(
        13000,
        TestPlan {
            requires_adaptation: false,
            requires_recovery: false,
            requires_mitigation: false,
        },
        FailurePoint::Diagnose,
    );

    let result = ResilienceController::new()
        .execute(&orchestration, &request);

    assert!(result.is_err());

    let events = orchestration.events();

    assert!(events.contains(&Stage::Diagnose));
    assert!(!events.contains(&Stage::Policy));
    assert!(!events.contains(&Stage::Verify));
    assert!(!events.contains(&Stage::Decide));
}

#[test]
fn policy_failure_stops_recovery_planning() {
    let request = test_request(
        "e2e-failure-policy",
        140,
        140,
        2,
    );

    let orchestration = EndToEndOrchestration::failing_at(
        14000,
        TestPlan {
            requires_adaptation: true,
            requires_recovery: true,
            requires_mitigation: true,
        },
        FailurePoint::Policy,
    );

    let result = ResilienceController::new()
        .execute(&orchestration, &request);

    assert!(result.is_err());

    let events = orchestration.events();

    assert!(events.contains(&Stage::Policy));
    assert!(!events.contains(&Stage::Plan));
    assert!(!events.contains(&Stage::Adapt));
    assert!(!events.contains(&Stage::Recover));
    assert!(!events.contains(&Stage::Verify));
}

#[test]
fn planning_failure_stops_adaptation() {
    let request = test_request(
        "e2e-failure-plan",
        150,
        150,
        2,
    );

    let orchestration = EndToEndOrchestration::failing_at(
        15000,
        TestPlan {
            requires_adaptation: true,
            requires_recovery: true,
            requires_mitigation: true,
        },
        FailurePoint::Plan,
    );

    let result = ResilienceController::new()
        .execute(&orchestration, &request);

    assert!(result.is_err());

    let events = orchestration.events();

    assert!(events.contains(&Stage::Plan));
    assert!(!events.contains(&Stage::Adapt));
    assert!(!events.contains(&Stage::Recover));
    assert!(!events.contains(&Stage::Verify));
}

#[test]
fn adaptation_failure_stops_recovery() {
    let request = test_request(
        "e2e-failure-adaptation",
        160,
        160,
        2,
    );

    let orchestration = EndToEndOrchestration::failing_at(
        16000,
        TestPlan {
            requires_adaptation: true,
            requires_recovery: true,
            requires_mitigation: true,
        },
        FailurePoint::Adapt,
    );

    let result = ResilienceController::new()
        .execute(&orchestration, &request);

    assert!(result.is_err());

    let events = orchestration.events();

    assert!(events.contains(&Stage::Adapt));
    assert!(!events.contains(&Stage::Recover));
    assert!(!events.contains(&Stage::Mitigate));
    assert!(!events.contains(&Stage::Verify));
}

#[test]
fn recovery_failure_stops_mitigation() {
    let request = test_request(
        "e2e-failure-recovery",
        170,
        170,
        2,
    );

    let orchestration = EndToEndOrchestration::failing_at(
        17000,
        TestPlan {
            requires_adaptation: true,
            requires_recovery: true,
            requires_mitigation: true,
        },
        FailurePoint::Recover,
    );

    let result = ResilienceController::new()
        .execute(&orchestration, &request);

    assert!(result.is_err());

    let events = orchestration.events();

    assert!(events.contains(&Stage::Recover));
    assert!(!events.contains(&Stage::Mitigate));
    assert!(!events.contains(&Stage::Verify));
}

#[test]
fn mitigation_failure_stops_verification() {
    let request = test_request(
        "e2e-failure-mitigation",
        180,
        180,
        2,
    );

    let orchestration = EndToEndOrchestration::failing_at(
        18000,
        TestPlan {
            requires_adaptation: true,
            requires_recovery: true,
            requires_mitigation: true,
        },
        FailurePoint::Mitigate,
    );

    let result = ResilienceController::new()
        .execute(&orchestration, &request);

    assert!(result.is_err());

    let events = orchestration.events();

    assert!(events.contains(&Stage::Mitigate));
    assert!(!events.contains(&Stage::Verify));
    assert!(!events.contains(&Stage::Decide));
}

#[test]
fn verification_failure_prevents_decision_and_response() {
    let request = test_request(
        "e2e-failure-verification",
        190,
        190,
        2,
    );

    let orchestration = EndToEndOrchestration::failing_at(
        19000,
        TestPlan {
            requires_adaptation: true,
            requires_recovery: true,
            requires_mitigation: true,
        },
        FailurePoint::Verify,
    );

    let result = ResilienceController::new()
        .execute(&orchestration, &request);

    assert!(result.is_err());

    let events = orchestration.events();

    assert!(events.contains(&Stage::Verify));
    assert!(!events.contains(&Stage::Decide));
    assert!(!events.contains(&Stage::BuildResponse));
}

#[test]
fn decision_failure_prevents_response() {
    let request = test_request(
        "e2e-failure-decision",
        200,
        200,
        2,
    );

    let orchestration = EndToEndOrchestration::failing_at(
        20000,
        TestPlan {
            requires_adaptation: false,
            requires_recovery: false,
            requires_mitigation: false,
        },
        FailurePoint::Decide,
    );

    let result = ResilienceController::new()
        .execute(&orchestration, &request);

    assert!(result.is_err());

    let events = orchestration.events();

    assert!(events.contains(&Stage::Decide));
    assert!(!events.contains(&Stage::BuildResponse));
}

#[test]
fn response_construction_failure_is_propagated() {
    let request = test_request(
        "e2e-failure-response",
        210,
        210,
        2,
    );

    let orchestration = EndToEndOrchestration::failing_at(
        21000,
        TestPlan {
            requires_adaptation: false,
            requires_recovery: false,
            requires_mitigation: false,
        },
        FailurePoint::BuildResponse,
    );

    let result = ResilienceController::new()
        .execute(&orchestration, &request);

    assert!(result.is_err());

    assert!(
        orchestration
            .events()
            .contains(&Stage::BuildResponse)
    );
}

// =============================================================================
// No hidden retry loop
// =============================================================================

#[test]
fn one_controller_call_performs_one_lifecycle_only() {
    let request = test_request(
        "e2e-single-cycle",
        220,
        220,
        4,
    );

    let orchestration = EndToEndOrchestration::new(
        22000,
        TestPlan {
            requires_adaptation: true,
            requires_recovery: true,
            requires_mitigation: true,
        },
    );

    let result = execute(&orchestration, &request);

    assert_eq!(
        result.summary().recoveries_requested(),
        1,
        "the controller must not secretly retry recovery"
    );

    assert_eq!(
        result.summary().adaptations_requested(),
        1
    );

    assert_eq!(
        result.summary().mitigations_requested(),
        1
    );

    assert_eq!(
        orchestration
            .events()
            .iter()
            .filter(|stage| **stage == Stage::Recover)
            .count(),
        1
    );
}

// =============================================================================
// Request immutability / reuse
// =============================================================================

#[test]
fn immutable_request_can_be_reused_for_independent_cycles() {
    let request = test_request(
        "e2e-request-reuse",
        230,
        230,
        8,
    );

    let first = EndToEndOrchestration::new(
        23000,
        TestPlan {
            requires_adaptation: false,
            requires_recovery: false,
            requires_mitigation: false,
        },
    );

    let second = EndToEndOrchestration::new(
        23001,
        TestPlan {
            requires_adaptation: true,
            requires_recovery: true,
            requires_mitigation: false,
        },
    );

    let first_result = execute(&first, &request);
    let second_result = execute(&second, &request);

    assert_eq!(
        request.request_id().as_str(),
        "e2e-request-reuse"
    );

    assert_eq!(
        request.logical_qubits().map(<[QubitId]>::len),
        Some(8)
    );

    assert_eq!(
        first_result.cycle_id().get(),
        23000
    );

    assert_eq!(
        second_result.cycle_id().get(),
        23001
    );

    assert_eq!(
        first_result.decision(),
        ResilienceDecision::Accept
    );

    assert_eq!(
        second_result.decision(),
        ResilienceDecision::Accept
    );

    assert_eq!(
        first_result.summary().adaptations_requested(),
        0
    );

    assert_eq!(
        second_result.summary().adaptations_requested(),
        1
    );
}

// =============================================================================
// Strict semantic acceptance
// =============================================================================

#[test]
fn strict_semantic_guarantee_does_not_allow_unverified_degradation() {
    let request = test_request(
        "e2e-strict-semantics",
        240,
        240,
        1,
    );

    assert_eq!(
        request.semantic_guarantee(),
        SemanticGuarantee::Strict
    );

    assert!(
        !request.degraded_acceptance_allowed(),
        "strict semantic guarantees must not silently allow degradation"
    );
}

// =============================================================================
// Explicit deterministic boundary
// =============================================================================

#[test]
fn strict_determinism_has_an_explicit_seed() {
    let request = test_request(
        "e2e-deterministic-seed",
        250,
        250,
        3,
    );

    assert_eq!(
        request.execution_mode(),
        ResilienceExecutionMode::StrictDeterministic
    );

    assert_eq!(
        request.deterministic_seed(),
        Some(DeterministicSeed::new(0))
    );

    assert!(
        request.strict_determinism(),
        "the request must explicitly declare strict deterministic execution"
    );
}

// =============================================================================
// Program identity remains independent from resilience identity
// =============================================================================

#[test]
fn resilience_request_does_not_replace_canonical_program_identity() {
    let request = test_request(
        "e2e-program-identity",
        260,
        260,
        2,
    );

    //
    // The resilience request exposes the actual canonical QuantumProgram.
    // It does not create a resilience-specific program representation.
    //
    let program = request.program();

    assert_eq!(
        program.qubit_count(),
        2,
        "the canonical program remains the semantic source of qubit declarations"
    );
}

// =============================================================================
// Zero-qubit semantic scope
// =============================================================================

#[test]
fn empty_logical_scope_is_supported_without_a_fake_machine_limit() {
    let request = test_request(
        "e2e-empty-scope",
        270,
        270,
        0,
    );

    assert_eq!(
        request.logical_qubits().map(<[QubitId]>::len),
        Some(0)
    );

    let orchestration = EndToEndOrchestration::new(
        27000,
        TestPlan {
            requires_adaptation: false,
            requires_recovery: false,
            requires_mitigation: false,
        },
    );

    let result = execute(&orchestration, &request);

    assert_eq!(
        result.decision(),
        ResilienceDecision::Accept
    );
}

// =============================================================================
// Controller statelessness
// =============================================================================

#[test]
fn controller_can_be_reused_without_cross_cycle_state() {
    let controller = ResilienceController::new();

    let request_a = test_request(
        "e2e-stateless-a",
        280,
        280,
        2,
    );

    let request_b = test_request(
        "e2e-stateless-b",
        281,
        281,
        5,
    );

    let orchestration_a = EndToEndOrchestration::new(
        28000,
        TestPlan {
            requires_adaptation: false,
            requires_recovery: false,
            requires_mitigation: false,
        },
    );

    let orchestration_b = EndToEndOrchestration::new(
        28001,
        TestPlan {
            requires_adaptation: true,
            requires_recovery: true,
            requires_mitigation: true,
        },
    );

    let result_a = controller
        .execute(&orchestration_a, &request_a)
        .expect("first controller cycle must succeed");

    let result_b = controller
        .execute(&orchestration_b, &request_b)
        .expect("second controller cycle must succeed");

    assert_eq!(
        result_a.cycle_id().get(),
        28000
    );

    assert_eq!(
        result_b.cycle_id().get(),
        28001
    );

    assert_eq!(
        result_a.response().request_id,
        "e2e-stateless-a"
    );

    assert_eq!(
        result_b.response().request_id,
        "e2e-stateless-b"
    );

    assert_eq!(
        result_a.summary().adaptations_requested(),
        0
    );

    assert_eq!(
        result_b.summary().adaptations_requested(),
        1
    );

    assert_eq!(
        result_a.summary().recoveries_requested(),
        0
    );

    assert_eq!(
        result_b.summary().recoveries_requested(),
        1
    );
}

// =============================================================================
// Complete integration contract matrix
// =============================================================================

#[test]
fn all_optional_stage_combinations_are_supported() {
    //
    // Three optional stages produce every possible boolean combination.
    // This is deliberately generated rather than testing only one "all on"
    // and one "all off" path.
    //
    let combinations = [
        (false, false, false),
        (false, false, true),
        (false, true, false),
        (false, true, true),
        (true, false, false),
        (true, false, true),
        (true, true, false),
        (true, true, true),
    ];

    for (index, (adaptation, recovery, mitigation)) in
        combinations.into_iter().enumerate()
    {
        let request = test_request(
            &format!("e2e-combination-{index}"),
            (300_u64).saturating_add(index as u64),
            (400_u64).saturating_add(index as u64),
            index.saturating_add(1),
        );

        let orchestration = EndToEndOrchestration::new(
            (30000_u64).saturating_add(index as u64),
            TestPlan {
                requires_adaptation: adaptation,
                requires_recovery: recovery,
                requires_mitigation: mitigation,
            },
        );

        let result = execute(&orchestration, &request);

        assert_eq!(
            result.decision(),
            ResilienceDecision::Accept,
            "optional combination {index} must succeed"
        );

        assert_eq!(
            result.summary().adaptations_requested(),
            u64::from(adaptation)
        );

        assert_eq!(
            result.summary().recoveries_requested(),
            u64::from(recovery)
        );

        assert_eq!(
            result.summary().mitigations_requested(),
            u64::from(mitigation)
        );

        assert_eq!(
            result.summary().verifications_performed(),
            1
        );
    }
}

// =============================================================================
// Error boundary integrity
// =============================================================================

#[test]
fn every_major_lifecycle_failure_class_remains_an_error() {
    let failures = [
        FailurePoint::Observe,
        FailurePoint::Detect,
        FailurePoint::Diagnose,
        FailurePoint::Policy,
        FailurePoint::Plan,
        FailurePoint::Adapt,
        FailurePoint::Recover,
        FailurePoint::Mitigate,
        FailurePoint::Verify,
        FailurePoint::Decide,
        FailurePoint::BuildResponse,
    ];

    for (index, failure) in failures.into_iter().enumerate() {
        let request = test_request(
            &format!("e2e-error-matrix-{index}"),
            (500_u64).saturating_add(index as u64),
            (600_u64).saturating_add(index as u64),
            2,
        );

        let orchestration = EndToEndOrchestration::failing_at(
            (50000_u64).saturating_add(index as u64),
            TestPlan {
                requires_adaptation: true,
                requires_recovery: true,
                requires_mitigation: true,
            },
            failure,
        );

        let result = ResilienceController::new()
            .execute(&orchestration, &request);

        assert!(
            result.is_err(),
            "failure at {failure:?} must never be converted into success"
        );
    }
}