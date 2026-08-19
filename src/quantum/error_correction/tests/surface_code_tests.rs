//! Production-grade tests for the Zamani surface-code implementation.
//!
//! This suite verifies the mathematical and structural contract of the
//! rotated planar surface code exposed by `surface_code.rs`.
//!
//! Coverage:
//!
//! - valid and invalid code distances;
//! - exact data-qubit topology;
//! - coordinate/index bijection;
//! - stabilizer count and support invariants;
//! - stabilizer weights and boundary classification;
//! - stabilizer commutation through full validation;
//! - logical X/Z construction;
//! - logical X/Z anticommutation;
//! - syndrome extraction;
//! - exact distance verification for small production fixtures;
//! - deterministic construction and validation;
//! - malformed-input handling;
//! - no-panic behaviour for public APIs.
//!
//! The tests intentionally exercise the public API rather than private
//! implementation details. This keeps the suite useful during refactoring
//! while preserving the mathematical contract of the code.

use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::quantum::error_correction::{
    Pauli,
    PauliString,
    QubitIndex,
    SurfaceCode,
    SurfaceCodeError,
};

use crate::quantum::error_correction::surface_code::{
    Boundary,
    Coordinate,
    StabilizerKind,
};

// ============================================================================
// Test helpers
// ============================================================================

/// Construct the smallest supported surface code.
fn code_d3() -> SurfaceCode {
    SurfaceCode::new(3)
        .expect("distance-3 surface code must be constructible")
}

/// Construct a distance-5 surface code.
fn code_d5() -> SurfaceCode {
    SurfaceCode::new(5)
        .expect("distance-5 surface code must be constructible")
}

/// Assert that a public operation never panics.
///
/// Surface-code parameters and syndrome data may eventually originate from
/// external hardware, files, simulations, or network-facing integrations.
/// Invalid input must therefore result in deterministic errors rather than
/// process-level unwinding.
fn assert_no_panic<T, F>(operation: F) -> T
where
    F: FnOnce() -> T,
{
    catch_unwind(AssertUnwindSafe(operation))
        .expect("surface-code API must not panic")
}

// ============================================================================
// Construction
// ============================================================================

#[test]
fn distance_three_constructs() {
    let code = SurfaceCode::new(3);

    assert!(
        code.is_ok(),
        "distance 3 is the minimum supported production code"
    );
}

#[test]
fn distance_five_constructs() {
    let code = SurfaceCode::new(5);

    assert!(
        code.is_ok(),
        "distance 5 must be supported"
    );
}

#[test]
fn odd_distances_construct() {
    for distance in [3usize, 5, 7, 9] {
        let result = assert_no_panic(|| SurfaceCode::new(distance));

        assert!(
            result.is_ok(),
            "odd distance {distance} must construct successfully"
        );
    }
}

#[test]
fn distance_below_three_is_rejected() {
    for distance in [0usize, 1, 2] {
        let result = assert_no_panic(|| SurfaceCode::new(distance));

        assert!(
            result.is_err(),
            "distance {distance} must be rejected"
        );
    }
}

#[test]
fn even_distance_is_rejected() {
    for distance in [4usize, 6, 8, 10] {
        let result = assert_no_panic(|| SurfaceCode::new(distance));

        assert!(
            result.is_err(),
            "even distance {distance} must be rejected"
        );
    }
}

#[test]
fn extreme_distance_is_rejected_or_handled_without_panic() {
    let result = assert_no_panic(|| SurfaceCode::new(usize::MAX));

    assert!(
        result.is_err(),
        "unrepresentable/extreme distance must not construct"
    );
}

#[test]
fn from_distance_matches_new() {
    for distance in [3usize, 5, 7] {
        let first = SurfaceCode::new(distance)
            .expect("new must succeed");

        let second = SurfaceCode::from_distance(distance)
            .expect("from_distance must succeed");

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
    }
}

// ============================================================================
// Code parameters
// ============================================================================

#[test]
fn distance_three_has_nine_data_qubits() {
    let code = code_d3();

    assert_eq!(
        code.num_data_qubits(),
        9
    );
}

#[test]
fn distance_five_has_twenty_five_data_qubits() {
    let code = code_d5();

    assert_eq!(
        code.num_data_qubits(),
        25
    );
}

#[test]
fn stabilizer_count_is_d_squared_minus_one() {
    for distance in [3usize, 5, 7] {
        let code = SurfaceCode::new(distance)
            .expect("valid code must construct");

        let expected =
            distance
                .checked_mul(distance)
                .and_then(|n| n.checked_sub(1))
                .expect("test distance must fit usize");

        assert_eq!(
            code.num_stabilizers(),
            expected,
            "stabilizer count invariant failed for distance {distance}"
        );
    }
}

