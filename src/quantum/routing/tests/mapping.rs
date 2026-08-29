//! Zamani Quantum Routing — Mapping Production Test Suite
//!
//! `src/quantum/routing/tests/mapping.rs`
//!
//! # Responsibility
//!
//! This file is the authoritative test suite for:
//!
//! ```text
//! src/quantum/routing/mapping.rs
//! ```
//!
//! It verifies the complete logical-to-physical mapping contract used by:
//!
//! - layout;
//! - shortest-path routing;
//! - basic routing;
//! - lookahead routing;
//! - SABRE;
//! - noise-aware routing;
//! - dynamic routing;
//! - routing transactions;
//! - verification;
//! - compiler/IR integration;
//! - hardware integration;
//! - benchmarking.
//!
//! # Architectural rule
//!
//! These tests test mapping semantics only.
//!
//! They deliberately do NOT test:
//!
//! - topology algorithms;
//! - routing heuristics;
//! - gate synthesis;
//! - OpenQASM;
//! - hardware provider SDKs;
//! - scheduling;
//! - pulse generation;
//! - simulation;
//! - QEC decoding.
//!
//! Those responsibilities belong to their respective test suites.
//!
//! # Integration contract
//!
//! This file assumes the stable mapping API provided by:
//!
//! ```text
//! src/quantum/routing/types.rs
//! src/quantum/routing/mapping.rs
//! ```
//!
//! It intentionally does not require future files such as:
//!
//! ```text
//! topology.rs
//! cost.rs
//! router.rs
//! algorithms/*.rs
//! verification.rs
//! transpiler.rs
//! ```
//!
//! to be implemented before these tests can define the mapping contract.
//!
//! Later routing components consume the exact behavior tested here.
//!
//! # Production guarantees tested
//!
//! The suite verifies:
//!
//! - empty mapping behavior;
//! - typed identifiers;
//! - assignment;
//! - duplicate logical rejection;
//! - duplicate physical rejection;
//! - checked assignment;
//! - atomic batch assignment;
//! - checked atomic batch assignment;
//! - logical lookup;
//! - physical lookup;
//! - required lookups;
//! - deterministic iteration;
//! - logical/physical enumeration;
//! - unassignment;
//! - moving into an empty physical location;
//! - collision-safe movement;
//! - physical swaps;
//! - logical swaps;
//! - empty-location swaps;
//! - self-swaps;
//! - sequential SWAP application;
//! - atomic SWAP limits;
//! - permutations;
//! - snapshots;
//! - restoration;
//! - transactional rollback;
//! - transactional commit;
//! - checked transactions;
//! - invariant validation;
//! - physical-domain validation;
//! - complete logical-qubit validation;
//! - physical-qubit validation;
//! - equality/equivalence;
//! - difference reporting;
//! - arbitrary physical identifiers;
//! - large mappings;
//! - deterministic behavior;
//! - no partial state after failed operations.
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
//! - no unsafe Rust.
//!
//! # Safety
//!
//! This test module contains no unsafe code.
//!
//! The explicit crate/module-level unsafe denial below ensures that an unsafe
//! block or unsafe item cannot accidentally be introduced into this test suite.
//!
//! # Integration into the routing test tree
//!
//! The intended production structure is:
//!
//! ```text
//! src/quantum/routing/tests/
//! ├── mapping.rs       <-- this file
//! ├── topology.rs
//! ├── layout.rs
//! ├── shortest_path.rs
//! ├── basic.rs
//! ├── lookahead.rs
//! ├── sabre.rs
//! ├── noise_aware.rs
//! ├── dynamic.rs
//! ├── multi_qubit.rs
//! ├── directed.rs
//! ├── transactional.rs
//! ├── verification.rs
//! └── end_to_end.rs
//! ```
//!
//! If a `tests/mod.rs` module is used, it should include:
//!
//! ```text
//! mod mapping;
//! ```
//!
//! This file does not depend on that future module implementation and can be
//! incorporated without changing its tests.
//!
//! # Definition of done
//!
//! This file is complete when:
//!
//! 1. every public mapping behavior is covered by at least one test;
//! 2. every mutation that promises atomicity has a rollback test;
//! 3. every bidirectional operation verifies both directions;
//! 4. deterministic APIs are tested without relying on HashMap iteration order;
//! 5. invalid operations prove that state is unchanged;
//! 6. snapshot/transaction semantics are verified;
//! 7. no test requires unsafe code;
//! 8. the suite compiles against Rust 1.97/1.97.1;
//! 9. later routing algorithms can rely on the mapping invariants established
//!    here without modifying this file merely because those algorithms are
//!    added.

// The routing root already denies unsafe code. Keep this test module explicit
// as well so it remains safe even if it is compiled through a different test
// harness arrangement.
#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use crate::quantum::routing::mapping::{
    MappingError,
    MappingTransactionError,
    QubitMapping,
};
use crate::quantum::routing::types::{
    LogicalQubitId,
    PhysicalQubitId,
};

// =============================================================================
// Test helpers
// =============================================================================

fn logical(index: usize) -> LogicalQubitId {
    LogicalQubitId::new(index)
}

fn physical(index: usize) -> PhysicalQubitId {
    PhysicalQubitId::new(index)
}

fn mapping_with_pairs(
    pairs: &[(usize, usize)],
) -> QubitMapping {
    QubitMapping::from_assignments(
        pairs
            .iter()
            .copied()
            .map(|(logical_id, physical_id)| {
                (logical(logical_id), physical(physical_id))
            }),
    )
    .expect("test mapping must be valid")
}

fn assert_mapping_valid(mapping: &QubitMapping) {
    mapping
        .validate()
        .expect("mapping must satisfy bidirectional invariants");
}

fn assert_mapping(
    mapping: &QubitMapping,
    expected: &[(usize, usize)],
) {
    let expected: Vec<_> = expected
        .iter()
        .copied()
        .map(|(logical_id, physical_id)| {
            (logical(logical_id), physical(physical_id))
        })
        .collect();

    assert_eq!(mapping.logical_to_physical(), expected);

    let mut expected_reverse: Vec<_> = expected
        .iter()
        .copied()
        .map(|(logical_id, physical_id)| {
            (physical_id, logical_id)
        })
        .collect();

    expected_reverse.sort_unstable_by_key(|(physical_id, _)| *physical_id);

    assert_eq!(
        mapping.physical_to_logical(),
        expected_reverse
    );

    assert_mapping_valid(mapping);
}

// =============================================================================
// Construction
// =============================================================================

