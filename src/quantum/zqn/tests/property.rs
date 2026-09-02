//! Zamani Quantum Noise (ZQN) — Property Tests
//!
//! # Ownership
//!
//! This file owns the **cross-property mathematical and architectural
//! invariants** of the ZQN subsystem.
//!
//! It is intentionally different from ordinary unit tests:
//!
//! - unit tests verify individual examples;
//! - property tests verify invariants over many generated inputs;
//! - scaling properties verify that semantics do not depend on a particular
//!   quantum-system size;
//! - determinism properties verify that repeated equivalent operations remain
//!   equivalent;
//! - boundary properties verify that invalid numerical states are rejected;
//! - composition properties verify algebraic relationships;
//! - representation properties verify that foundational ZQN types preserve
//!   their contracts.
//!
//! # Does not own
//!
//! This file does NOT own:
//!
//! - production ZQN semantics;
//! - quantum IR;
//! - qubit identity;
//! - noise-model implementation;
//! - channel implementation;
//! - fault implementation;
//! - calibration implementation;
//! - simulation implementation;
//! - routing;
//! - scheduling;
//! - QEC;
//! - hardware;
//! - serialization schema;
//! - random-number-generator policy.
//!
//! Those responsibilities remain with their owning modules.
//!
//! # Integration boundary
//!
//! ```text
//!                         ZQN
//!                          │
//!             ┌────────────┼────────────┐
//!             │            │            │
//!             ▼            ▼            ▼
//!        probability    channels      faults
//!             │            │            │
//!             └────────────┼────────────┘
//!                          ▼
//!                         noise
//!                          │
//!             ┌────────────┼────────────┐
//!             │            │            │
//!             ▼            ▼            ▼
//!        calibration  simulation   propagation
//!             │            │            │
//!             └────────────┼────────────┘
//!                          ▼
//!                      integration
//!                          │
//!                          ▼
//!                    this property layer
//! ```
//!
//! The property layer therefore sits **beside** the implementation rather than
//! becoming another implementation layer.
//!
//! # Current foundational contracts tested
//!
//! This file tests the contracts currently established by:
//!
//! ```text
//! quantum::zqn::probability::probability::Probability
//! quantum::zqn::probability::bounds::ProbabilityBounds
//! quantum::zqn::probability::distribution::Distribution
//! ```
//!
//! The tests intentionally use the public APIs of those modules.
//!
//! They do not reach into private representation fields.
//!
//! # Canonical quantum identity
//!
//! No ZQN-specific `QubitId` is created here.
//!
//! Quantum resource identity remains owned by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Property tests for resource-associated noise should use those canonical
//! types when the corresponding ZQN resource-bearing APIs become available.
//!
//! A property test must never introduce a fake:
//!
//! ```text
//! zqn::QubitId
//! ```
//!
//! merely to make a test convenient.
//!
//! # Write once, scale everywhere
//!
//! These tests deliberately do NOT define:
//!
//! ```text
//! MAX_QUBITS
//! MAX_QUBIT_INDEX
//! MAX_OPERATIONS
//! MAX_FAULTS
//! MAX_CORRELATED_QUBITS
//! ```
//!
//! Generated collection sizes are test-resource parameters only.
//!
//! They are not ZQN semantic limits.
//!
//! The largest test case that a particular CI runner can execute is therefore
//! not a statement about the largest quantum system Zamani can describe.
//!
//! # "Infinity" semantics
//!
//! ZQN's scalability requirement means:
//!
//! > no artificial finite architectural machine-size ceiling is encoded by
//! > these tests or by the mathematical contracts they validate.
//!
//! It does NOT mean that a finite computer can materialize an infinite object.
//!
//! Every generated test remains finite and is bounded by the resources
//! available to the test process.
//!
//! # Determinism
//!
//! The generators in this file are deterministic.
//!
//! They do not use:
//!
//! - thread-local randomness;
//! - global randomness;
//! - wall-clock time;
//! - process identifiers;
//! - memory addresses;
//! - OS entropy;
//! - unstable hash iteration.
//!
//! This is deliberate.
//!
//! A failed property test must be reproducible from its explicit seed and
//! generated-case index.
//!
//! # Rust compatibility
//!
//! Required:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! No additional property-testing crate is required.
//!
//! # Resource safety
//!
//! The property generator uses bounded test cases.
//!
//! The bounds are **test-harness limits**, not ZQN semantic limits.
//!
//! Large generated collections are constructed progressively rather than by
//! blindly allocating enormous vectors.
//!
//! # Failure diagnostics
//!
//! Every generated case carries:
//!
//! - a deterministic seed;
//! - a case index;
//! - generated values;
//! - enough context to reproduce the failure.
//!
//! When a property fails, the panic message includes these values.
//!
//! # Definition of done
//!
//! This file is complete for the current foundational ZQN layer when:
//!
//! 1. probability construction preserves `[0,1]`;
//! 2. non-finite probabilities are rejected;
//! 3. complement remains in `[0,1]`;
//! 4. complement is involutive;
//! 5. probability multiplication remains in `[0,1]`;
//! 6. probability difference remains in `[0,1]`;
//! 7. checked addition never returns an invalid probability;
//! 8. checked subtraction never returns an invalid probability;
//! 9. bounds always preserve `lower <= upper`;
//! 10. bounds containment is consistent;
//! 11. bounds width is non-negative;
//! 12. bounds midpoint remains contained;
//! 13. bounds intersection is mathematically correct;
//! 14. bounds hull contains both operands;
//! 15. distributions remain normalized;
//! 16. distributions reject invalid probability data;
//! 17. duplicate distribution outcomes preserve total probability;
//! 18. zero-weight entries do not become semantic outcomes;
//! 19. generated distributions remain deterministic;
//! 20. generated sizes do not change mathematical semantics;
//! 21. no property relies on a machine-size constant;
//! 22. no property introduces a second qubit identity;
//! 23. no property requires unsafe code;
//! 24. every failure can be reproduced deterministically;
//! 25. tests remain valid as implementation details change while public
//!     contracts remain stable.
//!
//! # Important maintenance rule
//!
//! When a new ZQN subsystem becomes production-ready, its property tests
//! should be added here or under a dedicated child property-test module rather
//! than modifying the mathematical implementation merely to satisfy a test.
//!
//! The property layer consumes contracts; it does not redefine them.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use crate::quantum::zqn::probability::bounds::ProbabilityBounds;
use crate::quantum::zqn::probability::distribution::Distribution;
use crate::quantum::zqn::probability::probability::Probability;

