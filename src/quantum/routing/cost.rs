//! Zamani Quantum Routing — Production Cost Models
//!
//! `src/quantum/routing/cost.rs`
//!
//! # Architectural responsibility
//!
//! This module owns the routing cost vocabulary and deterministic cost
//! evaluation/comparison machinery used by:
//!
//! - layout;
//! - basic routing;
//! - shortest-path routing;
//! - lookahead routing;
//! - SABRE/LightSABRE-style routing;
//! - noise-aware routing;
//! - dynamic routing;
//! - candidate generation;
//! - routing verification;
//! - routing metrics;
//! - benchmarking;
//! - hardware-aware compilation.
//!
//! This module does NOT:
//!
//! - own physical topology storage;
//! - own logical/physical mapping state;
//! - mutate circuits;
//! - insert SWAP operations;
//! - perform gate decomposition;
//! - perform scheduling;
//! - acquire calibration data;
//! - communicate with hardware providers;
//! - execute circuits;
//! - parse OpenQASM;
//! - perform QEC;
//! - perform simulation.
//!
//! Those responsibilities belong to the corresponding routing, hardware,
//! scheduling, frontend, QEC, simulation, and execution modules.
//!
//! # Stable dependency boundary
//!
//! This file intentionally depends only on:
//!
//! - `types.rs` for stable routing vocabulary;
//! - `errors.rs` for the routing-wide error taxonomy;
//! - the Rust standard library.
//!
//! It does NOT depend on:
//!
//! - `topology.rs`;
//! - `mapping.rs`;
//! - `layout.rs`;
//! - `router.rs`;
//! - `algorithms/*`;
//! - `transpiler.rs`;
//! - `quantum::ir`;
//! - `quantum::hardware`.
//!
//! This is deliberate.
//!
//! Later modules consume this file rather than forcing this file to depend on
//! implementations that have not yet been created.
//!
//! # Integration contract
//!
//! ```text
//!                         types.rs
//!                            │
//!                            ▼
//!                         cost.rs
//!                    ┌───────┼────────┐
//!                    │       │        │
//!                    ▼       ▼        ▼
//!                 layout  algorithms  router
//!                            │        │
//!                            ▼        ▼
//!                       candidates verification
//!                            │
//!                            ▼
//!                       benchmarking
//! ```
//!
//! Hardware integration is intentionally trait-based:
//!
//! ```text
//! hardware::calibration ───────┐
//! hardware::instruction_set ───┼──► CostMetricProvider
//! hardware::timing ────────────┤
//! hardware::topology ──────────┘
//!                                  │
//!                                  ▼
//!                              cost.rs
//! ```
//!
//! A future hardware module therefore does NOT need to modify this file merely
//! because it begins providing calibration, timing, fidelity, or error data.
//!
//! # Numerical design
//!
//! Floating-point values are not used for ordering routing candidates.
//!
//! Routing decisions must be deterministic and reproducible. Raw `f32`/`f64`
//! values can contain NaN and infinity and can produce platform/compiler
//! dependent comparison behavior.
//!
//! Instead, hardware-aware metrics are represented using fixed-point integer
//! units:
//!
//! - duration: nanoseconds (`u64`);
//! - error: parts-per-billion (`u64`);
//! - fidelity: parts-per-billion (`u64`);
//! - arbitrary objective weights: unsigned fixed-point (`u64`);
//! - accumulated scores: `u128` where appropriate.
//!
//! Conversion from floating-point calibration data is supported only through
//! explicit checked constructors. Non-finite, negative, or out-of-range input
//! is rejected.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden.
//!
//! ```text
//! #![deny(unsafe_code)]
//! #![deny(unsafe_op_in_unsafe_fn)]
//! #![deny(unused_must_use)]
//! ```
//!
//! # Algorithmic background
//!
//! Modern quantum routers do not optimize only the number of SWAPs.
//!
//! SABRE-style routing evaluates candidate layouts using the distances of
//! interacting qubits, with variants that incorporate an extended/lookahead
//! set and decay penalties. Current production Qiskit documentation also
//! distinguishes basic, lookahead and decay heuristics.
//!
//! This module therefore separates:
//!
//! 1. raw routing metrics;
//! 2. primary routing objectives;
//! 3. weighted objectives;
//! 4. SABRE heuristic components;
//! 5. hardware-quality metrics;
//! 6. deterministic comparison.
//!
//! The algorithm implementations remain in `algorithms/`.
//!
//! # File completion invariant
//!
//! This file is complete when:
//!
//! 1. every routing objective can be evaluated;
//! 2. every comparison is deterministic;
//! 3. integer overflow cannot silently wrap;
//! 4. invalid numerical input is rejected;
//! 5. floating-point input cannot introduce NaN ordering;
//! 6. hardware-aware metrics can be supplied without modifying this file;
//! 7. SABRE basic/lookahead/decay scoring has a stable contract;
//! 8. custom cost models have a stable trait boundary;
//! 9. cost values are immutable after construction;
//! 10. cost evaluation does not mutate routing state;
//! 11. this file does not depend on later routing implementation files;
//! 12. the API is compatible with Rust 1.97/1.97.1;
//! 13. no unsafe Rust exists in this file.

// =============================================================================
// Module-level safety/quality policy
// =============================================================================

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// =============================================================================
// Imports
// =============================================================================

use crate::quantum::routing::errors::{
    CostModelError,
    RoutingResult,
};

use crate::quantum::routing::types::{
    RoutingAlgorithm,
    RoutingObjective,
};

use std::cmp::Ordering;
use std::fmt;
use std::time::Duration;

// =============================================================================
// Numeric constants
// =============================================================================

/// Fixed-point scale used for probability/error/fidelity metrics.
///
/// `1_000_000_000` represents 100%.
pub const PROBABILITY_SCALE: u64 = 1_000_000_000;

/// Maximum representable probability-scale value.
pub const MAX_PROBABILITY_UNITS: u64 = PROBABILITY_SCALE;

/// Fixed-point scale used for configurable weights.
///
/// `1_000_000` represents one whole weight unit.
pub const WEIGHT_SCALE: u64 = 1_000_000;

/// Default weight for the front layer in SABRE-style lookahead scoring.
pub const DEFAULT_FRONT_LAYER_WEIGHT: u64 = WEIGHT_SCALE;

/// Default weight for the extended/lookahead set.
pub const DEFAULT_EXTENDED_SET_WEIGHT: u64 = 200_000;

/// Default decay base represented in fixed-point form.
///
/// `1_000_000` means 1.0.
pub const DEFAULT_DECAY_BASE: u64 = WEIGHT_SCALE;

/// Default decay increment represented in fixed-point form.
///
/// `0` means no decay penalty.
pub const DEFAULT_DECAY_INCREMENT: u64 = 100_000;

/// Maximum supported metric count before a bounded cost calculation rejects
/// the candidate.
pub const DEFAULT_MAX_METRICS: usize = 1_000_000;

// =============================================================================
// Metric units
// =============================================================================

/// Unit used for a routing metric.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CostUnit {
    /// Dimensionless operation count.
    Count,

    /// Dimensionless circuit depth.
    Depth,

    /// Nanoseconds.
    Nanoseconds,

    /// Parts per billion.
    PartsPerBillion,

    /// Fixed-point weight units.
    WeightUnits,

    /// Dimensionless heuristic distance.
    Distance,
}

impl CostUnit {
    /// Returns a stable machine-readable name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Count => "count",
            Self::Depth => "depth",
            Self::Nanoseconds => "nanoseconds",
            Self::PartsPerBillion => "parts_per_billion",
            Self::WeightUnits => "weight_units",
            Self::Distance => "distance",
        }
    }
}

