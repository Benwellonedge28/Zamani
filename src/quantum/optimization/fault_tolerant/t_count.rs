//! Zamani Quantum Optimization — Fault-Tolerant T-Count Analysis
//!
//! Production-grade T-resource accounting for logical Clifford+T quantum
//! circuits.
//!
//! # Architectural role
//!
//! This module owns the semantic accounting of T-family resources:
//!
//! ```text
//!                         quantum::ir
//!                              │
//!                              ▼
//!                fault_tolerant::t_count
//!                              │
//!          ┌───────────────────┼───────────────────┐
//!          ▼                   ▼                   ▼
//!       cost.rs          phase_polynomial      reports
//!          │                   │                   │
//!          ▼                   ▼                   ▼
//!      objectives        transformations       metrics
//! ```
//!
//! The module deliberately separates:
//!
//! - measuring T resources;
//! - comparing T-resource costs;
//! - calculating exact T-count deltas;
//! - deriving conservative lower bounds;
//! - classifying T/Tdg operations;
//! - preserving deterministic accounting;
//!
//! from:
//!
//! - phase-polynomial synthesis;
//! - TODD/Reed-Muller optimization;
//! - circuit rewriting;
//! - Clifford synthesis;
//! - routing;
//! - scheduling;
//! - hardware execution.
//!
//! Those transformations belong to other optimization modules.
//!
//! # Canonical IR
//!
//! The only circuit representation accepted here is:
//!
//! `crate::quantum::ir::QuantumCircuit`
//!
//! This file does NOT define:
//!
//! - QuantumGate;
//! - QuantumOperation;
//! - QuantumCircuit;
//! - a private gate enum;
//! - a second qubit representation.
//!
//! T and T-dagger classification is performed using the canonical:
//!
//! `crate::quantum::ir::gate::GateKind`
//!
//! # Why T-count is its own resource
//!
//! Ordinary gate count is not an adequate fault-tolerant resource metric.
//!
//! A circuit with fewer total gates can still be substantially more expensive
//! if it contains more non-Clifford gates.
//!
//! In fault-tolerant Clifford+T computation, T operations normally require
//! expensive non-Clifford resources such as magic states or equivalent logical
//! resources. Therefore T-count must remain independently measurable.
//!
//! The optimization subsystem already models `TCount` and `TDepth` as separate
//! cost dimensions. This module owns the exact structural T-count metric;
//! `cost.rs` owns multi-objective comparison and target-specific cost policy.
//!
//! # Exact semantics
//!
//! For a canonical circuit:
//!
//! ```text
//! T-count = number of GateKind::T operations
//!         + number of GateKind::Tdg operations
//! ```
//!
//! More specifically:
//!
//! ```text
//! t_count        = number of T
//! t_dagger_count = number of Tdg
//! t_family_count = t_count + t_dagger_count
//! ```
//!
//! No phase cancellation is inferred merely from these aggregate counts.
//!
//! For example:
//!
//! ```text
//! T q0
//! Tdg q0
//! ```
//!
//! has T-count 2 at the raw circuit level even though a separate rewrite pass
//! may legally reduce the pair to identity.
//!
//! This distinction is critical:
//!
//! ```text
//! analysis  != transformation
//! ```
//!
//! # T-family exponent model
//!
//! The Clifford+T phase subgroup can be represented modulo eight:
//!
//! ```text
//! T^0 = I
//! T^1 = T
//! T^2 = S
//! T^3 = S T
//! T^4 = Z
//! T^5 = Z T
//! T^6 = Sdg
//! T^7 = Sdg T
//! T^8 = I
//! ```
//!
//! This module therefore exposes a small, exact `TExponent` utility for
//! reasoning about signed T-family powers.
//!
//! It does NOT rewrite circuits.
//!
//! A transformation pass can use this type when implementing:
//!
//! - local T-run reduction;
//! - phase-polynomial optimization;
//! - Clifford+T normalization;
//! - T-count-aware synthesis.
//!
//! # Global phase
//!
//! This module never silently discards global phase.
//!
//! Aggregate T-count does not itself imply a semantic equivalence relation.
//! A transformation that changes a circuit up to global phase must be validated
//! against the optimization subsystem's configured equivalence policy.
//!
//! In particular, this file does not claim that every T-count reduction is
//! valid under exact unitary equivalence.
//!
//! # Scaling
//!
//! The analysis delegates circuit traversal to the canonical gate-count
//! analysis:
//!
//! `optimization::analysis::gate_counts`
//!
//! Therefore the complexity is:
//!
//! ```text
//! Time:   O(N + A + P)
//! Memory: O(1) additional memory
//! ```
//!
//! where:
//!
//! - N = number of canonical operations;
//! - A = total qubit operands;
//! - P = total gate parameters.
//!
//! The returned T-count result is a fixed-size value.
//!
//! No per-gate heap allocation is performed by this module.
//!
//! This permits circuits ranging from tiny examples to the largest circuits
//! representable by the canonical IR and host resources, subject to explicit
//! IR/optimizer resource limits.
//!
//! # Integer width
//!
//! Structural T-resource counters are represented as `u128` in this module.
//!
//! The canonical circuit currently exposes `usize`-based collection lengths,
//! so the actual circuit cannot contain more operations than the host address
//! space can represent. Nevertheless, converting the result to `u128` avoids
//! unnecessarily narrowing the optimizer's accounting layer and permits safe
//! accumulation with other `u128` resource models.
//!
//! # Determinism
//!
//! All analysis is deterministic.
//!
//! No:
//!
//! - random numbers;
//! - global mutable state;
//! - hash maps;
//! - backend state;
//! - threads;
//! - floating-point arithmetic
//!
//! are required for the primary T-count analysis.
//!
//! # Safety
//!
//! This module explicitly forbids unsafe Rust.
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
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
//! - no external dependencies;
//! - no unsafe code.
//!
//! # Integration contract
//!
//! ## `fault_tolerant/mod.rs`
//!
//! Export:
//!
//! ```text
//! pub mod t_count;
//! ```
//!
//! Recommended re-exports:
//!
//! ```text
//! pub use t_count::{analyze_t_count, TCountAnalysis, TCountDelta};
//! ```
//!
//! ## `analysis/gate_counts.rs`
//!
//! This module consumes its existing authoritative:
//!
//! - T count;
//! - T-dagger count;
//! - T-family count.
//!
//! No second circuit traversal is necessary.
//!
//! ## `cost.rs`
//!
//! `cost.rs` owns the general `CostMetric::TCount` dimension and should consume
//! `TCountAnalysis::t_count()` or `t_family_count()` according to the selected
//! fault-tolerant policy.
//!
//! ## `phase_polynomial.rs`
//!
//! Phase-polynomial optimization may consume this analysis before and after a
//! transformation and compare the resulting T-count.
//!
//! This file intentionally does not depend on `phase_polynomial.rs`, preventing
//! a dependency cycle.
//!
//! ## `t_gate_reduction.rs`
//!
//! The T-gate reduction pass can use `TExponent` and `TCountDelta`.
//!
//! `t_gate_reduction.rs` owns the actual circuit transformation.
//!
//! This file owns the accounting semantics.
//!
//! ## `t_depth.rs`
//!
//! T-depth is a different metric and must remain independently calculated.
//!
//! T-count does not imply T-depth.
//!
//! Example:
//!
//! ```text
//! T q0
//! T q1
//! ```
//!
//! has T-count 2 but can have T-depth 1.
//!
//! ## `pass.rs`
//!
//! If the optimizer exposes a dedicated T-count optimization pass, that pass
//! should use the types in this file rather than defining another T-count
//! result structure.
//!
//! The transformation itself belongs to the pass implementation.
//!
//! ## `context.rs`
//!
//! `TCountAnalysis` is immutable and can safely be inserted into the typed
//! analysis cache.
//!
//! Any transformation that changes:
//!
//! - operation kind;
//! - operation sequence;
//! - operation count;
//! - operation parameters in a way that changes gate classification;
//!
//! invalidates the cached T-count analysis.
//!
//! ## `benchmarking`
//!
//! Benchmarking may consume T-count before/after optimization.
//!
//! This module never depends on benchmarking.
//!
//! ## `routing`
//!
//! Routing may increase physical/logical operation count and may create extra
//! Clifford operations, but it must not be a dependency of this analysis.
//!
//! ## `scheduling`
//!
//! Scheduling owns timing and parallel execution.
//!
//! It must not be used to calculate T-count.
//!
//! # Design rule
//!
//! A future contributor adding a new non-Clifford gate must not silently assume
//! that it is a T gate.
//!
//! Such a gate must be explicitly classified by the appropriate fault-tolerant
//! resource model.
//!
//! For example, a generic `RZ(theta)` is NOT counted as a T gate merely because
//! some value of `theta` happens to equal pi/4.
//!
//! Value-sensitive Clifford/T classification belongs to a dedicated algebraic
//! or phase-analysis subsystem.
//!
//! -----------------------------------------------------------------------------
//! Implementation
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]

