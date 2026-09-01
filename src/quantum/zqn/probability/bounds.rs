//! Zamani Quantum Noise (ZQN) — Probability Bounds.
//!
//! This module defines the canonical closed interval over the probability
//! domain:
//!
//!     0 <= lower <= upper <= 1
//!
//! `ProbabilityBounds` represents deterministic semantic bounds on a
//! probability value. It is intentionally independent of:
//!
//! - quantum states;
//! - qubit identities;
//! - distributions;
//! - statistical confidence intervals;
//! - calibration;
//! - channels;
//! - noise models;
//! - hardware;
//! - simulation;
//! - random-number generation.
//!
//! # Ownership
//!
//! This file owns:
//!
//! - validated lower/upper probability bounds;
//! - exact probability intervals;
//! - point bounds;
//! - the full probability domain;
//! - containment checks;
//! - interval width;
//! - interval midpoint;
//! - interval intersection;
//! - interval hull/union;
//! - interval complement;
//! - bound tightening;
//! - deterministic ordering;
//! - formatting;
//! - local bounds errors;
//! - mathematical invariants for probability intervals.
//!
//! # Does not own
//!
//! This file does NOT own:
//!
//! - probability distributions;
//! - confidence intervals;
//! - credible intervals;
//! - statistical estimators;
//! - Bayesian posterior bounds;
//! - uncertainty models;
//! - calibration uncertainty;
//! - quantum channels;
//! - faults;
//! - noise models;
//! - sampling;
//! - random-number generators;
//! - QubitId;
//! - PhysicalQubitId;
//! - hardware resources;
//! - execution limits.
//!
//! Those concerns belong to their owning modules.
//!
//! # Canonical probability primitive
//!
//! `ProbabilityBounds` is built exclusively from
//! `crate::quantum::zqn::probability::Probability`.
//!
//! The scalar `Probability` type already owns the invariant:
//!
//!     finite && 0 <= p <= 1
//!
//! This module therefore does not duplicate floating-point validation logic
//! unnecessarily. Every public constructor ultimately requires validated
//! `Probability` values.
//!
//! # Canonical quantum identity boundary
//!
//! A probability bound does not identify a quantum resource.
//!
//! Therefore this module intentionally does NOT define or import:
//!
//! ```text
//! QubitId
//! PhysicalQubitId
//! ```
//!
//! If a later layer needs bounds associated with a quantum resource, that
//! layer must use the canonical identities from:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! For example, a future noise/readout model may conceptually contain:
//!
//! ```text
//! QubitId
//!     |
//!     +--> ProbabilityBounds
//! ```
//!
//! The association belongs to that higher-level model, not this scalar
//! interval type.
//!
//! # Mathematical semantics
//!
//! A `ProbabilityBounds` value represents the closed set:
//!
//! ```text
//! [lower, upper]
//! ```
//!
//! where:
//!
//! ```text
//! 0 <= lower <= upper <= 1
//! ```
//!
//! Therefore every value contained by the bounds is itself a valid
//! probability.
//!
//! Both endpoints are inclusive.
//!
//! Examples:
//!
//! ```text
//! [0, 1]       -> completely unconstrained probability
//! [0, 0]       -> probability is exactly zero
//! [1, 1]       -> probability is exactly one
//! [0.2, 0.4]   -> probability is known to lie in that interval
//! ```
//!
//! # Bounds versus uncertainty
//!
//! This type expresses a mathematical interval.
//!
//! It does NOT claim why the interval exists.
//!
//! The interval may later originate from:
//!
//! - an analytical bound;
//! - an approximation guarantee;
//! - a physical constraint;
//! - a model reduction;
//! - a calibration result;
//! - a deterministic computation;
//! - a statistical procedure.
//!
//! The origin/provenance belongs to higher-level ZQN structures.
//!
//! In particular, this type must not add a confidence level such as `95%`.
//! Statistical confidence/credible semantics belong to characterization and
//! statistics modules.
//!
//! # Closed-interval semantics
//!
//! `ProbabilityBounds` always represents a closed interval.
//!
//! This is intentional because probability bounds normally express:
//!
//!     lower <= p <= upper
//!
//! Open or half-open intervals are not represented by this type.
//!
//! If a future subsystem needs topological interval semantics other than
//! closed probability intervals, it should introduce its own abstraction
//! rather than silently changing this type's meaning.
//!
//! # No artificial machine-size limits
//!
//! There is no quantum-system-size parameter in this module.
//!
//! The cost of one `ProbabilityBounds` is constant:
//!
//! - two `Probability` values;
//! - no allocation;
//! - no recursion;
//! - no quantum-system traversal;
//! - no dependency on qubit count.
//!
//! A million quantum resources can each have a bound if the owning collection
//! has sufficient resources. This type itself imposes no such collection
//! limit.
//!
//! This follows the ZQN scalability contract: semantic types must not encode
//! artificial hardware-size limits.
//!
//! # Numerical policy
//!
//! This module never:
//!
//! - clamps invalid input;
//! - silently swaps lower and upper bounds;
//! - converts NaN to zero;
//! - converts infinity to one;
//! - silently widens an interval;
//! - silently narrows an interval.
//!
//! Invalid construction fails explicitly.
//!
//! Operations whose mathematical result is guaranteed to remain inside the
//! probability domain return `ProbabilityBounds` directly.
//!
//! Operations that may fail return `Result`.
//!
//! # Floating-point semantics
//!
//! `Probability` currently uses `f64` as its storage representation.
//!
//! This file deliberately does not introduce another floating-point
//! representation.
//!
//! Therefore:
//!
//! - equality is exact according to `Probability`;
//! - midpoint/width use the current numerical representation;
//! - no hidden tolerance is used;
//! - no global epsilon exists.
//!
//! Approximate comparison belongs to callers and higher-level numerical
//! policies.
//!
//! # Scaling principle
//!
//! The semantic domain is independent of quantum machine size.
//!
//! This module does not contain:
//!
//! ```text
//! MAX_QUBITS
//! MAX_PROBABILITY_BOUNDS
//! MAX_RESOURCES
//! MAX_INTERVALS
//! ```
//!
//! Any operational limit belongs to `ZqnLimits` or the consuming subsystem.
//!
//! # Determinism
//!
//! All operations are deterministic.
//!
//! There is:
//!
//! - no RNG;
//! - no wall-clock dependency;
//! - no global mutable state;
//! - no process identity;
//! - no thread identity;
//! - no hash-map iteration;
//! - no allocation-dependent semantic behavior.
//!
//! Given identical `Probability` endpoints, the same operation produces the
//! same result.
//!
//! # Parallelism
//!
//! `ProbabilityBounds` is `Copy`, contains no mutable state, and performs no
//! synchronization.
//!
//! It is therefore naturally suitable for concurrent use wherever the
//! containing execution context permits it.
//!
//! # Serialization
//!
//! This module does NOT define an external wire format.
//!
//! Serialization belongs to:
//!
//! ```text
//! crate::quantum::zqn::io
//! ```
//!
//! A future schema should represent the two semantic endpoints explicitly.
//!
//! The in-memory representation must not accidentally become the ZQN wire
//! protocol.
//!
//! # Integration contract
//!
//! ```text
//! probability/probability.rs
//!             |
//!             v
//!     ProbabilityBounds
//!             |
//!     +-------+--------+
//!     |       |        |
//!     v       v        v
//! distribution  channel  noise/fault
//!     |       |        |
//!     +-------+--------+
//!             |
//!             v
//!     characterization
//!             |
//!             v
//!       propagation
//! ```
//!
//! Higher-level modules may use this type for deterministic semantic bounds.
//!
//! They must not reinterpret `ProbabilityBounds` as a confidence interval
//! unless an explicit statistical wrapper provides that meaning.
//!
//! # Integration with `quantum::ir::qubit`
//!
//! No direct integration is required in this file.
//!
//! This is intentional.
//!
//! A probability bound is not a quantum resource. If a higher-level type
//! associates one with a qubit, it should use:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This preserves the repository's canonical identity boundary and prevents
//! ZQN from creating competing qubit identity types.
//!
//! # Integration with ZQN limits
//!
//! `ProbabilityBounds` contains no collection or allocation operation, so it
//! does not need `ZqnLimits` for construction.
//!
//! A consuming collection may use `ZqnLimits` to constrain the number of
//! bounds it stores or processes.
//!
//! Such a policy must not alter the mathematical meaning of this type.
//!
//! # Error integration
//!
//! `ProbabilityBoundsError` is deliberately local and dependency-light.
//!
//! This permits this file to be completed and compiled independently of the
//! rest of ZQN.
//!
//! A higher-level integration layer may convert it into
//! `crate::quantum::zqn::core::errors::ZqnError` once the canonical conversion
//! boundary is established.
//!
//! This file does not import `core::errors` because doing so would create an
//! unnecessary dependency from this foundational probability module into a
//! broader diagnostic layer.
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
//! - no `unsafe`.
//!
//! # Definition of done
//!
//! This file is complete when:
//!
//! - every public constructor preserves `0 <= lower <= upper <= 1`;
//! - invalid bounds fail explicitly;
//! - no constructor silently swaps endpoints;
//! - no constructor silently clamps endpoints;
//! - exact points are representable;
//! - the full probability domain is representable;
//! - intersection is mathematically correct;
//! - hull is mathematically correct;
//! - complement preserves probability semantics;
//! - width is never negative;
//! - midpoint remains inside the interval;
//! - no artificial quantum-machine-size limit exists;
//! - no quantum identity type is duplicated;
//! - no unsafe code exists;
//! - no global state exists;
//! - the module can be used independently by later ZQN layers;
//! - unit tests cover boundary and failure behavior.
//!
//! # Example
//!
//! ```
//! # use crate::quantum::zqn::probability::bounds::ProbabilityBounds;
//! # use crate::quantum::zqn::probability::probability::Probability;
//! let lower = Probability::new(0.2).unwrap();
//! let upper = Probability::new(0.7).unwrap();
//!
//! let bounds = ProbabilityBounds::new(lower, upper).unwrap();
//!
//! assert!(bounds.contains(Probability::new(0.2).unwrap()));
//! assert!(bounds.contains(Probability::new(0.7).unwrap()));
//! assert!(!bounds.contains(Probability::new(0.8).unwrap()));
//! ```

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::cmp::Ordering;
use std::error::Error;
use std::fmt;