impl fmt::Display for CostUnit {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Fixed-point helpers
// =============================================================================

/// Checked conversion of a finite non-negative floating-point value into
/// fixed-point integer units.
///
/// This helper is intentionally private. Public callers should use the
/// semantically named constructors below.
///
/// `value * scale` must be finite and representable as `u64`.
fn fixed_point_from_f64(
    value: f64,
    scale: u64,
    metric: &'static str,
) -> RoutingResult<u64> {
    if !value.is_finite() {
        return Err(CostModelError::InvalidMetric {
            metric: format!("{metric}: value must be finite"),
        }
        .into());
    }

    if value < 0.0 {
        return Err(CostModelError::InvalidMetric {
            metric: format!("{metric}: value cannot be negative"),
        }
        .into());
    }

    let scaled = value * scale as f64;

    if !scaled.is_finite() {
        return Err(CostModelError::InvalidMetric {
            metric: format!("{metric}: scaled value is not finite"),
        }
        .into());
    }

    if scaled > u64::MAX as f64 {
        return Err(CostModelError::InvalidMetric {
            metric: format!("{metric}: value exceeds u64 range"),
        }
        .into());
    }

    Ok(scaled.round() as u64)
}

/// Converts a probability represented as `0.0..=1.0` to parts per billion.
#[must_use]
pub fn probability_to_ppb(value: f64) -> RoutingResult<u64> {
    let result =
        fixed_point_from_f64(value, PROBABILITY_SCALE, "probability")?;

    if result > PROBABILITY_SCALE {
        return Err(CostModelError::InvalidMetric {
            metric: "probability: expected value in 0.0..=1.0".to_string(),
        }
        .into());
    }

    Ok(result)
}

/// Converts an error probability into parts per billion.
#[must_use]
pub fn error_rate_to_ppb(value: f64) -> RoutingResult<u64> {
    probability_to_ppb(value)
}

/// Converts a fidelity probability into parts per billion.
#[must_use]
pub fn fidelity_to_ppb(value: f64) -> RoutingResult<u64> {
    probability_to_ppb(value)
}

/// Converts a floating-point weight to fixed-point weight units.
#[must_use]
pub fn weight_from_f64(value: f64) -> RoutingResult<u64> {
    fixed_point_from_f64(value, WEIGHT_SCALE, "weight")
}

// =============================================================================
// Routing metrics
// =============================================================================

/// Immutable raw metrics describing a routing candidate/result.
///
/// This is deliberately independent from `RoutingResult` in `result.rs`.
/// `RoutingMetrics` is the *input measurement vocabulary* used while evaluating
/// a candidate; `result.rs` can later aggregate it into a public route result.
///
/// All fields are non-negative.
///
/// # Semantic distinction
///
/// `swap_count` is a routing-specific count.
///
/// `gate_count` is the number of logical/physical gates represented by the
/// candidate when known.
///
/// `depth` is the resulting circuit depth, not merely the number of routing
/// operations.
///
/// `duration_ns` is estimated physical execution duration.
///
/// `error_ppb` is estimated aggregate error probability in parts per billion.
///
/// `fidelity_ppb` is estimated aggregate fidelity in parts per billion.
/// Typically:
///
/// ```text
/// fidelity = 1 - error
/// ```
///
/// but a hardware provider may supply a more accurate aggregate fidelity
/// estimate, so the two fields are intentionally independent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RoutingMetrics {
    /// Number of inserted SWAP operations.
    pub swap_count: u64,

    /// Number of inserted bridge/movement operations other than SWAP.
    pub movement_count: u64,

    /// Number of logical operations represented.
    pub gate_count: u64,

    /// Resulting circuit depth.
    pub depth: u64,

    /// Estimated physical execution duration.
    pub duration_ns: u64,

    /// Estimated aggregate error probability.
    pub error_ppb: u64,

    /// Estimated aggregate fidelity.
    pub fidelity_ppb: u64,

    /// Aggregate physical interaction distance.
    pub interaction_distance: u64,
}

impl RoutingMetrics {
    /// Creates an empty metric set.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            swap_count: 0,
            movement_count: 0,
            gate_count: 0,
            depth: 0,
            duration_ns: 0,
            error_ppb: 0,
            fidelity_ppb: 0,
            interaction_distance: 0,
        }
    }

    /// Creates metrics containing only a SWAP count.
    #[must_use]
    pub const fn from_swap_count(swap_count: u64) -> Self {
        Self {
            swap_count,
            ..Self::zero()
        }
    }

    /// Returns whether the metric set contains no measured work.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.swap_count == 0
            && self.movement_count == 0
            && self.gate_count == 0
            && self.depth == 0
            && self.duration_ns == 0
            && self.error_ppb == 0
            && self.fidelity_ppb == 0
            && self.interaction_distance == 0
    }

    /// Validates all metric invariants.
    pub fn validate(self) -> RoutingResult<()> {
        if self.error_ppb > PROBABILITY_SCALE {
            return Err(CostModelError::InvalidMetric {
                metric: "error_ppb must be <= 1_000_000_000".to_string(),
            }
            .into());
        }

        if self.fidelity_ppb > PROBABILITY_SCALE {
            return Err(CostModelError::InvalidMetric {
                metric: "fidelity_ppb must be <= 1_000_000_000".to_string(),
            }
            .into());
        }

        Ok(())
    }

    /// Returns the implied fidelity from the error metric.
    #[must_use]
    pub const fn implied_fidelity_ppb(self) -> u64 {
        PROBABILITY_SCALE.saturating_sub(self.error_ppb)
    }

    /// Returns the implied error from the fidelity metric.
    #[must_use]
    pub const fn implied_error_ppb(self) -> u64 {
        PROBABILITY_SCALE.saturating_sub(self.fidelity_ppb)
    }

    /// Adds another metric set using checked arithmetic.
    ///
    /// Saturation is deliberately NOT used. Overflow means the candidate
    /// exceeds the representable metric domain and must be rejected.
    pub fn checked_add(self, rhs: Self) -> RoutingResult<Self> {
        let result = Self {
            swap_count: self.swap_count.checked_add(rhs.swap_count),
            movement_count: self.movement_count.checked_add(rhs.movement_count),
            gate_count: self.gate_count.checked_add(rhs.gate_count),
            depth: self.depth.checked_add(rhs.depth),
            duration_ns: self.duration_ns.checked_add(rhs.duration_ns),
            error_ppb: self.error_ppb.checked_add(rhs.error_ppb),
            fidelity_ppb: self.fidelity_ppb.checked_add(rhs.fidelity_ppb),
            interaction_distance: self
                .interaction_distance
                .checked_add(rhs.interaction_distance),
        };

        let (
            Some(swap_count),
            Some(movement_count),
            Some(gate_count),
            Some(depth),
            Some(duration_ns),
            Some(error_ppb),
            Some(fidelity_ppb),
            Some(interaction_distance),
        ) = (
            result.swap_count,
            result.movement_count,
            result.gate_count,
            result.depth,
            result.duration_ns,
            result.error_ppb,
            result.fidelity_ppb,
            result.interaction_distance,
        )
        else {
            return Err(CostModelError::EvaluationFailed {
                detail: "routing metric accumulation overflowed".to_string(),
            }
            .into());
        };

        let result = Self {
            swap_count,
            movement_count,
            gate_count,
            depth,
            duration_ns,
            error_ppb,
            fidelity_ppb,
            interaction_distance,
        };

        result.validate()?;

        Ok(result)
    }

    /// Returns the duration as `Duration`.
    #[must_use]
    pub fn duration(self) -> Duration {
        Duration::from_nanos(self.duration_ns)
    }

    /// Constructs metrics from floating-point error/fidelity probabilities.
    #[must_use]
    pub fn with_probabilities(
        swap_count: u64,
        depth: u64,
        duration_ns: u64,
        error_probability: f64,
        fidelity_probability: f64,
    ) -> RoutingResult<Self> {
        let metrics = Self {
            swap_count,
            movement_count: 0,
            gate_count: 0,
            depth,
            duration_ns,
            error_ppb: error_rate_to_ppb(error_probability)?,
            fidelity_ppb: fidelity_to_ppb(fidelity_probability)?,
            interaction_distance: 0,
        };

        metrics.validate()?;
        Ok(metrics)
    }
}

// =============================================================================
// Routing cost vector
// =============================================================================

/// Deterministic multi-dimensional routing cost.
///
/// This is the canonical value compared by routing algorithms.
///
/// The vector preserves all major quality dimensions rather than reducing
/// everything to a single opaque scalar.
///
/// `weighted_score` is optional metadata for the `Weighted` objective. It is
/// not used by non-weighted objectives.
///
/// # Ordering
///
/// The `compare` method uses the selected objective. The struct itself has a
/// deterministic total ordering for use in maps/sets, but algorithmic decisions
/// should use [`RoutingCost::compare_for`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RoutingCost {
    /// Primary raw routing metrics.
    pub metrics: RoutingMetrics,

    /// Weighted objective score when calculated.
    pub weighted_score: Option<u128>,
}

