//! Zamani Quantum Memory — Stabilizer Integration / Conformance Tests.
//!
//! Production-grade conformance tests for
//! `crate::quantum::memory::stabilizer`.
//!
//! # Purpose
//!
//! This file verifies the externally observable contract of the canonical
//! stabilizer representation without depending on:
//!
//! - private tableau fields;
//! - implementation-specific bitset storage;
//! - a particular simulator;
//! - a particular CPU;
//! - SIMD;
//! - GPU APIs;
//! - distributed-memory APIs;
//! - a QPU vendor;
//! - provider SDKs;
//! - network access;
//! - credentials;
//! - routing;
//! - scheduling;
//! - benchmarking;
//! - QEC decoder implementations;
//! - frontend/source-language syntax.
//!
//! The stabilizer representation is a mathematical Clifford-state substrate.
//! Physical QPUs are not required to expose a stabilizer tableau. Hardware
//! adapters may use this representation for Clifford-compatible simulation,
//! verification, QEC, Pauli-frame processing, or hybrid execution, but must
//! preserve the mathematical semantics tested here.
//!
//! # Production boundary
//!
//! ```text
//!                    quantum::ir
//!                        |
//!                        v
//!                  execution layer
//!                        |
//!                        v
//!              quantum::memory::stabilizer
//!                        |
//!          +-------------+-------------+
//!          |             |             |
//!          v             v             v
//!        CPU           GPU        distributed
//!          |             |             |
//!          +-------------+-------------+
//!                        |
//!                        v
//!                 hardware adapters
//!                        |
//!              +---------+---------+
//!              |                   |
//!              v                   v
//!             QPU              simulator
//!
//! Tests sit beside the production representation and verify its public
//! mathematical contract. They do not implement another stabilizer engine.
//! ```
//!
//! # Covered contract
//!
//! This suite verifies:
//!
//! 1. construction;
//! 2. zero-qubit and one-qubit boundary cases;
//! 3. tableau dimensions;
//! 4. stabilizer generators;
//! 5. destabilizer generators;
//! 6. Pauli construction;
//! 7. Pauli signs;
//! 8. Pauli commutation;
//! 9. dimension checking;
//! 10. invalid qubit handling;
//! 11. Clifford dispatch;
//! 12. H;
//! 13. S;
//! 14. S†;
//! 15. X;
//! 16. Y;
//! 17. Z;
//! 18. CNOT;
//! 19. CZ;
//! 20. SWAP;
//! 21. Bell states;
//! 22. graph states;
//! 23. GHZ-compatible Clifford evolution;
//! 24. stabilizer membership;
//! 25. signed observables;
//! 26. expectation values;
//! 27. deterministic measurement;
//! 28. random measurement;
//! 29. X measurement;
//! 30. Y measurement;
//! 31. Z measurement;
//! 32. arbitrary Pauli measurement;
//! 33. measurement collapse;
//! 34. correlated measurements;
//! 35. reset;
//! 36. reset-many;
//! 37. Pauli frames;
//! 38. deterministic RNG;
//! 39. RNG checkpoint/restart;
//! 40. word-boundary qubit counts;
//! 41. larger stabilizer states;
//! 42. cloning/isolation;
//! 43. invariant preservation;
//! 44. hardware neutrality;
//! 45. absence of hidden randomness.
//!
//! # Hardware / QPU rule
//!
//! These tests deliberately do not instantiate or contact:
//!
//! - IBM;
//! - Quantinuum;
//! - IonQ;
//! - Rigetti;
//! - IQM;
//! - Pasqal;
//! - Google;
//! - AWS;
//! - Azure;
//! - D-Wave;
//! - NVIDIA;
//! - CUDA;
//! - ROCm;
//! - Metal;
//! - Vulkan;
//! - SYCL;
//! - any cloud API;
//! - any physical QPU.
//!
//! This is intentional.
//!
//! "Supports all QPUs" at this layer means that the stabilizer contract is
//! vendor-neutral. A hardware adapter can consume the same mathematical
//! contract regardless of the physical technology behind it.
//!
//! # QEC rule
//!
//! Stabilizer states are particularly important to quantum error correction.
//! These tests therefore exercise:
//!
//! - Pauli operators;
//! - stabilizer membership;
//! - syndrome-like measurements;
//! - Pauli frames;
//! - deterministic measurement;
//! - reset;
//! - large polynomial-memory tableaux.
//!
//! The tests do not implement a QEC decoder.
//!
//! # Determinism
//!
//! Randomness is always explicitly injected.
//!
//! No global RNG is used.
//! No wall-clock state is used.
//! No environment variables are read.
//! No provider state is used.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only.
//!
//! No nightly features are required.
//!
//! # Safety
//!
//! This file contains no unsafe code.
//!
//! `unsafe` is explicitly denied.
//!
//! # Integration contract
//!
//! This test file consumes only:
//!
//! ```text
//! crate::quantum::memory::stabilizer
//! ```
//!
//! It intentionally does not require later memory modules such as:
//!
//! - allocator.rs;
//! - pool.rs;
//! - reservation.rs;
//! - gpu.rs;
//! - distributed.rs;
//! - snapshot.rs;
//! - checkpoint.rs;
//! - migration.rs;
//! - telemetry.rs.
//!
//! Therefore this file can be completed and frozen independently.
//!
//! When `src/quantum/memory/tests/mod.rs` is introduced, it should mount this
//! file exactly once:
//!
//! ```text
//! #[path = "stabilizer.rs"]
//! mod stabilizer;
//! ```
//!
//! `memory/mod.rs` should mount the test composition module, not this file
//! directly.
//!
//! The production `stabilizer.rs` must never import this test file.
//!
//! # Completion rule
//!
//! This file is complete when:
//!
//! - it uses only the public stabilizer API;
//! - it has no vendor dependencies;
//! - it has no network dependencies;
//! - it has no credential dependencies;
//! - it has no unsafe code;
//! - it has deterministic fixtures;
//! - it verifies invalid paths as well as successful paths;
//! - it verifies word-boundary storage sizes;
//! - it verifies tableau invariants;
//! - it verifies measurement semantics;
//! - it verifies Pauli-frame semantics;
//! - it verifies the public hardware-neutral contract;
//! - later GPU/QPU/routing/QEC/benchmarking implementations do not require
//!   this file to be edited merely because those implementations are added.
//!
//! -----------------------------------------------------------------------------
//! Implementation
//! -----------------------------------------------------------------------------

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use crate::quantum::memory::stabilizer::{
    CliffordGate,
    MeasurementBasis,
    Pauli,
    PauliFrame,
    PauliString,
    RandomSource,
    StabilizerError,
    StabilizerState,
    XorShift64,
};