#[test]
fn new_mapping_is_empty_and_valid() {
    let mapping = QubitMapping::new();

    assert!(mapping.is_empty());
    assert_eq!(mapping.len(), 0);
    assert!(mapping.logical_qubits().is_empty());
    assert!(mapping.physical_qubits().is_empty());

    assert_mapping_valid(&mapping);
}

#[test]
fn default_mapping_is_empty_and_valid() {
    let mapping = QubitMapping::default();

    assert!(mapping.is_empty());
    assert_eq!(mapping.len(), 0);
    assert_mapping_valid(&mapping);
}

#[test]
fn with_capacity_does_not_change_semantics() {
    let mut mapping = QubitMapping::with_capacity(1024);

    assert!(mapping.is_empty());

    mapping
        .assign(logical(0), physical(99))
        .expect("assignment should succeed");

    assert_eq!(
        mapping.physical_of(logical(0)),
        Some(physical(99))
    );

    assert_mapping_valid(&mapping);
}

#[test]
fn from_assignments_constructs_valid_mapping() {
    let mapping = mapping_with_pairs(&[
        (0, 4),
        (1, 9),
        (2, 20),
    ]);

    assert_mapping(
        &mapping,
        &[
            (0, 4),
            (1, 9),
            (2, 20),
        ],
    );
}

#[test]
fn from_assignments_rejects_duplicate_logical_qubit() {
    let result = QubitMapping::from_assignments([
        (logical(0), physical(0)),
        (logical(0), physical(1)),
    ]);

    assert_eq!(
        result,
        Err(MappingError::LogicalAlreadyMapped {
            logical: logical(0),
            physical: physical(0),
        })
    );
}

#[test]
fn from_assignments_rejects_duplicate_physical_qubit() {
    let result = QubitMapping::from_assignments([
        (logical(0), physical(0)),
        (logical(1), physical(0)),
    ]);

    assert_eq!(
        result,
        Err(MappingError::PhysicalAlreadyMapped {
            physical: physical(0),
            logical: logical(0),
        })
    );
}

#[test]
fn from_assignments_with_limit_accepts_exact_limit() {
    let mapping = QubitMapping::from_assignments_with_limit(
        [
            (logical(0), physical(0)),
            (logical(1), physical(1)),
            (logical(2), physical(2)),
        ],
        3,
    )
    .expect("exactly the configured number of assignments is valid");

    assert_eq!(mapping.len(), 3);
    assert_mapping_valid(&mapping);
}

#[test]
fn from_assignments_with_limit_rejects_excess_input() {
    let result = QubitMapping::from_assignments_with_limit(
        [
            (logical(0), physical(0)),
            (logical(1), physical(1)),
            (logical(2), physical(2)),
        ],
        2,
    );

    assert_eq!(
        result,
        Err(MappingError::OperationLimitExceeded {
            requested: 3,
            maximum: 2,
        })
    );
}

// =============================================================================
// Basic lookup semantics
// =============================================================================

#[test]
fn logical_and_physical_membership_are_bidirectional() {
    let mapping = mapping_with_pairs(&[
        (2, 7),
        (5, 13),
    ]);

    assert!(mapping.contains_logical(logical(2)));
    assert!(mapping.contains_logical(logical(5)));
    assert!(!mapping.contains_logical(logical(0)));

    assert!(mapping.contains_physical(physical(7)));
    assert!(mapping.contains_physical(physical(13)));
    assert!(!mapping.contains_physical(physical(0)));
}

#[test]
fn physical_of_returns_current_location() {
    let mapping = mapping_with_pairs(&[
        (0, 10),
        (1, 20),
    ]);

    assert_eq!(
        mapping.physical_of(logical(0)),
        Some(physical(10))
    );
    assert_eq!(
        mapping.physical_of(logical(1)),
        Some(physical(20))
    );
    assert_eq!(mapping.physical_of(logical(2)), None);
}

#[test]
fn logical_at_returns_current_occupant() {
    let mapping = mapping_with_pairs(&[
        (0, 10),
        (1, 20),
    ]);

    assert_eq!(
        mapping.logical_at(physical(10)),
        Some(logical(0))
    );
    assert_eq!(
        mapping.logical_at(physical(20)),
        Some(logical(1))
    );
    assert_eq!(mapping.logical_at(physical(30)), None);
}

#[test]
fn require_physical_returns_assignment_or_explicit_error() {
    let mapping = mapping_with_pairs(&[(4, 12)]);

    assert_eq!(
        mapping.require_physical(logical(4)),
        Ok(physical(12))
    );

    assert_eq!(
        mapping.require_physical(logical(9)),
        Err(MappingError::LogicalNotMapped {
            logical: logical(9),
        })
    );
}

#[test]
fn require_logical_returns_occupant_or_explicit_error() {
    let mapping = mapping_with_pairs(&[(4, 12)]);

    assert_eq!(
        mapping.require_logical(physical(12)),
        Ok(logical(4))
    );

    assert_eq!(
        mapping.require_logical(physical(9)),
        Err(MappingError::PhysicalNotMapped {
            physical: physical(9),
        })
    );
}

// =============================================================================
// Assignment
// =============================================================================

#[test]
fn assign_creates_both_mapping_directions() {
    let mut mapping = QubitMapping::new();

    mapping
        .assign(logical(7), physical(42))
        .expect("fresh assignment should succeed");

    assert_eq!(
        mapping.physical_of(logical(7)),
        Some(physical(42))
    );
    assert_eq!(
        mapping.logical_at(physical(42)),
        Some(logical(7))
    );

    assert_eq!(mapping.len(), 1);
    assert_mapping_valid(&mapping);
}

#[test]
fn assigning_already_mapped_logical_qubit_is_rejected_without_mutation() {
    let mut mapping = mapping_with_pairs(&[
        (0, 10),
        (1, 11),
    ]);

    let before = mapping.snapshot();

    let result = mapping.assign(
        logical(0),
        physical(99),
    );

    assert_eq!(
        result,
        Err(MappingError::LogicalAlreadyMapped {
            logical: logical(0),
            physical: physical(10),
        })
    );

    assert_eq!(mapping.snapshot(), before);
    assert_mapping_valid(&mapping);
}

#[test]
fn assigning_occupied_physical_qubit_is_rejected_without_mutation() {
    let mut mapping = mapping_with_pairs(&[
        (0, 10),
        (1, 11),
    ]);

    let before = mapping.snapshot();

    let result = mapping.assign(
        logical(2),
        physical(10),
    );

    assert_eq!(
        result,
        Err(MappingError::PhysicalAlreadyMapped {
            physical: physical(10),
            logical: logical(0),
        })
    );

    assert_eq!(mapping.snapshot(), before);
    assert_mapping_valid(&mapping);
}

