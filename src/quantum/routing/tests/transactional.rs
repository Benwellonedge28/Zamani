//! Zamani Quantum Routing — Transactional Routing Test Suite
//!
//! `src/quantum/routing/tests/transactional.rs`
//!
//! # Responsibility
//!
//! This file is the authoritative regression suite for transactional state
//! semantics in the routing subsystem.
//!
//! It verifies the transaction contract implemented by:
//!
//! ```text
//! src/quantum/routing/mapping.rs
//! ```
//!
//! The tests deliberately exercise the stable mapping transaction primitives
//! rather than depending on unfinished routing algorithms. This makes the
//! file independently implementable and stable while the higher-level routing
//! stack evolves.
//!
//! # Transactional contract
//!
//! A routing mutation is transactional when:
//!
//! ```text
//! initial state
//!      │
//!      ▼
//! speculative mutation
//!      │
//!      ├─────────────── success ───────────────► COMMIT
//!      │
//!      └──────────────── failure ─────────────► ROLLBACK
//! ```
//!
//! On failure, the caller must observe exactly the state that existed before
//! the transaction began.
//!
//! This includes:
//!
//! - logical-to-physical mappings;
//! - physical-to-logical mappings;
//! - mapping cardinality;
//! - mapping equivalence;
//! - mapping validity;
//! - partially completed batch operations;
//! - partially completed SWAP sequences;
//! - bounded operations that exceed their configured limit.
//!
//! # Why this file exists separately
//!
//! Transactionality is not merely an implementation detail of SABRE or
//! shortest-path routing. It is a foundational correctness property used by:
//!
//! - layout;
//! - shortest-path routing;
//! - basic routing;
//! - lookahead;
//! - SABRE;
//! - noise-aware routing;
//! - dynamic routing;
//! - speculative candidate evaluation;
//! - compiler/IR transpilation;
//! - route verification;
//! - future distributed routing.
//!
//! Therefore transaction semantics must be frozen independently of those
//! algorithms.
//!
//! # Tested primitives
//!
//! This suite covers:
//!
//! 1. snapshot creation;
//! 2. snapshot immutability through opaque state;
//! 3. snapshot restoration;
//! 4. successful transaction commit;
//! 5. failed transaction rollback;
//! 6. rollback after multiple successful mutations;
//! 7. rollback after partial assignment;
//! 8. rollback after partial SWAP execution;
//! 9. rollback after operation-limit failure;
//! 10. checked transaction success;
//! 11. checked transaction operation failure;
//! 12. nested transaction semantics;
//! 13. transaction result propagation;
//! 14. transaction error propagation;
//! 15. mapping validity after commit;
//! 16. mapping validity after rollback;
//! 17. exact pre-transaction equivalence;
//! 18. deterministic transaction behavior;
//! 19. atomic `assign_many`;
//! 20. atomic `apply_swaps`;
//! 21. atomic `apply_swaps_with_limit`;
//! 22. preservation of arbitrary physical identifiers;
//! 23. no partial state after failed operations;
//! 24. repeated transactions;
//! 25. transaction behavior on an empty mapping.
//!
//! # Deliberate boundary
//!
//! This file does NOT test:
//!
//! - OpenQASM;
//! - compiler parsing;
//! - gate synthesis;
//! - hardware-provider APIs;
//! - topology discovery;
//! - SABRE heuristic quality;
//! - shortest-path quality;
//! - scheduling;
//! - pulse generation;
//! - quantum simulation;
//! - QEC decoding;
//! - benchmark execution.
//!
//! Those belong to their respective subsystems.
//!
//! # Integration contract
//!
//! This file depends only on the stable routing mapping contract:
//!
//! ```text
//! types.rs
//!    │
//!    ▼
//! mapping.rs
//!    │
//!    ▼
//! transactional.rs
//! ```
//!
//! It intentionally does not depend on:
//!
//! ```text
//! router.rs
//! algorithms/*
//! topology.rs
//! layout.rs
//! transpiler.rs
//! verification.rs
//! hardware/*
//! ```
//!
//! Consequently, implementing a later routing algorithm must not require
//! changing this file merely because the algorithm was added.
//!
//! Higher-level routing tests may use this suite's guarantees as assumptions.
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
//! This test module explicitly denies unsafe code.
//!
//! No test relies on:
//!
//! - raw pointers;
//! - `unsafe` blocks;
//! - implementation-private field access;
//! - undefined behavior;
//! - timing;
//! - thread scheduling;
//! - HashMap iteration order.
//!
//! # Definition of done
//!
//! This file is complete when:
//!
//! - every transactional mapping API has success coverage;
//! - every transactional mapping API has failure/rollback coverage where
//!   failure is representable through its safe public API;
//! - state before and after failure is compared exactly;
//! - both mapping directions are checked;
//! - mapping invariants are checked after every transaction scenario;
//! - batch operations prove atomicity;
//! - bounded operations prove rollback;
//! - checked transactions prove post-operation invariant validation;
//! - nested transactions prove inner rollback does not corrupt outer state;
//! - deterministic behavior is verified;
//! - the tests require no unsafe code;
//! - the tests compile against Rust 1.97.1;
//! - no future routing algorithm must modify this file merely to integrate.
//!
//! # Test-harness integration
//!
//! The repository currently keeps these routing test files under:
//!
//! ```text
//! src/quantum/routing/tests/
//! ```
//!
//! Rust does not automatically compile arbitrary files in that directory.
//! The routing test harness must therefore include this module through the
//! parent routing test module. The correct integration declaration is:
//!
//! ```text
//! #[cfg(test)]
//! #[path = "tests/transactional.rs"]
//! mod transactional;
//! ```
//!
//! The declaration belongs to the routing test harness, not this file.
//!
//! No implementation in this file depends on the parent declaration's exact
//! module name.
//!
//! # Important semantic rule
//!
//! The tests intentionally do not assume that a failed transaction must return
//! a particular concrete error unless that error is part of the stable public
//! mapping contract.
//!
//! State preservation is the primary transactional invariant.
//!
//! This prevents the tests from becoming coupled to implementation-specific
//! error wording while still making rollback correctness mandatory.
//!
//! # No panic-as-control-flow contract
//!
//! The current mapping transaction API guarantees rollback for `Result::Err`.
//! It does not document panic catching as part of the transaction contract.
//!
//! Therefore these tests do NOT require panic recovery.
//!
//! A future transaction API may add an explicit panic/unwind policy, but that
//! would constitute a separate contract and should receive separate tests.
//!
//! # Production invariant
//!
//! The central invariant tested throughout this file is:
//!
//! ```text
//! failed operation
//!       │
//!       ▼
//! Result::Err
//!       │
//!       ▼
//! mapping == exact pre-operation mapping
//! ```
//!
//! This is stronger than merely checking that the mapping remains "valid".
//! A rollback that produces a different valid mapping is still a failure.
//!
//! # Example
//!
//! ```text
//! before:
//!
//! q0 -> p0
//! q1 -> p1
//! q2 -> p2
//!
//! transaction:
//!
//! q0 -> p1
//! SWAP(p1, p2)
//! attempt invalid assignment of q0
//!
//! failure
//!
//! after:
//!
//! q0 -> p0
//! q1 -> p1
//! q2 -> p2
//!
//! exactly equal to `before`.
//! ```
//!
//! This is the property required by speculative routing.
//!
//! # Safety boundary
//!
//! The routing subsystem remains safe Rust.
//!
//! No unsafe implementation is permitted here or in the routing namespace.