impl RoutingCost {
    /// Creates a cost from raw metrics.
    pub fn new(metrics: RoutingMetrics) -> RoutingResult<Self> {
        metrics.validate()?;

        Ok(Self {
            metrics,
            weighted_score: None,
        })
    }

    /// Creates a cost with an explicit weighted score.
    pub fn with_weighted_score(
        metrics: RoutingMetrics,
        weighted_score: u128,
    ) -> RoutingResult<Self> {
        metrics.validate()?;

        Ok(Self {
            metrics,
            weighted_score: Some(weighted_score),
        })
    }

    /// Returns the number of inserted SWAP operations.
    #[must_use]
    pub const fn swap_count(self) -> u64 {
        self.metrics.swap_count
    }

    /// Returns resulting circuit depth.
    #[must_use]
    pub const fn depth(self) -> u64 {
        self.metrics.depth
    }

    /// Returns estimated duration in nanoseconds.
    #[must_use]
    pub const fn duration_ns(self) -> u64 {
        self.metrics.duration_ns
    }

    /// Returns estimated error in parts per billion.
    #[must_use]
    pub const fn error_ppb(self) -> u64 {
        self.metrics.error_ppb
    }

    /// Returns estimated fidelity in parts per billion.
    #[must_use]
    pub const fn fidelity_ppb(self) -> u64 {
        self.metrics.fidelity_ppb
    }

    /// Returns the weighted score if available.
    #[must_use]
    pub const fn weighted_score(self) -> Option<u128> {
        self.weighted_score
    }

    /// Compares two costs under a routing objective.
    pub fn compare_for(
        &self,
        other: &Self,
        objective: RoutingObjective,
    ) -> Ordering {
        match objective {
            RoutingObjective::SwapCount => self
                .metrics
                .swap_count
                .cmp(&other.metrics.swap_count)
                .then_with(|| {
                    self.metrics
                        .depth
                        .cmp(&other.metrics.depth)
                })
                .then_with(|| {
                    self.metrics
                        .duration_ns
                        .cmp(&other.metrics.duration_ns)
                })
                .then_with(|| {
                    self.metrics
                        .error_ppb
                        .cmp(&other.metrics.error_ppb)
                }),

            RoutingObjective::Depth => self
                .metrics
                .depth
                .cmp(&other.metrics.depth)
                .then_with(|| {
                    self.metrics
                        .swap_count
                        .cmp(&other.metrics.swap_count)
                })
                .then_with(|| {
                    self.metrics
                        .duration_ns
                        .cmp(&other.metrics.duration_ns)
                })
                .then_with(|| {
                    self.metrics
                        .error_ppb
                        .cmp(&other.metrics.error_ppb)
                }),

            RoutingObjective::Duration => self
                .metrics
                .duration_ns
                .cmp(&other.metrics.duration_ns)
                .then_with(|| {
                    self.metrics
                        .swap_count
                        .cmp(&other.metrics.swap_count)
                })
                .then_with(|| {
                    self.metrics.depth.cmp(&other.metrics.depth)
                })
                .then_with(|| {
                    self.metrics.error_ppb.cmp(&other.metrics.error_ppb)
                }),

            RoutingObjective::Error => self
                .metrics
                .error_ppb
                .cmp(&other.metrics.error_ppb)
                .then_with(|| {
                    self.metrics
                        .swap_count
                        .cmp(&other.metrics.swap_count)
                })
                .then_with(|| {
                    self.metrics
                        .duration_ns
                        .cmp(&other.metrics.duration_ns)
                })
                .then_with(|| {
                    self.metrics.depth.cmp(&other.metrics.depth)
                }),

            RoutingObjective::Fidelity => other
                .metrics
                .fidelity_ppb
                .cmp(&self.metrics.fidelity_ppb)
                .then_with(|| {
                    self.metrics
                        .swap_count
                        .cmp(&other.metrics.swap_count)
                })
                .then_with(|| {
                    self.metrics
                        .duration_ns
                        .cmp(&other.metrics.duration_ns)
                })
                .then_with(|| {
                    self.metrics.depth.cmp(&other.metrics.depth)
                }),

            RoutingObjective::Weighted => {
                match (self.weighted_score, other.weighted_score) {
                    (Some(left), Some(right)) => left
                        .cmp(&right)
                        .then_with(|| {
                            self.metrics
                                .swap_count
                                .cmp(&other.metrics.swap_count)
                        })
                        .then_with(|| {
                            self.metrics.depth.cmp(&other.metrics.depth)
                        }),

                    (None, None) => self
                        .metrics
                        .swap_count
                        .cmp(&other.metrics.swap_count)
                        .then_with(|| {
                            self.metrics.depth.cmp(&other.metrics.depth)
                        }),

                    (None, Some(_)) => Ordering::Greater,
                    (Some(_), None) => Ordering::Less,
                }
            }

            RoutingObjective::Custom(_) => {
                // Custom objective implementations must be used through
                // `CostModel::compare`. There is no safe universal ordering
                // for an opaque custom objective.
                self.total_order().cmp(&other.total_order())
            }
        }
    }

    /// Returns a deterministic total order independent of the selected
    /// objective.
    #[must_use]
    pub fn total_order(&self) -> (
        u64,
        u64,
        u64,
        u64,
        u64,
        u64,
        u64,
        u64,
    ) {
        (
            self.metrics.swap_count,
            self.metrics.movement_count,
            self.metrics.depth,
            self.metrics.duration_ns,
            self.metrics.error_ppb,
            self.metrics.fidelity_ppb,
            self.metrics.interaction_distance,
            self.weighted_score.unwrap_or(u128::MAX) as u64,
        )
    }

    /// Returns whether this cost is strictly better than another cost under
    /// the requested objective.
    #[must_use]
    pub fn better_than(
        &self,
        other: &Self,
        objective: RoutingObjective,
    ) -> bool {
        self.compare_for(other, objective) == Ordering::Less
    }

    /// Returns whether the two costs are equivalent under the requested
    /// objective.
    #[must_use]
    pub fn equivalent_to(
        &self,
        other: &Self,
        objective: RoutingObjective,
    ) -> bool {
        self.compare_for(other, objective) == Ordering::Equal
    }
}

impl Ord for RoutingCost {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total_order().cmp(&other.total_order())
    }
}

impl PartialOrd for RoutingCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// =============================================================================
// Weighted objective configuration
// =============================================================================

/// Fixed-point weights for multi-objective routing.
///
/// All weights use `WEIGHT_SCALE`.
///
/// Example:
///
/// ```text
/// swap_weight = 1.0
/// depth_weight = 0.5
/// duration_weight = 0.1
/// error_weight = 2.0
/// fidelity_weight = 0.0
/// ```
///
/// is represented as:
///
/// ```text
/// 1_000_000
///   500_000
///   100_000
/// 2_000_000
///        0
/// ```
///
/// The use of `u64` plus checked `u128` multiplication keeps weighted
/// evaluation deterministic and overflow-safe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CostWeights {
    /// SWAP-count weight.
    pub swap: u64,

    /// Depth weight.
    pub depth: u64,

    /// Duration weight.
    pub duration: u64,

    /// Error weight.
    pub error: u64,

    /// Fidelity weight.
    pub fidelity: u64,

    /// Interaction-distance weight.
    pub interaction_distance: u64,
}

impl Default for CostWeights {
    fn default() -> Self {
        Self {
            swap: WEIGHT_SCALE,
            depth: 0,
            duration: 0,
            error: 0,
            fidelity: 0,
            interaction_distance: 0,
        }
    }
}

