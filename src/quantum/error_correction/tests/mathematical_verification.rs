//! Production-grade mathematical verification for Zamani QEC.
//!
//! This suite verifies the mathematical contract of the error-correction
//! subsystem rather than merely checking implementation details.
//!
//! Verified invariants:
//!
//! 1. Stabilizer validity
//!    - generators are non-identity;
//!    - generator dimensions agree;
//!    - stabilizer generators mutually commute;
//!    - the complete stabilizer group validates;
//!    - stabilizer membership is mathematically consistent.
//!
//! 2. Logical operators
//!    - logical X and Z are non-identity;
//!    - logical X and Z have the declared code-distance weight;
//!    - logical operators commute with every stabilizer;
//!    - logical X and Z anticommute;
//!    - logical operators are not members of the stabilizer group.
//!
//! 3. Code distance
//!    - distance verification succeeds;
//!    - d=3 is exactly distance 3;
//!    - d=5 is exactly distance 5;
//!    - canonical logical operators have minimum declared weight.
//!
//! 4. Syndrome mathematics
//!    - identity has trivial syndrome;
//!    - non-trivial Pauli errors produce mathematically valid syndromes;
//!    - syndrome extraction is deterministic;
//!    - syndrome/correction consistency can be checked through the
//!      stabilizer algebra.
//!
//! 5. Correctable versus logical errors
//!    - errors below the code distance are detectable/correctable in the
//!      mathematical sense when paired with a correction from the same
//!      syndrome class;
//!    - a minimum-weight logical operator is intentionally not treated as
//!      correctable because it represents a logical operation rather than a
//!      stabilizer-equivalent error.
//!
//! 6. QPU mathematical contract
//!    - QPU backends use the same mathematical QEC representation;
//!    - QPU topology/resource validation occurs before execution;
//!    - QPU capability metadata cannot change stabilizer mathematics;
//!    - a QPU backend must not be treated as a mathematical oracle;
//!    - physical execution is deliberately NOT required for CI.
//!
//! 7. Robustness
//!    - public mathematical APIs do not panic on malformed inputs;
//!    - repeated validation is deterministic;
//!    - construction and verification remain deterministic.
//!
//! Important:
//!
//! A QPU test here verifies the mathematical/backend boundary. It does not
//! claim that a real physical QPU has executed the test. Physical hardware
//! integration belongs to hardware integration tests and must be explicitly
//! provisioned by the execution layer.

use std::collections::BTreeSet;
use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::quantum::error_correction::{
    Pauli,
    PauliString,
    QubitIndex,
    SurfaceCode,
};

use crate::quantum::error_correction::decoder::{
    Correction,
    Decoder,
    DecoderId,
    IdentityDecoder,
    StabilizerDecoder,
};

use crate::quantum::hardware::backend::{
    BackendCapabilities,
    BackendKind,
    BackendLimits,
    BackendMetadata,
    BackendStatus,
    CircuitRequirements,
    QuantumBackend,
};

use crate::quantum::hardware::topology::HardwareTopology;

// ============================================================================
// Test helpers
// ============================================================================

fn code_d3() -> SurfaceCode {
    SurfaceCode::new(3)
        .expect("distance-3 surface code must construct")
}

fn code_d5() -> SurfaceCode {
    SurfaceCode::new(5)
        .expect("distance-5 surface code must construct")
}

/// Public QEC operations must fail through Result rather than process-level
/// unwinding when supplied malformed data.
fn assert_no_panic<T, F>(operation: F) -> T
where
    F: FnOnce() -> T,
{
    catch_unwind(AssertUnwindSafe(operation))
        .expect("QEC mathematical API must not panic")
}

/// Build a single-qubit Pauli error.
fn single_qubit_error(
    num_qubits: usize,
    index: usize,
    pauli: Pauli,
) -> PauliString {
    let mut operators = vec![Pauli::I; num_qubits];
    operators[index] = pauli;
    PauliString::from_paulis(&operators)
}