use std::cmp::Ordering;
use std::fmt;

use crate::quantum::ir::gate::GateKind;
use crate::quantum::ir::QuantumCircuit;

use crate::quantum::optimization::analysis::gate_counts::{
    analyze_gate_counts,
    GateCountAnalysis,
};

// =============================================================================
// Public result aliases
// =============================================================================

/// Result type returned by T-count analysis.
pub type TCountResult<T> = Result<T, TCountError>;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by the fault-tolerant T-count subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TCountError {
    /// The canonical circuit could not be analyzed.
    InvalidCircuit {
        /// Canonical IR validation/analyzer failure.
        message: String,
    },

    /// An arithmetic operation overflowed.
    ArithmeticOverflow {
        /// Calculation that overflowed.
        calculation: &'static str,
    },

    /// A supplied T exponent was invalid.
    InvalidExponent {
        /// Supplied exponent.
        exponent: i16,
    },

    /// A resource comparison was requested with incompatible semantics.
    IncompatibleComparison {
        /// Human-readable reason.
        message: &'static str,
    },

    /// An internal invariant was violated.
    InvariantViolation {
        /// Description of the invariant failure.
        message: &'static str,
    },
}

impl fmt::Display for TCountError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCircuit { message } => {
                write!(
                    formatter,
                    "cannot analyze T-count: invalid quantum circuit: {message}"
                )
            }

            Self::ArithmeticOverflow { calculation } => {
                write!(
                    formatter,
                    "arithmetic overflow while calculating {calculation}"
                )
            }

            Self::InvalidExponent { exponent } => {
                write!(
                    formatter,
                    "invalid T exponent `{exponent}`"
                )
            }

            Self::IncompatibleComparison { message } => {
                write!(
                    formatter,
                    "incompatible T-count comparison: {message}"
                )
            }

            Self::InvariantViolation { message } => {
                write!(
                    formatter,
                    "T-count invariant violated: {message}"
                )
            }
        }
    }
}

