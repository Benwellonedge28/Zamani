//! Zamani Quantum Noise (ZQN) — Categorical Probability Distribution
//!
//! This module provides the foundational finite categorical probability
//! distribution used by ZQN.
//!
//! # Architectural responsibility
//!
//! This file owns the mathematical representation and validation of a
//! categorical distribution:
//!
//! ```text
//! outcome -> probability
//! ```
//!
//! where outcomes are arbitrary caller-owned values and probabilities are
//! finite, non-negative weights whose total is exactly one within the
//! explicitly selected validation policy.
//!
//! This file is intentionally independent of:
//!
//! - quantum circuits;
//! - quantum gates;
//! - qubits;
//! - hardware;
//! - noise models;
//! - channels;
//! - QEC;
//! - simulation engines;
//! - random-number generators;
//! - calibration;
//! - routing;
//! - scheduling;
//! - serialization frameworks.
//!
//! Those systems may consume this type.
//!
//! # Why this is not qubit-specific
//!
//! A categorical distribution is a mathematical primitive. Its outcomes can
//! represent:
//!
//! - measurement outcomes;
//! - Pauli faults;
//! - leakage states;
//! - erasure states;
//! - calibration categories;
//! - discrete noise regimes;
//! - arbitrary user-defined events;
//! - future quantum-resource outcome domains.
//!
//! Consequently this module MUST NOT depend on
//! `crate::quantum::ir::qubit`.
//!
//! When a later ZQN module associates a distribution with a quantum resource,
//! that later module should use the canonical types from:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! ZQN must not define replacement qubit identifiers. The repository's IR
//! explicitly establishes `quantum::ir::qubit` as the canonical identity
//! boundary. This preserves the separation between mathematical probability
//! and quantum-resource identity.
//!
//! # Write once, scale everywhere
//!
//! There is deliberately no semantic maximum number of categories.
//!
//! A categorical distribution may contain one category, many categories, or
//! as many categories as the selected storage and execution resources can
//! support.
//!
//! This module therefore contains no:
//!
//! ```text
//! MAX_CATEGORIES
//! MAX_OUTCOMES
//! MAX_QUBITS
//! MAX_FAULTS
//! ```
//!
//! statements.
//!
//! "Infinity" in Zamani means that the semantic model does not impose an
//! artificial finite machine-size ceiling. Actual allocation, memory,
//! serialization, execution and runtime limits remain external resource
//! policies.
//!
//! # Mathematical contract
//!
//! For a categorical distribution with outcomes `x_i` and probabilities
//! `p_i`:
//!
//! ```text
//! p_i >= 0
//!
//! sum(p_i) = 1
//! ```
//!
//! within the explicitly selected numerical validation policy.
//!
//! Probabilities must be finite.
//!
//! NaN and positive/negative infinity are rejected.
//!
//! Negative probabilities are rejected.
//!
//! A zero-probability category is mathematically valid and is retained when
//! explicitly supplied. Removing zero-probability categories is a policy
//! decision and is therefore not performed implicitly by this module.
//!
//! # Numerical policy
//!
//! This type stores `f64` probabilities because it is intended to integrate
//! with the existing quantum subsystem, which currently exposes probability
//! APIs using `f64` in multiple memory, benchmarking and simulation paths.
//!
//! This does NOT make `f64` the universal numerical representation of ZQN.
//!
//! Future exact/rational/interval/arbitrary-precision representations belong
//! in the broader probability subsystem.
//!
//! In particular, this type does not claim that every probability in quantum
//! computing can be represented exactly by binary floating point.
//!
//! # Normalization policy
//!
//! The constructor validates rather than silently normalizing.
//!
//! This is intentional.
//!
//! Silently transforming:
//!
//! ```text
//! [0.2, 0.2]
//! ```
//!
//! into:
//!
//! ```text
//! [0.5, 0.5]
//! ```
//!
//! would hide an upstream semantic error.
//!
//! Callers that intentionally have unnormalized weights should explicitly
//! normalize them before constructing this distribution.
//!
//! # Determinism
//!
//! This module contains no random-number generator and no hidden global
//! state.
//!
//! Iteration order is the order established by the caller's input sequence.
//!
//! No hash map is used internally, so hash iteration order cannot affect
//! results.
//!
//! This makes construction, lookup, equality and serialization-oriented
//! traversal deterministic for a fixed input sequence.
//!
//! Sampling is deliberately NOT implemented here. Sampling requires an
//! explicit reproducibility context and RNG policy and therefore belongs in
//! the ZQN sampling/simulation layer.
//!
//! # Resource safety
//!
//! The implementation uses caller-owned storage and does not impose an
//! arbitrary semantic size limit.
//!
//! The constructor may allocate according to the supplied input.
//!
//! Runtime/resource-policy layers are responsible for limiting allocations
//! when inputs are untrusted.
//!
//! This module does not catch allocation failure because Rust allocation
//! failure is a process-level resource event rather than a categorical
//! probability semantic error.
//!
//! # Complexity
//!
//! Given `n` categories:
//!
//! - construction: `O(n)`;
//! - validation: `O(n)`;
//! - indexed access: `O(1)`;
//! - linear outcome lookup: `O(n)`;
//! - expectation of a caller-provided function: `O(n)`;
//! - iteration: `O(n)`.
//!
//! The representation deliberately avoids hidden indexes or hash tables so
//! that deterministic ordering is intrinsic.
//!
//! If a consumer needs high-volume lookup, it can maintain an external index
//! appropriate to its workload.
//!
//! # Empty distributions
//!
//! An empty categorical distribution is not a probability distribution because
//! it cannot satisfy:
//!
//! ```text
//! sum(p_i) = 1
//! ```
//!
//! Therefore `new([])` returns an error.
//!
//! # Duplicate outcomes
//!
//! Duplicate outcomes are rejected.
//!
//! A categorical distribution represents a function from outcome identity to
//! probability. Accepting duplicate outcome entries would create ambiguity
//! about whether the values should be summed, overwritten, or treated as
//! distinct events.
//!
//! Such policy belongs outside this foundational type.
//!
//! Callers that have weighted duplicate observations must aggregate them first.
//!
//! # Equality
//!
//! Equality is structural:
//!
//! - same outcome sequence;
//! - same probabilities.
//!
//! Reordering categories changes equality even when the mathematical
//! distribution is equivalent.
//!
//! This is intentional because deterministic canonicalization is a separate
//! concern from mathematical equivalence.
//!
//! # Serialization
//!
//! No serialization framework is required here.
//!
//! A later schema layer may encode:
//!
//! ```text
//! [
//!   { outcome: ..., probability: ... },
//!   ...
//! ]
//! ```
//!
//! The schema layer must preserve category ordering or explicitly canonicalize
//! it before hashing/equality.
//!
//! This module does not depend on serde merely to remain a foundational,
//! dependency-light component.
//!
//! # Security
//!
//! The following inputs are rejected:
//!
//! - NaN;
//! - positive infinity;
//! - negative infinity;
//! - negative probability;
//! - invalid normalization;
//! - duplicate outcomes;
//! - empty distributions.
//!
//! This prevents common numerical-invalid-state propagation.
//!
//! Resource exhaustion caused by enormous untrusted input is handled by the
//! caller's resource policy rather than by a fixed category ceiling here.
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
//! - no unsafe code.
//!
//! # Integration contract
//!
//! This file is intentionally one of the earliest ZQN files that can be
//! completed independently.
//!
//! Dependency direction:
//!
//! ```text
//! categorical.rs
//!     │
//!     ├── channel
//!     ├── fault
//!     ├── noise
//!     ├── calibration
//!     ├── characterization
//!     ├── simulation
//!     └── propagation
//! ```
//!
//! None of those downstream modules should be required to compile this file.
//!
//! # Future integration
//!
//! A later `probability::distribution` module may define a common distribution
//! trait. If that occurs, this type should implement that trait without
//! changing its mathematical invariants.
//!
//! A later sampling module may consume this type through iteration over
//! `(outcome, probability)` pairs.
//!
//! A later serialization module may use the public iterator and accessors.
//!
//! A later noise model may attach this distribution to a
//! `PhysicalQubitId`, but that association belongs outside this file.
//!
//! # Testing contract
//!
//! Tests must cover:
//!
//! 1. valid distributions;
//! 2. empty input;
//! 3. negative probabilities;
//! 4. NaN;
//! 5. infinity;
//! 6. under-normalized distributions;
//! 7. over-normalized distributions;
//! 8. duplicate outcomes;
//! 9. zero-probability categories;
//! 10. singleton distributions;
//! 11. large generated distributions;
//! 12. deterministic iteration;
//! 13. equality semantics;
//! 14. expectation;
//! 15. total probability;
//! 16. no hidden mutable state.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]

