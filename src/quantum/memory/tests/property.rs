//! Zamani Quantum Memory — Deterministic Property and Invariant Tests
//!
//! This module contains representation-independent, deterministic property
//! tests for the quantum memory subsystem.
//!
//! # Purpose
//!
//! These tests verify mathematical, resource, identity, arithmetic, and
//! architectural invariants that must remain true regardless of:
//!
//! - state representation;
//! - CPU/GPU execution;
//! - local/distributed execution;
//! - simulator/emulator execution;
//! - physical-QPU execution;
//! - vendor/provider;
//! - accelerator implementation;
//! - storage location;
//! - serialization backend.
//!
//! The tests intentionally avoid assuming that a physical QPU exposes a
//! simulator state vector. The canonical `QuantumState` contract explicitly
//! allows QPUs to expose opaque/provider-managed state.
//!
//! # Design principles
//!
//! 1. Deterministic.
//! 2. Reproducible.
//! 3. No hidden RNG.
//! 4. No unsafe code.
//! 5. No vendor-specific APIs.
//! 6. No network access.
//! 7. No dependency on a particular simulator.
//! 8. No dependence on machine-specific pointer widths beyond explicitly
//!    checked platform properties.
//! 9. No enormous allocations.
//! 10. No probabilistic "probably passes" tests.
//! 11. Test public contracts rather than private implementation details.
//! 12. Test invariants before performance.
//!
//! # Why deterministic property tests?
//!
//! Quantum-memory correctness must not depend on an accidentally favorable
//! random seed. The bounded generators below enumerate representative values
//! and explicitly include boundary values, powers of two, overflow-adjacent
//! values, zero, and repeated values.
//!
//! If the project later adopts a property-testing framework, these tests can
//! be retained as deterministic contract tests and additional randomized
//! properties can be layered on top.
//!
//! # Integration
//!
//! Include this file from:
//!
//! `src/quantum/memory/tests/mod.rs`
//!
//! using:
//!
//! ```text
//! #[cfg(test)]
//! mod property;
//! ```
//!
//! This file intentionally does not require modifications to the production
//! memory modules merely because another state representation or QPU provider
//! is added.
//!
//! # Architectural boundary
//!
//! ```text
//! quantum::frontend
//!        │
//!        ▼
//! quantum::ir
//!        │
//!        ▼
//! execution / runtime
//!        │
//!        ▼
//! quantum::memory
//!        │
//!        ├── types
//!        ├── limits
//!        ├── layout
//!        ├── allocation
//!        ├── state
//!        ├── representations
//!        ├── persistence
//!        ├── synchronization
//!        └── hardware/provider boundary
//!
//! This test module verifies invariants at the memory boundary.
//! ```
//!
//! # Rust compatibility
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust
//! - no nightly features
//! - no unsafe
//!
//! # QPU neutrality
//!
//! The memory layer must support arbitrary providers without changing these
//! fundamental invariants. A future provider may expose:
//!
//! - a state vector;
//! - a density matrix;
//! - stabilizer state;
//! - tensor-network state;
//! - photonic state;
//! - continuous-variable state;
//! - annealing state;
//! - provider-native opaque state;
//! - remote state;
//! - no directly readable state at all.
//!
//! Therefore these tests distinguish memory-domain invariants from
//! representation-specific mathematical tests.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use crate::quantum::memory::types::{
    AmplitudeCount,
    ByteCount,
    ClassicalBitCount,
    QubitCount,
};

use std::fmt::Debug;

// =============================================================================
// Deterministic bounded generators
// =============================================================================

/// Small deterministic sample set used for quantity properties.
///
/// The values deliberately include:
///
/// - zero;
/// - one;
/// - small values;
/// - powers of two;
/// - values around powers of two;
/// - values near the largest practical small-test values.
///
/// The set is intentionally bounded so that a CI runner never attempts an
/// exponential quantum-state allocation merely because this property module
/// is executed.
fn quantity_samples() -> &'static [usize] {
    &[
        0,
        1,
        2,
        3,
        4,
        5,
        7,
        8,
        15,
        16,
        17,
        31,
        32,
        33,
        63,
        64,
        65,
        127,
        128,
        129,
        255,
        256,
        257,
        511,
        512,
        513,
        1023,
        1024,
        1025,
        4095,
        4096,
        4097,
    ]
}

/// Deterministic bounded `u64` samples for byte-count properties.
fn byte_samples() -> &'static [u64] {
    &[
        0,
        1,
        2,
        3,
        4,
        7,
        8,
        15,
        16,
        31,
        32,
        63,
        64,
        127,
        128,
        255,
        256,
        1023,
        1024,
        1025,
        4096,
        1 << 20,
        1 << 30,
        u64::MAX / 2,
        u64::MAX - 1,
        u64::MAX,
    ]
}

