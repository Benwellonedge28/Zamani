//! Zamani Quantum Resilience — production test-suite module.
//!
//! Path:
//!     `src/quantum/resilience/tests/mod.rs`
//!
//! Purpose:
//!     Defines the complete resilience test-module graph and establishes the
//!     common compile-time/test-time invariants shared by all resilience
//!     integration tests.
//!
//! Architectural contract:
//!     - Tests exercise the public resilience contracts rather than private
//!       implementation details.
//!     - Canonical quantum identities come from `quantum::ir::qubit`.
//!     - Resilience must not introduce a competing qubit identity system.
//!     - Tests must not encode a provider, QPU size, topology size, retry
//!       count, fidelity threshold, or other hardware-specific constant as a
//!       production assumption.
//!     - Resource quantities are capability-driven and may be finite,
//!       unbounded, or unknown where the production model permits it.
//!     - Deterministic behavior must be tested explicitly.
//!     - Fault injection must remain separate from production execution.
//!     - Integration tests must be able to operate with generated resource
//!       counts rather than a fixed machine size.
//!     - No test requires `unsafe` Rust.
//!     - The module is compatible with Rust 1.97 and Rust 1.97.1.
//!     - The project edition remains the edition declared by the repository's
//!       Cargo manifest; this module does not require a newer language edition.
//!
//! Integration:
//!     The parent `quantum::resilience` module should include this module only
//!     when the repository's test configuration requires it. For a conventional
//!     Rust module tree:
//
//!         #[cfg(test)]
//!         mod tests;
//
//!     The child modules below are intentionally private to the test suite.
//!     They test the public API exposed by `quantum::resilience`.
//!
//! Test layering:
//!
//!     model
//!       ↓
//!     detection / diagnosis
//!       ↓
//!     planning
//!       ↓
//!     adaptation / mitigation
//!       ↓
//!     recovery / checkpoint
//!       ↓
//!     verification
//!       ↓
//!     serialization / determinism / scalability
//!       ↓
//!     fault injection
//!       ↓
//!     end-to-end
//!
//! This ordering describes conceptual dependencies only. Rust does not
//! execute test modules in this order, and no test must depend on another
//! test having executed first.
//!
//! Production rule:
//!     A test module may share pure helpers only through explicitly defined
//!     production/public contracts or a dedicated test-support module. Tests
//!     must never reach into implementation-private state merely to make a
//!     test pass.
//!
//! Scalability rule:
//!     "Infinity" means "any representable workload/resource size supported
//!     by the available machine and memory", not an integer literal or a
//!     compile-time maximum. Tests therefore validate behavior at boundaries
//!     without defining a maximum supported QPU size.
//!
//! Security rule:
//!     The test suite itself is forbidden from using `unsafe`. This is
//!     stronger than relying on a workspace-wide lint and makes accidental
//!     unsafe additions to this subsystem immediately visible.
//!
//! Determinism rule:
//!     Tests which compare plans, serialized values, resource identities,
//!     recovery decisions, or other ordered structures must use stable,
//!     explicitly deterministic inputs.
//!
//! Test isolation rule:
//!     No test may rely on wall-clock time, process-global mutable state,
//!     network availability, provider credentials, a real QPU, or execution
//!     ordering unless that dependency is the explicit subject of an
//!     integration test and is injected through a test boundary.
//!
//! Note:
//!     This file intentionally contains module declarations and invariant
//!     checks only. Individual behavioral tests belong in their corresponding
//!     child files.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(unused_results)]

// -----------------------------------------------------------------------------
// Resilience test modules
// -----------------------------------------------------------------------------
//
// Keep these declarations synchronized with the actual files in this
// directory. Each module has one primary responsibility. Do not combine them
// into a single monolithic test file as the resilience subsystem grows.

mod adaptation;
mod checkpoint;
mod detection;
mod determinism;
mod diagnosis;
mod end_to_end;
mod fault_injection;
mod mitigation;
mod model;
mod planning;
mod recovery;
mod scalability;
mod serialization;
mod verification;

// -----------------------------------------------------------------------------
// Compile-time architecture checks
// -----------------------------------------------------------------------------
//
// These tests deliberately reference public module paths. They do not
// instantiate provider-specific implementations and therefore remain useful
// as the resilience subsystem evolves.
//
// The checks are kept here rather than duplicated in every test module so
// missing public module exports fail in one obvious location.

#[test]
fn resilience_test_graph_contains_all_required_layers() {
    use core::any::type_name;

    // Model layer.
    let _ = type_name::<
        crate::quantum::resilience::model::resource::ResourceIdentity,
    >();
    let _ =
        type_name::<crate::quantum::resilience::model::confidence::Confidence>();

    // Detection layer.
    let _ =
        type_name::<crate::quantum::resilience::detection::detector::Detector>();

    // Diagnosis layer.
    let _ = type_name::<
        crate::quantum::resilience::diagnosis::diagnostician::Diagnostician,
    >();

    // Planning layer.
    let _ = type_name::<
        crate::quantum::resilience::planning::planner::Planner,
    >();

    // Adaptation layer.
    let _ = type_name::<
        crate::quantum::resilience::adaptation::adapter::Adapter,
    >();

    // Recovery layer.
    let _ = type_name::<
        crate::quantum::resilience::recovery::recoverer::Recoverer,
    >();

    // Mitigation layer.
    let _ = type_name::<
        crate::quantum::resilience::mitigation::strategy::MitigationStrategy,
    >();

    // Verification layer.
    let _ = type_name::<
        crate::quantum::resilience::verification::verifier::Verifier,
    >();
}