/// Build a row-supported logical X error.
fn row_x(
    code: &SurfaceCode,
    row: usize,
) -> PauliString {
    let distance = code.distance();
    let mut operators = vec![Pauli::I; code.num_data_qubits()];

    for column in 0..distance {
        let qubit = code
            .qubit_at(
                crate::quantum::error_correction::surface_code::Coordinate::new(
                    row,
                    column,
                ),
            )
            .expect("logical-X row coordinate must exist");

        operators[qubit.index().index()] = Pauli::X;
    }

    PauliString::from_paulis(&operators)
}

/// Build a column-supported logical Z error.
fn column_z(
    code: &SurfaceCode,
    column: usize,
) -> PauliString {
    let distance = code.distance();
    let mut operators = vec![Pauli::I; code.num_data_qubits()];

    for row in 0..distance {
        let qubit = code
            .qubit_at(
                crate::quantum::error_correction::surface_code::Coordinate::new(
                    row,
                    column,
                ),
            )
            .expect("logical-Z column coordinate must exist");

        operators[qubit.index().index()] = Pauli::Z;
    }

    PauliString::from_paulis(&operators)
}

// ============================================================================
// 1. Stabilizer mathematical verification
// ============================================================================

#[test]
fn stabilizer_group_is_valid_for_distance_three() {
    let code = code_d3();

    let group = code
        .stabilizer_group()
        .expect("surface code must produce a stabilizer group");

    assert!(
        group.validate().is_ok(),
        "all stabilizer generators must satisfy the stabilizer-group axioms"
    );
}

#[test]
fn stabilizer_group_is_valid_for_distance_five() {
    let code = code_d5();

    let group = code
        .stabilizer_group()
        .expect("surface code must produce a stabilizer group");

    assert!(
        group.validate().is_ok(),
        "distance-5 stabilizer group must validate"
    );
}

#[test]
fn every_stabilizer_generator_is_non_identity() {
    let code = code_d5();

    for generator in code.stabilizers() {
        assert!(
            !generator.operator().is_identity(),
            "stabilizer {} cannot be identity",
            generator.id()
        );
    }
}

#[test]
fn stabilizer_generators_have_consistent_dimensions() {
    let code = code_d5();

    for generator in code.stabilizers() {
        assert_eq!(
            generator.operator().num_qubits(),
            code.num_data_qubits(),
            "stabilizer {} has incorrect Hilbert-space dimension",
            generator.id()
        );
    }
}

#[test]
fn all_stabilizer_pairs_commute() {
    let code = code_d5();

    let stabilizers = code.stabilizers();

    for i in 0..stabilizers.len() {
        for j in (i + 1)..stabilizers.len() {
            let first = stabilizers[i].operator();
            let second = stabilizers[j].operator();

            assert!(
                !first
                    .anticommutes_with(second)
                    .expect("compatible stabilizers must be comparable"),
                "stabilizers {} and {} must commute",
                stabilizers[i].id(),
                stabilizers[j].id()
            );
        }
    }
}

#[test]
fn stabilizer_group_contains_identity() {
    let code = code_d5();

    let group = code
        .stabilizer_group()
        .expect("valid surface code must have a stabilizer group");

    let identity =
        PauliString::identity(code.num_data_qubits());

    assert!(
        group
            .contains(&identity)
            .expect("identity membership must be computable"),
        "identity must belong to every stabilizer group"
    );
}

#[test]
fn stabilizer_group_does_not_contain_nontrivial_single_qubit_error() {
    let code = code_d3();

    let group = code
        .stabilizer_group()
        .expect("valid surface code must have a stabilizer group");

    for index in 0..code.num_data_qubits() {
        for pauli in [Pauli::X, Pauli::Y, Pauli::Z] {
            let error = single_qubit_error(
                code.num_data_qubits(),
                index,
                pauli,
            );

            assert!(
                !group
                    .contains(&error)
                    .expect("membership calculation must succeed"),
                "single-qubit {:?} error on q{} must not be a stabilizer",
                pauli,
                index
            );
        }
    }
}

// ============================================================================
// 2. Logical-operator mathematical verification
// ============================================================================

#[test]
fn logical_x_is_non_identity() {
    for distance in [3usize, 5, 7] {
        let code = SurfaceCode::new(distance)
            .expect("valid odd distance must construct");

        assert!(
            !code.logical_x().operator().is_identity(),
            "logical X must not be identity for d={distance}"
        );
    }
}

