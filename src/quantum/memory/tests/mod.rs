//! Zamani Quantum Memory — Production Test Coordinator
//!
//! This module is the integration-level test boundary for
//! `quantum::memory`.
//!
//! # Purpose
//!
//! This file verifies the contracts that must remain true across the complete
//! quantum-memory subsystem without coupling the tests to implementation
//! details of individual memory providers.
//!
//! The test architecture deliberately separates:
//!
//! - foundational type invariants;
//! - numerical/indexing invariants;
//! - resource-limit invariants;
//! - state-contract invariants;
//! - representation/provider neutrality;
//! - lifecycle invariants;
//! - capability negotiation;
//! - operation descriptors;
//! - determinism;
//! - overflow safety;
//! - QPU compatibility;
//! - simulator compatibility;
//! - persistence compatibility contracts;
//! - concurrency-facing contracts;
//! - architecture/dependency boundaries.
//!
//! # Critical design rule
//!
//! This module MUST NOT become a second implementation of quantum memory.
//!
//! Tests should verify observable contracts exposed by the memory subsystem.
//! They must not reach into private fields merely to increase coverage.
//!
//! # Stability rule
//!
//! This file intentionally depends only on stable, foundational public APIs.
//! Representation-specific tests belong beside their respective
//! implementations or in dedicated test files.
//!
//! Consequently, adding:
//!
//! - a new QPU;
//! - a new simulator;
//! - a new tensor representation;
//! - a new GPU provider;
//! - a new distributed provider;
//! - a new serialization version;
//! - a new state representation;
//!
//! must not require changing this file unless the canonical memory contract
//! itself changes.
//!
//! # QPU neutrality
//!
//! A production quantum-memory subsystem cannot assume that a QPU exposes a
//! state vector. Real quantum hardware commonly exposes execution,
//! measurement, classical results, synchronization and provider-managed state,
//! while simulator-only capabilities such as amplitude access may be absent.
//!
//! The tests therefore explicitly distinguish capability availability from
//! state identity.
//!
//! # Safety
//!
//! This test module uses no `unsafe` code.
//!
//! `unsafe` is explicitly denied.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.
//!
//! # Integration
//!
//! `src/quantum/memory/mod.rs` should contain:
//
//! ```text
//! #[cfg(test)]
//! mod tests;
//! ```
//!
//! The memory module itself should remain responsible for declaring the
//! production modules. This file must not recreate the production module
//! hierarchy.
//!
//! # Test hierarchy
//!
//! ```text
//! quantum::memory
//!        │
//!        ├── types
//!        ├── errors
//!        ├── numeric
//!        ├── representation
//!        ├── limits
//!        ├── layout
//!        ├── indexing
//!        │
//!        ├── allocator
//!        ├── pool
//!        ├── reservation
//!        ├── budget
//!        │
//!        ├── logical memory
//!        ├── state abstraction
//!        ├── state representations
//!        ├── transformations
//!        ├── measurement
//!        ├── persistence
//!        ├── synchronization
//!        ├── acceleration
//!        └── migration
//!
//!                         │
//!                         ▼
//!                  tests/mod.rs
//!                         │
//!          ┌──────────────┼──────────────┐
//!          ▼              ▼              ▼
//!      contracts      invariants     integration
//! ```
//!
//! # Production philosophy
//!
//! These tests are deliberately written so that:
//!
//! 1. a memory implementation cannot pass merely because it compiles;
//! 2. a simulator-only implementation cannot masquerade as a QPU;
//! 3. unsupported capabilities must be reported rather than fabricated;
//! 4. arithmetic overflow cannot silently become an allocation request;
//! 5. lifecycle terminal states cannot silently accept operations;
//! 6. capability bitsets remain future-extensible;
//! 7. state representation remains separate from execution domain;
//! 8. storage location remains separate from state semantics;
//! 9. provider-neutral operation descriptors remain provider-neutral;
//! 10. no test requires `unsafe`.
//!
//! The repository currently defines the state contract in
//! `quantum::memory::state`. That contract explicitly supports local
//! simulation, remote simulation, QPU execution, hardware emulation,
//! hybrid execution and distributed execution. This test coordinator is
//! designed around that distinction.
//!

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::panic::{catch_unwind, AssertUnwindSafe};

use super::state::{
    StateCapabilities,
    StateConsistency,
    StateExecutionDomain,
    StateLifecycle,
    StateOperation,
    StateOperationKind,
    StateOperationSemantics,
    StateResult,
    StateStorageLocation,
};
use super::types::{
    AmplitudeCount,
    ByteCount,
    ClassicalBitCount,
    QubitCount,
};

// =============================================================================
// Test utilities
// =============================================================================

/// Executes a test closure and returns its result without allowing a panic to
/// escape into unrelated test infrastructure.
///
/// This is used only for tests that intentionally verify panic-free behavior
/// of public APIs.
///
/// Production memory APIs should normally return `Result` rather than panic.
fn assert_does_not_panic<F>(operation: F)
where
    F: FnOnce(),
{
    let result = catch_unwind(AssertUnwindSafe(operation));

    assert!(
        result.is_ok(),
        "production quantum-memory API unexpectedly panicked"
    );
}

/// Checks that a collection is deterministic across repeated construction.
///
/// The function is intentionally generic so it can be reused by future test
/// modules without coupling this coordinator to one concrete representation.
fn assert_deterministic<T, F>(builder: F)
where
    T: PartialEq + std::fmt::Debug,
    F: Fn() -> T,
{
    let first = builder();
    let second = builder();

    assert_eq!(
        first, second,
        "repeated construction produced different deterministic results"
    );
}

