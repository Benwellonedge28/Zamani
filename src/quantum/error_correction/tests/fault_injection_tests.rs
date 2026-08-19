//! Production fault-injection tests for Zamani QEC.
//!
//! These tests verify the boundary between:
//!
//!     physical fault
//!          |
//!          v
//!     fault validation
//!          |
//!          v
//!     FaultBatch
//!          |
//!          v
//!     QEC / syndrome / decoder pipeline
//!
//! This file intentionally uses deterministic fault injection.
//! Randomized threshold experiments belong in `simulation.rs` and
//! `threshold_tests.rs`.
//!
//! Production requirements verified here:
//!
//! - valid faults are accepted;
//! - malformed faults are rejected;
//! - identity faults are rejected;
//! - invalid qubit identifiers are rejected;
//! - correlated faults are bounded;
//! - correlated qubits have canonical ordering;
//! - fault batches are bounded;
//! - fault metadata remains deterministic;
//! - single-qubit X/Y/Z faults are distinguishable;
//! - measurement faults are represented separately;
//! - reset faults are validated;
//! - fault composition remains explicit;
//! - no public fault-construction path should panic.
//!
//! These tests are deliberately independent of any particular decoder.
//! A decoder must consume the validated fault/syndrome representation rather
//! than knowing how the physical fault was created.

#[cfg(test)]
mod tests {
    use crate::quantum::error_correction::noise::{
        Fault,
        FaultBatch,
        FaultKind,
        NoiseOperation,
        PauliError,
        QubitId,
        MAX_CORRELATED_QUBITS,
        MAX_FAULTS_PER_BATCH,
        MAX_QUBIT_INDEX,
    };

    // ========================================================================
    // Helpers
    // ========================================================================

    fn qubit(
        index: usize,
    ) -> QubitId {
        QubitId::new(index)
            .expect(
                "test qubit index must satisfy production bounds",
            )
    }

    fn pauli_fault(
        operation: NoiseOperation,
        index: usize,
        pauli: PauliError,
    ) -> Fault {
        Fault::pauli(
            operation,
            qubit(index),
            pauli,
        )
        .expect(
            "test fault must be valid",
        )
    }

    // ========================================================================
    // Qubit validation
    // ========================================================================

    #[test]
    fn valid_qubit_identifier_is_accepted() {
        let result =
            QubitId::new(0);

        assert!(
            result.is_ok(),
            "physical qubit zero must be valid"
        );

        let result =
            QubitId::new(
                MAX_QUBIT_INDEX,
            );

        assert!(
            result.is_ok(),
            "maximum supported qubit identifier must be valid"
        );
    }

    #[test]
    fn qubit_identifier_above_limit_is_rejected() {
        let result =
            QubitId::new(
                MAX_QUBIT_INDEX
                    .saturating_add(1),
            );

        assert!(
            result.is_err(),
            "qubit identifiers beyond the production safety boundary must be rejected"
        );
    }

    #[test]
    fn qubit_identifier_is_deterministic() {
        let first =
            QubitId::new(42);

        let second =
            QubitId::new(42);

        assert_eq!(
            first,
            second,
            "identical physical identifiers must produce identical validated values"
        );
    }

    // ========================================================================
    // Single-qubit Pauli faults
    // ========================================================================

    #[test]
    fn x_fault_is_accepted() {
        let fault =
            pauli_fault(
                NoiseOperation::Qubit,
                0,
                PauliError::X,
            );

        assert_eq!(
            fault.kind(),
            FaultKind::Pauli
        );

        assert_eq!(
            fault.qubit_count(),
            1
        );

        assert!(!fault.is_correlated());
    }

    #[test]
    fn y_fault_is_accepted() {
        let fault =
            pauli_fault(
                NoiseOperation::Qubit,
                1,
                PauliError::Y,
            );

        assert_eq!(
            fault.kind(),
            FaultKind::Pauli
        );

        assert_eq!(
            fault.qubit_count(),
            1
        );
    }