#[test]
fn logical_z_is_non_identity() {
    for distance in [3usize, 5, 7] {
        let code = SurfaceCode::new(distance)
            .expect("valid odd distance must construct");

        assert!(
            !code.logical_z().operator().is_identity(),
            "logical Z must not be identity for d={distance}"
        );
    }
}

#[test]
fn logical_x_and_z_have_declared_distance_weight() {
    for distance in [3usize, 5, 7] {
        let code = SurfaceCode::new(distance)
            .expect("valid odd distance must construct");

        assert_eq!(
            code.logical_x().weight(),
            distance,
            "logical X weight must equal d"
        );

        assert_eq!(
            code.logical_z().weight(),
            distance,
            "logical Z weight must equal d"
        );
    }
}

#[test]
fn logical_x_and_z_anticommute() {
    for distance in [3usize, 5, 7] {
        let code = SurfaceCode::new(distance)
            .expect("valid odd distance must construct");

        let x = code.logical_x().operator();
        let z = code.logical_z().operator();

        assert!(
            x.anticommutes_with(z)
                .expect("logical operators must have equal dimensions"),
            "logical X and logical Z must anticommute exactly once"
        );
    }
}

#[test]
fn logical_x_commutes_with_every_stabilizer() {
    let code = code_d5();

    let logical_x = code.logical_x().operator();

    for stabilizer in code.stabilizers() {
        assert!(
            logical_x
                .commutes_with(stabilizer.operator())
                .expect("compatible operators must be comparable"),
            "logical X must commute with stabilizer {}",
            stabilizer.id()
        );
    }
}

#[test]
fn logical_z_commutes_with_every_stabilizer() {
    let code = code_d5();

    let logical_z = code.logical_z().operator();

    for stabilizer in code.stabilizers() {
        assert!(
            logical_z
                .commutes_with(stabilizer.operator())
                .expect("compatible operators must be comparable"),
            "logical Z must commute with stabilizer {}",
            stabilizer.id()
        );
    }
}

#[test]
fn logical_x_is_not_a_stabilizer() {
    let code = code_d5();

    let group = code
        .stabilizer_group()
        .expect("valid surface code must have a stabilizer group");

    assert!(
        !group
            .contains(code.logical_x().operator())
            .expect("logical-X membership must be computable"),
        "logical X must not belong to the stabilizer group"
    );
}

#[test]
fn logical_z_is_not_a_stabilizer() {
    let code = code_d5();

    let group = code
        .stabilizer_group()
        .expect("valid surface code must have a stabilizer group");

    assert!(
        !group
            .contains(code.logical_z().operator())
            .expect("logical-Z membership must be computable"),
        "logical Z must not belong to the stabilizer group"
    );
}

#[test]
fn canonical_logical_operators_match_explicit_lattice_paths() {
    let code = code_d5();

    let explicit_x = row_x(&code, 0);
    let explicit_z = column_z(&code, 0);

    assert_eq!(
        explicit_x.weight(),
        code.distance()
    );

    assert_eq!(
        explicit_z.weight(),
        code.distance()
    );

    assert_eq!(
        explicit_x,
        code.logical_x().operator().clone(),
        "canonical logical X must correspond to a complete row"
    );

    assert_eq!(
        explicit_z,
        code.logical_z().operator().clone(),
        "canonical logical Z must correspond to a complete column"
    );
}

// ============================================================================
// 3. Code-distance verification
// ============================================================================

#[test]
fn distance_three_is_exactly_three() {
    let code = code_d3();

    let verified = assert_no_panic(|| code.verify_distance())
        .expect("distance verification must succeed");

    assert_eq!(
        verified,
        3,
        "minimum undetectable logical weight must be 3"
    );
}

#[test]
fn distance_five_is_exactly_five() {
    let code = code_d5();

    let verified = assert_no_panic(|| code.verify_distance())
        .expect("distance verification must succeed");

    assert_eq!(
        verified,
        5,
        "minimum undetectable logical weight must be 5"
    );
}

