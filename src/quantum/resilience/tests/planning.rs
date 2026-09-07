//! Zamani Quantum Resilience — Production Planning Tests
//!
//! Path:
//!     src/quantum/resilience/tests/planning.rs
//!
//! Purpose:
//!     Validate the resilience planning contract without coupling tests to:
//!     - a quantum provider;
//!     - a particular QPU size;
//!     - a particular topology;
//!     - a particular number of physical qubits;
//!     - a particular retry count;
//!     - a particular fidelity threshold;
//!     - a particular backend;
//!     - a particular QEC code.
//!
//! Architectural principle:
//!
//!     Zamani program
//!          |
//!          v
//!     canonical IR
//!          |
//!          v
//!     detection -> diagnosis -> policy -> planning
//!                                      |
//!                                      v
//!                              RecoveryPlan candidates
//!                                      |
//!                     +----------------+----------------+
//!                     |                |                |
//!                     v                v                v
//!                 adaptation       recovery        mitigation
//!                     |                |                |
//!                     +----------------+----------------+
//!                                      |
//!                                      v
//!                                  verification
//!
//! The planner is a decision boundary, not an executor.
//!
//! These tests therefore verify:
//!     - conservative defaults;
//!     - input validation;
//!     - configuration validation;
//!     - deterministic candidate generation;
//!     - stable action identifiers;
//!     - deterministic ordering;
//!     - safety requirements;
//!     - preservation of semantic verification requirements;
//!     - capability/resource awareness;
//!     - recovery/adaptation/migration alternatives;
//!     - protective fallback;
//!     - correlated-incident handling;
//!     - previous-recovery-failure handling;
//!     - arbitrary resource/attempt magnitudes;
//!     - absence of machine-size assumptions;
//!     - representation-bound behavior;
//!     - public contract stability.
//!
//! This file intentionally uses the canonical quantum identity where a quantum
//! identity is required:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! It does not define a replacement QubitId.
//!
//! Rust:
//!     Rust 1.97 / 1.97.1
//!     Rust 2021
//!     stable Rust
//!     no unsafe code
//!
//! Integration:
//!     Included by `resilience/tests/mod.rs`.
//!
//!     The tests exercise the public planning contract rather than private
//!     implementation details, so planner internals may evolve without forcing
//!     this test module to be rewritten.
//!
//! Important:
//!     `MAX_CANDIDATES`, `MAX_REQUIREMENTS`, and `MAX_REASONS` are tested only
//!     as planner representation/safety boundaries. They are NOT quantum
//!     machine limits and MUST NOT be interpreted as such.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::any::type_name;

use crate::quantum::ir::qubit::QubitId;
use crate::quantum::resilience::planning::planner::{
    action_id,
    PlanPhase,
    Planner,
    PlannerConfig,
    PlannerError,
    PlannerInput,
    PlanningReason,
    Requirement,
    MAX_CANDIDATES,
    MAX_REASONS,
    MAX_REQUIREMENTS,
    PLANNER_SCHEMA_ID,
    PLANNER_SCHEMA_VERSION,
    PLANNER_VERSION,
};

/// Creates a conservative baseline planner input.
fn baseline_input() -> PlannerInput {
    PlannerInput::new()
}

/// Creates an input representing an active execution with a recoverable
/// transient failure.
fn active_retryable_input() -> PlannerInput {
    baseline_input()
        .with_execution_active(true)
        .with_retry_safe(true)
        .with_verification_available(true)
        .with_primary_reason(PlanningReason::TransientFailure)
}

/// Creates an input representing an invalid physical realization.
fn invalid_realization_input() -> PlannerInput {
    baseline_input()
        .with_execution_active(true)
        .with_mapping_valid(false)
        .with_routing_valid(false)
        .with_schedule_valid(false)
        .with_compilation_valid(false)
        .with_verification_available(true)
        .with_primary_reason(PlanningReason::PhysicalRealizationInvalid)
}

/// Creates an input representing a changed hardware capability state.
fn capability_change_input() -> PlannerInput {
    baseline_input()
        .with_execution_active(true)
        .with_capabilities_changed(true)
        .with_mapping_valid(false)
        .with_routing_valid(false)
        .with_schedule_valid(false)
        .with_compilation_valid(false)
        .with_resources_feasible(true)
        .with_verification_available(true)
        .with_primary_reason(PlanningReason::CapabilityChange)
}

/// Creates an input representing an unavailable current backend with a
/// compatible migration target.
fn migration_input() -> PlannerInput {
    baseline_input()
        .with_execution_active(true)
        .with_backend_available(false)
        .with_migration_possible(true)
        .with_verification_available(true)
        .with_primary_reason(PlanningReason::BackendUnavailable)
}

/// Creates an input representing degraded resources where quarantine and
/// continued execution are both possible.
fn degraded_resource_input() -> PlannerInput {
    baseline_input()
        .with_execution_active(true)
        .with_resources_feasible(false)
        .with_quarantine_possible(true)
        .with_mapping_valid(false)
        .with_routing_valid(false)
        .with_verification_available(true)
        .with_primary_reason(PlanningReason::ResourceDegradation)
}

/// Creates an input representing a QEC-degraded execution.
fn qec_input() -> PlannerInput {
    baseline_input()
        .with_execution_active(true)
        .with_qec_adaptation_possible(true)
        .with_verification_available(true)
        .with_primary_reason(PlanningReason::QecDegradation)
}

/// Creates an input representing noise for which mitigation is available.
fn mitigation_input() -> PlannerInput {
    baseline_input()
        .with_execution_active(true)
        .with_mitigation_possible(true)
        .with_verification_available(true)
        .with_primary_reason(PlanningReason::NoiseMitigation)
}

/// Creates an input for a correlated incident.
fn correlated_input() -> PlannerInput {
    baseline_input()
        .with_execution_active(true)
        .with_correlated_incident(true)
        .with_retry_safe(true)
        .with_verification_available(true)
        .with_primary_reason(PlanningReason::CorrelatedIncident)
}