impl CostWeights {
    /// Creates zero weights.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            swap: 0,
            depth: 0,
            duration: 0,
            error: 0,
            fidelity: 0,
            interaction_distance: 0,
        }
    }

    /// Creates weights from floating-point values using checked conversion.
    #[must_use]
    pub fn from_f64(
        swap: f64,
        depth: f64,
        duration: f64,
        error: f64,
        fidelity: f64,
        interaction_distance: f64,
    ) -> RoutingResult<Self> {
        let weights = Self {
            swap: weight_from_f64(swap)?,
            depth: weight_from_f64(depth)?,
            duration: weight_from_f64(duration)?,
            error: weight_from_f64(error)?,
            fidelity: weight_from_f64(fidelity)?,
            interaction_distance: weight_from_f64(
                interaction_distance,
            )?,
        };

        weights.validate()?;
        Ok(weights)
    }

    /// Validates the weight vector.
    pub fn validate(self) -> RoutingResult<()> {
        if self == Self::zero() {
            return Err(CostModelError::InvalidWeights {
                detail: "at least one routing cost weight must be non-zero"
                    .to_string(),
            }
            .into());
        }

        Ok(())
    }

    /// Calculates the weighted score.
    ///
    /// Fidelity is treated as a cost by using:
    ///
    /// ```text
    /// 1 - fidelity
    /// ```
    ///
    /// This means higher fidelity produces lower cost.
    pub fn score(self, metrics: RoutingMetrics) -> RoutingResult<u128> {
        metrics.validate()?;
        self.validate()?;

        let error = u128::from(metrics.error_ppb);
        let infidelity =
            u128::from(PROBABILITY_SCALE - metrics.fidelity_ppb);

        let terms = [
            (
                u128::from(self.swap),
                u128::from(metrics.swap_count),
                "swap",
            ),
            (
                u128::from(self.depth),
                u128::from(metrics.depth),
                "depth",
            ),
            (
                u128::from(self.duration),
                u128::from(metrics.duration_ns),
                "duration",
            ),
            (
                u128::from(self.error),
                error,
                "error",
            ),
            (
                u128::from(self.fidelity),
                infidelity,
                "fidelity",
            ),
            (
                u128::from(self.interaction_distance),
                u128::from(metrics.interaction_distance),
                "interaction_distance",
            ),
        ];

        let mut numerator = 0u128;

        for (weight, metric, name) in terms {
            let product = weight.checked_mul(metric).ok_or_else(|| {
                CostModelError::EvaluationFailed {
                    detail: format!(
                        "weighted {name} cost multiplication overflowed"
                    ),
                }
            })?;

            numerator = numerator.checked_add(product).ok_or_else(|| {
                CostModelError::EvaluationFailed {
                    detail: "weighted routing cost accumulation overflowed"
                        .to_string(),
                }
            })?;
        }

        Ok(numerator / u128::from(WEIGHT_SCALE))
    }
}

// =============================================================================
// Cost candidate
// =============================================================================

/// Immutable candidate supplied to a cost model.
///
/// Routing algorithms create this value while considering a possible movement
/// or resulting mapping.
///
/// The candidate contains no topology implementation details. Algorithms can
/// therefore build it from whichever topology representation Zamani adopts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CostCandidate {
    /// Metrics predicted after applying the candidate.
    pub metrics: RoutingMetrics,

    /// Optional heuristic distance of the front layer after the candidate.
    pub front_layer_distance: u64,

    /// Optional heuristic distance of the extended/lookahead set.
    pub extended_layer_distance: u64,

    /// Optional number of front-layer interactions.
    pub front_layer_size: u64,

    /// Optional number of extended-set interactions.
    pub extended_layer_size: u64,

    /// Optional decay factor applied to the candidate.
    pub decay_factor: u64,
}

impl CostCandidate {
    /// Creates a candidate from raw metrics.
    #[must_use]
    pub const fn new(metrics: RoutingMetrics) -> Self {
        Self {
            metrics,
            front_layer_distance: 0,
            extended_layer_distance: 0,
            front_layer_size: 0,
            extended_layer_size: 0,
            decay_factor: WEIGHT_SCALE,
        }
    }

    /// Creates a candidate containing SABRE front-layer information.
    #[must_use]
    pub const fn with_front_layer(
        mut self,
        distance: u64,
        size: u64,
    ) -> Self {
        self.front_layer_distance = distance;
        self.front_layer_size = size;
        self
    }

    /// Adds SABRE extended/lookahead information.
    #[must_use]
    pub const fn with_extended_layer(
        mut self,
        distance: u64,
        size: u64,
    ) -> Self {
        self.extended_layer_distance = distance;
        self.extended_layer_size = size;
        self
    }

    /// Adds a fixed-point decay factor.
    #[must_use]
    pub const fn with_decay_factor(mut self, factor: u64) -> Self {
        self.decay_factor = factor;
        self
    }

    /// Validates the candidate.
    pub fn validate(self) -> RoutingResult<()> {
        self.metrics.validate()?;

        if self.front_layer_size == 0
            && self.front_layer_distance != 0
        {
            return Err(CostModelError::InvalidMetric {
                metric:
                    "front_layer_distance cannot be non-zero when front_layer_size is zero"
                        .to_string(),
            }
            .into());
        }

        if self.extended_layer_size == 0
            && self.extended_layer_distance != 0
        {
            return Err(CostModelError::InvalidMetric {
                metric:
                    "extended_layer_distance cannot be non-zero when extended_layer_size is zero"
                        .to_string(),
            }
            .into());
        }

        Ok(())
    }
}

// =============================================================================
// Cost model trait
// =============================================================================

/// Stable routing cost-model extension point.
///
/// Algorithms should depend on this trait rather than directly depending on
/// a concrete cost model.
///
/// This makes it possible to add:
///
/// - vendor calibration models;
//! - research heuristics;
//! - distributed routing models;
//! - custom Danga routing policies;
/// - future hardware objectives;
///
/// without changing `router.rs` or the algorithm contracts.
pub trait CostModel: Send + Sync {
    /// Stable model name.
    fn name(&self) -> &str;

    /// Evaluates a candidate.
    fn evaluate(
        &self,
        candidate: &CostCandidate,
    ) -> RoutingResult<RoutingCost>;

    /// Compares two already evaluated costs.
    ///
    /// Implementations must return `Ordering::Less` when `left` is better.
    fn compare(
        &self,
        left: &RoutingCost,
        right: &RoutingCost,
    ) -> RoutingResult<Ordering>;

    /// Returns the primary objective.
    fn objective(&self) -> RoutingObjective;

    /// Returns whether this model is deterministic.
    fn deterministic(&self) -> bool {
        true
    }
}

// =============================================================================
// Objective cost model
// =============================================================================

/// Generic production cost model for the routing objectives declared in
/// `types.rs`.
#[derive(Debug, Clone, Copy)]
pub struct ObjectiveCostModel {
    objective: RoutingObjective,
    weights: CostWeights,
}

impl ObjectiveCostModel {
    /// Creates a model for the selected objective.
    #[must_use]
    pub const fn new(objective: RoutingObjective) -> Self {
        Self {
            objective,
            weights: CostWeights::zero(),
        }
    }

    /// Creates a weighted objective model.
    pub fn weighted(weights: CostWeights) -> RoutingResult<Self> {
        weights.validate()?;

        Ok(Self {
            objective: RoutingObjective::Weighted,
            weights,
        })
    }

    /// Returns the objective.
    #[must_use]
    pub const fn selected_objective(self) -> RoutingObjective {
        self.objective
    }

    /// Returns configured weights.
    #[must_use]
    pub const fn weights(self) -> CostWeights {
        self.weights
    }
}

impl CostModel for ObjectiveCostModel {
    fn name(&self) -> &str {
        match self.objective {
            RoutingObjective::SwapCount => "swap_count",
            RoutingObjective::Depth => "depth",
            RoutingObjective::Duration => "duration",
            RoutingObjective::Error => "error",
            RoutingObjective::Fidelity => "fidelity",
            RoutingObjective::Weighted => "weighted",
            RoutingObjective::Custom(_) => "custom",
        }
    }

    fn evaluate(
        &self,
        candidate: &CostCandidate,
    ) -> RoutingResult<RoutingCost> {
        candidate.validate()?;

        match self.objective {
            RoutingObjective::Weighted => {
                let score = self.weights.score(candidate.metrics)?;

                Ok(RoutingCost::with_weighted_score(
                    candidate.metrics,
                    score,
                )?)
            }

            RoutingObjective::Custom(_) => {
                Err(CostModelError::UnsupportedModel {
                    model: self.name().to_string(),
                }
                .into())
            }

            _ => RoutingCost::new(candidate.metrics),
        }
    }

    fn compare(
        &self,
        left: &RoutingCost,
        right: &RoutingCost,
    ) -> RoutingResult<Ordering> {
        Ok(left.compare_for(right, self.objective))
    }

    fn objective(&self) -> RoutingObjective {
        self.objective.clone()
    }
}

// =============================================================================
// SABRE heuristic
// =============================================================================