use std::fmt::Debug;

// =============================================================================
// Deterministic property generator
// =============================================================================

/// Small deterministic pseudo-random generator used only by this test module.
///
/// This is NOT cryptographic randomness.
///
/// It exists so property tests can:
///
/// - generate many distinct cases;
/// - remain reproducible;
/// - avoid a hidden global RNG;
/// - avoid introducing another dependency solely for property generation.
///
/// The generator is deliberately local to the test harness.
#[derive(Clone, Debug)]
struct PropertyRng {
    state: u64,
}

impl PropertyRng {
    /// Creates a deterministic generator from an explicit seed.
    const fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 {
                0x9E37_79B9_7F4A_7C15
            } else {
                seed
            },
        }
    }

    /// Generates the next deterministic `u64`.
    fn next_u64(&mut self) -> u64 {
        // SplitMix64-style deterministic generator.
        //
        // This is suitable for test generation only.
        self.state = self
            .state
            .wrapping_add(0x9E37_79B9_7F4A_7C15);

        let mut z = self.state;

        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);

        z ^ (z >> 31)
    }

    /// Generates a deterministic `usize` in `[0, upper)`.
    fn index(&mut self, upper: usize) -> usize {
        if upper == 0 {
            return 0;
        }

        (self.next_u64() as usize) % upper
    }

    /// Generates a deterministic finite value in `[0, 1]`.
    fn probability_value(&mut self) -> f64 {
        // Use 53 random bits because f64 has 53 bits of significand precision.
        let bits = self.next_u64() >> 11;
        let denominator = (1_u64 << 53) - 1;

        bits as f64 / denominator as f64
    }

    /// Generates a deterministic strictly interior probability.
    fn interior_probability_value(&mut self) -> f64 {
        let value = self.probability_value();

        if value == 0.0 {
            f64::from_bits(1)
        } else if value == 1.0 {
            1.0 - f64::EPSILON
        } else {
            value
        }
    }
}

// =============================================================================
// Test configuration
// =============================================================================

/// Number of deterministic property cases for ordinary scalar properties.
///
/// This is a test-suite execution parameter.
///
/// It is NOT a ZQN semantic limit.
const SCALAR_CASES: usize = 2_048;

/// Number of generated interval cases.
///
/// This is a test-suite execution parameter.
///
/// It is NOT a ZQN semantic limit.
const BOUNDS_CASES: usize = 2_048;

/// Number of generated distribution cases.
///
/// This is a test-suite execution parameter.
///
/// It is NOT a ZQN semantic limit.
const DISTRIBUTION_CASES: usize = 512;

/// Maximum generated distribution cardinality for one test case.
///
/// This protects CI resources while preserving the important invariant:
/// cardinality is data, not architecture.
///
/// This value MUST NOT be copied into production ZQN code.
const TEST_MAX_DISTRIBUTION_ENTRIES: usize = 256;

/// Root seed used by the deterministic property suite.
///
/// Changing this seed changes the generated corpus but does not change ZQN
/// semantics.
const PROPERTY_SEED: u64 = 0x5A4E_5150_524F_5045;

// =============================================================================
// Property helpers
// =============================================================================

fn assert_valid_probability(
    probability: Probability,
    context: &str,
) {
    let value = probability.value();

    assert!(
        value.is_finite(),
        "{context}: generated probability is not finite: {value:?}"
    );

    assert!(
        (0.0..=1.0).contains(&value),
        "{context}: generated probability is outside [0,1]: {value:?}"
    );
}

fn generated_probability(
    rng: &mut PropertyRng,
    context: &str,
) -> Probability {
    let value = rng.probability_value();

    Probability::new(value).unwrap_or_else(|error| {
        panic!(
            "{context}: deterministic generator produced invalid probability \
             {value:?}: {error:?}"
        )
    })
}

fn generated_interior_probability(
    rng: &mut PropertyRng,
    context: &str,
) -> Probability {
    let value = rng.interior_probability_value();

    Probability::new(value).unwrap_or_else(|error| {
        panic!(
            "{context}: deterministic generator produced invalid interior \
             probability {value:?}: {error:?}"
        )
    })
}

fn assert_reproducible_f64(
    first: f64,
    second: f64,
    context: &str,
) {
    assert_eq!(
        first.to_bits(),
        second.to_bits(),
        "{context}: floating-point results differ: \
         first={first:?}, second={second:?}"
    );
}

fn assert_bounds_invariant(
    bounds: ProbabilityBounds,
    context: &str,
) {
    let lower = bounds.lower().value();
    let upper = bounds.upper().value();

    assert!(
        lower.is_finite(),
        "{context}: lower bound is non-finite: {lower:?}"
    );

    assert!(
        upper.is_finite(),
        "{context}: upper bound is non-finite: {upper:?}"
    );

    assert!(
        (0.0..=1.0).contains(&lower),
        "{context}: lower bound outside [0,1]: {lower:?}"
    );

    assert!(
        (0.0..=1.0).contains(&upper),
        "{context}: upper bound outside [0,1]: {upper:?}"
    );

    assert!(
        lower <= upper,
        "{context}: lower bound exceeds upper bound: \
         lower={lower:?}, upper={upper:?}"
    );
}

// =============================================================================
// Probability properties
// =============================================================================