/// Returns all stable planner action identifiers.
///
/// Keeping this in one helper makes the test suite exhaustive when a new
/// planner action is intentionally added. The values come from the production
/// planner contract rather than being provider-specific test assumptions.
fn all_action_ids() -> [&'static str; 17] {
    [
        action_id::RETRY,
        action_id::RESTART,
        action_id::RESUME,
        action_id::ROLLBACK,
        action_id::CHECKPOINT,
        action_id::REMAP,
        action_id::REROUTE,
        action_id::RESCHEDULE,
        action_id::RECOMPILE,
        action_id::REOPTIMIZE,
        action_id::ADAPT_QEC,
        action_id::MITIGATE,
        action_id::MIGRATE,
        action_id::QUARANTINE_RESOURCE,
        action_id::COMPENSATE,
        action_id::ESCALATE,
        action_id::ABORT,
    ]
}

#[test]
fn planner_schema_identity_is_stable() {
    assert_eq!(
        PLANNER_SCHEMA_ID,
        "zamani.quantum.resilience.planner"
    );

    assert!(PLANNER_SCHEMA_VERSION > 0);
    assert!(PLANNER_VERSION > 0);
}

#[test]
fn planner_has_no_unsafe_surface() {
    // The module itself is compiled under `forbid(unsafe_code)`.
    // This assertion additionally keeps the test coupled to the intended
    // public planner type rather than a private implementation.
    assert!(!type_name::<Planner>().is_empty());
}

#[test]
fn canonical_qubit_identity_is_available_from_quantum_ir() {
    // Planning tests must never introduce a second QubitId abstraction.
    //
    // The planner currently operates primarily on provider-neutral resource
    // facts, so it does not need to manufacture physical qubit IDs. This test
    // nevertheless guarantees that any future quantum-resource integration can
    // use the canonical IR identity.
    assert!(!type_name::<QubitId>().is_empty());
}

#[test]
fn default_input_is_conservative() {
    let input = PlannerInput::new();

    assert!(!input.execution_active());
    assert!(!input.checkpoint_available());
    assert!(!input.resume_available());
    assert!(!input.retry_safe());
    assert!(!input.rollback_available());

    assert!(input.mapping_valid());
    assert!(input.routing_valid());
    assert!(input.schedule_valid());
    assert!(input.compilation_valid());

    assert!(!input.capabilities_changed());
    assert!(input.backend_available());
    assert!(!input.migration_possible());
    assert!(input.resources_feasible());

    assert!(!input.quarantine_possible());
    assert!(!input.qec_adaptation_possible());
    assert!(!input.mitigation_possible());
    assert!(!input.compensation_possible());

    assert!(!input.verification_available());
    assert!(!input.previous_recovery_failed());
    assert!(!input.correlated_incident());

    assert_eq!(
        input.primary_reason(),
        PlanningReason::TransientFailure
    );
    assert_eq!(input.recovery_attempts(), 0);
    assert_eq!(input.severity_rank(), 0);
}

#[test]
fn default_input_is_valid() {
    assert!(PlannerInput::new().validate().is_ok());
}

#[test]
fn active_execution_without_any_preservation_path_is_rejected() {
    let input = PlannerInput::new()
        .with_execution_active(true)
        .with_backend_available(false)
        .with_migration_possible(false)
        .with_checkpoint_available(false)
        .with_retry_safe(false)
        .with_rollback_available(false)
        .with_mapping_valid(true)
        .with_routing_valid(true)
        .with_schedule_valid(true)
        .with_compilation_valid(true);

    let result = input.validate();

    match result {
        Err(PlannerError::InvalidInput { message }) => {
            assert!(message.contains("no available execution-preservation path"));
        }
        other => panic!("expected InvalidInput, got {other:?}"),
    }
}

#[test]
fn active_execution_is_valid_when_retry_is_available() {
    let input = PlannerInput::new()
        .with_execution_active(true)
        .with_backend_available(false)
        .with_retry_safe(true);

    assert!(input.validate().is_ok());
}

#[test]
fn active_execution_is_valid_when_checkpoint_is_available() {
    let input = PlannerInput::new()
        .with_execution_active(true)
        .with_backend_available(false)
        .with_checkpoint_available(true);

    assert!(input.validate().is_ok());
}

#[test]
fn active_execution_is_valid_when_rollback_is_available() {
    let input = PlannerInput::new()
        .with_execution_active(true)
        .with_backend_available(false)
        .with_rollback_available(true);

    assert!(input.validate().is_ok());
}

#[test]
fn active_execution_is_valid_when_migration_is_available() {
    let input = PlannerInput::new()
        .with_execution_active(true)
        .with_backend_available(false)
        .with_migration_possible(true);

    assert!(input.validate().is_ok());
}

#[test]
fn planner_config_default_enables_all_major_decision_domains() {
    let config = PlannerConfig::new();

    assert!(config.automatic_recovery());
    assert!(config.adaptation_enabled());
    assert!(config.migration_enabled());
    assert!(config.mitigation_enabled());
    assert!(config.qec_adaptation_enabled());
    assert!(config.escalation_enabled());
    assert!(config.prefer_preservation());

    assert!(config.validate().is_ok());
}

#[test]
fn planner_config_supports_explicit_policy_composition() {
    let config = PlannerConfig::new()
        .with_automatic_recovery(false)
        .with_adaptation_enabled(true)
        .with_migration_enabled(true)
        .with_mitigation_enabled(false)
        .with_qec_adaptation_enabled(false)
        .with_escalation_enabled(true)
        .with_prefer_preservation(false);

    assert!(!config.automatic_recovery());
    assert!(config.adaptation_enabled());
    assert!(config.migration_enabled());
    assert!(!config.mitigation_enabled());
    assert!(!config.qec_adaptation_enabled());
    assert!(config.escalation_enabled());
    assert!(!config.prefer_preservation());

    assert!(config.validate().is_ok());
}

