//! Zamani Quantum Noise (ZQN) — Differential Tests.
//!
//! # Ownership
//!
//! This file owns differential/conformance tests for mathematically equivalent
//! ZQN representations and independently derived reference calculations.
//!
//! It specifically verifies that independently implemented representations do
//! not silently disagree.
//!
//! # Does not own
//!
//! This file does not own:
//!
//! - quantum-channel mathematics;
//! - Kraus implementation;
//! - Choi implementation;
//! - process-matrix implementation;
//! - simulator implementation;
//! - numerical linear-algebra implementation;
//! - quantum IR;
//! - qubit identity;
//! - hardware;
//! - routing;
//! - scheduling;
//! - QEC;
//! - calibration;
//! - runtime policy;
//! - resource limits;
//! - production execution.
//!
//! # Differential-testing principle
//!
//! Differential testing must compare independently derived semantics rather
//! than simply calling the same implementation twice.
//!
//! The primary contract tested here is:
//!
//! ```text
//!             canonical Kraus data
//!                    │
//!             ┌──────┴──────┐
//!             ▼             ▼
//!       ZQN Kraus path   independent reference
//!             │             │
//!             │             │
//!             └──────┬──────┘
//!                    ▼
//!             expected channel
//!                    │
//!                    ▼
//!              Choi construction
//!                    │
//!                    ▼
//!           independent Choi reference
//! ```
//!
//! This catches representation-conversion errors that ordinary unit tests can
//! miss.
//!
//! # Representations covered
//!
//! The tests in this file currently cover the concrete representations that
//! have stable repository APIs:
//!
//! - Kraus;
//! - Choi.
//!
//! The test architecture intentionally leaves room for:
//!
//! - process matrix;
//! - superoperator;
//! - Pauli transfer;
//! - Liouville;
//! - stochastic;
//! - Lindblad;
//! - future representations.
//!
//! Those representations must only be added here after their public contracts
//! are stable. This prevents this test module from coupling prematurely to
//! implementation details.
//!
//! # Mathematical convention
//!
//! The Choi implementation uses:
//!
//! ```text
//! J(E) = Σ_r |K_r>> <<K_r|
//! ```
//!
//! with:
//!
//! ```text
//! row    = output_row * input_dimension + input_row
//! column = output_col * input_dimension + input_col
//!
//! J[(a,i),(b,j)]
//!     = Σ_r K_r[a,i] * conjugate(K_r[b,j])
//! ```
//!
//! The independent reference implementation below reproduces this equation
//! directly. It does not call ZQN's Choi implementation.
//!
//! # Scalability
//!
//! No machine-size limit is encoded here.
//!
//! The test dimensions are generated from small deterministic cases because a
//! test process itself has finite resources. That is a testing-resource policy,
//! not a ZQN semantic limit.
//!
//! The implementation deliberately avoids:
//!
//! ```text
//! MAX_QUBITS
//! MAX_QUDITS
//! MAX_CHANNEL_SIZE
//! MAX_KRAUS_OPERATORS
//! ```
//!
//! Instead, test cases are parameterized by dimensions supplied by the test
//! generator.
//!
//! The same reference algorithm works for any finite dimensions representable
//! by the underlying ZQN representation.
//!
//! # Determinism
//!
//! This module contains no global RNG and no wall-clock dependence.
//!
//! The pseudo-random-looking generated values are produced by a local,
//! deterministic state machine. The same seed always produces the same cases,
//! regardless of thread scheduling.
//!
//! This is important because a differential test failure must be reproducible.
//!
//! # Numerical policy
//!
//! Floating-point equality is never used for mathematical equivalence.
//!
//! Comparisons use an explicit absolute/relative tolerance.
//!
//! NaN and infinite values are rejected by the test helpers rather than being
//! silently normalized.
//!
//! # Resource safety
//!
//! Differential reference matrices are only materialized for dimensions
//! selected by the test case generator.
//!
//! Checked arithmetic is used before constructing reference storage.
//!
//! The test generator also uses bounded case counts so a corrupted generator
//! cannot accidentally create an unbounded test loop.
//!
//! These bounds are test-execution safeguards only. They are not ZQN semantic
//! limits.
//!
//! # Canonical quantum identity
//!
//! This file does not define `QubitId` or `PhysicalQubitId`.
//!
//! ZQN follows the repository's canonical identity boundary:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! No resource identity is needed by the mathematical differential tests,
//! because the representations under comparison describe channel semantics,
//! not physical placement.
//!
//! # Integration
//!
//! The intended module graph is:
//!
//! ```text
//! quantum::zqn::tests::differential
//!             │
//!             ├──────────────► channel::kraus
//!             │
//!             ├──────────────► channel::choi
//!             │
//!             └──────────────► channel::representation
//! ```
//!
//! This module is a consumer only.
//!
//! Concrete representation modules must not depend on this test module.
//!
//! # Integration with tests/mod.rs
//!
//! The ZQN test composition root should expose this module with:
//!
//! ```text
//! #[cfg(test)]
//! mod differential;
//! ```
//!
//! when `tests/mod.rs` is used as the unit-test composition root.
//!
//! Alternatively, if the ZQN module directly declares test modules, the
//! equivalent declaration belongs there.
//!
//! No production dependency is required.
//!
//! # Integration with Cargo
//!
//! No new dependency is required.
//!
//! The implementation uses only:
//!
//! - Rust standard library;
//! - existing ZQN channel APIs;
//! - existing Zamani `Complex64`.
//!
//! This is intentional because the repository currently has no property-testing
//! dependency and differential testing does not require one.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe.
//!
//! # Safety
//!
//! Unsafe Rust is explicitly forbidden.
//!
//! # File-completion contract
//!
//! This file is complete when:
//!
//! 1. Kraus-to-Choi conversion is compared against an independent reference;
//! 2. trace-preserving channels are validated through both representations;
//! 3. non-trace-preserving channels are not incorrectly accepted as TP;
//! 4. non-square input/output dimensions are tested;
//! 5. multiple Kraus operators are tested;
//! 6. identity channels are tested;
//! 7. deterministic generated cases are tested;
//! 8. canonical element ordering is tested;
//! 9. numerical comparison is tolerance-aware;
//! 10. non-finite reference values are rejected;
//! 11. integer overflow is checked;
//! 12. no fixed machine size is assumed;
//! 13. no vendor is referenced;
//! 14. no qubit identity is duplicated;
//! 15. no external test dependency is required;
//! 16. no unsafe code exists;
//! 17. failures identify the representation, dimensions, and matrix position;
//! 18. future representations can be added without changing the reference
//!     semantics already established here.
//!
//! ============================================================================
//! Implementation
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