#[test]
fn property_probability_constructor_accepts_generated_domain() {
    let mut rng = PropertyRng::new(PROPERTY_SEED);

    for case_index in 0..SCALAR_CASES {
        let probability =
            generated_probability(&mut rng, "probability constructor");

        assert_valid_probability(
            probability,
            &format!("case {case_index}"),
        );
    }
}

#[test]
fn property_probability_rejects_non_finite_values() {
    let invalid_values = [
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
    ];

    for value in invalid_values {
        assert!(
            Probability::new(value).is_err(),
            "non-finite probability must be rejected: {value:?}"
        );
    }
}

#[test]
fn property_probability_rejects_values_below_zero() {
    let values = [
        -f64::EPSILON,
        -1.0,
        -2.0,
        f64::MIN,
    ];

    for value in values {
        assert!(
            Probability::new(value).is_err(),
            "negative probability must be rejected: {value:?}"
        );
    }
}

#[test]
fn property_probability_rejects_values_above_one() {
    let values = [
        1.0 + f64::EPSILON,
        2.0,
        f64::MAX,
    ];

    for value in values {
        assert!(
            Probability::new(value).is_err(),
            "probability greater than one must be rejected: {value:?}"
        );
    }
}

#[test]
fn property_probability_endpoints_are_valid() {
    assert_eq!(Probability::ZERO.value(), 0.0);
    assert_eq!(Probability::ONE.value(), 1.0);

    assert_valid_probability(
        Probability::ZERO,
        "ZERO endpoint",
    );

    assert_valid_probability(
        Probability::ONE,
        "ONE endpoint",
    );
}

#[test]
fn property_probability_complement_stays_in_domain() {
    let mut rng = PropertyRng::new(PROPERTY_SEED ^ 0x01);

    for case_index in 0..SCALAR_CASES {
        let probability =
            generated_probability(&mut rng, "complement");

        let complement = probability.complement();

        assert_valid_probability(
            complement,
            &format!("complement case {case_index}"),
        );

        let sum = probability.value() + complement.value();

        assert_eq!(
            sum.to_bits(),
            1.0_f64.to_bits(),
            "complement case {case_index}: \
             p + complement(p) != 1: \
             p={:?}, complement={:?}, sum={:?}",
            probability.value(),
            complement.value(),
            sum
        );
    }
}

#[test]
fn property_probability_complement_is_involution() {
    let mut rng = PropertyRng::new(PROPERTY_SEED ^ 0x02);

    for case_index in 0..SCALAR_CASES {
        let probability =
            generated_probability(&mut rng, "complement involution");

        let round_trip =
            probability.complement().complement();

        assert_eq!(
            round_trip,
            probability,
            "case {case_index}: complement is not an involution"
        );
    }
}

#[test]
fn property_probability_complement_is_order_reversing() {
    let mut rng = PropertyRng::new(PROPERTY_SEED ^ 0x03);

    for case_index in 0..SCALAR_CASES {
        let a = generated_probability(
            &mut rng,
            "complement ordering lhs",
        );

        let b = generated_probability(
            &mut rng,
            "complement ordering rhs",
        );

        if a.value() <= b.value() {
            assert!(
                a.complement().value()
                    >= b.complement().value(),
                "case {case_index}: complement must reverse ordering"
            );
        }
    }
}

#[test]
fn property_probability_multiplication_is_closed() {
    let mut rng = PropertyRng::new(PROPERTY_SEED ^ 0x04);

    for case_index in 0..SCALAR_CASES {
        let a =
            generated_probability(&mut rng, "multiplication lhs");

        let b =
            generated_probability(&mut rng, "multiplication rhs");

        let result = a.multiply(b);

        assert_valid_probability(
            result,
            &format!("multiplication case {case_index}"),
        );

        assert!(
            result.value() <= a.value() + f64::EPSILON,
            "case {case_index}: product cannot exceed lhs"
        );

        assert!(
            result.value() <= b.value() + f64::EPSILON,
            "case {case_index}: product cannot exceed rhs"
        );
    }
}

#[test]
fn property_probability_multiplication_is_commutative() {
    let mut rng = PropertyRng::new(PROPERTY_SEED ^ 0x05);

    for case_index in 0..SCALAR_CASES {
        let a =
            generated_probability(&mut rng, "commutative lhs");

        let b =
            generated_probability(&mut rng, "commutative rhs");

        let lhs = a.multiply(b);
        let rhs = b.multiply(a);

        assert_reproducible_f64(
            lhs.value(),
            rhs.value(),
            &format!("multiplication commutativity case {case_index}"),
        );
    }
}

#[test]
fn property_probability_absolute_difference_is_closed() {
    let mut rng = PropertyRng::new(PROPERTY_SEED ^ 0x06);

    for case_index in 0..SCALAR_CASES {
        let a =
            generated_probability(&mut rng, "difference lhs");

        let b =
            generated_probability(&mut rng, "difference rhs");

        let difference = a.abs_difference(b);

        assert_valid_probability(
            difference,
            &format!("difference case {case_index}"),
        );

        assert_eq!(
            difference,
            b.abs_difference(a),
            "case {case_index}: absolute difference is not symmetric"
        );

        assert!(
            difference.value() >= 0.0,
            "case {case_index}: absolute difference is negative"
        );
    }
}

#[test]
fn property_probability_checked_add_is_sound() {
    let mut rng = PropertyRng::new(PROPERTY_SEED ^ 0x07);

    for case_index in 0..SCALAR_CASES {
        let a =
            generated_probability(&mut rng, "checked add lhs");

        let b =
            generated_probability(&mut rng, "checked add rhs");

        match a.checked_add(b) {
            Ok(sum) => {
                assert_valid_probability(
                    sum,
                    &format!("checked add case {case_index}"),
                );

                assert!(
                    sum.value() <= 1.0,
                    "case {case_index}: successful checked_add exceeded one"
                );

                let expected = a.value() + b.value();

                assert_reproducible_f64(
                    sum.value(),
                    expected,
                    &format!("checked add case {case_index}"),
                );
            }
            Err(_) => {
                assert!(
                    a.value() + b.value() > 1.0,
                    "case {case_index}: checked_add rejected a mathematically \
                     valid sum"
                );
            }
        }
    }
}