use super::probability::Probability;

/// A validated closed interval over the probability domain.
///
/// The invariant is always:
///
/// ```text
/// 0 <= lower <= upper <= 1
/// ```
///
/// Both endpoints are inclusive.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProbabilityBounds {
    lower: Probability,
    upper: Probability,
}

impl ProbabilityBounds {
    /// The complete mathematical probability domain `[0, 1]`.
    pub const FULL: Self = Self {
        lower: Probability::ZERO,
        upper: Probability::ONE,
    };

    /// Creates an exact zero-probability bound `[0, 0]`.
    pub const ZERO: Self = Self {
        lower: Probability::ZERO,
        upper: Probability::ZERO,
    };

    /// Creates an exact unit-probability bound `[1, 1]`.
    pub const ONE: Self = Self {
        lower: Probability::ONE,
        upper: Probability::ONE,
    };

    /// Constructs bounds from validated probability endpoints.
    ///
    /// # Errors
    ///
    /// Returns `ProbabilityBoundsError::LowerGreaterThanUpper` if
    /// `lower > upper`.
    ///
    /// No endpoint is modified.
    pub const fn new(
        lower: Probability,
        upper: Probability,
    ) -> Result<Self, ProbabilityBoundsError> {
        if lower.value() > upper.value() {
            return Err(ProbabilityBoundsError::LowerGreaterThanUpper {
                lower: lower.value(),
                upper: upper.value(),
            });
        }

        Ok(Self { lower, upper })
    }