impl std::error::Error for TCountError {}

// =============================================================================
// T exponent
// =============================================================================

/// A signed power of T reduced modulo eight.
///
/// The representation is always one of:
///
/// ```text
/// 0, 1, 2, 3, 4, 5, 6, 7
/// ```
///
/// It is useful for exact local reasoning about sequences containing T and
/// T-dagger operations.
///
/// This type does not perform circuit rewrites.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TExponent(u8);

impl TExponent {
    /// The identity exponent.
    pub const ZERO: Self = Self(0);

    /// One T.
    pub const T: Self = Self(1);

    /// T squared.
    pub const T2: Self = Self(2);

    /// T cubed.
    pub const T3: Self = Self(3);

    /// T to the fourth power.
    pub const T4: Self = Self(4);

    /// T to the fifth power.
    pub const T5: Self = Self(5);

    /// T to the sixth power.
    pub const T6: Self = Self(6);

    /// T to the seventh power.
    pub const T7: Self = Self(7);

    /// Creates an exponent from an arbitrary signed integer.
    ///
    /// The value is normalized modulo eight.
    #[must_use]
    pub const fn new(value: i16) -> Self {
        let normalized = value.rem_euclid(8) as u8;
        Self(normalized)
    }

    /// Creates an exponent from an unsigned value.
    ///
    /// The value is normalized modulo eight.
    #[must_use]
    pub const fn from_u8(value: u8) -> Self {
        Self(value % 8)
    }

    /// Returns the canonical exponent in the range `0..8`.
    #[must_use]
    pub const fn value(self) -> u8 {
        self.0
    }

    /// Returns the signed representative in the range `-4..=3`.
    ///
    /// This is useful when choosing between positive T and negative T powers.
    #[must_use]
    pub const fn signed_value(self) -> i8 {
        match self.0 {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => -4,
            5 => -3,
            6 => -2,
            7 => -1,
            _ => unreachable!(),
        }
    }

    /// Returns the inverse exponent.
    #[must_use]
    pub const fn inverse(self) -> Self {
        Self::new(-(self.0 as i16))
    }

    /// Adds two T exponents modulo eight.
    #[must_use]
    pub const fn add(self, other: Self) -> Self {
        Self::from_u8(self.0 + other.0)
    }

    /// Subtracts two T exponents modulo eight.
    #[must_use]
    pub const fn sub(self, other: Self) -> Self {
        Self::new(self.0 as i16 - other.0 as i16)
    }

    /// Returns whether this exponent is the identity.
    #[must_use]
    pub const fn is_identity(self) -> bool {
        self.0 == 0
    }

    /// Returns the minimum number of T/Tdg factors needed to represent this
    /// exponent when Clifford powers are allowed to replace groups.
    ///
    /// This is a *phase-subgroup decomposition metric*, not a circuit rewrite.
    ///
    /// Examples:
    ///
    /// ```text
    /// T^0 -> 0 T resources
    /// T^1 -> 1
    /// T^2 -> 0  (S)
    /// T^3 -> 1  (S*T)
    /// T^4 -> 0  (Z)
    /// T^5 -> 1  (Z*T)
    /// T^6 -> 0  (Sdg)
    /// T^7 -> 1  (Sdg*T)
    /// ```
    ///
    /// This assumes the surrounding compiler permits the corresponding
    /// Clifford replacement.
    #[must_use]
    pub const fn minimum_non_clifford_factors(self) -> u8 {
        match self.0 {
            0 | 2 | 4 | 6 => 0,
            1 | 3 | 5 | 7 => 1,
            _ => 0,
        }
    }

    /// Returns whether the exponent itself is a Clifford phase.
    #[must_use]
    pub const fn is_clifford_phase(self) -> bool {
        self.0 % 2 == 0
    }

    /// Returns whether the exponent contains a non-Clifford component.
    #[must_use]
    pub const fn is_non_clifford_phase(self) -> bool {
        !self.is_clifford_phase()
    }

    /// Returns the number of T-family operations represented by the canonical
    /// positive-power decomposition.
    ///
    /// This is:
    ///
    /// ```text
    /// exponent
    /// ```
    ///
    /// for the canonical positive representation and is intentionally
    /// different from `minimum_non_clifford_factors()`.
    #[must_use]
    pub const fn positive_factor_count(self) -> u8 {
        self.0
    }

    /// Converts a canonical gate kind into its signed T exponent.
    ///
    /// Returns:
    ///
    /// - `Some(+1)` for T;
    /// - `Some(-1)` for Tdg;
    /// - `None` for every other gate kind.
    #[must_use]
    pub const fn from_gate_kind(kind: GateKind) -> Option<Self> {
        match kind {
            GateKind::T => Some(Self::T),
            GateKind::Tdg => Some(Self::new(-1)),
            _ => None,
        }
    }
}

impl Default for TExponent {
    fn default() -> Self {
        Self::ZERO
    }
}

impl fmt::Display for TExponent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "T^{}", self.0)
    }
}

// =============================================================================
// T-resource objective
// =============================================================================

/// Resource interpretation used when comparing T-count candidates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TCountMetric {
    /// Count T gates only.
    T,

    /// Count T-dagger gates only.
    Tdg,

    /// Count both T and T-dagger.
    TFamily,
}

impl TCountMetric {
    /// Returns a stable machine-readable name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::T => "t_count",
            Self::Tdg => "t_dagger_count",
            Self::TFamily => "t_family_count",
        }
    }
}