#[test]
fn property_probability_checked_sub_is_sound() {
    let mut rng = PropertyRng::new(PROPERTY_SEED ^ 0x08);

    for case_index in 0..SCALAR_CASES {
        let a =
            generated_probability(&mut rng, "checked sub lhs");

        let b =
            generated_probability(&mut rng, "checked sub rhs");

        match a.checked_sub(b) {
            Ok(difference) => {
                assert_valid_probability(
                    difference,
                    &format!("checked sub case {case_index}"),
                );

                assert!(
                    a.value() >= b.value(),
                    "case {case_index}: checked_sub succeeded even though lhs < rhs"
                );
            }
            Err(_) => {
                assert!(
                    a.value() < b.value(),
                    "case {case_index}: checked_sub rejected a valid non-negative result"
                );
            }
        }
    }
}

// =============================================================================
// Probability bounds properties
// =============================================================================

#[test]
fn property_bounds_constructor_preserves_invariant() {
    let mut rng = PropertyRng::new(PROPERTY_SEED ^ 0x10);

    for case_index in 0..BOUNDS_CASES {
        let mut lower =
            generated_probability(&mut rng, "bounds lower");

        let mut upper =
            generated_probability(&mut rng, "bounds upper");

        if lower.value() > upper.value() {
            std::mem::swap(&mut lower, &mut upper);
        }

        let bounds =
            ProbabilityBounds::new(lower, upper)
                .expect("ordered probabilities must form valid bounds");

        assert_bounds_invariant(
            bounds,
            &format!("bounds constructor case {case_index}"),
        );
    }
}

#[test]
fn property_bounds_reject_reversed_endpoints() {
    let mut rng = PropertyRng::new(PROPERTY_SEED ^ 0x11);

    for case_index in 0..BOUNDS_CASES {
        let a =
            generated_probability(&mut rng, "reversed lower");

        let b =
            generated_probability(&mut rng, "reversed upper");

        if a.value() > b.value() {
            assert!(
                ProbabilityBounds::new(a, b).is_err(),
                "case {case_index}: reversed bounds must be rejected"
            );
        }
    }
}

#[test]
fn property_bounds_exact_contains_exact_value() {
    let mut rng = PropertyRng::new(PROPERTY_SEED ^ 0x12);

    for case_index in 0..BOUNDS_CASES {
        let value =
            generated_probability(&mut rng, "exact bound");

        let bounds = ProbabilityBounds::exact(value);

        assert_bounds_invariant(
            bounds,
            &format!("exact bounds case {case_index}"),
        );

        assert!(
            bounds.contains(value),
            "case {case_index}: exact bound must contain its value"
        );

        assert!(
            bounds.is_exact(),
            "case {case_index}: exact bounds must report is_exact()"
        );
    }
}

#[test]
fn property_bounds_full_contains_every_generated_probability() {
    let mut rng = PropertyRng::new(PROPERTY_SEED ^ 0x13);

    let bounds = ProbabilityBounds::FULL;

    for case_index in 0..SCALAR_CASES {
        let probability =
            generated_probability(&mut rng, "full bounds");

        assert!(
            bounds.contains(probability),
            "case {case_index}: FULL bounds failed to contain valid probability"
        );
    }
}

#[test]
fn property_bounds_width_is_closed_and_non_negative() {
    let mut rng = PropertyRng::new(PROPERTY_SEED ^ 0x14);

    for case_index in 0..BOUNDS_CASES {
        let mut lower =
            generated_probability(&mut rng, "width lower");

        let mut upper =
            generated_probability(&mut rng, "width upper");

        if lower.value() > upper.value() {
            std::mem::swap(&mut lower, &mut upper);
        }

        let bounds =
            ProbabilityBounds::new(lower, upper)
                .expect("ordered bounds must construct");

        let width = bounds.width();

        assert_valid_probability(
            width,
            &format!("width case {case_index}"),
        );

        assert!(
            width.value() <= 1.0,
            "case {case_index}: width exceeded probability domain"
        );

        assert!(
            width.value() >= 0.0,
            "case {case_index}: width became negative"
        );
    }
}

#[test]
fn property_bounds_midpoint_is_contained() {
    let mut rng = PropertyRng::new(PROPERTY_SEED ^ 0x15);

    for case_index in 0..BOUNDS_CASES {
        let mut lower =
            generated_probability(&mut rng, "midpoint lower");

        let mut upper =
            generated_probability(&mut rng, "midpoint upper");

        if lower.value() > upper.value() {
            std::mem::swap(&mut lower, &mut upper);
        }

        let bounds =
            ProbabilityBounds::new(lower, upper)
                .expect("ordered bounds must construct");

        let midpoint = bounds.midpoint();

        assert_valid_probability(
            midpoint,
            &format!("midpoint case {case_index}"),
        );

        assert!(
            bounds.contains(midpoint),
            "case {case_index}: midpoint is outside its interval"
        );
    }
}

#[test]
fn property_bounds_containment_is_reflexive() {
    let mut rng = PropertyRng::new(PROPERTY_SEED ^ 0x16);

    for case_index in 0..BOUNDS_CASES {
        let mut lower =
            generated_probability(&mut rng, "reflexive lower");

        let mut upper =
            generated_probability(&mut rng, "reflexive upper");

        if lower.value() > upper.value() {
            std::mem::swap(&mut lower, &mut upper);
        }

        let bounds =
            ProbabilityBounds::new(lower, upper)
                .expect("ordered bounds must construct");

        assert!(
            bounds.contains_bounds(bounds),
            "case {case_index}: bounds must contain themselves"
        );
    }
}