#[cfg(test)]
mod tests {
    use crate::quantum::memory::complex::Complex64;
    use crate::quantum::zqn::channel::choi::{
        Choi,
        ChoiValidationTolerance,
    };
    use crate::quantum::zqn::channel::kraus::KrausChannel;

    // ========================================================================
    // Test policy
    // ========================================================================

    /// Absolute tolerance for independent differential comparisons.
    ///
    /// This is a test policy, not a ZQN semantic constant.
    const ABSOLUTE_TOLERANCE: f64 = 1.0e-10;

    /// Relative tolerance for independent differential comparisons.
    const RELATIVE_TOLERANCE: f64 = 1.0e-9;

    /// Number of generated differential cases.
    ///
    /// This bounds the test process itself and is deliberately unrelated to
    /// machine capacity.
    const GENERATED_CASES: usize = 64;

    /// Deterministic seed for the generated differential corpus.
    const TEST_SEED: u64 = 0x5A4E_5144_4946_4601;

    // ========================================================================
    // Deterministic scalar generator
    // ========================================================================

    /// Small deterministic generator used only by this test module.
    ///
    /// It is intentionally not a production ZQN RNG and must never be reused
    /// by simulation or execution code.
    #[derive(Clone, Copy, Debug)]
    struct DeterministicGenerator {
        state: u64,
    }

    impl DeterministicGenerator {
        const fn new(seed: u64) -> Self {
            Self { state: seed }
        }

        fn next_u64(&mut self) -> u64 {
            // SplitMix64-style deterministic state transition.
            //
            // This is used only to create a reproducible test corpus. It is
            // not intended to provide cryptographic randomness or physical
            // sampling semantics.
            self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);

            let mut value = self.state;
            value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);