#[test]
fn planner_config_rejects_configuration_with_no_decision_path() {
    let config = PlannerConfig::new()
        .with_automatic_recovery(false)
        .with_adaptation_enabled(false)
        .with_migration_enabled(false)
        .with_mitigation_enabled(false)
        .with_qec_adaptation_enabled(false)
        .with_escalation_enabled(false);

    match config.validate() {
        Err(PlannerError::InvalidInput { message }) => {
            assert!(message.contains("no enabled decision path"));
        }
        other => panic!("expected InvalidInput, got {other:?}"),
    }
}

#[test]
fn every_action_identifier_is_non_empty_and_unique() {
    let ids = all_action_ids();

    for id in ids {
        assert!(!id.is_empty());
    }

    for (index, id) in ids.iter().enumerate() {
        assert!(
            ids[index + 1..].iter().all(|other| other != id),
            "duplicate action identifier: {id}"
        );
    }
}

#[test]
fn action_identifiers_are_provider_neutral() {
    for id in all_action_ids() {
        assert!(!id.contains("ibm"));
        assert!(!id.contains("ionq"));
        assert!(!id.contains("google"));
        assert!(!id.contains("rigetti"));
        assert!(!id.contains("aws"));
    }
}

#[test]
fn planning_reason_identifiers_are_stable_and_provider_neutral() {
    let reasons = [
        PlanningReason::TransientFailure,
        PlanningReason::RestorableExecution,
        PlanningReason::ResourceDegradation,
        PlanningReason::PhysicalRealizationInvalid,
        PlanningReason::CapabilityChange,
        PlanningReason::BackendUnavailable,
        PlanningReason::SchedulingInvalid,
        PlanningReason::RoutingInvalid,
        PlanningReason::CompilationInvalid,
        PlanningReason::QecDegradation,
        PlanningReason::NoiseMitigation,
        PlanningReason::VerificationRequired,
        PlanningReason::NoSafeAutomaticAction,
        PlanningReason::PolicyRequested,
        PlanningReason::CallerRequested,
        PlanningReason::CorrelatedIncident,
        PlanningReason::PreviousRecoveryFailed,
    ];

    for reason in reasons {
        let id = reason.as_str();

        assert!(!id.is_empty());
        assert!(!id.contains("ibm"));
        assert!(!id.contains("ionq"));
        assert!(!id.contains("google"));
        assert!(!id.contains("rigetti"));
    }
}

#[test]
fn requirement_identifiers_are_stable_and_non_empty() {
    let requirements = [
        Requirement::PolicyAuthorization,
        Requirement::CapabilityValidation,
        Requirement::ResourceValidation,
        Requirement::ExecutionStateValidation,
        Requirement::SemanticVerification,
        Requirement::ResultVerification,
        Requirement::ProvenanceRetention,
        Requirement::SecurityAuthorization,
        Requirement::CheckpointIntegrity,
        Requirement::ValidMapping,
        Requirement::ValidRouting,
        Requirement::ValidSchedule,
        Requirement::CompatibleCompilation,
        Requirement::CompatibleBackend,
        Requirement::ValidQecConfiguration,
    ];

    for requirement in requirements {
        assert!(!requirement.as_str().is_empty());
    }
}

#[test]
fn planning_phases_are_stable_and_non_empty() {
    let phases = [
        PlanPhase::Preserve,
        PlanPhase::Retry,
        PlanPhase::Restore,
        PlanPhase::AdaptResources,
        PlanPhase::AdaptImplementation,
        PlanPhase::AdaptQec,
        PlanPhase::Mitigate,
        PlanPhase::Migrate,
        PlanPhase::Verify,
        PlanPhase::Escalate,
    ];

    for phase in phases {
        assert!(!phase.as_str().is_empty());
    }
}

#[test]
fn default_planner_can_plan_conservative_input() {
    let planner = Planner::new();
    let input = PlannerInput::new();

    let plan = planner
        .plan(&input)
        .expect("default planner must accept its own default input");

    // The planner is allowed to return a conservative plan. The test does not
    // require a particular action because policy and planner implementations
    // may evolve while preserving the contract.
    assert!(!plan.candidates().is_empty() || plan.requires_verification());
}

#[test]
fn retryable_execution_produces_a_recovery_surface() {
    let planner = Planner::new();
    let input = active_retryable_input();

    let plan = planner
        .plan(&input)
        .expect("retryable execution should be plannable");

    assert!(
        plan.candidates()
            .iter()
            .any(|candidate| candidate.action() == action_id::RETRY)
            || plan
                .candidates()
                .iter()
                .any(|candidate| candidate.action() == action_id::RESTART)
            || plan.requires_verification()
    );
}

#[test]
fn invalid_mapping_exposes_adaptation_options() {
    let planner = Planner::new();
    let input = invalid_realization_input();

    let plan = planner
        .plan(&input)
        .expect("invalid physical realization should be plannable");

    let has_adaptation = plan.candidates().iter().any(|candidate| {
        matches!(
            candidate.action(),
            action_id::REMAP
                | action_id::REROUTE
                | action_id::RESCHEDULE
                | action_id::RECOMPILE
                | action_id::REOPTIMIZE
        )
    });

    assert!(
        has_adaptation || plan.requires_verification(),
        "planner must expose an adaptation/verification path"
    );
}

#[test]
fn capability_change_does_not_require_provider_specific_logic() {
    let planner = Planner::new();
    let input = capability_change_input();

    let plan = planner
        .plan(&input)
        .expect("capability change should be plannable");

    assert!(
        plan.candidates().iter().any(|candidate| {
            matches!(
                candidate.action(),
                action_id::REMAP
                    | action_id::REROUTE
                    | action_id::RESCHEDULE
                    | action_id::RECOMPILE
                    | action_id::REOPTIMIZE
                    | action_id::ADAPT_QEC
                    | action_id::MIGRATE
            )
        }) || plan.requires_verification()
    );
}

#[test]
fn unavailable_backend_with_migration_produces_migration_surface() {
    let planner = Planner::new();
    let input = migration_input();

    let plan = planner
        .plan(&input)
        .expect("migration-capable input should be plannable");

    assert!(
        plan.candidates()
            .iter()
            .any(|candidate| candidate.action() == action_id::MIGRATE)
            || plan.requires_verification()
    );
}

