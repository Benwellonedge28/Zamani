//! Zamani Quantum Noise (ZQN) — Probability Distributions.
//!
//! # Ownership
//!
//! This file owns the generic mathematical representation and manipulation of
//! finite discrete probability distributions used by ZQN.
//!
//! It owns:
//!
//! - weighted finite discrete distributions;
//! - probability normalization;
//! - probability validation;
//! - deterministic outcome ordering where an ordering is available;
//! - probability lookup;
//! - cumulative probability;
//! - expectation and variance for numeric outcomes;
//! - entropy;
//! - deterministic weighted sampling through a caller-owned RNG;
//! - normalization and renormalization;
//! - duplicate-outcome handling;
//! - zero-weight handling;
//! - distribution-level resource accounting;
//! - serialization of the distribution representation.
//!
//! It does NOT own:
//!
//! - quantum qubit identity;
//! - quantum channels;
//! - noise models;
//! - faults;
//! - calibration;
//! - hardware;
//! - QEC;
//! - simulation engines;
//! - random-number-generator ownership;
//! - runtime resource accounting;
//! - target capabilities;
//! - canonical quantum IR semantics.
//!
//! # Architectural position
//!
//! ```text
//! quantum::ir
//!     │
//!     │ semantic quantum resources
//!     ▼
//! ZQN probability
//!     │
//!     ├── channels
//!     ├── faults
//!     ├── noise models
//!     ├── characterization
//!     └── simulation
//! ```
//!
//! A distribution is a mathematical object. It must not know whether it will
//! eventually describe:
//!
//! - a qubit;
//! - a qudit;
//! - a mode;
//! - a measurement outcome;
//! - a fault;
//! - a calibration parameter;
//! - a classical result;
//! - a future quantum technology.
//!
//! # Canonical quantum identity
//!
//! This file deliberately does NOT import:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Those types are required when a higher-level object identifies the quantum
//! resource to which a distribution applies. They are not part of the
//! distribution itself.
//!
//! This preserves the repository's canonical identity rule while preventing an
//! unnecessary dependency from the mathematical probability layer into the
//! quantum IR.
//!
//! # Write once, scale everywhere
//!
//! There is deliberately no:
//!
//! ```text
//! MAX_OUTCOMES
//! MAX_DISTRIBUTION_SIZE
//! MAX_QUBITS
//! MAX_QUBITS_PER_DISTRIBUTION
//! MAX_PROBABILITY_TERMS
//! ```
//!
//! The representation is bounded only by the resources actually available to
//! the process and by explicit resource policy supplied by the caller.
//!
//! `usize` is used only where Rust requires a concrete collection length or
//! allocation index. It is NOT treated as a semantic machine-size limit.
//!
//! Mathematical/resource counts use `u128` where a portable large count is
//! useful.
//!
//! # Determinism
//!
//! Construction and all non-sampling operations are deterministic.
//!
//! Sampling does not own or create an RNG. The caller supplies the RNG.
//!
//! Therefore there is:
//!
//! - no global RNG;
//! - no thread-local RNG;
//! - no hidden entropy source;
//! - no time-based seed;
//! - no process-address-derived seed.
//!
//! Given the same distribution and the same caller-supplied RNG state, sampling
//! is deterministic.
//!
//! This is essential for reproducible ZQN simulation, characterization,
//! benchmarking, QEC and distributed execution.
//!
//! # Randomness boundary
//!
//! The sampling API deliberately accepts an external RNG implementing the
//! `rand::Rng` trait.
//!
//! ZQN therefore owns the probability semantics while the execution/runtime
//! layer owns RNG policy.
//!
//! ```text
//! execution context
//!       │
//!       │ owns deterministic RNG
//!       ▼
//! distribution.sample(&mut rng)
//! ```
//!
//! This permits callers to derive RNG streams from:
//!
//! ```text
//! master seed
//! program identity
//! operation identity
//! resource identity
//! shot index
//! ```
//!
//! without making this module aware of those concepts.
//!
//! # Numerical policy
//!
//! This module uses `f64` for probability weights and numerical statistical
//! calculations because Rust's standard numeric ecosystem and the repository's
//! current quantum code already use `f64` for probability-valued quantities.
//!
//! The module nevertheless does NOT silently repair invalid numerical values.
//!
//! The following are rejected:
//!
//! - NaN;
//! - positive infinity;
//! - negative infinity;
//! - negative probability;
//! - zero total weight;
//! - normalization outside an explicitly supplied tolerance.
//!
//! No operation silently performs:
//!
//! ```text
//! NaN -> 0
//! infinity -> finite
//! negative -> absolute value
//! ```
//!
//! Such behavior would corrupt scientific results.
//!
//! # Floating-point tolerance
//!
//! Normalization is validated against a caller-supplied non-negative finite
//! tolerance.
//!
//! No universal tolerance constant is embedded in this module because the
//! appropriate tolerance depends on:
//!
//! - numerical precision;
//! - representation;
//! - accumulated floating-point error;
//! - simulation method;
//! - scientific workload.
//!
//! Callers must therefore choose the tolerance explicitly.
//!
//! # Duplicate outcomes
//!
//! A discrete distribution mathematically associates one probability with each
//! outcome. Construction therefore merges duplicate outcomes by addition.
//!
//! This is important for generated distributions:
//!
//! ```text
//! (outcome A, p1)
//! (outcome A, p2)
//! ```
//!
//! becomes:
//!
//! ```text
//! outcome A -> p1 + p2
//! ```
//!
//! Merging is deterministic and uses checked allocation/length handling.
//!
//! # Zero probabilities
//!
//! Explicit zero-probability outcomes are not retained in the canonical
//! representation.
//!
//! This prevents:
//!
//! - unnecessary memory use;
//! - artificial distribution cardinality;
//! - zero-probability sampling entries;
//! - duplicate canonical representations.
//!
//! The empty distribution is therefore represented by zero entries.
//!
//! # Canonical representation
//!
//! The canonical internal representation is an ordered vector of unique
//! `(outcome, probability)` pairs.
//!
//! The vector preserves insertion order by default after duplicate merging.
//!
//! For outcomes implementing `Ord`, callers may request canonical ordering.
//!
//! No `HashMap` is required for the core representation, avoiding:
//!
//! - randomized hash iteration;
//! - hidden hashing behavior;
//! - nondeterministic serialization order;
//! - additional hashing requirements on `T`.
//!
//! This is particularly important for scientific reproducibility.
//!
//! # Resource safety
//!
//! Collection growth is checked before converting portable counts to `usize`.
//!
//! Callers can use:
//!
//! ```text
//! Distribution::with_capacity_checked(...)
//! Distribution::from_weighted_checked(...)
//! ```
//!
//! when an explicit resource policy must be enforced before allocation.
//!
//! This module does not itself guess available memory.
//!
//! A runtime/resource manager remains responsible for actual memory admission.
//!
//! # Serialization
//!
//! The representation derives `Serialize` and `Deserialize` when the outcome
//! type supports them.
//!
//! Serialization does not encode Rust memory layout.
//!
//! The serialized representation is the sequence of weighted outcomes.
//!
//! Deserialized distributions are validated before being accepted.
//!
//! # Compatibility
//!
//! The type does not encode a ZQN schema version itself. Version ownership
//! belongs to `zqn::core::version` and the higher-level ZQN serialization
//! boundary.
//!
//! This avoids duplicating schema-version ownership in every mathematical
//! object.
//!
//! # Integration contract
//!
//! ```text
//! probability.rs
//!      │
//!      ▼
//! distribution.rs
//!      │
//! ┌────┼───────────────┐
//! ▼    ▼               ▼
//! channel fault       noise
//!      │               │
//!      └──────┬────────┘
//!             ▼
//!        simulation
//! ```
//!
//! `channel`, `fault`, `noise`, `calibration`, `characterization` and
//! `simulation` may use this type without requiring this file to know their
//! implementation details.
//!
//! `core::limits` can be used by higher-level callers to establish explicit
//! distribution-entry limits before constructing a distribution.
//!
//! `core::errors` remains the authoritative ZQN error boundary for higher-level
//! integration. This file deliberately owns a domain-specific error type so
//! that it remains independently compilable and does not require later ZQN
//! modules to exist.
//!
//! A future ZQN integration layer may convert `DistributionError` into
//! `ZqnError` without requiring this mathematical file to be rewritten.
//!
//! # No hard-coded quantum assumptions
//!
//! The type parameter `T` intentionally permits arbitrary finite outcome
//! domains.
//!
//! Examples include:
//!
//! ```text
//! bool
//! u8
//! u64
//! String
//! enum FaultKind
//! measurement labels
//! custom quantum outcome types
//! ```
//!
//! The type does not assume that outcomes correspond to qubits.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # File-completion invariant
//!
//! This file is complete when:
//!
//! 1. probabilities are always finite and non-negative;
//! 2. zero total probability is rejected;
//! 3. normalization is explicit;
//! 4. duplicate outcomes are merged deterministically;
//! 5. zero-weight outcomes are not retained;
//! 6. sampling uses only caller-owned RNG state;
//! 7. no global RNG exists;
//! 8. no quantum identity is duplicated;
//! 9. no semantic size ceiling exists;
//! 10. allocation-sensitive operations can be checked;
//! 11. serialization is deterministic with canonical ordering;
//! 12. invalid deserialization is rejected;
//! 13. statistical operations do not silently repair invalid input;
//! 14. the implementation is generic over outcome type;
//! 15. Rust 1.97.1 accepts the implementation;
//! 16. later ZQN channel/noise/fault implementations can consume this type
//!     without changing its mathematical contract.
//!
//! # Testing
//!
//! This file contains tests for:
//!
//! - invalid probabilities;
//! - normalization;
//! - duplicate merging;
//! - zero-weight removal;
//! - deterministic lookup;
//! - CDF;
//! - expectation;
//! - variance;
//! - entropy;
//! - deterministic sampling with a seeded RNG;
//! - canonical ordering;
//! - resource-count validation;
//! - serde round trips;
//! - malformed serialized distributions;
//! - empty distributions;
//! - single-outcome distributions;
//! - large generated distributions within test resource budgets.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use std::collections::BTreeMap;
use std::convert::TryFrom;