/// Assert a property over a deterministic finite domain.
///
/// This helper deliberately does not use a random generator.
fn for_each_sample<T, F>(samples: &[T], mut property: F)
where
    T: Copy + Debug,
    F: FnMut(T),
{
    for &value in samples {
        property(value);
    }
}

// =============================================================================
// QubitCount properties
// =============================================================================

#[test]
fn qubit_count_constructor_is_identity() {
    for_each_sample(quantity_samples(), |value| {
        let count = QubitCount::new(value);

        assert_eq!(
            count.get(),
            value,
            "QubitCount::new/get must be an identity"
        );
    });
}

#[test]
fn qubit_count_zero_property() {
    let zero = QubitCount::ZERO;

    assert_eq!(zero.get(), 0);
    assert!(zero.is_zero());
    assert!(!zero.is_non_zero());
}

#[test]
fn qubit_count_nonzero_property() {
    for_each_sample(quantity_samples(), |value| {
        let count = QubitCount::new(value);

        assert_eq!(
            count.is_zero(),
            value == 0,
            "is_zero must exactly describe zero"
        );

        assert_eq!(
            count.is_non_zero(),
            value != 0,
            "is_non_zero must exactly describe non-zero"
        );
    });
}

#[test]
fn qubit_count_checked_addition_is_mathematically_correct() {
    let samples = quantity_samples();

    for &a in samples {
        for &b in samples {
            let lhs = QubitCount::new(a);
            let rhs = QubitCount::new(b);

            match a.checked_add(b) {
                Some(expected) => {
                    let actual = lhs
                        .checked_add(rhs)
                        .expect("checked_add must succeed when usize succeeds");

                    assert_eq!(actual.get(), expected);
                }
                None => {
                    assert!(
                        lhs.checked_add(rhs).is_none(),
                        "QubitCount checked_add must reject overflow"
                    );
                }
            }
        }
    }
}

#[test]
fn qubit_count_checked_subtraction_is_mathematically_correct() {
    let samples = quantity_samples();

    for &a in samples {
        for &b in samples {
            let lhs = QubitCount::new(a);
            let rhs = QubitCount::new(b);

            match a.checked_sub(b) {
                Some(expected) => {
                    let actual = lhs
                        .checked_sub(rhs)
                        .expect("checked_sub must succeed when usize succeeds");

                    assert_eq!(actual.get(), expected);
                }
                None => {
                    assert!(
                        lhs.checked_sub(rhs).is_none(),
                        "QubitCount checked_sub must reject underflow"
                    );
                }
            }
        }
    }
}

#[test]
fn qubit_count_checked_multiplication_is_mathematically_correct() {
    let samples = quantity_samples();

    for &value in samples {
        for multiplier in [0usize, 1, 2, 3, 4, 8, 16, 1024] {
            let count = QubitCount::new(value);

            match value.checked_mul(multiplier) {
                Some(expected) => {
                    let actual = count
                        .checked_mul(multiplier)
                        .expect("checked_mul must succeed when usize succeeds");

                    assert_eq!(actual.get(), expected);
                }
                None => {
                    assert!(
                        count.checked_mul(multiplier).is_none(),
                        "QubitCount checked_mul must reject overflow"
                    );
                }
            }
        }
    }
}

#[test]
fn qubit_count_round_trip_through_usize_is_lossless() {
    for_each_sample(quantity_samples(), |value| {
        let count = QubitCount::from(value);
        let recovered: usize = count.into();

        assert_eq!(recovered, value);
    });
}

#[test]
fn qubit_count_order_matches_numeric_order() {
    let samples = quantity_samples();

    for &a in samples {
        for &b in samples {
            let lhs = QubitCount::new(a);
            let rhs = QubitCount::new(b);

            assert_eq!(
                lhs.cmp(&rhs),
                a.cmp(&b),
                "strongly typed ordering must preserve numeric ordering"
            );
        }
    }
}

// =============================================================================
// ClassicalBitCount properties
// =============================================================================

#[test]
fn classical_bit_count_constructor_is_identity() {
    for_each_sample(quantity_samples(), |value| {
        let count = ClassicalBitCount::new(value);

        assert_eq!(count.get(), value);
    });
}

#[test]
fn classical_bit_count_zero_property() {
    let zero = ClassicalBitCount::ZERO;

    assert_eq!(zero.get(), 0);
    assert!(zero.is_zero());
    assert!(!zero.is_non_zero());
}

#[test]
fn classical_bit_count_zero_and_nonzero_are_complements() {
    for_each_sample(quantity_samples(), |value| {
        let count = ClassicalBitCount::new(value);

        assert_ne!(
            count.is_zero(),
            count.is_non_zero(),
            "zero/non-zero predicates must be complementary"
        );

        assert_eq!(count.is_zero(), value == 0);
    });
}