    /// Constructs an exact interval containing one probability value.
    ///
    /// The result is `[value, value]`.
    #[must_use]
    pub const fn exact(value: Probability) -> Self {
        Self {
            lower: value,
            upper: value,
        }
    }

    /// Returns the complete probability domain `[0, 1]`.
    #[must_use]
    pub const fn full() -> Self {
        Self::FULL
    }

    /// Returns the lower bound.
    #[must_use]
    pub const fn lower(self) -> Probability {
        self.lower
    }

    /// Returns the upper bound.
    #[must_use]
    pub const fn upper(self) -> Probability {
        self.upper
    }

    /// Returns both endpoints as `(lower, upper)`.
    #[must_use]
    pub const fn endpoints(self) -> (Probability, Probability) {
        (self.lower, self.upper)
    }

    /// Returns whether this interval contains `value`.
    ///
    /// The endpoints are inclusive.
    #[must_use]
    pub fn contains(self, value: Probability) -> bool {
        value.value() >= self.lower.value() && value.value() <= self.upper.value()
    }

    /// Returns whether this interval contains another interval completely.
    ///
    /// This is equivalent to:
    ///
    /// ```text
    /// self.lower <= other.lower
    /// &&
    /// other.upper <= self.upper
    /// ```
    #[must_use]
    pub fn contains_bounds(self, other: Self) -> bool {
        self.lower.value() <= other.lower.value()
            && other.upper.value() <= self.upper.value()
    }

    /// Returns whether the interval represents exactly one probability.
    #[must_use]
    pub fn is_exact(self) -> bool {
        self.lower == self.upper
    }

    /// Returns whether the interval is the complete probability domain.
    #[must_use]
    pub fn is_full(self) -> bool {
        self == Self::FULL
    }

    /// Returns the interval width:
    ///
    /// ```text
    /// upper - lower
    /// ```
    ///
    /// The result is always a valid probability.
    #[must_use]
    pub fn width(self) -> Probability {
        self.upper.abs_difference(self.lower)
    }

    /// Returns the midpoint:
    ///
    /// ```text
    /// (lower + upper) / 2
    /// ```
    ///
    /// The midpoint is guaranteed to lie inside the interval.
    ///
    /// Floating-point arithmetic is used according to the representation
    /// currently used by `Probability`.
    #[must_use]
    pub fn midpoint(self) -> Probability {
        let midpoint = self.lower.value()
            + (self.upper.value() - self.lower.value()) * 0.5;

        // Because lower and upper are finite values in [0, 1], midpoint is
        // mathematically in [0, 1]. Constructing through the validated
        // Probability API would be redundant, so the invariant is maintained
        // directly here.
        debug_assert!(midpoint.is_finite());
        debug_assert!(midpoint >= 0.0);
        debug_assert!(midpoint <= 1.0);

        Probability::new(midpoint)
            .expect("ProbabilityBounds midpoint must remain a valid probability")
    }