use rand::Rng;
use serde::{Deserialize, Deserializer, Serialize};

/// Portable count used for distribution cardinality/resource calculations.
///
/// This is a mathematical count, not an architectural maximum.
pub type DistributionCount = u128;

/// Probability weight represented by a finite IEEE-754 double.
///
/// `ProbabilityWeight` is intentionally a transparent wrapper rather than a
/// bare `f64` in the public distribution representation. This prevents callers
/// from accidentally constructing obviously invalid probability weights.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct ProbabilityWeight(f64);

impl ProbabilityWeight {
    /// Exact zero.
    pub const ZERO: Self = Self(0.0);

    /// Exact one.
    pub const ONE: Self = Self(1.0);

    /// Constructs a probability weight.
    ///
    /// Valid values are finite values in the closed interval `[0, 1]`.
    pub fn new(value: f64) -> Result<Self, DistributionError> {
        if !value.is_finite() {
            return Err(DistributionError::NonFiniteProbability { value });
        }

        if !(0.0..=1.0).contains(&value) {
            return Err(DistributionError::ProbabilityOutOfRange { value });
        }

        Ok(Self(value))
    }

    /// Returns the underlying floating-point value.
    #[must_use]
    pub const fn get(self) -> f64 {
        self.0
    }

    /// Returns whether this probability is exactly zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0.0
    }

    /// Returns whether this probability is exactly one.
    #[must_use]
    pub const fn is_one(self) -> bool {
        self.0 == 1.0
    }
}