#[test]
fn one_logical_qubit_is_encoded() {
    for distance in [3usize, 5, 7] {
        let code = SurfaceCode::new(distance)
            .expect("valid code must construct");

        assert_eq!(
            code.num_logical_qubits(),
            1
        );
    }
}

// ============================================================================
// Data-qubit topology
// ============================================================================

#[test]
fn data_qubit_indices_are_contiguous() {
    let code = code_d5();

    for (expected, qubit) in
        code.data_qubits().iter().enumerate()
    {
        assert_eq!(
            qubit.index(),
            QubitIndex::new(expected),
            "data-qubit indices must be contiguous"
        );
    }
}

#[test]
fn data_qubit_coordinates_are_unique() {
    let code = code_d5();

    let mut coordinates = std::collections::BTreeSet::new();

    for qubit in code.data_qubits() {
        assert!(
            coordinates.insert(qubit.coordinate()),
            "duplicate data-qubit coordinate detected"
        );
    }
}

#[test]
fn every_coordinate_maps_to_exactly_one_qubit() {
    let code = code_d5();

    for row in 0..5 {
        for column in 0..5 {
            let coordinate =
                Coordinate::new(row, column);

            let qubit = code
                .qubit_at(coordinate)
                .expect("valid coordinate must map to a qubit");

            assert_eq!(
                qubit.coordinate(),
                coordinate
            );

            assert_eq!(
                qubit.index().index(),
                row * 5 + column
            );
        }
    }
}

#[test]
fn qubit_coordinate_mapping_is_reversible() {
    let code = code_d5();

    for qubit in code.data_qubits() {
        let coordinate =
            code.coordinate_of(qubit.index())
                .expect("valid qubit must have a coordinate");

        assert_eq!(
            coordinate,
            qubit.coordinate()
        );
    }
}

#[test]
fn first_and_last_coordinates_are_correct() {
    let code = code_d5();

    let first = code
        .qubit_at(Coordinate::new(0, 0))
        .expect("origin must exist");

    assert_eq!(
        first.index(),
        QubitIndex::new(0)
    );

    let last = code
        .qubit_at(Coordinate::new(4, 4))
        .expect("last coordinate must exist");

    assert_eq!(
        last.index(),
        QubitIndex::new(24)
    );
}

#[test]
fn out_of_range_coordinate_is_rejected() {
    let code = code_d5();

    for coordinate in [
        Coordinate::new(5, 0),
        Coordinate::new(0, 5),
        Coordinate::new(5, 5),
        Coordinate::new(usize::MAX, 0),
        Coordinate::new(0, usize::MAX),
    ] {
        let result =
            assert_no_panic(|| code.qubit_at(coordinate));

        assert!(
            result.is_err(),
            "out-of-range coordinate must be rejected"
        );
    }
}

#[test]
fn nonexistent_qubit_is_rejected() {
    let code = code_d5();

    let result =
        assert_no_panic(|| {
            code.coordinate_of(QubitIndex::new(25))
        });

    assert!(
        result.is_err(),
        "qubit outside the code must be rejected"
    );
}

// ============================================================================
// Face topology
// ============================================================================

#[test]
fn_distance_three_faces_have_four_data_qubits() {
    let code = code_d3();

    for row in 0..2 {
        for column in 0..2 {
            let face = code
                .face_qubits(row, column)
                .expect("valid face must have four qubits");

            let unique: std::collections::BTreeSet<_> =
                face.into_iter().collect();

            assert_eq!(
                unique.len(),
                4,
                "face support must contain four unique qubits"
            );
        }
    }
}

#[test]
fn face_coordinates_are_geometrically_adjacent() {
    let code = code_d5();

    let face = code
        .face_qubits(1, 2)
        .expect("valid face must construct");

    let coordinates: Vec<_> = face
        .iter()
        .map(|qubit| {
            code.coordinate_of(*qubit)
                .expect("face qubit must exist")
        })
        .collect();

    assert!(
        coordinates.contains(&Coordinate::new(1, 2))
    );

    assert!(
        coordinates.contains(&Coordinate::new(1, 3))
    );

    assert!(
        coordinates.contains(&Coordinate::new(2, 2))
    );

    assert!(
        coordinates.contains(&Coordinate::new(2, 3))
    );
}

#[test]
fn out_of_range_face_is_rejected() {
    let code = code_d5();

    for (row, column) in [
        (4usize, 0usize),
        (0, 4),
        (4, 4),
        (usize::MAX, 0),
        (0, usize::MAX),
    ] {
        let result =
            assert_no_panic(|| code.face_qubits(row, column));

        assert!(
            result.is_err(),
            "invalid face ({row}, {column}) must be rejected"
        );
    }
}