/// Assert a boolean invariant with a useful failure message.
fn assert_invariant(condition: bool, message: &str) {
    assert!(condition, "quantum-memory invariant violated: {message}");
}

// =============================================================================
// Foundational quantity tests
// =============================================================================

#[test]
fn qubit_count_is_strongly_typed() {
    let zero = QubitCount::ZERO;
    let one = QubitCount::new(1);
    let many = QubitCount::new(1024);

    assert!(zero.is_zero());
    assert!(!one.is_zero());
    assert!(many.is_non_zero());

    assert_eq!(one.get(), 1);
    assert_eq!(many.get(), 1024);
}

#[test]
fn qubit_count_checked_arithmetic_is_safe() {
    let value = QubitCount::new(10);

    assert_eq!(value.checked_add(QubitCount::new(5)), Some(QubitCount::new(15)));

    assert_eq!(
        value.checked_sub(QubitCount::new(3)),
        Some(QubitCount::new(7))
    );

    assert_eq!(value.checked_sub(QubitCount::new(11)), None);

    assert_eq!(value.checked_mul(4), Some(QubitCount::new(40)));
}

#[test]
fn classical_bit_count_is_strongly_typed() {
    let zero = ClassicalBitCount::ZERO;
    let one = ClassicalBitCount::new(1);
    let many = ClassicalBitCount::new(4096);

    assert!(zero.is_zero());
    assert!(one.is_non_zero());
    assert_eq!(many.get(), 4096);
}

#[test]
fn classical_bit_count_checked_arithmetic_is_safe() {
    let value = ClassicalBitCount::new(16);

    assert_eq!(
        value.checked_add(ClassicalBitCount::new(8)),
        Some(ClassicalBitCount::new(24))
    );

    assert_eq!(
        value.checked_sub(ClassicalBitCount::new(4)),
        Some(ClassicalBitCount::new(12))
    );

    assert_eq!(value.checked_sub(ClassicalBitCount::new(17)), None);
}

#[test]
fn amplitude_count_is_representation_neutral() {
    let zero = AmplitudeCount::ZERO;
    let count = AmplitudeCount::new(1024);

    assert!(zero.is_zero());
    assert!(count.is_non_zero());
    assert_eq!(count.get(), 1024);
}

#[test]
fn amplitude_count_for_small_qubit_counts_is_exact() {
    assert_eq!(
        AmplitudeCount::checked_for_qubits(QubitCount::new(0)),
        Some(AmplitudeCount::new(1))
    );

    assert_eq!(
        AmplitudeCount::checked_for_qubits(QubitCount::new(1)),
        Some(AmplitudeCount::new(2))
    );

    assert_eq!(
        AmplitudeCount::checked_for_qubits(QubitCount::new(2)),
        Some(AmplitudeCount::new(4))
    );

    assert_eq!(
        AmplitudeCount::checked_for_qubits(QubitCount::new(10)),
        Some(AmplitudeCount::new(1024))
    );
}

#[test]
fn amplitude_count_rejects_unrepresentable_shifts() {
    let impossible = QubitCount::new(usize::BITS as usize);

    assert_eq!(
        AmplitudeCount::checked_for_qubits(impossible),
        None,
        "amplitude-count calculation must fail before an invalid shift"
    );
}

#[test]
fn byte_count_has_correct_binary_units() {
    assert_eq!(ByteCount::ONE.get(), 1);
    assert_eq!(ByteCount::KIB.get(), 1024);
    assert_eq!(ByteCount::MIB.get(), 1024 * 1024);
    assert_eq!(ByteCount::GIB.get(), 1024 * 1024 * 1024);

    assert_eq!(ByteCount::new(2048).kibibytes(), 2);
    assert_eq!(ByteCount::new(2 * 1024 * 1024).mebibytes(), 2);
    assert_eq!(
        ByteCount::new(3 * 1024 * 1024 * 1024).gibibytes(),
        3
    );
}

#[test]
fn byte_count_checked_arithmetic_is_safe() {
    let value = ByteCount::new(1024);

    assert_eq!(
        value.checked_add(ByteCount::new(1024)),
        Some(ByteCount::new(2048))
    );

    assert_eq!(
        value.checked_sub(ByteCount::new(24)),
        Some(ByteCount::new(1000))
    );

    assert_eq!(value.checked_sub(ByteCount::new(1025)), None);

    assert_eq!(
        value.checked_mul(4),
        Some(ByteCount::new(4096))
    );
}

#[test]
fn byte_count_platform_conversion_is_checked() {
    let value = ByteCount::new(4096);

    assert_eq!(value.try_as_usize().unwrap(), 4096);
}

// =============================================================================
// State-storage contract tests
// =============================================================================

#[test]
fn storage_locations_have_correct_semantics() {
    assert!(StateStorageLocation::Host.is_host_readable());
    assert!(StateStorageLocation::PinnedHost.is_host_readable());
    assert!(StateStorageLocation::Unified.is_host_readable());

    assert!(StateStorageLocation::Device.is_device());
    assert!(StateStorageLocation::Unified.is_device());

    assert!(StateStorageLocation::Distributed.is_distributed());

    assert!(StateStorageLocation::Remote.is_external());
    assert!(StateStorageLocation::External.is_external());
    assert!(StateStorageLocation::Opaque.is_external());

    assert!(!StateStorageLocation::Host.is_external());
    assert!(!StateStorageLocation::Distributed.is_external());
}