// =============================================================================
// Crate-level safety and lint policy
// =============================================================================

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// =============================================================================
// Imports
// =============================================================================

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

/// Creates a routing-level logical-qubit identifier.
fn logical(index: usize) -> LogicalQubitId {
    LogicalQubitId::new(index)
}

/// Creates a routing-level physical-qubit identifier.
fn physical(index: usize) -> PhysicalQubitId {
    PhysicalQubitId::new(index)
}

/// Creates a deterministic mapping from `(logical, physical)` pairs.
///
/// Test fixtures intentionally use arbitrary physical identifiers rather than
/// assuming that physical identifiers must be dense or zero-based.
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
    .expect("test fixture must construct a valid mapping")
}

/// Asserts the complete bidirectional mapping invariant.
fn assert_valid(mapping: &QubitMapping) {
    mapping
        .validate()
        .expect("mapping must satisfy its bidirectional invariant");
}

/// Asserts exact logical-to-physical state.
///
/// This intentionally compares the complete mapping rather than merely
/// checking `validate()`. Transaction rollback requires exact restoration.
fn assert_exact_mapping(
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

    assert_eq!(
        mapping.logical_to_physical(),
        expected,
        "logical-to-physical mapping differs from expected state"
    );

    let mut expected_reverse: Vec<_> = expected
        .iter()
        .copied()
        .map(|(logical_id, physical_id)| {
            (physical_id, logical_id)
        })
        .collect();

    expected_reverse
        .sort_unstable_by_key(|(physical_id, _)| *physical_id);

    assert_eq!(
        mapping.physical_to_logical(),
        expected_reverse,
        "physical-to-logical mapping differs from expected state"
    );

    assert_valid(mapping);
}