use std::fmt;
use std::ops::Index;

/// Default absolute normalization tolerance.
///
/// This value is deliberately a validation tolerance, not a semantic
/// probability floor or a machine-size limit.
///
/// It is exposed as a constant so callers and tests can use the exact same
/// default policy without duplicating a magic number.
///
/// The tolerance is appropriate for ordinary `f64` accumulation of finite
/// probability vectors, while callers requiring stricter numerical contracts
/// can use [`Categorical::with_tolerance`].
pub const DEFAULT_NORMALIZATION_TOLERANCE: f64 = 1.0e-12;

/// A single outcome/probability pair.
///
/// The outcome type is generic because ZQN probability semantics must not be
/// restricted to strings, integers, qubits, Pauli operators, or any other
/// particular domain.
///
/// # Examples
///
/// ```
/// use zamani::quantum::zqn::probability::categorical::CategoricalEntry;
///
/// let entry = CategoricalEntry::new("success", 1.0);
/// assert_eq!(entry.outcome(), &"success");
/// assert_eq!(entry.probability(), 1.0);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct CategoricalEntry<T> {
    outcome: T,
    probability: f64,
}

impl<T> CategoricalEntry<T> {
    /// Creates an entry.
    ///
    /// This constructor does not validate the probability because an entry
    /// can be assembled before being inserted into a distribution.
    ///
    /// [`Categorical::new`] and [`Categorical::with_tolerance`] perform the
    /// complete distribution-level validation.
    #[must_use]
    pub const fn new(outcome: T, probability: f64) -> Self {
        Self {
            outcome,
            probability,
        }
    }