// =============================================================================
// Test helpers
// =============================================================================

/// Constructs a Pauli string from a list of `(qubit, Pauli)` terms.
fn pauli(qubits: usize, terms: &[(usize, Pauli)]) -> PauliString {
    let mut result =
        PauliString::identity(qubits).expect("valid Pauli dimension");

    for &(qubit, value) in terms {
        result
            .set(qubit, value)
            .expect("valid Pauli term");
    }

    result
}

/// Asserts an expectation value.
fn expect(
    state: &StabilizerState,
    terms: &[(usize, Pauli)],
    expected: i8,
) {
    let observable = pauli(state.qubits(), terms);

    assert_eq!(
        state
            .expectation(&observable)
            .expect("expectation must succeed"),
        expected
    );
}

/// Deterministic test RNG that alternates 0, 1, 0, 1...
///
/// This avoids statistical flakiness while still exercising the random
/// measurement path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AlternatingRng {
    next: bool,
}

impl AlternatingRng {
    const fn new() -> Self {
        Self { next: false }
    }
}

impl RandomSource for AlternatingRng {
    fn next_u64(&mut self) -> u64 {
        let value = if self.next { u64::MAX } else { 0 };

        self.next = !self.next;

        value
    }
}

// =============================================================================
// Construction
// =============================================================================

#[test]
fn zero_state_has_exact_tableau_shape_and_generators() {
    let state =
        StabilizerState::new(4).expect("state");

    assert_eq!(state.qubits(), 4);
    assert_eq!(state.tableau_rows(), 8);
    assert_eq!(state.destabilizers().len(), 4);
    assert_eq!(state.stabilizers().len(), 4);

    state
        .validate()
        .expect("fresh tableau must validate");

    for qubit in 0..4 {
        expect(
            &state,
            &[(qubit, Pauli::Z)],
            1,
        );

        assert_eq!(
            state
                .stabilizer(qubit)
                .expect("stabilizer")
                .get(qubit)
                .expect("Pauli"),
            Pauli::Z
        );

        assert_eq!(
            state
                .destabilizer(qubit)
                .expect("destabilizer")
                .get(qubit)
                .expect("Pauli"),
            Pauli::X
        );
    }
}

#[test]
fn zero_qubit_state_is_well_formed() {
    let state =
        StabilizerState::new(0)
            .expect("zero-qubit state");

    assert_eq!(state.qubits(), 0);
    assert_eq!(state.tableau_rows(), 0);

    state
        .validate()
        .expect("zero-qubit tableau must validate");
}

