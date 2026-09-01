//! Zamani Quantum IR — Cross-Module Integration Test Suite.
//!
//! Path:
//!
//!     src/quantum/ir/tests/mod.rs
//!
//! # Purpose
//!
//! This module is the integration boundary for the canonical Zamani Quantum
//! IR. It verifies contracts BETWEEN IR modules rather than testing private
//! implementation details.
//!
//! The canonical architecture is:
//!
//! ```text
//!                         Zamani source
//!                              |
//!                              v
//!                          frontend
//!                              |
//!                              v
//!                     +------------------+
//!                     |   quantum::ir    |
//!                     |  semantic "WHAT" |
//!                     +--------+---------+
//!                              |
//!          +-------------------+-------------------+
//!          |                   |                   |
//!          v                   v                   v
//!     optimization          routing            scheduling
//!          |                   |                   |
//!          +-------------------+-------------------+
//!                              |
//!                              v
//!                           hardware
//!                              |
//!                              v
//!                           backend
//!                              |
//!                              v
//!                          execution
//! ```
//!
//! The IR tests MUST preserve this boundary.
//!
//! They do NOT test:
//!
//! - hardware topology;
//! - routing algorithms;
//! - scheduling algorithms;
//! - backend execution;
//! - QPU communication;
//! - calibration;
//! - pulse synthesis;
//! - simulator state;
//! - QEC decoder implementations;
//! - optimization algorithms;
//! - frontend parsing.
//!
//! Those are downstream responsibilities.
//!
//! # Production properties verified here
//!
//! The suite verifies the properties required for a production semantic IR:
//!
//! - canonical logical-qubit identity;
//! - separation of logical and physical identity;
//! - no dependency on the legacy `qubits` implementation path;
//! - explicit resource/security policies;
//! - no architectural fixed quantum-machine size;
//! - fallible resource-bounded construction;
//! - atomic mutation;
//! - namespace validation;
//! - operation validation;
//! - gate arity validation;
//! - duplicate-qubit rejection;
//! - parameter validity;
//! - NaN/Infinity rejection;
//! - classical namespace handling;
//! - deterministic analysis;
//! - deterministic operation ordering;
//! - version identity;
//! - circuit identity;
//! - validation of completed circuits;
//! - explicit distinction between resource policy and architectural capability;
//! - sparse large-namespace behaviour;
//! - safe arithmetic boundaries;
//! - compatibility of the public IR API.
//!
//! # Scalability principle
//!
//! Test values such as 1, 2, 8, 64, 128, or 1_000 are TEST POINTS only.
//!
//! They MUST NOT be interpreted as:
//!
//! ```text
//! maximum supported qubits
//! ```
//!
//! The semantic IR has no such architectural maximum.
//!
//! Actual limits are supplied by `QuantumIrLimits`, the host environment,
//! compiler resources, and the selected target.
//!
//! # Canonical qubit path
//!
//! New code in this test suite deliberately uses:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! The legacy `qubits` alias is tested only as a compatibility mechanism.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! No external test dependencies are required.
//!
//! # Integration contract
//!
//! This module is intentionally limited to public IR APIs. If an implementation
//! changes internally while preserving those APIs and their documented
//! semantics, these tests should continue to pass.
//!
//! If a test fails because another module was internally reorganized, the
//! correct response is to repair the public integration contract rather than
//! reaching into private implementation details.
//!
//! # Test organization
//!
//! This file intentionally starts with the stable cross-module contract.
//!
//! The suite may later be split into:
//!
//! ```text
//! tests/
//! ├── mod.rs
//! ├── invariants.rs
//! ├── scaling.rs
//! ├── determinism.rs
//! ├── serialization.rs
//! ├── hashing.rs
//! ├── dynamic.rs
//! ├── pulse.rs
//! ├── analog.rs
//! └── compatibility.rs
//! ```
//!
//! Such decomposition MUST NOT change the public semantic contracts tested
//! here.
//!
//! # Important integration note
//!
//! `tests.rs` was the previous flat integration-test module. This directory
//! module supersedes it.
//!
//! `src/quantum/ir/mod.rs` MUST register this directory as:
//!
//! ```rust
//! #[path = "tests/mod.rs"]
//! pub mod tests;
//! ```
//!
//! and the old `src/quantum/ir/tests.rs` module MUST no longer be registered
//! as `tests` at the same time.
//!
//! Keeping both:
//!
//! ```text
//! tests.rs
//! tests/mod.rs
//! ```
//!
//! under the same `mod tests;` declaration causes an ambiguous/duplicate
//! module layout and must be avoided.
//!
//! -----------------------------------------------------------------------------
//! No unsafe code.
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