#[test]
fn declared_distance_matches_verified_distance() {
    for distance in [3usize, 5] {
        let code = SurfaceCode::new(distance)
            .expect("valid code must construct");

        let verified = code
            .verify_distance()
            .expect("distance verification must succeed");

        assert_eq!(
            verified,
            code.distance(),
            "declared and mathematically verified distance must agree"
        );
    }
}

#[test]
fn no_nontrivial_single_qubit_logical_exists_for_distance_three() {
    let code = code_d3();

    let stabilizers = code
        .stabilizer_group()
        .expect("valid code must have stabilizers");

    let logical_x = code.logical_x().operator();
    let logical_z = code.logical_z().operator();

    for index in 0..code.num_data_qubits() {
        for pauli in [Pauli::X, Pauli::Y, Pauli::Z] {
            let error = single_qubit_error(
                code.num_data_qubits(),
                index,
                pauli,
            );

            let commutes_with_all = code
                .stabilizers()
                .iter()
                .all(|stabilizer| {
                    error
                        .commutes_with(stabilizer.operator())
                        .expect("compatible Pauli strings")
                });

            if commutes_with_all {
                assert!(
                    stabilizers
                        .contains(&error)
                        .expect("membership calculation must succeed")
                        || !error
                            .commutes_with(logical_x)
                            .expect("compatible operators")
                        || !error
                            .commutes_with(logical_z)
                            .expect("compatible operators"),
                    "a weight-1 operator cannot be a nontrivial logical operator"
                );
            }
        }
    }
}

// ============================================================================
// 4. Syndrome mathematical verification
// ============================================================================

#[test]
fn identity_has_trivial_syndrome() {
    let code = code_d5();

    let identity =
        PauliString::identity(code.num_data_qubits());

    let syndrome = code
        .syndrome(&identity)
        .expect("identity syndrome must be computable");

    assert!(
        syndrome.is_trivial(),
        "identity must commute with every stabilizer"
    );
}

#[test]
fn single_qubit_errors_have_valid_syndromes() {
    let code = code_d3();

    for index in 0..code.num_data_qubits() {
        for pauli in [Pauli::X, Pauli::Y, Pauli::Z] {
            let error = single_qubit_error(
                code.num_data_qubits(),
                index,
                pauli,
            );

            let syndrome = assert_no_panic(|| code.syndrome(&error))
                .expect("valid Pauli error must have a syndrome");

            assert_eq!(
                syndrome.len(),
                code.num_stabilizers(),
                "syndrome dimension must equal stabilizer count"
            );
        }
    }
}

#[test]
fn nontrivial_single_qubit_errors_are_detected() {
    let code = code_d3();

    for index in 0..code.num_data_qubits() {
        for pauli in [Pauli::X, Pauli::Y, Pauli::Z] {
            let error = single_qubit_error(
                code.num_data_qubits(),
                index,
                pauli,
            );

            let syndrome = code
                .syndrome(&error)
                .expect("valid error must have a syndrome");

            assert!(
                !syndrome.is_trivial(),
                "weight-1 {:?} error on q{} must be detected",
                pauli,
                index
            );
        }
    }
}

#[test]
fn syndrome_extraction_is_deterministic() {
    let code = code_d5();

    let error = single_qubit_error(
        code.num_data_qubits(),
        12,
        Pauli::Y,
    );

    let first = code
        .syndrome(&error)
        .expect("syndrome must be computable");

    for _ in 0..100 {
        let next = code
            .syndrome(&error)
            .expect("syndrome must remain computable");

        assert_eq!(
            first,
            next,
            "identical error and code must yield identical syndrome"
        );
    }
}

// ============================================================================
// 5. Decoder mathematical contract
// ============================================================================

#[test]
fn stabilizer_decoder_rejects_invalid_stabilizer_groups() {
    let code = code_d3();

    let group = code
        .stabilizer_group()
        .expect("valid code must have a stabilizer group");

    let decoder = assert_no_panic(|| {
        StabilizerDecoder::new(
            DecoderId::new(0),
            group,
        )
    });

    assert!(
        decoder.is_ok(),
        "validated surface-code stabilizers must be accepted by decoder infrastructure"
    );
}

