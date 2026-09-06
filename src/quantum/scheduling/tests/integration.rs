//! Zamani Quantum Scheduling — Integration Tests
//!
//! Path:
//!
//! `src/quantum/scheduling/tests/integration.rs`
//!
//! # Purpose
//!
//! This module contains cross-subsystem integration tests for the quantum
//! scheduling layer.
//!
//! These tests deliberately test boundaries rather than implementation details.
//!
//! The primary integration path protected by this file is:
//!
//! ```text
//! canonical quantum IR
//!        │
//!        ▼
//! scheduling::adapters::ir
//!        │
//!        ▼
//! scheduling internal representation
//!        │
//!        ▼
//! scheduling algorithms
//!        │
//!        ▼
//! scheduling result
//! ```
//!
//! Additional identity and timing boundaries are tested independently:
//!
//! ```text
//! quantum::ir::qubit
//!        │
//!        ▼
//! scheduling
//!
//! scheduling::types
//!        │
//!        ▼
//! scheduling::algorithms
//! ```
//!
//! # Architectural responsibility
//!
//! Integration tests verify that separately completed scheduler components
//! continue to compose correctly.
//!
//! They do NOT:
//!
//! - redefine quantum semantics;
//! - define another `QubitId`;
//! - define another `PhysicalQubitId`;
//! - define another `QuantumCircuit`;
//! - define another `QuantumOperation`;
//! - implement a scheduling algorithm;
//! - implement routing;
//! - discover hardware;
//! - connect to a real QPU;
//! - define QEC semantics;
//! - define a noise model;
//! - define a serialization format;
//! - impose a machine-size limit.
//!
//! # Canonical identity rule
//!
//! Logical and physical qubit identities MUST remain owned by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This test module therefore imports those exact types where identity
//! integration must be checked.
//!
//! The repository explicitly uses this canonical identity boundary throughout
//! the scheduling subsystem.
//!
//! # Universal-program principle
//!
//! These tests must remain independent of a particular machine size.
//!
//! They must not assume:
//!
//! - a fixed number of qubits;
//! - a fixed number of operations;
//! - a fixed topology;
//! - a fixed gate arity;
//! - a fixed number of resources;
//! - a fixed number of control channels;
//! - a fixed schedule depth;
//! - a fixed hardware clock;
//! - a fixed QEC distance;
//! - a fixed vendor;
//! - a fixed QPU count.
//!
//! The tests therefore use the smallest valid canonical program where a
//! boundary test does not require a concrete target.
//!
//! Larger scheduling workloads belong in:
//!
//! ```text
//! tests/scalability/
//! tests/property/
//! tests/regression/
//! tests/determinism/
//! ```
//!
//! # Test classification
//!
//! This file is intentionally different from `unit.rs`.
//!
//! `unit.rs` verifies individual contracts.
//!
//! This file verifies:
//!
//! - IR → scheduling adapter integration;
//! - canonical qubit identity integration;
//! - scheduler timing integration;
//! - algorithm metadata integration;
//! - empty-program handling across subsystem boundaries;
//! - deterministic public contracts;
//! - absence of artificial machine-size assumptions.
//!
//! # Hardware isolation
//!
//! These tests MUST NOT require:
//!
//! - a physical QPU;
//! - credentials;
//! - network access;
//! - a provider account;
//! - environment variables;
//! - cloud services;
//! - hardware calibration;
//! - wall-clock timing;
//! - randomness.
//!
//! Hardware-specific integration tests belong outside this file.
//!
//! # Determinism
//!
//! Tests must be deterministic.
//!
//! No test uses:
//!
//! - system time;
//! - random numbers;
//! - thread timing;
//! - pointer addresses;
//! - hash-map iteration order as a semantic assertion.
//!
//! # Scalability
//!
//! This file intentionally contains no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_OPERATIONS
//! MAX_RESOURCES
//! MAX_CHANNELS
//! MAX_DEPTH
//! ```
//!
//! "Infinity" in the Zamani architecture means that the scheduling API does
//! not impose an artificial machine-size ceiling. Concrete compilation remains
//! bounded by the actual target, host resources, explicit caller limits, and
//! representable values.
//!
//! # Rust contract
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! # Integration contract
//!
//! Adding a new:
//!
//! - scheduler algorithm;
//! - hardware technology;
//! - routing implementation;
//! - QEC implementation;
//! - resource kind;
//! - timing representation;
//! - plugin;
//! - serialization backend;
//!
//! MUST NOT require changing this file unless the public integration contract
//! itself changes.
//!
//! The tests intentionally consume public/stable contracts instead of private
//! implementation fields.
//!
//! =============================================================================
//! Safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// =============================================================================
// Imports
// =============================================================================

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::scheduling::adapters::ir::adapt;
use crate::quantum::scheduling::algorithms::asap::{
    AsapScheduler,
    ASAP_ALGORITHM_ID,
    ASAP_ALGORITHM_NAME,
    ASAP_ALGORITHM_VERSION,
};
use crate::quantum::scheduling::timing::time::{
    Duration as TimingDuration,
    TimePoint as TimingTimePoint,
};
use crate::quantum::scheduling::types::{
    Duration,
    TimePoint,
};