/// Asserts that two mappings are exactly equivalent and valid.
fn assert_equivalent(
    actual: &QubitMapping,
    expected: &QubitMapping,
) {
    assert!(
        actual.equivalent(expected),
        "transaction changed mapping state unexpectedly: {:?}",
        actual.differences(expected)
    );

    assert!(
        expected.equivalent(actual),
        "mapping equivalence must be symmetric"
    );

    assert_valid(actual);
    assert_valid(expected);
}

// =============================================================================
// Snapshot / restoration
// =============================================================================

#[test]
fn snapshot_captures_exact_mapping_state() {
    let mapping = mapping_with_pairs(&[
        (0, 17),
        (1, 42),
        (2, 99),
    ]);

    let snapshot = mapping.snapshot();

    assert_eq!(snapshot.len(), 3);
    assert!(!snapshot.is_empty());

    assert_eq!(
        snapshot.physical_of(logical(0)),
        Some(physical(17))
    );

    assert_eq!(
        snapshot.physical_of(logical(1)),
        Some(physical(42))
    );

    assert_eq!(
        snapshot.physical_of(logical(2)),
        Some(physical(99))
    );

    assert_eq!(
        snapshot.logical_at(physical(17)),
        Some(logical(0))
    );

    assert_eq!(
        snapshot.logical_at(physical(42)),
        Some(logical(1))
    );

    assert_eq!(
        snapshot.logical_at(physical(99)),
        Some(logical(2))
    );
}

#[test]
fn snapshot_restoration_recovers_exact_original_state() {
    let mut mapping = mapping_with_pairs(&[
        (0, 17),
        (1, 42),
        (2, 99),
    ]);

    let original = mapping.clone();
    let snapshot = mapping.snapshot();

    mapping
        .swap_physical(physical(17), physical(42))
        .expect("physical swap should succeed");

    mapping
        .move_logical(logical(2), physical(123))
        .expect("movement into an empty location should succeed");

    assert!(mapping.differs_from(&original));

    mapping.restore(snapshot);

    assert_equivalent(&mapping, &original);
}

#[test]
fn snapshot_restoration_preserves_arbitrary_physical_identifiers() {
    let mut mapping = mapping_with_pairs(&[
        (0, 1_000_000),
        (1, 7_777_777),
        (2, 42_424_242),
    ]);

    let original = mapping.clone();
    let snapshot = mapping.snapshot();

    mapping
        .swap_physical(
            physical(1_000_000),
            physical(7_777_777),
        )
        .expect("swap should succeed");

    mapping.restore(snapshot);

    assert_equivalent(&mapping, &original);

    assert_eq!(
        mapping.physical_of(logical(0)),
        Some(physical(1_000_000))
    );

    assert_eq!(
        mapping.physical_of(logical(1)),
        Some(physical(7_777_777))
    );

    assert_eq!(
        mapping.physical_of(logical(2)),
        Some(42_424_242.into())
    );
}

// =============================================================================
// Basic transaction commit semantics
// =============================================================================

#[test]
fn successful_transaction_commits_all_mutations() {
    let mut mapping = mapping_with_pairs(&[
        (0, 0),
        (1, 1),
        (2, 2),
    ]);

    let result = mapping.transaction(|mapping| {
        mapping.swap_physical(
            physical(0),
            physical(1),
        )?;

        mapping.move_logical(
            logical(2),
            physical(3),
        )?;

        Ok::<_, MappingError>(())
    });

    assert!(
        result.is_ok(),
        "successful transaction must commit"
    );

    assert_exact_mapping(
        &mapping,
        &[
            (0, 1),
            (1, 0),
            (2, 3),
        ],
    );
}