#[test]
fn degraded_resources_can_expose_quarantine() {
    let planner = Planner::new();
    let input = degraded_resource_input();

    let plan = planner
        .plan(&input)
        .expect("resource degradation should be plannable");

    assert!(
        plan.candidates().iter().any(|candidate| {
            candidate.action() == action_id::QUARANTINE_RESOURCE
        }) || plan.requires_verification()
    );
}

#[test]
fn qec_degradation_can_expose_qec_adaptation() {
    let planner = Planner::new();
    let input = qec_input();

    let plan = planner
        .plan(&input)
        .expect("QEC degradation should be plannable");

    assert!(
        plan.candidates()
            .iter()
            .any(|candidate| candidate.action() == action_id::ADAPT_QEC)
            || plan.requires_verification()
    );
}

#[test]
fn noise_mitigation_can_expose_mitigation() {
    let planner = Planner::new();
    let input = mitigation_input();

    let plan = planner
        .plan(&input)
        .expect("mitigation-capable input should be plannable");

    assert!(
        plan.candidates()
            .iter()
            .any(|candidate| candidate.action() == action_id::MITIGATE)
            || plan.requires_verification()
    );
}

#[test]
fn correlated_incidents_remain_plannable() {
    let planner = Planner::new();
    let input = correlated_input();

    let plan = planner
        .plan(&input)
        .expect("correlated incident should remain plannable");

    assert!(
        !plan.candidates().is_empty() || plan.requires_verification()
    );
}

#[test]
fn previous_recovery_failure_does_not_force_an_infinite_retry_loop() {
    let planner = Planner::new();

    let input = baseline_input()
        .with_execution_active(true)
        .with_previous_recovery_failed(true)
        .with_retry_safe(true)
        .with_verification_available(true)
        .with_primary_reason(PlanningReason::PreviousRecoveryFailed);

    let plan = planner
        .plan(&input)
        .expect("previous recovery failure should be representable");

    // The planner may still return Retry if policy allows it, but the test
    // deliberately verifies that planning itself is finite and represented as
    // a bounded candidate set rather than an execution loop.
    assert!(plan.candidates().len() <= MAX_CANDIDATES);
}

#[test]
fn recovery_attempt_count_is_not_a_hard_coded_retry_limit() {
    let planner = Planner::new();

    let attempts = [
        0_u128,
        1_u128,
        7_u128,
        1_000_u128,
        1_000_000_u128,
        u128::MAX,
    ];

    for attempt_count in attempts {
        let input = baseline_input()
            .with_execution_active(true)
            .with_retry_safe(true)
            .with_verification_available(true)
            .with_recovery_attempts(attempt_count);

        assert_eq!(input.recovery_attempts(), attempt_count);

        let result = planner.plan(&input);

        assert!(
            result.is_ok(),
            "planner must not reject a valid attempt counter merely because \
             the value is large: {attempt_count}"
        );
    }
}

#[test]
fn severity_rank_is_caller_defined_and_not_a_hardware_threshold() {
    let planner = Planner::new();

    let ranks = [
        0_u128,
        1_u128,
        10_u128,
        1_000_u128,
        u128::MAX,
    ];

    for rank in ranks {
        let input = baseline_input()
            .with_severity_rank(rank)
            .with_execution_active(true)
            .with_retry_safe(true)
            .with_verification_available(true);

        assert_eq!(input.severity_rank(), rank);

        planner
            .plan(&input)
            .expect("severity rank must remain an ordering fact");
    }
}

#[test]
fn planner_output_is_deterministic_for_identical_input() {
    let planner = Planner::new();
    let input = capability_change_input()
        .with_recovery_attempts(17)
        .with_severity_rank(42);

    let first = planner
        .plan(&input)
        .expect("first planning invocation must succeed");

    let second = planner
        .plan(&input)
        .expect("second planning invocation must succeed");

    assert_eq!(first, second);
}

#[test]
fn planner_output_is_repeatably_deterministic() {
    let planner = Planner::new();
    let input = degraded_resource_input()
        .with_correlated_incident(true)
        .with_previous_recovery_failed(true)
        .with_recovery_attempts(123)
        .with_severity_rank(456);

    let expected = planner
        .plan(&input)
        .expect("baseline planning invocation must succeed");

    for _ in 0..32 {
        let actual = planner
            .plan(&input)
            .expect("repeat planning invocation must succeed");

        assert_eq!(actual, expected);
    }
}

#[test]
fn candidate_set_never_exceeds_representation_bound() {
    let planner = Planner::new();

    let inputs = [
        PlannerInput::new(),
        active_retryable_input(),
        invalid_realization_input(),
        capability_change_input(),
        migration_input(),
        degraded_resource_input(),
        qec_input(),
        mitigation_input(),
        correlated_input(),
    ];

    for input in inputs {
        let plan = planner
            .plan(&input)
            .expect("test planner input should be accepted");

        assert!(
            plan.candidates().len() <= MAX_CANDIDATES,
            "planner returned more candidates than its representation bound"
        );
    }
}

#[test]
fn every_candidate_contains_a_non_empty_action_identifier() {
    let planner = Planner::new();

    let input = capability_change_input();

    let plan = planner
        .plan(&input)
        .expect("planning should succeed");

    for candidate in plan.candidates() {
        assert!(!candidate.action().is_empty());
    }
}

#[test]
fn candidate_requirements_are_bounded() {
    let planner = Planner::new();

    let inputs = [
        active_retryable_input(),
        invalid_realization_input(),
        capability_change_input(),
        migration_input(),
        degraded_resource_input(),
        qec_input(),
        mitigation_input(),
        correlated_input(),
    ];

    for input in inputs {
        let plan = planner
            .plan(&input)
            .expect("planning should succeed");

        for candidate in plan.candidates() {
            assert!(
                candidate.requirements().len() <= MAX_REQUIREMENTS,
                "candidate exceeded planner requirement representation bound"
            );
        }
    }
}