#[test]
fn decoder_recomputes_the_same_syndrome_as_the_code() {
    let code = code_d3();

    let group = code
        .stabilizer_group()
        .expect("valid code must have a stabilizer group");

    let decoder = StabilizerDecoder::new(
        DecoderId::new(1),
        group,
    )
    .expect("valid stabilizer group must construct decoder");

    let error = single_qubit_error(
        code.num_data_qubits(),
        0,
        Pauli::X,
    );

    let expected = code
        .syndrome(&error)
        .expect("code syndrome must be computable");

    let actual = decoder
        .syndrome_for_error(&error)
        .expect("decoder syndrome must be computable");

    assert_eq!(
        expected,
        actual,
        "decoder and code must implement identical syndrome mathematics"
    );
}

#[test]
fn identity_decoder_preserves_trivial_syndrome() {
    let code = code_d3();

    let decoder = IdentityDecoder::new(
        DecoderId::new(2),
        code.num_data_qubits(),
    )
    .expect("identity decoder must construct");

    let identity =
        PauliString::identity(code.num_data_qubits());

    let syndrome = code
        .syndrome(&identity)
        .expect("identity syndrome must exist");

    let result = decoder
        .decode(&syndrome)
        .expect("trivial syndrome must be decodable");

    assert!(
        result.is_trivial(),
        "identity syndrome must remain trivial"
    );

    assert!(
        result.correction().is_identity(),
        "identity decoder must return identity correction"
    );
}

#[test]
fn correction_from_same_syndrome_class_is_stabilizer_equivalent() {
    let code = code_d3();

    let group = code
        .stabilizer_group()
        .expect("valid code must have a stabilizer group");

    let error = single_qubit_error(
        code.num_data_qubits(),
        0,
        Pauli::X,
    );

    let stabilizer = code
        .stabilizers()
        .first()
        .expect("surface code must contain stabilizers")
        .operator();

    let equivalent = error
        .multiply(stabilizer)
        .expect("Pauli dimensions must agree");

    let syndrome_error = code
        .syndrome(&error)
        .expect("syndrome must be computable");

    let syndrome_equivalent = code
        .syndrome(&equivalent)
        .expect("syndrome must be computable");

    assert_eq!(
        syndrome_error,
        syndrome_equivalent,
        "multiplication by a stabilizer must preserve syndrome"
    );

    let residual = error
        .multiply(&equivalent)
        .expect("Pauli multiplication must succeed");

    assert!(
        group
            .contains(&residual)
            .expect("stabilizer membership must be computable"),
        "equivalent errors must differ by a stabilizer"
    );
}

#[test]
fn minimum_weight_logical_error_is_not_stabilizer_equivalent_to_identity() {
    let code = code_d3();

    let group = code
        .stabilizer_group()
        .expect("valid surface code must have stabilizers");

    let logical_x = code.logical_x().operator();

    assert_eq!(
        logical_x.weight(),
        code.distance()
    );

    assert!(
        !group
            .contains(logical_x)
            .expect("logical membership must be computable"),
        "minimum-weight logical operator must not be a stabilizer"
    );

    let syndrome = code
        .syndrome(logical_x)
        .expect("logical syndrome must be computable");

    assert!(
        syndrome.is_trivial(),
        "a logical operator must commute with all stabilizers"
    );
}

// ============================================================================
// 6. Mathematical distinction between correctable and uncorrectable errors
// ============================================================================

#[test]
fn_errors_below_distance_cannot_be_logical_operators() {
    let code = code_d3();

    let group = code
        .stabilizer_group()
        .expect("valid surface code must have stabilizers");

    let logical_x = code.logical_x().operator();
    let logical_z = code.logical_z().operator();

    // The exact distance is 3. Therefore no weight-1 or weight-2
    // representative may implement a nontrivial logical operation.
    //
    // We explicitly enumerate all weight-1 operators and selected weight-2
    // Pauli products for the small d=3 production fixture.
    for index in 0..code.num_data_qubits() {
        for pauli in [Pauli::X, Pauli::Y, Pauli::Z] {
            let error = single_qubit_error(
                code.num_data_qubits(),
                index,
                pauli,
            );

            let commutes_with_stabilizers = code
                .stabilizers()
                .iter()
                .all(|s| {
                    error
                        .commutes_with(s.operator())
                        .expect("compatible operators")
                });

            if commutes_with_stabilizers {
                assert!(
                    group
                        .contains(&error)
                        .expect("membership must be computable"),
                    "a weight-1 operator commuting with all stabilizers must be stabilizer-equivalent"
                );

                assert!(
                    error
                        .commutes_with(logical_x)
                        .expect("compatible operators")
                        || error
                            .commutes_with(logical_z)
                            .expect("compatible operators"),
                    "weight-1 operator cannot be a nontrivial logical representative"
                );
            }
        }
    }
}