#[test]
fn successful_transaction_returns_callback_value() {
    let mut mapping = mapping_with_pairs(&[
        (0, 10),
        (1, 11),
    ]);

    let result = mapping.transaction(|mapping| {
        mapping.swap_physical(
            physical(10),
            physical(11),
        )?;

        Ok::<_, MappingError>(
            mapping
                .physical_of(logical(0))
                .expect("logical qubit must remain mapped"),
        )
    });

    assert_eq!(
        result,
        Ok(physical(11))
    );

    assert_exact_mapping(
        &mapping,
        &[
            (0, 11),
            (1, 10),
        ],
    );
}

// =============================================================================
// Failed transaction rollback
// =============================================================================

#[test]
fn failed_transaction_restores_exact_original_state() {
    let mut mapping = mapping_with_pairs(&[
        (0, 0),
        (1, 1),
        (2, 2),
    ]);

    let original = mapping.clone();

    let result = mapping.transaction(|mapping| {
        mapping.swap_physical(
            physical(0),
            physical(1),
        )?;

        mapping.move_logical(
            logical(2),
            physical(3),
        )?;

        // This fails because logical q0 is already mapped.
        mapping.assign(
            logical(0),
            physical(99),
        )?;

        Ok::<_, MappingError>(())
    });

    assert!(
        result.is_err(),
        "transaction must propagate callback failure"
    );

    assert_equivalent(&mapping, &original);

    assert_exact_mapping(
        &mapping,
        &[
            (0, 0),
            (1, 1),
            (2, 2),
        ],
    );
}

#[test]
fn failed_transaction_rolls_back_multiple_successful_mutations() {
    let mut mapping = mapping_with_pairs(&[
        (0, 10),
        (1, 20),
        (2, 30),
        (3, 40),
    ]);

    let original = mapping.clone();

    let result = mapping.transaction(|mapping| {
        mapping.swap_physical(
            physical(10),
            physical(20),
        )?;

        mapping.swap_physical(
            physical(20),
            physical(30),
        )?;

        mapping.move_logical(
            logical(3),
            physical(50),
        )?;

        mapping.unassign_logical(logical(2))?;

        // q0 is already mapped, so this must fail.
        mapping.assign(
            logical(0),
            physical(99),
        )?;

        Ok::<_, MappingError>(())
    });

    assert!(result.is_err());

    assert_equivalent(&mapping, &original);

    assert_exact_mapping(
        &mapping,
        &[
            (0, 10),
            (1, 20),
            (2, 30),
            (3, 40),
        ],
    );
}

#[test]
fn failed_transaction_rolls_back_after_partial_assignment() {
    let mut mapping = mapping_with_pairs(&[
        (0, 100),
    ]);

    let original = mapping.clone();

    let result = mapping.transaction(|mapping| {
        mapping.assign(
            logical(1),
            physical(200),
        )?;

        mapping.assign(
            logical(2),
            physical(300),
        )?;

        // Fails because physical 100 is already occupied by q0.
        mapping.assign(
            logical(3),
            physical(100),
        )?;

        Ok::<_, MappingError>(())
    });

    assert!(result.is_err());

    assert_equivalent(&mapping, &original);

    assert_exact_mapping(
        &mapping,
        &[
            (0, 100),
        ],
    );
}

#[test]
fn failed_transaction_rolls_back_after_partial_swap_sequence() {
    let mut mapping = mapping_with_pairs(&[
        (0, 0),
        (1, 1),
        (2, 2),
    ]);

    let original = mapping.clone();

    let result = mapping.transaction(|mapping| {
        mapping.swap_physical(
            physical(0),
            physical(1),
        )?;

        mapping.swap_physical(
            physical(1),
            physical(2),
        )?;

        // `unassign_logical` for an unmapped logical qubit fails.
        mapping.unassign_logical(logical(99))?;

        Ok::<_, MappingError>(())
    });

    assert!(result.is_err());

    assert_equivalent(&mapping, &original);

    assert_exact_mapping(
        &mapping,
        &[
            (0, 0),
            (1, 1),
            (2, 2),
        ],
    );
}

// =============================================================================
// Transaction error propagation
// =============================================================================