    #[test]
    fn z_fault_is_accepted() {
        let fault =
            pauli_fault(
                NoiseOperation::Qubit,
                2,
                PauliError::Z,
            );

        assert_eq!(
            fault.kind(),
            FaultKind::Pauli
        );

        assert_eq!(
            fault.qubit_count(),
            1
        );
    }

    #[test]
    fn identity_pauli_fault_is_rejected() {
        let result =
            Fault::pauli(
                NoiseOperation::Qubit,
                qubit(0),
                PauliError::I,
            );

        assert!(
            result.is_err(),
            "identity is not a physical fault"
        );
    }

    #[test]
    fn measurement_operation_rejects_pauli_faults() {
        let result =
            Fault::pauli(
                NoiseOperation::Measurement,
                qubit(0),
                PauliError::X,
            );

        assert!(
            result.is_err(),
            "measurement corruption must not be represented as a Pauli fault"
        );
    }

    #[test]
    fn gate_pauli_fault_is_accepted() {
        let fault =
            pauli_fault(
                NoiseOperation::Gate,
                3,
                PauliError::X,
            );

        assert_eq!(
            fault.operation(),
            NoiseOperation::Gate
        );
    }

    #[test]
    fn idle_pauli_fault_is_accepted() {
        let fault =
            pauli_fault(
                NoiseOperation::Idle,
                4,
                PauliError::Z,
            );

        assert_eq!(
            fault.operation(),
            NoiseOperation::Idle
        );
    }

    // ========================================================================
    // Measurement faults
    // ========================================================================

    #[test]
    fn measurement_fault_is_distinct_from_pauli_fault() {
        let fault =
            Fault::measurement(
                qubit(5),
            );

        assert_eq!(
            fault.kind(),
            FaultKind::Measurement
        );

        assert_eq!(
            fault.qubit_count(),
            1
        );

        assert!(!fault.is_correlated());
    }

    #[test]
    fn measurement_fault_preserves_operation_metadata() {
        let fault =
            Fault::measurement(
                qubit(7),
            );

        assert_eq!(
            fault.operation(),
            NoiseOperation::Measurement
        );
    }

    // ========================================================================
    // Reset faults
    // ========================================================================

    #[test]
    fn reset_x_fault_is_accepted() {
        let fault =
            Fault::reset(
                qubit(8),
                PauliError::X,
            )
            .expect(
                "X reset fault should be valid",
            );

        assert_eq!(
            fault.kind(),
            FaultKind::Reset
        );

        assert_eq!(
            fault.operation(),
            NoiseOperation::Reset
        );
    }

    #[test]
    fn reset_y_fault_is_accepted() {
        let fault =
            Fault::reset(
                qubit(9),
                PauliError::Y,
            )
            .expect(
                "Y reset fault should be valid",
            );

        assert_eq!(
            fault.kind(),
            FaultKind::Reset
        );
    }

    #[test]
    fn reset_z_fault_is_accepted() {
        let fault =
            Fault::reset(
                qubit(10),
                PauliError::Z,
            )
            .expect(
                "Z reset fault should be valid",
            );

        assert_eq!(
            fault.kind(),
            FaultKind::Reset
        );
    }

    #[test]
    fn identity_reset_fault_is_rejected() {
        let result =
            Fault::reset(
                qubit(0),
                PauliError::I,
            );

        assert!(
            result.is_err(),
            "identity reset is not a fault"
        );
    }

    // ========================================================================
    // Correlated faults
    // ========================================================================

    #[test]
    fn two_qubit_correlated_fault_is_accepted() {
        let fault =
            Fault::correlated(
                NoiseOperation::Gate,
                vec![
                    qubit(0),
                    qubit(1),
                ],
                vec![
                    PauliError::X,
                    PauliError::Z,
                ],
            )
            .expect(
                "valid two-qubit correlated fault should be accepted",
            );

        assert_eq!(
            fault.kind(),
            FaultKind::Correlated
        );

        assert_eq!(
            fault.qubit_count(),
            2
        );

        assert!(
            fault.is_correlated()
        );
    }