    /// Returns the outcome.
    #[must_use]
    pub const fn outcome(&self) -> &T {
        &self.outcome
    }

    /// Returns the probability.
    #[must_use]
    pub const fn probability(&self) -> f64 {
        self.probability
    }

    /// Consumes the entry and returns its components.
    #[must_use]
    pub fn into_parts(self) -> (T, f64) {
        (self.outcome, self.probability)
    }
}

/// Errors produced while constructing or validating a categorical
/// probability distribution.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CategoricalError {
    /// No categories were supplied.
    Empty,

    /// A supplied probability was NaN or infinite.
    NonFiniteProbability {
        /// Zero-based category position.
        index: usize,
        /// Invalid value.
        value: f64,
    },

    /// A supplied probability was negative.
    NegativeProbability {
        /// Zero-based category position.
        index: usize,
        /// Invalid value.
        value: f64,
    },

    /// The same outcome occurred more than once.
    ///
    /// The actual outcome is deliberately not embedded in this error because
    /// `T` is not required to implement `Debug`, `Display`, `Clone`, or any
    /// other formatting trait.
    DuplicateOutcome {
        /// First occurrence.
        first_index: usize,
        /// Later occurrence.
        duplicate_index: usize,
    },

    /// The probabilities do not sum to one within the selected tolerance.
    NotNormalized {
        /// Calculated sum.
        sum: f64,
        /// Absolute difference from one.
        difference: f64,
        /// Accepted tolerance.
        tolerance: f64,
    },

    /// The supplied normalization tolerance is invalid.
    InvalidTolerance {
        /// Supplied tolerance.
        tolerance: f64,
    },
}