impl Default for TCountMetric {
    fn default() -> Self {
        Self::TFamily
    }
}

impl fmt::Display for TCountMetric {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// T-count analysis
// =============================================================================

/// Immutable fault-tolerant T-resource analysis.
///
/// This is deliberately independent from a circuit lifetime.
///
/// It can therefore be:
///
/// - cached in `OptimizationContext`;
/// - compared before/after a pass;
/// - serialized by a future reporting layer;
/// - consumed by `cost.rs`;
/// - consumed by `phase_polynomial.rs`;
/// - retained for provenance.
///
/// All structural counters are exact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TCountAnalysis {
    /// Number of T gates.
    t_count: u128,

    /// Number of T-dagger gates.
    t_dagger_count: u128,

    /// Combined T/T-dagger count.
    t_family_count: u128,

    /// Number of all logical operations.
    operation_count: u128,

    /// Number of logical unitary operations.
    gate_count: u128,

    /// Number of Clifford operations.
    clifford_operation_count: u128,

    /// Number of non-Clifford operations according to the canonical IR's
    /// conservative gate-kind classification.
    non_clifford_operation_count: u128,

    /// Number of T-family operations that are also classified as non-Clifford.
    non_clifford_t_family_count: u128,
}

impl TCountAnalysis {
    /// Creates an analysis from the canonical gate-count analysis.
    ///
    /// This is public so another analysis subsystem can avoid traversing the
    /// circuit twice.
    pub fn from_gate_counts(
        counts: &GateCountAnalysis,
    ) -> TCountResult<Self> {
        let t_count = counts.t_count() as u128;
        let t_dagger_count = counts.t_dagger_count() as u128;

        let t_family_count = t_count
            .checked_add(t_dagger_count)
            .ok_or(TCountError::ArithmeticOverflow {
                calculation: "T-family count",
            })?;

        if t_family_count != counts.t_family_count() as u128 {
            return Err(TCountError::InvariantViolation {
                message: "gate-count analysis T-family total is inconsistent",
            });
        }

        let non_clifford_t_family_count = t_family_count;

        let result = Self {
            t_count,
            t_dagger_count,
            t_family_count,
            operation_count: counts.operation_count() as u128,
            gate_count: counts.gate_count() as u128,
            clifford_operation_count:
                counts.clifford_operation_count() as u128,
            non_clifford_operation_count:
                counts.non_clifford_operation_count() as u128,
            non_clifford_t_family_count,
        };

        result.validate()?;

        Ok(result)
    }

    /// Returns the number of T gates.
    #[must_use]
    pub const fn t_count(&self) -> u128 {
        self.t_count
    }

    /// Returns the number of T-dagger gates.
    #[must_use]
    pub const fn t_dagger_count(&self) -> u128 {
        self.t_dagger_count
    }

    /// Returns the combined T-family count.
    #[must_use]
    pub const fn t_family_count(&self) -> u128 {
        self.t_family_count
    }

    /// Returns the total logical operation count.
    #[must_use]
    pub const fn operation_count(&self) -> u128 {
        self.operation_count
    }

    /// Returns the total unitary gate count.
    #[must_use]
    pub const fn gate_count(&self) -> u128 {
        self.gate_count
    }

    /// Returns the canonical-IR Clifford count.
    #[must_use]
    pub const fn clifford_operation_count(&self) -> u128 {
        self.clifford_operation_count
    }

    /// Returns the canonical-IR non-Clifford count.
    #[must_use]
    pub const fn non_clifford_operation_count(&self) -> u128 {
        self.non_clifford_operation_count
    }

    /// Returns the number of non-Clifford operations represented by the
    /// T-family.
    #[must_use]
    pub const fn non_clifford_t_family_count(&self) -> u128 {
        self.non_clifford_t_family_count
    }

    /// Returns whether the circuit contains any T-family operation.
    #[must_use]
    pub const fn has_t_family(&self) -> bool {
        self.t_family_count != 0
    }

    /// Returns whether the circuit contains no T-family operations.
    #[must_use]
    pub const fn is_t_free(&self) -> bool {
        self.t_family_count == 0
    }

    /// Returns the selected resource metric.
    #[must_use]
    pub const fn metric(self, metric: TCountMetric) -> u128 {
        match metric {
            TCountMetric::T => self.t_count,
            TCountMetric::Tdg => self.t_dagger_count,
            TCountMetric::TFamily => self.t_family_count,
        }
    }

    /// Returns the ratio of T-family operations to unitary gates.
    ///
    /// Returns `None` when the circuit contains no unitary gates.
    #[must_use]
    pub fn t_family_fraction(&self) -> Option<f64> {
        if self.gate_count == 0 {
            None
        } else {
            Some(self.t_family_count as f64 / self.gate_count as f64)
        }
    }

    /// Returns the ratio of T-family operations to all logical operations.
    ///
    /// Returns `None` for an empty circuit.
    #[must_use]
    pub fn t_family_operation_fraction(&self) -> Option<f64> {
        if self.operation_count == 0 {
            None
        } else {
            Some(self.t_family_count as f64 / self.operation_count as f64)
        }
    }

    /// Returns the T/Tdg imbalance.
    ///
    /// A positive value means more T gates than T-dagger gates.
    /// A negative value means more T-dagger gates.
    #[must_use]
    pub fn signed_t_imbalance(&self) -> i128 {
        self.t_count as i128 - self.t_dagger_count as i128
    }

