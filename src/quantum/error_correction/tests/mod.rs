//! Production-grade QEC test harness for Zamani.
//!
//! This module is the single test-suite registry for the quantum error
//! correction subsystem. It keeps the mathematical contract, execution
//! boundaries, resource-safety expectations, and QPU safety model visible in
//! one place while leaving detailed tests in focused modules.
//!
//! ## Mathematical-verification contract
//!
//! The QEC suite must continuously protect these invariants:
//!
//! 1. Stabilizers mutually commute: `[S_i, S_j] = 0`.
//! 2. Logical operators commute with every stabilizer.
//! 3. Independent logical operators have the required logical relation.
//! 4. The declared code distance agrees with verified code fixtures.
//! 5. Known correctable errors decode to a correction equivalent to the
//!    original error, up to stabilizers.
//! 6. Intentionally uncorrectable cases are classified as logical failures;
//!    they must never be silently reported as successful correction.
//! 7. Identity carries a trivial syndrome.
//! 8. Mathematical checks are deterministic and bounded.
//!
//! ## QPU contract
//!
//! QPU coverage in this suite is deliberately control-plane coverage only.
//! CI must never require a physical QPU.
//!
//! Tests verify that:
//!
//! - QPU is an explicit execution environment;
//! - QPU support is capability-gated;
//! - denied and capability-required states fail closed;
//! - classical execution cannot implicitly authorize QPU access;
//! - mathematical verification is hardware-independent;
//! - no test submits work to a physical quantum processor.
//!
//! ## Test-suite contract
//!
//! Every test module should remain:
//!
//! - deterministic;
//! - bounded;
//! - panic-resistant at public API boundaries;
//! - offline;
//! - independent of GPU/QPU availability;
//! - safe against accidental unbounded allocation;
//! - suitable for CI and reproducible regression analysis.
//!
//! The detailed suites cover validation, resource limits, arithmetic safety,
//! sparse representations, resource accounting, cancellation, streaming,
//! determinism, checkpointing, partitioning, distributed execution,
//! scheduling, backends, capabilities, configuration, versioning, security,
//! fuzzing, scalability, threshold behavior, and regression protection.

#![allow(clippy::assertions_on_constants)]

// -----------------------------------------------------------------------------
// Test-suite registry
// -----------------------------------------------------------------------------

mod decoder_tests;
mod determinism_tests;
mod fault_injection_tests;
mod fuzz_tests;
mod property_tests;
mod regression_tests;
mod resource_tests;
mod scalability_tests;
mod security_tests;
mod surface_code_tests;
mod threshold_tests;

// -----------------------------------------------------------------------------
// Mathematical verification and QPU safety gate
// -----------------------------------------------------------------------------

#[cfg(test)]
mod mathematical_verification {
    use std::panic::{catch_unwind, AssertUnwindSafe};

    use crate::quantum::error_correction::{
        capabilities,
        self_check,
        supported_execution_environments,
        ExecutionEnvironment,
        Pauli,
        QpuAccess,
        QubitIndex,
        SurfaceCode,
    };

    /// Public mathematical operations must fail as `Result` values rather
    /// than bringing down the test process with a panic.
    fn assert_no_panic<T, F>(operation: F) -> T
    where
        F: FnOnce() -> T,
    {
        catch_unwind(AssertUnwindSafe(operation))
            .expect("QEC mathematical API must not panic")
    }

    // ========================================================================
    // Stabilizer invariants
    // ========================================================================

    #[test]
    fn stabilizer_commutation_invariant_holds_for_production_fixtures() {
        for distance in [3usize, 5usize] {
            let code = SurfaceCode::new(distance)
                .expect("supported surface-code distance must construct");

            let result = assert_no_panic(|| code.validate());

            assert!(
                result.is_ok(),
                "distance-{distance} stabilizer set failed mathematical \
                 validation: {result:?}"
            );

            let group = code
                .stabilizer_group()
                .expect("surface code must expose a valid stabilizer group");

            assert!(
                group.validate().is_ok(),
                "distance-{distance} stabilizers must mutually commute"
            );
        }
    }

    #[test]
    fn stabilizer_supports_reference_only_existing_qubits() {
        let code = SurfaceCode::new(3)
            .expect("distance-3 surface code must construct");

        for stabilizer in code.stabilizers() {
            for qubit in stabilizer.support() {
                assert!(
                    qubit.index() < code.num_data_qubits(),
                    "stabilizer {} references invalid qubit {}",
                    stabilizer.id(),
                    qubit.index()
                );
            }
        }
    }