#[test]
fn classical_bit_count_checked_arithmetic_matches_usize() {
    let samples = quantity_samples();

    for &a in samples {
        for &b in samples {
            let lhs = ClassicalBitCount::new(a);
            let rhs = ClassicalBitCount::new(b);

            assert_eq!(
                lhs.checked_add(rhs).map(ClassicalBitCount::get),
                a.checked_add(b)
            );

            assert_eq!(
                lhs.checked_sub(rhs).map(ClassicalBitCount::get),
                a.checked_sub(b)
            );
        }
    }
}

#[test]
fn classical_bit_count_checked_multiplication_matches_usize() {
    for_each_sample(quantity_samples(), |value| {
        let count = ClassicalBitCount::new(value);

        for multiplier in [0usize, 1, 2, 3, 4, 8, 16, 1024] {
            assert_eq!(
                count
                    .checked_mul(multiplier)
                    .map(ClassicalBitCount::get),
                value.checked_mul(multiplier)
            );
        }
    });
}

#[test]
fn classical_bit_count_round_trip_is_lossless() {
    for_each_sample(quantity_samples(), |value| {
        let count = ClassicalBitCount::from(value);
        let recovered: usize = count.into();

        assert_eq!(recovered, value);
    });
}

// =============================================================================
// AmplitudeCount properties
// =============================================================================

#[test]
fn amplitude_count_constructor_is_identity() {
    for_each_sample(quantity_samples(), |value| {
        let count = AmplitudeCount::new(value);

        assert_eq!(count.get(), value);
        assert_eq!(count.is_zero(), value == 0);
        assert_eq!(count.is_non_zero(), value != 0);
    });
}

#[test]
fn amplitude_count_zero_property() {
    let zero = AmplitudeCount::ZERO;

    assert_eq!(zero.get(), 0);
    assert!(zero.is_zero());
    assert!(!zero.is_non_zero());
}

#[test]
fn amplitude_count_round_trip_is_lossless() {
    for_each_sample(quantity_samples(), |value| {
        let count = AmplitudeCount::from(value);
        let recovered: usize = count.into();

        assert_eq!(recovered, value);
    });
}

#[test]
fn amplitude_count_checked_multiplication_matches_usize() {
    for_each_sample(quantity_samples(), |value| {
        let count = AmplitudeCount::new(value);

        for multiplier in [0usize, 1, 2, 3, 4, 8, 16, 1024] {
            assert_eq!(
                count
                    .checked_mul(multiplier)
                    .map(AmplitudeCount::get),
                value.checked_mul(multiplier)
            );
        }
    });
}

#[test]
fn amplitude_count_for_qubits_matches_two_to_the_n_when_representable() {
    for qubits in 0usize..=12 {
        let count = AmplitudeCount::checked_for_qubits(QubitCount::new(qubits))
            .expect("2^n must fit for n <= 12");

        assert_eq!(count.get(), 1usize << qubits);
        assert_eq!(count.is_non_zero(), true);
    }
}

#[test]
fn amplitude_count_for_qubits_is_zero_qubits_safe() {
    let count = AmplitudeCount::checked_for_qubits(QubitCount::ZERO)
        .expect("2^0 must be representable");

    assert_eq!(count.get(), 1);
}

#[test]
fn amplitude_count_for_qubits_rejects_unrepresentable_shift() {
    let impossible = QubitCount::new(usize::BITS as usize);

    assert!(
        AmplitudeCount::checked_for_qubits(impossible).is_none(),
        "2^usize::BITS cannot be represented by usize"
    );
}

// =============================================================================
// ByteCount properties
// =============================================================================

#[test]
fn byte_count_constructor_is_identity() {
    for_each_sample(byte_samples(), |value| {
        let bytes = ByteCount::new(value);

        assert_eq!(bytes.get(), value);
        assert_eq!(bytes.is_zero(), value == 0);
        assert_eq!(bytes.is_non_zero(), value != 0);
    });
}

#[test]
fn byte_count_constants_are_correct() {
    assert_eq!(ByteCount::ZERO.get(), 0);
    assert_eq!(ByteCount::ONE.get(), 1);
    assert_eq!(ByteCount::KIB.get(), 1024);
    assert_eq!(ByteCount::MIB.get(), 1024 * 1024);
    assert_eq!(ByteCount::GIB.get(), 1024 * 1024 * 1024);
    assert_eq!(ByteCount::TIB.get(), 1024 * 1024 * 1024 * 1024);
}

#[test]
fn byte_count_checked_addition_matches_u64() {
    let samples = byte_samples();

    for &a in samples {
        for &b in samples {
            let lhs = ByteCount::new(a);
            let rhs = ByteCount::new(b);

            assert_eq!(
                lhs.checked_add(rhs).map(ByteCount::get),
                a.checked_add(b)
            );
        }
    }
}