#[cfg(test)]
mod integration {
    use super::super::analysis::{
        analyze,
        analyze_with_limits,
        basic_statistics,
        basic_statistics_with_limits,
    };
    use super::super::circuit::{CircuitError, QuantumCircuit};
    use super::super::gate::{Gate, GateKind};
    use super::super::identity::{CircuitId, IrVersion};
    use super::super::limits::QuantumIrLimits;
    use super::super::parameter::Parameter;

    // -------------------------------------------------------------------------
    // IMPORTANT:
    //
    // The canonical qubit implementation is `quantum::ir::qubit`.
    //
    // Never replace this with:
    //
    //     super::super::qubits
    //
    // in new code.
    // -------------------------------------------------------------------------

    use super::super::qubit::{
        PhysicalQubitId,
        QubitId,
        QubitRef,
    };

    // ========================================================================
    // Test construction helpers
    // ========================================================================

    /// Creates a valid non-parameterized gate.
    fn gate(kind: GateKind, qubit: usize) -> Gate {
        Gate::new(
            kind,
            vec![QubitId::new(qubit)],
            Vec::new(),
            None,
            None,
        )
        .expect("test gate must satisfy its gate contract")
    }

    /// Creates a valid two-qubit gate.
    fn two_qubit_gate(
        kind: GateKind,
        first: usize,
        second: usize,
    ) -> Gate {
        Gate::new(
            kind,
            vec![
                QubitId::new(first),
                QubitId::new(second),
            ],
            Vec::new(),
            None,
            None,
        )
        .expect("test two-qubit gate must satisfy its gate contract")
    }

    /// Creates a valid parameterized gate.
    fn parameterized_gate(
        kind: GateKind,
        qubit: usize,
        value: f64,
    ) -> Gate {
        let parameter = Parameter::constant(value)
            .expect("finite test parameter must be accepted");

        Gate::new(
            kind,
            vec![QubitId::new(qubit)],
            vec![parameter],
            None,
            None,
        )
        .expect("parameterized test gate must satisfy its gate contract")
    }

    /// Returns a deliberately small explicit policy.
    ///
    /// These numbers are test-policy values only. They are NOT Zamani
    /// architectural limits.
    fn small_limits() -> QuantumIrLimits {
        QuantumIrLimits::production()
            .with_max_qubits(8)
            .with_max_classical_bits(8)
            .with_max_operations(16)
            .with_max_operands(8)
            .with_max_parameters(8)
            .with_max_metadata_bytes(64)
            .with_max_depth(16)
            .with_max_measurements(4)
            .with_max_barriers(4)
            .with_max_validation_steps(1_000)
            .with_max_analysis_steps(1_000)
    }

    /// Creates a small valid circuit.
    fn small_circuit() -> QuantumCircuit {
        QuantumCircuit::try_new_with_limits(
            3,
            3,
            small_limits(),
        )
        .expect("small test circuit must be valid")
    }

    // ========================================================================
    // Policy invariants
    // ========================================================================

    #[test]
    fn production_limits_are_self_consistent() {
        QuantumIrLimits::production()
            .validate()
            .expect("production limits must be internally valid");
    }

    #[test]
    fn deny_all_policy_is_structurally_valid() {
        QuantumIrLimits::deny_all()
            .validate()
            .expect("deny-all policy must itself be a valid policy");
    }

    #[test]
    fn zero_resource_policy_is_representable() {
        let limits = QuantumIrLimits::production()
            .with_max_qubits(0)
            .with_max_classical_bits(0)
            .with_max_operations(0)
            .with_max_operands(0)
            .with_max_parameters(0)
            .with_max_metadata_bytes(0)
            .with_max_depth(0)
            .with_max_measurements(0)
            .with_max_barriers(0);

        limits
            .validate()
            .expect("zero resource capacities should be representable");
    }