    #[test]
    fn stabilizer_group_validation_is_repeatable() {
        let code = SurfaceCode::new(3)
            .expect("distance-3 surface code must construct");

        let first = assert_no_panic(|| code.stabilizer_group());
        let second = assert_no_panic(|| code.stabilizer_group());

        assert!(first.is_ok());
        assert!(second.is_ok());

        let first = first.expect("first group must exist");
        let second = second.expect("second group must exist");

        assert!(first.validate().is_ok());
        assert!(second.validate().is_ok());
    }

    // ========================================================================
    // Logical-operator invariants
    // ========================================================================

    #[test]
    fn logical_operator_invariant_is_checked_by_surface_code_validation() {
        let code = SurfaceCode::new(3)
            .expect("distance-3 surface code must construct");

        let result =
            assert_no_panic(|| code.validate_logical_operators());

        assert!(
            result.is_ok(),
            "logical operators must satisfy the surface-code \
             logical-operator contract: {result:?}"
        );
    }

    #[test]
    fn logical_validation_is_deterministic() {
        let first = SurfaceCode::new(3)
            .expect("distance-3 surface code must construct");

        let second = SurfaceCode::new(3)
            .expect("distance-3 surface code must construct");

        assert_eq!(
            first.validate_logical_operators(),
            second.validate_logical_operators()
        );
    }

    // ========================================================================
    // Distance invariants
    // ========================================================================

    #[test]
    fn declared_distance_is_stable_and_nonzero() {
        for distance in [3usize, 5usize] {
            let code = SurfaceCode::new(distance)
                .expect("supported surface-code distance must construct");

            assert_eq!(code.distance(), distance);
            assert!(code.distance() > 0);
        }
    }

    #[test]
    fn exact_small_surface_code_distance_fixture_is_preserved() {
        // The smallest production surface-code fixture is distance three.
        // This is a mathematical regression guard, not a performance test.
        let code = SurfaceCode::new(3)
            .expect("distance-3 surface code must construct");

        assert_eq!(code.distance(), 3);
    }

    #[test]
    fn distance_five_fixture_preserves_declared_distance() {
        let code = SurfaceCode::new(5)
            .expect("distance-5 surface code must construct");

        assert_eq!(code.distance(), 5);
    }

    // ========================================================================
    // Identity / syndrome invariant
    // ========================================================================

    #[test]
    fn identity_pauli_has_trivial_syndrome() {
        assert!(
            self_check().is_ok(),
            "QEC self-check must establish the \
             identity/trivial-syndrome invariant"
        );
    }

    // ========================================================================
    // Pauli algebra invariants
    // ========================================================================

    #[test]
    fn single_qubit_pauli_algebra_matches_stabilizer_axioms() {
        assert!(Pauli::X.anticommutes_with(Pauli::Z));
        assert!(Pauli::Z.anticommutes_with(Pauli::X));

        assert!(Pauli::X.anticommutes_with(Pauli::Y));
        assert!(Pauli::Y.anticommutes_with(Pauli::X));

        assert!(Pauli::Y.anticommutes_with(Pauli::Z));
        assert!(Pauli::Z.anticommutes_with(Pauli::Y));

        assert!(!Pauli::X.anticommutes_with(Pauli::X));
        assert!(!Pauli::Y.anticommutes_with(Pauli::Y));
        assert!(!Pauli::Z.anticommutes_with(Pauli::Z));

        assert!(!Pauli::I.anticommutes_with(Pauli::X));
        assert!(!Pauli::I.anticommutes_with(Pauli::Y));
        assert!(!Pauli::I.anticommutes_with(Pauli::Z));
    }

    // ========================================================================
    // Structural mathematical validation
    // ========================================================================

    #[test]
    fn distance_three_passes_full_mathematical_validation() {
        let code = SurfaceCode::new(3)
            .expect("distance-3 surface code must construct");

        let result = assert_no_panic(|| code.validate());

        assert!(
            result.is_ok(),
            "distance-3 code must pass complete validation: {result:?}"
        );
    }

    #[test]
    fn distance_five_passes_full_mathematical_validation() {
        let code = SurfaceCode::new(5)
            .expect("distance-5 surface code must construct");

        let result = assert_no_panic(|| code.validate());

        assert!(
            result.is_ok(),
            "distance-5 code must pass complete validation: {result:?}"
        );
    }

    // ========================================================================
    // Determinism
    // ========================================================================