#[test]
fn one_qubit_constructor_is_equivalent_to_new_one() {
    assert_eq!(
        StabilizerState::zero().expect("zero"),
        StabilizerState::new(1).expect("new")
    );
}

// =============================================================================
// Pauli API
// =============================================================================

#[test]
fn pauli_string_accessors_and_commutation_are_correct() {
    let x =
        pauli(2, &[(0, Pauli::X)]);

    let z =
        pauli(2, &[(0, Pauli::Z)]);

    let y =
        pauli(2, &[(0, Pauli::Y)]);

    let remote_z =
        pauli(2, &[(1, Pauli::Z)]);

    assert_eq!(
        x.get(0).expect("X"),
        Pauli::X
    );

    assert_eq!(
        y.get(0).expect("Y"),
        Pauli::Y
    );

    assert_eq!(
        z.get(0).expect("Z"),
        Pauli::Z
    );

    assert!(
        x.anticommutes_with(&z)
            .expect("commutation")
    );

    assert!(
        x.anticommutes_with(&y)
            .expect("commutation")
    );

    assert!(
        y.anticommutes_with(&z)
            .expect("commutation")
    );

    assert!(
        x.commutes_with(&remote_z)
            .expect("commutation")
    );
}

#[test]
fn pauli_string_identity_and_sign_are_exact() {
    let mut identity =
        PauliString::identity(3)
            .expect("identity");

    assert!(identity.is_identity());
    assert_eq!(identity.sign(), 1);
    assert_eq!(identity.phase(), 0);

    identity
        .set_sign(-1)
        .expect("negative identity");

    assert!(identity.is_identity());
    assert_eq!(identity.sign(), -1);
    assert_eq!(identity.phase(), 2);

    identity
        .set_sign(1)
        .expect("positive identity");

    assert_eq!(identity.sign(), 1);
}

#[test]
fn pauli_dimension_mismatch_is_rejected() {
    let state =
        StabilizerState::new(2)
            .expect("state");

    let observable =
        PauliString::single(
            1,
            0,
            Pauli::Z,
        )
        .expect("observable");

    assert!(matches!(
        state.expectation(&observable),
        Err(StabilizerError::DimensionMismatch {
            expected: 2,
            actual: 1
        })
    ));

    assert!(matches!(
        state.contains_stabilizer(&observable),
        Err(StabilizerError::DimensionMismatch {
            expected: 2,
            actual: 1
        })
    ));
}

#[test]
fn invalid_qubit_access_is_rejected_without_panicking() {
    let mut state =
        StabilizerState::new(2)
            .expect("state");

    assert!(matches!(
        state.h(2),
        Err(StabilizerError::QubitOutOfRange {
            qubit: 2,
            qubits: 2
        })
    ));

    assert!(matches!(
        state.cnot(0, 2),
        Err(StabilizerError::QubitOutOfRange {
            qubit: 2,
            qubits: 2
        })
    ));

    assert!(matches!(
        state.cnot(1, 1),
        Err(StabilizerError::DuplicateQubit {
            qubit: 1
        })
    ));

    assert!(matches!(
        state.stabilizer(2),
        Err(StabilizerError::QubitOutOfRange {
            qubit: 2,
            qubits: 2
        })
    ));
}

// =============================================================================
// Clifford dispatch
// =============================================================================

#[test]
fn gate_dispatch_matches_direct_gate_methods() {
    let mut dispatched =
        StabilizerState::new(3)
            .expect("state");

    let mut direct =
        StabilizerState::new(3)
            .expect("state");

    dispatched
        .apply(CliffordGate::H { qubit: 0 })
        .expect("H");

    direct.h(0).expect("H");

    dispatched
        .apply(CliffordGate::S { qubit: 1 })
        .expect("S");

    direct.s(1).expect("S");

    dispatched
        .apply(CliffordGate::Sdg { qubit: 1 })
        .expect("Sdg");

    direct.sdg(1).expect("Sdg");

    dispatched
        .apply(CliffordGate::X { qubit: 2 })
        .expect("X");

    direct.x(2).expect("X");

    dispatched
        .apply(CliffordGate::Y { qubit: 0 })
        .expect("Y");

    direct.y(0).expect("Y");

    dispatched
        .apply(CliffordGate::Z { qubit: 1 })
        .expect("Z");

    direct.z(1).expect("Z");

    dispatched
        .apply(CliffordGate::Cnot {
            control: 0,
            target: 1,
        })
        .expect("CNOT");

    direct
        .cnot(0, 1)
        .expect("CNOT");

    dispatched
        .apply(CliffordGate::Cz {
            control: 1,
            target: 2,
        })
        .expect("CZ");

    direct
        .cz(1, 2)
        .expect("CZ");

    dispatched
        .apply(CliffordGate::Swap {
            first: 0,
            second: 2,
        })
        .expect("SWAP");

    direct
        .swap(0, 2)
        .expect("SWAP");

    assert_eq!(
        dispatched,
        direct
    );

    dispatched
        .validate()
        .expect("dispatch result must validate");
}