#[test]
fn assign_checked_accepts_existing_physical_resource() {
    let mut mapping = QubitMapping::new();

    mapping
        .assign_checked(
            logical(0),
            physical(3),
            |candidate| candidate == physical(3),
        )
        .expect("physical resource should be accepted");

    assert_eq!(
        mapping.physical_of(logical(0)),
        Some(physical(3))
    );

    assert_mapping_valid(&mapping);
}

#[test]
fn assign_checked_rejects_missing_physical_resource() {
    let mut mapping = QubitMapping::new();

    let result = mapping.assign_checked(
        logical(0),
        physical(3),
        |candidate| candidate == physical(4),
    );

    assert!(matches!(
        result,
        Err(MappingError::InvariantViolation { .. })
    ));

    assert!(mapping.is_empty());
    assert_mapping_valid(&mapping);
}

#[test]
fn assign_many_is_atomic_on_duplicate_logical_failure() {
    let mut mapping = mapping_with_pairs(&[(0, 10)]);
    let before = mapping.snapshot();

    let result = mapping.assign_many([
        (logical(1), physical(11)),
        (logical(0), physical(12)),
        (logical(2), physical(13)),
    ]);

    assert_eq!(
        result,
        Err(MappingError::LogicalAlreadyMapped {
            logical: logical(0),
            physical: physical(10),
        })
    );

    assert_eq!(mapping.snapshot(), before);
    assert_mapping_valid(&mapping);
}

#[test]
fn assign_many_is_atomic_on_duplicate_physical_failure() {
    let mut mapping = mapping_with_pairs(&[(0, 10)]);
    let before = mapping.snapshot();

    let result = mapping.assign_many([
        (logical(1), physical(11)),
        (logical(2), physical(10)),
        (logical(3), physical(13)),
    ]);

    assert_eq!(
        result,
        Err(MappingError::PhysicalAlreadyMapped {
            physical: physical(10),
            logical: logical(0),
        })
    );

    assert_eq!(mapping.snapshot(), before);
    assert_mapping_valid(&mapping);
}

#[test]
fn assign_many_commits_all_assignments_on_success() {
    let mut mapping = QubitMapping::new();

    mapping
        .assign_many([
            (logical(0), physical(100)),
            (logical(1), physical(200)),
            (logical(2), physical(300)),
        ])
        .expect("all assignments are valid");

    assert_mapping(
        &mapping,
        &[
            (0, 100),
            (1, 200),
            (2, 300),
        ],
    );
}

#[test]
fn assign_many_checked_is_atomic_when_resource_validation_fails() {
    let mut mapping = mapping_with_pairs(&[(0, 10)]);
    let before = mapping.snapshot();

    let result = mapping.assign_many_checked(
        [
            (logical(1), physical(11)),
            (logical(2), physical(12)),
            (logical(3), physical(13)),
        ],
        |candidate| candidate != physical(12),
    );

    assert!(matches!(
        result,
        Err(MappingError::InvariantViolation { .. })
    ));

    assert_eq!(mapping.snapshot(), before);
    assert_mapping_valid(&mapping);
}

#[test]
fn assign_many_checked_commits_when_all_resources_exist() {
    let mut mapping = QubitMapping::new();

    mapping
        .assign_many_checked(
            [
                (logical(0), physical(5)),
                (logical(1), physical(8)),
                (logical(2), physical(13)),
            ],
            |candidate| {
                matches!(
                    candidate,
                    PhysicalQubitId if false
                )
            },
        )
        .expect_err(
            "this deliberately rejects all physical resources",
        );

    assert!(mapping.is_empty());
    assert_mapping_valid(&mapping);

    mapping
        .assign_many_checked(
            [
                (logical(0), physical(5)),
                (logical(1), physical(8)),
                (logical(2), physical(13)),
            ],
            |candidate| {
                candidate == physical(5)
                    || candidate == physical(8)
                    || candidate == physical(13)
            },
        )
        .expect("all physical resources exist");

    assert_mapping(
        &mapping,
        &[
            (0, 5),
            (1, 8),
            (2, 13),
        ],
    );
}

// =============================================================================
// Deterministic enumeration
// =============================================================================

#[test]
fn logical_to_physical_is_deterministically_sorted_by_logical_id() {
    let mapping = mapping_with_pairs(&[
        (9, 90),
        (2, 20),
        (7, 70),
        (0, 10),
    ]);

    assert_eq!(
        mapping.logical_to_physical(),
        vec![
            (logical(0), physical(10)),
            (logical(2), physical(20)),
            (logical(7), physical(70)),
            (logical(9), physical(90)),
        ]
    );
}

#[test]
fn physical_to_logical_is_deterministically_sorted_by_physical_id() {
    let mapping = mapping_with_pairs(&[
        (9, 90),
        (2, 20),
        (7, 70),
        (0, 10),
    ]);

    assert_eq!(
        mapping.physical_to_logical(),
        vec![
            (physical(10), logical(0)),
            (physical(20), logical(2)),
            (physical(70), logical(7)),
            (physical(90), logical(9)),
        ]
    );
}

#[test]
fn logical_qubits_are_deterministically_sorted() {
    let mapping = mapping_with_pairs(&[
        (20, 1),
        (2, 2),
        (11, 3),
        (0, 4),
    ]);

    assert_eq!(
        mapping.logical_qubits(),
        vec![
            logical(0),
            logical(2),
            logical(11),
            logical(20),
        ]
    );
}

#[test]
fn physical_qubits_are_deterministically_sorted() {
    let mapping = mapping_with_pairs(&[
        (20, 101),
        (2, 3),
        (11, 77),
        (0, 42),
    ]);

    assert_eq!(
        mapping.physical_qubits(),
        vec![
            physical(3),
            physical(42),
            physical(77),
            physical(101),
        ]
    );
}

// =============================================================================
// Unassignment
// =============================================================================

#[test]
fn unassign_logical_removes_both_directions() {
    let mut mapping = mapping_with_pairs(&[
        (0, 10),
        (1, 11),
    ]);

    let released = mapping
        .unassign_logical(logical(0))
        .expect("mapped logical qubit should be removable");

    assert_eq!(released, physical(10));
    assert!(!mapping.contains_logical(logical(0)));
    assert!(!mapping.contains_physical(physical(10)));

    assert_eq!(
        mapping.physical_of(logical(1)),
        Some(physical(11))
    );

    assert_eq!(mapping.len(), 1);
    assert_mapping_valid(&mapping);
}