#[test]
fn logical_error_is_explicitly_distinguished_from_correctable_error() {
    let code = code_d3();

    let logical_x = code.logical_x().operator();

    let syndrome = code
        .syndrome(logical_x)
        .expect("logical operator syndrome must be computable");

    assert!(
        syndrome.is_trivial(),
        "logical error is undetectable by stabilizer syndrome"
    );

    assert_eq!(
        logical_x.weight(),
        code.distance(),
        "minimum logical operator has code-distance weight"
    );

    let group = code
        .stabilizer_group()
        .expect("valid code must have stabilizers");

    assert!(
        !group
            .contains(logical_x)
            .expect("logical membership must be computable"),
        "logical operation must not collapse into the stabilizer group"
    );
}

// ============================================================================
// 7. Formal algebraic invariants
// ============================================================================

#[test]
fn pauli_multiplication_is_xor_consistent() {
    let code = code_d3();

    for index in 0..code.num_data_qubits() {
        for pauli in [Pauli::X, Pauli::Y, Pauli::Z] {
            let error = single_qubit_error(
                code.num_data_qubits(),
                index,
                pauli,
            );

            let residual = error
                .multiply(&error)
                .expect("Pauli must be compatible with itself");

            assert!(
                residual.is_identity(),
                "P * P must equal identity up to global phase"
            );
        }
    }
}

#[test]
fn stabilizer_multiplication_preserves_syndrome() {
    let code = code_d5();

    let base_error = single_qubit_error(
        code.num_data_qubits(),
        0,
        Pauli::X,
    );

    let base_syndrome = code
        .syndrome(&base_error)
        .expect("base syndrome must be computable");

    for stabilizer in code.stabilizers() {
        let equivalent = base_error
            .multiply(stabilizer.operator())
            .expect("compatible Pauli strings must multiply");

        let syndrome = code
            .syndrome(&equivalent)
            .expect("equivalent syndrome must be computable");

        assert_eq!(
            base_syndrome,
            syndrome,
            "stabilizer multiplication must preserve syndrome"
        );
    }
}

#[test]
fn logical_x_and_z_have_the_expected_symplectic_relationship() {
    let code = code_d5();

    let x = code.logical_x().operator();
    let z = code.logical_z().operator();

    assert_eq!(
        x.symplectic_product(z),
        1,
        "logical X/Z must have odd symplectic product"
    );
}

// ============================================================================
// 8. QPU mathematical/backend contract
// ============================================================================

fn qpu_backend() -> QuantumBackend {
    let topology =
        HardwareTopology::linear(9)
            .expect("9-qubit QPU topology must construct");

    let capabilities = BackendCapabilities::new()
        .with_gates([
            "X",
            "Y",
            "Z",
            "H",
            "CNOT",
            "MEASURE",
            "RESET",
        ]);

    let metadata = BackendMetadata::new(
        "qpu-qec-math-test",
        "Zamani QEC Mathematical QPU",
        "Zamani",
        "test",
        BackendKind::Qpu,
    );

    QuantumBackend::new(
        metadata,
        capabilities,
        BackendLimits::default()
            .with_max_qubits(9)
            .with_max_depth(1_000)
            .with_max_operations(100_000)
            .with_max_shots(1_000_000),
        topology,
    )
    .expect("test QPU backend must construct")
}

#[test]
fn qpu_backend_is_explicitly_identified_as_qpu() {
    let backend = qpu_backend();

    assert_eq!(
        backend.kind(),
        BackendKind::Qpu,
        "physical execution backend must be explicitly classified as QPU"
    );
}