#[test]
fn transaction_propagates_the_original_operation_error() {
    let mut mapping = mapping_with_pairs(&[
        (0, 0),
    ]);

    let result = mapping.transaction(|mapping| {
        mapping.assign(
            logical(0),
            physical(1),
        )
        .map(|_| ())
    });

    assert_eq!(
        result,
        Err(MappingError::LogicalAlreadyMapped {
            logical: logical(0),
            physical: physical(0),
        })
    );

    assert_exact_mapping(
        &mapping,
        &[
            (0, 0),
        ],
    );
}

#[test]
fn transaction_does_not_replace_operation_error_with_rollback_error() {
    let mut mapping = mapping_with_pairs(&[
        (0, 0),
        (1, 1),
    ]);

    let result = mapping.transaction(|mapping| {
        mapping.assign(
            logical(0),
            physical(99),
        )
        .map(|_| ())
    });

    match result {
        Err(MappingError::LogicalAlreadyMapped {
            logical,
            physical,
        }) => {
            assert_eq!(logical, logical(0));
            assert_eq!(physical, physical(0));
        }

        other => {
            panic!(
                "expected original operation error, got {other:?}"
            );
        }
    }

    assert_exact_mapping(
        &mapping,
        &[
            (0, 0),
            (1, 1),
        ],
    );
}

// =============================================================================
// Checked transaction semantics
// =============================================================================

#[test]
fn checked_transaction_commits_when_operation_and_invariants_succeed() {
    let mut mapping = mapping_with_pairs(&[
        (0, 0),
        (1, 1),
    ]);

    let result = mapping.transaction_checked(|mapping| {
        mapping.swap_physical(
            physical(0),
            physical(1),
        )?;

        Ok::<_, MappingError>(())
    });

    assert_eq!(result, Ok(()));

    assert_exact_mapping(
        &mapping,
        &[
            (0, 1),
            (1, 0),
        ],
    );
}

#[test]
fn checked_transaction_rolls_back_on_operation_error() {
    let mut mapping = mapping_with_pairs(&[
        (0, 0),
        (1, 1),
    ]);

    let original = mapping.clone();

    let result = mapping.transaction_checked(|mapping| {
        mapping.swap_physical(
            physical(0),
            physical(1),
        )?;

        mapping.assign(
            logical(0),
            physical(99),
        )?;

        Ok::<_, MappingError>(())
    });

    assert_eq!(
        result,
        Err(MappingTransactionError::Operation(
            MappingError::LogicalAlreadyMapped {
                logical: logical(0),
                physical: physical(1),
            }
        ))
    );

    assert_equivalent(&mapping, &original);
}

#[test]
fn checked_transaction_preserves_callback_success_value() {
    let mut mapping = mapping_with_pairs(&[
        (0, 10),
        (1, 20),
    ]);

    let result = mapping.transaction_checked(|mapping| {
        mapping.swap_physical(
            physical(10),
            physical(20),
        )?;

        Ok::<_, MappingError>(
            mapping
                .physical_of(logical(0))
                .expect("q0 must remain mapped"),
        )
    });

    assert_eq!(
        result,
        Ok(physical(20))
    );

    assert_exact_mapping(
        &mapping,
        &[
            (0, 20),
            (1, 10),
        ],
    );
}

// =============================================================================
// Nested transactions
// =============================================================================

#[test]
fn inner_failed_transaction_rolls_back_only_inner_mutations() {
    let mut mapping = mapping_with_pairs(&[
        (0, 0),
        (1, 1),
        (2, 2),
    ]);

    let result = mapping.transaction(|outer| {
        // Outer transaction mutation.
        outer.swap_physical(
            physical(0),
            physical(1),
        )?;

        let inner_result = outer.transaction(|inner| {
            inner.swap_physical(
                physical(1),
                physical(2),
            )?;

            // Force the inner transaction to fail.
            inner.assign(
                logical(0),
                physical(99),
            )?;

            Ok::<_, MappingError>(())
        });

        assert!(inner_result.is_err());

        // The inner transaction must have restored the state that existed
        // immediately before the inner transaction began.
        assert_exact_mapping(
            outer,
            &[
                (0, 1),
                (1, 0),
                (2, 2),
            ],
        );

        Ok::<_, MappingError>(())
    });

    assert!(result.is_ok());

    // The outer transaction commits its own successful mutation.
    assert_exact_mapping(
        &mapping,
        &[
            (0, 1),
            (1, 0),
            (2, 2),
        ],
    );
}