    #[test]
    fn zero_validation_budget_is_rejected() {
        let limits = QuantumIrLimits::production()
            .with_max_validation_steps(0);

        assert!(
            limits.validate().is_err(),
            "validation cannot have a zero work budget"
        );
    }

    #[test]
    fn zero_analysis_budget_is_rejected() {
        let limits = QuantumIrLimits::production()
            .with_max_analysis_steps(0);

        assert!(
            limits.validate().is_err(),
            "analysis cannot have a zero work budget"
        );
    }

    // ========================================================================
    // Explicit resource-policy boundary
    // ========================================================================

    #[test]
    fn qubit_policy_is_not_a_language_level_limit() {
        let limits = small_limits()
            .with_max_qubits(2);

        let result = QuantumCircuit::try_new_with_limits(
            3,
            0,
            limits,
        );

        match result {
            Err(CircuitError::QubitLimitExceeded {
                requested,
                maximum,
            }) => {
                assert_eq!(requested, 3);
                assert_eq!(maximum, 2);
            }

            other => panic!(
                "expected explicit policy rejection, got {other:?}"
            ),
        }
    }

    #[test]
    fn classical_policy_is_not_a_language_level_limit() {
        let limits = small_limits()
            .with_max_classical_bits(2);

        let result = QuantumCircuit::try_new_with_limits(
            0,
            3,
            limits,
        );

        match result {
            Err(CircuitError::ClassicalBitLimitExceeded {
                requested,
                maximum,
            }) => {
                assert_eq!(requested, 3);
                assert_eq!(maximum, 2);
            }

            other => panic!(
                "expected explicit policy rejection, got {other:?}"
            ),
        }
    }

    #[test]
    fn operation_policy_is_enforced_without_partial_mutation() {
        let limits = small_limits()
            .with_max_operations(1);

        let mut circuit =
            QuantumCircuit::try_new_with_limits(
                1,
                0,
                limits,
            )
            .expect("circuit construction should succeed");

        circuit
            .push(gate(GateKind::X, 0))
            .expect("first operation should fit");

        let before_len = circuit.len();

        let result =
            circuit.push(gate(GateKind::X, 0));

        assert!(
            result.is_err(),
            "second operation must exceed policy"
        );

        assert_eq!(
            circuit.len(),
            before_len,
            "failed mutation must be atomic"
        );
    }

    // ========================================================================
    // Parameter invariants
    // ========================================================================

    #[test]
    fn finite_parameter_is_accepted() {
        let parameter =
            Parameter::constant(1.25)
                .expect("finite parameter must be accepted");

        assert!(
            parameter.is_constant(),
            "finite scalar should remain a constant parameter"
        );

        assert_eq!(
            parameter.as_constant(),
            Some(1.25)
        );
    }

    #[test]
    fn nan_parameter_is_rejected() {
        assert!(
            Parameter::constant(f64::NAN).is_err(),
            "NaN must never enter canonical IR"
        );
    }

    #[test]
    fn positive_infinity_parameter_is_rejected() {
        assert!(
            Parameter::constant(f64::INFINITY).is_err(),
            "positive infinity must never enter canonical IR"
        );
    }

    #[test]
    fn negative_infinity_parameter_is_rejected() {
        assert!(
            Parameter::constant(f64::NEG_INFINITY).is_err(),
            "negative infinity must never enter canonical IR"
        );
    }

    #[test]
    fn parameterized_gate_requires_expected_parameter_count() {
        let result = Gate::new(
            GateKind::RX,
            vec![QubitId::new(0)],
            Vec::new(),
            None,
            None,
        );

        assert!(
            result.is_err(),
            "RX without its parameter must be rejected"
        );
    }

    #[test]
    fn non_parameterized_gate_rejects_parameters() {
        let parameter =
            Parameter::constant(0.5)
                .expect("finite parameter must be valid");

        let result = Gate::new(
            GateKind::X,
            vec![QubitId::new(0)],
            vec![parameter],
            None,
            None,
        );

        assert!(
            result.is_err(),
            "X must not accept parameters"
        );
    }

