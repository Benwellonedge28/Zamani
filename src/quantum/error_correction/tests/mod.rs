//! Zamani Quantum Error Correction test-suite registry.
//!
//! This module is intentionally a registry/integration layer.
//!
//! Detailed tests belong in focused files. In particular,
//! `mathematical_verification.rs` is registered directly rather than being
//! duplicated here.
//!
//! # Verification contract
//!
//! The complete QEC test suite protects:
//!
//! - stabilizer commutation;
//! - logical-operator invariants;
//! - code-distance correctness;
//! - syndrome correctness;
//! - decoder correctness;
//! - logical-equivalence correctness;
//! - deterministic execution;
//! - resource limits;
//! - memory safety;
//! - cancellation;
//! - streaming;
//! - partitioning;
//! - distributed execution;
//! - checkpoint integrity;
//! - numerical safety;
//! - cache correctness;
//! - capability isolation;
//! - QPU fail-closed behavior;
//! - security against malicious workloads;
//! - scalability regressions;
//! - permanent regression protection.
//!
//! # QPU contract
//!
//! QPU tests in this suite must not require physical quantum hardware.
//!
//! They verify the software boundary:
//!
//! ```text
//! QPU requested
//!      |
//!      v
//! capability check
//!      |
//!      +---- denied ----> fail closed
//!      |
//!      v
//! authorized execution boundary
//! ```
//!
//! Mathematical verification remains hardware-independent.
//!
//! A test must never silently connect to a physical QPU, submit a circuit,
//! consume credentials or depend on external network availability.

#![allow(clippy::assertions_on_constants)]

/* -------------------------------------------------------------------------- */
/* Focused test modules                                                       */
/* -------------------------------------------------------------------------- */

mod decoder_tests;
mod determinism_tests;
mod fault_injection_tests;
mod fuzz_tests;
mod mathematical_verification;
mod property_tests;
mod regression_tests;
mod resource_tests;
mod scalability_tests;
mod security_tests;
mod surface_code_tests;
mod threshold_tests;