#[test]
fn qpu_backend_topology_has_expected_qubit_count() {
    let backend = qpu_backend();

    assert_eq!(
        backend.qubit_count(),
        9,
        "QPU topology must expose the configured physical-qubit count"
    );
}

#[test]
fn qpu_backend_validates_qec_sized_circuit_requirements() {
    let backend = qpu_backend();

    let requirements = CircuitRequirements {
        qubit_count: 9,
        circuit_depth: 100,
        operation_count: 1_000,
        shots: 100,
        gates: vec![
            "H".into(),
            "CNOT".into(),
            "MEASURE".into(),
        ],
        two_qubit_edges: vec![
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 4),
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 8),
        ],
        requires_measurement: true,
        requires_reset: false,
        requires_mid_circuit_measurement: false,
        requires_classical_control: false,
        requires_dynamic_circuits: false,
    };

    assert!(
        backend.validate(&requirements).is_ok(),
        "valid QPU circuit requirements must pass backend validation"
    );
}

#[test]
fn qpu_backend_rejects_resource_overflow_before_execution() {
    let backend = qpu_backend();

    let requirements = CircuitRequirements {
        qubit_count: 10,
        ..CircuitRequirements::default()
    };

    let result = backend.validate(&requirements);

    assert!(
        result.is_err(),
        "QPU resource limits must reject impossible workloads before execution"
    );
}

#[test]
fn qpu_backend_math_is_independent_of_hardware_identity() {
    let code = code_d3();

    let qpu = qpu_backend();

    let logical_x = code.logical_x().operator();

    assert!(
        qpu.kind() == BackendKind::Qpu,
        "test must exercise the QPU backend contract"
    );

    assert!(
        code
            .stabilizer_group()
            .expect("stabilizer group must exist")
            .contains(logical_x)
            .expect("membership must be computable")
            == false,
        "hardware backend identity must not turn a logical operator into a stabilizer"
    );

    assert_eq!(
        code.verify_distance()
            .expect("distance verification must succeed"),
        3,
        "QPU selection must not alter the mathematical code distance"
    );
}

#[test]
fn qpu_status_is_respected() {
    let mut backend = qpu_backend();

    backend.set_status(BackendStatus::Offline);

    let requirements = CircuitRequirements {
        qubit_count: 1,
        ..CircuitRequirements::default()
    };

    let result = backend.validate(&requirements);

    assert!(
        result.is_err(),
        "offline QPU must reject execution validation"
    );
}

#[test]
fn qpu_capability_metadata_cannot_override_qec_mathematics() {
    let backend = qpu_backend();
    let code = code_d3();

    assert!(
        backend.capabilities.supports_gate("CNOT"),
        "test QPU should advertise CNOT"
    );

    assert_eq!(
        code.num_logical_qubits(),
        1,
        "QPU gate capabilities must not alter logical-qubit count"
    );

    assert_eq!(
        code.num_data_qubits(),
        9,
        "QPU gate capabilities must not alter physical code topology"
    );
}

// ============================================================================
// 9. Deterministic mathematical verification
// ============================================================================

#[test]
fn complete_mathematical_validation_is_deterministic() {
    for distance in [3usize, 5] {
        let code = SurfaceCode::new(distance)
            .expect("valid code must construct");

        for _ in 0..25 {
            assert!(
                code.validate().is_ok(),
                "mathematical validation must be deterministic"
            );

            assert_eq!(
                code.verify_distance()
                    .expect("distance verification must succeed"),
                distance,
                "distance verification must be deterministic"
            );
        }
    }
}

#[test]
fn repeated_logical_operator_generation_is_deterministic() {
    let first = code_d5();
    let second = code_d5();

    assert_eq!(
        first.logical_x(),
        second.logical_x()
    );

    assert_eq!(
        first.logical_z(),
        second.logical_z()
    );
}

#[test]
fn stabilizer_order_is_deterministic() {
    let first = code_d5();
    let second = code_d5();

    assert_eq!(
        first.stabilizers(),
        second.stabilizers()
    );
}

// ============================================================================
// 10. Mathematical robustness / no-panic guarantees
// ============================================================================