// ============================================================================
// Stabilizer topology
// ============================================================================

#[test]
fn stabilizer_ids_are_contiguous() {
    let code = code_d5();

    let mut ids = std::collections::BTreeSet::new();

    for stabilizer in code.stabilizers() {
        assert!(
            ids.insert(stabilizer.id()),
            "stabilizer IDs must be unique"
        );
    }

    for id in 0..code.num_stabilizers() {
        assert!(
            ids.contains(&id),
            "missing stabilizer ID {id}"
        );
    }
}

#[test]
fn stabilizer_supports_contain_no_duplicate_qubits() {
    let code = code_d5();

    for stabilizer in code.stabilizers() {
        let unique: std::collections::BTreeSet<_> =
            stabilizer.support().iter().copied().collect();

        assert_eq!(
            unique.len(),
            stabilizer.support().len(),
            "stabilizer {} contains duplicate qubits",
            stabilizer.id()
        );
    }
}

#[test]
fn stabilizer_supports_reference_existing_qubits() {
    let code = code_d5();

    for stabilizer in code.stabilizers() {
        for qubit in stabilizer.support() {
            assert!(
                qubit.index() < code.num_data_qubits(),
                "stabilizer {} references nonexistent qubit {}",
                stabilizer.id(),
                qubit.index()
            );
        }
    }
}

#[test]
fn bulk_stabilizers_have_weight_four() {
    let code = code_d5();

    for stabilizer in code.stabilizers() {
        if !stabilizer.is_boundary() {
            assert_eq!(
                stabilizer.weight(),
                4,
                "bulk stabilizer {} must have weight 4",
                stabilizer.id()
            );
        }
    }
}

#[test]
fn boundary_stabilizers_have_weight_two() {
    let code = code_d5();

    for stabilizer in code.stabilizers() {
        if stabilizer.is_boundary() {
            assert_eq!(
                stabilizer.weight(),
                2,
                "boundary stabilizer {} must have weight 2",
                stabilizer.id()
            );
        }
    }
}

#[test]
fn stabilizers_are_x_or_z_type() {
    let code = code_d5();

    for stabilizer in code.stabilizers() {
        assert!(
            matches!(
                stabilizer.kind(),
                StabilizerKind::X | StabilizerKind::Z
            ),
            "stabilizer must have a valid X/Z type"
        );
    }
}

#[test]
fn boundary_classification_is_explicit() {
    let code = code_d5();

    for stabilizer in code.stabilizers() {
        if let Some(boundary) = stabilizer.boundary() {
            assert!(
                matches!(
                    boundary,
                    Boundary::Top
                        | Boundary::Bottom
                        | Boundary::Left
                        | Boundary::Right
                )
            );
        }
    }
}

// ============================================================================
// Mathematical validation
// ============================================================================

#[test]
fn distance_three_passes_full_validation() {
    let code = code_d3();

    let result =
        assert_no_panic(|| code.validate());

    assert!(
        result.is_ok(),
        "distance-3 code must pass structural and mathematical validation: {result:?}"
    );
}

#[test]
fn distance_five_passes_full_validation() {
    let code = code_d5();

    let result =
        assert_no_panic(|| code.validate());

    assert!(
        result.is_ok(),
        "distance-5 code must pass validation: {result:?}"
    );
}

#[test]
fn stabilizer_group_is_valid() {
    let code = code_d3();

    let group = code
        .stabilizer_group()
        .expect("surface code must produce a valid stabilizer group");

    assert!(
        group.validate().is_ok(),
        "stabilizer generators must mutually commute"
    );
}

#[test]
fn logical_operators_pass_validation() {
    let code = code_d3();

    assert!(
        code.validate_logical_operators().is_ok(),
        "logical X and Z must satisfy the surface-code logical-operator contract"
    );
}

#[test]
fn logical_x_has_expected_weight() {
    for distance in [3usize, 5, 7] {
        let code = SurfaceCode::new(distance)
            .expect("valid code must construct");

        assert_eq!(
            code.logical_x().weight(),
            distance,
            "logical X must have weight equal to code distance"
        );
    }
}

#[test]
fn logical_z_has_expected_weight() {
    for distance in [3usize, 5, 7] {
        let code = SurfaceCode::new(distance)
            .expect("valid code must construct");

        assert_eq!(
            code.logical_z().weight(),
            distance,
            "logical Z must have weight equal to code distance"
        );
    }
}