// =============================================================================
// Single-qubit Clifford correctness
// =============================================================================

#[test]
fn hadamard_is_involutory_and_maps_x_to_z() {
    let mut state =
        StabilizerState::new(1)
            .expect("state");

    state.h(0).expect("H");

    expect(
        &state,
        &[(0, Pauli::X)],
        1,
    );

    expect(
        &state,
        &[(0, Pauli::Z)],
        0,
    );

    state.h(0).expect("H");

    expect(
        &state,
        &[(0, Pauli::Z)],
        1,
    );

    state
        .validate()
        .expect("tableau");
}

#[test]
fn phase_gate_maps_x_to_y_with_correct_sign() {
    let mut state =
        StabilizerState::new(1)
            .expect("state");

    state.h(0).expect("H");
    state.s(0).expect("S");

    expect(
        &state,
        &[(0, Pauli::Y)],
        1,
    );

    expect(
        &state,
        &[(0, Pauli::X)],
        0,
    );

    state.sdg(0).expect("Sdg");

    expect(
        &state,
        &[(0, Pauli::X)],
        1,
    );

    state
        .validate()
        .expect("tableau");
}

#[test]
fn s_and_sdg_are_inverses_on_single_qubit_states() {
    for preparation in [
        Pauli::I,
        Pauli::X,
        Pauli::Y,
        Pauli::Z,
    ] {
        let mut state =
            StabilizerState::new(1)
                .expect("state");

        match preparation {
            Pauli::I => {}

            Pauli::X => {
                state.x(0).expect("X");
            }

            Pauli::Y => {
                state.y(0).expect("Y");
            }

            Pauli::Z => {
                state.z(0).expect("Z");
            }
        }

        let before = state.clone();

        state.s(0).expect("S");
        state.sdg(0).expect("Sdg");

        assert_eq!(
            state,
            before
        );

        state
            .validate()
            .expect("tableau");
    }
}

#[test]
fn pauli_x_y_z_are_involutions() {
    for gate in [
        Pauli::X,
        Pauli::Y,
        Pauli::Z,
    ] {
        let mut state =
            StabilizerState::new(1)
                .expect("state");

        match gate {
            Pauli::X => {
                state.x(0).expect("X");
                state.x(0).expect("X");
            }

            Pauli::Y => {
                state.y(0).expect("Y");
                state.y(0).expect("Y");
            }

            Pauli::Z => {
                state.z(0).expect("Z");
                state.z(0).expect("Z");
            }

            Pauli::I => {
                unreachable!();
            }
        }

        assert_eq!(
            state,
            StabilizerState::new(1)
                .expect("zero")
        );

        state
            .validate()
            .expect("tableau");
    }
}

// =============================================================================
// Two-qubit Clifford correctness
// =============================================================================

#[test]
fn bell_state_has_xx_and_zz_stabilizers() {
    let mut state =
        StabilizerState::new(2)
            .expect("state");

    state.h(0).expect("H");
    state.cnot(0, 1).expect("CNOT");

    expect(
        &state,
        &[
            (0, Pauli::X),
            (1, Pauli::X),
        ],
        1,
    );

    expect(
        &state,
        &[
            (0, Pauli::Z),
            (1, Pauli::Z),
        ],
        1,
    );

    expect(
        &state,
        &[(0, Pauli::Z)],
        0,
    );

    expect(
        &state,
        &[(1, Pauli::Z)],
        0,
    );

    state
        .validate()
        .expect("Bell tableau");
}

#[test]
fn cnot_maps_plus_state_to_bell_generators() {
    let mut state =
        StabilizerState::new(2)
            .expect("state");

    state.h(0).expect("H");
    state.cnot(0, 1).expect("CNOT");

    assert!(
        state
            .contains_stabilizer(
                &pauli(
                    2,
                    &[
                        (0, Pauli::X),
                        (1, Pauli::X),
                    ],
                ),
            )
            .expect("XX membership")
    );

    assert!(
        state
            .contains_stabilizer(
                &pauli(
                    2,
                    &[
                        (0, Pauli::Z),
                        (1, Pauli::Z),
                    ],
                ),
            )
            .expect("ZZ membership")
    );
}