    #[test]
    fn finite_parameterized_gate_is_valid() {
        let gate =
            parameterized_gate(
                GateKind::RX,
                0,
                0.5,
            );

        assert_eq!(
            gate.kind(),
            GateKind::RX
        );

        assert_eq!(
            gate.parameters().len(),
            1
        );
    }

    // ========================================================================
    // Gate structural invariants
    // ========================================================================

    #[test]
    fn single_qubit_gate_has_one_operand() {
        let operation =
            gate(GateKind::X, 0);

        assert_eq!(
            operation.qubits().len(),
            1
        );
    }

    #[test]
    fn two_qubit_gate_has_two_operands() {
        let operation =
            two_qubit_gate(
                GateKind::CX,
                0,
                1,
            );

        assert_eq!(
            operation.qubits().len(),
            2
        );
    }

    #[test]
    fn two_qubit_gate_rejects_one_operand() {
        let result = Gate::new(
            GateKind::CX,
            vec![QubitId::new(0)],
            Vec::new(),
            None,
            None,
        );

        assert!(
            result.is_err(),
            "CX must reject an invalid operand count"
        );
    }

    #[test]
    fn three_qubit_gate_rejects_two_operands() {
        let result = Gate::new(
            GateKind::CCX,
            vec![
                QubitId::new(0),
                QubitId::new(1),
            ],
            Vec::new(),
            None,
            None,
        );

        assert!(
            result.is_err(),
            "CCX must reject an invalid operand count"
        );
    }

    #[test]
    fn duplicate_logical_qubits_are_rejected() {
        let result = Gate::new(
            GateKind::CX,
            vec![
                QubitId::new(0),
                QubitId::new(0),
            ],
            Vec::new(),
            None,
            None,
        );

        assert!(
            result.is_err(),
            "an operation must not contain duplicate logical operands"
        );
    }

    #[test]
    fn empty_barrier_is_rejected() {
        let result = Gate::new(
            GateKind::Barrier,
            Vec::new(),
            Vec::new(),
            None,
            None,
        );

        assert!(
            result.is_err(),
            "a barrier requires at least one operand"
        );
    }

    #[test]
    fn reset_requires_exactly_one_qubit() {
        let result = Gate::new(
            GateKind::Reset,
            vec![
                QubitId::new(0),
                QubitId::new(1),
            ],
            Vec::new(),
            None,
            None,
        );

        assert!(
            result.is_err(),
            "reset must target exactly one logical qubit"
        );
    }

    // ========================================================================
    // Canonical qubit identity contract
    // ========================================================================

    #[test]
    fn canonical_logical_qubit_identity_is_stable() {
        let first = QubitId::new(7);
        let second = QubitId::new(7);

        assert_eq!(
            first,
            second,
            "equal logical identifiers must compare equal"
        );

        assert_eq!(
            first.index(),
            7
        );
    }

    #[test]
    fn logical_qubit_identity_is_distinct_from_physical_identity() {
        let logical =
            QubitId::new(7);

        let physical =
            PhysicalQubitId::new(7);

        let logical_reference =
            QubitRef::from(logical);

        let physical_reference =
            QubitRef::from(physical);

        assert!(
            logical_reference.is_logical()
        );

        assert!(
            physical_reference.is_physical()
        );

        assert_ne!(
            logical_reference,
            physical_reference,
            "logical and physical namespaces must never collapse"
        );
    }

    #[test]
    fn qubit_ids_have_checked_successors() {
        let id =
            QubitId::new(41);

        assert_eq!(
            id.checked_next(),
            Some(QubitId::new(42))
        );

        let physical =
            PhysicalQubitId::new(41);

        assert_eq!(
            physical.checked_next(),
            Some(PhysicalQubitId::new(42))
        );
    }

    // ========================================================================
    // Circuit namespace contract
    // ========================================================================

    #[test]
    fn circuit_construction_is_fallible() {
        let circuit =
            small_circuit();

        assert_eq!(
            circuit.num_qubits(),
            3
        );

        assert_eq!(
            circuit.num_classical_bits(),
            3
        );

        assert!(
            circuit.is_empty(),
            "new circuit should contain no operations"
        );
    }