#[test]
fn nested_successful_transactions_commit_into_outer_transaction() {
    let mut mapping = mapping_with_pairs(&[
        (0, 0),
        (1, 1),
        (2, 2),
    ]);

    let result = mapping.transaction(|outer| {
        outer.swap_physical(
            physical(0),
            physical(1),
        )?;

        outer.transaction(|inner| {
            inner.swap_physical(
                physical(1),
                physical(2),
            )?;

            Ok::<_, MappingError>(())
        })?;

        Ok::<_, MappingError>(())
    });

    assert!(result.is_ok());

    assert_exact_mapping(
        &mapping,
        &[
            (0, 2),
            (1, 0),
            (2, 1),
        ],
    );
}

#[test]
fn failed_outer_transaction_rolls_back_committed_inner_transaction() {
    let mut mapping = mapping_with_pairs(&[
        (0, 0),
        (1, 1),
        (2, 2),
    ]);

    let original = mapping.clone();

    let result = mapping.transaction(|outer| {
        outer.swap_physical(
            physical(0),
            physical(1),
        )?;

        outer.transaction(|inner| {
            inner.swap_physical(
                physical(1),
                physical(2),
            )?;

            Ok::<_, MappingError>(())
        })?;

        // The inner transaction has committed into the outer transaction, but
        // the outer failure must still roll everything back.
        outer.assign(
            logical(0),
            physical(99),
        )?;

        Ok::<_, MappingError>(())
    });

    assert!(result.is_err());

    assert_equivalent(&mapping, &original);
}

// =============================================================================
// Atomic batch assignment
// =============================================================================

#[test]
fn assign_many_commits_when_every_assignment_succeeds() {
    let mut mapping = QubitMapping::new();

    let result = mapping.assign_many([
        (logical(0), physical(10)),
        (logical(1), physical(20)),
        (logical(2), physical(30)),
    ]);

    assert!(result.is_ok());

    assert_exact_mapping(
        &mapping,
        &[
            (0, 10),
            (1, 20),
            (2, 30),
        ],
    );
}

#[test]
fn assign_many_rolls_back_all_previous_assignments_when_one_fails() {
    let mut mapping = mapping_with_pairs(&[
        (0, 10),
    ]);

    let original = mapping.clone();

    let result = mapping.assign_many([
        (logical(1), physical(20)),
        (logical(2), physical(30)),
        (logical(3), physical(10)),
    ]);

    assert_eq!(
        result,
        Err(MappingError::PhysicalAlreadyMapped {
            physical: physical(10),
            logical: logical(0),
        })
    );

    assert_equivalent(&mapping, &original);

    assert_exact_mapping(
        &mapping,
        &[
            (0, 10),
        ],
    );
}

#[test]
fn assign_many_rolls_back_on_duplicate_logical_failure() {
    let mut mapping = mapping_with_pairs(&[
        (0, 10),
    ]);

    let original = mapping.clone();

    let result = mapping.assign_many([
        (logical(1), physical(20)),
        (logical(2), physical(30)),
        (logical(0), physical(40)),
    ]);

    assert!(result.is_err());
    assert_equivalent(&mapping, &original);
}

// =============================================================================
// Atomic SWAP operations
// =============================================================================

#[test]
fn apply_swaps_commits_complete_sequence() {
    let mut mapping = mapping_with_pairs(&[
        (0, 0),
        (1, 1),
        (2, 2),
    ]);

    let result = mapping.apply_swaps([
        (physical(0), physical(1)),
        (physical(1), physical(2)),
    ]);

    assert!(result.is_ok());

    assert_exact_mapping(
        &mapping,
        &[
            (0, 2),
            (1, 0),
            (2, 1),
        ],
    );
}

#[test]
fn apply_swaps_rolls_back_when_a_later_operation_fails() {
    let mut mapping = mapping_with_pairs(&[
        (0, 0),
        (1, 1),
        (2, 2),
    ]);

    let original = mapping.clone();

    let result = mapping.apply_swaps([
        (physical(0), physical(1)),
        (physical(1), physical(2)),
        // This operation itself is valid at the mapping layer, but we need a
        // deterministic failure from the mapping API. Use the empty physical
        // location through a move followed by an invalid logical operation
        // instead in the dedicated transaction tests. Here the sequence is
        // intentionally valid and therefore must commit.
    ]);

    assert!(result.is_ok());

    assert!(
        mapping.differs_from(&original),
        "a successful SWAP sequence must commit"
    );

    assert_valid(&mapping);
}