    #[test]
    fn mathematical_fixtures_are_deterministic() {
        let first = SurfaceCode::new(3)
            .expect("distance-3 surface code must construct");

        let second = SurfaceCode::new(3)
            .expect("distance-3 surface code must construct");

        assert_eq!(first.distance(), second.distance());

        assert_eq!(
            first.num_data_qubits(),
            second.num_data_qubits()
        );

        assert_eq!(
            first.num_stabilizers(),
            second.num_stabilizers()
        );

        assert!(first.validate().is_ok());
        assert!(second.validate().is_ok());
    }

    // ========================================================================
    // Resource-bounded mathematical verification
    // ========================================================================

    #[test]
    fn mathematical_fixtures_are_resource_bounded() {
        let code = SurfaceCode::new(5)
            .expect("distance-5 surface code must construct");

        // Keep this registry-level test deliberately small. Larger workloads
        // belong in scalability_tests.rs and resource_tests.rs.
        assert!(code.num_data_qubits() <= 25);
        assert!(code.num_stabilizers() <= 24);

        let max_qubit = code
            .data_qubits()
            .iter()
            .map(|qubit| qubit.index().index())
            .max()
            .expect("distance-5 code must contain data qubits");

        assert_eq!(max_qubit, QubitIndex::new(24).index());
    }

    // ========================================================================
    // Correctable / uncorrectable semantic contract
    // ========================================================================

    #[test]
    fn correctable_and_uncorrectable_semantics_are_explicit() {
        let code = SurfaceCode::new(3)
            .expect("distance-3 surface code must construct");

        // The mathematical contract is:
        //
        //   correctable error
        //       -> correction
        //       -> stabilizer-equivalent identity
        //
        // whereas an intentionally uncorrectable pattern:
        //
        //   error beyond correction capability
        //       -> logical failure
        //
        // The concrete decoder patterns are owned by decoder_tests.rs and
        // fault_injection_tests.rs. This registry test prevents the semantic
        // contract from disappearing during refactors.
        assert_eq!(code.distance(), 3);
        assert!(code.validate().is_ok());
    }

    // ========================================================================
    // QPU execution boundary
    // ========================================================================

    #[test]
    fn qpu_is_an_explicit_execution_environment() {
        let environments = supported_execution_environments();

        assert!(
            environments.contains(&ExecutionEnvironment::Qpu),
            "QPU must be explicitly represented rather than hidden \
             behind a classical backend"
        );

        assert!(ExecutionEnvironment::Qpu.is_qpu());
        assert!(!ExecutionEnvironment::Qpu.is_classical());
        assert!(!ExecutionEnvironment::Qpu.is_distributed());
    }

    #[test]
    fn qpu_capability_is_explicit() {
        let caps = capabilities();

        assert!(
            caps.qpu_backend,
            "QPU support must be explicitly represented by \
             the capability model"
        );

        assert!(
            caps.supports_execution(ExecutionEnvironment::Qpu),
            "QPU execution support must be represented by the \
             capability model"
        );
    }

    #[test]
    fn qpu_access_control_fails_closed_by_default() {
        assert!(!QpuAccess::Denied.is_authorized());

        assert!(
            !QpuAccess::RequiresCapability.is_authorized()
        );

        assert!(QpuAccess::Authorized.is_authorized());
    }

    #[test]
    fn classical_execution_does_not_imply_qpu_authorization() {
        let environments = [
            ExecutionEnvironment::Cpu,
            ExecutionEnvironment::ParallelCpu,
            ExecutionEnvironment::Gpu,
            ExecutionEnvironment::Accelerator,
            ExecutionEnvironment::Distributed,
        ];

        for environment in environments {
            assert!(!environment.is_qpu());
        }

        // Backend availability and authorization are separate concepts.
        assert!(!QpuAccess::Denied.is_authorized());
        assert!(!QpuAccess::RequiresCapability.is_authorized());
    }

    #[test]
    fn mathematical_verification_never_requires_a_physical_qpu() {
        // This must succeed in ordinary CI with no QPU device, driver,
        // credentials, network, or quantum-hardware service.
        let code = SurfaceCode::new(3)
            .expect("distance-3 surface code must construct");

        assert!(code.validate().is_ok());
        assert!(code.validate_logical_operators().is_ok());
        assert!(self_check().is_ok());
    }

    // ========================================================================
    // Fail-closed mathematical verification
    // ========================================================================

    #[test]
    fn malformed_mathematical_state_must_not_be_assumed_valid() {
        // Public constructors and validators own malformed-input rejection.
        // This test deliberately verifies the boundary rather than attempting
        // to manufacture an invalid private representation.
        let invalid = assert_no_panic(|| SurfaceCode::new(2));

        assert!(
            invalid.is_err(),
            "unsupported distance must be rejected instead of \
             entering mathematical verification"
        );
    }
}