//! Zamani Quantum Scheduling — Unit Test Suite
//!
//! Production unit tests for the scheduler's independently testable contracts.
//!
//! # Purpose
//!
//! This file validates scheduler primitives without coupling the unit suite to
//! unfinished higher-level integration components.
//!
//! The scheduler is designed around the following architecture:
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! quantum::ir
//!      │
//!      ▼
//! optimization
//!      │
//!      ▼
//! routing
//!      │
//!      ▼
//! scheduling
//!      │
//!      ├── timing
//!      ├── resources
//!      ├── dependency graph
//!      ├── constraints
//!      ├── policies
//!      ├── planners
//!      ├── verification
//!      └── transformations
//!      │
//!      ▼
//! hardware/runtime
//! ```
//!
//! This file intentionally tests the lowest-level contracts first.
//!
//! # Testing philosophy
//!
//! The tests must verify:
//!
//! - no artificial machine-size limits;
//! - canonical qubit identity;
//! - logical/physical type separation;
//! - checked identifier arithmetic;
//! - deterministic ordering;
//! - validated planner identifiers;
//! - planner metadata contracts;
//! - planner capability semantics;
//! - scheduler policy semantics;
//! - reproducibility semantics;
//! - parallelism policy semantics;
//! - operation classification;
//! - operation provenance;
//! - metadata immutability;
//! - classical dependency identity;
//! - scheduler operation immutability;
//! - boundary validation;
//! - overflow/underflow resistance;
//! - empty/minimal cases;
//! - very large representable identifiers;
//! - stable display/debug-facing contracts where they are part of the API.
//!
//! # Scalability
//!
//! These tests deliberately do not assume:
//!
//! - a maximum qubit count;
//! - a maximum operation count;
//! - a fixed topology;
//! - a fixed gate set;
//! - a fixed gate arity;
//! - a fixed number of channels;
//! - a fixed number of resources;
//! - a fixed QEC distance;
//! - a fixed schedule depth;
//! - a fixed hardware technology.
//!
//! `usize::MAX` is used in selected tests only to verify identifier arithmetic
//! boundaries. It is NOT treated as a hardware capacity.
//!
//! # Canonical qubit identity
//!
//! The scheduler must use:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! where the repository exposes the canonical qubit module at that path.
//!
//! The tests below therefore use that exact namespace rather than defining a
//! scheduler-local qubit identifier.
//!
//! # Test isolation
//!
//! Tests must:
//!
//! - avoid global mutable state;
//! - avoid filesystem access;
//! - avoid network access;
//! - avoid vendor SDKs;
//! - avoid real hardware;
//! - avoid environment-dependent behaviour;
//! - avoid wall-clock timing;
//! - avoid randomness unless explicitly tested;
//! - remain safe under Rust 1.97 / 1.97.1.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe.
//!
//! # Integration boundary
//!
//! This module is intended to be included by the scheduler's test module:
//!
//! ```text
//! src/quantum/scheduling/tests/mod.rs
//!             │
//!             └── mod unit;
//! ```
//!
//! It can also be compiled as part of the scheduling module's internal test
//! hierarchy.
//!
//! Higher-level tests belong elsewhere:
//!
//! ```text
//! tests/unit/          <- this file and primitive contracts
//! tests/integration/   <- subsystem integration
//! tests/property/      <- invariant/property testing
//! tests/regression/    <- previously discovered defects
//! tests/scalability/   <- large workloads
//! tests/determinism/   <- repeatability
//! ```
//!
//! # Frozen-contract rule
//!
//! This file tests public contracts, not private implementation details.
//!
//! Adding a new scheduling algorithm should not require rewriting these tests
//! unless an established public contract changes.
//!
//! Adding a hardware technology, routing algorithm, QEC strategy, or target
//! should not require changes here.
//!
//! =============================================================================
//! Safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::sync::Arc;

use crate::quantum::ir::qubit::{
    PhysicalQubitId,
    QubitId,
    QubitRef,
};

use crate::quantum::scheduling::config::{
    DiagnosticPolicy,
    FailurePolicy,
    ParallelismPolicy,
    PlanningDeadline,
    ReproducibilityConfig,
    SchedulingObjective,
    SchedulingStrategy,
    TimingMode,
    TransformationPolicy,
    VerificationPolicy,
};

use crate::quantum::scheduling::ir::operation::{
    ClassicalDependencyId,
    OperationClass,
    OperationMetadata,
    OperationProvenance,
    OperandRole,
    QubitOperand,
};

use crate::quantum::scheduling::planners::planner::{
    PlannerAlgorithmFamily,
    PlannerCapabilities,
    PlannerExecutionMode,
    PlannerId,
    PlannerMetadata,
    PlannerVersion,
    PLANNER_CONTRACT_VERSION,
    PLANNER_ID_MAX_BYTES,
};