#[test]
fn unassign_logical_rejects_unknown_logical_qubit() {
    let mut mapping = mapping_with_pairs(&[(0, 10)]);
    let before = mapping.snapshot();

    let result = mapping.unassign_logical(logical(99));

    assert_eq!(
        result,
        Err(MappingError::LogicalNotMapped {
            logical: logical(99),
        })
    );

    assert_eq!(mapping.snapshot(), before);
    assert_mapping_valid(&mapping);
}

#[test]
fn unassign_physical_removes_both_directions() {
    let mut mapping = mapping_with_pairs(&[
        (0, 10),
        (1, 11),
    ]);

    let released = mapping
        .unassign_physical(physical(11))
        .expect("occupied physical qubit should be removable");

    assert_eq!(released, logical(1));
    assert!(!mapping.contains_logical(logical(1)));
    assert!(!mapping.contains_physical(physical(11)));

    assert_eq!(
        mapping.physical_of(logical(0)),
        Some(physical(10))
    );

    assert_eq!(mapping.len(), 1);
    assert_mapping_valid(&mapping);
}

#[test]
fn unassign_physical_rejects_empty_physical_location() {
    let mut mapping = mapping_with_pairs(&[(0, 10)]);
    let before = mapping.snapshot();

    let result = mapping.unassign_physical(physical(99));

    assert_eq!(
        result,
        Err(MappingError::PhysicalNotMapped {
            physical: physical(99),
        })
    );

    assert_eq!(mapping.snapshot(), before);
    assert_mapping_valid(&mapping);
}

#[test]
fn clear_removes_all_assignments() {
    let mut mapping = mapping_with_pairs(&[
        (0, 10),
        (1, 11),
        (2, 12),
    ]);

    mapping.clear();

    assert!(mapping.is_empty());
    assert_eq!(mapping.len(), 0);
    assert_mapping_valid(&mapping);
}

// =============================================================================
// Logical movement
// =============================================================================

#[test]
fn move_logical_moves_into_empty_physical_location() {
    let mut mapping = mapping_with_pairs(&[
        (0, 10),
        (1, 11),
    ]);

    let previous = mapping
        .move_logical(logical(0), physical(99))
        .expect("target physical location is empty");

    assert_eq!(previous, physical(10));

    assert_eq!(
        mapping.physical_of(logical(0)),
        Some(physical(99))
    );

    assert_eq!(
        mapping.logical_at(physical(99)),
        Some(logical(0))
    );

    assert_eq!(mapping.logical_at(physical(10)), None);

    assert_mapping_valid(&mapping);
}

#[test]
fn move_logical_to_same_location_is_a_noop() {
    let mut mapping = mapping_with_pairs(&[(0, 10)]);
    let before = mapping.snapshot();

    let previous = mapping
        .move_logical(logical(0), physical(10))
        .expect("moving to current location is valid");

    assert_eq!(previous, physical(10));
    assert_eq!(mapping.snapshot(), before);
    assert_mapping_valid(&mapping);
}

#[test]
fn move_logical_rejects_unknown_logical_qubit() {
    let mut mapping = mapping_with_pairs(&[(0, 10)]);
    let before = mapping.snapshot();

    let result = mapping.move_logical(
        logical(99),
        physical(20),
    );

    assert_eq!(
        result,
        Err(MappingError::LogicalNotMapped {
            logical: logical(99),
        })
    );

    assert_eq!(mapping.snapshot(), before);
    assert_mapping_valid(&mapping);
}

#[test]
fn move_logical_rejects_occupied_target_without_mutation() {
    let mut mapping = mapping_with_pairs(&[
        (0, 10),
        (1, 20),
    ]);

    let before = mapping.snapshot();

    let result = mapping.move_logical(
        logical(0),
        physical(20),
    );

    assert_eq!(
        result,
        Err(MappingError::PhysicalCollision {
            physical: physical(20),
            existing: logical(1),
            requested: logical(0),
        })
    );

    assert_eq!(mapping.snapshot(), before);
    assert_mapping_valid(&mapping);
}

// =============================================================================
// Physical SWAP semantics
// =============================================================================

#[test]
fn swap_physical_exchanges_two_occupied_locations() {
    let mut mapping = mapping_with_pairs(&[
        (0, 10),
        (1, 20),
    ]);

    mapping
        .swap_physical(physical(10), physical(20))
        .expect("occupied physical locations can be swapped");

    assert_mapping(
        &mapping,
        &[
            (0, 20),
            (1, 10),
        ],
    );
}

#[test]
fn swap_physical_moves_occupied_state_into_empty_location() {
    let mut mapping = mapping_with_pairs(&[(0, 10)]);

    mapping
        .swap_physical(physical(10), physical(20))
        .expect("one side may be empty");

    assert_mapping(
        &mapping,
        &[(0, 20)],
    );
}

#[test]
fn swap_physical_moves_occupied_state_from_second_location_into_empty_first() {
    let mut mapping = mapping_with_pairs(&[(0, 20)]);

    mapping
        .swap_physical(physical(10), physical(20))
        .expect("one side may be empty");

    assert_mapping(
        &mapping,
        &[(0, 10)],
    );
}

#[test]
fn swap_physical_with_two_empty_locations_is_noop() {
    let mut mapping = mapping_with_pairs(&[(0, 10)]);
    let before = mapping.snapshot();

    mapping
        .swap_physical(physical(20), physical(30))
        .expect("empty locations can be swapped");

    assert_eq!(mapping.snapshot(), before);
    assert_mapping_valid(&mapping);
}

#[test]
fn swap_physical_with_same_location_is_noop() {
    let mut mapping = mapping_with_pairs(&[
        (0, 10),
        (1, 20),
    ]);

    let before = mapping.snapshot();

    mapping
        .swap_physical(physical(10), physical(10))
        .expect("self-swap is defined as a no-op");

    assert_eq!(mapping.snapshot(), before);
    assert_mapping_valid(&mapping);
}

#[test]
fn swap_logical_exchanges_the_physical_locations_of_two_logical_qubits() {
    let mut mapping = mapping_with_pairs(&[
        (0, 10),
        (1, 20),
    ]);

    mapping
        .swap_logical(logical(0), logical(1))
        .expect("both logical qubits are mapped");

    assert_mapping(
        &mapping,
        &[
            (0, 20),
            (1, 10),
        ],
    );
}

#[test]
fn swap_logical_with_same_logical_qubit_is_noop() {
    let mut mapping = mapping_with_pairs(&[(0, 10)]);
    let before = mapping.snapshot();

    mapping
        .swap_logical(logical(0), logical(0))
        .expect("self logical swap is a no-op");

    assert_eq!(mapping.snapshot(), before);
    assert_mapping_valid(&mapping);
}