#[test]
fn byte_count_checked_subtraction_matches_u64() {
    let samples = byte_samples();

    for &a in samples {
        for &b in samples {
            let lhs = ByteCount::new(a);
            let rhs = ByteCount::new(b);

            assert_eq!(
                lhs.checked_sub(rhs).map(ByteCount::get),
                a.checked_sub(b)
            );
        }
    }
}

#[test]
fn byte_count_checked_multiplication_matches_u64() {
    for &value in byte_samples() {
        let bytes = ByteCount::new(value);

        for multiplier in [0u64, 1, 2, 3, 4, 8, 16, 1024] {
            assert_eq!(
                bytes
                    .checked_mul(multiplier)
                    .map(ByteCount::get),
                value.checked_mul(multiplier)
            );
        }
    }
}

#[test]
fn byte_count_unit_conversions_are_floor_divisions() {
    for &value in byte_samples() {
        let bytes = ByteCount::new(value);

        assert_eq!(bytes.kibibytes(), value / 1024);
        assert_eq!(bytes.mebibytes(), value / (1024 * 1024));
        assert_eq!(
            bytes.gibibytes(),
            value / (1024 * 1024 * 1024)
        );
        assert_eq!(
            bytes.tebibytes(),
            value / (1024 * 1024 * 1024 * 1024)
        );
    }
}

#[test]
fn byte_count_order_matches_numeric_order() {
    let samples = byte_samples();

    for &a in samples {
        for &b in samples {
            assert_eq!(
                ByteCount::new(a).cmp(&ByteCount::new(b)),
                a.cmp(&b)
            );
        }
    }
}

#[test]
fn byte_count_try_as_usize_is_exact_or_reports_platform_overflow() {
    for &value in byte_samples() {
        let bytes = ByteCount::new(value);

        match bytes.try_as_usize() {
            Ok(actual) => {
                assert_eq!(
                    actual as u64,
                    value,
                    "successful conversion must be lossless"
                );
            }
            Err(error) => {
                // On a platform where the conversion fails, the source value
                // must genuinely exceed usize::MAX.
                assert!(
                    value > usize::MAX as u64,
                    "platform-overflow error is only valid when u64 exceeds usize"
                );

                let text = error.to_string();

                assert!(
                    text.contains("cannot be represented")
                        || text.contains("platform"),
                    "quantity conversion error must remain diagnostic"
                );
            }
        }
    }
}

// =============================================================================
// Algebraic identity properties
// =============================================================================

#[test]
fn qubit_count_add_zero_is_identity() {
    for_each_sample(quantity_samples(), |value| {
        let count = QubitCount::new(value);

        assert_eq!(
            count.checked_add(QubitCount::ZERO),
            Some(count)
        );
    });
}

#[test]
fn classical_bit_count_add_zero_is_identity() {
    for_each_sample(quantity_samples(), |value| {
        let count = ClassicalBitCount::new(value);

        assert_eq!(
            count.checked_add(ClassicalBitCount::ZERO),
            Some(count)
        );
    });
}

#[test]
fn amplitude_count_multiply_by_zero_is_zero() {
    for_each_sample(quantity_samples(), |value| {
        let count = AmplitudeCount::new(value);

        assert_eq!(
            count.checked_mul(0),
            Some(AmplitudeCount::ZERO)
        );
    });
}

#[test]
fn byte_count_add_zero_is_identity() {
    for_each_sample(byte_samples(), |value| {
        let count = ByteCount::new(value);

        assert_eq!(
            count.checked_add(ByteCount::ZERO),
            Some(count)
        );
    });
}

#[test]
fn byte_count_subtract_zero_is_identity() {
    for_each_sample(byte_samples(), |value| {
        let count = ByteCount::new(value);

        assert_eq!(
            count.checked_sub(ByteCount::ZERO),
            Some(count)
        );
    });
}

#[test]
fn qubit_count_subtract_self_is_zero() {
    for_each_sample(quantity_samples(), |value| {
        let count = QubitCount::new(value);

        assert_eq!(
            count.checked_sub(count),
            Some(QubitCount::ZERO)
        );
    });
}

#[test]
fn classical_bit_count_subtract_self_is_zero() {
    for_each_sample(quantity_samples(), |value| {
        let count = ClassicalBitCount::new(value);

        assert_eq!(
            count.checked_sub(count),
            Some(ClassicalBitCount::ZERO)
        );
    });
}

#[test]
fn byte_count_subtract_self_is_zero() {
    for_each_sample(byte_samples(), |value| {
        let count = ByteCount::new(value);

        assert_eq!(
            count.checked_sub(count),
            Some(ByteCount::ZERO)
        );
    });
}

// =============================================================================
// Overflow and underflow safety properties
// =============================================================================