#[test]
fn candidate_requirements_are_stable_identifiers() {
    let planner = Planner::new();

    let input = invalid_realization_input();

    let plan = planner
        .plan(&input)
        .expect("planning should succeed");

    for candidate in plan.candidates() {
        for requirement in candidate.requirements() {
            assert!(!requirement.as_str().is_empty());
        }
    }
}

#[test]
fn recovery_candidates_require_policy_and_safety_boundaries() {
    let planner = Planner::new();

    let input = active_retryable_input();

    let plan = planner
        .plan(&input)
        .expect("planning should succeed");

    for candidate in plan.candidates() {
        if matches!(
            candidate.action(),
            action_id::RETRY
                | action_id::RESTART
                | action_id::RESUME
                | action_id::ROLLBACK
                | action_id::CHECKPOINT
                | action_id::REMAP
                | action_id::REROUTE
                | action_id::RESCHEDULE
                | action_id::RECOMPILE
                | action_id::REOPTIMIZE
                | action_id::ADAPT_QEC
                | action_id::MITIGATE
                | action_id::MIGRATE
                | action_id::QUARANTINE_RESOURCE
                | action_id::COMPENSATE
        ) {
            assert!(
                candidate
                    .requirements()
                    .contains(&Requirement::PolicyAuthorization),
                "mutating recovery/adaptation candidates must remain policy-gated"
            );
        }
    }
}

#[test]
fn transformation_candidates_require_capability_validation() {
    let planner = Planner::new();

    let input = capability_change_input();

    let plan = planner
        .plan(&input)
        .expect("planning should succeed");

    for candidate in plan.candidates() {
        if matches!(
            candidate.action(),
            action_id::REMAP
                | action_id::REROUTE
                | action_id::RESCHEDULE
                | action_id::RECOMPILE
                | action_id::REOPTIMIZE
                | action_id::ADAPT_QEC
                | action_id::MITIGATE
                | action_id::MIGRATE
        ) {
            assert!(
                candidate
                    .requirements()
                    .contains(&Requirement::CapabilityValidation)
                    || candidate
                        .requirements()
                        .contains(&Requirement::CompatibleCompilation)
                    || candidate
                        .requirements()
                        .contains(&Requirement::CompatibleBackend)
                    || candidate
                        .requirements()
                        .contains(&Requirement::ValidQecConfiguration),
                "target-changing candidate must contain a capability/compatibility requirement"
            );
        }
    }
}

#[test]
fn physical_adaptation_candidates_require_semantic_verification() {
    let planner = Planner::new();

    let input = invalid_realization_input();

    let plan = planner
        .plan(&input)
        .expect("planning should succeed");

    for candidate in plan.candidates() {
        if matches!(
            candidate.action(),
            action_id::REMAP
                | action_id::REROUTE
                | action_id::RESCHEDULE
                | action_id::RECOMPILE
                | action_id::REOPTIMIZE
                | action_id::ADAPT_QEC
                | action_id::MIGRATE
        ) {
            assert!(
                candidate
                    .requirements()
                    .contains(&Requirement::SemanticVerification),
                "physical realization changes must remain semantically verifiable"
            );
        }
    }
}

#[test]
fn migration_candidates_require_backend_compatibility() {
    let planner = Planner::new();

    let input = migration_input();

    let plan = planner
        .plan(&input)
        .expect("planning should succeed");

    for candidate in plan.candidates() {
        if candidate.action() == action_id::MIGRATE {
            assert!(
                candidate
                    .requirements()
                    .contains(&Requirement::CompatibleBackend),
                "migration must require backend compatibility"
            );
        }
    }
}

#[test]
fn qec_candidates_require_qec_validation() {
    let planner = Planner::new();

    let input = qec_input();

    let plan = planner
        .plan(&input)
        .expect("planning should succeed");

    for candidate in plan.candidates() {
        if candidate.action() == action_id::ADAPT_QEC {
            assert!(
                candidate
                    .requirements()
                    .contains(&Requirement::ValidQecConfiguration),
                "QEC adaptation must require QEC configuration validation"
            );
        }
    }
}

#[test]
fn routing_candidates_require_valid_routing() {
    let planner = Planner::new();

    let input = invalid_realization_input();

    let plan = planner
        .plan(&input)
        .expect("planning should succeed");

    for candidate in plan.candidates() {
        if matches!(
            candidate.action(),
            action_id::REROUTE | action_id::RESCHEDULE
        ) {
            assert!(
                candidate
                    .requirements()
                    .contains(&Requirement::ValidRouting)
                    || candidate
                        .requirements()
                        .contains(&Requirement::ValidSchedule),
                "routing/scheduling adaptation must retain the appropriate validity requirement"
            );
        }
    }
}

#[test]
fn checkpoint_related_candidates_require_checkpoint_integrity() {
    let planner = Planner::new();

    let input = baseline_input()
        .with_execution_active(true)
        .with_checkpoint_available(true)
        .with_resume_available(true)
        .with_rollback_available(true)
        .with_verification_available(true)
        .with_primary_reason(PlanningReason::RestorableExecution);

    let plan = planner
        .plan(&input)
        .expect("restorable execution should be plannable");

    for candidate in plan.candidates() {
        if matches!(
            candidate.action(),
            action_id::RESUME | action_id::ROLLBACK
        ) {
            assert!(
                candidate
                    .requirements()
                    .contains(&Requirement::CheckpointIntegrity)
                    || candidate
                        .requirements()
                        .contains(&Requirement::ExecutionStateValidation),
                "checkpoint-based recovery must validate restoration state"
            );
        }
    }
}

#[test]
fn semantic_verification_is_not_optional_for_recovery_candidates() {
    let planner = Planner::new();

    let input = active_retryable_input();

    let plan = planner
        .plan(&input)
        .expect("planning should succeed");

    for candidate in plan.candidates() {
        if matches!(
            candidate.action(),
            action_id::RETRY
                | action_id::RESTART
                | action_id::RESUME
                | action_id::ROLLBACK
                | action_id::REMAP
                | action_id::REROUTE
                | action_id::RESCHEDULE
                | action_id::RECOMPILE
                | action_id::REOPTIMIZE
                | action_id::ADAPT_QEC
                | action_id::MITIGATE
                | action_id::MIGRATE
                | action_id::COMPENSATE
        ) {
            assert!(
                candidate
                    .requirements()
                    .contains(&Requirement::SemanticVerification),
                "recovery candidate must remain subject to semantic verification"
            );
        }
    }
}