#[test]
fn swap_logical_rejects_unknown_first_logical_qubit() {
    let mut mapping = mapping_with_pairs(&[(1, 20)]);
    let before = mapping.snapshot();

    let result = mapping.swap_logical(
        logical(0),
        logical(1),
    );

    assert_eq!(
        result,
        Err(MappingError::LogicalNotMapped {
            logical: logical(0),
        })
    );

    assert_eq!(mapping.snapshot(), before);
    assert_mapping_valid(&mapping);
}

#[test]
fn swap_logical_rejects_unknown_second_logical_qubit() {
    let mut mapping = mapping_with_pairs(&[(0, 10)]);
    let before = mapping.snapshot();

    let result = mapping.swap_logical(
        logical(0),
        logical(1),
    );

    assert_eq!(
        result,
        Err(MappingError::LogicalNotMapped {
            logical: logical(1),
        })
    );

    assert_eq!(mapping.snapshot(), before);
    assert_mapping_valid(&mapping);
}

// =============================================================================
// Sequential SWAP and permutation semantics
// =============================================================================

#[test]
fn apply_swaps_applies_swaps_in_order() {
    let mut mapping = mapping_with_pairs(&[
        (0, 10),
        (1, 20),
        (2, 30),
    ]);

    mapping
        .apply_swaps([
            (physical(10), physical(20)),
            (physical(20), physical(30)),
        ])
        .expect("both swaps are valid");

    // Initial:
    //
    // p10 = q0
    // p20 = q1
    // p30 = q2
    //
    // First swap:
    //
    // p10 = q1
    // p20 = q0
    // p30 = q2
    //
    // Second swap:
    //
    // p10 = q1
    // p20 = q2
    // p30 = q0

    assert_mapping(
        &mapping,
        &[
            (0, 30),
            (1, 10),
            (2, 20),
        ],
    );
}

#[test]
fn apply_swaps_with_limit_accepts_exact_limit() {
    let mut mapping = mapping_with_pairs(&[
        (0, 10),
        (1, 20),
        (2, 30),
    ]);

    let count = mapping
        .apply_swaps_with_limit(
            [
                (physical(10), physical(20)),
                (physical(20), physical(30)),
            ],
            2,
        )
        .expect("two swaps are within the limit");

    assert_eq!(count, 2);

    assert_mapping(
        &mapping,
        &[
            (0, 30),
            (1, 10),
            (2, 20),
        ],
    );
}

#[test]
fn apply_swaps_with_limit_rolls_back_when_limit_is_exceeded() {
    let mut mapping = mapping_with_pairs(&[
        (0, 10),
        (1, 20),
        (2, 30),
    ]);

    let before = mapping.snapshot();

    let result = mapping.apply_swaps_with_limit(
        [
            (physical(10), physical(20)),
            (physical(20), physical(30)),
            (physical(10), physical(30)),
        ],
        2,
    );

    assert_eq!(
        result,
        Err(MappingError::OperationLimitExceeded {
            requested: 3,
            maximum: 2,
        })
    );

    assert_eq!(mapping.snapshot(), before);
    assert_mapping_valid(&mapping);
}

#[test]
fn apply_permutation_uses_sequential_transposition_semantics() {
    let mut mapping = mapping_with_pairs(&[
        (0, 10),
        (1, 20),
        (2, 30),
        (3, 40),
    ]);

    mapping
        .apply_permutation([
            (physical(10), physical(20)),
            (physical(20), physical(30)),
            (physical(30), physical(40)),
        ])
        .expect("permutation should be valid");

    assert_mapping(
        &mapping,
        &[
            (0, 40),
            (1, 10),
            (2, 20),
            (3, 30),
        ],
    );
}

// =============================================================================
// Snapshot semantics
// =============================================================================

#[test]
fn snapshot_is_an_immutable_point_in_time_view() {
    let mut mapping = mapping_with_pairs(&[
        (0, 10),
        (1, 20),
    ]);

    let snapshot = mapping.snapshot();

    mapping
        .swap_physical(physical(10), physical(20))
        .expect("swap should succeed");

    assert_eq!(
        snapshot.physical_of(logical(0)),
        Some(physical(10))
    );
    assert_eq!(
        snapshot.physical_of(logical(1)),
        Some(physical(20))
    );

    assert_eq!(
        mapping.physical_of(logical(0)),
        Some(physical(20))
    );
    assert_eq!(
        mapping.physical_of(logical(1)),
        Some(physical(10))
    );
}

#[test]
fn snapshot_preserves_both_mapping_directions() {
    let mapping = mapping_with_pairs(&[
        (0, 10),
        (1, 20),
        (2, 30),
    ]);

    let snapshot = mapping.snapshot();

    assert_eq!(
        snapshot.logical_to_physical(),
        mapping.logical_to_physical()
    );

    assert_eq!(
        snapshot.physical_to_logical(),
        mapping.physical_to_logical()
    );

    assert_eq!(snapshot.len(), mapping.len());
    assert_eq!(snapshot.is_empty(), mapping.is_empty());
}

#[test]
fn restore_returns_mapping_to_snapshot_state() {
    let mut mapping = mapping_with_pairs(&[
        (0, 10),
        (1, 20),
    ]);

    let snapshot = mapping.snapshot();

    mapping
        .swap_physical(physical(10), physical(20))
        .expect("swap should succeed");

    mapping.restore(snapshot);

    assert_mapping(
        &mapping,
        &[
            (0, 10),
            (1, 20),
        ],
    );
}

// =============================================================================
// Transaction semantics
// =============================================================================

#[test]
fn transaction_commits_successful_mutation() {
    let mut mapping = mapping_with_pairs(&[(0, 10)]);

    let result = mapping.transaction(|mapping| {
        mapping
            .assign(logical(1), physical(20))
            .map(|()| 123usize)
    });

    assert_eq!(result, Ok(123));

    assert_mapping(
        &mapping,
        &[
            (0, 10),
            (1, 20),
        ],
    );
}

#[test]
fn transaction_rolls_back_failed_mutation() {
    let mut mapping = mapping_with_pairs(&[(0, 10)]);
    let before = mapping.snapshot();

    let result = mapping.transaction(|mapping| {
        mapping
            .assign(logical(1), physical(20))
            .expect("first mutation should succeed");

        mapping
            .assign(logical(2), physical(10))
            .map(|()| ())
    });

    assert_eq!(
        result,
        Err(MappingError::PhysicalAlreadyMapped {
            physical: physical(10),
            logical: logical(0),
        })
    );

    assert_eq!(mapping.snapshot(), before);
    assert_mapping_valid(&mapping);
}