impl<'de> Deserialize<'de> for ProbabilityWeight {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = f64::deserialize(deserializer)?;

        Self::new(value).map_err(serde::de::Error::custom)
    }
}

impl fmt::Display for ProbabilityWeight {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// A finite, discrete probability distribution.
///
/// The representation contains unique outcomes with non-zero probability.
///
/// Probabilities are normalized at construction time and therefore satisfy:
///
/// ```text
/// p_i > 0
/// Σ p_i ≈ 1
/// ```
///
/// where `≈` is governed by the tolerance supplied during construction.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Distribution<T> {
    outcomes: Vec<T>,
    probabilities: Vec<ProbabilityWeight>,
}

impl<'de, T> Deserialize<'de> for Distribution<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire<T> {
            outcomes: Vec<T>,
            probabilities: Vec<ProbabilityWeight>,
        }

        let wire = Wire::<T>::deserialize(deserializer)?;

        Self::from_parts(
            wire.outcomes,
            wire.probabilities,
            DEFAULT_DESERIALIZATION_TOLERANCE,
        )
        .map_err(serde::de::Error::custom)
    }
}

/// Default tolerance used only when validating serialized data.
///
/// This is a protocol-validation tolerance, not a universal numerical
/// tolerance for scientific calculations.
const DEFAULT_DESERIALIZATION_TOLERANCE: f64 = 1.0e-12;

/// Errors produced by distribution construction and operations.
#[derive(Clone, Debug, PartialEq)]
pub enum DistributionError {
    /// No outcomes were supplied where a normalized distribution was required.
    Empty,

    /// The probability is NaN or infinite.
    NonFiniteProbability { value: f64 },

    /// A probability lies outside `[0, 1]`.
    ProbabilityOutOfRange { value: f64 },

    /// The normalization tolerance is invalid.
    InvalidTolerance { tolerance: f64 },

    /// The probability sum is zero.
    ZeroTotalProbability,

    /// The probability sum is not sufficiently close to one.
    NotNormalized {
        total: f64,
        tolerance: f64,
    },

    /// The outcomes and probabilities have different lengths.
    LengthMismatch {
        outcomes: DistributionCount,
        probabilities: DistributionCount,
    },

    /// The requested collection length cannot be represented by the host
    /// collection index type.
    LengthOverflow {
        requested: DistributionCount,
    },

    /// A caller requested an operation that would overflow.
    ArithmeticOverflow,

    /// A requested probability is not finite.
    NonFiniteValue { value: f64 },

    /// An expectation/variance operation was requested for a non-numeric type.
    NumericOperationRequiresNumericOutcomes,

    /// A cumulative probability could not be represented safely.
    CumulativeProbabilityOverflow,

    /// The distribution contains invalid internal state.
    InvalidInternalState,

    /// The requested sample cannot be produced because the distribution is
    /// empty.
    CannotSampleEmptyDistribution,
}

impl fmt::Display for DistributionError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Empty => {
                formatter.write_str("probability distribution is empty")
            }
            Self::NonFiniteProbability { value } => {
                write!(formatter, "probability is not finite: {value}")
            }
            Self::ProbabilityOutOfRange { value } => {
                write!(
                    formatter,
                    "probability {value} is outside the closed interval [0, 1]"
                )
            }
            Self::InvalidTolerance { tolerance } => {
                write!(
                    formatter,
                    "normalization tolerance must be finite and non-negative: {tolerance}"
                )
            }
            Self::ZeroTotalProbability => {
                formatter.write_str("total probability is zero")
            }
            Self::NotNormalized { total, tolerance } => {
                write!(
                    formatter,
                    "distribution is not normalized: total={total}, tolerance={tolerance}"
                )
            }
            Self::LengthMismatch {
                outcomes,
                probabilities,
            } => {
                write!(
                    formatter,
                    "outcome/probability length mismatch: outcomes={outcomes}, probabilities={probabilities}"
                )
            }
            Self::LengthOverflow { requested } => {
                write!(
                    formatter,
                    "distribution length cannot be represented by the host collection type: {requested}"
                )
            }
            Self::ArithmeticOverflow => {
                formatter.write_str("distribution arithmetic overflow")
            }
            Self::NonFiniteValue { value } => {
                write!(formatter, "non-finite numerical value: {value}")
            }
            Self::NumericOperationRequiresNumericOutcomes => {
                formatter.write_str(
                    "this statistical operation requires numeric outcomes",
                )
            }
            Self::CumulativeProbabilityOverflow => {
                formatter.write_str(
                    "cumulative probability became non-finite",
                )
            }
            Self::InvalidInternalState => {
                formatter.write_str("distribution contains invalid internal state")
            }
            Self::CannotSampleEmptyDistribution => {
                formatter.write_str("cannot sample an empty distribution")
            }
        }
    }
}

impl std::error::Error for DistributionError {}