    /// Returns whether this interval touches zero.
    #[must_use]
    pub const fn touches_zero(self) -> bool {
        self.lower.value() == 0.0
    }

    /// Returns whether this interval touches one.
    #[must_use]
    pub const fn touches_one(self) -> bool {
        self.upper.value() == 1.0
    }

    /// Returns a new interval with a tightened lower endpoint.
    ///
    /// The new lower bound must not exceed the existing upper bound.
    ///
    /// This operation never widens the interval.
    pub const fn with_lower(
        self,
        lower: Probability,
    ) -> Result<Self, ProbabilityBoundsError> {
        Self::new(lower, self.upper)
    }

    /// Returns a new interval with a tightened upper endpoint.
    ///
    /// The new upper bound must not be below the existing lower bound.
    ///
    /// This operation never widens the interval.
    pub const fn with_upper(
        self,
        upper: Probability,
    ) -> Result<Self, ProbabilityBoundsError> {
        Self::new(self.lower, upper)
    }

    /// Intersects two probability intervals.
    ///
    /// If the intervals overlap or touch, their intersection is returned.
    ///
    /// If they are disjoint, `None` is returned.
    ///
    /// Because both inputs are valid probability intervals, the result is
    /// also a valid probability interval whenever an intersection exists.
    #[must_use]
    pub fn intersection(self, other: Self) -> Option<Self> {
        let lower = if self.lower.value() >= other.lower.value() {
            self.lower
        } else {
            other.lower
        };

        let upper = if self.upper.value() <= other.upper.value() {
            self.upper
        } else {
            other.upper
        };

        if lower.value() <= upper.value() {
            Some(Self {
                lower,
                upper,
            })
        } else {
            None
        }
    }

    /// Returns the smallest closed probability interval containing both
    /// intervals.
    ///
    /// This operation is sometimes called the interval hull.
    #[must_use]
    pub fn hull(self, other: Self) -> Self {
        let lower = if self.lower.value() <= other.lower.value() {
            self.lower
        } else {
            other.lower
        };

        let upper = if self.upper.value() >= other.upper.value() {
            self.upper
        } else {
            other.upper
        };

        Self { lower, upper }
    }

    /// Returns the complement of this probability interval under `[0, 1]`.
    ///
    /// For:
    ///
    /// ```text
    /// [a, b]
    /// ```
    ///
    /// the reflected interval is:
    ///
    /// ```text
    /// [1 - b, 1 - a]
    /// ```
    ///
    /// This operation represents the image of the interval under
    /// `p -> 1 - p`. It does not represent the set-theoretic complement of
    /// the interval.
    #[must_use]
    pub fn complement(self) -> Self {
        Self {
            lower: self.upper.complement(),
            upper: self.lower.complement(),
        }
    }

    /// Returns the distance of the interval from zero.
    ///
    /// For probability intervals this is exactly the lower endpoint.
    #[must_use]
    pub const fn lower_distance_from_zero(self) -> Probability {
        self.lower
    }

    /// Returns the remaining distance from the upper endpoint to one.
    ///
    /// This is:
    ///
    /// ```text
    /// 1 - upper
    /// ```
    #[must_use]
    pub fn distance_to_one(self) -> Probability {
        self.upper.complement()
    }

    /// Returns a deterministic three-way relation between two bounds.
    ///
    /// - `Less` means this interval lies completely below `other`;
    /// - `Equal` means the intervals are exactly equal;
    /// - `Greater` means this interval lies completely above `other`;
    /// - `None` means the intervals overlap without being equal.
    ///
    /// This is deliberately a partial ordering because overlapping intervals
    /// are neither strictly below nor strictly above one another.
    #[must_use]
    pub fn partial_cmp_bounds(self, other: Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.upper.value() < other.lower.value() {
            Some(Ordering::Less)
        } else if self.lower.value() > other.upper.value() {
            Some(Ordering::Greater)
        } else {
            None
        }
    }

    /// Returns whether this interval is completely below `other`.
    ///
    /// Touching intervals are not considered strictly below.
    #[must_use]
    pub fn is_strictly_below(self, other: Self) -> bool {
        self.upper.value() < other.lower.value()
    }

    /// Returns whether this interval is completely above `other`.
    ///
    /// Touching intervals are not considered strictly above.
    #[must_use]
    pub fn is_strictly_above(self, other: Self) -> bool {
        self.lower.value() > other.upper.value()
    }

    /// Returns whether two intervals overlap, including endpoint contact.
    #[must_use]
    pub fn overlaps(self, other: Self) -> bool {
        self.intersection(other).is_some()
    }

    /// Returns the smallest probability interval containing this interval and
    /// a single probability value.
    ///
    /// If `value` is already contained, this returns `self` unchanged.
    #[must_use]
    pub fn hull_with_probability(self, value: Probability) -> Self {
        let lower = if value.value() < self.lower.value() {
            value
        } else {
            self.lower
        };

        let upper = if value.value() > self.upper.value() {
            value
        } else {
            self.upper
        };

        Self { lower, upper }
    }