#[test]
fn transaction_checked_commits_when_result_and_invariants_are_valid() {
    let mut mapping = mapping_with_pairs(&[(0, 10)]);

    let result = mapping.transaction_checked(|mapping| {
        mapping
            .assign(logical(1), physical(20))
            .map(|()| "committed")
    });

    assert_eq!(result, Ok("committed"));

    assert_mapping(
        &mapping,
        &[
            (0, 10),
            (1, 20),
        ],
    );
}

#[test]
fn transaction_checked_rolls_back_callback_error() {
    let mut mapping = mapping_with_pairs(&[(0, 10)]);
    let before = mapping.snapshot();

    let result = mapping.transaction_checked(
        |mapping| -> Result<(), &'static str> {
            mapping
                .assign(logical(1), physical(20))
                .expect("mutation should initially succeed");

            Err("deliberate failure")
        },
    );

    assert_eq!(
        result,
        Err(MappingTransactionError::Operation(
            "deliberate failure"
        ))
    );

    assert_eq!(mapping.snapshot(), before);
    assert_mapping_valid(&mapping);
}

// =============================================================================
// Mapping validation
// =============================================================================

#[test]
fn validate_accepts_empty_mapping() {
    let mapping = QubitMapping::new();

    assert!(mapping.validate().is_ok());
}

#[test]
fn validate_with_accepts_mapping_when_all_physical_resources_exist() {
    let mapping = mapping_with_pairs(&[
        (0, 10),
        (1, 20),
    ]);

    mapping
        .validate_with(|physical_id| {
            physical_id == physical(10)
                || physical_id == physical(20)
        })
        .expect("all physical resources exist");
}

#[test]
fn validate_with_rejects_missing_physical_resource() {
    let mapping = mapping_with_pairs(&[
        (0, 10),
        (1, 20),
    ]);

    let result = mapping.validate_with(|physical_id| {
        physical_id == physical(10)
    });

    assert!(matches!(
        result,
        Err(MappingError::InvariantViolation { .. })
    ));
}

#[test]
fn validate_logical_qubits_accepts_completely_mapped_collection() {
    let mapping = mapping_with_pairs(&[
        (0, 10),
        (1, 20),
        (2, 30),
    ]);

    mapping
        .validate_logical_qubits([
            logical(0),
            logical(1),
            logical(2),
        ])
        .expect("all logical qubits are mapped");
}

#[test]
fn validate_logical_qubits_rejects_missing_logical_qubit() {
    let mapping = mapping_with_pairs(&[
        (0, 10),
        (1, 20),
    ]);

    let result = mapping.validate_logical_qubits([
        logical(0),
        logical(1),
        logical(2),
    ]);

    assert_eq!(
        result,
        Err(MappingError::LogicalNotMapped {
            logical: logical(2),
        })
    );
}

#[test]
fn validate_physical_qubits_accepts_occupied_physical_collection() {
    let mapping = mapping_with_pairs(&[
        (0, 10),
        (1, 20),
    ]);

    mapping
        .validate_physical_qubits([
            physical(10),
            physical(20),
        ])
        .expect("all supplied physical qubits are mapped");
}

#[test]
fn validate_physical_qubits_rejects_unoccupied_physical_qubit() {
    let mapping = mapping_with_pairs(&[(0, 10)]);

    let result = mapping.validate_physical_qubits([
        physical(10),
        physical(20),
    ]);

    assert_eq!(
        result,
        Err(MappingError::PhysicalNotMapped {
            physical: physical(20),
        })
    );
}

// =============================================================================
// Equivalence and difference reporting
// =============================================================================

#[test]
fn equivalent_returns_true_for_identical_assignments() {
    let left = mapping_with_pairs(&[
        (0, 10),
        (1, 20),
    ]);

    let right = mapping_with_pairs(&[
        (1, 20),
        (0, 10),
    ]);

    assert!(left.equivalent(&right));
    assert!(!left.differs_from(&right));
    assert!(left.differences(&right).is_empty());
}

#[test]
fn equivalent_returns_false_for_different_assignments() {
    let left = mapping_with_pairs(&[
        (0, 10),
        (1, 20),
    ]);

    let right = mapping_with_pairs(&[
        (0, 20),
        (1, 10),
    ]);

    assert!(!left.equivalent(&right));
    assert!(left.differs_from(&right));
}

#[test]
fn differences_reports_moved_logical_qubits_deterministically() {
    let left = mapping_with_pairs(&[
        (0, 10),
        (1, 20),
        (2, 30),
    ]);

    let right = mapping_with_pairs(&[
        (0, 20),
        (1, 10),
        (3, 40),
    ]);

    assert_eq!(
        left.differences(&right),
        vec![
            (
                logical(0),
                Some(physical(10)),
                Some(physical(20)),
            ),
            (
                logical(1),
                Some(physical(20)),
                Some(physical(10)),
            ),
            (
                logical(2),
                Some(physical(30)),
                None,
            ),
            (
                logical(3),
                None,
                Some(physical(40)),
            ),
        ]
    );
}

#[test]
fn differences_is_empty_when_mappings_are_equal() {
    let mapping = mapping_with_pairs(&[
        (0, 100),
        (1, 200),
    ]);

    assert!(mapping.differences(&mapping).is_empty());
}

// =============================================================================
// Arbitrary physical identifiers
// =============================================================================

#[test]
fn mapping_does_not_assume_dense_physical_ids() {
    let mapping = mapping_with_pairs(&[
        (0, 7),
        (1, 1024),
        (2, 65_535),
        (3, 1_000_000),
    ]);

    assert_mapping(
        &mapping,
        &[
            (0, 7),
            (1, 1024),
            (2, 65_535),
            (3, 1_000_000),
        ],
    );
}

#[test]
fn mapping_does_not_require_logical_ids_to_be_dense() {
    let mapping = mapping_with_pairs(&[
        (3, 7),
        (100, 8),
        (10_000, 9),
    ]);

    assert_mapping(
        &mapping,
        &[
            (3, 7),
            (100, 8),
            (10_000, 9),
        ],
    );
}

#[test]
fn mapping_preserves_large_identifier_values() {
    let logical_id = usize::MAX - 10;
    let physical_id = usize::MAX - 20;

    let mapping = mapping_with_pairs(&[
        (logical_id, physical_id),
    ]);

    assert_eq!(
        mapping.physical_of(logical(logical_id)),
        Some(physical(physical_id))
    );

    assert_eq!(
        mapping.logical_at(physical(physical_id)),
        Some(logical(logical_id))
    );

    assert_mapping_valid(&mapping);
}