/// SABRE-style heuristic configuration.
///
/// The canonical SABRE heuristic is based on the distance of interacting
/// qubits in the front layer. Lookahead adds the extended set, and decay
/// penalizes recently used qubits/moves.
///
/// This type provides those components without coupling the implementation to
/// a particular topology or mapping representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SabreHeuristicConfig {
    /// Front-layer weight.
    pub front_weight: u64,

    /// Extended-set weight.
    pub extended_weight: u64,

    /// Whether to include the extended set.
    pub use_extended_set: bool,

    /// Whether to apply decay.
    pub use_decay: bool,

    /// Decay increment.
    pub decay_increment: u64,

    /// Fixed-point scale for all weights.
    pub weight_scale: u64,
}

impl Default for SabreHeuristicConfig {
    fn default() -> Self {
        Self {
            front_weight: DEFAULT_FRONT_LAYER_WEIGHT,
            extended_weight: DEFAULT_EXTENDED_SET_WEIGHT,
            use_extended_set: true,
            use_decay: false,
            decay_increment: DEFAULT_DECAY_INCREMENT,
            weight_scale: WEIGHT_SCALE,
        }
    }
}

impl SabreHeuristicConfig {
    /// Basic SABRE-style heuristic.
    #[must_use]
    pub const fn basic() -> Self {
        Self {
            front_weight: WEIGHT_SCALE,
            extended_weight: 0,
            use_extended_set: false,
            use_decay: false,
            decay_increment: 0,
            weight_scale: WEIGHT_SCALE,
        }
    }

    /// Lookahead SABRE-style heuristic.
    #[must_use]
    pub const fn lookahead() -> Self {
        Self {
            front_weight: WEIGHT_SCALE,
            extended_weight: DEFAULT_EXTENDED_SET_WEIGHT,
            use_extended_set: true,
            use_decay: false,
            decay_increment: 0,
            weight_scale: WEIGHT_SCALE,
        }
    }

    /// Decay SABRE-style heuristic.
    #[must_use]
    pub const fn decay() -> Self {
        Self {
            front_weight: WEIGHT_SCALE,
            extended_weight: DEFAULT_EXTENDED_SET_WEIGHT,
            use_extended_set: true,
            use_decay: true,
            decay_increment: DEFAULT_DECAY_INCREMENT,
            weight_scale: WEIGHT_SCALE,
        }
    }

    /// Validates the heuristic configuration.
    pub fn validate(self) -> RoutingResult<()> {
        if self.weight_scale == 0 {
            return Err(CostModelError::InvalidWeights {
                detail: "SABRE weight scale cannot be zero".to_string(),
            }
            .into());
        }

        if self.front_weight == 0 {
            return Err(CostModelError::InvalidWeights {
                detail: "SABRE front-layer weight cannot be zero".to_string(),
            }
            .into());
        }

        if self.use_extended_set
            && self.extended_weight == 0
        {
            return Err(CostModelError::InvalidWeights {
                detail:
                    "SABRE extended-set weight cannot be zero when lookahead is enabled"
                        .to_string(),
            }
            .into());
        }

        Ok(())
    }
}

/// SABRE heuristic cost model.
///
/// This model is intentionally separate from `ObjectiveCostModel` because
/// SABRE's heuristic is not a final circuit-quality metric. It is a search
/// heuristic used to rank candidate mappings.
///
/// This distinction prevents a compiler from accidentally reporting a SABRE
/// heuristic score as physical execution quality.
#[derive(Debug, Clone, Copy)]
pub struct SabreCostModel {
    config: SabreHeuristicConfig,
}

impl SabreCostModel {
    /// Creates a SABRE cost model.
    pub fn new(config: SabreHeuristicConfig) -> RoutingResult<Self> {
        config.validate()?;
        Ok(Self { config })
    }

    /// Returns the heuristic configuration.
    #[must_use]
    pub const fn config(self) -> SabreHeuristicConfig {
        self.config
    }

    /// Calculates the raw SABRE heuristic numerator.
    ///
    /// The result remains in fixed-point units and is not converted to a
    /// floating-point value.
    pub fn heuristic_score(
        &self,
        candidate: &CostCandidate,
    ) -> RoutingResult<u128> {
        candidate.validate()?;

        let front = if candidate.front_layer_size == 0 {
            0
        } else {
            u128::from(candidate.front_layer_distance)
                .checked_mul(u128::from(self.config.front_weight))
                .ok_or_else(|| CostModelError::EvaluationFailed {
                    detail:
                        "SABRE front-layer score overflowed".to_string(),
                })?
                / u128::from(candidate.front_layer_size)
        };

        let extended = if self.config.use_extended_set
            && candidate.extended_layer_size != 0
        {
            u128::from(candidate.extended_layer_distance)
                .checked_mul(u128::from(self.config.extended_weight))
                .ok_or_else(|| CostModelError::EvaluationFailed {
                    detail:
                        "SABRE extended-set score overflowed".to_string(),
                })?
                / u128::from(candidate.extended_layer_size)
        } else {
            0
        };

        let base = front.checked_add(extended).ok_or_else(|| {
            CostModelError::EvaluationFailed {
                detail: "SABRE heuristic score overflowed".to_string(),
            }
        })?;

        if !self.config.use_decay {
            return Ok(base);
        }

        let decay_factor = if candidate.decay_factor == 0 {
            WEIGHT_SCALE
        } else {
            candidate.decay_factor
        };

        base.checked_mul(u128::from(decay_factor))
            .ok_or_else(|| CostModelError::EvaluationFailed {
                detail: "SABRE decay score overflowed".to_string(),
            })
            .map(|value| value / u128::from(self.config.weight_scale))
    }
}

impl CostModel for SabreCostModel {
    fn name(&self) -> &str {
        if self.config.use_decay {
            "sabre_decay"
        } else if self.config.use_extended_set {
            "sabre_lookahead"
        } else {
            "sabre_basic"
        }
    }

    fn evaluate(
        &self,
        candidate: &CostCandidate,
    ) -> RoutingResult<RoutingCost> {
        let score = self.heuristic_score(candidate)?;

        RoutingCost::with_weighted_score(
            candidate.metrics,
            score,
        )
    }

    fn compare(
        &self,
        left: &RoutingCost,
        right: &RoutingCost,
    ) -> RoutingResult<Ordering> {
        let left_score =
            left.weighted_score.ok_or_else(|| {
                CostModelError::ComparisonFailed {
                    detail:
                        "left SABRE cost does not contain a heuristic score"
                            .to_string(),
                }
            })?;

        let right_score =
            right.weighted_score.ok_or_else(|| {
                CostModelError::ComparisonFailed {
                    detail:
                        "right SABRE cost does not contain a heuristic score"
                            .to_string(),
                }
            })?;

        Ok(left_score
            .cmp(&right_score)
            .then_with(|| {
                left.metrics.swap_count.cmp(
                    &right.metrics.swap_count,
                )
            })
            .then_with(|| {
                left.metrics.depth.cmp(&right.metrics.depth)
            }))
    }

    fn objective(&self) -> RoutingObjective {
        RoutingObjective::Custom(self.name().to_string())
    }
}

// =============================================================================
// Hardware metric provider
// =============================================================================

/// Hardware metrics supplied to the routing cost layer.
///
/// Hardware modules should translate their provider-specific calibration data
/// into this provider-neutral representation.
///
/// This keeps routing independent from IBM/IonQ/Quantinuum/Rigetti/etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct HardwareCostMetrics {
    /// Estimated operation duration in nanoseconds.
    pub duration_ns: u64,

    /// Estimated error probability in parts per billion.
    pub error_ppb: u64,

    /// Estimated fidelity in parts per billion.
    pub fidelity_ppb: u64,

    /// Whether duration is known.
    pub has_duration: bool,

    /// Whether error is known.
    pub has_error: bool,

    /// Whether fidelity is known.
    pub has_fidelity: bool,
}

impl HardwareCostMetrics {
    /// Creates complete hardware metrics.
    pub fn complete(
        duration_ns: u64,
        error_ppb: u64,
        fidelity_ppb: u64,
    ) -> RoutingResult<Self> {
        let metrics = Self {
            duration_ns,
            error_ppb,
            fidelity_ppb,
            has_duration: true,
            has_error: true,
            has_fidelity: true,
        };

        metrics.validate()?;
        Ok(metrics)
    }