    #[test]
    fn correlated_fault_requires_matching_lengths() {
        let result =
            Fault::correlated(
                NoiseOperation::Gate,
                vec![
                    qubit(0),
                    qubit(1),
                ],
                vec![
                    PauliError::X,
                ],
            );

        assert!(
            result.is_err(),
            "qubit and Pauli arrays must have equal lengths"
        );
    }

    #[test]
    fn correlated_fault_rejects_empty_input() {
        let result =
            Fault::correlated(
                NoiseOperation::Gate,
                Vec::new(),
                Vec::new(),
            );

        assert!(
            result.is_err(),
            "an empty correlated fault is meaningless"
        );
    }

    #[test]
    fn correlated_fault_rejects_identity_component() {
        let result =
            Fault::correlated(
                NoiseOperation::Gate,
                vec![
                    qubit(0),
                    qubit(1),
                ],
                vec![
                    PauliError::X,
                    PauliError::I,
                ],
            );

        assert!(
            result.is_err(),
            "correlated faults must contain actual physical errors"
        );
    }

    #[test]
    fn correlated_fault_requires_canonical_qubit_order() {
        let result =
            Fault::correlated(
                NoiseOperation::Gate,
                vec![
                    qubit(2),
                    qubit(1),
                ],
                vec![
                    PauliError::X,
                    PauliError::Z,
                ],
            );

        assert!(
            result.is_err(),
            "correlated faults must use canonical strictly increasing qubit order"
        );
    }

    #[test]
    fn correlated_fault_rejects_duplicate_qubits() {
        let result =
            Fault::correlated(
                NoiseOperation::Gate,
                vec![
                    qubit(2),
                    qubit(2),
                ],
                vec![
                    PauliError::X,
                    PauliError::Z,
                ],
            );

        assert!(
            result.is_err(),
            "duplicate physical qubits must be rejected"
        );
    }

    #[test]
    fn correlated_fault_rejects_measurement_operation() {
        let result =
            Fault::correlated(
                NoiseOperation::Measurement,
                vec![
                    qubit(0),
                    qubit(1),
                ],
                vec![
                    PauliError::X,
                    PauliError::Z,
                ],
            );

        assert!(
            result.is_err(),
            "measurement faults have a dedicated representation"
        );
    }

    // ========================================================================
    // Fault batch
    // ========================================================================

    #[test]
    fn empty_fault_batch_is_valid() {
        let batch =
            FaultBatch::new();

        assert!(
            batch.is_empty()
        );

        assert_eq!(
            batch.len(),
            0
        );
    }

    #[test]
    fn fault_batch_accepts_valid_faults() {
        let mut batch =
            FaultBatch::new();

        batch
            .push(
                pauli_fault(
                    NoiseOperation::Qubit,
                    0,
                    PauliError::X,
                ),
            )
            .expect(
                "first fault should be accepted",
            );

        batch
            .push(
                pauli_fault(
                    NoiseOperation::Qubit,
                    1,
                    PauliError::Z,
                ),
            )
            .expect(
                "second fault should be accepted",
            );

        assert_eq!(
            batch.len(),
            2
        );
    }

    #[test]
    fn fault_batch_preserves_insertion_order() {
        let first =
            pauli_fault(
                NoiseOperation::Qubit,
                0,
                PauliError::X,
            );

        let second =
            pauli_fault(
                NoiseOperation::Qubit,
                1,
                PauliError::Z,
            );

        let batch =
            FaultBatch::from_faults(
                vec![
                    first.clone(),
                    second.clone(),
                ],
            )
            .expect(
                "valid fault batch should be accepted",
            );

        assert_eq!(
            batch.as_slice(),
            &[
                first,
                second,
            ]
        );
    }