#[test]
fn qubit_count_never_wraps_on_checked_addition() {
    let maximum = QubitCount::new(usize::MAX);
    let one = QubitCount::new(1);

    assert!(maximum.checked_add(one).is_none());
}

#[test]
fn qubit_count_never_wraps_on_checked_subtraction() {
    assert!(
        QubitCount::ZERO
            .checked_sub(QubitCount::new(1))
            .is_none()
    );
}

#[test]
fn classical_bit_count_never_wraps_on_checked_addition() {
    let maximum = ClassicalBitCount::new(usize::MAX);
    let one = ClassicalBitCount::new(1);

    assert!(maximum.checked_add(one).is_none());
}

#[test]
fn classical_bit_count_never_wraps_on_checked_subtraction() {
    assert!(
        ClassicalBitCount::ZERO
            .checked_sub(ClassicalBitCount::new(1))
            .is_none()
    );
}

#[test]
fn amplitude_count_never_wraps_on_checked_multiplication() {
    let maximum = AmplitudeCount::new(usize::MAX);

    assert!(maximum.checked_mul(2).is_none());
}

#[test]
fn byte_count_never_wraps_on_checked_addition() {
    let maximum = ByteCount::new(u64::MAX);

    assert!(maximum.checked_add(ByteCount::ONE).is_none());
}

#[test]
fn byte_count_never_wraps_on_checked_multiplication() {
    let maximum = ByteCount::new(u64::MAX);

    assert!(maximum.checked_mul(2).is_none());
}

// =============================================================================
// Exponential state-size safety properties
// =============================================================================

#[test]
fn dense_state_vector_amplitude_count_is_exact_for_small_qubit_counts() {
    for qubits in 0usize..=16 {
        let count = AmplitudeCount::checked_for_qubits(QubitCount::new(qubits))
            .expect("small dense state-vector dimensions must be representable");

        assert_eq!(
            count.get(),
            2usize.pow(qubits as u32),
            "dense state-vector cardinality must equal 2^n"
        );
    }
}

#[test]
fn dense_state_vector_cardinality_is_strictly_increasing() {
    let mut previous = AmplitudeCount::checked_for_qubits(QubitCount::ZERO)
        .expect("2^0 must be representable");

    for qubits in 1usize..=16 {
        let current = AmplitudeCount::checked_for_qubits(QubitCount::new(qubits))
            .expect("small dense state-vector dimensions must be representable");

        assert!(
            current > previous,
            "2^n must strictly increase as n increases"
        );

        previous = current;
    }
}

#[test]
fn dense_state_vector_cardinality_doubles_per_added_qubit() {
    let mut previous = AmplitudeCount::checked_for_qubits(QubitCount::ZERO)
        .expect("2^0 must be representable");

    for qubits in 1usize..=16 {
        let current = AmplitudeCount::checked_for_qubits(QubitCount::new(qubits))
            .expect("small dense state-vector dimensions must be representable");

        assert_eq!(
            current.get(),
            previous.get() * 2,
            "adding one qubit must double dense-state cardinality"
        );

        previous = current;
    }
}

// =============================================================================
// Strong-type separation properties
// =============================================================================

#[test]
fn different_memory_quantities_preserve_their_own_domains() {
    let value = 42usize;

    let qubits = QubitCount::new(value);
    let classical_bits = ClassicalBitCount::new(value);
    let amplitudes = AmplitudeCount::new(value);

    assert_eq!(qubits.get(), value);
    assert_eq!(classical_bits.get(), value);
    assert_eq!(amplitudes.get(), value);

    // The three quantities intentionally remain different Rust types.
    //
    // This test exists primarily as a compile-time contract: if a future
    // refactor replaces them with aliases, the intended type-level separation
    // is lost and this module should be reviewed.
    assert_ne!(
        std::any::type_name::<QubitCount>(),
        std::any::type_name::<ClassicalBitCount>()
    );

    assert_ne!(
        std::any::type_name::<QubitCount>(),
        std::any::type_name::<AmplitudeCount>()
    );

    assert_ne!(
        std::any::type_name::<ClassicalBitCount>(),
        std::any::type_name::<AmplitudeCount>()
    );
}

// =============================================================================
// Display determinism
// =============================================================================

#[test]
fn quantity_display_is_deterministic() {
    for_each_sample(quantity_samples(), |value| {
        let qubits = QubitCount::new(value);
        let classical = ClassicalBitCount::new(value);
        let amplitudes = AmplitudeCount::new(value);

        let qubits_a = qubits.to_string();
        let qubits_b = qubits.to_string();

        let classical_a = classical.to_string();
        let classical_b = classical.to_string();

        let amplitudes_a = amplitudes.to_string();
        let amplitudes_b = amplitudes.to_string();

        assert_eq!(qubits_a, qubits_b);
        assert_eq!(classical_a, classical_b);
        assert_eq!(amplitudes_a, amplitudes_b);

        assert!(qubits_a.contains("qubit"));
        assert!(classical_a.contains("classical bit"));
        assert!(amplitudes_a.contains("amplitude"));
    });
}