    #[test]
    fn circuit_does_not_allocate_an_operation_for_each_qubit() {
        let circuit =
            QuantumCircuit::try_new_with_limits(
                8,
                8,
                small_limits(),
            )
            .expect("namespace declaration must succeed");

        assert_eq!(
            circuit.num_qubits(),
            8
        );

        assert_eq!(
            circuit.len(),
            0,
            "declared qubits must not imply materialized operations"
        );
    }

    #[test]
    fn sparse_namespace_can_be_used_without_dense_operation_storage() {
        let circuit =
            QuantumCircuit::try_new_with_limits(
                8,
                8,
                small_limits(),
            )
            .expect("namespace declaration must succeed");

        assert_eq!(
            circuit.num_qubits(),
            8
        );

        assert!(
            circuit.operations().is_empty()
        );
    }

    // ========================================================================
    // Version and identity contract
    // ========================================================================

    #[test]
    fn circuit_uses_current_ir_version() {
        let circuit =
            small_circuit();

        assert_eq!(
            circuit.version(),
            IrVersion::CURRENT
        );
    }

    #[test]
    fn circuit_identity_is_explicit_and_stable() {
        let id =
            CircuitId::new(42);

        let circuit =
            QuantumCircuit::with_identity(
                id,
                2,
                2,
                small_limits(),
            )
            .expect("explicit identity must be accepted");

        assert_eq!(
            circuit.id(),
            id
        );
    }

    #[test]
    fn circuit_identity_can_be_reassigned_without_rebuilding() {
        let mut circuit =
            small_circuit();

        let new_id =
            CircuitId::new(99);

        circuit.set_id(new_id);

        assert_eq!(
            circuit.id(),
            new_id
        );
    }

    #[test]
    fn current_ir_version_can_be_reapplied() {
        let mut circuit =
            small_circuit();

        circuit
            .set_version(IrVersion::CURRENT)
            .expect("current IR version must remain supported");

        assert_eq!(
            circuit.version(),
            IrVersion::CURRENT
        );
    }

    // ========================================================================
    // Mutation and ordering contract
    // ========================================================================

    #[test]
    fn push_preserves_operation_order() {
        let mut circuit =
            small_circuit();

        circuit
            .push(gate(GateKind::H, 0))
            .expect("H must be inserted");

        circuit
            .push(gate(GateKind::X, 0))
            .expect("X must be inserted");

        circuit
            .push(gate(GateKind::Z, 0))
            .expect("Z must be inserted");

        assert_eq!(
            circuit.len(),
            3
        );

        assert_eq!(
            circuit.operations()[0].kind(),
            GateKind::H
        );

        assert_eq!(
            circuit.operations()[1].kind(),
            GateKind::X
        );

        assert_eq!(
            circuit.operations()[2].kind(),
            GateKind::Z
        );
    }

    #[test]
    fn failed_operation_insertion_does_not_corrupt_order() {
        let mut circuit =
            QuantumCircuit::try_new_with_limits(
                1,
                0,
                small_limits().with_max_operations(1),
            )
            .expect("test circuit must be constructible");

        circuit
            .push(gate(GateKind::H, 0))
            .expect("first operation must succeed");

        let original_kind =
            circuit.operations()[0].kind();

        let result =
            circuit.push(gate(GateKind::X, 0));

        assert!(
            result.is_err()
        );

        assert_eq!(
            circuit.len(),
            1
        );

        assert_eq!(
            circuit.operations()[0].kind(),
            original_kind,
            "failed insertion must not mutate existing operations"
        );
    }

    // ========================================================================
    // Whole-circuit validation
    // ========================================================================

    #[test]
    fn valid_circuit_passes_canonical_validation() {
        let mut circuit =
            small_circuit();

        circuit
            .push(gate(GateKind::H, 0))
            .expect("H insertion must succeed");

        circuit
            .push(two_qubit_gate(
                GateKind::CX,
                0,
                1,
            ))
            .expect("CX insertion must succeed");

        super::super::validation::validate_circuit(
            &circuit,
        )
        .expect("valid circuit must pass canonical validation");
    }