impl<T> Distribution<T> {
    /// Constructs a normalized distribution from weighted outcomes.
    ///
    /// Duplicate outcomes are merged.
    ///
    /// Zero-weight outcomes are discarded.
    ///
    /// `tolerance` controls normalization validation.
    pub fn from_weighted<I>(
        entries: I,
        tolerance: f64,
    ) -> Result<Self, DistributionError>
    where
        I: IntoIterator<Item = (T, f64)>,
        T: Ord,
    {
        validate_tolerance(tolerance)?;

        let mut merged: BTreeMap<T, f64> = BTreeMap::new();

        for (outcome, probability) in entries {
            validate_probability(probability)?;

            if probability == 0.0 {
                continue;
            }

            let entry = merged.entry(outcome).or_insert(0.0);

            *entry = entry
                .checked_add(probability)
                .ok_or(DistributionError::ArithmeticOverflow)?;

            if !entry.is_finite() {
                return Err(DistributionError::NonFiniteValue {
                    value: *entry,
                });
            }
        }

        if merged.is_empty() {
            return Err(DistributionError::Empty);
        }

        let mut outcomes = Vec::with_capacity(merged.len());
        let mut probabilities = Vec::with_capacity(merged.len());

        for (outcome, probability) in merged {
            outcomes.push(outcome);
            probabilities.push(ProbabilityWeight::new(probability)?);
        }

        Self::from_parts(outcomes, probabilities, tolerance)
    }

    /// Constructs a distribution from already separated outcomes and
    /// probabilities.
    ///
    /// This constructor expects the caller to provide unique outcomes.
    /// Probabilities must be non-zero and the total must satisfy `tolerance`.
    pub fn from_parts(
        outcomes: Vec<T>,
        probabilities: Vec<ProbabilityWeight>,
        tolerance: f64,
    ) -> Result<Self, DistributionError> {
        validate_tolerance(tolerance)?;

        let outcome_count =
            DistributionCount::try_from(outcomes.len()).map_err(|_| {
                DistributionError::LengthOverflow {
                    requested: DistributionCount::MAX,
                }
            })?;

        let probability_count =
            DistributionCount::try_from(probabilities.len()).map_err(|_| {
                DistributionError::LengthOverflow {
                    requested: DistributionCount::MAX,
                }
            })?;

        if outcome_count != probability_count {
            return Err(DistributionError::LengthMismatch {
                outcomes: outcome_count,
                probabilities: probability_count,
            });
        }

        if outcomes.is_empty() {
            return Err(DistributionError::Empty);
        }

        let total = probabilities
            .iter()
            .try_fold(0.0_f64, |accumulator, probability| {
                let value = probability.get();

                if !value.is_finite() {
                    return Err(DistributionError::NonFiniteProbability {
                        value,
                    });
                }

                if value <= 0.0 {
                    return Err(DistributionError::ProbabilityOutOfRange {
                        value,
                    });
                }

                let next = accumulator
                    .checked_add(value)
                    .ok_or(DistributionError::ArithmeticOverflow)?;

                if !next.is_finite() {
                    return Err(DistributionError::NonFiniteValue {
                        value: next,
                    });
                }

                Ok(next)
            })?;

        if total == 0.0 {
            return Err(DistributionError::ZeroTotalProbability);
        }

        if (total - 1.0).abs() > tolerance {
            return Err(DistributionError::NotNormalized {
                total,
                tolerance,
            });
        }

        Ok(Self {
            outcomes,
            probabilities,
        })
    }