#[test]
fn cz_maps_plus_plus_to_graph_state_generators() {
    let mut state =
        StabilizerState::new(2)
            .expect("state");

    state.h(0).expect("H");
    state.h(1).expect("H");
    state.cz(0, 1).expect("CZ");

    expect(
        &state,
        &[
            (0, Pauli::X),
            (1, Pauli::Z),
        ],
        1,
    );

    expect(
        &state,
        &[
            (0, Pauli::Z),
            (1, Pauli::X),
        ],
        1,
    );

    state
        .validate()
        .expect("CZ tableau");
}

#[test]
fn swap_is_involutory_and_exchanges_qubit_information() {
    let mut state =
        StabilizerState::new(2)
            .expect("state");

    state.x(0).expect("X");

    let before = state.clone();

    state.swap(0, 1).expect("SWAP");

    expect(
        &state,
        &[(0, Pauli::Z)],
        1,
    );

    expect(
        &state,
        &[(1, Pauli::Z)],
        -1,
    );

    state.swap(0, 1).expect("SWAP");

    assert_eq!(
        state,
        before
    );

    state
        .validate()
        .expect("tableau");
}

// =============================================================================
// General Clifford invariant testing
// =============================================================================

#[test]
fn arbitrary_clifford_sequence_preserves_tableau_invariants() {
    let mut state =
        StabilizerState::new(16)
            .expect("state");

    for round in 0..32usize {
        let a = round % 16;
        let b = (round * 7 + 3) % 16;
        let c = (round * 11 + 5) % 16;

        state.h(a).expect("H");
        state.s(b).expect("S");
        state.sdg(c).expect("Sdg");

        if a != b {
            state
                .cnot(a, b)
                .expect("CNOT");
        }

        if b != c {
            state
                .cz(b, c)
                .expect("CZ");
        }

        if a != c {
            state
                .swap(a, c)
                .expect("SWAP");
        }

        state
            .validate()
            .expect("invariants after Clifford sequence");
    }
}

// =============================================================================
// Expectation and membership
// =============================================================================

#[test]
fn identity_expectation_is_plus_or_minus_one_by_sign() {
    let state =
        StabilizerState::new(5)
            .expect("state");

    let positive =
        PauliString::identity(5)
            .expect("identity");

    assert_eq!(
        state
            .expectation(&positive)
            .expect("expectation"),
        1
    );

    let mut negative =
        PauliString::identity(5)
            .expect("identity");

    negative
        .set_sign(-1)
        .expect("negative sign");

    assert_eq!(
        state
            .expectation(&negative)
            .expect("expectation"),
        -1
    );
}

#[test]
fn negative_stabilizer_observable_has_negative_expectation() {
    let state =
        StabilizerState::new(1)
            .expect("state");

    let mut observable =
        pauli(
            1,
            &[(0, Pauli::Z)],
        );

    observable
        .set_sign(-1)
        .expect("negative Z");

    assert_eq!(
        state
            .expectation(&observable)
            .expect("expectation"),
        -1
    );

    assert!(
        !state
            .contains_stabilizer(&observable)
            .expect("membership")
    );
}

#[test]
fn contains_stabilizer_distinguishes_signed_membership() {
    let mut state =
        StabilizerState::new(1)
            .expect("state");

    assert!(
        state
            .contains_stabilizer(
                &pauli(
                    1,
                    &[(0, Pauli::Z)],
                ),
            )
            .expect("+Z")
    );

    let mut negative_z =
        pauli(
            1,
            &[(0, Pauli::Z)],
        );

    negative_z
        .set_sign(-1)
        .expect("-Z");

    assert!(
        !state
            .contains_stabilizer(&negative_z)
            .expect("-Z membership")
    );

    state.x(0).expect("X");

    assert!(
        state
            .contains_stabilizer(&negative_z)
            .expect("-Z membership")
    );
}

// =============================================================================
// Measurement
// =============================================================================

#[test]
fn deterministic_z_measurement_does_not_consume_rng() {
    let mut state =
        StabilizerState::new(1)
            .expect("state");

    let mut rng =
        XorShift64::new(1234);

    let before = rng.state();

    let result =
        state
            .measure_z(0, &mut rng)
            .expect("measurement");

    assert_eq!(result.bit, 0);
    assert_eq!(result.eigenvalue, 1);
    assert!(result.deterministic);
    assert_eq!(
        rng.state(),
        before
    );

    state
        .validate()
        .expect("tableau");
}