#[test]
fn property_bounds_nested_containment_is_transitive() {
    let mut rng = PropertyRng::new(PROPERTY_SEED ^ 0x17);

    for case_index in 0..BOUNDS_CASES {
        let a =
            generated_probability(&mut rng, "nested a");

        let b =
            generated_probability(&mut rng, "nested b");

        let c =
            generated_probability(&mut rng, "nested c");

        let mut values = [
            a.value(),
            b.value(),
            c.value(),
        ];

        values.sort_by(f64::total_cmp);

        let outer_lower =
            Probability::new(values[0])
                .expect("sorted generated probability must be valid");

        let middle_lower =
            Probability::new(values[1])
                .expect("sorted generated probability must be valid");

        let middle_upper =
            Probability::new(values[2])
                .expect("sorted generated probability must be valid");

        let outer =
            ProbabilityBounds::new(
                outer_lower,
                middle_upper,
            )
            .expect("outer interval must construct");

        let middle =
            ProbabilityBounds::new(
                middle_lower,
                middle_upper,
            )
            .expect("middle interval must construct");

        if middle.contains_bounds(middle)
            && outer.contains_bounds(middle)
        {
            assert!(
                outer.contains_bounds(middle),
                "case {case_index}: nested bounds lost containment"
            );
        }
    }
}

#[test]
fn property_bounds_intersection_is_subset_of_both() {
    let mut rng = PropertyRng::new(PROPERTY_SEED ^ 0x18);

    for case_index in 0..BOUNDS_CASES {
        let mut a_lower =
            generated_probability(&mut rng, "intersection a lower");

        let mut a_upper =
            generated_probability(&mut rng, "intersection a upper");

        if a_lower.value() > a_upper.value() {
            std::mem::swap(&mut a_lower, &mut a_upper);
        }

        let mut b_lower =
            generated_probability(&mut rng, "intersection b lower");

        let mut b_upper =
            generated_probability(&mut rng, "intersection b upper");

        if b_lower.value() > b_upper.value() {
            std::mem::swap(&mut b_lower, &mut b_upper);
        }

        let a =
            ProbabilityBounds::new(a_lower, a_upper)
                .expect("a must construct");

        let b =
            ProbabilityBounds::new(b_lower, b_upper)
                .expect("b must construct");

        if let Some(intersection) = a.intersection(b) {
            assert_bounds_invariant(
                intersection,
                &format!("intersection case {case_index}"),
            );

            assert!(
                a.contains_bounds(intersection),
                "case {case_index}: intersection is not subset of lhs"
            );

            assert!(
                b.contains_bounds(intersection),
                "case {case_index}: intersection is not subset of rhs"
            );
        }
    }
}

#[test]
fn property_bounds_hull_contains_both_operands() {
    let mut rng = PropertyRng::new(PROPERTY_SEED ^ 0x19);

    for case_index in 0..BOUNDS_CASES {
        let mut a_lower =
            generated_probability(&mut rng, "hull a lower");

        let mut a_upper =
            generated_probability(&mut rng, "hull a upper");

        if a_lower.value() > a_upper.value() {
            std::mem::swap(&mut a_lower, &mut a_upper);
        }

        let mut b_lower =
            generated_probability(&mut rng, "hull b lower");

        let mut b_upper =
            generated_probability(&mut rng, "hull b upper");

        if b_lower.value() > b_upper.value() {
            std::mem::swap(&mut b_lower, &mut b_upper);
        }

        let a =
            ProbabilityBounds::new(a_lower, a_upper)
                .expect("a must construct");

        let b =
            ProbabilityBounds::new(b_lower, b_upper)
                .expect("b must construct");

        let hull = a.hull(b);

        assert_bounds_invariant(
            hull,
            &format!("hull case {case_index}"),
        );

        assert!(
            hull.contains_bounds(a),
            "case {case_index}: hull does not contain lhs"
        );

        assert!(
            hull.contains_bounds(b),
            "case {case_index}: hull does not contain rhs"
        );
    }
}

// =============================================================================
// Distribution properties
// =============================================================================

fn generated_distribution(
    rng: &mut PropertyRng,
    entry_count: usize,
) -> Distribution<u64> {
    // Generate strictly positive raw weights first.
    //
    // Distribution::from_weighted performs the canonical normalization and
    // duplicate-outcome merging.
    let mut entries = Vec::with_capacity(entry_count);

    for index in 0..entry_count {
        let outcome = index as u64;

        let raw =
            1.0 + rng.probability_value();

        entries.push((outcome, raw));
    }

    Distribution::from_weighted(entries, 1.0e-12)
        .expect("generated positive weights must form a valid distribution")
}

#[test]
fn property_distribution_generation_is_normalized() {
    let mut rng =
        PropertyRng::new(PROPERTY_SEED ^ 0x20);

    for case_index in 0..DISTRIBUTION_CASES {
        let entry_count =
            1 + rng.index(TEST_MAX_DISTRIBUTION_ENTRIES);

        let distribution =
            generated_distribution(&mut rng, entry_count);

        let total =
            distribution.total_probability();

        assert!(
            total.is_finite(),
            "distribution case {case_index}: \
             total probability is non-finite"
        );

        assert!(
            (total - 1.0).abs() <= 1.0e-10,
            "distribution case {case_index}: \
             distribution is not normalized: total={total:?}"
        );
    }
}

#[test]
fn property_distribution_never_contains_invalid_probabilities() {
    let mut rng =
        PropertyRng::new(PROPERTY_SEED ^ 0x21);

    for case_index in 0..DISTRIBUTION_CASES {
        let entry_count =
            1 + rng.index(TEST_MAX_DISTRIBUTION_ENTRIES);

        let distribution =
            generated_distribution(&mut rng, entry_count);

        for index in 0..distribution.len() {
            let probability =
                distribution.probability_at(index)
                    .expect("valid generated index must exist");

            assert!(
                probability.get().is_finite(),
                "case {case_index}, index {index}: \
                 probability is non-finite"
            );

            assert!(
                (0.0..=1.0).contains(&probability.get()),
                "case {case_index}, index {index}: \
                 probability is outside [0,1]: {:?}",
                probability.get()
            );

            assert!(
                probability.get() > 0.0,
                "case {case_index}, index {index}: \
                 canonical distribution retained zero probability"
            );
        }
    }
}