impl fmt::Display for CategoricalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => {
                write!(formatter, "categorical distribution cannot be empty")
            }
            Self::NonFiniteProbability { index, value } => write!(
                formatter,
                "categorical probability at index {index} is not finite: {value:?}"
            ),
            Self::NegativeProbability { index, value } => write!(
                formatter,
                "categorical probability at index {index} is negative: {value:?}"
            ),
            Self::DuplicateOutcome {
                first_index,
                duplicate_index,
            } => write!(
                formatter,
                "categorical distribution contains a duplicate outcome: \
                 first index {first_index}, duplicate index {duplicate_index}"
            ),
            Self::NotNormalized {
                sum,
                difference,
                tolerance,
            } => write!(
                formatter,
                "categorical probabilities are not normalized: \
                 sum={sum:?}, difference={difference:?}, tolerance={tolerance:?}"
            ),
            Self::InvalidTolerance { tolerance } => write!(
                formatter,
                "categorical normalization tolerance must be finite and \
                 non-negative: {tolerance:?}"
            ),
        }
    }
}

impl std::error::Error for CategoricalError {}

/// A finite categorical probability distribution.
///
/// `Categorical<T>` stores an ordered collection of distinct outcomes and
/// their probabilities.
///
/// The type guarantees after successful construction:
///
/// ```text
/// outcomes.len() > 0
///
/// every probability is finite
///
/// every probability >= 0
///
/// outcomes are pairwise distinct
///
/// |sum(probabilities) - 1| <= tolerance
/// ```
///
/// The tolerance used by the instance is retained so that validation policy
/// remains part of the object's explicit construction contract.
///
/// # Generic requirements
///
/// `T` only needs `PartialEq` because duplicate detection is performed without
/// imposing hashing or ordering requirements on user outcomes.
///
/// This intentionally permits outcomes such as:
///
/// - quantum IR values;
/// - user-defined structs;
/// - enum states;
/// - tuples;
/// - strings;
/// - integers;
/// - future quantum resource descriptors.
///
/// The distribution itself does not require `T: Clone`, `T: Hash`, or
/// `T: Ord`.
#[derive(Debug, Clone, PartialEq)]
pub struct Categorical<T> {
    entries: Vec<CategoricalEntry<T>>,
    tolerance: f64,
}