#[test]
fn deterministic_x_measurement_after_hadamard_is_plus_one() {
    let mut state =
        StabilizerState::new(1)
            .expect("state");

    state.h(0).expect("H");

    let mut rng =
        XorShift64::new(7);

    let result =
        state
            .measure_x(0, &mut rng)
            .expect("measurement");

    assert_eq!(result.bit, 0);
    assert_eq!(result.eigenvalue, 1);
    assert!(result.deterministic);

    assert_eq!(
        state
            .expectation(
                &pauli(
                    1,
                    &[(0, Pauli::X)],
                ),
            )
            .expect("X"),
        1
    );
}

#[test]
fn deterministic_y_measurement_after_s_h_is_plus_one() {
    let mut state =
        StabilizerState::new(1)
            .expect("state");

    state.h(0).expect("H");
    state.s(0).expect("S");

    let mut rng =
        XorShift64::new(7);

    let result =
        state
            .measure_y(0, &mut rng)
            .expect("measurement");

    assert_eq!(result.bit, 0);
    assert_eq!(result.eigenvalue, 1);
    assert!(result.deterministic);
}

#[test]
fn random_measurement_consumes_rng_and_collapses_state() {
    let mut state =
        StabilizerState::new(1)
            .expect("state");

    state.h(0).expect("H");

    let mut rng =
        AlternatingRng::new();

    let first =
        state
            .measure_z(0, &mut rng)
            .expect("measurement");

    assert_eq!(first.bit, 0);
    assert!(!first.deterministic);

    expect(
        &state,
        &[(0, Pauli::Z)],
        1,
    );

    let mut state =
        StabilizerState::new(1)
            .expect("state");

    state.h(0).expect("H");

    let second =
        state
            .measure_z(0, &mut rng)
            .expect("measurement");

    assert_eq!(second.bit, 1);
    assert!(!second.deterministic);

    expect(
        &state,
        &[(0, Pauli::Z)],
        -1,
    );
}

#[test]
fn bell_z_measurements_are_perfectly_correlated() {
    let mut state =
        StabilizerState::new(2)
            .expect("state");

    state.h(0).expect("H");
    state.cnot(0, 1).expect("CNOT");

    let mut rng =
        AlternatingRng::new();

    let first =
        state
            .measure_z(0, &mut rng)
            .expect("first measurement");

    let second =
        state
            .measure_z(1, &mut rng)
            .expect("second measurement");

    assert_eq!(
        first.bit,
        second.bit
    );

    assert!(
        second.deterministic
    );

    state
        .validate()
        .expect("post-measurement tableau");
}

#[test]
fn arbitrary_pauli_measurement_of_bell_xx_is_deterministic() {
    let mut state =
        StabilizerState::new(2)
            .expect("state");

    state.h(0).expect("H");
    state.cnot(0, 1).expect("CNOT");

    let mut rng =
        XorShift64::new(99);

    let xx =
        pauli(
            2,
            &[
                (0, Pauli::X),
                (1, Pauli::X),
            ],
        );

    let result =
        state
            .measure_pauli(
                &xx,
                &mut rng,
            )
            .expect("XX measurement");

    assert_eq!(result.bit, 0);
    assert_eq!(result.eigenvalue, 1);
    assert!(result.deterministic);

    state
        .validate()
        .expect("tableau");
}

#[test]
fn signed_pauli_measurement_preserves_observable_sign_semantics() {
    let mut state =
        StabilizerState::new(1)
            .expect("state");

    let mut negative_z =
        pauli(
            1,
            &[(0, Pauli::Z)],
        );

    negative_z
        .set_sign(-1)
        .expect("-Z");

    let mut rng =
        XorShift64::new(10);

    let result =
        state
            .measure_pauli(
                &negative_z,
                &mut rng,
            )
            .expect("-Z measurement");

    assert_eq!(result.bit, 1);
    assert_eq!(result.eigenvalue, -1);
    assert!(result.deterministic);

    state
        .validate()
        .expect("tableau");
}

#[test]
fn identity_pauli_measurement_is_rejected() {
    let mut state =
        StabilizerState::new(2)
            .expect("state");

    let identity =
        PauliString::identity(2)
            .expect("identity");

    let mut rng =
        XorShift64::new(1);

    assert!(matches!(
        state.measure_pauli(
            &identity,
            &mut rng,
        ),
        Err(StabilizerError::EmptyPauliString)
    ));
}