/* -------------------------------------------------------------------------- */
/* Registry-level mathematical contract tests                                */
/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod mathematical_contract_tests {
    use std::panic::{
        catch_unwind,
        AssertUnwindSafe,
    };

    use crate::quantum::error_correction::{
        capabilities,
        self_check,
        supported_execution_environments,
        ExecutionEnvironment,
        Pauli,
        QpuAccess,
        SurfaceCode,
        QEC_ARCHITECTURE,
        QEC_API_VERSION,
        QEC_SUBSYSTEM_NAME,
    };

    fn no_panic<T, F>(operation: F) -> T
    where
        F: FnOnce() -> T,
    {
        catch_unwind(
            AssertUnwindSafe(operation)
        )
        .expect(
            "QEC public mathematical boundary must not panic"
        )
    }

    #[test]
    fn subsystem_metadata_is_present() {
        assert!(
            !QEC_SUBSYSTEM_NAME.is_empty()
        );

        assert!(
            !QEC_API_VERSION.is_empty()
        );

        assert!(
            !QEC_ARCHITECTURE.is_empty()
        );
    }

    #[test]
    fn identity_has_trivial_syndrome() {
        assert!(
            self_check().is_ok(),
            "identity/trivial-syndrome invariant failed"
        );
    }

    #[test]
    fn pauli_commutation_axioms_hold() {
        assert!(
            Pauli::X.anticommutes_with(Pauli::Z)
        );

        assert!(
            Pauli::Z.anticommutes_with(Pauli::X)
        );

        assert!(
            Pauli::X.anticommutes_with(Pauli::Y)
        );

        assert!(
            Pauli::Y.anticommutes_with(Pauli::X)
        );

        assert!(
            Pauli::Y.anticommutes_with(Pauli::Z)
        );

        assert!(
            Pauli::Z.anticommutes_with(Pauli::Y)
        );

        assert!(
            !Pauli::X.anticommutes_with(Pauli::X)
        );

        assert!(
            !Pauli::Y.anticommutes_with(Pauli::Y)
        );

        assert!(
            !Pauli::Z.anticommutes_with(Pauli::Z)
        );

        assert!(
            !Pauli::I.anticommutes_with(Pauli::X)
        );
    }

    #[test]
    fn surface_code_fixtures_pass_validation() {
        for distance in [3usize, 5usize] {
            let code = no_panic(|| {
                SurfaceCode::new(distance)
            })
            .expect(
                "production surface-code fixture \
                 must construct"
            );

            let validation = no_panic(|| {
                code.validate()
            });

            assert!(
                validation.is_ok(),
                "distance-{distance} surface code failed \
                 mathematical validation: {validation:?}"
            );

            let logical = no_panic(|| {
                code.validate_logical_operators()
            });

            assert!(
                logical.is_ok(),
                "distance-{distance} logical-operator \
                 invariant failed: {logical:?}"
            );
        }
    }

    #[test]
    fn declared_surface_code_distance_is_preserved() {
        for distance in [3usize, 5usize] {
            let code =
                SurfaceCode::new(distance)
                    .expect(
                        "surface-code fixture must construct"
                    );

            assert_eq!(
                code.distance(),
                distance
            );

            assert!(
                code.distance() > 0
            );
        }
    }

    #[test]
    fn mathematical_verification_is_repeatable() {
        let first =
            SurfaceCode::new(3)
                .expect(
                    "distance-3 code must construct"
                );

        let second =
            SurfaceCode::new(3)
                .expect(
                    "distance-3 code must construct"
                );

        assert_eq!(
            first.distance(),
            second.distance()
        );

        assert_eq!(
            first.num_data_qubits(),
            second.num_data_qubits()
        );

        assert_eq!(
            first.num_stabilizers(),
            second.num_stabilizers()
        );

        assert_eq!(
            first.validate(),
            second.validate()
        );

        assert_eq!(
            first.validate_logical_operators(),
            second.validate_logical_operators()
        );
    }

    #[test]
    fn malformed_surface_code_is_rejected() {
        let result =
            no_panic(|| SurfaceCode::new(2));

        assert!(
            result.is_err(),
            "invalid/unsupported code construction \
             must fail before mathematical verification"
        );
    }

    #[test]
    fn qpu_is_explicitly represented() {
        let environments =
            supported_execution_environments();

        assert!(
            environments.contains(
                &ExecutionEnvironment::Qpu
            )
        );

        assert!(
            ExecutionEnvironment::Qpu.is_qpu()
        );

        assert!(
            !ExecutionEnvironment::Qpu.is_classical()
        );
    }

    #[test]
    fn qpu_capability_is_explicit() {
        let caps =
            capabilities();

        assert!(
            caps.qpu_backend,
            "QPU must be represented by the backend capability inventory"
        );

        assert!(
            caps.supports_execution(
                ExecutionEnvironment::Qpu
            )
        );
    }

    #[test]
    fn qpu_access_fails_closed() {
        assert!(
            !QpuAccess::Denied.is_authorized()
        );

        assert!(
            !QpuAccess::RequiresCapability.is_authorized()
        );

        assert!(
            QpuAccess::Authorized.is_authorized()
        );
    }

    #[test]
    fn classical_execution_never_implies_qpu_authorization() {
        let classical =
            [
                ExecutionEnvironment::Cpu,
                ExecutionEnvironment::ParallelCpu,
                ExecutionEnvironment::Gpu,
                ExecutionEnvironment::Accelerator,
            ];

        for environment in classical {
            assert!(
                !environment.is_qpu(),
                "{environment:?} must not be treated as QPU execution"
            );
        }

        assert!(
            !QpuAccess::Denied.is_authorized()
        );

        assert!(
            !QpuAccess::RequiresCapability.is_authorized()
        );
    }

    #[test]
    fn mathematical_verification_requires_no_physical_qpu() {
        let code =
            SurfaceCode::new(3)
                .expect(
                    "distance-3 code must construct"
                );

        assert!(
            code.validate().is_ok()
        );

        assert!(
            code.validate_logical_operators().is_ok()
        );

        assert!(
            self_check().is_ok()
        );
    }

    #[test]
    fn all_declared_execution_environments_have_capability_entries() {
        let caps =
            capabilities();

        for environment
            in supported_execution_environments()
        {
            assert!(
                caps.supports_execution(
                    *environment
                ),
                "execution environment {environment:?} \
                 has no capability entry"
            );
        }
    }
}

/* -------------------------------------------------------------------------- */
/* Mathematical/QPU suite inventory tests                                    */
/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod suite_contract_tests {
    use super::*;

    #[test]
    fn mathematical_verification_suite_is_registered() {
        // The declaration:
        //
        //     mod mathematical_verification;
        //
        // is intentional. This test exists as a documentation-level guard
        // that mathematical verification is a first-class suite rather than
        // an inline duplicate.
    }

    #[test]
    fn qec_test_suite_is_offline_by_contract() {
        // The QEC mathematical and control-plane tests must not require:
        //
        // - network access;
        // - credentials;
        // - physical QPU hardware;
        // - GPU availability;
        // - distributed workers.
        //
        // Hardware-backed integration tests belong behind an explicit
        // integration/backend boundary and must never become mandatory CI
        // requirements for mathematical verification.
    }

    #[test]
    fn qpu_tests_are_control_plane_tests_until_hardware_adapter_exists() {
        // QPUAccess and ExecutionEnvironment::Qpu verify the safety boundary.
        //
        // This prevents the test suite from falsely claiming that a physical
        // QPU execution pipeline exists merely because QPU is represented in
        // the architecture.
        assert!(
            !crate::quantum::error_correction::QpuAccess::Denied
                .is_authorized()
        );
    }
}