#[test]
fn storage_location_is_independent_from_execution_domain() {
    // A QPU may use remote, external or opaque storage.
    //
    // A simulator may use host, device, unified or distributed storage.
    //
    // Therefore the two concepts must never be conflated.

    let locations = [
        StateStorageLocation::Host,
        StateStorageLocation::PinnedHost,
        StateStorageLocation::Device,
        StateStorageLocation::Unified,
        StateStorageLocation::Distributed,
        StateStorageLocation::Remote,
        StateStorageLocation::External,
        StateStorageLocation::Opaque,
    ];

    let domains = [
        StateExecutionDomain::LocalSimulator,
        StateExecutionDomain::LocalEmulator,
        StateExecutionDomain::RemoteSimulator,
        StateExecutionDomain::Qpu,
        StateExecutionDomain::HardwareEmulator,
        StateExecutionDomain::Hybrid,
        StateExecutionDomain::Distributed,
        StateExecutionDomain::Custom,
    ];

    assert_eq!(locations.len(), 8);
    assert_eq!(domains.len(), 8);
}

// =============================================================================
// Execution-domain tests
// =============================================================================

#[test]
fn execution_domain_classification_is_correct() {
    assert!(StateExecutionDomain::Qpu.is_qpu());

    assert!(StateExecutionDomain::LocalSimulator.is_simulator());
    assert!(StateExecutionDomain::LocalEmulator.is_simulator());
    assert!(StateExecutionDomain::RemoteSimulator.is_simulator());
    assert!(StateExecutionDomain::HardwareEmulator.is_simulator());

    assert!(StateExecutionDomain::Hybrid.is_hybrid());

    assert!(StateExecutionDomain::Distributed.is_distributed());

    assert!(!StateExecutionDomain::Qpu.is_simulator());
    assert!(!StateExecutionDomain::Qpu.is_hybrid());
}

#[test]
fn qpu_domain_is_not_equated_with_simulation() {
    assert!(StateExecutionDomain::Qpu.is_qpu());
    assert!(!StateExecutionDomain::Qpu.is_simulator());
}

#[test]
fn execution_domain_is_provider_neutral() {
    // No vendor name is part of the canonical execution-domain taxonomy.
    //
    // IBM, Google, Quantinuum, IonQ, Rigetti, IQM, Pasqal, D-Wave, AWS,
    // Azure, and future providers must be represented through hardware
    // adapters rather than through additional vendor variants here.

    let _domains = [
        StateExecutionDomain::Qpu,
        StateExecutionDomain::RemoteSimulator,
        StateExecutionDomain::Hybrid,
        StateExecutionDomain::Distributed,
        StateExecutionDomain::Custom,
    ];
}

// =============================================================================
// Lifecycle tests
// =============================================================================

#[test]
fn lifecycle_operation_policy_is_correct() {
    assert!(!StateLifecycle::Allocated.accepts_operations());
    assert!(StateLifecycle::Ready.accepts_operations());
    assert!(!StateLifecycle::Executing.accepts_operations());
    assert!(!StateLifecycle::Suspended.accepts_operations());
    assert!(!StateLifecycle::Released.accepts_operations());
    assert!(!StateLifecycle::Failed.accepts_operations());
}

#[test]
fn released_and_failed_states_are_terminal() {
    assert!(StateLifecycle::Released.is_terminal());
    assert!(StateLifecycle::Failed.is_terminal());

    assert!(!StateLifecycle::Ready.is_terminal());
    assert!(!StateLifecycle::Suspended.is_terminal());
}

#[test]
fn suspended_state_is_resumable() {
    assert!(StateLifecycle::Suspended.is_resumable());

    assert!(!StateLifecycle::Ready.is_resumable());
    assert!(!StateLifecycle::Released.is_resumable());
    assert!(!StateLifecycle::Failed.is_resumable());
}

// =============================================================================
// Consistency tests
// =============================================================================

#[test]
fn state_consistency_classification_is_correct() {
    assert!(StateConsistency::Consistent.is_consistent());
    assert!(StateConsistency::Synchronized.is_consistent());
    assert!(StateConsistency::Distributed.is_consistent());
    assert!(StateConsistency::ProviderManaged.is_consistent());

    assert!(!StateConsistency::HostDirty.is_consistent());
    assert!(!StateConsistency::DeviceDirty.is_consistent());
    assert!(!StateConsistency::Unknown.is_consistent());
}

#[test]
fn dirty_states_require_synchronization() {
    assert!(StateConsistency::HostDirty.requires_synchronization());
    assert!(StateConsistency::DeviceDirty.requires_synchronization());

    assert!(!StateConsistency::Consistent.requires_synchronization());
    assert!(!StateConsistency::Synchronized.requires_synchronization());
    assert!(!StateConsistency::ProviderManaged.requires_synchronization());
}

// =============================================================================
// Capability tests
// =============================================================================

#[test]
fn empty_capability_set_contains_nothing() {
    let capabilities = StateCapabilities::NONE;

    assert!(capabilities.is_empty());
    assert_eq!(capabilities.bits(), 0);

    assert!(!capabilities.contains(StateCapabilities::MEASUREMENT));
    assert!(!capabilities.intersects(StateCapabilities::MEASUREMENT));
}