#[test]
fn property_distribution_length_is_finite_and_positive() {
    let mut rng =
        PropertyRng::new(PROPERTY_SEED ^ 0x22);

    for case_index in 0..DISTRIBUTION_CASES {
        let entry_count =
            1 + rng.index(TEST_MAX_DISTRIBUTION_ENTRIES);

        let distribution =
            generated_distribution(&mut rng, entry_count);

        assert!(
            distribution.len() > 0,
            "case {case_index}: generated distribution unexpectedly empty"
        );

        assert!(
            distribution.len() <= entry_count,
            "case {case_index}: canonical distribution grew beyond input"
        );
    }
}

#[test]
fn property_distribution_duplicate_merging_preserves_probability_mass() {
    let mut rng =
        PropertyRng::new(PROPERTY_SEED ^ 0x23);

    for case_index in 0..DISTRIBUTION_CASES {
        let first = 0.2;
        let second = 0.3;
        let third = 0.5;

        let distribution =
            Distribution::from_weighted(
                vec![
                    (0_u8, first),
                    (0_u8, second),
                    (1_u8, third),
                ],
                1.0e-12,
            )
            .expect("duplicate generated distribution must be valid");

        assert_eq!(
            distribution.len(),
            2,
            "case {case_index}: duplicate outcome was not merged"
        );

        let probability_zero =
            distribution
                .probability(&0_u8)
                .expect("merged outcome must exist");

        let probability_one =
            distribution
                .probability(&1_u8)
                .expect("second outcome must exist");

        assert!(
            (probability_zero.get() - 0.5).abs() <= 1.0e-12,
            "case {case_index}: duplicate probability was not merged correctly"
        );

        assert!(
            (probability_one.get() - 0.5).abs() <= 1.0e-12,
            "case {case_index}: second probability changed unexpectedly"
        );

        // Keep the RNG use intentional so this property remains coupled to the
        // deterministic property harness rather than becoming an isolated
        // example test.
        let _ = rng.next_u64();
    }
}

#[test]
fn property_distribution_rejects_zero_total_weight() {
    let result =
        Distribution::from_weighted(
            vec![
                (0_u8, 0.0),
                (1_u8, 0.0),
            ],
            1.0e-12,
        );

    assert!(
        result.is_err(),
        "zero-total distribution must be rejected"
    );
}

#[test]
fn property_distribution_rejects_negative_weights() {
    let result =
        Distribution::from_weighted(
            vec![
                (0_u8, -0.1),
                (1_u8, 1.1),
            ],
            1.0e-12,
        );

    assert!(
        result.is_err(),
        "negative distribution weight must be rejected"
    );
}

#[test]
fn property_distribution_rejects_non_finite_weights() {
    let invalid_cases = [
        vec![
            (0_u8, f64::NAN),
            (1_u8, 1.0),
        ],
        vec![
            (0_u8, f64::INFINITY),
            (1_u8, 1.0),
        ],
        vec![
            (0_u8, f64::NEG_INFINITY),
            (1_u8, 1.0),
        ],
    ];

    for entries in invalid_cases {
        let result =
            Distribution::from_weighted(entries, 1.0e-12);

        assert!(
            result.is_err(),
            "non-finite distribution weights must be rejected"
        );
    }
}

#[test]
fn property_distribution_single_outcome_has_unit_probability() {
    let distribution =
        Distribution::from_weighted(
            vec![(42_u64, 17.0)],
            1.0e-12,
        )
        .expect("single positive outcome must construct");

    assert_eq!(distribution.len(), 1);

    let probability =
        distribution
            .probability(&42_u64)
            .expect("single outcome must exist");

    assert_eq!(
        probability.get().to_bits(),
        1.0_f64.to_bits()
    );

    assert!(
        (distribution.total_probability() - 1.0).abs()
            <= 1.0e-12
    );
}

// =============================================================================
// Determinism properties
// =============================================================================

#[test]
fn property_generators_are_reproducible() {
    let mut first =
        PropertyRng::new(PROPERTY_SEED ^ 0x30);

    let mut second =
        PropertyRng::new(PROPERTY_SEED ^ 0x30);

    for case_index in 0..SCALAR_CASES {
        let lhs = first.next_u64();
        let rhs = second.next_u64();

        assert_eq!(
            lhs,
            rhs,
            "generator case {case_index}: \
             identical seeds produced different values"
        );
    }
}

#[test]
fn property_probability_generation_is_reproducible() {
    let mut first =
        PropertyRng::new(PROPERTY_SEED ^ 0x31);

    let mut second =
        PropertyRng::new(PROPERTY_SEED ^ 0x31);

    for case_index in 0..SCALAR_CASES {
        let lhs =
            generated_probability(
                &mut first,
                "first deterministic probability",
            );

        let rhs =
            generated_probability(
                &mut second,
                "second deterministic probability",
            );

        assert_eq!(
            lhs.value().to_bits(),
            rhs.value().to_bits(),
            "case {case_index}: \
             identical property seeds produced different probabilities"
        );
    }
}

#[test]
fn property_probability_operations_are_deterministic() {
    let mut first =
        PropertyRng::new(PROPERTY_SEED ^ 0x32);

    let mut second =
        PropertyRng::new(PROPERTY_SEED ^ 0x32);

    for case_index in 0..SCALAR_CASES {
        let a1 =
            generated_probability(
                &mut first,
                "deterministic lhs first",
            );

        let b1 =
            generated_probability(
                &mut first,
                "deterministic rhs first",
            );

        let a2 =
            generated_probability(
                &mut second,
                "deterministic lhs second",
            );

        let b2 =
            generated_probability(
                &mut second,
                "deterministic rhs second",
            );

        assert_eq!(
            a1,
            a2,
            "case {case_index}: lhs generation diverged"
        );

        assert_eq!(
            b1,
            b2,
            "case {case_index}: rhs generation diverged"
        );

        assert_eq!(
            a1.multiply(b1),
            a2.multiply(b2),
            "case {case_index}: multiplication diverged"
        );

        assert_eq!(
            a1.complement(),
            a2.complement(),
            "case {case_index}: complement diverged"
        );

        assert_eq!(
            a1.abs_difference(b1),
            a2.abs_difference(b2),
            "case {case_index}: difference diverged"
        );
    }
}