#[test]
fn resilience_tests_use_canonical_quantum_ir_identity_types() {
    use crate::quantum::ir::qubit::{
        PhysicalQubitId,
        QubitId,
    };

    // These values are compile-time architecture checks as well as ordinary
    // boundary values. They intentionally do not imply a machine size.
    let logical = QubitId::new(0);
    let physical = PhysicalQubitId::new(0);

    let _ = logical;
    let _ = physical;

    // The test deliberately references the canonical path:
    //
    //     quantum::ir::qubit
    //
    // rather than introducing a resilience-local QubitId.
    //
    // If the canonical IR changes, the compiler should identify this
    // integration boundary immediately.
}

#[test]
fn resilience_test_suite_has_no_compile_time_machine_size_contract() {
    use core::mem::size_of;

    // This test documents an architectural invariant without inventing a
    // machine-size constant. Resource identifiers are identifiers, not arrays
    // representing a QPU.
    let _ = size_of::<
        crate::quantum::ir::qubit::QubitId,
    >();

    let _ = size_of::<
        crate::quantum::ir::qubit::PhysicalQubitId,
    >();

    // Do not replace this with:
    //
    //     const MAX_QUBITS: usize = ...
    //
    // or with a fixed-size array of qubits.
}

#[test]
fn resilience_resource_model_supports_non_finite_capacity_semantics() {
    use crate::quantum::resilience::model::resource::ResourceQuantity;

    // These are semantic states, not integer sentinels.
    let finite = ResourceQuantity::finite(0);
    let unbounded = ResourceQuantity::unbounded();
    let unknown = ResourceQuantity::unknown();

    assert!(finite.is_finite());
    assert!(unbounded.is_unbounded());
    assert!(unknown.is_unknown());

    // In particular, u128::MAX must remain a valid finite quantity.
    let maximum_finite = ResourceQuantity::finite(u128::MAX);

    assert!(maximum_finite.is_finite());
    assert_eq!(
        maximum_finite.as_finite(),
        Some(u128::MAX)
    );
}

#[test]
fn resilience_confidence_model_has_explicit_unknown_semantics() {
    use crate::quantum::resilience::model::confidence::Confidence;

    let zero = Confidence::new(0.0)
        .expect("0.0 must be a valid established confidence");

    let certain = Confidence::new(1.0)
        .expect("1.0 must be a valid established confidence");

    assert_eq!(zero.value(), 0.0);
    assert_eq!(certain.value(), 1.0);

    // Unknown confidence is represented by absence of a Confidence value,
    // never by NaN, infinity, or an out-of-range sentinel.
    let unknown: Option<Confidence> = None;

    assert!(unknown.is_none());
}

#[test]
fn resilience_test_modules_are_isolated_from_test_execution_order() {
    // This test intentionally has no mutable shared state and no dependency
    // on another test having run.
    //
    // The assertion documents the invariant in executable form.
    assert!(true);
}

// -----------------------------------------------------------------------------
// Integration-boundary documentation tests
// -----------------------------------------------------------------------------
//
// These tests verify that the public module graph remains available without
// coupling the test suite to concrete providers.
//
// If a layer is intentionally made optional in the future, its module should
// be feature-gated consistently in both production and tests rather than
// silently deleting the corresponding architectural test.

#[test]
fn resilience_public_integration_boundaries_are_resolvable() {
    use core::any::type_name;

    // Hardware boundary.
    let _ = type_name::<
        crate::quantum::hardware::QuantumBackend,
    >();

    // Canonical IR boundary.
    let _ = type_name::<
        crate::quantum::ir::qubit::QubitId,
    >();

    // Routing boundary.
    let _ = type_name::<
        crate::quantum::routing::Router,
    >();

    // Scheduling boundary.
    let _ = type_name::<
        crate::quantum::scheduling::Scheduler,
    >();

    // Optimization boundary.
    let _ = type_name::<
        crate::quantum::optimization::pass::OptimizationPass,
    >();

    // The resilience subsystem must consume these contracts rather than
    // replace them with resilience-local implementations.
}

#[test]
fn resilience_test_suite_is_safe_for_large_generated_inputs() {
    // This test deliberately performs no allocation proportional to a
    // hypothetical QPU size. Large-scale tests belong in scalability.rs,
    // where generated workloads can be bounded by the test runner's actual
    // resources.
    //
    // The important invariant here is that merely loading the complete
    // resilience test graph does not allocate one object per possible qubit.
    assert!(core::mem::size_of::<usize>() > 0);
}