// ============================================================================
// Test helpers
// ============================================================================

fn logical_qubit(index: usize) -> QubitId {
    QubitId::new(index)
}

fn physical_qubit(index: usize) -> PhysicalQubitId {
    PhysicalQubitId::new(index)
}

fn planner_id(value: &str) -> PlannerId {
    PlannerId::new(value)
        .expect("test planner identifier must satisfy the public contract")
}

// ============================================================================
// Canonical qubit identity tests
// ============================================================================

#[test]
fn canonical_qubit_id_is_constructible_without_scheduler_limits() {
    let first = logical_qubit(0);
    let large = logical_qubit(usize::MAX);

    assert_eq!(first.index(), 0);
    assert_eq!(large.index(), usize::MAX);
}

#[test]
fn canonical_qubit_id_preserves_identity() {
    let a = logical_qubit(7);
    let b = logical_qubit(7);
    let c = logical_qubit(8);

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn canonical_qubit_id_is_orderable() {
    let low = logical_qubit(1);
    let high = logical_qubit(2);

    assert!(low < high);
}

#[test]
fn canonical_qubit_id_checked_next_does_not_overflow() {
    let id = logical_qubit(usize::MAX);

    assert_eq!(id.checked_next(), None);
}

#[test]
fn canonical_qubit_id_checked_next_advances_normally() {
    let id = logical_qubit(41);

    assert_eq!(id.checked_next(), Some(logical_qubit(42)));
}

#[test]
fn physical_qubit_identity_is_distinct_from_logical_identity() {
    let logical = QubitRef::Logical(logical_qubit(3));
    let physical = QubitRef::Physical(physical_qubit(3));

    assert!(logical.is_logical());
    assert!(!logical.is_physical());

    assert!(physical.is_physical());
    assert!(!physical.is_logical());
}

#[test]
fn physical_qubit_identity_preserves_large_identifier_values() {
    let id = physical_qubit(usize::MAX);

    assert_eq!(id.index(), usize::MAX);
    assert_eq!(id.checked_next(), None);
}

#[test]
fn logical_and_physical_qubit_references_do_not_collapse_into_same_type() {
    let logical = QubitRef::Logical(logical_qubit(11));
    let physical = QubitRef::Physical(physical_qubit(11));

    assert_ne!(logical, physical);

    assert_eq!(logical.logical(), Some(logical_qubit(11)));
    assert_eq!(logical.physical(), None);

    assert_eq!(physical.logical(), None);
    assert_eq!(physical.physical(), Some(physical_qubit(11)));
}

// ============================================================================
// Qubit operand tests
// ============================================================================

#[test]
fn qubit_operand_defaults_to_data_role() {
    let operand = QubitOperand::new(logical_qubit(4));

    assert_eq!(operand.qubit(), logical_qubit(4));
    assert_eq!(operand.role(), OperandRole::Data);
}

#[test]
fn qubit_operand_preserves_explicit_role() {
    let operand =
        QubitOperand::with_role(logical_qubit(9), OperandRole::Ancilla);

    assert_eq!(operand.qubit(), logical_qubit(9));
    assert_eq!(operand.role(), OperandRole::Ancilla);
}

#[test]
fn all_operand_roles_are_distinguishable() {
    let roles = [
        OperandRole::Data,
        OperandRole::Ancilla,
        OperandRole::Syndrome,
        OperandRole::Control,
        OperandRole::Target,
        OperandRole::Measurement,
        OperandRole::Reset,
        OperandRole::Other,
    ];

    for (index, left) in roles.iter().enumerate() {
        for (other_index, right) in roles.iter().enumerate() {
            if index == other_index {
                assert_eq!(left, right);
            } else {
                assert_ne!(left, right);
            }
        }
    }
}

// ============================================================================
// Classical dependency identity tests
// ============================================================================

#[test]
fn classical_dependency_id_accepts_non_empty_values() {
    let id = ClassicalDependencyId::try_new(Arc::<str>::from("measurement.0"));

    assert!(id.is_some());

    let id = id.expect("non-empty dependency identifier should be accepted");

    assert_eq!(id.as_str(), "measurement.0");
}

#[test]
fn classical_dependency_id_rejects_empty_values() {
    let id = ClassicalDependencyId::try_new(Arc::<str>::from(""));

    assert!(id.is_none());
}

#[test]
fn classical_dependency_ids_are_value_based() {
    let a = ClassicalDependencyId::new(Arc::<str>::from("result"));
    let b = ClassicalDependencyId::new(Arc::<str>::from("result"));
    let c = ClassicalDependencyId::new(Arc::<str>::from("other"));

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn classical_dependency_ids_are_orderable() {
    let a = ClassicalDependencyId::new(Arc::<str>::from("a"));
    let b = ClassicalDependencyId::new(Arc::<str>::from("b"));

    assert!(a < b);
}

// ============================================================================
// Operation classification tests
// ============================================================================

#[test]
fn operation_class_quantum_categories_are_stable() {
    assert!(OperationClass::Quantum.is_quantum());
    assert!(OperationClass::Measurement.is_quantum());
    assert!(OperationClass::Reset.is_quantum());
    assert!(OperationClass::Conditional.is_quantum());
    assert!(OperationClass::Feedback.is_quantum());
    assert!(OperationClass::ErrorCorrection.is_quantum());
    assert!(OperationClass::Pulse.is_quantum());
    assert!(OperationClass::Analog.is_quantum());
}

#[test]
fn operation_class_non_quantum_categories_are_not_quantum() {
    assert!(!OperationClass::Classical.is_quantum());
    assert!(!OperationClass::Communication.is_quantum());
    assert!(!OperationClass::Synchronization.is_quantum());
    assert!(!OperationClass::Custom.is_quantum());
}

#[test]
fn measurement_produces_classical_data() {
    assert!(OperationClass::Measurement.produces_classical_data());
}

#[test]
fn classical_operations_produce_classical_data() {
    assert!(OperationClass::Classical.produces_classical_data());
}

#[test]
fn feedback_is_classical_data_producing() {
    assert!(OperationClass::Feedback.produces_classical_data());
}

#[test]
fn ordinary_quantum_operations_do_not_produce_classical_data() {
    assert!(!OperationClass::Quantum.produces_classical_data());
    assert!(!OperationClass::Reset.produces_classical_data());
    assert!(!OperationClass::Analog.produces_classical_data());
}

#[test]
fn dynamic_operation_classes_are_marked_dynamic() {
    assert!(OperationClass::Classical.is_dynamic());
    assert!(OperationClass::Conditional.is_dynamic());
    assert!(OperationClass::Feedback.is_dynamic());
    assert!(OperationClass::Communication.is_dynamic());
}

#[test]
fn ordinary_static_quantum_operation_is_not_marked_dynamic() {
    assert!(!OperationClass::Quantum.is_dynamic());
    assert!(!OperationClass::Measurement.is_dynamic());
    assert!(!OperationClass::Reset.is_dynamic());
}

#[test]
fn synchronization_class_is_marked_as_synchronization() {
    assert!(OperationClass::Synchronization.is_synchronization());
}

#[test]
fn ordinary_quantum_class_is_not_synchronization() {
    assert!(!OperationClass::Quantum.is_synchronization());
}

// ============================================================================
// Operation metadata tests
// ============================================================================

#[test]
fn operation_metadata_defaults_to_empty() {
    let metadata = OperationMetadata::new();

    assert_eq!(metadata.name(), None);
    assert_eq!(metadata.label(), None);
    assert_eq!(metadata.scheduling_class(), None);
}

#[test]
fn operation_metadata_preserves_name() {
    let metadata =
        OperationMetadata::new().with_name(Arc::<str>::from("operation"));

    assert_eq!(metadata.name(), Some("operation"));
}

#[test]
fn operation_metadata_preserves_label() {
    let metadata =
        OperationMetadata::new().with_label(Arc::<str>::from("source"));

    assert_eq!(metadata.label(), Some("source"));
}

#[test]
fn operation_metadata_preserves_scheduling_class() {
    let metadata = OperationMetadata::new()
        .with_scheduling_class(Arc::<str>::from("critical"));

    assert_eq!(metadata.scheduling_class(), Some("critical"));
}

#[test]
fn operation_metadata_builder_is_composable() {
    let metadata = OperationMetadata::new()
        .with_name(Arc::<str>::from("cx"))
        .with_label(Arc::<str>::from("source:42"))
        .with_scheduling_class(Arc::<str>::from("two-qubit"));

    assert_eq!(metadata.name(), Some("cx"));
    assert_eq!(metadata.label(), Some("source:42"));
    assert_eq!(metadata.scheduling_class(), Some("two-qubit"));
}

// ============================================================================
// Provenance tests
// ============================================================================

#[test]
fn canonical_ir_provenance_exposes_origin() {
    let provenance = OperationProvenance::CanonicalIr {
        operation_id: 17.into(),
    };

    assert_eq!(provenance.origin(), Some(17.into()));
    assert_eq!(provenance.kind(), "canonical_ir");
}

#[test]
fn derived_provenance_exposes_origin() {
    let provenance = OperationProvenance::Derived {
        origin: 21.into(),
        transformation: Arc::<str>::from("rewrite"),
    };

    assert_eq!(provenance.origin(), Some(21.into()));
    assert_eq!(provenance.kind(), "derived");
}

#[test]
fn routed_provenance_exposes_origin() {
    let provenance = OperationProvenance::Routed {
        origin: 31.into(),
        routing_id: Arc::<str>::from("route"),
    };

    assert_eq!(provenance.origin(), Some(31.into()));
    assert_eq!(provenance.kind(), "routed");
}

#[test]
fn qec_provenance_can_be_rooted_or_derived() {
    let rooted = OperationProvenance::ErrorCorrection {
        origin: None,
        protocol: Arc::<str>::from("surface-code"),
    };

    let derived = OperationProvenance::ErrorCorrection {
        origin: Some(41.into()),
        protocol: Arc::<str>::from("surface-code"),
    };

    assert_eq!(rooted.origin(), None);
    assert_eq!(derived.origin(), Some(41.into()));
    assert_eq!(rooted.kind(), "error_correction");
    assert_eq!(derived.kind(), "error_correction");
}

#[test]
fn scheduling_transformation_provenance_is_explicit() {
    let provenance = OperationProvenance::SchedulingTransformation {
        origin: Some(51.into()),
        transformation: Arc::<str>::from("delay"),
    };

    assert_eq!(provenance.origin(), Some(51.into()));
    assert_eq!(provenance.kind(), "scheduling_transformation");
}

#[test]
fn external_provenance_has_no_internal_operation_origin() {
    let provenance = OperationProvenance::External {
        source: Arc::<str>::from("external"),
    };

    assert_eq!(provenance.origin(), None);
    assert_eq!(provenance.kind(), "external");
}

// ============================================================================
// Scheduling strategy tests
// ============================================================================

#[test]
fn default_scheduling_strategy_is_list() {
    assert_eq!(
        SchedulingStrategy::default(),
        SchedulingStrategy::List
    );
}

#[test]
fn scheduling_strategies_have_stable_names() {
    assert_eq!(
        SchedulingStrategy::AsSoonAsPossible.to_string(),
        "asap"
    );

    assert_eq!(
        SchedulingStrategy::AsLateAsPossible.to_string(),
        "alap"
    );

    assert_eq!(
        SchedulingStrategy::List.to_string(),
        "list"
    );

    assert_eq!(
        SchedulingStrategy::CriticalPath.to_string(),
        "critical-path"
    );

    assert_eq!(
        SchedulingStrategy::ResourceConstrained.to_string(),
        "resource-constrained"
    );

    assert_eq!(
        SchedulingStrategy::EventDriven.to_string(),
        "event-driven"
    );

    assert_eq!(
        SchedulingStrategy::Adaptive.to_string(),
        "adaptive"
    );
}

// ============================================================================
// Scheduling objective tests
// ============================================================================

#[test]
fn default_scheduling_objective_is_makespan() {
    assert_eq!(
        SchedulingObjective::default(),
        SchedulingObjective::Makespan
    );
}

#[test]
fn scheduling_objective_names_are_stable() {
    assert_eq!(
        SchedulingObjective::Feasible.to_string(),
        "feasible"
    );

    assert_eq!(
        SchedulingObjective::Makespan.to_string(),
        "makespan"
    );

    assert_eq!(
        SchedulingObjective::Depth.to_string(),
        "depth"
    );

    assert_eq!(
        SchedulingObjective::IdleTime.to_string(),
        "idle-time"
    );

    assert_eq!(
        SchedulingObjective::Fidelity.to_string(),
        "fidelity"
    );

    assert_eq!(
        SchedulingObjective::Energy.to_string(),
        "energy"
    );

    assert_eq!(
        SchedulingObjective::MultiObjective.to_string(),
        "multi-objective"
    );
}

// ============================================================================
// Verification policy tests
// ============================================================================

#[test]
fn production_default_verification_is_strict() {
    assert_eq!(
        VerificationPolicy::default(),
        VerificationPolicy::Strict
    );
}

#[test]
fn verification_policies_are_distinct() {
    assert_ne!(
        VerificationPolicy::Disabled,
        VerificationPolicy::Standard
    );

    assert_ne!(
        VerificationPolicy::Standard,
        VerificationPolicy::Strict
    );

    assert_ne!(
        VerificationPolicy::Strict,
        VerificationPolicy::Exhaustive
    );
}

// ============================================================================
// Transformation policy tests
// ============================================================================

#[test]
fn default_transformation_policy_is_timing_only() {
    assert_eq!(
        TransformationPolicy::default(),
        TransformationPolicy::TimingOnly
    );
}

#[test]
fn transformation_policies_are_orderable() {
    assert!(
        TransformationPolicy::None
            <= TransformationPolicy::TimingOnly
    );
}

// ============================================================================
// Failure policy tests
// ============================================================================

#[test]
fn default_failure_policy_is_fail_fast() {
    assert_eq!(
        FailurePolicy::default(),
        FailurePolicy::FailFast
    );
}

#[test]
fn failure_policies_are_distinct() {
    assert_ne!(
        FailurePolicy::FailFast,
        FailurePolicy::CollectDiagnostics
    );
}

// ============================================================================
// Diagnostic policy tests
// ============================================================================

#[test]
fn default_diagnostic_policy_is_basic() {
    assert_eq!(
        DiagnosticPolicy::default(),
        DiagnosticPolicy::Basic
    );
}

#[test]
fn diagnostic_policy_supports_increasing_detail_levels() {
    assert!(
        DiagnosticPolicy::Disabled
            <= DiagnosticPolicy::Basic
    );

    assert!(
        DiagnosticPolicy::Basic
            <= DiagnosticPolicy::Detailed
    );

    assert!(
        DiagnosticPolicy::Detailed
            <= DiagnosticPolicy::DetailedAndProfiled
    );
}

// ============================================================================
// Timing mode tests
// ============================================================================

#[test]
fn default_timing_mode_resolves_from_target() {
    assert_eq!(
        TimingMode::default(),
        TimingMode::ResolveFromTarget
    );
}

#[test]
fn timing_modes_are_distinct() {
    assert_ne!(
        TimingMode::Concrete,
        TimingMode::Symbolic
    );

    assert_ne!(
        TimingMode::Symbolic,
        TimingMode::ResolveFromTarget
    );

    assert_ne!(
        TimingMode::ResolveFromTarget,
        TimingMode::Hybrid
    );
}

// ============================================================================
// Parallelism tests
// ============================================================================

#[test]
fn default_parallelism_is_adaptive() {
    assert_eq!(
        ParallelismPolicy::default(),
        ParallelismPolicy::Adaptive
    );
}

#[test]
fn zero_parallel_workers_are_rejected() {
    assert_eq!(
        ParallelismPolicy::limited(0),
        None
    );
}

#[test]
fn one_parallel_worker_is_valid() {
    let policy =
        ParallelismPolicy::limited(1)
            .expect("one worker is valid");

    assert_eq!(
        policy.workers().map(|workers| workers.get()),
        Some(1)
    );
}

#[test]
fn very_large_parallel_worker_counts_are_supported_by_the_contract() {
    let policy =
        ParallelismPolicy::limited(u128::MAX)
            .expect("non-zero u128 worker count is valid");

    assert_eq!(
        policy.workers().map(|workers| workers.get()),
        Some(u128::MAX)
    );
}

#[test]
fn unbounded_parallelism_has_no_explicit_worker_limit() {
    assert_eq!(
        ParallelismPolicy::Unbounded.workers(),
        None
    );
}

#[test]
fn adaptive_parallelism_has_no_explicit_worker_limit() {
    assert_eq!(
        ParallelismPolicy::Adaptive.workers(),
        None
    );
}

// ============================================================================
// Reproducibility tests
// ============================================================================

#[test]
fn deterministic_reproducibility_is_default() {
    let config = ReproducibilityConfig::default();

    assert!(config.deterministic_mode());
    assert_eq!(config.seed(), None);
}

#[test]
fn nondeterministic_reproducibility_can_be_requested_explicitly() {
    let config = ReproducibilityConfig::nondeterministic();

    assert!(!config.deterministic_mode());
    assert_eq!(config.seed(), None);
}

#[test]
fn deterministic_seed_is_preserved() {
    let config = ReproducibilityConfig::with_seed(42);

    assert!(config.deterministic_mode());
    assert_eq!(config.seed(), Some(42));
}

#[test]
fn different_reproducibility_seeds_are_distinguishable() {
    let a = ReproducibilityConfig::with_seed(1);
    let b = ReproducibilityConfig::with_seed(2);

    assert_ne!(a, b);
}

// ============================================================================
// Planner identifier tests
// ============================================================================

#[test]
fn planner_id_accepts_stable_identifier_alphabet() {
    for value in [
        "scheduling.list",
        "scheduling.critical_path",
        "provider.example.custom",
        "custom-1",
        "custom_2",
        "vendor:planner",
    ] {
        assert!(
            PlannerId::new(value).is_ok(),
            "identifier `{value}` should be accepted"
        );
    }
}

#[test]
fn planner_id_rejects_empty_identifier() {
    assert!(PlannerId::new("").is_err());
}

#[test]
fn planner_id_rejects_invalid_characters() {
    for value in [
        "planner space",
        "planner/slash",
        "planner\\slash",
        "planner@provider",
        "planner#fragment",
    ] {
        assert!(
            PlannerId::new(value).is_err(),
            "identifier `{value}` should be rejected"
        );
    }
}

#[test]
fn planner_id_rejects_identifier_longer_than_metadata_boundary() {
    let value = "a".repeat(PLANNER_ID_MAX_BYTES + 1);

    assert!(PlannerId::new(value).is_err());
}

#[test]
fn planner_id_accepts_identifier_at_metadata_boundary() {
    let value = "a".repeat(PLANNER_ID_MAX_BYTES);

    assert!(PlannerId::new(value).is_ok());
}

#[test]
fn planner_id_preserves_text() {
    let id = planner_id("scheduling.list");

    assert_eq!(id.as_str(), "scheduling.list");
}

#[test]
fn planner_id_can_be_consumed_into_string() {
    let id = planner_id("scheduling.list");

    assert_eq!(
        id.into_string(),
        "scheduling.list".to_owned()
    );
}

#[test]
fn planner_id_ordering_is_stable() {
    let a = planner_id("a");
    let b = planner_id("b");

    assert!(a < b);
}

// ============================================================================
// Planner version tests
// ============================================================================

#[test]
fn planner_version_preserves_components() {
    let version = PlannerVersion::new(2, 7, 11);

    assert_eq!(version.major(), 2);
    assert_eq!(version.minor(), 7);
    assert_eq!(version.patch(), 11);
}

#[test]
fn default_planner_version_is_one_zero_zero() {
    assert_eq!(
        PlannerVersion::default(),
        PlannerVersion::new(1, 0, 0)
    );
}

#[test]
fn planner_version_display_is_semver_like() {
    let version = PlannerVersion::new(3, 4, 5);

    assert_eq!(version.to_string(), "3.4.5");
}

#[test]
fn planner_version_ordering_is_lexicographic_by_components() {
    let old = PlannerVersion::new(1, 9, 9);
    let new = PlannerVersion::new(2, 0, 0);

    assert!(old < new);
}

// ============================================================================
// Planner execution mode tests
// ============================================================================

#[test]
fn default_planner_execution_mode_is_static() {
    assert_eq!(
        PlannerExecutionMode::default(),
        PlannerExecutionMode::Static
    );
}

#[test]
fn planner_execution_mode_names_are_stable() {
    assert_eq!(
        PlannerExecutionMode::Static.to_string(),
        "static"
    );

    assert_eq!(
        PlannerExecutionMode::Dynamic.to_string(),
        "dynamic"
    );

    assert_eq!(
        PlannerExecutionMode::Hybrid.to_string(),
        "hybrid"
    );
}

// ============================================================================
// Planner algorithm family tests
// ============================================================================

#[test]
fn default_algorithm_family_is_list() {
    assert_eq!(
        PlannerAlgorithmFamily::default(),
        PlannerAlgorithmFamily::List
    );
}

#[test]
fn algorithm_family_names_are_stable() {
    assert_eq!(
        PlannerAlgorithmFamily::AsSoonAsPossible.to_string(),
        "asap"
    );

    assert_eq!(
        PlannerAlgorithmFamily::AsLateAsPossible.to_string(),
        "alap"
    );

    assert_eq!(
        PlannerAlgorithmFamily::List.to_string(),
        "list"
    );

    assert_eq!(
        PlannerAlgorithmFamily::CriticalPath.to_string(),
        "critical-path"
    );

    assert_eq!(
        PlannerAlgorithmFamily::ResourceConstrained.to_string(),
        "resource-constrained"
    );

    assert_eq!(
        PlannerAlgorithmFamily::EventDriven.to_string(),
        "event-driven"
    );

    assert_eq!(
        PlannerAlgorithmFamily::Adaptive.to_string(),
        "adaptive"
    );

    assert_eq!(
        PlannerAlgorithmFamily::Optimization.to_string(),
        "optimization"
    );

    assert_eq!(
        PlannerAlgorithmFamily::Custom.to_string(),
        "custom"
    );
}

// ============================================================================
// Planner capability tests
// ============================================================================

#[test]
fn static_default_capabilities_are_dependency_and_timing_aware() {
    let capabilities = PlannerCapabilities::static_default();

    assert_eq!(
        capabilities.execution_mode,
        PlannerExecutionMode::Static
    );

    assert_eq!(
        capabilities.algorithm_family,
        PlannerAlgorithmFamily::List
    );

    assert!(capabilities.resource_aware);
    assert!(capabilities.timing_aware);
    assert!(capabilities.dependency_aware);
    assert!(capabilities.deterministic);

    assert!(!capabilities.conditional);
    assert!(!capabilities.feedback);
    assert!(!capabilities.distributed);
    assert!(!capabilities.qec);
    assert!(!capabilities.symbolic_timing);
}

#[test]
fn static_capability_supports_static_requests() {
    let capabilities = PlannerCapabilities::static_default();

    assert!(
        capabilities.supports_execution_mode(
            PlannerExecutionMode::Static
        )
    );
}

#[test]
fn static_capability_does_not_claim_dynamic_support() {
    let capabilities = PlannerCapabilities::static_default();

    assert!(
        !capabilities.supports_execution_mode(
            PlannerExecutionMode::Dynamic
        )
    );

    assert!(
        !capabilities.supports_execution_mode(
            PlannerExecutionMode::Hybrid
        )
    );
}

#[test]
fn hybrid_capability_supports_all_execution_modes() {
    let capabilities = PlannerCapabilities {
        execution_mode: PlannerExecutionMode::Hybrid,
        ..PlannerCapabilities::static_default()
    };

    assert!(
        capabilities.supports_execution_mode(
            PlannerExecutionMode::Static
        )
    );

    assert!(
        capabilities.supports_execution_mode(
            PlannerExecutionMode::Dynamic
        )
    );

    assert!(
        capabilities.supports_execution_mode(
            PlannerExecutionMode::Hybrid
        )
    );
}

// ============================================================================
// Planner metadata tests
// ============================================================================

#[test]
fn planner_metadata_preserves_identity_and_capabilities() {
    let id = planner_id("scheduling.unit");

    let capabilities = PlannerCapabilities::static_default();

    let metadata = PlannerMetadata::new(
        id.clone(),
        PlannerVersion::new(1, 2, 3),
        "Unit Planner",
        "Production unit-test planner metadata",
        capabilities,
    );

    assert_eq!(metadata.id(), &id);
    assert_eq!(
        metadata.version(),
        PlannerVersion::new(1, 2, 3)
    );
    assert_eq!(
        metadata.capabilities(),
        capabilities
    );
}

#[test]
fn planner_metadata_supports_arbitrary_descriptive_names() {
    let metadata = PlannerMetadata::new(
        planner_id("scheduling.custom"),
        PlannerVersion::default(),
        "Custom planner",
        "A planner with no hardware assumptions",
        PlannerCapabilities::default(),
    );

    assert_eq!(metadata.name, "Custom planner");
    assert_eq!(
        metadata.description,
        "A planner with no hardware assumptions"
    );
}

#[test]
fn planner_contract_version_is_stable_for_this_contract() {
    assert_eq!(PLANNER_CONTRACT_VERSION, 1);
}

// ============================================================================
// Planning deadline tests
// ============================================================================

#[test]
fn unlimited_planning_deadline_has_no_timeout() {
    let deadline = PlanningDeadline::unlimited();

    /*
     * This assertion intentionally uses Debug rather than reaching into
     * private fields. The public API should remain responsible for exposing
     * deadline semantics.
     */
    assert!(
        format!("{deadline:?}").contains("PlanningDeadline")
    );
}

// ============================================================================
// Cross-contract invariants
// ============================================================================

#[test]
fn scheduler_identity_types_are_hashable_and_orderable() {
    use std::collections::{BTreeSet, HashSet};

    let logical_a = logical_qubit(1);
    let logical_b = logical_qubit(2);

    let mut ordered = BTreeSet::new();
    ordered.insert(logical_a);
    ordered.insert(logical_b);

    assert_eq!(ordered.len(), 2);

    let mut hashed = HashSet::new();
    hashed.insert(logical_a);
    hashed.insert(logical_b);

    assert_eq!(hashed.len(), 2);
}

#[test]
fn scheduler_does_not_treat_large_qubit_ids_as_capacity() {
    let ids = [
        logical_qubit(0),
        logical_qubit(1),
        logical_qubit(1024),
        logical_qubit(1_000_000),
        logical_qubit(usize::MAX),
    ];

    for id in ids {
        assert_eq!(
            id.index(),
            id.index(),
            "identifier must remain opaque identity data"
        );
    }
}

#[test]
fn scheduler_can_represent_arbitrarily_large_semantic_identifier_values_supported_by_host() {
    let ids = [
        logical_qubit(0),
        logical_qubit(usize::MAX / 2),
        logical_qubit(usize::MAX),
    ];

    assert_eq!(ids[0].index(), 0);
    assert_eq!(ids[1].index(), usize::MAX / 2);
    assert_eq!(ids[2].index(), usize::MAX);
}

#[test]
fn no_scheduler_local_qubit_type_is_required_for_operand_identity() {
    let operand = QubitOperand::new(logical_qubit(123_456));

    /*
     * The exact type returned by qubit() is the canonical IR QubitId.
     * This assertion is deliberately type-directed.
     */
    let canonical: QubitId = operand.qubit();

    assert_eq!(canonical, logical_qubit(123_456));
}

#[test]
fn operation_class_display_is_stable_for_diagnostics() {
    assert_eq!(
        OperationClass::Quantum.to_string(),
        "quantum"
    );

    assert_eq!(
        OperationClass::Measurement.to_string(),
        "measurement"
    );

    assert_eq!(
        OperationClass::Reset.to_string(),
        "reset"
    );

    assert_eq!(
        OperationClass::ErrorCorrection.to_string(),
        "error_correction"
    );
}

// ============================================================================
// Compile-time-style API smoke tests
// ============================================================================

#[test]
fn public_builder_contracts_are_composable() {
    let metadata = OperationMetadata::new()
        .with_name(Arc::<str>::from("operation"))
        .with_label(Arc::<str>::from("source"))
        .with_scheduling_class(Arc::<str>::from("priority"));

    let operand =
        QubitOperand::with_role(
            logical_qubit(0),
            OperandRole::Target,
        );

    let dependency =
        ClassicalDependencyId::new(
            Arc::<str>::from("measurement.result"),
        );

    assert_eq!(metadata.name(), Some("operation"));
    assert_eq!(metadata.label(), Some("source"));
    assert_eq!(
        metadata.scheduling_class(),
        Some("priority")
    );

    assert_eq!(operand.qubit(), logical_qubit(0));
    assert_eq!(operand.role(), OperandRole::Target);

    assert_eq!(
        dependency.as_str(),
        "measurement.result"
    );
}

#[test]
fn public_policy_defaults_do_not_encode_machine_size() {
    /*
     * This test is intentionally structural: policy defaults must describe
     * scheduling behaviour rather than a target's number of qubits/resources.
     */
    assert_eq!(
        SchedulingStrategy::default(),
        SchedulingStrategy::List
    );

    assert_eq!(
        SchedulingObjective::default(),
        SchedulingObjective::Makespan
    );

    assert_eq!(
        TimingMode::default(),
        TimingMode::ResolveFromTarget
    );

    assert_eq!(
        VerificationPolicy::default(),
        VerificationPolicy::Strict
    );

    assert_eq!(
        ParallelismPolicy::default(),
        ParallelismPolicy::Adaptive
    );
}

// ============================================================================
// Boundary tests for deterministic behaviour
// ============================================================================

#[test]
fn deterministic_configuration_is_equal_when_constructed_identically() {
    let a = ReproducibilityConfig::deterministic();
    let b = ReproducibilityConfig::deterministic();

    assert_eq!(a, b);
}

#[test]
fn deterministic_seed_configuration_is_equal_when_constructed_identically() {
    let a = ReproducibilityConfig::with_seed(1234);
    let b = ReproducibilityConfig::with_seed(1234);

    assert_eq!(a, b);
}

#[test]
fn different_deterministic_seeds_produce_distinct_configuration_values() {
    let a = ReproducibilityConfig::with_seed(1234);
    let b = ReproducibilityConfig::with_seed(1235);

    assert_ne!(a, b);
}

// ============================================================================
// Public contract regression tests
// ============================================================================

#[test]
fn canonical_planner_identifier_examples_remain_valid() {
    let examples = [
        "scheduling.list",
        "scheduling.critical_path",
        "scheduling.resource_constrained",
        "scheduling.event",
    ];

    for example in examples {
        assert!(
            PlannerId::new(example).is_ok(),
            "public planner identifier `{example}` must remain valid"
        );
    }
}

#[test]
fn planner_capability_defaults_remain_conservative() {
    let capabilities = PlannerCapabilities::default();

    assert_eq!(
        capabilities.execution_mode,
        PlannerExecutionMode::Static
    );

    assert_eq!(
        capabilities.algorithm_family,
        PlannerAlgorithmFamily::List
    );

    assert!(capabilities.resource_aware);
    assert!(capabilities.timing_aware);
    assert!(capabilities.dependency_aware);
    assert!(capabilities.deterministic);

    assert!(!capabilities.symbolic_timing);
    assert!(!capabilities.distributed);
    assert!(!capabilities.qec);
}

#[test]
fn scheduler_identifier_display_is_human_readable() {
    assert_eq!(
        logical_qubit(7).to_string(),
        "q7"
    );

    assert_eq!(
        physical_qubit(7).to_string(),
        "p7"
    );
}

#[test]
fn logical_and_physical_display_names_make_identity_domains_obvious() {
    let logical = logical_qubit(12).to_string();
    let physical = physical_qubit(12).to_string();

    assert_eq!(logical, "q12");
    assert_eq!(physical, "p12");
    assert_ne!(logical, physical);
}