#[test]
fn all_measurement_bases_are_dimension_safe() {
    let mut state =
        StabilizerState::new(3)
            .expect("state");

    let mut rng =
        XorShift64::new(55);

    for basis in [
        MeasurementBasis::X,
        MeasurementBasis::Y,
        MeasurementBasis::Z,
    ] {
        state
            .reset_zero(0, &mut rng)
            .expect("reset");

        let _ =
            state
                .measure(
                    0,
                    basis,
                    &mut rng,
                )
                .expect("measurement");

        state
            .validate()
            .expect("tableau");
    }
}

// =============================================================================
// Reset
// =============================================================================

#[test]
fn reset_zero_forces_zero_eigenstate() {
    let mut state =
        StabilizerState::new(1)
            .expect("state");

    state.x(0).expect("X");

    let mut rng =
        XorShift64::new(1);

    state
        .reset_zero(0, &mut rng)
        .expect("reset");

    expect(
        &state,
        &[(0, Pauli::Z)],
        1,
    );

    let measurement =
        state
            .measure_z(0, &mut rng)
            .expect("measurement");

    assert_eq!(
        measurement.bit,
        0
    );

    assert!(
        measurement.deterministic
    );
}

#[test]
fn reset_zero_many_is_order_deterministic() {
    let mut state =
        StabilizerState::new(4)
            .expect("state");

    state.x(0).expect("X");
    state.x(2).expect("X");
    state.h(3).expect("H");

    let mut rng =
        XorShift64::new(42);

    state
        .reset_zero_many(
            &[3, 0, 2, 1],
            &mut rng,
        )
        .expect("reset many");

    for qubit in 0..4 {
        expect(
            &state,
            &[(qubit, Pauli::Z)],
            1,
        );
    }

    state
        .validate()
        .expect("tableau");
}

// =============================================================================
// Pauli frame
// =============================================================================

#[test]
fn pauli_frame_is_dimensioned_and_composable() {
    let state =
        StabilizerState::new(4)
            .expect("state");

    let mut first =
        state
            .pauli_frame()
            .expect("frame");

    let mut second =
        PauliFrame::new(4)
            .expect("frame");

    assert_eq!(
        first.qubits(),
        4
    );

    assert!(
        first.is_identity()
    );

    first
        .set(0, Pauli::X)
        .expect("X frame");

    second
        .set(0, Pauli::Z)
        .expect("Z frame");

    second
        .set(1, Pauli::Y)
        .expect("Y frame");

    first
        .compose(&second)
        .expect("compose");

    assert_eq!(
        first.get(0).expect("frame q0"),
        Pauli::Y
    );

    assert_eq!(
        first.get(1).expect("frame q1"),
        Pauli::Y
    );

    let observable =
        first.as_pauli_string();

    assert_eq!(
        observable
            .get(0)
            .expect("Pauli"),
        Pauli::Y
    );

    assert_eq!(
        observable
            .get(1)
            .expect("Pauli"),
        Pauli::Y
    );
}

#[test]
fn pauli_frame_rejects_dimension_mismatch() {
    let mut first =
        PauliFrame::new(2)
            .expect("frame");

    let second =
        PauliFrame::new(3)
            .expect("frame");

    assert!(matches!(
        first.compose(&second),
        Err(StabilizerError::DimensionMismatch {
            expected: 2,
            actual: 3
        })
    ));
}

#[test]
fn pauli_frame_out_of_range_is_rejected() {
    let mut frame =
        PauliFrame::new(2)
            .expect("frame");

    assert!(matches!(
        frame.set(2, Pauli::X),
        Err(StabilizerError::QubitOutOfRange {
            qubit: 2,
            qubits: 2
        })
    ));

    assert!(matches!(
        frame.get(2),
        Err(StabilizerError::QubitOutOfRange {
            qubit: 2,
            qubits: 2
        })
    ));
}

// =============================================================================
// RNG contract
// =============================================================================

#[test]
fn xorshift_seed_zero_is_non_absorbing_and_reproducible() {
    let mut a =
        XorShift64::new(0);

    let mut b =
        XorShift64::new(0);

    let first_a =
        a.next_u64();

    let first_b =
        b.next_u64();

    assert_ne!(
        first_a,
        0
    );

    assert_eq!(
        first_a,
        first_b
    );

    assert_eq!(
        a.state(),
        b.state()
    );
}

#[test]
fn xorshift_checkpoint_state_round_trips() {
    let mut rng =
        XorShift64::new(123456);

    let _ =
        rng.next_u64();

    let checkpoint =
        rng.state();

    let expected =
        rng.next_u64();

    let mut restored =
        XorShift64::from_state(
            checkpoint
        );

    assert_eq!(
        restored.next_u64(),
        expected
    );
}