impl<T> Categorical<T>
where
    T: PartialEq,
{
    /// Constructs a categorical distribution using the default normalization
    /// tolerance.
    ///
    /// The input is consumed.
    ///
    /// This constructor never silently normalizes probabilities.
    ///
    /// # Errors
    ///
    /// Returns [`CategoricalError`] if:
    ///
    /// - the input is empty;
    /// - a probability is non-finite;
    /// - a probability is negative;
    /// - an outcome is duplicated;
    /// - the probabilities do not sum to one within the default tolerance.
    pub fn new(entries: Vec<CategoricalEntry<T>>) -> Result<Self, CategoricalError> {
        Self::with_tolerance(entries, DEFAULT_NORMALIZATION_TOLERANCE)
    }

    /// Constructs a categorical distribution using an explicitly selected
    /// normalization tolerance.
    ///
    /// A tolerance of zero requests exact equality of the accumulated `f64`
    /// sum with one. This is intentionally strict and should generally only be
    /// used when the input values are known to sum exactly in the chosen
    /// representation.
    ///
    /// The tolerance must be finite and non-negative.
    pub fn with_tolerance(
        entries: Vec<CategoricalEntry<T>>,
        tolerance: f64,
    ) -> Result<Self, CategoricalError> {
        validate_tolerance(tolerance)?;

        if entries.is_empty() {
            return Err(CategoricalError::Empty);
        }

        let mut sum = 0.0_f64;

        for (index, entry) in entries.iter().enumerate() {
            validate_probability(index, entry.probability)?;

            for (previous_index, previous_entry) in entries[..index].iter().enumerate() {
                if entry.outcome == previous_entry.outcome {
                    return Err(CategoricalError::DuplicateOutcome {
                        first_index: previous_index,
                        duplicate_index: index,
                    });
                }
            }

            sum += entry.probability;
        }

        let difference = (sum - 1.0).abs();

        if difference > tolerance {
            return Err(CategoricalError::NotNormalized {
                sum,
                difference,
                tolerance,
            });
        }

        Ok(Self {
            entries,
            tolerance,
        })
    }

    /// Returns the number of categories.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns `true` if the distribution contains no categories.
    ///
    /// A successfully constructed `Categorical` is never empty, but this
    /// method is provided for collection-style generic code.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the normalization tolerance used to validate this instance.
    #[must_use]
    pub const fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Returns the total probability as stored/accumulated in `f64`.
    ///
    /// For a successfully constructed distribution, the result differs from
    /// one by no more than the distribution's normalization tolerance.
    #[must_use]
    pub fn total_probability(&self) -> f64 {
        self.entries
            .iter()
            .map(CategoricalEntry::probability)
            .sum()
    }

    /// Returns the probability associated with an outcome.
    ///
    /// Returns `None` if the outcome is not present.
    ///
    /// Lookup is linear and deterministic.
    ///
    /// If high-volume indexed lookup is required, callers should maintain an
    /// external index rather than forcing a hash-table policy into this
    /// foundational mathematical type.
    #[must_use]
    pub fn probability_of(&self, outcome: &T) -> Option<f64> {
        self.entries
            .iter()
            .find(|entry| &entry.outcome == outcome)
            .map(CategoricalEntry::probability)
    }

    /// Returns the entry at a zero-based category index.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&CategoricalEntry<T>> {
        self.entries.get(index)
    }

    /// Returns an iterator over entries in deterministic insertion order.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &CategoricalEntry<T>> {
        self.entries.iter()
    }

    /// Returns an iterator over `(outcome, probability)` pairs.
    ///
    /// This is useful for downstream sampling, serialization, channel
    /// construction and statistical analysis without exposing internal
    /// storage.
    pub fn outcomes_and_probabilities(
        &self,
    ) -> impl ExactSizeIterator<Item = (&T, f64)> {
        self.entries
            .iter()
            .map(|entry| (&entry.outcome, entry.probability))
    }

    /// Returns the expected value of a caller-supplied function.
    ///
    /// For outcome function `f`, computes:
    ///
    /// ```text
    /// E[f(X)] = Σ p(x) f(x)
    /// ```
    ///
    /// The function must return a finite value for every outcome.
    ///
    /// A non-finite function result is reported as `None`.
    ///
    /// `None` therefore means that the expectation could not be represented
    /// as a finite `f64`.
    #[must_use]
    pub fn expectation<F>(&self, mut function: F) -> Option<f64>
    where
        F: FnMut(&T) -> f64,
    {
        let mut total = 0.0_f64;

        for entry in &self.entries {
            let value = function(&entry.outcome);

            if !value.is_finite() {
                return None;
            }

            let contribution = entry.probability * value;

            if !contribution.is_finite() {
                return None;
            }

            total += contribution;

            if !total.is_finite() {
                return None;
            }
        }

        Some(total)
    }

    /// Returns the variance of a caller-supplied numerical outcome function.
    ///
    /// Computes:
    ///
    /// ```text
    /// Var(X) = E[X²] - E[X]²
    /// ```
    ///
    /// Small negative values caused by floating-point roundoff are clamped to
    /// zero. A materially negative variance is treated as numerical failure
    /// and returns `None`.
    #[must_use]
    pub fn variance<F>(&self, mut function: F) -> Option<f64>
    where
        F: FnMut(&T) -> f64,
    {
        let mean = self.expectation(&mut function)?;

        let second_moment = self.expectation(|outcome| {
            let value = function(outcome);
            value * value
        })?;

        let variance = second_moment - (mean * mean);

        if !variance.is_finite() {
            return None;
        }

        if variance >= 0.0 {
            return Some(variance);
        }

        // Floating-point cancellation can produce a tiny negative value for
        // an exactly non-negative mathematical variance.
        let scale = second_moment.abs().max((mean * mean).abs()).max(1.0);
        let numerical_tolerance = 64.0 * f64::EPSILON * scale;

        if variance >= -numerical_tolerance {
            Some(0.0)
        } else {
            None
        }
    }

    /// Returns whether the distribution contains an outcome.
    #[must_use]
    pub fn contains(&self, outcome: &T) -> bool {
        self.entries
            .iter()
            .any(|entry| &entry.outcome == outcome)
    }

    /// Consumes the distribution and returns its entries.
    ///
    /// The validation guarantees are no longer relevant after ownership is
    /// transferred because the caller receives the raw entries.
    #[must_use]
    pub fn into_entries(self) -> Vec<CategoricalEntry<T>> {
        self.entries
    }

    /// Returns a borrowed slice of the validated entries.
    ///
    /// This is the preferred zero-copy integration point for downstream ZQN
    /// modules that need to inspect the complete distribution.
    #[must_use]
    pub fn as_slice(&self) -> &[CategoricalEntry<T>] {
        &self.entries
    }
}