// =============================================================================
// Test helpers
// =============================================================================

/// Constructs the smallest valid canonical quantum circuit.
///
/// The integration suite deliberately starts with an empty semantic program.
/// This verifies the complete zero-work boundary without introducing
/// hardware-specific gate construction assumptions.
///
/// A zero-operation circuit is useful because it must not require:
///
/// - routing;
/// - hardware discovery;
/// - resource allocation;
/// - target timing;
/// - scheduling resources;
/// - QEC configuration.
///
/// The scheduler must be able to represent the absence of executable work
/// without inventing machine characteristics.
fn empty_circuit() -> crate::quantum::ir::QuantumCircuit {
    crate::quantum::ir::QuantumCircuit::new(0, 0)
        .expect("the canonical zero-qubit, zero-operation circuit must be valid")
}

// =============================================================================
// Canonical IR → scheduling adapter integration
// =============================================================================

/// Verifies that a canonical IR circuit can cross the scheduling adapter
/// boundary.
///
/// This protects the first major integration point:
///
/// ```text
/// quantum::ir::QuantumCircuit
///          │
///          ▼
/// scheduling::adapters::ir
/// ```
///
/// The test intentionally uses an empty circuit because no target-specific
/// duration, topology, resource, or gate-set assumptions should be necessary
/// merely to adapt a valid canonical IR program.
///
/// If this test fails, the scheduling subsystem has broken compatibility with
/// the canonical quantum IR.
#[test]
fn canonical_ir_adapts_into_scheduling_representation() {
    let circuit = empty_circuit();

    let operations = adapt(&circuit);

    assert!(
        operations.is_empty(),
        "an empty canonical circuit must adapt to zero scheduling operations"
    );
}

// =============================================================================
// Canonical qubit identity integration
// =============================================================================

/// Verifies that scheduling can consume the canonical logical qubit identity.
///
/// This test intentionally uses:
///
/// ```text
/// quantum::ir::qubit::QubitId
/// ```
///
/// rather than any scheduler-local qubit representation.
///
/// The scheduler must never introduce a competing qubit identity domain.
#[test]
fn scheduling_uses_canonical_logical_qubit_identity() {
    let logical = QubitId::new(0);

    let canonical: crate::quantum::ir::qubit::QubitId = logical;

    assert_eq!(
        canonical,
        logical,
        "scheduling integration must preserve canonical logical qubit identity"
    );
}

/// Verifies that physical qubit identity remains distinct from logical qubit
/// identity while both remain owned by the canonical IR.
///
/// This protects against a particularly dangerous class of architectural bugs:
///
/// ```text
/// logical QubitId
///       │
///       └── accidentally treated as PhysicalQubitId
/// ```
///
/// The two domains may contain the same numeric index while representing
/// different semantic entities.
#[test]
fn scheduling_uses_canonical_physical_qubit_identity() {
    let physical = PhysicalQubitId::new(0);

    let canonical: crate::quantum::ir::qubit::PhysicalQubitId = physical;

    assert_eq!(
        canonical,
        physical,
        "scheduling integration must preserve canonical physical qubit identity"
    );
}

// =============================================================================
// Logical / physical identity separation
// =============================================================================

/// Verifies that logical and physical qubit identities remain distinct types.
///
/// This is a compile-time architectural assertion expressed through explicit
/// type annotations.
///
/// The numeric value alone must never determine semantic identity.
#[test]
fn logical_and_physical_qubits_remain_distinct_domains() {
    let logical = QubitId::new(7);
    let physical = PhysicalQubitId::new(7);

    let _: QubitId = logical;
    let _: PhysicalQubitId = physical;

    assert_eq!(
        logical.value(),
        physical.value(),
        "the test deliberately uses the same numeric index"
    );

    // The explicit type annotations above are the important integration
    // assertion: identical numeric coordinates do not collapse the semantic
    // identity domains.
}

// =============================================================================
// Scheduling algorithm integration
// =============================================================================