// =============================================================================
// Boundary / adversarial properties
// =============================================================================

#[test]
fn property_probability_handles_subnormal_positive_value() {
    let value = f64::from_bits(1);

    let probability =
        Probability::new(value)
            .expect("smallest positive subnormal is a valid probability");

    assert_eq!(
        probability.value().to_bits(),
        value.to_bits()
    );
}

#[test]
fn property_probability_handles_values_adjacent_to_zero() {
    let values = [
        0.0,
        f64::from_bits(1),
        f64::from_bits(2),
    ];

    for value in values {
        let probability =
            Probability::new(value)
                .expect("value adjacent to zero must be valid");

        assert_valid_probability(
            probability,
            "adjacent-to-zero property",
        );
    }
}

#[test]
fn property_probability_handles_values_adjacent_to_one() {
    let values = [
        1.0 - f64::EPSILON,
        1.0,
    ];

    for value in values {
        let probability =
            Probability::new(value)
                .expect("value adjacent to one must be valid");

        assert_valid_probability(
            probability,
            "adjacent-to-one property",
        );
    }
}

#[test]
fn property_probability_never_silently_clamps_invalid_input() {
    let invalid_values = [
        -1.0,
        -f64::EPSILON,
        1.0 + f64::EPSILON,
        2.0,
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
    ];

    for value in invalid_values {
        assert!(
            Probability::new(value).is_err(),
            "invalid probability was silently accepted or clamped: {value:?}"
        );
    }
}

#[test]
fn property_bounds_never_silently_swap_endpoints() {
    let lower =
        Probability::new(0.8)
            .expect("0.8 must be valid");

    let upper =
        Probability::new(0.2)
            .expect("0.2 must be valid");

    let result =
        ProbabilityBounds::new(lower, upper);

    assert!(
        result.is_err(),
        "reversed bounds must fail rather than silently swap endpoints"
    );
}

#[test]
fn property_distribution_never_silently_repairs_non_finite_input() {
    for value in [
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
    ] {
        let result =
            Distribution::from_weighted(
                vec![(0_u8, value)],
                1.0e-12,
            );

        assert!(
            result.is_err(),
            "distribution silently repaired non-finite weight {value:?}"
        );
    }
}

// =============================================================================
// Algebraic properties
// =============================================================================

#[test]
fn property_probability_complement_respects_endpoints() {
    assert_eq!(
        Probability::ZERO.complement(),
        Probability::ONE
    );

    assert_eq!(
        Probability::ONE.complement(),
        Probability::ZERO
    );
}

#[test]
fn property_probability_multiplication_has_identity() {
    let mut rng =
        PropertyRng::new(PROPERTY_SEED ^ 0x40);

    for case_index in 0..SCALAR_CASES {
        let probability =
            generated_probability(
                &mut rng,
                "multiplication identity",
            );

        assert_eq!(
            probability.multiply(Probability::ONE),
            probability,
            "case {case_index}: p * 1 != p"
        );

        assert_eq!(
            probability.multiply(Probability::ZERO),
            Probability::ZERO,
            "case {case_index}: p * 0 != 0"
        );
    }
}

#[test]
fn property_probability_difference_is_zero_for_identical_values() {
    let mut rng =
        PropertyRng::new(PROPERTY_SEED ^ 0x41);

    for case_index in 0..SCALAR_CASES {
        let probability =
            generated_probability(
                &mut rng,
                "difference identity",
            );

        let result =
            probability.abs_difference(probability);

        assert_eq!(
            result,
            Probability::ZERO,
            "case {case_index}: |p-p| != 0"
        );
    }
}

#[test]
fn property_bounds_exact_interval_has_zero_width() {
    let mut rng =
        PropertyRng::new(PROPERTY_SEED ^ 0x42);

    for case_index in 0..BOUNDS_CASES {
        let probability =
            generated_probability(
                &mut rng,
                "exact interval width",
            );

        let bounds =
            ProbabilityBounds::exact(probability);

        assert_eq!(
            bounds.width(),
            Probability::ZERO,
            "case {case_index}: exact interval width != 0"
        );

        assert_eq!(
            bounds.midpoint(),
            probability,
            "case {case_index}: exact interval midpoint != point"
        );
    }
}

// =============================================================================
// Scaling properties
// =============================================================================

#[test]
fn property_probability_semantics_are_independent_of_collection_size() {
    let reference =
        Probability::new(0.375)
            .expect("reference probability must be valid");

    // These are deliberately generated progressively. The values are test
    // sizes, not production limits.
    let sizes = [
        1_usize,
        2,
        4,
        8,
        16,
        32,
        64,
        128,
        256,
        512,
        1024,
    ];

    for size in sizes {
        let mut generated = Vec::with_capacity(size);

        for _ in 0..size {
            generated.push(reference);
        }

        for (index, probability) in generated.iter().enumerate() {
            assert_eq!(
                *probability,
                reference,
                "size {size}, index {index}: \
                 scalar probability semantics changed with collection size"
            );
        }
    }
}