    /// Returns a conservative lower bound on the number of non-Clifford
    /// resources remaining after arbitrary *local phase-subgroup*
    /// simplification.
    ///
    /// This is not a global optimum and must never be presented as the optimum
    /// T-count of an arbitrary Clifford+T circuit.
    #[must_use]
    pub const fn local_phase_lower_bound(&self) -> u128 {
        // Every T-family operation contributes one odd phase exponent at the
        // raw structural level. Aggregate parity alone cannot establish a
        // stronger global lower bound without circuit algebra.
        //
        // Therefore the universally safe lower bound at this layer is zero.
        0
    }

    /// Validates internal invariants.
    pub fn validate(&self) -> TCountResult<()> {
        let expected_t_family = self
            .t_count
            .checked_add(self.t_dagger_count)
            .ok_or(TCountError::ArithmeticOverflow {
                calculation: "T-family invariant",
            })?;

        if expected_t_family != self.t_family_count {
            return Err(TCountError::InvariantViolation {
                message: "T + Tdg != T-family count",
            });
        }

        if self.t_family_count > self.gate_count {
            return Err(TCountError::InvariantViolation {
                message: "T-family count exceeds total unitary gate count",
            });
        }

        if self.gate_count > self.operation_count {
            return Err(TCountError::InvariantViolation {
                message: "gate count exceeds operation count",
            });
        }

        if self.non_clifford_t_family_count > self.non_clifford_operation_count {
            return Err(TCountError::InvariantViolation {
                message: "T-family count exceeds non-Clifford operation count",
            });
        }

        Ok(())
    }
}

// =============================================================================
// T-count delta
// =============================================================================

/// Exact before/after T-resource difference.
///
/// Positive `removed_*` values mean the candidate uses fewer resources.
///
/// Negative values mean the candidate uses more resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TCountDelta {
    /// Difference in T gates.
    pub t_count: i128,

    /// Difference in T-dagger gates.
    pub t_dagger_count: i128,

    /// Difference in combined T-family operations.
    pub t_family_count: i128,
}

impl TCountDelta {
    /// Calculates the difference from `before` to `after`.
    ///
    /// The result is:
    ///
    /// ```text
    /// after - before
    /// ```
    ///
    /// Therefore:
    ///
    /// ```text
    /// -3 => three fewer T resources
    /// +2 => two more T resources
    /// ```
    pub fn between(
        before: &TCountAnalysis,
        after: &TCountAnalysis,
    ) -> TCountResult<Self> {
        let t_count = signed_difference(
            after.t_count(),
            before.t_count(),
            "T-count delta",
        )?;

        let t_dagger_count = signed_difference(
            after.t_dagger_count(),
            before.t_dagger_count(),
            "T-dagger count delta",
        )?;

        let t_family_count = signed_difference(
            after.t_family_count(),
            before.t_family_count(),
            "T-family count delta",
        )?;

        Ok(Self {
            t_count,
            t_dagger_count,
            t_family_count,
        })
    }

    /// Returns true when the candidate reduced T-family count.
    #[must_use]
    pub const fn improved(&self) -> bool {
        self.t_family_count < 0
    }

    /// Returns true when the candidate increased T-family count.
    #[must_use]
    pub const fn regressed(&self) -> bool {
        self.t_family_count > 0
    }

    /// Returns true when the T-family count did not change.
    #[must_use]
    pub const fn unchanged(&self) -> bool {
        self.t_family_count == 0
    }

    /// Returns the number of T-family operations removed.
    #[must_use]
    pub fn removed_t_family(&self) -> u128 {
        if self.t_family_count < 0 {
            (-self.t_family_count) as u128
        } else {
            0
        }
    }

    /// Returns the number of T-family operations added.
    #[must_use]
    pub fn added_t_family(&self) -> u128 {
        if self.t_family_count > 0 {
            self.t_family_count as u128
        } else {
            0
        }
    }

    /// Returns a percentage reduction when `before` is non-zero.
    #[must_use]
    pub fn reduction_percentage(
        &self,
        before: &TCountAnalysis,
    ) -> Option<f64> {
        if before.t_family_count() == 0 {
            return None;
        }

        let removed = self.removed_t_family() as f64;
        let initial = before.t_family_count() as f64;

        Some((removed / initial) * 100.0)
    }
}

fn signed_difference(
    after: u128,
    before: u128,
    calculation: &'static str,
) -> TCountResult<i128> {
    if after >= before {
        let difference = after - before;

        if difference > i128::MAX as u128 {
            return Err(TCountError::ArithmeticOverflow { calculation });
        }

        Ok(difference as i128)
    } else {
        let difference = before - after;

        if difference > i128::MAX as u128 {
            return Err(TCountError::ArithmeticOverflow { calculation });
        }

        Ok(-(difference as i128))
    }
}

// =============================================================================
// Candidate comparison
// =============================================================================

/// Compares two T-count analyses using one selected metric.
///
/// This is intentionally independent of the global optimization objective
/// system. `cost.rs` remains the owner of multi-objective optimization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TCountOrdering {
    /// First candidate is cheaper.
    Better,

    /// Both candidates have the same selected T resource.
    Equivalent,

    /// First candidate is more expensive.
    Worse,
}

impl TCountOrdering {
    /// Converts Rust ordering into the T-count semantic ordering.
    #[must_use]
    pub const fn from_ordering(ordering: Ordering) -> Self {
        match ordering {
            Ordering::Less => Self::Better,
            Ordering::Equal => Self::Equivalent,
            Ordering::Greater => Self::Worse,
        }
    }
}