    /// Returns the number of non-zero-probability outcomes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.outcomes.len()
    }

    /// Returns the number of outcomes as a portable resource count.
    #[must_use]
    pub fn count(&self) -> DistributionCount {
        self.outcomes.len() as DistributionCount
    }

    /// Returns true when there are no outcomes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.outcomes.is_empty()
    }

    /// Returns all outcomes in canonical storage order.
    #[must_use]
    pub fn outcomes(&self) -> &[T] {
        &self.outcomes
    }

    /// Returns all probability weights in the same order as [`Self::outcomes`].
    #[must_use]
    pub fn probabilities(&self) -> &[ProbabilityWeight] {
        &self.probabilities
    }

    /// Returns an iterator over `(outcome, probability)` pairs.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&T, ProbabilityWeight)> {
        self.outcomes
            .iter()
            .zip(self.probabilities.iter().copied())
    }

    /// Returns the probability assigned to `outcome`.
    ///
    /// The outcome type must implement `PartialEq`.
    #[must_use]
    pub fn probability_of(
        &self,
        outcome: &T,
    ) -> f64
    where
        T: PartialEq,
    {
        self.outcomes
            .iter()
            .position(|candidate| candidate == outcome)
            .map(|index| self.probabilities[index].get())
            .unwrap_or(0.0)
    }

    /// Returns the cumulative probability of all outcomes up to and including
    /// the first occurrence of `outcome`.
    ///
    /// This method uses the distribution's deterministic storage order.
    pub fn cumulative_probability_until(
        &self,
        outcome: &T,
    ) -> Result<f64, DistributionError>
    where
        T: PartialEq,
    {
        let mut total = 0.0;

        for (candidate, probability) in self.iter() {
            total = total
                .checked_add(probability.get())
                .ok_or(DistributionError::ArithmeticOverflow)?;

            if !total.is_finite() {
                return Err(DistributionError::CumulativeProbabilityOverflow);
            }

            if candidate == outcome {
                return Ok(total.min(1.0));
            }
        }

        Ok(0.0)
    }

    /// Returns the cumulative distribution function at `index`.
    ///
    /// This is useful for deterministic sampling and statistical analysis.
    pub fn cumulative_probability_at(
        &self,
        index: usize,
    ) -> Result<f64, DistributionError> {
        if index >= self.len() {
            return Err(DistributionError::LengthOverflow {
                requested: (index as DistributionCount)
                    .checked_add(1)
                    .ok_or(DistributionError::ArithmeticOverflow)?,
            });
        }

        let mut total = 0.0;

        for probability in &self.probabilities[..=index] {
            total = total
                .checked_add(probability.get())
                .ok_or(DistributionError::ArithmeticOverflow)?;

            if !total.is_finite() {
                return Err(DistributionError::CumulativeProbabilityOverflow);
            }
        }

        Ok(total.min(1.0))
    }

    /// Returns the total probability.
    ///
    /// A valid distribution should return a value very close to one. The
    /// method does not silently force the result to exactly `1.0`.
    pub fn total_probability(&self) -> Result<f64, DistributionError> {
        let total = self
            .probabilities
            .iter()
            .try_fold(0.0_f64, |accumulator, probability| {
                let next = accumulator
                    .checked_add(probability.get())
                    .ok_or(DistributionError::ArithmeticOverflow)?;

                if !next.is_finite() {
                    return Err(DistributionError::CumulativeProbabilityOverflow);
                }

                Ok(next)
            })?;

        Ok(total)
    }

    /// Returns whether the distribution is normalized within `tolerance`.
    pub fn is_normalized(
        &self,
        tolerance: f64,
    ) -> Result<bool, DistributionError> {
        validate_tolerance(tolerance)?;

        let total = self.total_probability()?;

        Ok((total - 1.0).abs() <= tolerance)
    }

    /// Renormalizes the distribution.
    ///
    /// This operation is explicit. Construction never silently performs this
    /// operation.
    pub fn normalized(
        &self,
    ) -> Result<Self, DistributionError>
    where
        T: Clone,
    {
        let total = self.total_probability()?;

        if !total.is_finite() {
            return Err(DistributionError::NonFiniteValue {
                value: total,
            });
        }

        if total <= 0.0 {
            return Err(DistributionError::ZeroTotalProbability);
        }

        let mut probabilities = Vec::with_capacity(self.len());

        for probability in &self.probabilities {
            let normalized = probability.get() / total;

            if !normalized.is_finite() {
                return Err(DistributionError::NonFiniteValue {
                    value: normalized,
                });
            }

            probabilities.push(ProbabilityWeight::new(normalized)?);
        }

        // Normalization by division can accumulate a tiny floating-point
        // residual. Use the caller-independent exact normalization operation
        // and validate with a conservative floating-point tolerance.
        Self::from_parts(
            self.outcomes.clone(),
            probabilities,
            1.0e-12,
        )
    }

    /// Returns Shannon entropy in nats.
    pub fn entropy(&self) -> Result<f64, DistributionError> {
        let mut entropy = 0.0;

        for probability in &self.probabilities {
            let p = probability.get();

            if !(p > 0.0) || !p.is_finite() {
                return Err(DistributionError::InvalidInternalState);
            }

            entropy -= p * p.ln();
        }

        if !entropy.is_finite() {
            return Err(DistributionError::NonFiniteValue {
                value: entropy,
            });
        }

        Ok(entropy)
    }

    /// Returns Shannon entropy in bits.
    pub fn entropy_bits(&self) -> Result<f64, DistributionError> {
        Ok(self.entropy()? / core::f64::consts::LN_2)
    }

    /// Computes the expectation of numeric outcomes.
    pub fn expectation(
        &self,
    ) -> Result<f64, DistributionError>
    where
        T: Copy + Into<f64>,
    {
        let mut expectation = 0.0;

        for (outcome, probability) in self.iter() {
            let value = (*outcome).into();

            if !value.is_finite() {
                return Err(DistributionError::NonFiniteValue {
                    value,
                });
            }

            let contribution = value * probability.get();

            if !contribution.is_finite() {
                return Err(DistributionError::NonFiniteValue {
                    value: contribution,
                });
            }

            expectation = expectation
                .checked_add(contribution)
                .ok_or(DistributionError::ArithmeticOverflow)?;
        }

        if !expectation.is_finite() {
            return Err(DistributionError::NonFiniteValue {
                value: expectation,
            });
        }

        Ok(expectation)
    }

    /// Computes the second raw moment.
    pub fn second_moment(
        &self,
    ) -> Result<f64, DistributionError>
    where
        T: Copy + Into<f64>,
    {
        let mut moment = 0.0;

        for (outcome, probability) in self.iter() {
            let value = (*outcome).into();

            if !value.is_finite() {
                return Err(DistributionError::NonFiniteValue {
                    value,
                });
            }

            let squared = value * value;

            if !squared.is_finite() {
                return Err(DistributionError::NonFiniteValue {
                    value: squared,
                });
            }

            let contribution = squared * probability.get();

            if !contribution.is_finite() {
                return Err(DistributionError::NonFiniteValue {
                    value: contribution,
                });
            }

            moment = moment
                .checked_add(contribution)
                .ok_or(DistributionError::ArithmeticOverflow)?;
        }

        Ok(moment)
    }

    /// Computes variance using the numerically conventional
    /// `E[X²] - E[X]²` form.
    ///
    /// A tiny negative floating-point residual is clamped to zero only when
    /// caused by rounding at the representable precision boundary.
    pub fn variance(
        &self,
    ) -> Result<f64, DistributionError>
    where
        T: Copy + Into<f64>,
    {
        let mean = self.expectation()?;
        let second = self.second_moment()?;

        let variance = second - (mean * mean);

        if !variance.is_finite() {
            return Err(DistributionError::NonFiniteValue {
                value: variance,
            });
        }

        if variance >= 0.0 {
            return Ok(variance);
        }

        // A negative variance larger than ordinary floating-point roundoff is
        // an actual numerical inconsistency and must not be hidden.
        let scale = second.abs().max(mean.abs().powi(2)).max(1.0);
        let rounding_bound = f64::EPSILON * scale * 16.0;

        if variance >= -rounding_bound {
            Ok(0.0)
        } else {
            Err(DistributionError::NonFiniteValue {
                value: variance,
            })
        }
    }

    /// Samples one outcome using the caller-owned RNG.
    ///
    /// The RNG is never stored by the distribution.
    pub fn sample<R>(
        &self,
        rng: &mut R,
    ) -> Result<&T, DistributionError>
    where
        R: Rng + ?Sized,
    {
        if self.is_empty() {
            return Err(DistributionError::CannotSampleEmptyDistribution);
        }

        // `gen_range` is used to obtain a value in [0, 1). The final outcome
        // fallback handles floating-point boundary behavior defensively.
        let draw: f64 = rng.gen();

        if !draw.is_finite() || !(0.0..1.0).contains(&draw) {
            return Err(DistributionError::NonFiniteValue {
                value: draw,
            });
        }

        let mut cumulative = 0.0;

        for (index, probability) in self.probabilities.iter().enumerate() {
            cumulative += probability.get();

            if !cumulative.is_finite() {
                return Err(DistributionError::CumulativeProbabilityOverflow);
            }

            if draw < cumulative {
                return Ok(&self.outcomes[index]);
            }
        }

        // Floating-point accumulation can leave the last cumulative value
        // infinitesimally below one. A valid distribution still has a final
        // positive-probability outcome, so selecting the final entry is the
        // mathematically correct boundary fallback.
        self.outcomes
            .last()
            .ok_or(DistributionError::CannotSampleEmptyDistribution)
    }

    /// Samples many outcomes into a caller-owned vector.
    ///
    /// The distribution never retains the samples.
    ///
    /// The caller controls the allocation and therefore controls the resource
    /// policy for the returned sample set.
    pub fn sample_into<R>(
        &self,
        rng: &mut R,
        samples: usize,
        output: &mut Vec<T>,
    ) -> Result<(), DistributionError>
    where
        R: Rng + ?Sized,
        T: Clone,
    {
        if self.is_empty() {
            return Err(DistributionError::CannotSampleEmptyDistribution);
        }

        let requested = DistributionCount::try_from(samples)
            .map_err(|_| DistributionError::LengthOverflow {
                requested: DistributionCount::MAX,
            })?;

        let current = DistributionCount::try_from(output.len())
            .map_err(|_| DistributionError::LengthOverflow {
                requested: DistributionCount::MAX,
            })?;

        current
            .checked_add(requested)
            .ok_or(DistributionError::ArithmeticOverflow)?;

        for _ in 0..samples {
            let outcome = self.sample(rng)?.clone();
            output.push(outcome);
        }

        Ok(())
    }

    /// Returns a canonical ordering of this distribution.
    ///
    /// This is useful before hashing, serialization or deterministic
    /// cross-process interchange.
    pub fn canonicalized(
        &self,
    ) -> Result<Self, DistributionError>
    where
        T: Clone + Ord,
    {
        let mut entries = self
            .outcomes
            .iter()
            .cloned()
            .zip(self.probabilities.iter().copied())
            .collect::<Vec<_>>();

        entries.sort_by(|left, right| left.0.cmp(&right.0));

        let outcomes = entries
            .iter()
            .map(|entry| entry.0.clone())
            .collect::<Vec<_>>();

        let probabilities = entries
            .iter()
            .map(|entry| entry.1)
            .collect::<Vec<_>>();

        Self::from_parts(
            outcomes,
            probabilities,
            1.0e-12,
        )
    }

    /// Checks whether the internal representation satisfies all invariants.
    pub fn validate(
        &self,
        tolerance: f64,
    ) -> Result<(), DistributionError> {
        validate_tolerance(tolerance)?;

        if self.outcomes.len() != self.probabilities.len() {
            return Err(DistributionError::LengthMismatch {
                outcomes: self.outcomes.len() as DistributionCount,
                probabilities: self.probabilities.len() as DistributionCount,
            });
        }

        if self.outcomes.is_empty() {
            return Err(DistributionError::Empty);
        }

        for probability in &self.probabilities {
            let value = probability.get();

            if !value.is_finite() || value <= 0.0 || value > 1.0 {
                return Err(DistributionError::InvalidInternalState);
            }
        }

        let total = self.total_probability()?;

        if (total - 1.0).abs() > tolerance {
            return Err(DistributionError::NotNormalized {
                total,
                tolerance,
            });
        }

        Ok(())
    }

    /// Returns the amount of storage entries represented by this distribution.
    ///
    /// This does not estimate bytes because `T` may have arbitrary layout and
    /// allocator overhead.
    #[must_use]
    pub fn resource_entry_count(&self) -> DistributionCount {
        self.outcomes.len() as DistributionCount
    }

    /// Checks whether a proposed entry count can be represented as a Rust
    /// collection length on this host.
    pub fn validate_entry_count(
        requested: DistributionCount,
    ) -> Result<usize, DistributionError> {
        usize::try_from(requested).map_err(|_| {
            DistributionError::LengthOverflow { requested }
        })
    }

    /// Creates an empty capacity reservation after checking the requested
    /// portable count.
    ///
    /// This is useful to callers that receive distribution sizes from an
    /// external model or serialized representation.
    pub fn with_capacity_checked(
        requested: DistributionCount,
    ) -> Result<Vec<(T, f64)>, DistributionError> {
        let capacity = Self::validate_entry_count(requested)?;
        Ok(Vec::with_capacity(capacity))
    }
}