    #[test]
    fn validation_accepts_explicit_policy() {
        let mut circuit =
            small_circuit();

        circuit
            .push(gate(GateKind::X, 0))
            .expect("X insertion must succeed");

        super::super::validation::validate_circuit_with_limits(
            &circuit,
            &small_limits(),
        )
        .expect("valid circuit must satisfy its explicit policy");
    }

    #[test]
    fn empty_circuit_can_be_valid_when_policy_allows_it() {
        let circuit =
            QuantumCircuit::try_new_with_limits(
                0,
                0,
                small_limits(),
            )
            .expect("empty namespace must be constructible");

        super::super::validation::validate_circuit(
            &circuit,
        )
        .expect("empty circuit is valid under the default production policy");
    }

    // ========================================================================
    // Analysis contract
    // ========================================================================

    #[test]
    fn analysis_accepts_valid_circuit() {
        let mut circuit =
            small_circuit();

        circuit
            .push(gate(GateKind::H, 0))
            .expect("H insertion must succeed");

        circuit
            .push(two_qubit_gate(
                GateKind::CX,
                0,
                1,
            ))
            .expect("CX insertion must succeed");

        let result =
            analyze(&circuit);

        assert!(
            result.is_ok(),
            "valid circuit must be analyzable"
        );
    }

    #[test]
    fn analysis_with_explicit_limits_is_bounded() {
        let mut circuit =
            small_circuit();

        circuit
            .push(gate(GateKind::X, 0))
            .expect("X insertion must succeed");

        let result =
            analyze_with_limits(
                &circuit,
                &small_limits(),
            );

        assert!(
            result.is_ok(),
            "analysis should respect a sufficient explicit work budget"
        );
    }

    #[test]
    fn basic_statistics_are_available_for_valid_circuit() {
        let mut circuit =
            small_circuit();

        circuit
            .push(gate(GateKind::H, 0))
            .expect("H insertion must succeed");

        let result =
            basic_statistics(&circuit);

        assert!(
            result.is_ok(),
            "basic statistics must work for a valid circuit"
        );
    }

    #[test]
    fn basic_statistics_with_limits_are_bounded() {
        let mut circuit =
            small_circuit();

        circuit
            .push(gate(GateKind::X, 0))
            .expect("X insertion must succeed");

        let result =
            basic_statistics_with_limits(
                &circuit,
                &small_limits(),
            );

        assert!(
            result.is_ok(),
            "bounded statistics must succeed under sufficient policy"
        );
    }

    // ========================================================================
    // Gate semantic classification contract
    // ========================================================================

    #[test]
    fn fixed_single_qubit_gate_is_unitary() {
        assert!(
            GateKind::X.is_unitary()
        );

        assert!(
            GateKind::H.is_unitary()
        );

        assert!(
            GateKind::Z.is_unitary()
        );
    }

    #[test]
    fn measurement_is_non_unitary() {
        assert!(
            !GateKind::Measure.is_unitary()
        );

        assert!(
            GateKind::Measure.is_measurement()
        );
    }

    #[test]
    fn reset_is_non_unitary() {
        assert!(
            !GateKind::Reset.is_unitary()
        );

        assert!(
            GateKind::Reset.is_reset()
        );
    }

    #[test]
    fn barrier_is_identified_without_being_hardware_specific() {
        assert!(
            GateKind::Barrier.is_barrier()
        );

        assert!(
            GateKind::Barrier.is_unitary(),
            "barrier is a semantic ordering marker, not a state-transforming
             non-unitary operation"
        );
    }

    // ========================================================================
    // Gate cardinality contract
    // ========================================================================

    #[test]
    fn standard_gate_cardinality_is_explicit() {
        assert_eq!(
            GateKind::X.operand_count().to_string(),
            "1"
        );

        assert_eq!(
            GateKind::CX.operand_count().to_string(),
            "2"
        );

        assert_eq!(
            GateKind::CCX.operand_count().to_string(),
            "3"
        );

        assert_eq!(
            GateKind::Barrier.operand_count().to_string(),
            "at least 1"
        );
    }

    #[test]
    fn standard_parameter_cardinality_is_explicit() {
        assert_eq!(
            GateKind::X.parameter_count(),
            0
        );

        assert_eq!(
            GateKind::RX.parameter_count(),
            1
        );

        assert_eq!(
            GateKind::U2.parameter_count(),
            2
        );

        assert_eq!(
            GateKind::U3.parameter_count(),
            3
        );
    }