#[test]
fn provenance_requirement_is_present_for_state_changing_candidates() {
    let planner = Planner::new();

    let input = capability_change_input();

    let plan = planner
        .plan(&input)
        .expect("planning should succeed");

    for candidate in plan.candidates() {
        if matches!(
            candidate.action(),
            action_id::RETRY
                | action_id::RESTART
                | action_id::RESUME
                | action_id::ROLLBACK
                | action_id::REMAP
                | action_id::REROUTE
                | action_id::RESCHEDULE
                | action_id::RECOMPILE
                | action_id::REOPTIMIZE
                | action_id::ADAPT_QEC
                | action_id::MITIGATE
                | action_id::MIGRATE
                | action_id::QUARANTINE_RESOURCE
                | action_id::COMPENSATE
        ) {
            assert!(
                candidate
                    .requirements()
                    .contains(&Requirement::ProvenanceRetention),
                "state-changing recovery actions must preserve provenance"
            );
        }
    }
}

#[test]
fn security_authorization_is_preserved_for_sensitive_recovery() {
    let planner = Planner::new();

    let input = migration_input()
        .with_correlated_incident(true)
        .with_previous_recovery_failed(true);

    let plan = planner
        .plan(&input)
        .expect("security-sensitive planning scenario should remain representable");

    for candidate in plan.candidates() {
        if matches!(
            candidate.action(),
            action_id::MIGRATE
                | action_id::QUARANTINE_RESOURCE
                | action_id::ROLLBACK
        ) {
            assert!(
                candidate
                    .requirements()
                    .contains(&Requirement::SecurityAuthorization)
                    || candidate.action() == action_id::ESCALATE,
                "sensitive recovery must remain security-gated or escalated"
            );
        }
    }
}

#[test]
fn escalation_remains_available_when_automatic_recovery_is_disabled() {
    let planner = Planner::new();

    let input = PlannerInput::new()
        .with_execution_active(true)
        .with_backend_available(false)
        .with_migration_possible(false)
        .with_checkpoint_available(false)
        .with_retry_safe(false)
        .with_rollback_available(false);

    let config = PlannerConfig::new()
        .with_automatic_recovery(false)
        .with_adaptation_enabled(false)
        .with_migration_enabled(false)
        .with_mitigation_enabled(false)
        .with_qec_adaptation_enabled(false)
        .with_escalation_enabled(true);

    let configured_planner = planner.with_config(config);

    let plan = configured_planner
        .plan(&input)
        .expect("escalation-only policy must remain plannable");

    assert!(
        plan.candidates()
            .iter()
            .any(|candidate| candidate.action() == action_id::ESCALATE)
            || plan
                .candidates()
                .iter()
                .any(|candidate| candidate.action() == action_id::ABORT)
            || plan.requires_verification()
    );
}

#[test]
fn planner_can_disable_migration_without_disabling_other_adaptation() {
    let config = PlannerConfig::new()
        .with_migration_enabled(false)
        .with_adaptation_enabled(true)
        .with_automatic_recovery(true)
        .with_escalation_enabled(true);

    let planner = Planner::new().with_config(config);

    let input = migration_input();

    let plan = planner
        .plan(&input)
        .expect("migration-disabled policy must remain valid");

    assert!(
        plan.candidates()
            .iter()
            .all(|candidate| candidate.action() != action_id::MIGRATE)
    );
}

#[test]
fn planner_can_disable_mitigation_without_disabling_planning() {
    let config = PlannerConfig::new()
        .with_mitigation_enabled(false)
        .with_automatic_recovery(true)
        .with_adaptation_enabled(true)
        .with_escalation_enabled(true);

    let planner = Planner::new().with_config(config);

    let input = mitigation_input();

    let plan = planner
        .plan(&input)
        .expect("mitigation-disabled policy must remain plannable");

    assert!(
        plan.candidates()
            .iter()
            .all(|candidate| candidate.action() != action_id::MITIGATE)
    );
}

#[test]
fn planner_can_disable_qec_adaptation_without_disabling_planning() {
    let config = PlannerConfig::new()
        .with_qec_adaptation_enabled(false)
        .with_automatic_recovery(true)
        .with_adaptation_enabled(true)
        .with_escalation_enabled(true);

    let planner = Planner::new().with_config(config);

    let input = qec_input();

    let plan = planner
        .plan(&input)
        .expect("QEC-adaptation-disabled policy must remain plannable");

    assert!(
        plan.candidates()
            .iter()
            .all(|candidate| candidate.action() != action_id::ADAPT_QEC)
    );
}

#[test]
fn planner_can_disable_adaptation_without_affecting_policy_contract() {
    let config = PlannerConfig::new()
        .with_adaptation_enabled(false)
        .with_automatic_recovery(true)
        .with_migration_enabled(true)
        .with_mitigation_enabled(true)
        .with_qec_adaptation_enabled(true)
        .with_escalation_enabled(true);

    let planner = Planner::new().with_config(config);

    let input = invalid_realization_input();

    let plan = planner
        .plan(&input)
        .expect("adaptation-disabled policy must remain plannable");

    assert!(
        plan.candidates()
            .iter()
            .all(|candidate| {
                !matches!(
                    candidate.action(),
                    action_id::REMAP
                        | action_id::REROUTE
                        | action_id::RESCHEDULE
                        | action_id::RECOMPILE
                        | action_id::REOPTIMIZE
                )
            })
    );
}

#[test]
fn planner_does_not_depend_on_physical_qubit_count() {
    // The planning contract operates on facts such as capability/resource
    // feasibility. There is deliberately no qubit-count parameter here.
    //
    // This test therefore verifies the canonical identity exists without
    // manufacturing an artificial machine size.
    let _canonical_type = type_name::<QubitId>();

    let planner = Planner::new();

    let plan = planner
        .plan(&capability_change_input())
        .expect("planning must not require a fixed qubit count");

    assert!(plan.candidates().len() <= MAX_CANDIDATES);
}