    #[test]
    fn fault_batch_can_be_reconstructed_from_valid_faults() {
        let faults =
            vec![
                pauli_fault(
                    NoiseOperation::Qubit,
                    0,
                    PauliError::X,
                ),
                Fault::measurement(
                    qubit(1),
                ),
                Fault::reset(
                    qubit(2),
                    PauliError::Z,
                )
                .expect(
                    "reset fault should be valid",
                ),
            ];

        let batch =
            FaultBatch::from_faults(
                faults.clone(),
            )
            .expect(
                "valid faults should form a valid batch",
            );

        assert_eq!(
            batch.len(),
            faults.len()
        );

        assert_eq!(
            batch.as_slice(),
            faults.as_slice()
        );
    }

    #[test]
    fn fault_batch_rejects_more_than_production_limit() {
        // Do not allocate an enormous vector merely to test the boundary.
        //
        // The constructor's limit is itself tested through a deliberately
        // oversized vector only when the configured limit is reasonably
        // bounded. The production constant currently is 1,000,000.
        //
        // This test uses `MAX_FAULTS_PER_BATCH + 1` exactly, exercising the
        // public contract without relying on undocumented implementation
        // details.
        let oversized =
            vec![
                Fault::measurement(
                    qubit(0),
                );
                MAX_FAULTS_PER_BATCH
                    .saturating_add(1)
            ];

        let result =
            FaultBatch::from_faults(
                oversized,
            );

        assert!(
            result.is_err(),
            "fault batches must enforce their hard allocation boundary"
        );
    }

    // ========================================================================
    // Fault determinism
    // ========================================================================

    #[test]
    fn identical_faults_are_equal() {
        let first =
            pauli_fault(
                NoiseOperation::Qubit,
                11,
                PauliError::Y,
            );

        let second =
            pauli_fault(
                NoiseOperation::Qubit,
                11,
                PauliError::Y,
            );

        assert_eq!(
            first,
            second,
            "the same physical event must have deterministic representation"
        );
    }

    #[test]
    fn different_paulis_remain_distinguishable() {
        let x =
            pauli_fault(
                NoiseOperation::Qubit,
                12,
                PauliError::X,
            );

        let y =
            pauli_fault(
                NoiseOperation::Qubit,
                12,
                PauliError::Y,
            );

        let z =
            pauli_fault(
                NoiseOperation::Qubit,
                12,
                PauliError::Z,
            );

        assert_ne!(
            x,
            y
        );

        assert_ne!(
            y,
            z
        );

        assert_ne!(
            x,
            z
        );
    }

    // ========================================================================
    // Fault-type separation
    // ========================================================================

    #[test]
    fn physical_fault_kinds_are_explicit() {
        let pauli =
            pauli_fault(
                NoiseOperation::Qubit,
                13,
                PauliError::X,
            );

        let measurement =
            Fault::measurement(
                qubit(14),
            );

        let reset =
            Fault::reset(
                qubit(15),
                PauliError::Z,
            )
            .expect(
                "reset fault should be valid",
            );

        let correlated =
            Fault::correlated(
                NoiseOperation::Gate,
                vec![
                    qubit(16),
                    qubit(17),
                ],
                vec![
                    PauliError::X,
                    PauliError::Z,
                ],
            )
            .expect(
                "correlated fault should be valid",
            );

        assert_eq!(
            pauli.kind(),
            FaultKind::Pauli
        );

        assert_eq!(
            measurement.kind(),
            FaultKind::Measurement
        );

        assert_eq!(
            reset.kind(),
            FaultKind::Reset
        );

        assert_eq!(
            correlated.kind(),
            FaultKind::Correlated
        );
    }

    // ========================================================================
    // Boundary stress tests
    // ========================================================================