    // ========================================================================
    // Namespace boundary validation
    // ========================================================================

    #[test]
    fn operation_outside_logical_namespace_is_rejected() {
        let circuit =
            QuantumCircuit::try_new_with_limits(
                1,
                0,
                small_limits(),
            )
            .expect("one-qubit circuit must be constructible");

        let operation =
            gate(GateKind::X, 1);

        let result =
            super::super::validation::validate_operation(
                &operation,
                circuit.num_qubits(),
                circuit.num_classical_bits(),
                &super::super::validation::ValidationConfig::production(),
            );

        assert!(
            result.is_err(),
            "operation referring to q1 must be rejected in a one-qubit namespace"
        );
    }

    #[test]
    fn operation_inside_logical_namespace_is_accepted() {
        let circuit =
            QuantumCircuit::try_new_with_limits(
                2,
                0,
                small_limits(),
            )
            .expect("two-qubit circuit must be constructible");

        let operation =
            gate(GateKind::X, 1);

        let result =
            super::super::validation::validate_operation(
                &operation,
                circuit.num_qubits(),
                circuit.num_classical_bits(),
                &super::super::validation::ValidationConfig::production(),
            );

        assert!(
            result.is_ok(),
            "q1 must be valid in a two-qubit namespace"
        );
    }

    // ========================================================================
    // Scaling invariants
    // ========================================================================

    #[test]
    fn test_sizes_are_policy_values_not_architectural_constants() {
        //
        // These are deliberately generated rather than encoded into IR logic.
        //
        // The important assertion is that the same API handles different
        // namespace sizes without changing the semantic representation.
        //
        for qubit_count in [0usize, 1, 2, 3, 8] {
            let limits =
                small_limits()
                    .with_max_qubits(qubit_count);

            let circuit =
                QuantumCircuit::try_new_with_limits(
                    qubit_count,
                    0,
                    limits,
                )
                .expect("test namespace must fit explicit test policy");

            assert_eq!(
                circuit.num_qubits(),
                qubit_count
            );

            assert!(
                circuit.operations().is_empty()
            );
        }
    }

    #[test]
    fn increasing_namespace_does_not_change_gate_semantics() {
        let one =
            gate(GateKind::X, 0);

        let many_namespace =
            QuantumCircuit::try_new_with_limits(
                8,
                0,
                small_limits(),
            )
            .expect("larger namespace must be constructible");

        let many =
            gate(GateKind::X, 0);

        assert_eq!(
            one.kind(),
            many.kind()
        );

        assert_eq!(
            one.qubits(),
            many.qubits()
        );

        assert_eq!(
            many_namespace.num_qubits(),
            8
        );
    }

    // ========================================================================
    // Determinism of observable public behaviour
    // ========================================================================

    #[test]
    fn identical_circuits_have_identical_public_operation_sequences() {
        let mut first =
            small_circuit();

        let mut second =
            small_circuit();

        for circuit in [&mut first, &mut second] {
            circuit
                .push(gate(GateKind::H, 0))
                .expect("H insertion must succeed");

            circuit
                .push(two_qubit_gate(
                    GateKind::CX,
                    0,
                    1,
                ))
                .expect("CX insertion must succeed");

            circuit
                .push(gate(GateKind::X, 1))
                .expect("X insertion must succeed");
        }

        assert_eq!(
            first.operations(),
            second.operations()
        );
    }

    #[test]
    fn repeated_analysis_is_stable() {
        let mut circuit =
            small_circuit();

        circuit
            .push(gate(GateKind::H, 0))
            .expect("H insertion must succeed");

        circuit
            .push(two_qubit_gate(
                GateKind::CX,
                0,
                1,
            ))
            .expect("CX insertion must succeed");

        let first =
            basic_statistics(&circuit)
                .expect("first analysis must succeed");

        let second =
            basic_statistics(&circuit)
                .expect("second analysis must succeed");

        assert_eq!(
            first,
            second,
            "repeated analysis of immutable IR must be deterministic"
        );
    }

    // ========================================================================
    // Compatibility contract
    // ========================================================================