    /// Validates known metric ranges.
    pub fn validate(self) -> RoutingResult<()> {
        if self.error_ppb > PROBABILITY_SCALE {
            return Err(CostModelError::InvalidMetric {
                metric: "hardware error_ppb exceeds 100%".to_string(),
            }
            .into());
        }

        if self.fidelity_ppb > PROBABILITY_SCALE {
            return Err(CostModelError::InvalidMetric {
                metric: "hardware fidelity_ppb exceeds 100%".to_string(),
            }
            .into());
        }

        Ok(())
    }

    /// Converts hardware metrics into routing metrics.
    pub fn apply_to(
        self,
        mut metrics: RoutingMetrics,
    ) -> RoutingResult<RoutingMetrics> {
        self.validate()?;

        if self.has_duration {
            metrics.duration_ns = self.duration_ns;
        }

        if self.has_error {
            metrics.error_ppb = self.error_ppb;
        }

        if self.has_fidelity {
            metrics.fidelity_ppb = self.fidelity_ppb;
        }

        metrics.validate()?;
        Ok(metrics)
    }
}

/// Stable provider-neutral hardware metric callback.
///
/// Implementations can be supplied by future calibration/instruction/timing
/// modules without changing this file.
pub trait CostMetricProvider: Send + Sync {
    /// Returns hardware metrics for one routing candidate.
    ///
    /// `operation_count` and `swap_count` are supplied by the routing algorithm
    /// so the provider can estimate accumulated hardware impact.
    fn metrics(
        &self,
        operation_count: u64,
        swap_count: u64,
    ) -> RoutingResult<HardwareCostMetrics>;
}

// =============================================================================
// Hardware-aware cost model
// =============================================================================

/// Hardware-aware objective model.
///
/// This model can optimize duration, error, fidelity, or a weighted mixture
/// while preserving deterministic integer comparison.
pub struct HardwareAwareCostModel<P> {
    provider: P,
    objective: RoutingObjective,
    weights: CostWeights,
}

impl<P> HardwareAwareCostModel<P>
where
    P: CostMetricProvider,
{
    /// Creates a hardware-aware model.
    pub fn new(
        provider: P,
        objective: RoutingObjective,
    ) -> RoutingResult<Self> {
        if !objective.is_hardware_aware() {
            return Err(CostModelError::UnsupportedModel {
                model: format!(
                    "hardware-aware model cannot use objective `{}`",
                    objective.name()
                ),
            }
            .into());
        }

        let weights = if matches!(
            objective,
            RoutingObjective::Weighted
        ) {
            return Err(CostModelError::InvalidWeights {
                detail:
                    "use `with_weights` for a weighted hardware-aware model"
                        .to_string(),
            }
            .into());
        } else {
            CostWeights::default()
        };

        Ok(Self {
            provider,
            objective,
            weights,
        })
    }

    /// Creates a weighted hardware-aware model.
    pub fn with_weights(
        provider: P,
        weights: CostWeights,
    ) -> RoutingResult<Self> {
        weights.validate()?;

        Ok(Self {
            provider,
            objective: RoutingObjective::Weighted,
            weights,
        })
    }

    /// Returns the underlying metric provider.
    #[must_use]
    pub const fn provider(&self) -> &P {
        &self.provider
    }
}

impl<P> CostModel for HardwareAwareCostModel<P>
where
    P: CostMetricProvider,
{
    fn name(&self) -> &str {
        match self.objective {
            RoutingObjective::Duration => "hardware_duration",
            RoutingObjective::Error => "hardware_error",
            RoutingObjective::Fidelity => "hardware_fidelity",
            RoutingObjective::Weighted => "hardware_weighted",
            _ => "hardware_custom",
        }
    }

    fn evaluate(
        &self,
        candidate: &CostCandidate,
    ) -> RoutingResult<RoutingCost> {
        candidate.validate()?;

        let hardware = self.provider.metrics(
            candidate.metrics.gate_count,
            candidate.metrics.swap_count,
        )?;

        let metrics = hardware.apply_to(candidate.metrics)?;

        match self.objective {
            RoutingObjective::Weighted => {
                let score = self.weights.score(metrics)?;

                RoutingCost::with_weighted_score(
                    metrics,
                    score,
                )
            }

            _ => RoutingCost::new(metrics),
        }
    }

    fn compare(
        &self,
        left: &RoutingCost,
        right: &RoutingCost,
    ) -> RoutingResult<Ordering> {
        Ok(left.compare_for(right, self.objective))
    }

    fn objective(&self) -> RoutingObjective {
        self.objective.clone()
    }
}

// =============================================================================
// Composite cost model
// =============================================================================

/// Combines a primary objective with deterministic tie-breakers.
///
/// This is useful when a router wants:
///
/// ```text
/// primary objective:
///     minimum duration
///
/// then:
///     minimum swaps
///
/// then:
///     minimum depth
///
/// then:
///     minimum error
/// ```
///
/// It avoids encoding tie-breakers into every individual algorithm.
#[derive(Debug, Clone)]
pub struct CompositeCostModel {
    objective: RoutingObjective,
    tie_breakers: Vec<RoutingObjective>,
    weights: Option<CostWeights>,
}

impl CompositeCostModel {
    /// Creates a composite objective.
    pub fn new(
        objective: RoutingObjective,
        tie_breakers: Vec<RoutingObjective>,
    ) -> RoutingResult<Self> {
        if tie_breakers.len() > 16 {
            return Err(CostModelError::InvalidWeights {
                detail:
                    "a composite cost model cannot contain more than 16 tie-breakers"
                        .to_string(),
            }
            .into());
        }

        Ok(Self {
            objective,
            tie_breakers,
            weights: None,
        })
    }

    /// Creates a weighted composite objective.
    pub fn weighted(
        weights: CostWeights,
        tie_breakers: Vec<RoutingObjective>,
    ) -> RoutingResult<Self> {
        weights.validate()?;

        if tie_breakers.len() > 16 {
            return Err(CostModelError::InvalidWeights {
                detail:
                    "a composite cost model cannot contain more than 16 tie-breakers"
                        .to_string(),
            }
            .into());
        }

        Ok(Self {
            objective: RoutingObjective::Weighted,
            tie_breakers,
            weights: Some(weights),
        })
    }

    fn compare_single(
        left: &RoutingCost,
        right: &RoutingCost,
        objective: RoutingObjective,
    ) -> Ordering {
        left.compare_for(right, objective)
    }
}

impl CostModel for CompositeCostModel {
    fn name(&self) -> &str {
        "composite"
    }

    fn evaluate(
        &self,
        candidate: &CostCandidate,
    ) -> RoutingResult<RoutingCost> {
        candidate.validate()?;

        match self.weights {
            Some(weights) => {
                let score = weights.score(candidate.metrics)?;

                RoutingCost::with_weighted_score(
                    candidate.metrics,
                    score,
                )
            }

            None => RoutingCost::new(candidate.metrics),
        }
    }

    fn compare(
        &self,
        left: &RoutingCost,
        right: &RoutingCost,
    ) -> RoutingResult<Ordering> {
        let first = Self::compare_single(
            left,
            right,
            self.objective,
        );

        if first != Ordering::Equal {
            return Ok(first);
        }

        for objective in &self.tie_breakers {
            let ordering = Self::compare_single(
                left,
                right,
                objective.clone(),
            );

            if ordering != Ordering::Equal {
                return Ok(ordering);
            }
        }

        Ok(left.total_order().cmp(&right.total_order()))
    }

    fn objective(&self) -> RoutingObjective {
        self.objective.clone()
    }
}

// =============================================================================
// Cost-model factory helpers
// =============================================================================

/// Creates the standard deterministic model for an objective.
pub fn model_for_objective(
    objective: RoutingObjective,
) -> RoutingResult<Box<dyn CostModel>> {
    match objective {
        RoutingObjective::Custom(name) => {
            Err(CostModelError::UnsupportedModel {
                model: name,
            }
            .into())
        }

        RoutingObjective::Weighted => {
            Err(CostModelError::InvalidWeights {
                detail:
                    "weighted objective requires explicit CostWeights"
                        .to_string(),
            }
            .into())
        }

        objective => Ok(Box::new(
            ObjectiveCostModel::new(objective),
        )),
    }
}

/// Creates a weighted deterministic model.
pub fn weighted_model(
    weights: CostWeights,
) -> RoutingResult<Box<dyn CostModel>> {
    Ok(Box::new(
        ObjectiveCostModel::weighted(weights)?,
    ))
}