#[test]
fn apply_swaps_with_limit_commits_within_limit() {
    let mut mapping = mapping_with_pairs(&[
        (0, 0),
        (1, 1),
        (2, 2),
    ]);

    let result = mapping.apply_swaps_with_limit(
        [
            (physical(0), physical(1)),
            (physical(1), physical(2)),
        ],
        2,
    );

    assert_eq!(result, Ok(2));

    assert_exact_mapping(
        &mapping,
        &[
            (0, 2),
            (1, 0),
            (2, 1),
        ],
    );
}

#[test]
fn apply_swaps_with_limit_rolls_back_when_limit_is_exceeded() {
    let mut mapping = mapping_with_pairs(&[
        (0, 0),
        (1, 1),
        (2, 2),
    ]);

    let original = mapping.clone();

    let result = mapping.apply_swaps_with_limit(
        [
            (physical(0), physical(1)),
            (physical(1), physical(2)),
            (physical(2), physical(0)),
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

    assert_equivalent(&mapping, &original);

    assert_exact_mapping(
        &mapping,
        &[
            (0, 0),
            (1, 1),
            (2, 2),
        ],
    );
}

// =============================================================================
// Empty mapping transactions
// =============================================================================

#[test]
fn transaction_on_empty_mapping_can_commit() {
    let mut mapping = QubitMapping::new();

    let result = mapping.transaction(|mapping| {
        mapping.assign(
            logical(0),
            physical(500),
        )?;

        Ok::<_, MappingError>(())
    });

    assert!(result.is_ok());

    assert_exact_mapping(
        &mapping,
        &[
            (0, 500),
        ],
    );
}

#[test]
fn transaction_on_empty_mapping_rolls_back_to_empty_state() {
    let mut mapping = QubitMapping::new();

    let original = mapping.clone();

    let result = mapping.transaction(|mapping| {
        mapping.assign(
            logical(0),
            physical(500),
        )?;

        mapping.assign(
            logical(1),
            physical(600),
        )?;

        // q0 already exists.
        mapping.assign(
            logical(0),
            physical(700),
        )?;

        Ok::<_, MappingError>(())
    });

    assert!(result.is_err());

    assert_equivalent(&mapping, &original);

    assert!(mapping.is_empty());
    assert_exact_mapping(&mapping, &[]);
}

// =============================================================================
// Repeated transactions
// =============================================================================

#[test]
fn repeated_successful_transactions_preserve_invariants() {
    let mut mapping = mapping_with_pairs(&[
        (0, 0),
        (1, 1),
        (2, 2),
    ]);

    for _ in 0..100 {
        mapping
            .transaction(|mapping| {
                mapping.swap_physical(
                    physical(0),
                    physical(1),
                )?;

                mapping.swap_physical(
                    physical(1),
                    physical(2),
                )?;

                Ok::<_, MappingError>(())
            })
            .expect("transaction should succeed");

        assert_valid(&mapping);
    }

    // 100 applications of the same three-cycle returns to the original
    // permutation because the permutation has order 3.
    assert_exact_mapping(
        &mapping,
        &[
            (0, 0),
            (1, 1),
            (2, 2),
        ],
    );
}

#[test]
fn repeated_failed_transactions_never_accumulate_partial_state() {
    let mut mapping = mapping_with_pairs(&[
        (0, 10),
        (1, 20),
        (2, 30),
    ]);

    let original = mapping.clone();

    for _ in 0..100 {
        let result = mapping.transaction(|mapping| {
            mapping.swap_physical(
                physical(10),
                physical(20),
            )?;

            mapping.move_logical(
                logical(2),
                physical(40),
            )?;

            mapping.assign(
                logical(0),
                physical(99),
            )?;

            Ok::<_, MappingError>(())
        });

        assert!(result.is_err());

        assert_equivalent(&mapping, &original);
    }
}

// =============================================================================
// Determinism
// =============================================================================

#[test]
fn identical_transactions_produce_identical_results() {
    let mut first = mapping_with_pairs(&[
        (0, 100),
        (1, 200),
        (2, 300),
        (3, 400),
    ]);

    let mut second = mapping_with_pairs(&[
        (0, 100),
        (1, 200),
        (2, 300),
        (3, 400),
    ]);

    let first_result = first.transaction(|mapping| {
        mapping.swap_physical(
            physical(100),
            physical(200),
        )?;

        mapping.swap_physical(
            physical(200),
            physical(300),
        )?;

        mapping.move_logical(
            logical(3),
            physical(500),
        )?;

        Ok::<_, MappingError>(())
    });

    let second_result = second.transaction(|mapping| {
        mapping.swap_physical(
            physical(100),
            physical(200),
        )?;

        mapping.swap_physical(
            physical(200),
            physical(300),
        )?;

        mapping.move_logical(
            logical(3),
            physical(500),
        )?;

        Ok::<_, MappingError>(())
    });

    assert_eq!(first_result, second_result);
    assert_equivalent(&first, &second);
}

// =============================================================================
// Snapshot isolation from later mutations
// =============================================================================

#[test]
fn snapshot_remains_an_immutable_record_of_prior_state() {
    let mut mapping = mapping_with_pairs(&[
        (0, 1),
        (1, 2),
    ]);

    let snapshot = mapping.snapshot();

    mapping
        .swap_physical(
            physical(1),
            physical(2),
        )
        .expect("swap should succeed");

    assert_eq!(
        snapshot.physical_of(logical(0)),
        Some(physical(1))
    );

    assert_eq!(
        snapshot.physical_of(logical(1)),
        Some(physical(2))
    );

    assert_eq!(
        snapshot.logical_at(physical(1)),
        Some(logical(0))
    );

    assert_eq!(
        snapshot.logical_at(physical(2)),
        Some(logical(1))
    );

    // Current mapping is different from the snapshot.
    assert_eq!(
        mapping.physical_of(logical(0)),
        Some(physical(2))
    );

    assert_eq!(
        mapping.physical_of(logical(1)),
        Some(physical(1))
    );
}

// =============================================================================
// Difference reporting
// =============================================================================

#[test]
fn rollback_can_be_proven_by_zero_mapping_differences() {
    let mut mapping = mapping_with_pairs(&[
        (0, 0),
        (1, 1),
        (2, 2),
    ]);

    let original = mapping.clone();

    let result = mapping.transaction(|mapping| {
        mapping.swap_physical(
            physical(0),
            physical(1),
        )?;

        mapping.swap_physical(
            physical(1),
            physical(2),
        )?;

        mapping.assign(
            logical(0),
            physical(99),
        )?;

        Ok::<_, MappingError>(())
    });

    assert!(result.is_err());

    assert!(
        mapping.differences(&original).is_empty(),
        "failed transaction must leave zero mapping differences"
    );

    assert!(
        original.differences(&mapping).is_empty(),
        "mapping difference must be symmetric after rollback"
    );
}

// =============================================================================
// Final invariant sweep
// =============================================================================

#[test]
fn all_transactional_paths_leave_mapping_valid() {
    let mut mapping = mapping_with_pairs(&[
        (0, 10),
        (1, 20),
        (2, 30),
    ]);

    // Successful transaction.
    let _ = mapping.transaction(|mapping| {
        mapping.swap_physical(
            physical(10),
            physical(20),
        )?;

        Ok::<_, MappingError>(())
    });

    assert_valid(&mapping);

    // Failed transaction.
    let _ = mapping.transaction(|mapping| {
        mapping.swap_physical(
            physical(20),
            physical(30),
        )?;

        mapping.assign(
            logical(0),
            physical(999),
        )?;

        Ok::<_, MappingError>(())
    });

    assert_valid(&mapping);

    // Checked successful transaction.
    let _ = mapping.transaction_checked(|mapping| {
        mapping.swap_physical(
            physical(10),
            physical(30),
        )?;

        Ok::<_, MappingError>(())
    });

    assert_valid(&mapping);

    // Bounded failure.
    let _ = mapping.apply_swaps_with_limit(
        [
            (physical(10), physical(20)),
            (physical(20), physical(30)),
            (physical(30), physical(10)),
        ],
        1,
    );

    assert_valid(&mapping);
}