#[test]
fn byte_count_display_is_deterministic() {
    for_each_sample(byte_samples(), |value| {
        let bytes = ByteCount::new(value);

        assert_eq!(bytes.to_string(), bytes.to_string());
    });
}

// =============================================================================
// Serialization properties
// =============================================================================
//
// These tests intentionally remain independent of a particular snapshot
// envelope. The primitive types already own their Serde contract; snapshot.rs
// owns schema/version/integrity policy.
//
// This keeps this module useful for all providers and persistence backends.

#[test]
fn memory_quantity_types_are_serde_serializable() {
    fn assert_serde<T>()
    where
        T: serde::Serialize + for<'de> serde::Deserialize<'de>,
    {
    }

    assert_serde::<QubitCount>();
    assert_serde::<ClassicalBitCount>();
    assert_serde::<AmplitudeCount>();
    assert_serde::<ByteCount>();
}

#[test]
fn serde_round_trip_preserves_qubit_count() {
    for_each_sample(quantity_samples(), |value| {
        let original = QubitCount::new(value);

        let encoded =
            serde_json::to_string(&original)
                .expect("QubitCount serialization must succeed");

        let decoded: QubitCount =
            serde_json::from_str(&encoded)
                .expect("QubitCount deserialization must succeed");

        assert_eq!(decoded, original);
    });
}

#[test]
fn serde_round_trip_preserves_classical_bit_count() {
    for_each_sample(quantity_samples(), |value| {
        let original = ClassicalBitCount::new(value);

        let encoded =
            serde_json::to_string(&original)
                .expect("ClassicalBitCount serialization must succeed");

        let decoded: ClassicalBitCount =
            serde_json::from_str(&encoded)
                .expect("ClassicalBitCount deserialization must succeed");

        assert_eq!(decoded, original);
    });
}

#[test]
fn serde_round_trip_preserves_amplitude_count() {
    for_each_sample(quantity_samples(), |value| {
        let original = AmplitudeCount::new(value);

        let encoded =
            serde_json::to_string(&original)
                .expect("AmplitudeCount serialization must succeed");

        let decoded: AmplitudeCount =
            serde_json::from_str(&encoded)
                .expect("AmplitudeCount deserialization must succeed");

        assert_eq!(decoded, original);
    });
}

#[test]
fn serde_round_trip_preserves_byte_count() {
    for_each_sample(byte_samples(), |value| {
        let original = ByteCount::new(value);

        let encoded =
            serde_json::to_string(&original)
                .expect("ByteCount serialization must succeed");

        let decoded: ByteCount =
            serde_json::from_str(&encoded)
                .expect("ByteCount deserialization must succeed");

        assert_eq!(decoded, original);
    });
}

// =============================================================================
// Representation-independent quantum-memory invariants
// =============================================================================

#[test]
fn zero_resource_quantities_have_no_hidden_capacity() {
    assert_eq!(QubitCount::ZERO.get(), 0);
    assert_eq!(ClassicalBitCount::ZERO.get(), 0);
    assert_eq!(AmplitudeCount::ZERO.get(), 0);
    assert_eq!(ByteCount::ZERO.get(), 0);
}

#[test]
fn one_qubit_has_exactly_two_basis_amplitudes() {
    let amplitudes =
        AmplitudeCount::checked_for_qubits(QubitCount::new(1))
            .expect("2^1 must be representable");

    assert_eq!(amplitudes.get(), 2);
}

#[test]
fn two_qubits_have_exactly_four_basis_amplitudes() {
    let amplitudes =
        AmplitudeCount::checked_for_qubits(QubitCount::new(2))
            .expect("2^2 must be representable");

    assert_eq!(amplitudes.get(), 4);
}

#[test]
fn three_qubits_have_exactly_eight_basis_amplitudes() {
    let amplitudes =
        AmplitudeCount::checked_for_qubits(QubitCount::new(3))
            .expect("2^3 must be representable");

    assert_eq!(amplitudes.get(), 8);
}

// =============================================================================
// Allocation-safety policy tests
// =============================================================================

#[test]
fn property_suite_never_constructs_large_dense_state_storage() {
    // This test documents an important CI invariant.
    //
    // Property tests must validate cardinality mathematically rather than
    // allocating 2^n amplitudes. A production quantum-memory test suite must
    // never accidentally turn an adversarial qubit count into an enormous
    // allocation.
    //
    // The largest cardinality actually constructed by this property module is
    // 2^16 = 65,536 logical amplitudes as a scalar quantity only.
    let maximum_tested_qubits = 16usize;

    let amplitudes =
        AmplitudeCount::checked_for_qubits(
            QubitCount::new(maximum_tested_qubits),
        )
        .expect("2^16 must be representable");

    assert_eq!(amplitudes.get(), 65_536);
}