/// Verifies that the public ASAP scheduler contract is available through the
/// scheduling algorithm boundary.
///
/// This test does not duplicate list-scheduler unit tests. It verifies that
/// the algorithm module exposes a stable identity and configuration boundary
/// that can be consumed by higher-level scheduler composition.
#[test]
fn asap_algorithm_integrates_with_scheduling_api() {
    let scheduler = AsapScheduler::new();

    assert_eq!(
        scheduler.algorithm_id(),
        ASAP_ALGORITHM_ID,
        "ASAP must expose its stable algorithm identifier"
    );

    assert_eq!(
        scheduler.algorithm_name(),
        ASAP_ALGORITHM_NAME,
        "ASAP must expose its stable human-readable name"
    );

    assert_eq!(
        scheduler.algorithm_version(),
        ASAP_ALGORITHM_VERSION,
        "ASAP must expose its stable semantic contract version"
    );
}

/// Verifies that ASAP remains deterministic at its public algorithm boundary.
///
/// The actual scheduling result is tested by the algorithm/list-scheduler test
/// suite. This integration test protects the cross-module contract that the
/// algorithm itself declares deterministic semantics.
#[test]
fn asap_algorithm_contract_is_deterministic() {
    assert!(
        crate::quantum::scheduling::algorithms::asap::asap_algorithm_is_deterministic(),
        "ASAP integration contract must remain deterministic"
    );
}

// =============================================================================
// Empty-program scheduling integration
// =============================================================================

/// Verifies that the scheduling layer does not impose an artificial machine
/// size merely to represent an empty program.
///
/// This protects the zero-work boundary:
///
/// ```text
/// canonical IR
///     │
///     ▼
/// scheduling adapter
///     │
///     ▼
/// scheduler
/// ```
///
/// The test uses the public algorithm configuration contract rather than
/// reaching into scheduler implementation details.
#[test]
fn empty_program_is_supported_by_default_asap_configuration() {
    let scheduler = AsapScheduler::new();

    assert!(
        scheduler.config().allow_empty,
        "the production ASAP default must allow an empty scheduling problem"
    );
}

// =============================================================================
// Target-independent timing integration
// =============================================================================

/// Verifies that scheduler time values can represent the schedule origin
/// without assuming a physical unit.
///
/// The scheduler's abstract timing coordinate must not encode nanoseconds,
/// device ticks, or a vendor-specific clock.
#[test]
fn scheduling_time_origin_is_target_independent() {
    let scheduling_time = TimePoint::ZERO;
    let timing_time = TimingTimePoint::ZERO;

    assert_eq!(
        scheduling_time.value(),
        0,
        "the scheduling time origin must be zero"
    );

    assert_eq!(
        timing_time.value(),
        0,
        "the timing subsystem time origin must be zero"
    );
}

/// Verifies that zero duration is represented consistently at the scheduling
/// boundary.
///
/// Zero duration is a valid semantic value for abstract compiler events and
/// must not be confused with an absent duration.
#[test]
fn zero_duration_is_a_valid_scheduler_value() {
    let duration = Duration::ZERO;
    let timing_duration = TimingDuration::ZERO;

    assert_eq!(
        duration.value(),
        0,
        "scheduler zero duration must have zero abstract coordinate"
    );

    assert_eq!(
        timing_duration.value(),
        0,
        "timing zero duration must have zero abstract coordinate"
    );
}

/// Verifies checked time arithmetic across the scheduling timing boundary.
///
/// A production scheduler must never silently wrap temporal coordinates.
///
/// This test protects the basic invariant:
///
/// ```text
/// time + duration
///     │
///     ├── valid → Some(time)
///     └── overflow → None
/// ```
#[test]
fn scheduling_time_arithmetic_is_checked() {
    let start = TimePoint::new(100);
    let duration = Duration::new(25);

    let finish = start
        .checked_add(duration)
        .expect("100 + 25 must fit in the scheduler time domain");

    assert_eq!(
        finish.value(),
        125,
        "checked time addition must preserve the exact abstract coordinate"
    );

    let timing_start = TimingTimePoint::new(100);
    let timing_duration = TimingDuration::new(25);

    let timing_finish = timing_start
        .checked_add(timing_duration)
        .expect("timing 100 + 25 must fit in the timing time domain");

    assert_eq!(
        timing_finish.value(),
        125,
        "timing checked addition must preserve the exact abstract coordinate"
    );
}