// =============================================================================
// State preservation after failed operations
// =============================================================================

#[test]
fn failed_assignment_does_not_change_unrelated_assignments() {
    let mut mapping = mapping_with_pairs(&[
        (0, 10),
        (1, 20),
        (2, 30),
    ]);

    let before = mapping.snapshot();

    let _ = mapping.assign(
        logical(3),
        physical(20),
    );

    assert_eq!(mapping.snapshot(), before);
    assert_mapping_valid(&mapping);
}

#[test]
fn failed_move_does_not_change_source_or_target() {
    let mut mapping = mapping_with_pairs(&[
        (0, 10),
        (1, 20),
    ]);

    let before = mapping.snapshot();

    let _ = mapping.move_logical(
        logical(0),
        physical(20),
    );

    assert_eq!(mapping.snapshot(), before);
    assert_mapping_valid(&mapping);
}

#[test]
fn failed_logical_swap_does_not_change_mapping() {
    let mut mapping = mapping_with_pairs(&[
        (0, 10),
        (1, 20),
    ]);

    let before = mapping.snapshot();

    let _ = mapping.swap_logical(
        logical(0),
        logical(99),
    );

    assert_eq!(mapping.snapshot(), before);
    assert_mapping_valid(&mapping);
}

#[test]
fn failed_limited_swap_sequence_rolls_back_every_previous_swap() {
    let mut mapping = mapping_with_pairs(&[
        (0, 10),
        (1, 20),
        (2, 30),
        (3, 40),
    ]);

    let before = mapping.snapshot();

    let result = mapping.apply_swaps_with_limit(
        [
            (physical(10), physical(20)),
            (physical(20), physical(30)),
            (physical(30), physical(40)),
        ],
        2,
    );

    assert_eq!(
        result,
        Err(MappingError::OperationLimitExceeded {
            requested: 3,
            maximum: 2,
        })
    );

    assert_eq!(mapping.snapshot(), before);
    assert_mapping_valid(&mapping);
}

// =============================================================================
// Mapping algebra / reversibility
// =============================================================================

#[test]
fn swapping_the_same_two_physical_locations_twice_restores_state() {
    let mut mapping = mapping_with_pairs(&[
        (0, 10),
        (1, 20),
        (2, 30),
    ]);

    let before = mapping.snapshot();

    mapping
        .swap_physical(physical(10), physical(20))
        .expect("first swap should succeed");

    mapping
        .swap_physical(physical(10), physical(20))
        .expect("second swap should succeed");

    assert_eq!(mapping.snapshot(), before);
    assert_mapping_valid(&mapping);
}

#[test]
fn moving_a_qubit_through_an_empty_location_and_back_restores_state() {
    let mut mapping = mapping_with_pairs(&[(0, 10)]);

    let before = mapping.snapshot();

    mapping
        .move_logical(logical(0), physical(20))
        .expect("first move should succeed");

    mapping
        .move_logical(logical(0), physical(10))
        .expect("second move should succeed");

    assert_eq!(mapping.snapshot(), before);
    assert_mapping_valid(&mapping);
}

#[test]
fn cyclic_physical_swaps_preserve_bijection() {
    let mut mapping = mapping_with_pairs(&[
        (0, 10),
        (1, 20),
        (2, 30),
    ]);

    mapping
        .apply_swaps([
            (physical(10), physical(20)),
            (physical(20), physical(30)),
            (physical(30), physical(10)),
        ])
        .expect("cyclic swaps should succeed");

    assert_eq!(mapping.len(), 3);
    assert_mapping_valid(&mapping);

    // Every logical qubit remains mapped exactly once.
    assert_eq!(mapping.logical_qubits().len(), 3);
    assert_eq!(mapping.physical_qubits().len(), 3);
}

// =============================================================================
// Large mapping behavior
// =============================================================================

#[test]
fn large_mapping_preserves_bidirectional_invariant() {
    const COUNT: usize = 2_000;

    let assignments = (0..COUNT).map(|index| {
        (
            logical(index),
            physical(index * 3 + 1),
        )
    });

    let mapping = QubitMapping::from_assignments(assignments)
        .expect("large unique mapping should be accepted");

    assert_eq!(mapping.len(), COUNT);
    assert_eq!(mapping.logical_qubits().len(), COUNT);
    assert_eq!(mapping.physical_qubits().len(), COUNT);

    assert_mapping_valid(&mapping);

    assert_eq!(
        mapping.physical_of(logical(0)),
        Some(physical(1))
    );

    assert_eq!(
        mapping.physical_of(logical(COUNT - 1)),
        Some(physical((COUNT - 1) * 3 + 1))
    );
}

#[test]
fn large_swap_sequence_preserves_bijection() {
    const COUNT: usize = 1_000;

    let assignments = (0..COUNT).map(|index| {
        (
            logical(index),
            physical(index),
        )
    });

    let mut mapping = QubitMapping::from_assignments(assignments)
        .expect("large mapping should be valid");

    let swaps = (0..COUNT - 1).map(|index| {
        (physical(index), physical(index + 1))
    });

    mapping
        .apply_swaps(swaps)
        .expect("large sequential swap sequence should succeed");

    assert_eq!(mapping.len(), COUNT);
    assert_mapping_valid(&mapping);
}

// =============================================================================
// Cross-operation consistency
// =============================================================================

#[test]
fn physical_lookup_is_always_the_inverse_of_logical_lookup() {
    let mut mapping = mapping_with_pairs(&[
        (0, 10),
        (1, 20),
        (2, 30),
        (3, 40),
    ]);

    let operations = [
        (physical(10), physical(20)),
        (physical(20), physical(30)),
        (physical(30), physical(40)),
        (physical(10), physical(40)),
    ];

    for (a, b) in operations {
        mapping
            .swap_physical(a, b)
            .expect("swap should succeed");

        assert_mapping_valid(&mapping);

        for (logical_id, physical_id) in
            mapping.logical_to_physical()
        {
            assert_eq!(
                mapping.logical_at(physical_id),
                Some(logical_id),
                "reverse lookup must exactly invert forward lookup"
            );
        }

        for (physical_id, logical_id) in
            mapping.physical_to_logical()
        {
            assert_eq!(
                mapping.physical_of(logical_id),
                Some(physical_id),
                "forward lookup must exactly invert reverse lookup"
            );
        }
    }
}