    #[test]
    fn legacy_qubits_alias_resolves_to_canonical_qubit_module() {
        //
        // This test is deliberately written through the public compatibility
        // alias. It does NOT make `qubits` the canonical implementation.
        //
        // The assertion is semantic: the old path and new path must identify
        // the same logical-qubit type during the compatibility period.
        //
        let canonical =
            QubitId::new(12);

        let legacy =
            super::super::qubits::QubitId::new(12);

        assert_eq!(
            canonical,
            legacy,
            "legacy qubits alias must resolve to canonical QubitId"
        );
    }

    #[test]
    fn canonical_and_legacy_physical_ids_remain_identical_types() {
        let canonical =
            PhysicalQubitId::new(5);

        let legacy =
            super::super::qubits::PhysicalQubitId::new(5);

        assert_eq!(
            canonical,
            legacy,
            "legacy physical-qubit alias must not create a second identity type"
        );
    }

    // ========================================================================
    // No accidental hardware coupling
    // ========================================================================

    #[test]
    fn logical_gate_contains_no_physical_identity() {
        let operation =
            gate(GateKind::X, 0);

        //
        // This test intentionally verifies the public semantic surface rather
        // than inspecting private fields.
        //
        // A Gate exposes logical qubits, not a physical allocation.
        //
        assert_eq!(
            operation.qubits(),
            &[QubitId::new(0)]
        );
    }

    #[test]
    fn logical_gate_semantics_are_independent_of_machine_size() {
        let operation =
            gate(GateKind::X, 0);

        assert_eq!(
            operation.kind(),
            GateKind::X
        );

        assert_eq!(
            operation.qubits(),
            &[QubitId::new(0)]
        );

        //
        // No hardware target is consulted. The same semantic operation is
        // suitable for any compatible target that can execute X on its mapped
        // resource.
        //
    }

    // ========================================================================
    // Explicit anti-hardcoding regression checks
    // ========================================================================

    #[test]
    fn q64_is_not_special_to_the_semantic_gate_model() {
        let first =
            gate(GateKind::X, 0);

        let second =
            gate(GateKind::X, 64);

        assert_eq!(
            first.kind(),
            second.kind()
        );

        assert_eq!(
            first.parameters(),
            second.parameters()
        );

        assert_eq!(
            first.qubit_count(),
            second.qubit_count()
        );
    }

    #[test]
    fn q128_is_not_special_to_the_semantic_gate_model() {
        let operation =
            gate(GateKind::X, 128);

        assert_eq!(
            operation.qubits(),
            &[QubitId::new(128)]
        );
    }

    #[test]
    fn q4096_is_not_semantically_special() {
        //
        // This is intentionally a Gate-only test so it does not require a
        // 4097-qubit allocation. It proves that the semantic identity model
        // itself does not encode a 4096-qubit ceiling.
        //
        let operation =
            gate(GateKind::X, 4_096);

        assert_eq!(
            operation.qubits(),
            &[QubitId::new(4_096)]
        );
    }

    // ========================================================================
    // Final integration invariant
    // ========================================================================

    #[test]
    fn canonical_ir_pipeline_contract_is_preserved() {
        let mut circuit =
            small_circuit();

        circuit
            .push(gate(GateKind::H, 0))
            .expect("H insertion must succeed");

        circuit
            .push(two_qubit_gate(
                GateKind::CX,
                0,
                1,
            ))
            .expect("CX insertion must succeed");

        //
        // The integration pipeline tested here is intentionally limited to
        // semantic IR responsibilities:
        //
        //     construction
        //          |
        //          v
        //     canonical IR
        //          |
        //          +--> validation
        //          |
        //          +--> analysis
        //
        // Routing, scheduling, hardware and backend execution are deliberately
        // outside this test.
        //

        super::super::validation::validate_circuit(
            &circuit,
        )
        .expect("constructed IR must validate");

        let statistics =
            basic_statistics(&circuit)
                .expect("validated IR must be analyzable");

        //
        // The exact shape of the statistics object belongs to analysis.rs.
        // We therefore only require successful analysis here rather than
        // coupling the integration suite to implementation-specific counters.
        //
        let _ = statistics;
    }
}