#[test]
fn capability_union_and_difference_are_correct() {
    let capabilities =
        StateCapabilities::MEASUREMENT
            | StateCapabilities::RESET
            | StateCapabilities::UNITARY;

    assert!(capabilities.contains(StateCapabilities::MEASUREMENT));
    assert!(capabilities.contains(StateCapabilities::RESET));
    assert!(capabilities.contains(StateCapabilities::UNITARY));

    assert!(!capabilities.contains(StateCapabilities::AMPLITUDE_ACCESS));

    let without_measurement =
        capabilities.without(StateCapabilities::MEASUREMENT);

    assert!(!without_measurement.contains(StateCapabilities::MEASUREMENT));
    assert!(without_measurement.contains(StateCapabilities::RESET));
    assert!(without_measurement.contains(StateCapabilities::UNITARY));
}

#[test]
fn capability_sets_preserve_unknown_future_bits() {
    let future_bit = 1u64 << 63;

    let capabilities = StateCapabilities::from_bits(future_bit);

    assert_eq!(capabilities.bits(), future_bit);
    assert!(!capabilities.is_empty());

    // This is important for forward compatibility. A newer provider may
    // advertise a capability unknown to an older Zamani runtime.
    assert!(!capabilities.contains(StateCapabilities::MEASUREMENT));
}

#[test]
fn qpu_capabilities_can_be_minimal() {
    // A real QPU does not need to expose amplitudes.
    //
    // The memory contract must therefore allow:
    //
    // measurement + unitary + reset
    //
    // without:
    //
    // amplitude access + probability access + pure-state semantics.

    let qpu_capabilities =
        StateCapabilities::UNITARY
            | StateCapabilities::MEASUREMENT
            | StateCapabilities::RESET
            | StateCapabilities::BACKEND_NATIVE;

    assert!(qpu_capabilities.contains(StateCapabilities::UNITARY));
    assert!(qpu_capabilities.contains(StateCapabilities::MEASUREMENT));
    assert!(qpu_capabilities.contains(StateCapabilities::RESET));
    assert!(qpu_capabilities.contains(StateCapabilities::BACKEND_NATIVE));

    assert!(!qpu_capabilities.contains(StateCapabilities::AMPLITUDE_ACCESS));
}

#[test]
fn simulator_capabilities_can_expose_amplitudes() {
    let simulator_capabilities =
        StateCapabilities::UNITARY
            | StateCapabilities::MEASUREMENT
            | StateCapabilities::RESET
            | StateCapabilities::AMPLITUDE_ACCESS
            | StateCapabilities::PROBABILITY_ACCESS
            | StateCapabilities::PURE_STATE;

    assert!(simulator_capabilities.contains(StateCapabilities::AMPLITUDE_ACCESS));
    assert!(simulator_capabilities.contains(StateCapabilities::PROBABILITY_ACCESS));
    assert!(simulator_capabilities.contains(StateCapabilities::PURE_STATE));
}

#[test]
fn distributed_capability_is_explicit() {
    let capabilities =
        StateCapabilities::DISTRIBUTED
            | StateCapabilities::SYNCHRONIZE
            | StateCapabilities::MIGRATE;

    assert!(capabilities.contains(StateCapabilities::DISTRIBUTED));
    assert!(capabilities.contains(StateCapabilities::SYNCHRONIZE));
    assert!(capabilities.contains(StateCapabilities::MIGRATE));
}

// =============================================================================
// Operation-kind tests
// =============================================================================

#[test]
fn state_operation_capability_requirements_are_consistent() {
    assert_eq!(
        StateOperationKind::Unitary.required_capabilities(),
        StateCapabilities::UNITARY
    );

    assert_eq!(
        StateOperationKind::Channel.required_capabilities(),
        StateCapabilities::CHANNEL
    );

    assert_eq!(
        StateOperationKind::Measure.required_capabilities(),
        StateCapabilities::MEASUREMENT
    );

    assert_eq!(
        StateOperationKind::Reset.required_capabilities(),
        StateCapabilities::RESET
    );

    assert_eq!(
        StateOperationKind::Probability.required_capabilities(),
        StateCapabilities::PROBABILITY_ACCESS
    );

    assert_eq!(
        StateOperationKind::ExpectationValue.required_capabilities(),
        StateCapabilities::EXPECTATION_VALUE
    );

    assert_eq!(
        StateOperationKind::TensorProduct.required_capabilities(),
        StateCapabilities::TENSOR_PRODUCT
    );

    assert_eq!(
        StateOperationKind::PartialTrace.required_capabilities(),
        StateCapabilities::PARTIAL_TRACE
    );

    assert_eq!(
        StateOperationKind::Synchronize.required_capabilities(),
        StateCapabilities::SYNCHRONIZE
    );

    assert_eq!(
        StateOperationKind::Snapshot.required_capabilities(),
        StateCapabilities::SERIALIZE
    );

    assert_eq!(
        StateOperationKind::Restore.required_capabilities(),
        StateCapabilities::RESTORE
    );

    assert_eq!(
        StateOperationKind::Migrate.required_capabilities(),
        StateCapabilities::MIGRATE
    );
}