/// Compares two T-count analyses.
#[must_use]
pub fn compare_t_count(
    first: &TCountAnalysis,
    second: &TCountAnalysis,
    metric: TCountMetric,
) -> TCountOrdering {
    TCountOrdering::from_ordering(
        first.metric(metric).cmp(&second.metric(metric))
    )
}

// =============================================================================
// Public analysis API
// =============================================================================

/// Analyzes the exact T-family resources in a canonical Zamani quantum circuit.
///
/// This function performs no mutation.
///
/// It validates the circuit through the canonical gate-count analysis and then
/// derives the T-resource result from that authoritative analysis.
///
/// # Complexity
///
/// ```text
/// Time:   O(N + A + P)
/// Memory: O(1) additional memory
/// ```
///
/// # Example
///
/// ```ignore
/// let analysis = analyze_t_count(&circuit)?;
///
/// println!("T count: {}", analysis.t_count());
/// println!("Tdg count: {}", analysis.t_dagger_count());
/// println!("T-family count: {}", analysis.t_family_count());
/// ```
///
/// # Important
///
/// This function reports the circuit as it currently exists.
///
/// It does NOT perform:
///
/// - T cancellation;
/// - T/Tdg commutation;
/// - phase-polynomial optimization;
/// - TODD;
/// - Reed-Muller decoding;
/// - ZX optimization;
/// - synthesis.
///
/// Those are transformations and belong to dedicated optimization passes.
pub fn analyze_t_count(
    circuit: &QuantumCircuit,
) -> TCountResult<TCountAnalysis> {
    let gate_counts = analyze_gate_counts(circuit).map_err(|error| {
        TCountError::InvalidCircuit {
            message: error.to_string(),
        }
    })?;

    TCountAnalysis::from_gate_counts(&gate_counts)
}

/// Builds T-count analysis from an already computed canonical gate-count
/// analysis.
///
/// Use this function when the optimizer already has a cached
/// `GateCountAnalysis`, avoiding a second circuit traversal.
pub fn analyze_t_count_from_gate_counts(
    gate_counts: &GateCountAnalysis,
) -> TCountResult<TCountAnalysis> {
    TCountAnalysis::from_gate_counts(gate_counts)
}

// =============================================================================
// T-family classification helpers
// =============================================================================

/// Returns whether a canonical gate kind is exactly a T-family operation.
#[must_use]
pub const fn is_t_family(kind: GateKind) -> bool {
    matches!(kind, GateKind::T | GateKind::Tdg)
}

/// Returns the signed T exponent represented by a canonical gate kind.
///
/// ```text
/// T   -> +1
/// Tdg -> -1
/// ```
#[must_use]
pub const fn t_exponent(kind: GateKind) -> Option<TExponent> {
    TExponent::from_gate_kind(kind)
}

/// Converts a sequence of T/Tdg gate kinds into a phase exponent.
///
/// Non-T-family gates cause the function to return `None`, because this helper
/// deliberately does not infer commutation through other operations.
pub fn sequence_exponent<I>(
    kinds: I,
) -> Option<TExponent>
where
    I: IntoIterator<Item = GateKind>,
{
    let mut exponent = TExponent::ZERO;

    for kind in kinds {
        let gate_exponent = TExponent::from_gate_kind(kind)?;

        exponent = exponent.add(gate_exponent);
    }

    Some(exponent)
}

/// Returns the exact signed exponent for one T-family gate.
///
/// This is primarily useful to transformation passes.
#[must_use]
pub const fn gate_exponent(kind: GateKind) -> Option<i8> {
    match kind {
        GateKind::T => Some(1),
        GateKind::Tdg => Some(-1),
        _ => None,
    }
}

// =============================================================================
// T-count objective helpers
// =============================================================================

/// Returns whether `candidate` is strictly better than `current` for T-family
/// minimization.
#[must_use]
pub const fn improves_t_family(
    current: &TCountAnalysis,
    candidate: &TCountAnalysis,
) -> bool {
    candidate.t_family_count() < current.t_family_count()
}

/// Returns whether `candidate` is no worse than `current` for T-family
/// minimization.
#[must_use]
pub const fn does_not_increase_t_family(
    current: &TCountAnalysis,
    candidate: &TCountAnalysis,
) -> bool {
    candidate.t_family_count() <= current.t_family_count()
}

/// Returns whether `candidate` strictly reduces T gates.
#[must_use]
pub const fn improves_t_count(
    current: &TCountAnalysis,
    candidate: &TCountAnalysis,
) -> bool {
    candidate.t_count() < current.t_count()
}