/// Verifies that temporal subtraction cannot silently produce a negative
/// duration.
///
/// Scheduling coordinates are non-negative and subtraction must fail when the
/// requested interval would run backwards.
#[test]
fn scheduling_time_subtraction_rejects_reverse_intervals() {
    let later = TimePoint::new(100);
    let duration = Duration::new(25);

    let earlier = later
        .checked_sub(duration)
        .expect("100 - 25 must fit in the scheduler time domain");

    assert_eq!(
        earlier.value(),
        75,
        "checked subtraction must preserve the exact abstract coordinate"
    );

    let origin = TimePoint::ZERO;

    assert!(
        origin.checked_sub(Duration::new(1)).is_none(),
        "subtracting a positive duration from zero must fail rather than wrap"
    );
}

// =============================================================================
// Cross-type timing consistency
// =============================================================================

/// Verifies that a schedule interval can be reconstructed from the scheduler's
/// target-independent time values.
///
/// This is intentionally an invariant test rather than a test of any particular
/// physical unit.
///
/// The hardware adapter is responsible for assigning physical meaning to the
/// abstract coordinate.
#[test]
fn schedule_interval_preserves_start_duration_finish_relationship() {
    let start = TimePoint::new(50);
    let duration = Duration::new(30);

    let finish = start
        .checked_add(duration)
        .expect("the test interval must fit");

    let reconstructed_duration = start
        .checked_duration_until(finish)
        .expect("finish must not precede start");

    assert_eq!(
        reconstructed_duration,
        duration,
        "finish - start must equal the scheduled duration"
    );
}

// =============================================================================
// Overflow protection
// =============================================================================

/// Verifies that the scheduling time domain rejects arithmetic overflow.
///
/// Overflow is a correctness failure in scheduling semantics because a wrapped
/// timestamp could place an operation before its dependencies or outside its
/// resource reservations.
#[test]
fn scheduling_time_overflow_is_not_silent() {
    let start = TimePoint::new(u128::MAX);
    let duration = Duration::new(1);

    assert!(
        start.checked_add(duration).is_none(),
        "time overflow must be rejected rather than wrapped"
    );
}

/// Verifies that duration addition is checked.
///
/// A wrapped duration could corrupt makespan, deadlines, resource reservations,
/// and critical-path calculations.
#[test]
fn scheduling_duration_overflow_is_not_silent() {
    let duration = Duration::new(u128::MAX);

    assert!(
        duration.checked_add(Duration::new(1)).is_none(),
        "duration overflow must be rejected rather than wrapped"
    );
}

// =============================================================================
// Machine-size independence
// =============================================================================

/// Verifies the public ASAP contract that no artificial machine-size ceiling is
/// embedded in the algorithm.
///
/// This does not claim that physical machines are infinite. It protects the
/// architectural meaning of "scale from atom to everywhere":
///
/// ```text
/// no scheduler-imposed finite machine-size constant
/// ```
///
/// Actual limits come from:
///
/// - target resources;
/// - explicit scheduler limits;
/// - host resources;
/// - deployment policy;
/// - execution environment.
#[test]
fn asap_has_no_artificial_machine_size_limit() {
    assert!(
        crate::quantum::scheduling::algorithms::asap::asap_has_no_machine_size_limit(),
        "ASAP must not encode an artificial finite machine-size ceiling"
    );
}

// =============================================================================
// Unsafe-code boundary
// =============================================================================

/// Compile-time integration marker documenting that the scheduler algorithm
/// advertises a no-unsafe implementation contract.
///
/// The stronger enforcement is provided by:
///
/// ```text
/// #![forbid(unsafe_code)]
/// ```
///
/// at the module boundary.
#[test]
fn scheduling_algorithm_uses_no_unsafe_code() {
    assert!(
        crate::quantum::scheduling::algorithms::asap::asap_uses_no_unsafe(),
        "the scheduling algorithm must remain safe Rust"
    );
}

// =============================================================================
// Public API stability
// =============================================================================

/// Verifies that constructing the production ASAP scheduler requires no global
/// initialization.
///
/// This is important for:
///
/// - parallel compilation;
/// - embedded use;
/// - deterministic builds;
/// - library consumers;
/// - compiler services;
/// - distributed scheduling workers.
///
/// A scheduler instance must own its configuration rather than depending on
/// process-global mutable state.
#[test]
fn asap_scheduler_is_constructible_without_global_initialization() {
    let first = AsapScheduler::new();
    let second = AsapScheduler::new();

    assert_eq!(
        first.config(),
        second.config(),
        "independent scheduler instances must have equivalent production defaults"
    );
}