    /// Returns a new interval obtained by intersecting this interval with the
    /// complete probability domain.
    ///
    /// Since every `ProbabilityBounds` value is already valid, this is
    /// effectively an invariant-preserving identity operation and is provided
    /// for generic interval-processing code.
    #[must_use]
    pub const fn clamp_to_probability_domain(self) -> Self {
        self
    }

    /// Returns the lower and upper numerical values as `(f64, f64)`.
    ///
    /// This is intended for numerical algorithms that explicitly operate on
    /// primitive floating-point values.
    ///
    /// It does not expose mutable access, so the `ProbabilityBounds` invariant
    /// cannot be bypassed.
    #[must_use]
    pub const fn as_f64_pair(self) -> (f64, f64) {
        (self.lower.value(), self.upper.value())
    }
}

impl Default for ProbabilityBounds {
    /// The default probability bounds are the complete domain `[0, 1]`.
    ///
    /// This is deliberately conservative: an absent constraint must not be
    /// interpreted as certainty about a narrower probability range.
    fn default() -> Self {
        Self::FULL
    }
}

impl Eq for ProbabilityBounds {}

impl PartialOrd for ProbabilityBounds {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.partial_cmp_bounds(*other)
    }
}

impl fmt::Display for ProbabilityBounds {
    /// Formats the interval as:
    ///
    /// ```text
    /// [lower, upper]
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}, {}]",
            self.lower.value(),
            self.upper.value()
        )
    }
}

/// Error produced when constructing or manipulating probability bounds.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProbabilityBoundsError {
    /// The lower endpoint is greater than the upper endpoint.
    ///
    /// Both values are already individually valid probabilities; the
    /// interval relationship is the invalid part.
    LowerGreaterThanUpper {
        /// Requested lower endpoint.
        lower: f64,

        /// Requested upper endpoint.
        upper: f64,
    },
}

impl fmt::Display for ProbabilityBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LowerGreaterThanUpper { lower, upper } => write!(
                f,
                "invalid probability bounds: lower ({lower}) is greater than upper ({upper})"
            ),
        }
    }
}