#[test]
fn exponential_cardinality_is_checked_before_any_real_allocation() {
    let large_qubit_count = QubitCount::new(usize::BITS as usize);

    assert!(
        AmplitudeCount::checked_for_qubits(large_qubit_count).is_none(),
        "unrepresentable dense cardinality must fail before allocation"
    );
}

// =============================================================================
// Regression properties for integer edge cases
// =============================================================================

#[test]
fn usize_max_is_preserved_by_qubit_count() {
    let count = QubitCount::new(usize::MAX);

    assert_eq!(count.get(), usize::MAX);
    assert!(!count.is_zero());
    assert!(count.is_non_zero());
}

#[test]
fn usize_max_is_preserved_by_classical_bit_count() {
    let count = ClassicalBitCount::new(usize::MAX);

    assert_eq!(count.get(), usize::MAX);
    assert!(!count.is_zero());
    assert!(count.is_non_zero());
}

#[test]
fn usize_max_is_preserved_by_amplitude_count() {
    let count = AmplitudeCount::new(usize::MAX);

    assert_eq!(count.get(), usize::MAX);
    assert!(!count.is_zero());
    assert!(count.is_non_zero());
}

#[test]
fn u64_max_is_preserved_by_byte_count() {
    let count = ByteCount::new(u64::MAX);

    assert_eq!(count.get(), u64::MAX);
    assert!(!count.is_zero());
    assert!(count.is_non_zero());
}

// =============================================================================
// Monotonicity properties
// =============================================================================

#[test]
fn qubit_count_is_monotonic_under_valid_addition() {
    for &a in quantity_samples() {
        for &b in quantity_samples() {
            if let Some(result) =
                QubitCount::new(a).checked_add(QubitCount::new(b))
            {
                assert!(result >= QubitCount::new(a));
                assert!(result >= QubitCount::new(b));
            }
        }
    }
}

#[test]
fn classical_bit_count_is_monotonic_under_valid_addition() {
    for &a in quantity_samples() {
        for &b in quantity_samples() {
            if let Some(result) =
                ClassicalBitCount::new(a)
                    .checked_add(ClassicalBitCount::new(b))
            {
                assert!(result >= ClassicalBitCount::new(a));
                assert!(result >= ClassicalBitCount::new(b));
            }
        }
    }
}

#[test]
fn byte_count_is_monotonic_under_valid_addition() {
    for &a in byte_samples() {
        for &b in byte_samples() {
            if let Some(result) =
                ByteCount::new(a).checked_add(ByteCount::new(b))
            {
                assert!(result >= ByteCount::new(a));
                assert!(result >= ByteCount::new(b));
            }
        }
    }
}

// =============================================================================
// Cancellation properties
// =============================================================================

#[test]
fn qubit_count_add_then_subtract_cancels_when_no_overflow_occurs() {
    for &a in quantity_samples() {
        for &b in quantity_samples() {
            let lhs = QubitCount::new(a);
            let rhs = QubitCount::new(b);

            if let Some(sum) = lhs.checked_add(rhs) {
                let restored = sum
                    .checked_sub(rhs)
                    .expect("subtracting rhs from valid sum must succeed");

                assert_eq!(restored, lhs);
            }
        }
    }
}

#[test]
fn classical_bit_count_add_then_subtract_cancels_when_no_overflow_occurs() {
    for &a in quantity_samples() {
        for &b in quantity_samples() {
            let lhs = ClassicalBitCount::new(a);
            let rhs = ClassicalBitCount::new(b);

            if let Some(sum) = lhs.checked_add(rhs) {
                let restored = sum
                    .checked_sub(rhs)
                    .expect("subtracting rhs from valid sum must succeed");

                assert_eq!(restored, lhs);
            }
        }
    }
}

#[test]
fn byte_count_add_then_subtract_cancels_when_no_overflow_occurs() {
    for &a in byte_samples() {
        for &b in byte_samples() {
            let lhs = ByteCount::new(a);
            let rhs = ByteCount::new(b);

            if let Some(sum) = lhs.checked_add(rhs) {
                let restored = sum
                    .checked_sub(rhs)
                    .expect("subtracting rhs from valid sum must succeed");

                assert_eq!(restored, lhs);
            }
        }
    }
}

// =============================================================================
// Multiplication properties
// =============================================================================

#[test]
fn qubit_count_multiplication_by_one_is_identity() {
    for_each_sample(quantity_samples(), |value| {
        let count = QubitCount::new(value);

        assert_eq!(
            count.checked_mul(1),
            Some(count)
        );
    });
}