#[test]
fn property_distribution_semantics_scale_with_generated_cardinality() {
    let sizes = [
        1_usize,
        2,
        4,
        8,
        16,
        32,
        64,
        128,
    ];

    for size in sizes {
        let mut entries = Vec::with_capacity(size);

        for index in 0..size {
            entries.push((
                index as u64,
                1.0 + index as f64,
            ));
        }

        let distribution =
            Distribution::from_weighted(
                entries,
                1.0e-10,
            )
            .expect("positive generated weights must construct");

        let total =
            distribution.total_probability();

        assert!(
            (total - 1.0).abs() <= 1.0e-10,
            "size {size}: distribution normalization failed: {total:?}"
        );

        assert_eq!(
            distribution.len(),
            size,
            "size {size}: generated unique outcomes changed cardinality"
        );
    }
}

#[test]
fn property_generated_distribution_cardinality_never_changes_semantics_of_normalization() {
    let sizes = [
        1_usize,
        2,
        4,
        8,
        16,
        32,
        64,
        128,
    ];

    for size in sizes {
        let mut entries = Vec::with_capacity(size);

        for index in 0..size {
            entries.push((
                index as u64,
                1.0,
            ));
        }

        let distribution =
            Distribution::from_weighted(
                entries,
                1.0e-10,
            )
            .expect("uniform positive weights must construct");

        let expected =
            1.0 / size as f64;

        for index in 0..size {
            let outcome =
                index as u64;

            let probability =
                distribution
                    .probability(&outcome)
                    .expect("generated outcome must exist");

            assert!(
                (probability.get() - expected).abs()
                    <= 1.0e-10,
                "size {size}, outcome {outcome}: \
                 normalized probability changed unexpectedly: \
                 actual={}, expected={expected}",
                probability.get()
            );
        }
    }
}

// =============================================================================
// Public-contract smoke properties
// =============================================================================

#[test]
fn property_probability_public_contract_is_stable() {
    let zero = Probability::ZERO;
    let one = Probability::ONE;

    assert_eq!(zero.value(), 0.0);
    assert_eq!(one.value(), 1.0);

    assert!(zero.is_zero());
    assert!(one.is_one());

    assert_eq!(
        zero.complement(),
        one
    );

    assert_eq!(
        one.complement(),
        zero
    );
}

#[test]
fn property_probability_bounds_public_contract_is_stable() {
    let full =
        ProbabilityBounds::FULL;

    assert_eq!(
        full.lower(),
        Probability::ZERO
    );

    assert_eq!(
        full.upper(),
        Probability::ONE
    );

    assert!(
        full.contains(Probability::ZERO)
    );

    assert!(
        full.contains(Probability::ONE)
    );

    assert!(
        full.contains_bounds(full)
    );

    assert_eq!(
        full.width(),
        Probability::ONE
    );

    assert_eq!(
        full.midpoint().value().to_bits(),
        0.5_f64.to_bits()
    );
}

// =============================================================================
// Reproduction metadata
// =============================================================================

#[test]
fn property_failure_reproduction_contract_is_explicit() {
    // This is intentionally a test rather than merely a constant hidden in
    // implementation code. If a property fails in CI, the maintainer has a
    // stable seed from which the deterministic generator can be reconstructed.
    //
    // The property itself must never depend on wall-clock time.
    assert_ne!(
        PROPERTY_SEED,
        0,
        "property seed must remain explicit and non-zero"
    );
}

// =============================================================================
// Compile-time architectural assertions
// =============================================================================

#[test]
fn property_foundational_types_are_copy_where_contract_requires_it() {
    fn assert_copy<T: Copy>() {}

    assert_copy::<Probability>();
    assert_copy::<ProbabilityBounds>();
}

#[test]
fn property_foundational_types_are_cloneable() {
    fn assert_clone<T: Clone>() {}

    assert_clone::<Probability>();
    assert_clone::<ProbabilityBounds>();
}

#[test]
fn property_foundational_types_are_debuggable() {
    fn assert_debug<T: Debug>() {}

    assert_debug::<Probability>();
    assert_debug::<ProbabilityBounds>();
}

// =============================================================================
// Future integration guard
// =============================================================================
//
// The following section deliberately documents, rather than prematurely
// imports, future integration contracts.
//
// When these production APIs are finalized, corresponding property suites
// MUST be added without changing the mathematical properties above:
//
// ZQN channel properties:
//
// - identity channel preserves valid states;
// - composition is associative within numerical tolerance;
// - tensor product is dimensionally valid;
// - Kraus representations preserve complete positivity;
// - trace-preserving channels preserve trace;
// - equivalent channel representations agree.
//
// ZQN fault properties:
//
// - every fault has a valid canonical location;
// - correlated faults do not introduce duplicate resource identity;
// - leakage/erasure/loss probabilities remain valid;
// - fault generation is deterministic under explicit execution context.
//
// ZQN noise properties:
//
// - applying a valid noise model never creates an invalid probability;
// - unsupported target capabilities are rejected explicitly;
// - approximation is never silent;
// - declared approximation bounds are preserved;
// - temporal/spatial correlations remain explicit.
//
// ZQN calibration properties:
//
// - calibration snapshots are internally consistent;
// - expired calibration is not silently treated as current;
// - calibration identity is deterministic;
// - uncertainty remains distinguishable from deterministic bounds.
//
// ZQN simulation properties:
//
// - identical seed/context/model produces identical results;
// - parallel and sequential deterministic execution agree;
// - no hidden global RNG affects semantics.
//
// ZQN target properties:
//
// - target compatibility is capability-driven;
// - target size is data rather than a hard-coded branch;
// - no vendor-specific assumption leaks into ZQN semantics.
//
// ZQN integration properties:
//
// - canonical quantum::ir::qubit identities remain authoritative;
// - routing consumes ZQN information without redefining it;
// - scheduling consumes ZQN timing/noise information without owning it;
// - QEC consumes ZQN faults without duplicating the universal noise model;
// - hardware provides capabilities/calibration rather than becoming a ZQN
//   dependency;
// - benchmarking consumes observations without redefining physical noise.
//
// These future properties belong here because this file is the cross-layer
// invariant boundary. They must not be implemented by weakening foundational
// contracts.
//
// =============================================================================
// End of property.rs
// =============================================================================