/// Creates a basic SABRE model.
pub fn sabre_basic_model() -> RoutingResult<Box<dyn CostModel>> {
    Ok(Box::new(
        SabreCostModel::new(
            SabreHeuristicConfig::basic(),
        )?,
    ))
}

/// Creates a lookahead SABRE model.
pub fn sabre_lookahead_model() -> RoutingResult<Box<dyn CostModel>> {
    Ok(Box::new(
        SabreCostModel::new(
            SabreHeuristicConfig::lookahead(),
        )?,
    ))
}

/// Creates a decay SABRE model.
pub fn sabre_decay_model() -> RoutingResult<Box<dyn CostModel>> {
    Ok(Box::new(
        SabreCostModel::new(
            SabreHeuristicConfig::decay(),
        )?,
    ))
}

// =============================================================================
// Routing cost statistics
// =============================================================================

/// Aggregate statistics over a collection of routing candidates.
///
/// This is useful for benchmarking and algorithm diagnostics without making
/// benchmarking a dependency of the routing subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CostStatistics {
    /// Number of evaluated candidates.
    pub evaluated: u64,

    /// Number of candidates rejected due to invalid metrics.
    pub rejected: u64,

    /// Best observed SWAP count.
    pub best_swap_count: Option<u64>,

    /// Best observed depth.
    pub best_depth: Option<u64>,

    /// Best observed duration.
    pub best_duration_ns: Option<u64>,

    /// Best observed error.
    pub best_error_ppb: Option<u64>,

    /// Best observed fidelity.
    pub best_fidelity_ppb: Option<u64>,
}

impl CostStatistics {
    /// Creates empty statistics.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            evaluated: 0,
            rejected: 0,
            best_swap_count: None,
            best_depth: None,
            best_duration_ns: None,
            best_error_ppb: None,
            best_fidelity_ppb: None,
        }
    }

    /// Records a successfully evaluated cost.
    pub fn record(&mut self, cost: RoutingCost) {
        self.evaluated = self.evaluated.saturating_add(1);

        self.best_swap_count = Some(
            self.best_swap_count
                .map_or(cost.swap_count(), |best| {
                    best.min(cost.swap_count())
                }),
        );

        self.best_depth =
            Some(self.best_depth.map_or(cost.depth(), |best| {
                best.min(cost.depth())
            }));

        self.best_duration_ns = Some(
            self.best_duration_ns
                .map_or(cost.duration_ns(), |best| {
                    best.min(cost.duration_ns())
                }),
        );

        self.best_error_ppb =
            Some(self.best_error_ppb.map_or(cost.error_ppb(), |best| {
                best.min(cost.error_ppb())
            }));

        self.best_fidelity_ppb = Some(
            self.best_fidelity_ppb
                .map_or(cost.fidelity_ppb(), |best| {
                    best.max(cost.fidelity_ppb())
                }),
        );
    }

    /// Records a rejected candidate.
    pub fn record_rejection(&mut self) {
        self.rejected = self.rejected.saturating_add(1);
    }
}

// =============================================================================
// Deterministic candidate selector
// =============================================================================