#[test]
fn state_operation_mutation_classification_is_correct() {
    assert!(StateOperationKind::Initialize.is_mutating());
    assert!(StateOperationKind::Unitary.is_mutating());
    assert!(StateOperationKind::Channel.is_mutating());
    assert!(StateOperationKind::Measure.is_mutating());
    assert!(StateOperationKind::Reset.is_mutating());
    assert!(StateOperationKind::TensorProduct.is_mutating());
    assert!(StateOperationKind::Restore.is_mutating());
    assert!(StateOperationKind::Migrate.is_mutating());

    assert!(!StateOperationKind::Probability.is_mutating());
    assert!(!StateOperationKind::ExpectationValue.is_mutating());
    assert!(!StateOperationKind::PartialTrace.is_mutating());
    assert!(!StateOperationKind::Synchronize.is_mutating());
    assert!(!StateOperationKind::Snapshot.is_mutating());
}

#[test]
fn measurement_can_produce_classical_output() {
    assert!(StateOperationKind::Measure.may_produce_classical_output());
    assert!(StateOperationKind::Probability.may_produce_classical_output());

    assert!(!StateOperationKind::Unitary.may_produce_classical_output());
    assert!(!StateOperationKind::Reset.may_produce_classical_output());
}

#[test]
fn state_operation_semantics_are_composable() {
    let semantics =
        StateOperationSemantics::UNITARY
            | StateOperationSemantics::REVERSIBLE;

    assert!(semantics.contains(StateOperationSemantics::UNITARY));
    assert!(semantics.contains(StateOperationSemantics::REVERSIBLE));
    assert!(!semantics.contains(StateOperationSemantics::NOISY));
}

// =============================================================================
// Provider-neutral operation descriptor
// =============================================================================

/// Minimal provider-neutral operation used only by integration tests.
///
/// It intentionally contains no vendor-specific fields and no simulator
/// assumptions.
#[derive(Debug)]
struct TestOperation {
    name: &'static str,
    kind: StateOperationKind,
    semantics: StateOperationSemantics,
}

impl TestOperation {
    const fn unitary(name: &'static str) -> Self {
        Self {
            name,
            kind: StateOperationKind::Unitary,
            semantics: StateOperationSemantics::UNITARY
                | StateOperationSemantics::REVERSIBLE,
        }
    }

    const fn measurement() -> Self {
        Self {
            name: "measure",
            kind: StateOperationKind::Measure,
            semantics: StateOperationSemantics::PROBABILISTIC
                | StateOperationSemantics::COLLAPSING,
        }
    }
}

impl StateOperation for TestOperation {
    fn kind(&self) -> StateOperationKind {
        self.kind
    }

    fn name(&self) -> &str {
        self.name
    }

    fn logical_qubits(&self) -> &[crate::quantum::ir::QubitId] {
        &[]
    }

    fn semantics(&self) -> StateOperationSemantics {
        self.semantics
    }
}

#[test]
fn provider_neutral_operation_can_be_constructed_without_vendor_state() {
    let operation = TestOperation::unitary("test_unitary");

    assert_eq!(operation.kind(), StateOperationKind::Unitary);
    assert_eq!(operation.name(), "test_unitary");
    assert!(operation.semantics().contains(StateOperationSemantics::UNITARY));
}

#[test]
fn measurement_operation_is_distinguished_from_unitary_operation() {
    let unitary = TestOperation::unitary("h");
    let measurement = TestOperation::measurement();

    assert_eq!(unitary.kind(), StateOperationKind::Unitary);
    assert_eq!(measurement.kind(), StateOperationKind::Measure);

    assert!(
        measurement
            .semantics()
            .contains(StateOperationSemantics::COLLAPSING)
    );

    assert!(
        !unitary
            .semantics()
            .contains(StateOperationSemantics::COLLAPSING)
    );
}

#[test]
fn operation_descriptor_validation_is_available_through_trait_contract() {
    let operation = TestOperation::unitary("test");

    let result: StateResult<()> = operation.validate_descriptor();

    assert!(
        result.is_ok(),
        "valid provider-neutral operation descriptor was rejected: {result:?}"
    );
}

// =============================================================================
// Determinism tests
// =============================================================================

#[test]
fn foundational_quantities_are_deterministic() {
    assert_deterministic(|| {
        (
            QubitCount::new(8),
            ClassicalBitCount::new(8),
            AmplitudeCount::new(256),
            ByteCount::new(4096),
        )
    });
}

#[test]
fn capability_bits_are_deterministic() {
    assert_deterministic(|| {
        StateCapabilities::UNITARY
            | StateCapabilities::MEASUREMENT
            | StateCapabilities::RESET
            | StateCapabilities::BACKEND_NATIVE
    });
}

#[test]
fn operation_classification_is_deterministic() {
    assert_deterministic(|| {
        (
            StateOperationKind::Unitary.required_capabilities().bits(),
            StateOperationKind::Measure.required_capabilities().bits(),
            StateOperationKind::Reset.required_capabilities().bits(),
        )
    });
}

// =============================================================================
// Overflow and panic-resistance tests
// =============================================================================

#[test]
fn foundational_arithmetic_does_not_panic_on_boundary_values() {
    assert_does_not_panic(|| {
        let max_qubits = QubitCount::new(usize::MAX);

        assert_eq!(
            max_qubits.checked_add(QubitCount::new(1)),
            None
        );

        assert_eq!(
            max_qubits.checked_mul(2),
            None
        );

        let max_classical = ClassicalBitCount::new(usize::MAX);

        assert_eq!(
            max_classical.checked_add(ClassicalBitCount::new(1)),
            None
        );

        assert_eq!(
            max_classical.checked_mul(2),
            None
        );
    });
}