impl<T> Index<usize> for Categorical<T>
where
    T: PartialEq,
{
    type Output = CategoricalEntry<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

/// Validates a probability value independently of any distribution.
fn validate_probability(index: usize, probability: f64) -> Result<(), CategoricalError> {
    if !probability.is_finite() {
        return Err(CategoricalError::NonFiniteProbability {
            index,
            value: probability,
        });
    }

    if probability < 0.0 {
        return Err(CategoricalError::NegativeProbability {
            index,
            value: probability,
        });
    }

    Ok(())
}

/// Validates the normalization tolerance.
fn validate_tolerance(tolerance: f64) -> Result<(), CategoricalError> {
    if !tolerance.is_finite() || tolerance < 0.0 {
        return Err(CategoricalError::InvalidTolerance { tolerance });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_valid_distribution() {
        let distribution = Categorical::new(vec![
            CategoricalEntry::new("a", 0.25),
            CategoricalEntry::new("b", 0.75),
        ])
        .expect("valid categorical distribution");

        assert_eq!(distribution.len(), 2);
        assert!(!distribution.is_empty());
        assert_eq!(distribution.probability_of(&"a"), Some(0.25));
        assert_eq!(distribution.probability_of(&"b"), Some(0.75));
        assert!((distribution.total_probability() - 1.0).abs() <= DEFAULT_NORMALIZATION_TOLERANCE);
    }

    #[test]
    fn singleton_distribution_is_valid() {
        let distribution = Categorical::new(vec![CategoricalEntry::new("only", 1.0)])
            .expect("singleton distribution is valid");

        assert_eq!(distribution.len(), 1);
        assert_eq!(distribution.probability_of(&"only"), Some(1.0));
    }

    #[test]
    fn empty_distribution_is_rejected() {
        let result: Result<Categorical<&str>, _> = Categorical::new(Vec::new());

        assert_eq!(result, Err(CategoricalError::Empty));
    }

    #[test]
    fn negative_probability_is_rejected() {
        let result = Categorical::new(vec![
            CategoricalEntry::new("a", -0.1),
            CategoricalEntry::new("b", 1.1),
        ]);

        assert!(matches!(
            result,
            Err(CategoricalError::NegativeProbability {
                index: 0,
                value: -0.1
            })
        ));
    }

    #[test]
    fn nan_probability_is_rejected() {
        let result = Categorical::new(vec![
            CategoricalEntry::new("a", f64::NAN),
        ]);

        assert!(matches!(
            result,
            Err(CategoricalError::NonFiniteProbability { index: 0, .. })
        ));
    }

    #[test]
    fn positive_infinity_is_rejected() {
        let result = Categorical::new(vec![
            CategoricalEntry::new("a", f64::INFINITY),
        ]);

        assert!(matches!(
            result,
            Err(CategoricalError::NonFiniteProbability { index: 0, .. })
        ));
    }

    #[test]
    fn negative_infinity_is_rejected() {
        let result = Categorical::new(vec![
            CategoricalEntry::new("a", f64::NEG_INFINITY),
        ]);

        assert!(matches!(
            result,
            Err(CategoricalError::NonFiniteProbability { index: 0, .. })
        ));
    }

    #[test]
    fn duplicate_outcomes_are_rejected() {
        let result = Categorical::new(vec![
            CategoricalEntry::new("a", 0.5),
            CategoricalEntry::new("a", 0.5),
        ]);

        assert_eq!(
            result,
            Err(CategoricalError::DuplicateOutcome {
                first_index: 0,
                duplicate_index: 1
            })
        );
    }

    #[test]
    fn zero_probability_is_allowed() {
        let distribution = Categorical::new(vec![
            CategoricalEntry::new("never", 0.0),
            CategoricalEntry::new("always", 1.0),
        ])
        .expect("zero-probability category is mathematically valid");

        assert_eq!(distribution.probability_of(&"never"), Some(0.0));
        assert_eq!(distribution.probability_of(&"always"), Some(1.0));
    }

    #[test]
    fn under_normalized_distribution_is_rejected() {
        let result = Categorical::new(vec![
            CategoricalEntry::new("a", 0.2),
            CategoricalEntry::new("b", 0.2),
        ]);

        assert!(matches!(
            result,
            Err(CategoricalError::NotNormalized { .. })
        ));
    }

    #[test]
    fn over_normalized_distribution_is_rejected() {
        let result = Categorical::new(vec![
            CategoricalEntry::new("a", 0.7),
            CategoricalEntry::new("b", 0.7),
        ]);

        assert!(matches!(
            result,
            Err(CategoricalError::NotNormalized { .. })
        ));
    }

    #[test]
    fn custom_tolerance_is_respected() {
        let distribution = Categorical::with_tolerance(
            vec![
                CategoricalEntry::new("a", 0.5),
                CategoricalEntry::new("b", 0.5000000000005),
            ],
            1.0e-9,
        )
        .expect("custom tolerance should accept small numerical discrepancy");

        assert_eq!(distribution.len(), 2);
        assert_eq!(distribution.tolerance(), 1.0e-9);
    }

    #[test]
    fn invalid_tolerance_is_rejected() {
        let result = Categorical::with_tolerance(
            vec![CategoricalEntry::new("a", 1.0)],
            f64::NAN,
        );

        assert!(matches!(
            result,
            Err(CategoricalError::InvalidTolerance { .. })
        ));
    }

    #[test]
    fn negative_tolerance_is_rejected() {
        let result = Categorical::with_tolerance(
            vec![CategoricalEntry::new("a", 1.0)],
            -1.0,
        );

        assert!(matches!(
            result,
            Err(CategoricalError::InvalidTolerance { .. })
        ));
    }

    #[test]
    fn iteration_is_deterministic() {
        let distribution = Categorical::new(vec![
            CategoricalEntry::new("first", 0.2),
            CategoricalEntry::new("second", 0.3),
            CategoricalEntry::new("third", 0.5),
        ])
        .expect("valid distribution");

        let outcomes: Vec<_> = distribution
            .iter()
            .map(CategoricalEntry::outcome)
            .copied()
            .collect();

        assert_eq!(outcomes, vec!["first", "second", "third"]);
    }

    #[test]
    fn outcome_probability_iteration_is_zero_copy() {
        let distribution = Categorical::new(vec![
            CategoricalEntry::new("a", 0.25),
            CategoricalEntry::new("b", 0.75),
        ])
        .expect("valid distribution");

        let values: Vec<_> = distribution.outcomes_and_probabilities().collect();

        assert_eq!(values, vec![(&"a", 0.25), (&"b", 0.75)]);
    }

    #[test]
    fn expectation_is_correct() {
        let distribution = Categorical::new(vec![
            CategoricalEntry::new(1_u32, 0.25),
            CategoricalEntry::new(3_u32, 0.75),
        ])
        .expect("valid distribution");

        let expectation = distribution
            .expectation(|value| f64::from(*value))
            .expect("finite expectation");

        assert!((expectation - 2.5).abs() < 1.0e-14);
    }

    #[test]
    fn variance_is_correct() {
        let distribution = Categorical::new(vec![
            CategoricalEntry::new(1_u32, 0.5),
            CategoricalEntry::new(3_u32, 0.5),
        ])
        .expect("valid distribution");

        let variance = distribution
            .variance(|value| f64::from(*value))
            .expect("finite variance");

        assert!((variance - 1.0).abs() < 1.0e-14);
    }

    #[test]
    fn non_finite_expectation_is_rejected() {
        let distribution = Categorical::new(vec![
            CategoricalEntry::new("a", 1.0),
        ])
        .expect("valid distribution");

        assert_eq!(
            distribution.expectation(|_| f64::INFINITY),
            None
        );
    }

    #[test]
    fn contains_and_get_work() {
        let distribution = Categorical::new(vec![
            CategoricalEntry::new("a", 0.25),
            CategoricalEntry::new("b", 0.75),
        ])
        .expect("valid distribution");

        assert!(distribution.contains(&"a"));
        assert!(distribution.contains(&"b"));
        assert!(!distribution.contains(&"c"));

        assert_eq!(distribution.get(0).map(CategoricalEntry::outcome), Some(&"a"));
        assert!(distribution.get(2).is_none());
    }

    #[test]
    fn indexing_is_supported() {
        let distribution = Categorical::new(vec![
            CategoricalEntry::new("a", 1.0),
        ])
        .expect("valid distribution");

        assert_eq!(distribution[0].outcome(), &"a");
        assert_eq!(distribution[0].probability(), 1.0);
    }

    #[test]
    fn into_entries_preserves_order() {
        let distribution = Categorical::new(vec![
            CategoricalEntry::new("a", 0.25),
            CategoricalEntry::new("b", 0.75),
        ])
        .expect("valid distribution");

        let entries = distribution.into_entries();

        assert_eq!(entries[0].outcome(), &"a");
        assert_eq!(entries[1].outcome(), &"b");
    }

    #[test]
    fn equality_is_structural_and_order_sensitive() {
        let first = Categorical::new(vec![
            CategoricalEntry::new("a", 0.25),
            CategoricalEntry::new("b", 0.75),
        ])
        .expect("valid distribution");

        let second = Categorical::new(vec![
            CategoricalEntry::new("a", 0.25),
            CategoricalEntry::new("b", 0.75),
        ])
        .expect("valid distribution");

        let reordered = Categorical::new(vec![
            CategoricalEntry::new("b", 0.75),
            CategoricalEntry::new("a", 0.25),
        ])
        .expect("valid distribution");

        assert_eq!(first, second);
        assert_ne!(first, reordered);
    }

    #[test]
    fn large_generated_distribution_has_no_architectural_category_limit() {
        let category_count = 10_000_usize;
        let probability = 1.0 / category_count as f64;

        let entries = (0..category_count)
            .map(|index| CategoricalEntry::new(index, probability))
            .collect();

        let distribution = Categorical::new(entries)
            .expect("generated finite categorical distribution should be valid");

        assert_eq!(distribution.len(), category_count);
    }

    #[test]
    fn categorical_entry_into_parts_works() {
        let entry = CategoricalEntry::new("outcome", 0.5);

        let (outcome, probability) = entry.into_parts();

        assert_eq!(outcome, "outcome");
        assert_eq!(probability, 0.5);
    }
}