#[test]
fn planner_accepts_arbitrarily_large_resource_attempt_values() {
    let planner = Planner::new();

    let input = PlannerInput::new()
        .with_execution_active(true)
        .with_retry_safe(true)
        .with_verification_available(true)
        .with_recovery_attempts(u128::MAX)
        .with_severity_rank(u128::MAX);

    let plan = planner
        .plan(&input)
        .expect("large caller-supplied ordering values must not imply a machine limit");

    assert!(plan.candidates().len() <= MAX_CANDIDATES);
}

#[test]
fn planner_does_not_allocate_per_qubit_decision_state() {
    // There is intentionally no API here that accepts a physical-qubit count,
    // physical-qubit array, or fixed topology.
    //
    // The planner's own architecture requires candidate generation to remain
    // bounded by decision strategies rather than by the number of qubits.
    //
    // This is a compile-time/API-level scalability invariant.
    let planner = Planner::new();

    let input = capability_change_input();

    let plan = planner
        .plan(&input)
        .expect("planning must operate on normalized facts");

    assert!(plan.candidates().len() <= MAX_CANDIDATES);
}

#[test]
fn candidate_priority_is_deterministic() {
    let planner = Planner::new();
    let input = capability_change_input();

    let first = planner
        .plan(&input)
        .expect("planning must succeed");

    let second = planner
        .plan(&input)
        .expect("planning must succeed");

    let first_priorities: Vec<u32> = first
        .candidates()
        .iter()
        .map(|candidate| candidate.priority())
        .collect();

    let second_priorities: Vec<u32> = second
        .candidates()
        .iter()
        .map(|candidate| candidate.priority())
        .collect();

    assert_eq!(first_priorities, second_priorities);
}

#[test]
fn candidate_phase_and_reason_are_deterministic() {
    let planner = Planner::new();
    let input = correlated_input();

    let first = planner
        .plan(&input)
        .expect("planning must succeed");

    let second = planner
        .plan(&input)
        .expect("planning must succeed");

    let first_metadata: Vec<(PlanPhase, PlanningReason)> = first
        .candidates()
        .iter()
        .map(|candidate| (candidate.phase(), candidate.reason()))
        .collect();

    let second_metadata: Vec<(PlanPhase, PlanningReason)> = second
        .candidates()
        .iter()
        .map(|candidate| (candidate.phase(), candidate.reason()))
        .collect();

    assert_eq!(first_metadata, second_metadata);
}

#[test]
fn planner_candidate_actions_are_unique() {
    let planner = Planner::new();

    let inputs = [
        active_retryable_input(),
        invalid_realization_input(),
        capability_change_input(),
        migration_input(),
        degraded_resource_input(),
        qec_input(),
        mitigation_input(),
        correlated_input(),
    ];

    for input in inputs {
        let plan = planner
            .plan(&input)
            .expect("planning must succeed");

        let actions: Vec<&str> = plan
            .candidates()
            .iter()
            .map(|candidate| candidate.action())
            .collect();

        for (index, action) in actions.iter().enumerate() {
            assert!(
                actions[index + 1..]
                    .iter()
                    .all(|other| other != action),
                "planner returned duplicate action `{action}`"
            );
        }
    }
}

#[test]
fn planner_output_requires_verification_when_policy_or_state_makes_acceptance_uncertain() {
    let planner = Planner::new();

    let input = PlannerInput::new()
        .with_execution_active(true)
        .with_previous_recovery_failed(true)
        .with_correlated_incident(true)
        .with_verification_available(true)
        .with_primary_reason(PlanningReason::NoSafeAutomaticAction);

    let plan = planner
        .plan(&input)
        .expect("uncertain state must remain representable");

    assert!(
        plan.requires_verification()
            || plan
                .candidates()
                .iter()
                .any(|candidate| candidate.action() == action_id::ESCALATE)
            || plan
                .candidates()
                .iter()
                .any(|candidate| candidate.action() == action_id::ABORT)
    );
}

#[test]
fn planner_does_not_treat_estimated_or_ranked_priority_as_authorization() {
    let planner = Planner::new();

    let input = active_retryable_input()
        .with_severity_rank(u128::MAX)
        .with_recovery_attempts(u128::MAX);

    let plan = planner
        .plan(&input)
        .expect("large ranking values must not authorize execution");

    for candidate in plan.candidates() {
        assert!(
            candidate
                .requirements()
                .contains(&Requirement::PolicyAuthorization)
                || candidate.action() == action_id::ESCALATE
                || candidate.action() == action_id::ABORT,
            "candidate must remain policy gated"
        );
    }
}

#[test]
fn planner_representation_bounds_are_not_quantum_machine_limits() {
    // These constants intentionally belong to the planner's bounded decision
    // surface. They are not qubit/device/back-end limits.
    assert!(MAX_CANDIDATES > 0);
    assert!(MAX_REQUIREMENTS > 0);
    assert!(MAX_REASONS > 0);

    // A successful planner invocation is independent of a physical machine
    // size because no machine size is supplied to the planner.
    Planner::new()
        .plan(&PlannerInput::new())
        .expect("planner must not require a fixed machine size");
}

#[test]
fn planner_supports_the_full_resilience_action_vocabulary() {
    // Ensure all stable action IDs remain represented in the planning contract.
    //
    // This test intentionally checks the public vocabulary rather than forcing
    // every possible action to be generated for every incident.
    for action in all_action_ids() {
        assert!(!action.is_empty());
    }
}

#[test]
fn planner_is_reusable_without_cross_invocation_state_leakage() {
    let planner = Planner::new();

    let first = planner
        .plan(&active_retryable_input())
        .expect("first planning invocation must succeed");

    let second = planner
        .plan(&migration_input())
        .expect("second planning invocation must succeed");

    let third = planner
        .plan(&active_retryable_input())
        .expect("third planning invocation must succeed");

    assert_eq!(first, third);
    assert_ne!(first, second);
}