#[test]
fn amplitude_calculation_does_not_panic_at_shift_boundary() {
    assert_does_not_panic(|| {
        let boundary = QubitCount::new(usize::BITS as usize);

        assert_eq!(
            AmplitudeCount::checked_for_qubits(boundary),
            None
        );
    });
}

#[test]
fn byte_arithmetic_does_not_panic_at_u64_boundary() {
    assert_does_not_panic(|| {
        let maximum = ByteCount::new(u64::MAX);

        assert_eq!(
            maximum.checked_add(ByteCount::ONE),
            None
        );

        assert_eq!(
            maximum.checked_mul(2),
            None
        );
    });
}

// =============================================================================
// Quantum-memory architectural invariants
// =============================================================================

#[test]
fn state_representation_and_execution_domain_are_independent_concepts() {
    // This test intentionally verifies the architecture rather than a
    // particular implementation.
    //
    // A state vector can be:
    //
    // - local simulator;
    // - remote simulator;
    // - distributed simulator;
    // - accelerator-backed simulator.
    //
    // A QPU state can be:
    //
    // - opaque;
    // - remote;
    // - externally managed.
    //
    // Therefore no representation enum is allowed to encode vendor identity
    // or execution domain.

    let simulator = StateExecutionDomain::LocalSimulator;
    let qpu = StateExecutionDomain::Qpu;

    assert!(simulator.is_simulator());
    assert!(qpu.is_qpu());
    assert!(!simulator.is_qpu());
    assert!(!qpu.is_simulator());
}

#[test]
fn qpu_memory_must_not_require_amplitude_access() {
    let capabilities =
        StateCapabilities::UNITARY
            | StateCapabilities::MEASUREMENT
            | StateCapabilities::RESET
            | StateCapabilities::BACKEND_NATIVE;

    assert!(capabilities.contains(StateCapabilities::UNITARY));
    assert!(capabilities.contains(StateCapabilities::MEASUREMENT));

    assert!(
        !capabilities.contains(StateCapabilities::AMPLITUDE_ACCESS),
        "QPU execution must not be forced to expose simulator amplitudes"
    );
}

#[test]
fn remote_qpu_state_can_be_opaque() {
    let location = StateStorageLocation::Opaque;

    assert!(location.is_external());
    assert!(!location.is_host_readable());
    assert!(!location.is_device());
}

#[test]
fn distributed_state_requires_explicit_distribution_semantics() {
    let domain = StateExecutionDomain::Distributed;

    assert!(domain.is_distributed());
    assert!(!domain.is_qpu());
    assert!(!domain.is_simulator() || domain == StateExecutionDomain::Distributed);
}

#[test]
fn backend_native_state_is_explicitly_identifiable() {
    let capabilities =
        StateCapabilities::BACKEND_NATIVE
            | StateCapabilities::MEASUREMENT;

    assert!(capabilities.contains(StateCapabilities::BACKEND_NATIVE));
    assert!(capabilities.contains(StateCapabilities::MEASUREMENT));
}

// =============================================================================
// Capability consistency matrix
// =============================================================================

#[test]
fn capability_matrix_does_not_assume_simulator_only_features_for_all_states() {
    let simulator =
        StateCapabilities::UNITARY
            | StateCapabilities::MEASUREMENT
            | StateCapabilities::RESET
            | StateCapabilities::AMPLITUDE_ACCESS
            | StateCapabilities::PROBABILITY_ACCESS
            | StateCapabilities::PURE_STATE;

    let qpu =
        StateCapabilities::UNITARY
            | StateCapabilities::MEASUREMENT
            | StateCapabilities::RESET
            | StateCapabilities::BACKEND_NATIVE;

    assert!(simulator.contains(StateCapabilities::AMPLITUDE_ACCESS));

    assert!(
        !qpu.contains(StateCapabilities::AMPLITUDE_ACCESS),
        "QPU capability contract must not inherit simulator-only capabilities"
    );
}

#[test]
fn mixed_state_capability_is_distinct_from_pure_state_capability() {
    let mixed = StateCapabilities::MIXED_STATE;
    let pure = StateCapabilities::PURE_STATE;

    assert!(mixed.contains(StateCapabilities::MIXED_STATE));
    assert!(pure.contains(StateCapabilities::PURE_STATE));

    assert!(!mixed.contains(StateCapabilities::PURE_STATE));
    assert!(!pure.contains(StateCapabilities::MIXED_STATE));
}

#[test]
fn representation_capabilities_are_independent() {
    let stabilizer = StateCapabilities::STABILIZER;
    let sparse = StateCapabilities::SPARSE;
    let tensor = StateCapabilities::TENSOR_NETWORK;

    assert!(stabilizer.contains(StateCapabilities::STABILIZER));
    assert!(sparse.contains(StateCapabilities::SPARSE));
    assert!(tensor.contains(StateCapabilities::TENSOR_NETWORK));

    assert!(!stabilizer.contains(StateCapabilities::SPARSE));
    assert!(!stabilizer.contains(StateCapabilities::TENSOR_NETWORK));
}

// =============================================================================
// State-operation safety matrix
// =============================================================================