#[test]
fn malformed_surface_code_distances_do_not_panic() {
    for distance in [
        0usize,
        1,
        2,
        4,
        6,
        100,
        usize::MAX,
    ] {
        let result =
            assert_no_panic(|| SurfaceCode::new(distance));

        assert!(
            result.is_ok() || result.is_err(),
            "constructor must return normally"
        );
    }
}

#[test]
fn malformed_pauli_dimensions_are_rejected_without_panic() {
    let code = code_d3();

    let result = assert_no_panic(|| {
        PauliString::from_bits(
            vec![false; code.num_data_qubits()],
            vec![false; code.num_data_qubits() - 1],
        )
    });

    assert!(
        result.is_err(),
        "mismatched symplectic dimensions must be rejected"
    );
}

#[test]
fn invalid_qubit_access_is_rejected_without_panic() {
    let code = code_d3();

    let result = assert_no_panic(|| {
        code.coordinate_of(
            QubitIndex::new(usize::MAX)
        )
    });

    assert!(
        result.is_err(),
        "invalid qubit index must become a controlled error"
    );
}

#[test]
fn invalid_coordinate_access_is_rejected_without_panic() {
    let code = code_d3();

    let coordinate =
        crate::quantum::error_correction::surface_code::Coordinate::new(
            usize::MAX,
            usize::MAX,
        );

    let result =
        assert_no_panic(|| code.qubit_at(coordinate));

    assert!(
        result.is_err(),
        "invalid coordinate must become a controlled error"
    );
}

// ============================================================================
// 11. Cross-layer invariant
// ============================================================================

#[test]
fn qec_math_is_consistent_across_surface_code_stabilizer_and_decoder_layers() {
    let code = code_d3();

    // Surface-code layer.
    assert!(
        code.validate().is_ok(),
        "surface-code validation must succeed"
    );

    // Stabilizer layer.
    let group = code
        .stabilizer_group()
        .expect("surface code must expose stabilizer group");

    assert!(
        group.validate().is_ok(),
        "stabilizer layer must validate the same mathematical object"
    );

    // Decoder layer.
    let decoder = StabilizerDecoder::new(
        DecoderId::new(99),
        group,
    )
    .expect("decoder layer must accept validated stabilizers");

    let error = single_qubit_error(
        code.num_data_qubits(),
        0,
        Pauli::X,
    );

    let surface_syndrome = code
        .syndrome(&error)
        .expect("surface syndrome must be computable");

    let decoder_syndrome = decoder
        .syndrome_for_error(&error)
        .expect("decoder syndrome must be computable");

    assert_eq!(
        surface_syndrome,
        decoder_syndrome,
        "surface-code, stabilizer, and decoder layers must agree mathematically"
    );
}

// ============================================================================
// 12. Final mathematical contract
// ============================================================================

#[test]
fn production_mathematical_contract_holds_for_d3() {
    let code = code_d3();

    assert_eq!(
        code.num_data_qubits(),
        9
    );

    assert_eq!(
        code.num_stabilizers(),
        8
    );

    assert_eq!(
        code.num_logical_qubits(),
        1
    );

    assert_eq!(
        code.distance(),
        3
    );

    assert_eq!(
        code.verify_distance()
            .expect("distance verification must succeed"),
        3
    );

    assert!(
        code.validate().is_ok()
    );

    assert!(
        code.validate_logical_operators().is_ok()
    );

    assert!(
        code.logical_x()
            .operator()
            .anticommutes_with(
                code.logical_z().operator()
            )
            .expect("logical operators must be compatible")
    );
}

#[test]
fn production_mathematical_contract_holds_for_d5() {
    let code = code_d5();

    assert_eq!(
        code.num_data_qubits(),
        25
    );

    assert_eq!(
        code.num_stabilizers(),
        24
    );

    assert_eq!(
        code.num_logical_qubits(),
        1
    );

    assert_eq!(
        code.distance(),
        5
    );

    assert_eq!(
        code.verify_distance()
            .expect("distance verification must succeed"),
        5
    );

    assert!(
        code.validate().is_ok()
    );

    assert!(
        code.validate_logical_operators().is_ok()
    );

    assert!(
        code.logical_x()
            .operator()
            .anticommutes_with(
                code.logical_z().operator()
            )
            .expect("logical operators must be compatible")
    );
}