            value ^ (value >> 31)
        }

        fn next_f64(&mut self) -> f64 {
            let value = self.next_u64() >> 11;
            (value as f64) / ((1u64 << 53) as f64)
        }

        fn next_signed(&mut self) -> f64 {
            self.next_f64() * 2.0 - 1.0
        }

        fn next_complex(&mut self) -> Complex64 {
            Complex64::new(
                self.next_signed() * 0.75,
                self.next_signed() * 0.75,
            )
        }
    }

    // ========================================================================
    // Reference model
    // ========================================================================

    /// Independent reference Choi matrix.
    ///
    /// This is intentionally separate from `Choi`.
    ///
    /// The reference implementation uses the mathematical definition directly
    /// rather than calling any ZQN conversion routine.
    #[derive(Clone, Debug)]
    struct ReferenceChoi {
        input_dimension: usize,
        output_dimension: usize,
        elements: Vec<Complex64>,
    }

    impl ReferenceChoi {
        fn from_kraus(
            input_dimension: usize,
            output_dimension: usize,
            kraus_operators: &[Vec<Complex64>],
        ) -> Self {
            assert!(
                input_dimension > 0,
                "reference input dimension must be non-zero"
            );
            assert!(
                output_dimension > 0,
                "reference output dimension must be non-zero"
            );

            let operator_dimension = checked_mul(
                input_dimension,
                output_dimension,
                "reference operator dimension",
            );

            let matrix_elements = checked_mul(
                operator_dimension,
                operator_dimension,
                "reference Choi element count",
            );

            for (operator_index, operator) in kraus_operators.iter().enumerate() {
                assert_eq!(
                    operator.len(),
                    operator_dimension,
                    "Kraus operator {operator_index} has {} elements; expected {operator_dimension}",
                    operator.len()
                );

                for (element_index, value) in operator.iter().enumerate() {
                    assert!(
                        value.is_finite(),
                        "Kraus operator {operator_index} contains non-finite element at index {element_index}"
                    );
                }
            }

            let mut elements = vec![Complex64::new(0.0, 0.0); matrix_elements];

            for operator in kraus_operators {
                for output_row in 0..output_dimension {
                    for input_row in 0..input_dimension {
                        let source_row = checked_add(
                            checked_mul(
                                output_row,
                                input_dimension,
                                "reference Kraus row",
                            ),
                            input_row,
                            "reference Kraus row",
                        );

                        let choi_row = source_row;

                        for output_column in 0..output_dimension {
                            for input_column in 0..input_dimension {
                                let source_column = checked_add(
                                    checked_mul(
                                        output_column,
                                        input_dimension,
                                        "reference Kraus column",
                                    ),
                                    input_column,
                                    "reference Kraus column",
                                );

                                let choi_column = source_column;

                                let matrix_index = checked_add(
                                    checked_mul(
                                        choi_row,
                                        operator_dimension,
                                        "reference Choi row offset",
                                    ),
                                    choi_column,
                                    "reference Choi index",
                                );

                                let contribution =
                                    operator[source_row] * operator[source_column].conjugate();

                                elements[matrix_index] =
                                    elements[matrix_index] + contribution;
                            }
                        }
                    }
                }
            }

            Self {
                input_dimension,
                output_dimension,
                elements,
            }
        }

        fn matrix_dimension(&self) -> usize {
            checked_mul(
                self.input_dimension,
                self.output_dimension,
                "reference matrix dimension",
            )
        }

        fn get(&self, row: usize, column: usize) -> Complex64 {
            let dimension = self.matrix_dimension();

            assert!(
                row < dimension,
                "reference row {row} outside dimension {dimension}"
            );
            assert!(
                column < dimension,
                "reference column {column} outside dimension {dimension}"
            );

            self.elements[row * dimension + column]
        }
    }

    // ========================================================================
    // Numerical helpers
    // ========================================================================

    fn approximately_equal(left: Complex64, right: Complex64) -> bool {
        if !left.is_finite() || !right.is_finite() {
            return false;
        }

        let difference = (left - right).magnitude();

        let scale = left
            .magnitude()
            .max(right.magnitude())
            .max(1.0);

        difference
            <= ABSOLUTE_TOLERANCE.max(RELATIVE_TOLERANCE * scale)
    }

    fn assert_complex_close(
        left: Complex64,
        right: Complex64,
        row: usize,
        column: usize,
        context: &str,
    ) {
        assert!(
            approximately_equal(left, right),
            "{context}: mismatch at ({row}, {column}): left={left:?}, right={right:?}, \
             absolute_tolerance={ABSOLUTE_TOLERANCE}, \
             relative_tolerance={RELATIVE_TOLERANCE}"
        );
    }

    fn checked_mul(left: usize, right: usize, context: &str) -> usize {
        left.checked_mul(right)
            .unwrap_or_else(|| panic!("{context}: usize multiplication overflow"))
    }

    fn checked_add(left: usize, right: usize, context: &str) -> usize {
        left.checked_add(right)
            .unwrap_or_else(|| panic!("{context}: usize addition overflow"))
    }

    fn choi_tolerance() -> ChoiValidationTolerance {
        ChoiValidationTolerance::new(
            ABSOLUTE_TOLERANCE,
            RELATIVE_TOLERANCE,
        )
        .expect("differential-test tolerance must be valid")
    }

    // ========================================================================
    // Test-data construction
    // ========================================================================

    /// Creates a deterministic trace-preserving diagonal channel.
    ///
    /// For probabilities p and 1-p:
    ///
    /// ```text
    /// K0 = diag(sqrt(p), sqrt(1-p))
    /// K1 = diag(sqrt(1-p), -sqrt(p))
    /// ```
    ///
    /// and:
    ///
    /// ```text
    /// K0†K0 + K1†K1 = I.
    /// ```
    fn qubit_trace_preserving_channel(
        probability: f64,
    ) -> Vec<Vec<Complex64>> {
        assert!(
            probability.is_finite(),
            "test probability must be finite"
        );
        assert!(
            (0.0..=1.0).contains(&probability),
            "test probability must be within [0, 1]"
        );

        let p = probability.sqrt();
        let q = (1.0 - probability).sqrt();

        vec![
            vec![
                Complex64::new(p, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(q, 0.0),
            ],
            vec![
                Complex64::new(q, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(-p, 0.0),
            ],
        ]
    }

    /// Creates a deterministic identity channel for arbitrary finite
    /// dimension.
    fn identity_kraus(dimension: usize) -> Vec<Vec<Complex64>> {
        assert!(dimension > 0, "identity dimension must be non-zero");

        let elements = checked_mul(
            dimension,
            dimension,
            "identity matrix element count",
        );

        let mut identity = vec![Complex64::new(0.0, 0.0); elements];

        for index in 0..dimension {
            let matrix_index = checked_add(
                checked_mul(index, dimension, "identity row offset"),
                index,
                "identity diagonal index",
            );

            identity[matrix_index] = Complex64::new(1.0, 0.0);
        }

        vec![identity]
    }

    /// Creates a rectangular completely-positive trace-nonincreasing
    /// one-Kraus operation.
    ///
    /// The dimensions intentionally need not be equal. This catches accidental
    /// qubit/square-matrix assumptions in representation conversion.
    fn rectangular_kraus(
        input_dimension: usize,
        output_dimension: usize,
    ) -> Vec<Vec<Complex64>> {
        let elements = checked_mul(
            input_dimension,
            output_dimension,
            "rectangular Kraus element count",
        );

        let mut operator = Vec::with_capacity(elements);

        for index in 0..elements {
            let row = index / input_dimension;
            let column = index % input_dimension;

            let value = if row == column {
                0.5
            } else if (row + column) % 3 == 0 {
                0.125
            } else {
                0.0
            };

            operator.push(Complex64::new(value, 0.0));
        }

        vec![operator]
    }

    // ========================================================================
    // Differential comparison
    // ========================================================================

    fn assert_kraus_to_choi_matches_reference(
        input_dimension: usize,
        output_dimension: usize,
        kraus: &[Vec<Complex64>],
        context: &str,
    ) {
        let expected = ReferenceChoi::from_kraus(
            input_dimension,
            output_dimension,
            kraus,
        );

        let actual = Choi::from_kraus(
            input_dimension as u128,
            output_dimension as u128,
            kraus,
        )
        .unwrap_or_else(|error| {
            panic!(
                "{context}: ZQN Choi construction failed for \
                 input_dimension={input_dimension}, \
                 output_dimension={output_dimension}: {error:?}"
            )
        });

        assert_eq!(
            actual.input_dimension() as usize,
            input_dimension,
            "{context}: Choi input dimension changed"
        );

        assert_eq!(
            actual.output_dimension() as usize,
            output_dimension,
            "{context}: Choi output dimension changed"
        );

        assert_eq!(
            actual.matrix_dimension() as usize,
            expected.matrix_dimension(),
            "{context}: Choi matrix dimension changed"
        );

        let actual_elements: Vec<_> = actual.indexed_elements().collect();

        let expected_dimension = expected.matrix_dimension();

        let expected_element_count = checked_mul(
            expected_dimension,
            expected_dimension,
            "expected differential element count",
        );

        assert_eq!(
            actual_elements.len(),
            expected_element_count,
            "{context}: unexpected number of indexed Choi elements"
        );

        for (row, column, actual_value) in actual_elements {
            let row = row as usize;
            let column = column as usize;

            let expected_value = expected.get(row, column);

            assert_complex_close(
                actual_value,
                expected_value,
                row,
                column,
                context,
            );
        }
    }

    // ========================================================================
    // Fundamental representation tests
    // ========================================================================

    #[test]
    fn kraus_and_choi_identity_are_differentially_equivalent() {
        for dimension in [1usize, 2, 3, 4, 5] {
            let kraus = identity_kraus(dimension);

            assert_kraus_to_choi_matches_reference(
                dimension,
                dimension,
                &kraus,
                "identity Kraus→Choi differential comparison",
            );

            let channel = KrausChannel::identity(dimension)
                .expect("identity Kraus channel must construct");

            assert_eq!(
                channel.input_dimension(),
                dimension,
                "identity channel input dimension must remain generic"
            );

            assert_eq!(
                channel.output_dimension(),
                dimension,
                "identity channel output dimension must remain generic"
            );
        }
    }

    #[test]
    fn nontrivial_kraus_channel_matches_independent_choi_reference() {
        let kraus = qubit_trace_preserving_channel(0.37);

        assert_kraus_to_choi_matches_reference(
            2,
            2,
            &kraus,
            "nontrivial qubit channel differential comparison",
        );

        let channel = KrausChannel::new(kraus)
            .expect("trace-preserving test channel must construct");

        channel
            .validate_trace_preserving(
                crate::quantum::zqn::channel::kraus::KrausTolerance::new(
                    ABSOLUTE_TOLERANCE,
                    RELATIVE_TOLERANCE,
                )
                .expect("Kraus differential tolerance must be valid"),
            )
            .expect("reference trace-preserving channel must remain TP");
    }

    #[test]
    fn choi_conversion_preserves_trace_preservation() {
        let kraus = qubit_trace_preserving_channel(0.23);

        let channel = KrausChannel::new(kraus.clone())
            .expect("trace-preserving channel must construct");

        channel
            .validate_trace_preserving(
                crate::quantum::zqn::channel::kraus::KrausTolerance::new(
                    ABSOLUTE_TOLERANCE,
                    RELATIVE_TOLERANCE,
                )
                .expect("Kraus tolerance must be valid"),
            )
            .expect("Kraus representation must be trace preserving");

        let choi = Choi::from_kraus(2, 2, &kraus)
            .expect("Choi conversion must construct");

        choi
            .validate_trace_preserving(choi_tolerance())
            .expect("equivalent Choi representation must be trace preserving");

        choi
            .validate_positive_semidefinite(choi_tolerance())
            .expect("Kraus-derived Choi matrix must be positive semidefinite");
    }

    #[test]
    fn kraus_and_choi_agree_for_rectangular_channels() {
        // This is deliberately not a qubit-only case.
        //
        // Input dimension = 2.
        // Output dimension = 3.
        //
        // The operation is completely positive but not trace preserving.
        let kraus = rectangular_kraus(2, 3);

        assert_kraus_to_choi_matches_reference(
            2,
            3,
            &kraus,
            "rectangular 2→3 differential comparison",
        );

        let choi = Choi::from_kraus(2, 3, &kraus)
            .expect("rectangular Choi construction must succeed");

        assert_eq!(choi.input_dimension(), 2);
        assert_eq!(choi.output_dimension(), 3);
        assert_eq!(choi.matrix_dimension(), 6);

        choi
            .validate_positive_semidefinite(choi_tolerance())
            .expect("Kraus-derived rectangular Choi must remain positive semidefinite");
    }

    #[test]
    fn multiple_kraus_operators_are_not_collapsed_or_reordered() {
        let kraus = vec![
            vec![
                Complex64::new(0.6, 0.0),
                Complex64::new(0.0, 0.1),
                Complex64::new(0.0, -0.1),
                Complex64::new(0.4, 0.0),
            ],
            vec![
                Complex64::new(0.2, 0.0),
                Complex64::new(0.3, 0.0),
                Complex64::new(-0.1, 0.0),
                Complex64::new(0.1, 0.0),
            ],
            vec![
                Complex64::new(0.0, 0.2),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, -0.2),
            ],
        ];

        assert_kraus_to_choi_matches_reference(
            2,
            2,
            &kraus,
            "multi-Kraus differential comparison",
        );
    }

    // ========================================================================
    // Structural invariants
    // ========================================================================

    #[test]
    fn choi_index_order_is_deterministic() {
        let kraus = identity_kraus(3);

        let first = Choi::from_kraus(3, 3, &kraus)
            .expect("first Choi construction must succeed");

        let second = Choi::from_kraus(3, 3, &kraus)
            .expect("second Choi construction must succeed");

        let first_elements: Vec<_> = first.indexed_elements().collect();
        let second_elements: Vec<_> = second.indexed_elements().collect();

        assert_eq!(
            first_elements,
            second_elements,
            "identical mathematical input must produce deterministic Choi iteration"
        );
    }

    #[test]
    fn generated_dimensions_are_not_semantically_fixed_to_qubits() {
        let dimensions = [1usize, 2, 3, 4, 5, 7];

        for input_dimension in dimensions {
            for output_dimension in dimensions {
                let kraus = rectangular_kraus(
                    input_dimension,
                    output_dimension,
                );

                assert_kraus_to_choi_matches_reference(
                    input_dimension,
                    output_dimension,
                    &kraus,
                    "dimension-generic differential comparison",
                );
            }
        }
    }

    // ========================================================================
    // Deterministic generated differential corpus
    // ========================================================================

    #[test]
    fn deterministic_generated_kraus_corpus_matches_reference() {
        let mut generator = DeterministicGenerator::new(TEST_SEED);

        for case_index in 0..GENERATED_CASES {
            // Dimensions are generated from data, not from a hard-coded
            // assumption that a quantum system contains two-level qubits.
            let input_dimension =
                1usize + (generator.next_u64() % 5) as usize;

            let output_dimension =
                1usize + (generator.next_u64() % 5) as usize;

            let operator_count =
                1usize + (generator.next_u64() % 4) as usize;

            let operator_size = checked_mul(
                input_dimension,
                output_dimension,
                "generated operator size",
            );

            let mut kraus = Vec::with_capacity(operator_count);

            for _ in 0..operator_count {
                let mut operator = Vec::with_capacity(operator_size);

                for _ in 0..operator_size {
                    operator.push(generator.next_complex());
                }

                kraus.push(operator);
            }

            assert_kraus_to_choi_matches_reference(
                input_dimension,
                output_dimension,
                &kraus,
                &format!(
                    "generated differential case {case_index} \
                     ({input_dimension}→{output_dimension}, \
                     {operator_count} Kraus operators)"
                ),
            );
        }
    }

    #[test]
    fn deterministic_generator_reproduces_the_same_corpus() {
        let mut first = DeterministicGenerator::new(TEST_SEED);
        let mut second = DeterministicGenerator::new(TEST_SEED);

        for _ in 0..GENERATED_CASES {
            let first_input = 1usize + (first.next_u64() % 5) as usize;
            let second_input = 1usize + (second.next_u64() % 5) as usize;

            assert_eq!(first_input, second_input);

            let first_output = 1usize + (first.next_u64() % 5) as usize;
            let second_output = 1usize + (second.next_u64() % 5) as usize;

            assert_eq!(first_output, second_output);

            let first_count = 1usize + (first.next_u64() % 4) as usize;
            let second_count = 1usize + (second.next_u64() % 4) as usize;

            assert_eq!(first_count, second_count);

            let element_count = checked_mul(
                first_input,
                first_output,
                "deterministic corpus element count",
            );

            for _ in 0..first_count {
                for _ in 0..element_count {
                    let left = first.next_complex();
                    let right = second.next_complex();

                    assert_complex_close(
                        left,
                        right,
                        0,
                        0,
                        "deterministic generator comparison",
                    );
                }
            }
        }
    }

    // ========================================================================
    // Mathematical sanity checks independent of ZQN conversion
    // ========================================================================

    #[test]
    fn independent_reference_identity_has_expected_choi_structure() {
        let kraus = identity_kraus(3);

        let reference = ReferenceChoi::from_kraus(3, 3, &kraus);

        let dimension = reference.matrix_dimension();

        assert_eq!(dimension, 9);

        for row in 0..dimension {
            for column in 0..dimension {
                let expected = if row == column {
                    Complex64::new(1.0, 0.0)
                } else {
                    Complex64::new(0.0, 0.0)
                };

                assert_complex_close(
                    reference.get(row, column),
                    expected,
                    row,
                    column,
                    "independent identity Choi reference",
                );
            }
        }
    }

    #[test]
    fn independent_reference_is_hermitian_for_arbitrary_kraus_data() {
        let mut generator = DeterministicGenerator::new(TEST_SEED ^ 0xA5A5);

        for _ in 0..16 {
            let input_dimension =
                1usize + (generator.next_u64() % 4) as usize;

            let output_dimension =
                1usize + (generator.next_u64() % 4) as usize;

            let operator_count =
                1usize + (generator.next_u64() % 3) as usize;

            let operator_size = checked_mul(
                input_dimension,
                output_dimension,
                "Hermitian reference operator size",
            );

            let mut kraus = Vec::with_capacity(operator_count);

            for _ in 0..operator_count {
                let mut operator = Vec::with_capacity(operator_size);

                for _ in 0..operator_size {
                    operator.push(generator.next_complex());
                }

                kraus.push(operator);
            }

            let reference = ReferenceChoi::from_kraus(
                input_dimension,
                output_dimension,
                &kraus,
            );

            let dimension = reference.matrix_dimension();

            for row in 0..dimension {
                for column in 0..dimension {
                    assert_complex_close(
                        reference.get(row, column),
                        reference.get(column, row).conjugate(),
                        row,
                        column,
                        "independent Choi Hermiticity",
                    );
                }
            }
        }
    }

    // ========================================================================
    // Failure-mode differential checks
    // ========================================================================

    #[test]
    fn non_finite_kraus_reference_data_is_rejected_by_test_reference() {
        let kraus = vec![vec![
            Complex64::new(f64::NAN, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(1.0, 0.0),
        ]];

        let contains_non_finite = kraus
            .iter()
            .flat_map(|operator| operator.iter())
            .any(|value| !value.is_finite());

        assert!(
            contains_non_finite,
            "the negative differential fixture must actually contain a non-finite value"
        );

        let result = Choi::from_kraus(2, 2, &kraus);

        assert!(
            result.is_err(),
            "ZQN must reject non-finite Kraus input rather than silently repairing it"
        );
    }

    #[test]
    fn empty_kraus_set_is_rejected() {
        let result = Choi::from_kraus(2, 2, &[]);

        assert!(
            result.is_err(),
            "an empty Kraus representation must not silently become an invalid channel"
        );
    }

    #[test]
    fn zero_dimensions_are_rejected() {
        assert!(
            Choi::from_kraus(0, 2, &[]).is_err(),
            "zero input dimension must be rejected"
        );

        assert!(
            Choi::from_kraus(2, 0, &[]).is_err(),
            "zero output dimension must be rejected"
        );
    }

    // ========================================================================
    // Resource arithmetic
    // ========================================================================

    #[test]
    fn reference_dimension_arithmetic_is_checked() {
        assert_eq!(
            checked_mul(3, 7, "test multiplication"),
            21
        );

        assert_eq!(
            checked_add(11, 13, "test addition"),
            24
        );
    }

    #[test]
    #[should_panic(expected = "usize multiplication overflow")]
    fn reference_multiplication_does_not_wrap() {
        let _ = checked_mul(
            usize::MAX,
            2,
            "usize multiplication overflow",
        );
    }

    #[test]
    #[should_panic(expected = "usize addition overflow")]
    fn reference_addition_does_not_wrap() {
        let _ = checked_add(
            usize::MAX,
            1,
            "usize addition overflow",
        );
    }
}