#[test]
fn logical_x_and_z_are_non_identity() {
    let code = code_d3();

    assert!(
        !code.logical_x().operator().is_identity(),
        "logical X cannot be identity"
    );

    assert!(
        !code.logical_z().operator().is_identity(),
        "logical Z cannot be identity"
    );
}

// ============================================================================
// Syndrome extraction
// ============================================================================

#[test]
fn identity_error_has_trivial_syndrome() {
    let code = code_d3();

    let identity =
        PauliString::identity(code.num_data_qubits());

    let syndrome = code
        .syndrome(&identity)
        .expect("identity error must have a valid syndrome");

    assert!(
        syndrome.is_trivial(),
        "identity error must produce no detection events"
    );
}

#[test]
fn single_qubit_x_error_is_representable() {
    let code = code_d3();

    let mut paulis =
        vec![Pauli::I; code.num_data_qubits()];

    paulis[0] = Pauli::X;

    let error =
        PauliString::from_paulis(&paulis);

    let result =
        assert_no_panic(|| code.syndrome(&error));

    assert!(
        result.is_ok(),
        "a valid single-qubit X error must produce a syndrome"
    );
}

#[test]
fn single_qubit_z_error_is_representable() {
    let code = code_d3();

    let mut paulis =
        vec![Pauli::I; code.num_data_qubits()];

    paulis[0] = Pauli::Z;

    let error =
        PauliString::from_paulis(&paulis);

    let result =
        assert_no_panic(|| code.syndrome(&error));

    assert!(
        result.is_ok(),
        "a valid single-qubit Z error must produce a syndrome"
    );
}

#[test]
fn single_qubit_y_error_is_representable() {
    let code = code_d3();

    let mut paulis =
        vec![Pauli::I; code.num_data_qubits()];

    paulis[4] = Pauli::Y;

    let error =
        PauliString::from_paulis(&paulis);

    let result =
        assert_no_panic(|| code.syndrome(&error));

    assert!(
        result.is_ok(),
        "a valid single-qubit Y error must produce a syndrome"
    );
}

// ============================================================================
// Exact distance
// ============================================================================

#[test]
fn distance_three_is_exactly_three() {
    let code = code_d3();

    let distance =
        assert_no_panic(|| code.verify_distance())
            .expect("distance verification must succeed");

    assert_eq!(
        distance,
        3
    );
}

#[test]
fn distance_five_is_exactly_five() {
    let code = code_d5();

    let distance =
        assert_no_panic(|| code.verify_distance())
            .expect("distance verification must succeed");

    assert_eq!(
        distance,
        5
    );
}

// ============================================================================
// Determinism
// ============================================================================

#[test]
fn repeated_construction_is_deterministic() {
    let first = code_d5();
    let second = code_d5();

    assert_eq!(
        first.distance(),
        second.distance()
    );

    assert_eq!(
        first.data_qubits(),
        second.data_qubits()
    );

    assert_eq!(
        first.stabilizers(),
        second.stabilizers()
    );

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
fn repeated_validation_is_deterministic() {
    let code = code_d5();

    for _ in 0..100 {
        assert!(
            code.validate().is_ok(),
            "validation result must remain deterministic"
        );
    }
}

// ============================================================================
// Resource / robustness checks
// ============================================================================

#[test]
fn public_constructors_do_not_panic_on_malformed_distances() {
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
fn topology_queries_do_not_panic_on_malformed_coordinates() {
    let code = code_d3();

    let coordinates = [
        Coordinate::new(usize::MAX, 0),
        Coordinate::new(0, usize::MAX),
        Coordinate::new(usize::MAX, usize::MAX),
    ];

    for coordinate in coordinates {
        let result =
            assert_no_panic(|| code.qubit_at(coordinate));

        assert!(
            result.is_err(),
            "malformed coordinate must be rejected"
        );
    }
}

#[test]
fn topology_queries_do_not_panic_on_invalid_qubit_indices() {
    let code = code_d3();

    for index in [
        9usize,
        10,
        usize::MAX,
    ] {
        let result =
            assert_no_panic(|| {
                code.coordinate_of(
                    QubitIndex::new(index)
                )
            });

        assert!(
            result.is_err(),
            "invalid qubit index {index} must be rejected"
        );
    }
}

// ============================================================================
// Regression guards
// ============================================================================

#[test]
fn distance_three_remains_the_minimum_supported_code() {
    assert!(
        SurfaceCode::new(3).is_ok()
    );

    assert!(
        SurfaceCode::new(1).is_err()
    );

    assert!(
        SurfaceCode::new(2).is_err()
    );
}

#[test]
fn canonical_code_dimensions_remain_stable() {
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
}