#[test]
fn deterministic_measurement_does_not_consume_xorshift_rng() {
    let mut state =
        StabilizerState::new(1)
            .expect("state");

    let mut rng =
        XorShift64::new(77);

    let before =
        rng.state();

    let result =
        state
            .measure(
                0,
                MeasurementBasis::Z,
                &mut rng,
            )
            .expect("measurement");

    assert!(
        result.deterministic
    );

    assert_eq!(
        rng.state(),
        before
    );
}

#[test]
fn no_hidden_randomness_is_used_by_deterministic_paths() {
    let mut a =
        StabilizerState::new(2)
            .expect("state");

    let mut b =
        StabilizerState::new(2)
            .expect("state");

    a.h(0).expect("H");
    b.h(0).expect("H");

    let mut rng_a =
        XorShift64::new(1);

    let mut rng_b =
        XorShift64::new(
            987_654_321
        );

    let a_result =
        a.measure_x(
            0,
            &mut rng_a,
        )
        .expect("measurement");

    let b_result =
        b.measure_x(
            0,
            &mut rng_b,
        )
        .expect("measurement");

    assert_eq!(
        a_result,
        b_result
    );

    assert_eq!(
        rng_a.state(),
        1
    );

    assert_eq!(
        rng_b.state(),
        987_654_321
    );
}

// =============================================================================
// Memory-layout boundaries
// =============================================================================

#[test]
fn word_boundary_qubit_counts_remain_valid() {
    for qubits in [
        1usize,
        63,
        64,
        65,
        127,
        128,
        129,
    ] {
        let mut state =
            StabilizerState::new(qubits)
                .expect("state");

        state.h(0).expect("H0");

        state
            .h(qubits - 1)
            .expect("Hlast");

        if qubits > 1 {
            state
                .cnot(
                    0,
                    qubits - 1,
                )
                .expect("CNOT");

            state
                .cz(
                    qubits - 1,
                    0,
                )
                .expect("CZ");

            state
                .swap(
                    0,
                    qubits - 1,
                )
                .expect("SWAP");
        }

        state
            .validate()
            .expect("word-boundary tableau");
    }
}

#[test]
fn large_clifford_tableau_remains_polynomial_and_valid() {
    let mut state =
        StabilizerState::new(512)
            .expect(
                "512-qubit stabilizer state"
            );

    for qubit in 0..512 {
        if qubit % 3 == 0 {
            state.h(qubit).expect("H");
        }

        if qubit % 5 == 0 {
            state.s(qubit).expect("S");
        }
    }

    for qubit in (0..511).step_by(2) {
        state
            .cnot(
                qubit,
                qubit + 1,
            )
            .expect("CNOT");
    }

    state
        .validate()
        .expect(
            "large tableau must remain valid"
        );

    assert_eq!(
        state.tableau_rows(),
        1024
    );
}

// =============================================================================
// Ownership / cloning
// =============================================================================

#[test]
fn cloning_preserves_independent_logical_state() {
    let mut original =
        StabilizerState::new(3)
            .expect("state");

    original.h(0).expect("H");
    original.cnot(0, 1).expect("CNOT");

    let mut clone =
        original.clone();

    clone.x(2).expect("X");

    assert_ne!(
        original,
        clone
    );

    original
        .validate()
        .expect("original");

    clone
        .validate()
        .expect("clone");

    expect(
        &original,
        &[(2, Pauli::Z)],
        1,
    );

    expect(
        &clone,
        &[(2, Pauli::Z)],
        -1,
    );
}

// =============================================================================
// Hardware neutrality
// =============================================================================

#[test]
fn hardware_neutrality_is_preserved_by_the_public_contract() {
    // No provider, device, backend, accelerator or network object is needed
    // to exercise the mathematical stabilizer contract.
    //
    // This is intentional: CPU, GPU, distributed and QPU adapters must remain
    // outside this representation-level test.
    let mut state =
        StabilizerState::new(2)
            .expect("state");

    state
        .apply(
            CliffordGate::H {
                qubit: 0,
            },
        )
        .expect("H");

    state
        .apply(
            CliffordGate::Cnot {
                control: 0,
                target: 1,
            },
        )
        .expect("CNOT");

    expect(
        &state,
        &[
            (0, Pauli::X),
            (1, Pauli::X),
        ],
        1,
    );

    expect(
        &state,
        &[
            (0, Pauli::Z),
            (1, Pauli::Z),
        ],
        1,
    );
}