#[test]
fn all_public_mutating_operations_preserve_invariants() {
    let mut mapping = QubitMapping::new();

    mapping
        .assign(logical(0), physical(10))
        .expect("assignment");

    assert_mapping_valid(&mapping);

    mapping
        .assign(logical(1), physical(20))
        .expect("assignment");

    assert_mapping_valid(&mapping);

    mapping
        .move_logical(logical(0), physical(30))
        .expect("move");

    assert_mapping_valid(&mapping);

    mapping
        .swap_physical(physical(20), physical(30))
        .expect("physical swap");

    assert_mapping_valid(&mapping);

    mapping
        .swap_logical(logical(0), logical(1))
        .expect("logical swap");

    assert_mapping_valid(&mapping);

    mapping
        .unassign_logical(logical(0))
        .expect("unassignment");

    assert_mapping_valid(&mapping);

    mapping
        .assign(logical(2), physical(40))
        .expect("assignment");

    assert_mapping_valid(&mapping);

    mapping
        .unassign_physical(physical(40))
        .expect("physical unassignment");

    assert_mapping_valid(&mapping);
}

// =============================================================================
// Snapshot and mapping equivalence under routing-like operations
// =============================================================================

#[test]
fn snapshot_can_be_used_as_a_routing_speculation_boundary() {
    let mut mapping = mapping_with_pairs(&[
        (0, 0),
        (1, 1),
        (2, 2),
        (3, 3),
    ]);

    let speculative_state = mapping.snapshot();

    mapping
        .apply_swaps([
            (physical(0), physical(1)),
            (physical(1), physical(2)),
            (physical(2), physical(3)),
        ])
        .expect("speculative routing sequence should succeed");

    assert!(mapping.differs_from(
        &QubitMapping::from_assignments(
            [
                (logical(0), physical(0)),
                (logical(1), physical(1)),
                (logical(2), physical(2)),
                (logical(3), physical(3)),
            ]
        )
        .expect("reference mapping")
    ));

    mapping.restore(speculative_state);

    assert_mapping(
        &mapping,
        &[
            (0, 0),
            (1, 1),
            (2, 2),
            (3, 3),
        ],
    );
}

#[test]
fn routing_style_swap_and_restore_is_exactly_reversible() {
    let original = mapping_with_pairs(&[
        (0, 3),
        (1, 7),
        (2, 11),
        (3, 19),
    ]);

    let mut mapping = original.clone();

    let snapshot = mapping.snapshot();

    mapping
        .apply_swaps([
            (physical(3), physical(7)),
            (physical(7), physical(11)),
            (physical(11), physical(19)),
        ])
        .expect("forward routing movement should succeed");

    mapping
        .apply_swaps([
            (physical(11), physical(19)),
            (physical(7), physical(11)),
            (physical(3), physical(7)),
        ])
        .expect("reverse routing movement should succeed");

    assert!(mapping.equivalent(&original));
    assert_eq!(mapping.snapshot(), snapshot);
    assert_mapping_valid(&mapping);
}

// =============================================================================
// Error representation
// =============================================================================

#[test]
fn mapping_errors_are_displayable_for_diagnostics() {
    let errors = [
        MappingError::LogicalAlreadyMapped {
            logical: logical(0),
            physical: physical(1),
        },
        MappingError::PhysicalAlreadyMapped {
            physical: physical(1),
            logical: logical(0),
        },
        MappingError::LogicalNotMapped {
            logical: logical(2),
        },
        MappingError::PhysicalNotMapped {
            physical: physical(3),
        },
        MappingError::PhysicalCollision {
            physical: physical(4),
            existing: logical(5),
            requested: logical(6),
        },
        MappingError::InvariantViolation {
            message: "test invariant".to_string(),
        },
        MappingError::InvalidPermutation {
            message: "test permutation".to_string(),
        },
        MappingError::OperationLimitExceeded {
            requested: 11,
            maximum: 10,
        },
    ];

    for error in errors {
        let rendered = error.to_string();

        assert!(
            !rendered.is_empty(),
            "every mapping error must produce a diagnostic"
        );
    }
}

#[test]
fn transaction_error_is_displayable() {
    let operation_error =
        MappingTransactionError::Operation("failed");

    let invariant_error =
        MappingTransactionError::Invariant(
            MappingError::InvariantViolation {
                message: "invalid".to_string(),
            },
        );

    assert!(!operation_error.to_string().is_empty());
    assert!(!invariant_error.to_string().is_empty());
}

// =============================================================================
// Final contract test
// =============================================================================

#[test]
fn production_mapping_contract_is_satisfied() {
    let mut mapping = QubitMapping::new();

    // Initial allocation.
    mapping
        .assign_many([
            (logical(0), physical(101)),
            (logical(1), physical(203)),
            (logical(2), physical(307)),
            (logical(3), physical(401)),
        ])
        .expect("initial allocation must succeed");

    assert_mapping_valid(&mapping);

    // Topology/resource integration hook.
    mapping
        .validate_with(|physical_id| {
            [
                physical(101),
                physical(203),
                physical(307),
                physical(401),
                physical(503),
            ]
            .contains(&physical_id)
        })
        .expect("all mapped physical resources must exist");

    // Snapshot for speculative routing.
    let snapshot = mapping.snapshot();

    // Simulate routing movement.
    mapping
        .apply_swaps([
            (physical(101), physical(203)),
            (physical(203), physical(307)),
        ])
        .expect("routing movement must succeed");

    assert_mapping_valid(&mapping);

    // Verify the state actually changed.
    assert!(mapping.differs_from(
        &QubitMapping::from_assignments([
            (logical(0), physical(101)),
            (logical(1), physical(203)),
            (logical(2), physical(307)),
            (logical(3), physical(401)),
        ])
        .expect("reference mapping")
    ));

    // Restore speculative routing state.
    mapping.restore(snapshot);

    // Mapping must be exactly the initial state.
    assert_mapping(
        &mapping,
        &[
            (0, 101),
            (1, 203),
            (2, 307),
            (3, 401),
        ],
    );

    // Transactional routing must roll back on failure.
    let before_failure = mapping.snapshot();

    let result = mapping.transaction(
        |mapping| -> Result<(), MappingError> {
            mapping
                .swap_physical(
                    physical(101),
                    physical(203),
                )?;

            mapping.assign(
                logical(4),
                physical(203),
            )?;

            Ok(())
        },
    );

    assert_eq!(
        result,
        Err(MappingError::PhysicalAlreadyMapped {
            physical: physical(203),
            logical: logical(0),
        })
    );

    assert_eq!(
        mapping.snapshot(),
        before_failure,
        "failed routing transaction must expose no partial state"
    );

    // Final invariant.
    mapping
        .validate()
        .expect("production mapping must remain valid");

    assert_eq!(mapping.len(), 4);
}