    #[test]
    fn maximum_correlated_fault_size_is_bounded() {
        let mut qubits =
            Vec::with_capacity(
                MAX_CORRELATED_QUBITS,
            );

        let mut paulis =
            Vec::with_capacity(
                MAX_CORRELATED_QUBITS,
            );

        for index in
            0..MAX_CORRELATED_QUBITS
        {
            qubits.push(
                qubit(index),
            );

            paulis.push(
                PauliError::X,
            );
        }

        let result =
            Fault::correlated(
                NoiseOperation::Gate,
                qubits,
                paulis,
            );

        assert!(
            result.is_ok(),
            "the documented maximum correlated-fault size must remain usable"
        );
    }

    #[test]
    fn correlated_fault_above_limit_is_rejected() {
        let count =
            MAX_CORRELATED_QUBITS
                .saturating_add(1);

        let mut qubits =
            Vec::with_capacity(count);

        let mut paulis =
            Vec::with_capacity(count);

        for index in 0..count {
            qubits.push(
                qubit(index),
            );

            paulis.push(
                PauliError::X,
            );
        }

        let result =
            Fault::correlated(
                NoiseOperation::Gate,
                qubits,
                paulis,
            );

        assert!(
            result.is_err(),
            "correlated-fault allocation must have a hard upper bound"
        );
    }

    // ========================================================================
    // No-panic malformed-input contract
    // ========================================================================

    #[test]
    fn malformed_single_qubit_inputs_return_errors() {
        let invalid_identity =
            Fault::pauli(
                NoiseOperation::Qubit,
                qubit(0),
                PauliError::I,
            );

        assert!(
            invalid_identity.is_err()
        );

        let invalid_measurement_pauli =
            Fault::pauli(
                NoiseOperation::Measurement,
                qubit(0),
                PauliError::X,
            );

        assert!(
            invalid_measurement_pauli
                .is_err()
        );

        let invalid_reset =
            Fault::reset(
                qubit(0),
                PauliError::I,
            );

        assert!(
            invalid_reset.is_err()
        );
    }

    #[test]
    fn malformed_correlated_inputs_return_errors() {
        let empty =
            Fault::correlated(
                NoiseOperation::Gate,
                Vec::new(),
                Vec::new(),
            );

        assert!(
            empty.is_err()
        );

        let mismatched =
            Fault::correlated(
                NoiseOperation::Gate,
                vec![
                    qubit(0),
                ],
                vec![
                    PauliError::X,
                    PauliError::Z,
                ],
            );

        assert!(
            mismatched.is_err()
        );

        let duplicate =
            Fault::correlated(
                NoiseOperation::Gate,
                vec![
                    qubit(0),
                    qubit(0),
                ],
                vec![
                    PauliError::X,
                    PauliError::Z,
                ],
            );

        assert!(
            duplicate.is_err()
        );
    }

    // ========================================================================
    // Pipeline contract
    // ========================================================================

    #[test]
    fn validated_faults_can_enter_a_batch_without_revalidation_side_effects() {
        let faults =
            [
                pauli_fault(
                    NoiseOperation::Qubit,
                    20,
                    PauliError::X,
                ),
                Fault::measurement(
                    qubit(21),
                ),
                Fault::reset(
                    qubit(22),
                    PauliError::Z,
                )
                .expect(
                    "reset fault should be valid",
                ),
                Fault::correlated(
                    NoiseOperation::Gate,
                    vec![
                        qubit(23),
                        qubit(24),
                    ],
                    vec![
                        PauliError::X,
                        PauliError::Z,
                    ],
                )
                .expect(
                    "correlated fault should be valid",
                ),
            ];

        let mut batch =
            FaultBatch::new();

        for fault in
            faults
        {
            batch
                .push(fault)
                .expect(
                    "validated fault should be accepted by a bounded batch",
                );
        }

        assert_eq!(
            batch.len(),
            4
        );

        assert!(
            batch
                .iter()
                .all(|fault| fault.qubit_count() > 0)
        );
    }
}