/// Returns whether `candidate` strictly reduces T-dagger gates.
#[must_use]
pub const fn improves_t_dagger_count(
    current: &TCountAnalysis,
    candidate: &TCountAnalysis,
) -> bool {
    candidate.t_dagger_count() < current.t_dagger_count()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // TExponent
    // -------------------------------------------------------------------------

    #[test]
    fn exponent_normalizes_positive_values() {
        assert_eq!(TExponent::new(0).value(), 0);
        assert_eq!(TExponent::new(1).value(), 1);
        assert_eq!(TExponent::new(8).value(), 0);
        assert_eq!(TExponent::new(9).value(), 1);
        assert_eq!(TExponent::new(16).value(), 0);
    }

    #[test]
    fn exponent_normalizes_negative_values() {
        assert_eq!(TExponent::new(-1).value(), 7);
        assert_eq!(TExponent::new(-2).value(), 6);
        assert_eq!(TExponent::new(-8).value(), 0);
        assert_eq!(TExponent::new(-9).value(), 7);
    }

    #[test]
    fn exponent_signed_representation_is_canonical() {
        assert_eq!(TExponent::new(0).signed_value(), 0);
        assert_eq!(TExponent::new(1).signed_value(), 1);
        assert_eq!(TExponent::new(2).signed_value(), 2);
        assert_eq!(TExponent::new(3).signed_value(), 3);
        assert_eq!(TExponent::new(4).signed_value(), -4);
        assert_eq!(TExponent::new(5).signed_value(), -3);
        assert_eq!(TExponent::new(6).signed_value(), -2);
        assert_eq!(TExponent::new(7).signed_value(), -1);
    }

    #[test]
    fn exponent_inverse_is_correct() {
        assert_eq!(
            TExponent::T.inverse(),
            TExponent::new(-1)
        );

        assert_eq!(
            TExponent::new(-1).inverse(),
            TExponent::T
        );

        assert_eq!(
            TExponent::T4.inverse(),
            TExponent::T4
        );
    }

    #[test]
    fn exponent_addition_is_modulo_eight() {
        assert_eq!(
            TExponent::new(7).add(TExponent::T),
            TExponent::ZERO
        );

        assert_eq!(
            TExponent::new(6).add(TExponent::T),
            TExponent::T7
        );

        assert_eq!(
            TExponent::new(3).add(TExponent::new(5)),
            TExponent::ZERO
        );
    }

    #[test]
    fn exponent_subtraction_is_modulo_eight() {
        assert_eq!(
            TExponent::T.sub(TExponent::T),
            TExponent::ZERO
        );

        assert_eq!(
            TExponent::ZERO.sub(TExponent::T),
            TExponent::T7
        );
    }

    #[test]
    fn exponent_clifford_classification_is_exact() {
        assert!(TExponent::ZERO.is_clifford_phase());
        assert!(TExponent::T2.is_clifford_phase());
        assert!(TExponent::T4.is_clifford_phase());
        assert!(TExponent::T6.is_clifford_phase());

        assert!(!TExponent::T.is_clifford_phase());
        assert!(!TExponent::T3.is_clifford_phase());
        assert!(!TExponent::T5.is_clifford_phase());
        assert!(!TExponent::T7.is_clifford_phase());
    }

    #[test]
    fn exponent_minimum_non_clifford_factor_count_is_correct() {
        assert_eq!(
            TExponent::new(0).minimum_non_clifford_factors(),
            0
        );

        assert_eq!(
            TExponent::new(1).minimum_non_clifford_factors(),
            1
        );

        assert_eq!(
            TExponent::new(2).minimum_non_clifford_factors(),
            0
        );

        assert_eq!(
            TExponent::new(3).minimum_non_clifford_factors(),
            1
        );

        assert_eq!(
            TExponent::new(4).minimum_non_clifford_factors(),
            0
        );

        assert_eq!(
            TExponent::new(5).minimum_non_clifford_factors(),
            1
        );

        assert_eq!(
            TExponent::new(6).minimum_non_clifford_factors(),
            0
        );

        assert_eq!(
            TExponent::new(7).minimum_non_clifford_factors(),
            1
        );
    }

    // -------------------------------------------------------------------------
    // Gate classification
    // -------------------------------------------------------------------------

    #[test]
    fn only_t_and_tdg_are_t_family() {
        assert!(is_t_family(GateKind::T));
        assert!(is_t_family(GateKind::Tdg));

        assert!(!is_t_family(GateKind::I));
        assert!(!is_t_family(GateKind::S));
        assert!(!is_t_family(GateKind::Sdg));
        assert!(!is_t_family(GateKind::Z));
        assert!(!is_t_family(GateKind::RX));
        assert!(!is_t_family(GateKind::RZ));
    }

    #[test]
    fn gate_exponents_are_correct() {
        assert_eq!(
            gate_exponent(GateKind::T),
            Some(1)
        );

        assert_eq!(
            gate_exponent(GateKind::Tdg),
            Some(-1)
        );

        assert_eq!(
            gate_exponent(GateKind::H),
            None
        );
    }

    #[test]
    fn sequence_exponent_handles_t_family_only() {
        let exponent = sequence_exponent([
            GateKind::T,
            GateKind::T,
            GateKind::Tdg,
        ])
        .expect("sequence contains only T-family operations");

        assert_eq!(exponent, TExponent::T);
    }

    #[test]
    fn sequence_exponent_returns_none_for_non_t_gate() {
        assert_eq!(
            sequence_exponent([
                GateKind::T,
                GateKind::H,
                GateKind::Tdg,
            ]),
            None
        );
    }

    #[test]
    fn eight_t_gates_are_identity_exponent() {
        let exponent = sequence_exponent([
            GateKind::T,
            GateKind::T,
            GateKind::T,
            GateKind::T,
            GateKind::T,
            GateKind::T,
            GateKind::T,
            GateKind::T,
        ])
        .expect("all operations are T gates");

        assert!(exponent.is_identity());
    }

    // -------------------------------------------------------------------------
    // TCountAnalysis construction
    // -------------------------------------------------------------------------

    #[test]
    fn analysis_from_zero_gate_counts_is_zero() {
        let counts = GateCountAnalysis::default();

        let result =
            TCountAnalysis::from_gate_counts(&counts)
                .expect("zero counts are valid");

        assert_eq!(result.t_count(), 0);
        assert_eq!(result.t_dagger_count(), 0);
        assert_eq!(result.t_family_count(), 0);
        assert!(result.is_t_free());
    }

    #[test]
    fn default_analysis_is_valid() {
        let analysis = TCountAnalysis::default();

        analysis
            .validate()
            .expect("default analysis must be valid");
    }

    // -------------------------------------------------------------------------
    // Delta
    // -------------------------------------------------------------------------

    #[test]
    fn delta_identifies_improvement() {
        let before = TCountAnalysis {
            t_count: 8,
            t_dagger_count: 2,
            t_family_count: 10,
            operation_count: 20,
            gate_count: 20,
            clifford_operation_count: 10,
            non_clifford_operation_count: 10,
            non_clifford_t_family_count: 10,
        };

        let after = TCountAnalysis {
            t_count: 4,
            t_dagger_count: 1,
            t_family_count: 5,
            operation_count: 15,
            gate_count: 15,
            clifford_operation_count: 10,
            non_clifford_operation_count: 5,
            non_clifford_t_family_count: 5,
        };

        let delta =
            TCountDelta::between(&before, &after)
                .expect("valid delta");

        assert_eq!(delta.t_count, -4);
        assert_eq!(delta.t_dagger_count, -1);
        assert_eq!(delta.t_family_count, -5);
        assert!(delta.improved());
        assert_eq!(delta.removed_t_family(), 5);
        assert_eq!(delta.added_t_family(), 0);
    }

    #[test]
    fn delta_identifies_regression() {
        let before = TCountAnalysis {
            t_count: 1,
            t_dagger_count: 1,
            t_family_count: 2,
            operation_count: 2,
            gate_count: 2,
            clifford_operation_count: 0,
            non_clifford_operation_count: 2,
            non_clifford_t_family_count: 2,
        };

        let after = TCountAnalysis {
            t_count: 3,
            t_dagger_count: 2,
            t_family_count: 5,
            operation_count: 5,
            gate_count: 5,
            clifford_operation_count: 0,
            non_clifford_operation_count: 5,
            non_clifford_t_family_count: 5,
        };

        let delta =
            TCountDelta::between(&before, &after)
                .expect("valid delta");

        assert!(delta.regressed());
        assert_eq!(delta.added_t_family(), 3);
    }

    #[test]
    fn delta_identifies_no_change() {
        let analysis = TCountAnalysis {
            t_count: 3,
            t_dagger_count: 2,
            t_family_count: 5,
            operation_count: 5,
            gate_count: 5,
            clifford_operation_count: 0,
            non_clifford_operation_count: 5,
            non_clifford_t_family_count: 5,
        };

        let delta =
            TCountDelta::between(&analysis, &analysis)
                .expect("identical analysis must compare");

        assert!(delta.unchanged());
        assert_eq!(delta.removed_t_family(), 0);
        assert_eq!(delta.added_t_family(), 0);
    }

    // -------------------------------------------------------------------------
    // Comparison
    // -------------------------------------------------------------------------

    #[test]
    fn comparison_prefers_lower_t_family_count() {
        let first = TCountAnalysis {
            t_count: 2,
            t_dagger_count: 2,
            t_family_count: 4,
            operation_count: 4,
            gate_count: 4,
            clifford_operation_count: 0,
            non_clifford_operation_count: 4,
            non_clifford_t_family_count: 4,
        };

        let second = TCountAnalysis {
            t_count: 4,
            t_dagger_count: 4,
            t_family_count: 8,
            operation_count: 8,
            gate_count: 8,
            clifford_operation_count: 0,
            non_clifford_operation_count: 8,
            non_clifford_t_family_count: 8,
        };

        assert_eq!(
            compare_t_count(
                &first,
                &second,
                TCountMetric::TFamily,
            ),
            TCountOrdering::Better
        );
    }

    #[test]
    fn comparison_can_target_t_only() {
        let first = TCountAnalysis {
            t_count: 1,
            t_dagger_count: 8,
            t_family_count: 9,
            operation_count: 9,
            gate_count: 9,
            clifford_operation_count: 0,
            non_clifford_operation_count: 9,
            non_clifford_t_family_count: 9,
        };

        let second = TCountAnalysis {
            t_count: 2,
            t_dagger_count: 0,
            t_family_count: 2,
            operation_count: 2,
            gate_count: 2,
            clifford_operation_count: 0,
            non_clifford_operation_count: 2,
            non_clifford_t_family_count: 2,
        };

        assert_eq!(
            compare_t_count(
                &first,
                &second,
                TCountMetric::T,
            ),
            TCountOrdering::Better
        );
    }

    // -------------------------------------------------------------------------
    // Optimization predicates
    // -------------------------------------------------------------------------

    #[test]
    fn improvement_predicates_are_correct() {
        let current = TCountAnalysis {
            t_count: 8,
            t_dagger_count: 2,
            t_family_count: 10,
            operation_count: 10,
            gate_count: 10,
            clifford_operation_count: 0,
            non_clifford_operation_count: 10,
            non_clifford_t_family_count: 10,
        };

        let candidate = TCountAnalysis {
            t_count: 4,
            t_dagger_count: 1,
            t_family_count: 5,
            operation_count: 5,
            gate_count: 5,
            clifford_operation_count: 0,
            non_clifford_operation_count: 5,
            non_clifford_t_family_count: 5,
        };

        assert!(improves_t_family(
            &current,
            &candidate
        ));

        assert!(does_not_increase_t_family(
            &current,
            &candidate
        ));

        assert!(improves_t_count(
            &current,
            &candidate
        ));

        assert!(improves_t_dagger_count(
            &current,
            &candidate
        ));
    }
}