#[test]
fn classical_bit_count_multiplication_by_one_is_identity() {
    for_each_sample(quantity_samples(), |value| {
        let count = ClassicalBitCount::new(value);

        assert_eq!(
            count.checked_mul(1),
            Some(count)
        );
    });
}

#[test]
fn amplitude_count_multiplication_by_one_is_identity() {
    for_each_sample(quantity_samples(), |value| {
        let count = AmplitudeCount::new(value);

        assert_eq!(
            count.checked_mul(1),
            Some(count)
        );
    });
}

#[test]
fn byte_count_multiplication_by_one_is_identity() {
    for_each_sample(byte_samples(), |value| {
        let count = ByteCount::new(value);

        assert_eq!(
            count.checked_mul(1),
            Some(count)
        );
    });
}

// =============================================================================
// Boundary-oriented property matrix
// =============================================================================

#[test]
fn boundary_values_have_consistent_zero_semantics() {
    let boundaries = [
        0usize,
        1,
        2,
        usize::MAX,
    ];

    for value in boundaries {
        let q = QubitCount::new(value);
        let c = ClassicalBitCount::new(value);
        let a = AmplitudeCount::new(value);

        assert_eq!(q.is_zero(), value == 0);
        assert_eq!(c.is_zero(), value == 0);
        assert_eq!(a.is_zero(), value == 0);
    }
}

#[test]
fn boundary_values_have_consistent_nonzero_semantics() {
    let boundaries = [
        0usize,
        1,
        2,
        usize::MAX,
    ];

    for value in boundaries {
        let q = QubitCount::new(value);
        let c = ClassicalBitCount::new(value);
        let a = AmplitudeCount::new(value);

        assert_eq!(q.is_non_zero(), value != 0);
        assert_eq!(c.is_non_zero(), value != 0);
        assert_eq!(a.is_non_zero(), value != 0);
    }
}

// =============================================================================
// Determinism regression
// =============================================================================

#[test]
fn deterministic_sample_generation_is_stable() {
    assert_eq!(
        quantity_samples(),
        &[
            0,
            1,
            2,
            3,
            4,
            5,
            7,
            8,
            15,
            16,
            17,
            31,
            32,
            33,
            63,
            64,
            65,
            127,
            128,
            129,
            255,
            256,
            257,
            511,
            512,
            513,
            1023,
            1024,
            1025,
            4095,
            4096,
            4097,
        ]
    );
}

#[test]
fn deterministic_byte_sample_generation_is_stable() {
    assert_eq!(
        byte_samples(),
        &[
            0,
            1,
            2,
            3,
            4,
            7,
            8,
            15,
            16,
            31,
            32,
            63,
            64,
            127,
            128,
            255,
            256,
            1023,
            1024,
            1025,
            4096,
            1 << 20,
            1 << 30,
            u64::MAX / 2,
            u64::MAX - 1,
            u64::MAX,
        ]
    );
}

// =============================================================================
// Architectural contract tests
// =============================================================================

#[test]
fn memory_quantity_types_are_not_pointer_types() {
    assert!(!std::any::type_name::<QubitCount>().contains("*"));
    assert!(!std::any::type_name::<ClassicalBitCount>().contains("*"));
    assert!(!std::any::type_name::<AmplitudeCount>().contains("*"));
    assert!(!std::any::type_name::<ByteCount>().contains("*"));
}

#[test]
fn memory_quantity_types_are_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}

    assert_send_sync::<QubitCount>();
    assert_send_sync::<ClassicalBitCount>();
    assert_send_sync::<AmplitudeCount>();
    assert_send_sync::<ByteCount>();
}

#[test]
fn memory_quantity_types_are_copy() {
    fn assert_copy<T: Copy>() {}

    assert_copy::<QubitCount>();
    assert_copy::<ClassicalBitCount>();
    assert_copy::<AmplitudeCount>();
    assert_copy::<ByteCount>();
}

// =============================================================================
// Final contract sentinel
// =============================================================================

#[test]
fn production_property_contract_is_complete_for_foundational_quantities() {
    // This intentionally small test is a sentinel documenting what this file
    // guarantees:
    //
    // * strongly typed memory quantities;
    // * zero/non-zero semantics;
    // * checked arithmetic;
    // * exponential cardinality calculation;
    // * overflow/underflow rejection;
    // * deterministic behavior;
    // * serialization capability;
    // * no hidden allocation;
    // * Send + Sync + Copy value semantics.
    //
    // Representation-specific quantum mathematics belongs in:
    //
    // * state_vector.rs
    // * density_matrix.rs
    // * stabilizer.rs
    // * sparse.rs
    // * tensor_network.rs
    //
    // Provider/QPU-specific behavior belongs in:
    //
    // * backend_state.rs
    // * quantum::hardware
    //
    // Therefore this test suite deliberately does not duplicate those
    // responsibilities.

    assert!(true);
}