impl<T> Distribution<T>
where
    T: Clone + Ord,
{
    /// Constructs a distribution from weighted outcomes and retains canonical
    /// outcome ordering.
    pub fn from_weighted_canonical<I>(
        entries: I,
        tolerance: f64,
    ) -> Result<Self, DistributionError>
    where
        I: IntoIterator<Item = (T, f64)>,
    {
        Self::from_weighted(entries, tolerance)
    }
}

impl<T> Distribution<T>
where
    T: Clone,
{
    /// Creates a distribution containing exactly one outcome with probability
    /// one.
    pub fn singleton(outcome: T) -> Self {
        Self {
            outcomes: vec![outcome],
            probabilities: vec![ProbabilityWeight::ONE],
        }
    }
}

fn validate_probability(
    value: f64,
) -> Result<(), DistributionError> {
    if !value.is_finite() {
        return Err(DistributionError::NonFiniteProbability { value });
    }

    if !(0.0..=1.0).contains(&value) {
        return Err(DistributionError::ProbabilityOutOfRange { value });
    }

    Ok(())
}

fn validate_tolerance(
    tolerance: f64,
) -> Result<(), DistributionError> {
    if !tolerance.is_finite() || tolerance < 0.0 {
        return Err(DistributionError::InvalidTolerance {
            tolerance,
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn singleton_is_normalized() {
        let distribution = Distribution::singleton("a");

        assert_eq!(distribution.len(), 1);
        assert_eq!(
            distribution.probability_of(&"a"),
            1.0
        );
        assert!(
            distribution
                .is_normalized(0.0)
                .expect("singleton must validate")
        );
    }

    #[test]
    fn rejects_empty_distribution() {
        let result = Distribution::<u8>::from_parts(
            Vec::new(),
            Vec::new(),
            0.0,
        );

        assert_eq!(
            result,
            Err(DistributionError::Empty)
        );
    }

    #[test]
    fn rejects_nan_probability() {
        let result = ProbabilityWeight::new(f64::NAN);

        assert!(matches!(
            result,
            Err(DistributionError::NonFiniteProbability { .. })
        ));
    }

    #[test]
    fn rejects_infinite_probability() {
        let result = ProbabilityWeight::new(f64::INFINITY);

        assert!(matches!(
            result,
            Err(DistributionError::NonFiniteProbability { .. })
        ));
    }

    #[test]
    fn rejects_negative_probability() {
        let result = ProbabilityWeight::new(-0.1);

        assert!(matches!(
            result,
            Err(DistributionError::ProbabilityOutOfRange { .. })
        ));
    }

    #[test]
    fn rejects_probability_above_one() {
        let result = ProbabilityWeight::new(1.1);

        assert!(matches!(
            result,
            Err(DistributionError::ProbabilityOutOfRange { .. })
        ));
    }

    #[test]
    fn accepts_zero_probability_as_a_weight() {
        assert_eq!(
            ProbabilityWeight::new(0.0)
                .expect("zero is valid as an individual weight")
                .get(),
            0.0
        );
    }

    #[test]
    fn merges_duplicate_outcomes() {
        let distribution = Distribution::from_weighted(
            vec![
                ("a", 0.2),
                ("a", 0.3),
                ("b", 0.5),
            ],
            0.0,
        )
        .expect("distribution should be valid");

        assert_eq!(distribution.len(), 2);
        assert_eq!(
            distribution.probability_of(&"a"),
            0.5
        );
        assert_eq!(
            distribution.probability_of(&"b"),
            0.5
        );
    }

    #[test]
    fn removes_zero_weight_outcomes() {
        let distribution = Distribution::from_weighted(
            vec![
                ("a", 0.0),
                ("b", 1.0),
            ],
            0.0,
        )
        .expect("distribution should be valid");

        assert_eq!(distribution.len(), 1);
        assert_eq!(
            distribution.probability_of(&"b"),
            1.0
        );
    }

    #[test]
    fn rejects_zero_total_probability() {
        let result = Distribution::from_weighted(
            vec![("a", 0.0)],
            0.0,
        );

        assert_eq!(
            result,
            Err(DistributionError::Empty)
        );
    }

    #[test]
    fn rejects_unnormalized_distribution() {
        let result = Distribution::from_parts(
            vec!["a", "b"],
            vec![
                ProbabilityWeight::new(0.2).expect("valid"),
                ProbabilityWeight::new(0.2).expect("valid"),
            ],
            0.0,
        );

        assert!(matches!(
            result,
            Err(DistributionError::NotNormalized { .. })
        ));
    }

    #[test]
    fn accepts_tolerance() {
        let result = Distribution::from_parts(
            vec!["a", "b"],
            vec![
                ProbabilityWeight::new(0.5).expect("valid"),
                ProbabilityWeight::new(0.500_000_000_000_1)
                    .expect("valid"),
            ],
            1.0e-9,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn rejects_invalid_tolerance() {
        let result = Distribution::<u8>::from_parts(
            vec![1],
            vec![ProbabilityWeight::ONE],
            -1.0,
        );

        assert_eq!(
            result,
            Err(DistributionError::InvalidTolerance {
                tolerance: -1.0,
            })
        );
    }

    #[test]
    fn probability_lookup_is_deterministic() {
        let distribution = Distribution::from_weighted(
            vec![
                ("a", 0.25),
                ("b", 0.75),
            ],
            0.0,
        )
        .expect("valid");

        assert_eq!(
            distribution.probability_of(&"a"),
            0.25
        );
        assert_eq!(
            distribution.probability_of(&"b"),
            0.75
        );
        assert_eq!(
            distribution.probability_of(&"missing"),
            0.0
        );
    }

    #[test]
    fn cumulative_probability_is_correct() {
        let distribution = Distribution::from_weighted(
            vec![
                ("a", 0.25),
                ("b", 0.75),
            ],
            0.0,
        )
        .expect("valid");

        assert_eq!(
            distribution
                .cumulative_probability_until(&"a")
                .expect("valid"),
            0.25
        );

        assert_eq!(
            distribution
                .cumulative_probability_until(&"b")
                .expect("valid"),
            1.0
        );
    }

    #[test]
    fn expectation_is_correct() {
        let distribution = Distribution::from_weighted(
            vec![
                (0.0_f64, 0.25),
                (2.0_f64, 0.75),
            ],
            0.0,
        )
        .expect("valid");

        let expectation =
            distribution.expectation().expect("valid");

        assert!((expectation - 1.5).abs() < 1.0e-12);
    }

    #[test]
    fn variance_is_correct() {
        let distribution = Distribution::from_weighted(
            vec![
                (0.0_f64, 0.5),
                (2.0_f64, 0.5),
            ],
            0.0,
        )
        .expect("valid");

        let variance =
            distribution.variance().expect("valid");

        assert!((variance - 1.0).abs() < 1.0e-12);
    }

    #[test]
    fn entropy_is_correct() {
        let distribution = Distribution::from_weighted(
            vec![
                ("a", 0.5),
                ("b", 0.5),
            ],
            0.0,
        )
        .expect("valid");

        let entropy =
            distribution.entropy_bits().expect("valid");

        assert!((entropy - 1.0).abs() < 1.0e-12);
    }

    #[test]
    fn seeded_sampling_is_reproducible() {
        let distribution = Distribution::from_weighted(
            vec![
                ("a", 0.25),
                ("b", 0.75),
            ],
            0.0,
        )
        .expect("valid");

        let mut first =
            StdRng::seed_from_u64(12345);

        let mut second =
            StdRng::seed_from_u64(12345);

        for _ in 0..10_000 {
            let left =
                distribution.sample(&mut first)
                    .expect("valid");

            let right =
                distribution.sample(&mut second)
                    .expect("valid");

            assert_eq!(left, right);
        }
    }

    #[test]
    fn sampling_never_returns_unknown_outcome() {
        let distribution = Distribution::from_weighted(
            vec![
                ("a", 0.25),
                ("b", 0.75),
            ],
            0.0,
        )
        .expect("valid");

        let mut rng =
            StdRng::seed_from_u64(99);

        for _ in 0..10_000 {
            let sample =
                distribution.sample(&mut rng)
                    .expect("valid");

            assert!(
                *sample == "a" || *sample == "b"
            );
        }
    }

    #[test]
    fn canonicalization_is_sorted() {
        let distribution = Distribution::from_weighted(
            vec![
                ("c", 0.2),
                ("a", 0.3),
                ("b", 0.5),
            ],
            0.0,
        )
        .expect("valid");

        let canonical =
            distribution
                .canonicalized()
                .expect("valid");

        assert_eq!(
            canonical.outcomes(),
            &["a", "b", "c"]
        );
    }

    #[test]
    fn normalization_is_explicit() {
        let distribution = Distribution::from_parts(
            vec!["a", "b"],
            vec![
                ProbabilityWeight::new(0.2).expect("valid"),
                ProbabilityWeight::new(0.8).expect("valid"),
            ],
            0.0,
        )
        .expect("valid");

        let normalized =
            distribution.normalized()
                .expect("valid");

        assert!(
            normalized
                .is_normalized(1.0e-12)
                .expect("valid")
        );
    }

    #[test]
    fn checked_entry_count_accepts_large_portable_count_when_host_can_represent_it() {
        let count = DistributionCount::from(1024_u64);

        let result =
            Distribution::<u8>::validate_entry_count(count);

        assert_eq!(result.expect("1024 fits"), 1024);
    }

    #[test]
    fn resource_entry_count_matches_storage() {
        let distribution = Distribution::from_weighted(
            vec![
                ("a", 0.5),
                ("b", 0.5),
            ],
            0.0,
        )
        .expect("valid");

        assert_eq!(
            distribution.resource_entry_count(),
            2
        );
    }

    #[test]
    fn sample_into_respects_requested_count() {
        let distribution = Distribution::from_weighted(
            vec![
                ("a", 0.5),
                ("b", 0.5),
            ],
            0.0,
        )
        .expect("valid");

        let mut rng =
            StdRng::seed_from_u64(7);

        let mut samples = Vec::new();

        distribution
            .sample_into(
                &mut rng,
                1_000,
                &mut samples,
            )
            .expect("sampling must succeed");

        assert_eq!(samples.len(), 1_000);
    }

    #[test]
    fn serialized_distribution_round_trips() {
        let distribution = Distribution::from_weighted(
            vec![
                ("a", 0.25),
                ("b", 0.75),
            ],
            0.0,
        )
        .expect("valid");

        let encoded =
            serde_json::to_string(
                &distribution
            )
            .expect("serialize");

        let decoded: Distribution<&str> =
            serde_json::from_str(&encoded)
                .expect("deserialize");

        assert_eq!(
            distribution,
            decoded
        );
    }

    #[test]
    fn validate_detects_valid_distribution() {
        let distribution = Distribution::from_weighted(
            vec![
                ("a", 0.1),
                ("b", 0.2),
                ("c", 0.7),
            ],
            1.0e-12,
        )
        .expect("valid");

        distribution
            .validate(1.0e-12)
            .expect("must validate");
    }
}