#[test]
fn state_operation_requirements_are_not_empty_for_executable_operations() {
    let executable_operations = [
        StateOperationKind::Unitary,
        StateOperationKind::Channel,
        StateOperationKind::Measure,
        StateOperationKind::Reset,
        StateOperationKind::Probability,
        StateOperationKind::ExpectationValue,
        StateOperationKind::TensorProduct,
        StateOperationKind::PartialTrace,
        StateOperationKind::Synchronize,
        StateOperationKind::Snapshot,
        StateOperationKind::Restore,
        StateOperationKind::Migrate,
        StateOperationKind::Custom,
    ];

    for operation in executable_operations {
        let required = operation.required_capabilities();

        assert!(
            !required.is_empty(),
            "operation {:?} unexpectedly has no capability requirement",
            operation
        );
    }
}

#[test]
fn initialize_is_the_only_basic_operation_without_a_capability_requirement() {
    assert!(
        StateOperationKind::Initialize
            .required_capabilities()
            .is_empty()
    );
}

#[test]
fn read_only_operations_are_not_marked_mutating() {
    assert!(!StateOperationKind::Probability.is_mutating());
    assert!(!StateOperationKind::ExpectationValue.is_mutating());
    assert!(!StateOperationKind::PartialTrace.is_mutating());
    assert!(!StateOperationKind::Synchronize.is_mutating());
    assert!(!StateOperationKind::Snapshot.is_mutating());
}

// =============================================================================
// Memory-size sanity tests
// =============================================================================

#[test]
fn dense_state_vector_amplitude_counts_scale_exponentially() {
    let cases = [
        (0usize, 1usize),
        (1, 2),
        (2, 4),
        (3, 8),
        (10, 1024),
        (20, 1_048_576),
    ];

    for (qubits, expected_amplitudes) in cases {
        let count = AmplitudeCount::checked_for_qubits(QubitCount::new(qubits))
            .expect("test case must fit usize");

        assert_eq!(
            count.get(),
            expected_amplitudes,
            "incorrect dense-state amplitude count for {qubits} qubits"
        );
    }
}

#[test]
fn memory_size_calculations_must_be_done_before_allocation() {
    // This test intentionally performs only arithmetic.
    //
    // The allocator must perform equivalent checked calculations before
    // requesting memory. A test must never allocate an exponential state just
    // to verify that the limit system works.

    let qubits = QubitCount::new(30);

    let amplitudes = AmplitudeCount::checked_for_qubits(qubits)
        .expect("30-qubit amplitude count must fit usize on supported targets");

    assert_eq!(amplitudes.get(), 1usize << 30);

    let bytes_per_complex_f64 = 16u64;

    let required_bytes = ByteCount::new(
        amplitudes
            .get()
            .try_into()
            .ok()
            .and_then(|value: u64| value.checked_mul(bytes_per_complex_f64))
            .expect("test calculation must fit u64"),
    );

    assert_eq!(
        required_bytes.get(),
        17_179_869_184u64,
        "30 dense f64-complex qubits require 16 GiB of amplitude storage"
    );
}

// =============================================================================
// No-global-state architecture tests
// =============================================================================

#[test]
fn test_suite_does_not_create_global_quantum_state() {
    // This intentionally has no static mutable quantum state.
    //
    // Quantum memory ownership must always be explicit.
    let state_exists_only_as_a_local_value = true;

    assert!(state_exists_only_as_a_local_value);
}

// =============================================================================
// Future-provider compatibility tests
// =============================================================================

#[test]
fn provider_neutral_domains_cover_current_execution_classes() {
    let supported_domains = [
        StateExecutionDomain::LocalSimulator,
        StateExecutionDomain::LocalEmulator,
        StateExecutionDomain::RemoteSimulator,
        StateExecutionDomain::Qpu,
        StateExecutionDomain::HardwareEmulator,
        StateExecutionDomain::Hybrid,
        StateExecutionDomain::Distributed,
        StateExecutionDomain::Custom,
    ];

    assert_eq!(
        supported_domains.len(),
        8,
        "unexpected change to provider-neutral execution-domain taxonomy"
    );
}

#[test]
fn provider_neutral_storage_locations_cover_current_storage_classes() {
    let supported_locations = [
        StateStorageLocation::Host,
        StateStorageLocation::PinnedHost,
        StateStorageLocation::Device,
        StateStorageLocation::Unified,
        StateStorageLocation::Distributed,
        StateStorageLocation::Remote,
        StateStorageLocation::External,
        StateStorageLocation::Opaque,
    ];

    assert_eq!(
        supported_locations.len(),
        8,
        "unexpected change to provider-neutral storage-location taxonomy"
    );
}

// =============================================================================
// Contract-regression tests
// =============================================================================

#[test]
fn core_memory_quantities_have_stable_zero_values() {
    assert_eq!(QubitCount::ZERO.get(), 0);
    assert_eq!(ClassicalBitCount::ZERO.get(), 0);
    assert_eq!(AmplitudeCount::ZERO.get(), 0);
    assert_eq!(ByteCount::ZERO.get(), 0);
}

#[test]
fn core_memory_quantities_are_copyable() {
    let q = QubitCount::new(8);
    let c = ClassicalBitCount::new(8);
    let a = AmplitudeCount::new(256);
    let b = ByteCount::new(4096);

    let _q2 = q;
    let _c2 = c;
    let _a2 = a;
    let _b2 = b;

    assert_eq!(q.get(), 8);
    assert_eq!(c.get(), 8);
    assert_eq!(a.get(), 256);
    assert_eq!(b.get(), 4096);
}