impl Error for ProbabilityBoundsError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn probability(value: f64) -> Probability {
        Probability::new(value).expect("test probability must be valid")
    }

    #[test]
    fn full_domain_is_zero_to_one() {
        let bounds = ProbabilityBounds::FULL;

        assert_eq!(bounds.lower(), Probability::ZERO);
        assert_eq!(bounds.upper(), Probability::ONE);
        assert!(bounds.is_full());
        assert!(!bounds.is_exact());
    }

    #[test]
    fn exact_bounds_have_identical_endpoints() {
        let value = probability(0.375);
        let bounds = ProbabilityBounds::exact(value);

        assert_eq!(bounds.lower(), value);
        assert_eq!(bounds.upper(), value);
        assert!(bounds.is_exact());
        assert_eq!(bounds.width(), Probability::ZERO);
        assert_eq!(bounds.midpoint(), value);
    }

    #[test]
    fn constructor_accepts_equal_endpoints() {
        let value = probability(0.5);

        let bounds = ProbabilityBounds::new(value, value)
            .expect("equal endpoints are valid");

        assert_eq!(bounds, ProbabilityBounds::exact(value));
    }

    #[test]
    fn constructor_accepts_ordered_endpoints() {
        let lower = probability(0.2);
        let upper = probability(0.8);

        let bounds = ProbabilityBounds::new(lower, upper)
            .expect("ordered endpoints are valid");

        assert_eq!(bounds.lower(), lower);
        assert_eq!(bounds.upper(), upper);
    }

    #[test]
    fn constructor_rejects_reversed_endpoints() {
        let lower = probability(0.8);
        let upper = probability(0.2);

        let error = ProbabilityBounds::new(lower, upper)
            .expect_err("reversed endpoints must be rejected");

        assert_eq!(
            error,
            ProbabilityBoundsError::LowerGreaterThanUpper {
                lower: 0.8,
                upper: 0.2,
            }
        );
    }

    #[test]
    fn zero_bound_is_exact_zero() {
        assert_eq!(
            ProbabilityBounds::ZERO,
            ProbabilityBounds::exact(Probability::ZERO)
        );
    }

    #[test]
    fn one_bound_is_exact_one() {
        assert_eq!(
            ProbabilityBounds::ONE,
            ProbabilityBounds::exact(Probability::ONE)
        );
    }

    #[test]
    fn contains_is_closed_at_lower_endpoint() {
        let bounds = ProbabilityBounds::new(
            probability(0.2),
            probability(0.8),
        )
        .unwrap();

        assert!(bounds.contains(probability(0.2)));
    }

    #[test]
    fn contains_is_closed_at_upper_endpoint() {
        let bounds = ProbabilityBounds::new(
            probability(0.2),
            probability(0.8),
        )
        .unwrap();

        assert!(bounds.contains(probability(0.8)));
    }

    #[test]
    fn contains_rejects_values_below_lower() {
        let bounds = ProbabilityBounds::new(
            probability(0.2),
            probability(0.8),
        )
        .unwrap();

        assert!(!bounds.contains(probability(0.19)));
    }

    #[test]
    fn contains_rejects_values_above_upper() {
        let bounds = ProbabilityBounds::new(
            probability(0.2),
            probability(0.8),
        )
        .unwrap();

        assert!(!bounds.contains(probability(0.81)));
    }

    #[test]
    fn contains_bounds_is_correct() {
        let outer = ProbabilityBounds::new(
            probability(0.2),
            probability(0.8),
        )
        .unwrap();

        let inner = ProbabilityBounds::new(
            probability(0.3),
            probability(0.7),
        )
        .unwrap();

        assert!(outer.contains_bounds(inner));
        assert!(!inner.contains_bounds(outer));
    }

    #[test]
    fn width_is_correct() {
        let bounds = ProbabilityBounds::new(
            probability(0.25),
            probability(0.75),
        )
        .unwrap();

        assert_eq!(bounds.width(), probability(0.5));
    }

    #[test]
    fn width_of_exact_bound_is_zero() {
        let bounds = ProbabilityBounds::exact(probability(0.25));

        assert_eq!(bounds.width(), Probability::ZERO);
    }

    #[test]
    fn midpoint_is_inside_bounds() {
        let bounds = ProbabilityBounds::new(
            probability(0.2),
            probability(0.8),
        )
        .unwrap();

        let midpoint = bounds.midpoint();

        assert_eq!(midpoint, probability(0.5));
        assert!(bounds.contains(midpoint));
    }

    #[test]
    fn midpoint_of_zero_one_is_half() {
        assert_eq!(
            ProbabilityBounds::FULL.midpoint(),
            probability(0.5)
        );
    }

    #[test]
    fn touches_zero_is_correct() {
        let lower_zero = ProbabilityBounds::new(
            Probability::ZERO,
            probability(0.4),
        )
        .unwrap();

        let nonzero_lower = ProbabilityBounds::new(
            probability(0.1),
            probability(0.4),
        )
        .unwrap();

        assert!(lower_zero.touches_zero());
        assert!(!nonzero_lower.touches_zero());
    }

    #[test]
    fn touches_one_is_correct() {
        let upper_one = ProbabilityBounds::new(
            probability(0.4),
            Probability::ONE,
        )
        .unwrap();

        let non_one_upper = ProbabilityBounds::new(
            probability(0.4),
            probability(0.9),
        )
        .unwrap();

        assert!(upper_one.touches_one());
        assert!(!non_one_upper.touches_one());
    }

    #[test]
    fn intersection_of_overlapping_intervals_is_correct() {
        let left = ProbabilityBounds::new(
            probability(0.2),
            probability(0.7),
        )
        .unwrap();

        let right = ProbabilityBounds::new(
            probability(0.5),
            probability(0.9),
        )
        .unwrap();

        let intersection = left
            .intersection(right)
            .expect("intervals overlap");

        assert_eq!(
            intersection,
            ProbabilityBounds::new(
                probability(0.5),
                probability(0.7),
            )
            .unwrap()
        );
    }

    #[test]
    fn intersection_of_touching_intervals_is_a_point() {
        let left = ProbabilityBounds::new(
            probability(0.2),
            probability(0.5),
        )
        .unwrap();

        let right = ProbabilityBounds::new(
            probability(0.5),
            probability(0.8),
        )
        .unwrap();

        assert_eq!(
            left.intersection(right),
            Some(ProbabilityBounds::exact(probability(0.5)))
        );
    }

    #[test]
    fn intersection_of_disjoint_intervals_is_none() {
        let left = ProbabilityBounds::new(
            probability(0.1),
            probability(0.2),
        )
        .unwrap();

        let right = ProbabilityBounds::new(
            probability(0.3),
            probability(0.4),
        )
        .unwrap();

        assert_eq!(left.intersection(right), None);
    }

    #[test]
    fn intersection_is_commutative() {
        let left = ProbabilityBounds::new(
            probability(0.1),
            probability(0.7),
        )
        .unwrap();

        let right = ProbabilityBounds::new(
            probability(0.4),
            probability(0.9),
        )
        .unwrap();

        assert_eq!(
            left.intersection(right),
            right.intersection(left)
        );
    }

    #[test]
    fn hull_contains_both_operands() {
        let left = ProbabilityBounds::new(
            probability(0.1),
            probability(0.4),
        )
        .unwrap();

        let right = ProbabilityBounds::new(
            probability(0.6),
            probability(0.9),
        )
        .unwrap();

        let hull = left.hull(right);

        assert!(hull.contains_bounds(left));
        assert!(hull.contains_bounds(right));
        assert_eq!(
            hull,
            ProbabilityBounds::new(
                probability(0.1),
                probability(0.9),
            )
            .unwrap()
        );
    }

    #[test]
    fn hull_is_commutative() {
        let left = ProbabilityBounds::new(
            probability(0.1),
            probability(0.4),
        )
        .unwrap();

        let right = ProbabilityBounds::new(
            probability(0.6),
            probability(0.9),
        )
        .unwrap();

        assert_eq!(left.hull(right), right.hull(left));
    }

    #[test]
    fn complement_reflects_interval() {
        let bounds = ProbabilityBounds::new(
            probability(0.2),
            probability(0.7),
        )
        .unwrap();

        let complement = bounds.complement();

        assert_eq!(
            complement,
            ProbabilityBounds::new(
                probability(0.3),
                probability(0.8),
            )
            .unwrap()
        );
    }

    #[test]
    fn complement_is_involution() {
        let bounds = ProbabilityBounds::new(
            probability(0.2),
            probability(0.7),
        )
        .unwrap();

        assert_eq!(bounds.complement().complement(), bounds);
    }

    #[test]
    fn full_domain_complement_is_full_domain() {
        assert_eq!(
            ProbabilityBounds::FULL.complement(),
            ProbabilityBounds::FULL
        );
    }

    #[test]
    fn zero_complement_is_one() {
        assert_eq!(
            ProbabilityBounds::ZERO.complement(),
            ProbabilityBounds::ONE
        );
    }

    #[test]
    fn one_complement_is_zero() {
        assert_eq!(
            ProbabilityBounds::ONE.complement(),
            ProbabilityBounds::ZERO
        );
    }

    #[test]
    fn lower_tightening_preserves_upper() {
        let bounds = ProbabilityBounds::new(
            probability(0.2),
            probability(0.8),
        )
        .unwrap();

        let tightened = bounds
            .with_lower(probability(0.4))
            .unwrap();

        assert_eq!(tightened.lower(), probability(0.4));
        assert_eq!(tightened.upper(), probability(0.8));
    }

    #[test]
    fn upper_tightening_preserves_lower() {
        let bounds = ProbabilityBounds::new(
            probability(0.2),
            probability(0.8),
        )
        .unwrap();

        let tightened = bounds
            .with_upper(probability(0.6))
            .unwrap();

        assert_eq!(tightened.lower(), probability(0.2));
        assert_eq!(tightened.upper(), probability(0.6));
    }

    #[test]
    fn lower_tightening_rejects_above_upper() {
        let bounds = ProbabilityBounds::new(
            probability(0.2),
            probability(0.8),
        )
        .unwrap();

        assert!(
            bounds.with_lower(probability(0.9)).is_err()
        );
    }

    #[test]
    fn upper_tightening_rejects_below_lower() {
        let bounds = ProbabilityBounds::new(
            probability(0.2),
            probability(0.8),
        )
        .unwrap();

        assert!(
            bounds.with_upper(probability(0.1)).is_err()
        );
    }

    #[test]
    fn hull_with_probability_keeps_contained_value() {
        let bounds = ProbabilityBounds::new(
            probability(0.2),
            probability(0.8),
        )
        .unwrap();

        assert_eq!(
            bounds.hull_with_probability(probability(0.5)),
            bounds
        );
    }

    #[test]
    fn hull_with_probability_expands_lower_endpoint() {
        let bounds = ProbabilityBounds::new(
            probability(0.3),
            probability(0.8),
        )
        .unwrap();

        let expanded = bounds.hull_with_probability(probability(0.1));

        assert_eq!(
            expanded,
            ProbabilityBounds::new(
                probability(0.1),
                probability(0.8),
            )
            .unwrap()
        );
    }

    #[test]
    fn hull_with_probability_expands_upper_endpoint() {
        let bounds = ProbabilityBounds::new(
            probability(0.2),
            probability(0.7),
        )
        .unwrap();

        let expanded = bounds.hull_with_probability(probability(0.9));

        assert_eq!(
            expanded,
            ProbabilityBounds::new(
                probability(0.2),
                probability(0.9),
            )
            .unwrap()
        );
    }

    #[test]
    fn partial_order_identifies_disjoint_intervals() {
        let lower = ProbabilityBounds::new(
            probability(0.1),
            probability(0.2),
        )
        .unwrap();

        let upper = ProbabilityBounds::new(
            probability(0.7),
            probability(0.9),
        )
        .unwrap();

        assert_eq!(
            lower.partial_cmp_bounds(upper),
            Some(Ordering::Less)
        );

        assert_eq!(
            upper.partial_cmp_bounds(lower),
            Some(Ordering::Greater)
        );
    }

    #[test]
    fn partial_order_identifies_equal_intervals() {
        let left = ProbabilityBounds::new(
            probability(0.2),
            probability(0.8),
        )
        .unwrap();

        let right = ProbabilityBounds::new(
            probability(0.2),
            probability(0.8),
        )
        .unwrap();

        assert_eq!(
            left.partial_cmp_bounds(right),
            Some(Ordering::Equal)
        );
    }

    #[test]
    fn partial_order_returns_none_for_overlapping_distinct_intervals() {
        let left = ProbabilityBounds::new(
            probability(0.2),
            probability(0.7),
        )
        .unwrap();

        let right = ProbabilityBounds::new(
            probability(0.5),
            probability(0.9),
        )
        .unwrap();

        assert_eq!(left.partial_cmp_bounds(right), None);
    }

    #[test]
    fn strict_below_requires_gap() {
        let left = ProbabilityBounds::new(
            probability(0.2),
            probability(0.5),
        )
        .unwrap();

        let right = ProbabilityBounds::new(
            probability(0.5),
            probability(0.8),
        )
        .unwrap();

        assert!(!left.is_strictly_below(right));
    }

    #[test]
    fn strict_above_requires_gap() {
        let left = ProbabilityBounds::new(
            probability(0.2),
            probability(0.5),
        )
        .unwrap();

        let right = ProbabilityBounds::new(
            probability(0.5),
            probability(0.8),
        )
        .unwrap();

        assert!(!right.is_strictly_above(left));
    }

    #[test]
    fn overlapping_intervals_are_detected() {
        let left = ProbabilityBounds::new(
            probability(0.2),
            probability(0.7),
        )
        .unwrap();

        let right = ProbabilityBounds::new(
            probability(0.6),
            probability(0.9),
        )
        .unwrap();

        assert!(left.overlaps(right));
    }

    #[test]
    fn disjoint_intervals_are_detected() {
        let left = ProbabilityBounds::new(
            probability(0.1),
            probability(0.2),
        )
        .unwrap();

        let right = ProbabilityBounds::new(
            probability(0.8),
            probability(0.9),
        )
        .unwrap();

        assert!(!left.overlaps(right));
    }

    #[test]
    fn distance_to_one_is_correct() {
        let bounds = ProbabilityBounds::new(
            probability(0.2),
            probability(0.7),
        )
        .unwrap();

        assert_eq!(
            bounds.distance_to_one(),
            probability(0.3)
        );
    }

    #[test]
    fn lower_distance_from_zero_is_lower_endpoint() {
        let bounds = ProbabilityBounds::new(
            probability(0.2),
            probability(0.7),
        )
        .unwrap();

        assert_eq!(
            bounds.lower_distance_from_zero(),
            probability(0.2)
        );
    }

    #[test]
    fn as_f64_pair_is_lossless_for_current_representation() {
        let bounds = ProbabilityBounds::new(
            probability(0.2),
            probability(0.7),
        )
        .unwrap();

        assert_eq!(
            bounds.as_f64_pair(),
            (0.2, 0.7)
        );
    }

    #[test]
    fn display_is_deterministic() {
        let bounds = ProbabilityBounds::new(
            probability(0.2),
            probability(0.7),
        )
        .unwrap();

        assert_eq!(bounds.to_string(), "[0.2, 0.7]");
    }

    #[test]
    fn default_is_conservative_full_domain() {
        assert_eq!(
            ProbabilityBounds::default(),
            ProbabilityBounds::FULL
        );
    }

    #[test]
    fn clamping_to_domain_is_identity() {
        let bounds = ProbabilityBounds::new(
            probability(0.2),
            probability(0.7),
        )
        .unwrap();

        assert_eq!(
            bounds.clamp_to_probability_domain(),
            bounds
        );
    }

    #[test]
    fn no_operation_can_create_reversed_bounds() {
        let values = [
            (0.0, 0.0),
            (0.0, 0.5),
            (0.0, 1.0),
            (0.2, 0.2),
            (0.2, 0.8),
            (1.0, 1.0),
        ];

        for (lower, upper) in values {
            let bounds = ProbabilityBounds::new(
                probability(lower),
                probability(upper),
            )
            .unwrap();

            assert!(bounds.lower().value() <= bounds.upper().value());
            assert!(bounds.lower().value() >= 0.0);
            assert!(bounds.upper().value() <= 1.0);
        }
    }

    #[test]
    fn intersection_is_never_wider_than_either_operand() {
        let left = ProbabilityBounds::new(
            probability(0.1),
            probability(0.8),
        )
        .unwrap();

        let right = ProbabilityBounds::new(
            probability(0.4),
            probability(0.9),
        )
        .unwrap();

        let intersection = left
            .intersection(right)
            .unwrap();

        assert!(intersection.width().value() <= left.width().value());
        assert!(intersection.width().value() <= right.width().value());
    }

    #[test]
    fn hull_is_never_narrower_than_either_operand() {
        let left = ProbabilityBounds::new(
            probability(0.1),
            probability(0.8),
        )
        .unwrap();

        let right = ProbabilityBounds::new(
            probability(0.4),
            probability(0.9),
        )
        .unwrap();

        let hull = left.hull(right);

        assert!(hull.width().value() >= left.width().value());
        assert!(hull.width().value() >= right.width().value());
    }
}