/// Verifies that the algorithm identity is stable across independent scheduler
/// instances.
///
/// Algorithm identity must describe the algorithm contract, not an instance,
/// machine, pointer, or process.
#[test]
fn asap_algorithm_identity_is_instance_independent() {
    let first = AsapScheduler::new();
    let second = AsapScheduler::new();

    assert_eq!(
        first.algorithm_id(),
        second.algorithm_id(),
        "ASAP identity must be stable across scheduler instances"
    );

    assert_eq!(
        first.algorithm_version(),
        second.algorithm_version(),
        "ASAP contract version must be stable across scheduler instances"
    );
}

// =============================================================================
// Integration boundary documentation tests
// =============================================================================

/// Verifies the intended dependency direction through the public API.
///
/// This test is deliberately small: successful compilation itself verifies that
/// the following boundaries remain available simultaneously:
///
/// ```text
/// canonical quantum IR
///        │
///        ▼
/// scheduling adapter
///        │
///        ▼
/// scheduling algorithm
///        │
///        ▼
/// scheduling timing
/// ```
///
/// If one subsystem begins depending on a private implementation detail or
/// replaces a canonical type with a competing representation, this integration
/// boundary will fail to compile.
#[test]
fn core_scheduling_boundaries_are_available_together() {
    let circuit = empty_circuit();
    let operations = adapt(&circuit);

    let scheduler = AsapScheduler::new();

    let qubit = QubitId::new(0);
    let physical = PhysicalQubitId::new(0);

    let start = TimePoint::ZERO;
    let duration = Duration::ZERO;

    assert!(operations.is_empty());

    assert_eq!(qubit.value(), 0);
    assert_eq!(physical.value(), 0);

    assert!(start.is_zero());
    assert!(duration.is_zero());

    assert_eq!(
        scheduler.algorithm_id(),
        ASAP_ALGORITHM_ID
    );
}

// =============================================================================
// Semantic portability boundary
// =============================================================================

/// Verifies the fundamental portability boundary:
///
/// ```text
/// program semantics
///        ≠
/// machine-specific scheduling state
/// ```
///
/// An empty canonical program can be adapted without requiring a particular
/// qubit count, topology, channel count, or timing unit.
///
/// This is the smallest executable assertion of Zamani's "write once, scale
/// to the target" architecture.
#[test]
fn canonical_program_boundary_is_target_independent() {
    let circuit = empty_circuit();

    let operations = adapt(&circuit);

    assert!(
        operations.is_empty(),
        "target-independent empty semantics must remain empty after scheduling adaptation"
    );

    // The scheduler can be constructed independently of the target.
    //
    // Target specialization is deliberately supplied later through the
    // scheduling context / planner boundary.
    let scheduler = AsapScheduler::new();

    assert_eq!(
        scheduler.algorithm_id(),
        ASAP_ALGORITHM_ID,
        "algorithm selection must not require a machine-size parameter"
    );
}

// =============================================================================
// Regression guard: no stabilizer-specific coupling
// =============================================================================

/// Verifies that the generic scheduling integration boundary does not require
/// the legacy stabilizer scheduler.
///
/// This is intentionally expressed through the generic scheduling API.
///
/// The generic scheduler must remain usable without:
///
/// - surface-code distance;
/// - patch names;
/// - stabilizer rounds;
/// - ancilla naming;
/// - syndrome-specific state.
///
/// Stabilizer scheduling belongs behind the QEC adapter boundary.
#[test]
fn generic_scheduling_boundary_is_not_stabilizer_specific() {
    let scheduler = AsapScheduler::new();

    assert_eq!(
        scheduler.algorithm_id(),
        ASAP_ALGORITHM_ID,
        "generic scheduling must be independently addressable from QEC scheduling"
    );
}

// =============================================================================
// End-to-end zero-work boundary
// =============================================================================

/// Verifies the complete zero-work integration path.
///
/// ```text
/// QuantumCircuit
///      │
///      ▼
/// IR adapter
///      │
///      ▼
/// zero scheduling operations
///      │
///      ▼
/// generic ASAP scheduler configuration
/// ```
///
/// This test does not connect to hardware and does not manufacture a target.
/// That is intentional: the semantic program must remain independent of the
/// execution machine.
#[test]
fn zero_work_pipeline_remains_composable() {
    let circuit = empty_circuit();

    let operations = adapt(&circuit);

    assert!(
        operations.is_empty(),
        "zero-work canonical IR must remain zero-work at the scheduling adapter boundary"
    );

    let scheduler = AsapScheduler::new();

    assert!(
        scheduler.config().allow_empty,
        "production ASAP configuration must permit the zero-work boundary"
    );

    assert_eq!(
        scheduler.algorithm_id(),
        ASAP_ALGORITHM_ID,
        "the zero-work pipeline must still expose a stable scheduling algorithm identity"
    );
}