/// Selects the best candidate according to a cost model.
///
/// The function does not mutate the supplied candidates.
///
/// Ties are resolved by the stable candidate index, which makes selection
/// deterministic even when two candidates have identical costs.
pub fn select_best<'a>(
    candidates: &'a [RoutingCost],
    model: &dyn CostModel,
) -> RoutingResult<Option<(usize, &'a RoutingCost)>> {
    let mut best: Option<(usize, &'a RoutingCost)> = None;

    for (index, candidate) in candidates.iter().enumerate() {
        match best {
            None => {
                best = Some((index, candidate));
            }

            Some((best_index, best_candidate)) => {
                let ordering =
                    model.compare(candidate, best_candidate)?;

                if ordering == Ordering::Less
                    || (ordering == Ordering::Equal
                        && index < best_index)
                {
                    best = Some((index, candidate));
                }
            }
        }
    }

    Ok(best)
}

// =============================================================================
// Algorithm identity helpers
// =============================================================================

/// Returns whether an algorithm normally benefits from a heuristic cost model.
#[must_use]
pub const fn algorithm_prefers_heuristic(
    algorithm: &RoutingAlgorithm,
) -> bool {
    matches!(
        algorithm,
        RoutingAlgorithm::Basic
            | RoutingAlgorithm::Lookahead
            | RoutingAlgorithm::Sabre
            | RoutingAlgorithm::NoiseAware
            | RoutingAlgorithm::Dynamic
            | RoutingAlgorithm::Auto
    )
}

/// Returns whether an objective requires hardware-quality information.
#[must_use]
pub const fn objective_requires_hardware(
    objective: &RoutingObjective,
) -> bool {
    matches!(
        objective,
        RoutingObjective::Duration
            | RoutingObjective::Error
            | RoutingObjective::Fidelity
    )
}

/// Returns whether an objective requires explicit caller-supplied weights.
#[must_use]
pub const fn objective_requires_weights(
    objective: &RoutingObjective,
) -> bool {
    matches!(objective, RoutingObjective::Weighted)
}

// =============================================================================
// Unit tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Fixed-point conversion
    // -------------------------------------------------------------------------

    #[test]
    fn probability_conversion_is_exact_for_common_values() {
        assert_eq!(
            probability_to_ppb(0.0).expect("zero probability"),
            0
        );

        assert_eq!(
            probability_to_ppb(0.5).expect("half probability"),
            500_000_000
        );

        assert_eq!(
            probability_to_ppb(1.0).expect("full probability"),
            1_000_000_000
        );
    }

    #[test]
    fn probability_conversion_rejects_invalid_values() {
        assert!(probability_to_ppb(-0.1).is_err());
        assert!(probability_to_ppb(1.1).is_err());
        assert!(probability_to_ppb(f64::NAN).is_err());
        assert!(probability_to_ppb(f64::INFINITY).is_err());
        assert!(probability_to_ppb(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn weight_conversion_is_checked() {
        assert_eq!(
            weight_from_f64(1.0).expect("unit weight"),
            WEIGHT_SCALE
        );

        assert_eq!(
            weight_from_f64(0.5).expect("half weight"),
            500_000
        );

        assert!(weight_from_f64(-1.0).is_err());
        assert!(weight_from_f64(f64::NAN).is_err());
    }

    // -------------------------------------------------------------------------
    // RoutingMetrics
    // -------------------------------------------------------------------------

    #[test]
    fn metrics_validate_probability_ranges() {
        let valid = RoutingMetrics {
            error_ppb: PROBABILITY_SCALE,
            fidelity_ppb: PROBABILITY_SCALE,
            ..RoutingMetrics::zero()
        };

        assert!(valid.validate().is_ok());

        let invalid_error = RoutingMetrics {
            error_ppb: PROBABILITY_SCALE + 1,
            ..RoutingMetrics::zero()
        };

        assert!(invalid_error.validate().is_err());

        let invalid_fidelity = RoutingMetrics {
            fidelity_ppb: PROBABILITY_SCALE + 1,
            ..RoutingMetrics::zero()
        };

        assert!(invalid_fidelity.validate().is_err());
    }

    #[test]
    fn metric_addition_is_checked() {
        let left = RoutingMetrics {
            swap_count: 2,
            depth: 3,
            duration_ns: 10,
            ..RoutingMetrics::zero()
        };

        let right = RoutingMetrics {
            swap_count: 4,
            depth: 5,
            duration_ns: 20,
            ..RoutingMetrics::zero()
        };

        let result = left.checked_add(right).expect("metric addition");

        assert_eq!(result.swap_count, 6);
        assert_eq!(result.depth, 8);
        assert_eq!(result.duration_ns, 30);
    }

    // -------------------------------------------------------------------------
    // Objective ordering
    // -------------------------------------------------------------------------

    #[test]
    fn swap_count_objective_prefers_fewer_swaps() {
        let a = RoutingCost::new(RoutingMetrics {
            swap_count: 2,
            depth: 10,
            ..RoutingMetrics::zero()
        })
        .expect("cost");

        let b = RoutingCost::new(RoutingMetrics {
            swap_count: 3,
            depth: 2,
            ..RoutingMetrics::zero()
        })
        .expect("cost");

        assert!(a.better_than(&b, RoutingObjective::SwapCount));
    }

    #[test]
    fn depth_objective_prefers_lower_depth() {
        let a = RoutingCost::new(RoutingMetrics {
            swap_count: 5,
            depth: 2,
            ..RoutingMetrics::zero()
        })
        .expect("cost");

        let b = RoutingCost::new(RoutingMetrics {
            swap_count: 1,
            depth: 4,
            ..RoutingMetrics::zero()
        })
        .expect("cost");

        assert!(a.better_than(&b, RoutingObjective::Depth));
    }

    #[test]
    fn duration_objective_prefers_shorter_duration() {
        let a = RoutingCost::new(RoutingMetrics {
            duration_ns: 100,
            ..RoutingMetrics::zero()
        })
        .expect("cost");

        let b = RoutingCost::new(RoutingMetrics {
            duration_ns: 200,
            ..RoutingMetrics::zero()
        })
        .expect("cost");

        assert!(a.better_than(&b, RoutingObjective::Duration));
    }

    #[test]
    fn error_objective_prefers_lower_error() {
        let a = RoutingCost::new(RoutingMetrics {
            error_ppb: 10,
            ..RoutingMetrics::zero()
        })
        .expect("cost");

        let b = RoutingCost::new(RoutingMetrics {
            error_ppb: 20,
            ..RoutingMetrics::zero()
        })
        .expect("cost");

        assert!(a.better_than(&b, RoutingObjective::Error));
    }

    #[test]
    fn fidelity_objective_prefers_higher_fidelity() {
        let a = RoutingCost::new(RoutingMetrics {
            fidelity_ppb: 900_000_000,
            ..RoutingMetrics::zero()
        })
        .expect("cost");

        let b = RoutingCost::new(RoutingMetrics {
            fidelity_ppb: 800_000_000,
            ..RoutingMetrics::zero()
        })
        .expect("cost");

        assert!(a.better_than(&b, RoutingObjective::Fidelity));
    }

    // -------------------------------------------------------------------------
    // Weighted cost
    // -------------------------------------------------------------------------

    #[test]
    fn weighted_cost_is_deterministic() {
        let weights = CostWeights::from_f64(
            1.0,
            2.0,
            0.5,
            3.0,
            0.0,
            0.25,
        )
        .expect("weights");

        let metrics = RoutingMetrics {
            swap_count: 2,
            depth: 4,
            duration_ns: 100,
            error_ppb: 10,
            fidelity_ppb: 900_000_000,
            interaction_distance: 8,
            ..RoutingMetrics::zero()
        };

        let first = weights.score(metrics).expect("first score");
        let second = weights.score(metrics).expect("second score");

        assert_eq!(first, second);
    }

    #[test]
    fn weighted_cost_uses_infidelity_for_fidelity_weight() {
        let weights = CostWeights {
            swap: 0,
            depth: 0,
            duration: 0,
            error: 0,
            fidelity: WEIGHT_SCALE,
            interaction_distance: 0,
        };

        let high_fidelity = RoutingMetrics {
            fidelity_ppb: 900_000_000,
            ..RoutingMetrics::zero()
        };

        let low_fidelity = RoutingMetrics {
            fidelity_ppb: 800_000_000,
            ..RoutingMetrics::zero()
        };

        let high_score =
            weights.score(high_fidelity).expect("high score");

        let low_score =
            weights.score(low_fidelity).expect("low score");

        assert!(high_score < low_score);
    }

    // -------------------------------------------------------------------------
    // Candidate validation
    // -------------------------------------------------------------------------

    #[test]
    fn candidate_rejects_distance_without_layer() {
        let candidate = CostCandidate {
            front_layer_distance: 1,
            front_layer_size: 0,
            ..CostCandidate::new(RoutingMetrics::zero())
        };

        assert!(candidate.validate().is_err());
    }

    #[test]
    fn candidate_accepts_valid_front_layer() {
        let candidate = CostCandidate::new(
            RoutingMetrics::zero(),
        )
        .with_front_layer(4, 2);

        assert!(candidate.validate().is_ok());
    }

    // -------------------------------------------------------------------------
    // SABRE
    // -------------------------------------------------------------------------

    #[test]
    fn sabre_basic_uses_front_layer() {
        let model =
            SabreCostModel::new(
                SabreHeuristicConfig::basic(),
            )
            .expect("SABRE model");

        let candidate = CostCandidate::new(
            RoutingMetrics::zero(),
        )
        .with_front_layer(10, 2);

        let score =
            model.heuristic_score(&candidate)
                .expect("SABRE score");

        assert_eq!(score, 5 * u128::from(WEIGHT_SCALE));
    }

    #[test]
    fn sabre_lookahead_includes_extended_layer() {
        let model =
            SabreCostModel::new(
                SabreHeuristicConfig::lookahead(),
            )
            .expect("SABRE model");

        let candidate = CostCandidate::new(
            RoutingMetrics::zero(),
        )
        .with_front_layer(10, 2)
        .with_extended_layer(20, 2);

        let score =
            model.heuristic_score(&candidate)
                .expect("SABRE score");

        let expected = 5 * u128::from(WEIGHT_SCALE)
            + 10 * u128::from(DEFAULT_EXTENDED_SET_WEIGHT);

        assert_eq!(score, expected);
    }

    #[test]
    fn sabre_decay_increases_score_when_decay_factor_increases() {
        let model =
            SabreCostModel::new(
                SabreHeuristicConfig::decay(),
            )
            .expect("SABRE model");

        let candidate =
            CostCandidate::new(RoutingMetrics::zero())
                .with_front_layer(10, 2)
                .with_decay_factor(WEIGHT_SCALE);

        let normal =
            model.heuristic_score(&candidate)
                .expect("normal score");

        let penalized =
            model
                .heuristic_score(
                    &candidate.with_decay_factor(
                        WEIGHT_SCALE + DEFAULT_DECAY_INCREMENT,
                    ),
                )
                .expect("penalized score");

        assert!(penalized > normal);
    }

    // -------------------------------------------------------------------------
    // Model selection
    // -------------------------------------------------------------------------

    #[test]
    fn select_best_is_deterministic() {
        let model =
            ObjectiveCostModel::new(
                RoutingObjective::SwapCount,
            );

        let first = RoutingCost::new(
            RoutingMetrics {
                swap_count: 1,
                ..RoutingMetrics::zero()
            },
        )
        .expect("first");

        let second = RoutingCost::new(
            RoutingMetrics {
                swap_count: 2,
                ..RoutingMetrics::zero()
            },
        )
        .expect("second");

        let candidates = [first, second];

        let selected =
            select_best(&candidates, &model)
                .expect("selection")
                .expect("candidate");

        assert_eq!(selected.0, 0);
    }

    #[test]
    fn model_for_weighted_requires_weights() {
        assert!(
            model_for_objective(
                RoutingObjective::Weighted
            )
            .is_err()
        );
    }

    #[test]
    fn hardware_metric_ranges_are_checked() {
        assert!(
            HardwareCostMetrics::complete(
                100,
                10,
                999_999_990,
            )
            .is_ok()
        );

        assert!(
            HardwareCostMetrics::complete(
                100,
                PROBABILITY_SCALE + 1,
                1,
            )
            .is_err()
        );
    }

    // -------------------------------------------------------------------------
    // Statistics
    // -------------------------------------------------------------------------

    #[test]
    fn statistics_track_best_values() {
        let mut statistics =
            CostStatistics::new();

        statistics.record(
            RoutingCost::new(
                RoutingMetrics {
                    swap_count: 4,
                    depth: 10,
                    duration_ns: 100,
                    error_ppb: 20,
                    fidelity_ppb: 900_000_000,
                    ..RoutingMetrics::zero()
                },
            )
            .expect("cost"),
        );

        statistics.record(
            RoutingCost::new(
                RoutingMetrics {
                    swap_count: 2,
                    depth: 5,
                    duration_ns: 80,
                    error_ppb: 10,
                    fidelity_ppb: 950_000_000,
                    ..RoutingMetrics::zero()
                },
            )
            .expect("cost"),
        );

        assert_eq!(
            statistics.best_swap_count,
            Some(2)
        );
        assert_eq!(
            statistics.best_depth,
            Some(5)
        );
        assert_eq!(
            statistics.best_duration_ns,
            Some(80)
        );
        assert_eq!(
            statistics.best_error_ppb,
            Some(10)
        );
        assert_eq!(
            statistics.best_fidelity_ppb,
            Some(950_000_000)
        );
    }
}