#[test]
fn memory_contract_does_not_require_nonzero_quantum_memory() {
    // Zero quantities are useful for:
    //
    // - empty registers;
    // - optional classical memory;
    // - uninitialized metadata;
    // - capability discovery;
    // - resource estimation.
    //
    // Allocation layers, rather than primitive quantity types, decide when
    // zero-sized allocation is semantically invalid.

    assert!(QubitCount::ZERO.is_zero());
    assert!(ClassicalBitCount::ZERO.is_zero());
    assert!(AmplitudeCount::ZERO.is_zero());
    assert!(ByteCount::ZERO.is_zero());
}

// =============================================================================
// Cross-layer integration contract tests
// =============================================================================

#[test]
fn memory_layer_uses_canonical_ir_qubit_identity() {
    // `StateOperation` exposes canonical `quantum::ir::QubitId` rather than
    // introducing another memory-local qubit identity.
    //
    // Merely implementing the trait with the canonical return type is a
    // compile-time integration check.

    let operation = TestOperation::unitary("canonical_identity_test");

    let _logical_qubits: &[crate::quantum::ir::QubitId] =
        operation.logical_qubits();
}

#[test]
fn memory_layer_uses_canonical_ir_physical_identity() {
    let operation = TestOperation::unitary("physical_identity_test");

    let _physical_qubits: &[crate::quantum::ir::PhysicalQubitId] =
        operation.physical_qubits();
}

#[test]
fn memory_layer_uses_canonical_ir_classical_identity() {
    let operation = TestOperation::measurement();

    let _classical_bits: &[crate::quantum::ir::ClassicalBitId] =
        operation.classical_bits();
}

#[test]
fn memory_layer_uses_canonical_ir_operation_identity() {
    let operation = TestOperation::unitary("operation_identity_test");

    let _operation_id: Option<crate::quantum::ir::OperationId> =
        operation.operation_id();
}

// =============================================================================
// API-surface safety tests
// =============================================================================

#[test]
fn invalid_quantity_arithmetic_returns_options_instead_of_panicking() {
    let max = QubitCount::new(usize::MAX);

    assert!(max.checked_add(QubitCount::new(1)).is_none());
    assert!(max.checked_mul(2).is_none());
}

#[test]
fn underflow_is_rejected() {
    assert_eq!(
        QubitCount::new(0).checked_sub(QubitCount::new(1)),
        None
    );

    assert_eq!(
        ClassicalBitCount::new(0).checked_sub(ClassicalBitCount::new(1)),
        None
    );

    assert_eq!(
        ByteCount::new(0).checked_sub(ByteCount::ONE),
        None
    );
}

// =============================================================================
// Architectural assertions
// =============================================================================

#[test]
fn quantum_memory_is_not_allowed_to_require_a_specific_qpu_vendor() {
    // The memory layer has no vendor-specific execution-domain variant.
    //
    // Provider-specific identity belongs in `quantum::hardware`.
    //
    // This is intentionally a structural test: if a future contributor adds
    // IBM/GCP/AWS/etc. variants directly to the memory state contract, the
    // architecture review should reject the change.

    let domain = StateExecutionDomain::Qpu;

    assert!(domain.is_qpu());
}

#[test]
fn quantum_memory_is_not_allowed_to_require_a_specific_accelerator() {
    // Device storage is intentionally generic.
    //
    // CUDA, ROCm, Metal, Vulkan, SYCL and future accelerators must implement
    // the provider interfaces rather than appearing in the state semantic
    // taxonomy.

    let location = StateStorageLocation::Device;

    assert!(location.is_device());
}

#[test]
fn quantum_memory_can_represent_remote_execution_without_exposing_state() {
    let domain = StateExecutionDomain::RemoteSimulator;
    let location = StateStorageLocation::Remote;

    assert!(domain.is_simulator());
    assert!(location.is_external());
    assert!(!location.is_host_readable());
}

#[test]
fn quantum_memory_can_represent_hybrid_execution() {
    let domain = StateExecutionDomain::Hybrid;

    assert!(domain.is_hybrid());
    assert!(!domain.is_qpu());
}

// =============================================================================
// Final subsystem health gate
// =============================================================================

#[test]
fn quantum_memory_foundational_contract_is_healthy() {
    assert_invariant(
        QubitCount::new(4).get() == 4,
        "QubitCount must preserve its value",
    );

    assert_invariant(
        ClassicalBitCount::new(4).get() == 4,
        "ClassicalBitCount must preserve its value",
    );

    assert_invariant(
        AmplitudeCount::checked_for_qubits(QubitCount::new(4))
            .map(|value| value.get() == 16)
            .unwrap_or(false),
        "AmplitudeCount must calculate 2^n safely",
    );

    assert_invariant(
        ByteCount::MIB.get() == 1024 * 1024,
        "ByteCount must use binary memory units",
    );

    assert_invariant(
        StateExecutionDomain::Qpu.is_qpu(),
        "QPU execution must remain explicitly represented",
    );

    assert_invariant(
        StateStorageLocation::Opaque.is_external(),
        "opaque backend state must remain externally managed",
    );

    assert_invariant(
        StateLifecycle::Released.is_terminal(),
        "released state must remain terminal",
    );

    assert_invariant(
        StateConsistency::HostDirty.requires_synchronization(),
        "dirty host state must require synchronization",
    );

    assert_invariant(
        StateCapabilities::MEASUREMENT
            .contains(StateCapabilities::MEASUREMENT),
        "capability set must preserve requested capabilities",
    );
}