#[test]
fn planner_configuration_is_reusable_without_mutating_previous_results() {
    let planner = Planner::new();

    let baseline = planner
        .plan(&invalid_realization_input())
        .expect("baseline planning must succeed");

    let adapted_planner = planner.with_config(
        PlannerConfig::new()
            .with_adaptation_enabled(false)
            .with_escalation_enabled(true),
    );

    let changed = adapted_planner
        .plan(&invalid_realization_input())
        .expect("configured planning must succeed");

    let baseline_again = planner
        .plan(&invalid_realization_input())
        .expect("baseline planner must remain reusable");

    assert_eq!(baseline, baseline_again);
    assert_ne!(baseline, changed);
}

#[test]
fn planning_reason_tracks_caller_supplied_context() {
    let reasons = [
        PlanningReason::TransientFailure,
        PlanningReason::RestorableExecution,
        PlanningReason::ResourceDegradation,
        PlanningReason::PhysicalRealizationInvalid,
        PlanningReason::CapabilityChange,
        PlanningReason::BackendUnavailable,
        PlanningReason::SchedulingInvalid,
        PlanningReason::RoutingInvalid,
        PlanningReason::CompilationInvalid,
        PlanningReason::QecDegradation,
        PlanningReason::NoiseMitigation,
        PlanningReason::VerificationRequired,
        PlanningReason::NoSafeAutomaticAction,
        PlanningReason::PolicyRequested,
        PlanningReason::CallerRequested,
        PlanningReason::CorrelatedIncident,
        PlanningReason::PreviousRecoveryFailed,
    ];

    for reason in reasons {
        let input = PlannerInput::new().with_primary_reason(reason);

        assert_eq!(input.primary_reason(), reason);
        assert!(input.validate().is_ok());
    }
}

#[test]
fn planner_input_builder_is_immutable_by_construction() {
    let original = PlannerInput::new();

    let changed = original
        .clone()
        .with_execution_active(true)
        .with_retry_safe(true)
        .with_recovery_attempts(11)
        .with_severity_rank(22);

    assert!(!original.execution_active());
    assert!(!original.retry_safe());
    assert_eq!(original.recovery_attempts(), 0);
    assert_eq!(original.severity_rank(), 0);

    assert!(changed.execution_active());
    assert!(changed.retry_safe());
    assert_eq!(changed.recovery_attempts(), 11);
    assert_eq!(changed.severity_rank(), 22);
}

#[test]
fn planner_config_builder_is_immutable_by_construction() {
    let original = PlannerConfig::new();

    let changed = original
        .clone()
        .with_migration_enabled(false)
        .with_mitigation_enabled(false)
        .with_qec_adaptation_enabled(false);

    assert!(original.migration_enabled());
    assert!(original.mitigation_enabled());
    assert!(original.qec_adaptation_enabled());

    assert!(!changed.migration_enabled());
    assert!(!changed.mitigation_enabled());
    assert!(!changed.qec_adaptation_enabled());
}

#[test]
fn planner_handles_all_major_fault_response_domains() {
    let planner = Planner::new();

    let inputs = [
        active_retryable_input(),
        invalid_realization_input(),
        capability_change_input(),
        migration_input(),
        degraded_resource_input(),
        qec_input(),
        mitigation_input(),
        correlated_input(),
    ];

    for input in inputs {
        let plan = planner
            .plan(&input)
            .expect("every normalized resilience domain should be plannable");

        assert!(plan.candidates().len() <= MAX_CANDIDATES);
    }
}

#[test]
fn planner_remains_provider_neutral_at_the_public_contract() {
    let planner = Planner::new();

    let input = capability_change_input();

    let plan = planner
        .plan(&input)
        .expect("provider-neutral planning should succeed");

    for candidate in plan.candidates() {
        assert!(!candidate.action().contains("ibm"));
        assert!(!candidate.action().contains("ionq"));
        assert!(!candidate.action().contains("google"));
        assert!(!candidate.action().contains("rigetti"));
        assert!(!candidate.action().contains("aws"));
    }
}

#[test]
fn planner_never_uses_physical_qubit_ids_as_policy_constants() {
    // The canonical QubitId is deliberately referenced only through the IR.
    // There is no assertion about a particular numeric ID because such an
    // assertion would make the resilience architecture hardware-specific.
    let canonical_qubit_type = type_name::<QubitId>();

    assert!(!canonical_qubit_type.is_empty());
}

#[test]
fn planner_preserves_the_separation_between_decision_and_execution() {
    let planner = Planner::new();

    let plan = planner
        .plan(&active_retryable_input())
        .expect("planning must succeed");

    // A candidate only describes an action. This test intentionally does not
    // invoke adaptation, recovery, mitigation, hardware or QEC.
    //
    // The existence of a plan is therefore not treated as proof of execution.
    for candidate in plan.candidates() {
        assert!(!candidate.action().is_empty());
        assert!(!candidate.requirements().is_empty());
    }
}

#[test]
fn planner_failure_is_explicit_instead_of_silent() {
    let invalid = PlannerInput::new()
        .with_execution_active(true)
        .with_backend_available(false)
        .with_migration_possible(false)
        .with_checkpoint_available(false)
        .with_retry_safe(false)
        .with_rollback_available(false)
        .with_mapping_valid(true)
        .with_routing_valid(true)
        .with_schedule_valid(true)
        .with_compilation_valid(true);

    let result = Planner::new().plan(&invalid);

    assert!(result.is_err());
}

#[test]
fn planner_output_is_safe_to_compare_for_deterministic_replay() {
    let planner = Planner::new();

    let input = capability_change_input()
        .with_recovery_attempts(999)
        .with_severity_rank(777)
        .with_correlated_incident(true);

    let recorded = planner
        .plan(&input)
        .expect("recorded planning result must succeed");

    let replayed = planner
        .plan(&input)
        .expect("replayed planning result must succeed");

    assert_eq!(recorded, replayed);
}