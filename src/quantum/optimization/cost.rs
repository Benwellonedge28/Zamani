//! Zamani Quantum Optimization — Cost Model
//!
//! Production-grade, backend-independent resource and objective modelling for
//! quantum-circuit optimization.
//!
//! # Architectural position
//!
//! ```text
//!                     quantum::ir
//!                          │
//!                          ▼
//!                 optimization::cost
//!                          │
//!             ┌────────────┼────────────┐
//!             ▼            ▼            ▼
//!          analyses      passes       planner
//!             │            │            │
//!             └────────────┼────────────┘
//!                          ▼
//!                   cost comparison
//!                          │
//!                          ▼
//!                    verification
//! ```
//!
//! This module owns:
//!
//! - optimization resource accounting;
//! - optimization objectives;
//! - multi-objective cost comparison;
//! - weighted objectives;
//! - lexicographic objectives;
//! - Pareto comparison;
//! - logical resource costs;
//! - hardware-independent estimated costs;
//! - target-sensitive cost configuration;
//! - overflow-safe resource accumulation;
//! - deterministic cost comparison;
//! - cost deltas between circuits;
//! - cost-model validation.
//!
//! This module deliberately does NOT own:
//!
//! - quantum circuit semantics;
//! - Quantum IR definitions;
//! - hardware topology;
//! - routing;
//! - scheduling;
//! - hardware execution;
//! - calibration acquisition;
//! - optimization passes;
//! - analysis caches;
//! - optimization configuration;
//! - serialization;
//! - benchmarking orchestration.
//!
//! Those responsibilities belong to their owning subsystems.
//!
//! # Canonical IR rule
//!
//! The canonical quantum representation is:
//!
//! `crate::quantum::ir`
//!
//! This file never defines another `QuantumGate`, `QuantumOperation`, or
//! circuit representation.
//!
//! # Resource model
//!
//! Quantum optimization is inherently multi-objective. Minimizing ordinary
//! gate count can be actively harmful when it increases two-qubit gates,
//! circuit depth, T-count, execution time, or estimated error.
//!
//! Consequently this module keeps the following dimensions independently:
//!
//! - total operations;
//! - single-qubit operations;
//! - two-qubit operations;
//! - multi-qubit operations;
//! - Clifford operations;
//! - non-Clifford operations;
//! - T operations;
//! - T-depth;
//! - circuit depth;
//! - two-qubit depth;
//! - measurements;
//! - resets;
//! - barriers;
//! - logical width;
//! - ancilla usage;
//! - estimated duration;
//! - estimated error;
//! - estimated energy;
//! - classical optimizer cost.
//!
//! # Numerical safety
//!
//! Integer resource counters use `u128` so very large circuits can be
//! represented without prematurely overflowing `usize`.
//!
//! Floating-point quantities are validated for finiteness. NaN and infinities
//! are rejected rather than silently contaminating optimization decisions.
//!
//! Floating-point ordering uses `total_cmp`, which is available on the stable
//! Rust toolchain targeted by Zamani.
//!
//! # No artificial "infinity"
//!
//! Zamani does not treat a resource value such as `usize::MAX` as a legitimate
//! unlimited circuit size. The IR and optimizer have explicit resource
//! policies. This module therefore supports arbitrarily large representable
//! `u128` accounting while allowing higher layers to impose actual limits.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features.
//! No unsafe code.
//! No external dependencies.
//!
//! # Integration contract
//!
//! Future optimization files should consume this module through:
//!
//! - [`CostModel`];
//! - [`CostVector`];
//! - [`CostDelta`];
//! - [`OptimizationObjective`];
//! - [`CostComparison`];
//! - [`CostError`];
//! - [`GateCost`];
//! - [`ResourceWeights`];
//!
//! No future optimization pass should define its own gate-count or depth-cost
//! structure.

use std::cmp::Ordering;
use std::fmt;

use crate::quantum::ir::GateKind;

// =============================================================================
// Result and errors
// =============================================================================

/// Result type used by the optimization cost subsystem.
pub type CostResult<T> = Result<T, CostError>;

/// Errors produced by the optimization cost subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CostError {
    /// A floating-point cost or weight is not finite.
    NonFiniteValue {
        /// Name of the affected field.
        field: &'static str,
    },

    /// A floating-point cost or weight is negative where negative values are
    /// not semantically meaningful.
    NegativeValue {
        /// Name of the affected field.
        field: &'static str,
    },

    /// An integer resource accumulation overflowed `u128`.
    ArithmeticOverflow {
        /// Name of the resource whose accumulation overflowed.
        resource: &'static str,
    },

    /// A weighted objective has no effective dimensions.
    EmptyObjective,

    /// A weighted objective contains a weight that is invalid.
    InvalidWeight {
        /// Name of the metric.
        metric: &'static str,
    },

    /// A supplied objective contains an invalid configuration.
    InvalidObjective {
        /// Description of the invalid configuration.
        message: &'static str,
    },

    /// A cost model is internally inconsistent.
    InvalidModel {
        /// Description of the invalid model.
        message: &'static str,
    },

    /// A cost vector contains a value that cannot participate in the requested
    /// calculation.
    InvalidCost {
        /// Name of the affected resource.
        resource: &'static str,
    },
}

impl fmt::Display for CostError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteValue { field } => {
                write!(f, "optimization cost field `{field}` must be finite")
            }

            Self::NegativeValue { field } => {
                write!(f, "optimization cost field `{field}` cannot be negative")
            }

            Self::ArithmeticOverflow { resource } => {
                write!(
                    f,
                    "optimization cost accumulation overflowed for `{resource}`"
                )
            }

            Self::EmptyObjective => {
                write!(f, "optimization objective contains no active dimensions")
            }

            Self::InvalidWeight { metric } => {
                write!(
                    f,
                    "optimization objective contains an invalid weight for `{metric}`"
                )
            }

            Self::InvalidObjective { message } => {
                write!(f, "invalid optimization objective: {message}")
            }

            Self::InvalidModel { message } => {
                write!(f, "invalid optimization cost model: {message}")
            }

            Self::InvalidCost { resource } => {
                write!(
                    f,
                    "invalid optimization cost value for `{resource}`"
                )
            }
        }
    }
}

impl std::error::Error for CostError {}

// =============================================================================
// Resource metric
// =============================================================================

/// A resource dimension that can participate in an optimization objective.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CostMetric {
    /// Total logical quantum operations.
    GateCount,

    /// Single-qubit operations.
    SingleQubitGateCount,

    /// Two-qubit operations.
    TwoQubitGateCount,

    /// Operations acting on three or more qubits.
    MultiQubitGateCount,

    /// Clifford operations.
    CliffordGateCount,

    /// Non-Clifford operations.
    NonCliffordGateCount,

    /// T gates.
    TCount,

    /// T-depth.
    TDepth,

    /// Total logical circuit depth.
    Depth,

    /// Depth considering only two-qubit operations.
    TwoQubitDepth,

    /// Number of measurements.
    MeasurementCount,

    /// Number of resets.
    ResetCount,

    /// Number of barriers.
    BarrierCount,

    /// Logical qubit width.
    QubitCount,

    /// Ancilla count.
    AncillaCount,

    /// Estimated execution duration.
    Duration,

    /// Estimated accumulated error.
    Error,

    /// Estimated energy/resource consumption.
    Energy,

    /// Estimated classical-side optimization cost.
    ClassicalCost,
}

impl CostMetric {
    /// Returns a stable machine-readable name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GateCount => "gate_count",
            Self::SingleQubitGateCount => "single_qubit_gate_count",
            Self::TwoQubitGateCount => "two_qubit_gate_count",
            Self::MultiQubitGateCount => "multi_qubit_gate_count",
            Self::CliffordGateCount => "clifford_gate_count",
            Self::NonCliffordGateCount => "non_clifford_gate_count",
            Self::TCount => "t_count",
            Self::TDepth => "t_depth",
            Self::Depth => "depth",
            Self::TwoQubitDepth => "two_qubit_depth",
            Self::MeasurementCount => "measurement_count",
            Self::ResetCount => "reset_count",
            Self::BarrierCount => "barrier_count",
            Self::QubitCount => "qubit_count",
            Self::AncillaCount => "ancilla_count",
            Self::Duration => "duration",
            Self::Error => "error",
            Self::Energy => "energy",
            Self::ClassicalCost => "classical_cost",
        }
    }

    /// Returns whether the metric is an integer-valued resource.
    #[must_use]
    pub const fn is_integer(self) -> bool {
        !matches!(
            self,
            Self::Duration
                | Self::Error
                | Self::Energy
                | Self::ClassicalCost
        )
    }
}

impl fmt::Display for CostMetric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Numeric resource value
// =============================================================================

/// Numeric value of one optimization resource dimension.
///
/// This abstraction prevents callers from mixing integer resource counters
/// with floating-point resource estimates accidentally.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CostValue {
    /// Exact integer resource.
    Integer(u128),

    /// Validated finite non-negative floating-point resource.
    Floating(f64),
}

impl CostValue {
    /// Creates an integer cost value.
    #[must_use]
    pub const fn integer(value: u128) -> Self {
        Self::Integer(value)
    }

    /// Creates a validated floating-point cost value.
    pub fn floating(value: f64) -> CostResult<Self> {
        if !value.is_finite() {
            return Err(CostError::NonFiniteValue {
                field: "cost value",
            });
        }

        if value < 0.0 {
            return Err(CostError::NegativeValue {
                field: "cost value",
            });
        }

        Ok(Self::Floating(value))
    }

    /// Returns the integer value if this is an integer resource.
    #[must_use]
    pub const fn as_integer(self) -> Option<u128> {
        match self {
            Self::Integer(value) => Some(value),
            Self::Floating(_) => None,
        }
    }

    /// Returns the floating-point value if this is a floating resource.
    #[must_use]
    pub const fn as_floating(self) -> Option<f64> {
        match self {
            Self::Integer(_) => None,
            Self::Floating(value) => Some(value),
        }
    }
}

// =============================================================================
// Gate-level cost
// =============================================================================

/// Cost contribution of one logical gate.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GateCost {
    /// Total operation contribution.
    pub gate_count: u128,

    /// Single-qubit contribution.
    pub single_qubit_gate_count: u128,

    /// Two-qubit contribution.
    pub two_qubit_gate_count: u128,

    /// Multi-qubit contribution.
    pub multi_qubit_gate_count: u128,

    /// Clifford contribution.
    pub clifford_gate_count: u128,

    /// Non-Clifford contribution.
    pub non_clifford_gate_count: u128,

    /// T contribution.
    pub t_count: u128,

    /// Measurement contribution.
    pub measurement_count: u128,

    /// Reset contribution.
    pub reset_count: u128,

    /// Barrier contribution.
    pub barrier_count: u128,

    /// Estimated duration contribution.
    pub duration: f64,

    /// Estimated error contribution.
    pub error: f64,

    /// Estimated energy contribution.
    pub energy: f64,

    /// Estimated classical-side cost contribution.
    pub classical_cost: f64,
}

impl GateCost {
    /// Creates a zero cost.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            gate_count: 0,
            single_qubit_gate_count: 0,
            two_qubit_gate_count: 0,
            multi_qubit_gate_count: 0,
            clifford_gate_count: 0,
            non_clifford_gate_count: 0,
            t_count: 0,
            measurement_count: 0,
            reset_count: 0,
            barrier_count: 0,
            duration: 0.0,
            error: 0.0,
            energy: 0.0,
            classical_cost: 0.0,
        }
    }

    /// Creates a unit cost for an ordinary single-qubit gate.
    #[must_use]
    pub const fn single_qubit() -> Self {
        Self {
            gate_count: 1,
            single_qubit_gate_count: 1,
            ..Self::zero()
        }
    }

    /// Creates a unit cost for an ordinary two-qubit gate.
    #[must_use]
    pub const fn two_qubit() -> Self {
        Self {
            gate_count: 1,
            two_qubit_gate_count: 1,
            ..Self::zero()
        }
    }

    /// Creates a unit cost for an ordinary multi-qubit gate.
    #[must_use]
    pub const fn multi_qubit() -> Self {
        Self {
            gate_count: 1,
            multi_qubit_gate_count: 1,
            ..Self::zero()
        }
    }

    /// Creates a measurement cost.
    #[must_use]
    pub const fn measurement() -> Self {
        Self {
            gate_count: 1,
            measurement_count: 1,
            ..Self::zero()
        }
    }

    /// Creates a reset cost.
    #[must_use]
    pub const fn reset() -> Self {
        Self {
            gate_count: 1,
            reset_count: 1,
            ..Self::zero()
        }
    }

    /// Creates a barrier cost.
    #[must_use]
    pub const fn barrier() -> Self {
        Self {
            gate_count: 1,
            barrier_count: 1,
            ..Self::zero()
        }
    }

    /// Creates a Clifford gate cost.
    #[must_use]
    pub const fn clifford_single_qubit() -> Self {
        Self {
            gate_count: 1,
            single_qubit_gate_count: 1,
            clifford_gate_count: 1,
            ..Self::zero()
        }
    }

    /// Creates a Clifford two-qubit gate cost.
    #[must_use]
    pub const fn clifford_two_qubit() -> Self {
        Self {
            gate_count: 1,
            two_qubit_gate_count: 1,
            clifford_gate_count: 1,
            ..Self::zero()
        }
    }

    /// Creates a T-gate cost.
    #[must_use]
    pub const fn t_gate() -> Self {
        Self {
            gate_count: 1,
            single_qubit_gate_count: 1,
            non_clifford_gate_count: 1,
            t_count: 1,
            ..Self::zero()
        }
    }

    /// Adds another gate cost using checked integer arithmetic.
    pub fn checked_add(self, other: Self) -> CostResult<Self> {
        Ok(Self {
            gate_count: checked_add(
                self.gate_count,
                other.gate_count,
                "gate_count",
            )?,
            single_qubit_gate_count: checked_add(
                self.single_qubit_gate_count,
                other.single_qubit_gate_count,
                "single_qubit_gate_count",
            )?,
            two_qubit_gate_count: checked_add(
                self.two_qubit_gate_count,
                other.two_qubit_gate_count,
                "two_qubit_gate_count",
            )?,
            multi_qubit_gate_count: checked_add(
                self.multi_qubit_gate_count,
                other.multi_qubit_gate_count,
                "multi_qubit_gate_count",
            )?,
            clifford_gate_count: checked_add(
                self.clifford_gate_count,
                other.clifford_gate_count,
                "clifford_gate_count",
            )?,
            non_clifford_gate_count: checked_add(
                self.non_clifford_gate_count,
                other.non_clifford_gate_count,
                "non_clifford_gate_count",
            )?,
            t_count: checked_add(
                self.t_count,
                other.t_count,
                "t_count",
            )?,
            measurement_count: checked_add(
                self.measurement_count,
                other.measurement_count,
                "measurement_count",
            )?,
            reset_count: checked_add(
                self.reset_count,
                other.reset_count,
                "reset_count",
            )?,
            barrier_count: checked_add(
                self.barrier_count,
                other.barrier_count,
                "barrier_count",
            )?,
            duration: checked_float_add(
                self.duration,
                other.duration,
                "duration",
            )?,
            error: checked_float_add(
                self.error,
                other.error,
                "error",
            )?,
            energy: checked_float_add(
                self.energy,
                other.energy,
                "energy",
            )?,
            classical_cost: checked_float_add(
                self.classical_cost,
                other.classical_cost,
                "classical_cost",
            )?,
        })
    }

    /// Returns a saturated addition.
    ///
    /// This is useful for exploratory analysis where retaining the fact that a
    /// resource became enormous is more useful than aborting an entire
    /// optimization attempt.
    #[must_use]
    pub fn saturating_add(self, other: Self) -> Self {
        Self {
            gate_count: self.gate_count.saturating_add(other.gate_count),
            single_qubit_gate_count: self
                .single_qubit_gate_count
                .saturating_add(other.single_qubit_gate_count),
            two_qubit_gate_count: self
                .two_qubit_gate_count
                .saturating_add(other.two_qubit_gate_count),
            multi_qubit_gate_count: self
                .multi_qubit_gate_count
                .saturating_add(other.multi_qubit_gate_count),
            clifford_gate_count: self
                .clifford_gate_count
                .saturating_add(other.clifford_gate_count),
            non_clifford_gate_count: self
                .non_clifford_gate_count
                .saturating_add(other.non_clifford_gate_count),
            t_count: self.t_count.saturating_add(other.t_count),
            measurement_count: self
                .measurement_count
                .saturating_add(other.measurement_count),
            reset_count: self
                .reset_count
                .saturating_add(other.reset_count),
            barrier_count: self
                .barrier_count
                .saturating_add(other.barrier_count),
            duration: saturating_float_add(self.duration, other.duration),
            error: saturating_float_add(self.error, other.error),
            energy: saturating_float_add(self.energy, other.energy),
            classical_cost: saturating_float_add(
                self.classical_cost,
                other.classical_cost,
            ),
        }
    }
}

// =============================================================================
// Resource weights
// =============================================================================

/// Weight applied to one optimization resource.
///
/// All weights are non-negative and finite.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResourceWeights {
    /// Weight of ordinary gate count.
    pub gate_count: f64,

    /// Weight of single-qubit gates.
    pub single_qubit_gate_count: f64,

    /// Weight of two-qubit gates.
    pub two_qubit_gate_count: f64,

    /// Weight of multi-qubit gates.
    pub multi_qubit_gate_count: f64,

    /// Weight of Clifford gates.
    pub clifford_gate_count: f64,

    /// Weight of non-Clifford gates.
    pub non_clifford_gate_count: f64,

    /// Weight of T gates.
    pub t_count: f64,

    /// Weight of T depth.
    pub t_depth: f64,

    /// Weight of total depth.
    pub depth: f64,

    /// Weight of two-qubit depth.
    pub two_qubit_depth: f64,

    /// Weight of measurements.
    pub measurement_count: f64,

    /// Weight of resets.
    pub reset_count: f64,

    /// Weight of barriers.
    pub barrier_count: f64,

    /// Weight of logical qubit count.
    pub qubit_count: f64,

    /// Weight of ancilla count.
    pub ancilla_count: f64,

    /// Weight of estimated duration.
    pub duration: f64,

    /// Weight of estimated error.
    pub error: f64,

    /// Weight of estimated energy.
    pub energy: f64,

    /// Weight of classical optimization cost.
    pub classical_cost: f64,
}

impl ResourceWeights {
    /// Creates a zero-weight vector.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            gate_count: 0.0,
            single_qubit_gate_count: 0.0,
            two_qubit_gate_count: 0.0,
            multi_qubit_gate_count: 0.0,
            clifford_gate_count: 0.0,
            non_clifford_gate_count: 0.0,
            t_count: 0.0,
            t_depth: 0.0,
            depth: 0.0,
            two_qubit_depth: 0.0,
            measurement_count: 0.0,
            reset_count: 0.0,
            barrier_count: 0.0,
            qubit_count: 0.0,
            ancilla_count: 0.0,
            duration: 0.0,
            error: 0.0,
            energy: 0.0,
            classical_cost: 0.0,
        }
    }

    /// Standard gate-count objective.
    #[must_use]
    pub const fn gate_count() -> Self {
        Self {
            gate_count: 1.0,
            ..Self::zero()
        }
    }

    /// Standard circuit-depth objective.
    #[must_use]
    pub const fn depth() -> Self {
        Self {
            depth: 1.0,
            ..Self::zero()
        }
    }

    /// Standard two-qubit objective.
    #[must_use]
    pub const fn two_qubit() -> Self {
        Self {
            two_qubit_gate_count: 1.0,
            ..Self::zero()
        }
    }

    /// Standard fault-tolerant T-count objective.
    #[must_use]
    pub const fn t_count() -> Self {
        Self {
            t_count: 1.0,
            ..Self::zero()
        }
    }

    /// Standard fault-tolerant T-depth objective.
    #[must_use]
    pub const fn t_depth() -> Self {
        Self {
            t_depth: 1.0,
            ..Self::zero()
        }
    }

    /// Objective emphasizing practical two-qubit hardware cost.
    #[must_use]
    pub const fn hardware_balanced() -> Self {
        Self {
            gate_count: 1.0,
            two_qubit_gate_count: 10.0,
            depth: 2.0,
            two_qubit_depth: 5.0,
            duration: 1.0,
            error: 1.0,
            ..Self::zero()
        }
    }

    /// Objective emphasizing fault-tolerant resources.
    #[must_use]
    pub const fn fault_tolerant() -> Self {
        Self {
            gate_count: 1.0,
            two_qubit_gate_count: 2.0,
            t_count: 20.0,
            t_depth: 10.0,
            depth: 1.0,
            ..Self::zero()
        }
    }

    /// Returns the weight for one metric.
    #[must_use]
    pub const fn get(self, metric: CostMetric) -> f64 {
        match metric {
            CostMetric::GateCount => self.gate_count,
            CostMetric::SingleQubitGateCount => self.single_qubit_gate_count,
            CostMetric::TwoQubitGateCount => self.two_qubit_gate_count,
            CostMetric::MultiQubitGateCount => self.multi_qubit_gate_count,
            CostMetric::CliffordGateCount => self.clifford_gate_count,
            CostMetric::NonCliffordGateCount => self.non_clifford_gate_count,
            CostMetric::TCount => self.t_count,
            CostMetric::TDepth => self.t_depth,
            CostMetric::Depth => self.depth,
            CostMetric::TwoQubitDepth => self.two_qubit_depth,
            CostMetric::MeasurementCount => self.measurement_count,
            CostMetric::ResetCount => self.reset_count,
            CostMetric::BarrierCount => self.barrier_count,
            CostMetric::QubitCount => self.qubit_count,
            CostMetric::AncillaCount => self.ancilla_count,
            CostMetric::Duration => self.duration,
            CostMetric::Error => self.error,
            CostMetric::Energy => self.energy,
            CostMetric::ClassicalCost => self.classical_cost,
        }
    }

    /// Sets one weight and returns the modified value.
    ///
    /// Invalid floating-point values are rejected.
    pub fn with(self, metric: CostMetric, value: f64) -> CostResult<Self> {
        validate_weight(metric.as_str(), value)?;

        let mut result = self;

        match metric {
            CostMetric::GateCount => result.gate_count = value,
            CostMetric::SingleQubitGateCount => {
                result.single_qubit_gate_count = value
            }
            CostMetric::TwoQubitGateCount => {
                result.two_qubit_gate_count = value
            }
            CostMetric::MultiQubitGateCount => {
                result.multi_qubit_gate_count = value
            }
            CostMetric::CliffordGateCount => {
                result.clifford_gate_count = value
            }
            CostMetric::NonCliffordGateCount => {
                result.non_clifford_gate_count = value
            }
            CostMetric::TCount => result.t_count = value,
            CostMetric::TDepth => result.t_depth = value,
            CostMetric::Depth => result.depth = value,
            CostMetric::TwoQubitDepth => result.two_qubit_depth = value,
            CostMetric::MeasurementCount => result.measurement_count = value,
            CostMetric::ResetCount => result.reset_count = value,
            CostMetric::BarrierCount => result.barrier_count = value,
            CostMetric::QubitCount => result.qubit_count = value,
            CostMetric::AncillaCount => result.ancilla_count = value,
            CostMetric::Duration => result.duration = value,
            CostMetric::Error => result.error = value,
            CostMetric::Energy => result.energy = value,
            CostMetric::ClassicalCost => result.classical_cost = value,
        }

        Ok(result)
    }

    /// Validates every configured weight.
    pub fn validate(self) -> CostResult<()> {
        validate_weight("gate_count", self.gate_count)?;
        validate_weight(
            "single_qubit_gate_count",
            self.single_qubit_gate_count,
        )?;
        validate_weight(
            "two_qubit_gate_count",
            self.two_qubit_gate_count,
        )?;
        validate_weight(
            "multi_qubit_gate_count",
            self.multi_qubit_gate_count,
        )?;
        validate_weight(
            "clifford_gate_count",
            self.clifford_gate_count,
        )?;
        validate_weight(
            "non_clifford_gate_count",
            self.non_clifford_gate_count,
        )?;
        validate_weight("t_count", self.t_count)?;
        validate_weight("t_depth", self.t_depth)?;
        validate_weight("depth", self.depth)?;
        validate_weight("two_qubit_depth", self.two_qubit_depth)?;
        validate_weight("measurement_count", self.measurement_count)?;
        validate_weight("reset_count", self.reset_count)?;
        validate_weight("barrier_count", self.barrier_count)?;
        validate_weight("qubit_count", self.qubit_count)?;
        validate_weight("ancilla_count", self.ancilla_count)?;
        validate_weight("duration", self.duration)?;
        validate_weight("error", self.error)?;
        validate_weight("energy", self.energy)?;
        validate_weight("classical_cost", self.classical_cost)?;

        Ok(())
    }

    /// Returns whether at least one weight is non-zero.
    #[must_use]
    pub fn has_active_metric(self) -> bool {
        const EPSILON: f64 = 0.0;

        self.gate_count > EPSILON
            || self.single_qubit_gate_count > EPSILON
            || self.two_qubit_gate_count > EPSILON
            || self.multi_qubit_gate_count > EPSILON
            || self.clifford_gate_count > EPSILON
            || self.non_clifford_gate_count > EPSILON
            || self.t_count > EPSILON
            || self.t_depth > EPSILON
            || self.depth > EPSILON
            || self.two_qubit_depth > EPSILON
            || self.measurement_count > EPSILON
            || self.reset_count > EPSILON
            || self.barrier_count > EPSILON
            || self.qubit_count > EPSILON
            || self.ancilla_count > EPSILON
            || self.duration > EPSILON
            || self.error > EPSILON
            || self.energy > EPSILON
            || self.classical_cost > EPSILON
    }
}

// =============================================================================
// Cost vector
// =============================================================================

/// Complete optimization resource vector.
///
/// This type is intentionally independent from the circuit container. It can
/// represent both an original circuit and an optimized circuit, and therefore
/// can also be used by future analyses without introducing circular
/// dependencies.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CostVector {
    /// Total operations.
    pub gate_count: u128,

    /// Single-qubit operations.
    pub single_qubit_gate_count: u128,

    /// Two-qubit operations.
    pub two_qubit_gate_count: u128,

    /// Three-or-more-qubit operations.
    pub multi_qubit_gate_count: u128,

    /// Clifford operations.
    pub clifford_gate_count: u128,

    /// Non-Clifford operations.
    pub non_clifford_gate_count: u128,

    /// T operations.
    pub t_count: u128,

    /// T-depth.
    pub t_depth: u128,

    /// Total circuit depth.
    pub depth: u128,

    /// Two-qubit depth.
    pub two_qubit_depth: u128,

    /// Measurements.
    pub measurement_count: u128,

    /// Resets.
    pub reset_count: u128,

    /// Barriers.
    pub barrier_count: u128,

    /// Logical qubit count.
    pub qubit_count: u128,

    /// Ancilla count.
    pub ancilla_count: u128,

    /// Estimated duration.
    pub duration: f64,

    /// Estimated accumulated error.
    pub error: f64,

    /// Estimated energy.
    pub energy: f64,

    /// Estimated classical-side cost.
    pub classical_cost: f64,
}

impl CostVector {
    /// Creates an empty resource vector.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            gate_count: 0,
            single_qubit_gate_count: 0,
            two_qubit_gate_count: 0,
            multi_qubit_gate_count: 0,
            clifford_gate_count: 0,
            non_clifford_gate_count: 0,
            t_count: 0,
            t_depth: 0,
            depth: 0,
            two_qubit_depth: 0,
            measurement_count: 0,
            reset_count: 0,
            barrier_count: 0,
            qubit_count: 0,
            ancilla_count: 0,
            duration: 0.0,
            error: 0.0,
            energy: 0.0,
            classical_cost: 0.0,
        }
    }

    /// Returns the value associated with a metric.
    #[must_use]
    pub const fn get(self, metric: CostMetric) -> CostValue {
        match metric {
            CostMetric::GateCount => CostValue::Integer(self.gate_count),
            CostMetric::SingleQubitGateCount => {
                CostValue::Integer(self.single_qubit_gate_count)
            }
            CostMetric::TwoQubitGateCount => {
                CostValue::Integer(self.two_qubit_gate_count)
            }
            CostMetric::MultiQubitGateCount => {
                CostValue::Integer(self.multi_qubit_gate_count)
            }
            CostMetric::CliffordGateCount => {
                CostValue::Integer(self.clifford_gate_count)
            }
            CostMetric::NonCliffordGateCount => {
                CostValue::Integer(self.non_clifford_gate_count)
            }
            CostMetric::TCount => CostValue::Integer(self.t_count),
            CostMetric::TDepth => CostValue::Integer(self.t_depth),
            CostMetric::Depth => CostValue::Integer(self.depth),
            CostMetric::TwoQubitDepth => {
                CostValue::Integer(self.two_qubit_depth)
            }
            CostMetric::MeasurementCount => {
                CostValue::Integer(self.measurement_count)
            }
            CostMetric::ResetCount => CostValue::Integer(self.reset_count),
            CostMetric::BarrierCount => {
                CostValue::Integer(self.barrier_count)
            }
            CostMetric::QubitCount => CostValue::Integer(self.qubit_count),
            CostMetric::AncillaCount => CostValue::Integer(self.ancilla_count),
            CostMetric::Duration => CostValue::Floating(self.duration),
            CostMetric::Error => CostValue::Floating(self.error),
            CostMetric::Energy => CostValue::Floating(self.energy),
            CostMetric::ClassicalCost => {
                CostValue::Floating(self.classical_cost)
            }
        }
    }

    /// Adds two cost vectors with overflow checking.
    pub fn checked_add(self, other: Self) -> CostResult<Self> {
        Ok(Self {
            gate_count: checked_add(
                self.gate_count,
                other.gate_count,
                "gate_count",
            )?,
            single_qubit_gate_count: checked_add(
                self.single_qubit_gate_count,
                other.single_qubit_gate_count,
                "single_qubit_gate_count",
            )?,
            two_qubit_gate_count: checked_add(
                self.two_qubit_gate_count,
                other.two_qubit_gate_count,
                "two_qubit_gate_count",
            )?,
            multi_qubit_gate_count: checked_add(
                self.multi_qubit_gate_count,
                other.multi_qubit_gate_count,
                "multi_qubit_gate_count",
            )?,
            clifford_gate_count: checked_add(
                self.clifford_gate_count,
                other.clifford_gate_count,
                "clifford_gate_count",
            )?,
            non_clifford_gate_count: checked_add(
                self.non_clifford_gate_count,
                other.non_clifford_gate_count,
                "non_clifford_gate_count",
            )?,
            t_count: checked_add(
                self.t_count,
                other.t_count,
                "t_count",
            )?,
            t_depth: checked_add(
                self.t_depth,
                other.t_depth,
                "t_depth",
            )?,
            depth: checked_add(self.depth, other.depth, "depth")?,
            two_qubit_depth: checked_add(
                self.two_qubit_depth,
                other.two_qubit_depth,
                "two_qubit_depth",
            )?,
            measurement_count: checked_add(
                self.measurement_count,
                other.measurement_count,
                "measurement_count",
            )?,
            reset_count: checked_add(
                self.reset_count,
                other.reset_count,
                "reset_count",
            )?,
            barrier_count: checked_add(
                self.barrier_count,
                other.barrier_count,
                "barrier_count",
            )?,
            qubit_count: checked_add(
                self.qubit_count,
                other.qubit_count,
                "qubit_count",
            )?,
            ancilla_count: checked_add(
                self.ancilla_count,
                other.ancilla_count,
                "ancilla_count",
            )?,
            duration: checked_float_add(
                self.duration,
                other.duration,
                "duration",
            )?,
            error: checked_float_add(
                self.error,
                other.error,
                "error",
            )?,
            energy: checked_float_add(
                self.energy,
                other.energy,
                "energy",
            )?,
            classical_cost: checked_float_add(
                self.classical_cost,
                other.classical_cost,
                "classical_cost",
            )?,
        })
    }

    /// Returns a saturated sum.
    #[must_use]
    pub fn saturating_add(self, other: Self) -> Self {
        Self {
            gate_count: self.gate_count.saturating_add(other.gate_count),
            single_qubit_gate_count: self
                .single_qubit_gate_count
                .saturating_add(other.single_qubit_gate_count),
            two_qubit_gate_count: self
                .two_qubit_gate_count
                .saturating_add(other.two_qubit_gate_count),
            multi_qubit_gate_count: self
                .multi_qubit_gate_count
                .saturating_add(other.multi_qubit_gate_count),
            clifford_gate_count: self
                .clifford_gate_count
                .saturating_add(other.clifford_gate_count),
            non_clifford_gate_count: self
                .non_clifford_gate_count
                .saturating_add(other.non_clifford_gate_count),
            t_count: self.t_count.saturating_add(other.t_count),
            t_depth: self.t_depth.saturating_add(other.t_depth),
            depth: self.depth.saturating_add(other.depth),
            two_qubit_depth: self
                .two_qubit_depth
                .saturating_add(other.two_qubit_depth),
            measurement_count: self
                .measurement_count
                .saturating_add(other.measurement_count),
            reset_count: self.reset_count.saturating_add(other.reset_count),
            barrier_count: self
                .barrier_count
                .saturating_add(other.barrier_count),
            qubit_count: self.qubit_count.saturating_add(other.qubit_count),
            ancilla_count: self
                .ancilla_count
                .saturating_add(other.ancilla_count),
            duration: saturating_float_add(self.duration, other.duration),
            error: saturating_float_add(self.error, other.error),
            energy: saturating_float_add(self.energy, other.energy),
            classical_cost: saturating_float_add(
                self.classical_cost,
                other.classical_cost,
            ),
        }
    }

    /// Calculates the weighted scalar cost.
    pub fn weighted_cost(self, weights: ResourceWeights) -> CostResult<f64> {
        weights.validate()?;

        if !weights.has_active_metric() {
            return Err(CostError::EmptyObjective);
        }

        let mut total = 0.0_f64;

        for metric in ALL_METRICS {
            let weight = weights.get(metric);

            if weight == 0.0 {
                continue;
            }

            let value = match self.get(metric) {
                CostValue::Integer(value) => {
                    integer_to_f64(value, metric.as_str())?
                }
                CostValue::Floating(value) => value,
            };

            let contribution = value * weight;

            if !contribution.is_finite() {
                return Err(CostError::NonFiniteValue {
                    field: metric.as_str(),
                });
            }

            total += contribution;

            if !total.is_finite() {
                return Err(CostError::NonFiniteValue {
                    field: "weighted_cost",
                });
            }
        }

        Ok(total)
    }

    /// Returns the difference `self - other` for integer resources and
    /// floating-point difference for estimated resources.
    #[must_use]
    pub fn delta(self, other: Self) -> CostDelta {
        CostDelta {
            gate_count: signed_difference(
                self.gate_count,
                other.gate_count,
            ),
            single_qubit_gate_count: signed_difference(
                self.single_qubit_gate_count,
                other.single_qubit_gate_count,
            ),
            two_qubit_gate_count: signed_difference(
                self.two_qubit_gate_count,
                other.two_qubit_gate_count,
            ),
            multi_qubit_gate_count: signed_difference(
                self.multi_qubit_gate_count,
                other.multi_qubit_gate_count,
            ),
            clifford_gate_count: signed_difference(
                self.clifford_gate_count,
                other.clifford_gate_count,
            ),
            non_clifford_gate_count: signed_difference(
                self.non_clifford_gate_count,
                other.non_clifford_gate_count,
            ),
            t_count: signed_difference(self.t_count, other.t_count),
            t_depth: signed_difference(self.t_depth, other.t_depth),
            depth: signed_difference(self.depth, other.depth),
            two_qubit_depth: signed_difference(
                self.two_qubit_depth,
                other.two_qubit_depth,
            ),
            measurement_count: signed_difference(
                self.measurement_count,
                other.measurement_count,
            ),
            reset_count: signed_difference(
                self.reset_count,
                other.reset_count,
            ),
            barrier_count: signed_difference(
                self.barrier_count,
                other.barrier_count,
            ),
            qubit_count: signed_difference(
                self.qubit_count,
                other.qubit_count,
            ),
            ancilla_count: signed_difference(
                self.ancilla_count,
                other.ancilla_count,
            ),
            duration: self.duration - other.duration,
            error: self.error - other.error,
            energy: self.energy - other.energy,
            classical_cost: self.classical_cost - other.classical_cost,
        }
    }
}

// =============================================================================
// Cost delta
// =============================================================================

/// Signed change between two resource vectors.
///
/// Positive means the left-hand vector is more expensive in that dimension.
/// Negative means it is cheaper.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CostDelta {
    /// Gate-count difference.
    pub gate_count: i128,

    /// Single-qubit gate-count difference.
    pub single_qubit_gate_count: i128,

    /// Two-qubit gate-count difference.
    pub two_qubit_gate_count: i128,

    /// Multi-qubit gate-count difference.
    pub multi_qubit_gate_count: i128,

    /// Clifford gate-count difference.
    pub clifford_gate_count: i128,

    /// Non-Clifford gate-count difference.
    pub non_clifford_gate_count: i128,

    /// T-count difference.
    pub t_count: i128,

    /// T-depth difference.
    pub t_depth: i128,

    /// Depth difference.
    pub depth: i128,

    /// Two-qubit-depth difference.
    pub two_qubit_depth: i128,

    /// Measurement difference.
    pub measurement_count: i128,

    /// Reset difference.
    pub reset_count: i128,

    /// Barrier difference.
    pub barrier_count: i128,

    /// Qubit-count difference.
    pub qubit_count: i128,

    /// Ancilla difference.
    pub ancilla_count: i128,

    /// Duration difference.
    pub duration: f64,

    /// Error difference.
    pub error: f64,

    /// Energy difference.
    pub energy: f64,

    /// Classical-cost difference.
    pub classical_cost: f64,
}

impl CostDelta {
    /// Returns whether every resource dimension is unchanged.
    #[must_use]
    pub fn is_zero(self) -> bool {
        self.gate_count == 0
            && self.single_qubit_gate_count == 0
            && self.two_qubit_gate_count == 0
            && self.multi_qubit_gate_count == 0
            && self.clifford_gate_count == 0
            && self.non_clifford_gate_count == 0
            && self.t_count == 0
            && self.t_depth == 0
            && self.depth == 0
            && self.two_qubit_depth == 0
            && self.measurement_count == 0
            && self.reset_count == 0
            && self.barrier_count == 0
            && self.qubit_count == 0
            && self.ancilla_count == 0
            && self.duration == 0.0
            && self.error == 0.0
            && self.energy == 0.0
            && self.classical_cost == 0.0
    }
}

// =============================================================================
// Optimization objective
// =============================================================================

/// Optimization objective used by the planner and pipeline.
#[derive(Debug, Clone, PartialEq)]
pub enum OptimizationObjective {
    /// Minimize one resource.
    Minimize(CostMetric),

    /// Minimize several resources lexicographically.
    ///
    /// The first metric has highest priority.
    Lexicographic(Vec<CostMetric>),

    /// Minimize a weighted scalar cost.
    Weighted(ResourceWeights),

    /// Treat every listed metric as a Pareto dimension.
    Pareto(Vec<CostMetric>),

    /// Minimize the first metric and use subsequent metrics as deterministic
    /// tie breakers.
    PrimaryThenTieBreak {
        /// Primary optimization dimension.
        primary: CostMetric,

        /// Ordered tie-break dimensions.
        tie_breakers: Vec<CostMetric>,
    },
}

impl OptimizationObjective {
    /// Returns a simple gate-count objective.
    #[must_use]
    pub const fn gate_count() -> Self {
        Self::Minimize(CostMetric::GateCount)
    }

    /// Returns a depth objective.
    #[must_use]
    pub const fn depth() -> Self {
        Self::Minimize(CostMetric::Depth)
    }

    /// Returns a two-qubit-gate objective.
    #[must_use]
    pub const fn two_qubit_gates() -> Self {
        Self::Minimize(CostMetric::TwoQubitGateCount)
    }

    /// Returns a T-count objective.
    #[must_use]
    pub const fn t_count() -> Self {
        Self::Minimize(CostMetric::TCount)
    }

    /// Returns a T-depth objective.
    #[must_use]
    pub const fn t_depth() -> Self {
        Self::Minimize(CostMetric::TDepth)
    }

    /// Returns a balanced hardware objective.
    #[must_use]
    pub const fn hardware_balanced() -> Self {
        Self::Weighted(ResourceWeights::hardware_balanced())
    }

    /// Returns a fault-tolerant objective.
    #[must_use]
    pub const fn fault_tolerant() -> Self {
        Self::Weighted(ResourceWeights::fault_tolerant())
    }

    /// Validates the objective.
    pub fn validate(&self) -> CostResult<()> {
        match self {
            Self::Minimize(_) => Ok(()),

            Self::Lexicographic(metrics) | Self::Pareto(metrics) => {
                if metrics.is_empty() {
                    return Err(CostError::EmptyObjective);
                }

                Ok(())
            }

            Self::Weighted(weights) => {
                weights.validate()?;

                if !weights.has_active_metric() {
                    return Err(CostError::EmptyObjective);
                }

                Ok(())
            }

            Self::PrimaryThenTieBreak {
                primary,
                tie_breakers,
            } => {
                if tie_breakers.iter().any(|metric| metric == primary) {
                    return Err(CostError::InvalidObjective {
                        message: "primary metric must not also be a tie-break metric",
                    });
                }

                Ok(())
            }
        }
    }
}

// =============================================================================
// Cost comparison
// =============================================================================

/// Result of comparing two resource vectors under an optimization objective.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CostComparison {
    /// The first cost is strictly better.
    Better,

    /// Both costs are equivalent under the objective.
    Equivalent,

    /// The first cost is strictly worse.
    Worse,

    /// Neither cost dominates the other under a Pareto objective.
    Incomparable,
}

/// Compares two cost vectors under an optimization objective.
///
/// All objectives represent minimization. `Better` therefore means that
/// `left` has a lower optimization cost than `right`.
pub fn compare_costs(
    left: CostVector,
    right: CostVector,
    objective: &OptimizationObjective,
) -> CostResult<CostComparison> {
    objective.validate()?;

    match objective {
        OptimizationObjective::Minimize(metric) => {
            Ok(compare_metric(left, right, *metric))
        }

        OptimizationObjective::Lexicographic(metrics) => {
            Ok(compare_lexicographic(left, right, metrics))
        }

        OptimizationObjective::Weighted(weights) => {
            let left_cost = left.weighted_cost(*weights)?;
            let right_cost = right.weighted_cost(*weights)?;

            Ok(compare_f64(left_cost, right_cost))
        }

        OptimizationObjective::Pareto(metrics) => {
            Ok(compare_pareto(left, right, metrics))
        }

        OptimizationObjective::PrimaryThenTieBreak {
            primary,
            tie_breakers,
        } => {
            let primary_result = compare_metric(left, right, *primary);

            if primary_result != CostComparison::Equivalent {
                return Ok(primary_result);
            }

            Ok(compare_lexicographic(left, right, tie_breakers))
        }
    }
}

// =============================================================================
// Default gate cost model
// =============================================================================

/// Defines how logical gate kinds contribute to resource cost.
///
/// The model is deliberately category-based rather than backend-specific.
/// Backend-specific timing, error and energy information can be supplied by
/// later target/hardware integration without changing this API.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GateCostModel {
    /// Base duration of a single-qubit gate.
    pub single_qubit_duration: f64,

    /// Base duration of a two-qubit gate.
    pub two_qubit_duration: f64,

    /// Base duration of a multi-qubit gate.
    pub multi_qubit_duration: f64,

    /// Estimated error of a single-qubit gate.
    pub single_qubit_error: f64,

    /// Estimated error of a two-qubit gate.
    pub two_qubit_error: f64,

    /// Estimated error of a multi-qubit gate.
    pub multi_qubit_error: f64,

    /// Estimated energy of a single-qubit gate.
    pub single_qubit_energy: f64,

    /// Estimated energy of a two-qubit gate.
    pub two_qubit_energy: f64,

    /// Estimated energy of a multi-qubit gate.
    pub multi_qubit_energy: f64,
}

impl GateCostModel {
    /// Hardware-neutral zero-estimate model.
    ///
    /// Exact logical resource counters remain active while duration, error and
    /// energy are deliberately left unspecified.
    #[must_use]
    pub const fn logical() -> Self {
        Self {
            single_qubit_duration: 0.0,
            two_qubit_duration: 0.0,
            multi_qubit_duration: 0.0,
            single_qubit_error: 0.0,
            two_qubit_error: 0.0,
            multi_qubit_error: 0.0,
            single_qubit_energy: 0.0,
            two_qubit_energy: 0.0,
            multi_qubit_energy: 0.0,
        }
    }

    /// Validates the model.
    pub fn validate(&self) -> CostResult<()> {
        validate_non_negative_finite(
            "single_qubit_duration",
            self.single_qubit_duration,
        )?;
        validate_non_negative_finite(
            "two_qubit_duration",
            self.two_qubit_duration,
        )?;
        validate_non_negative_finite(
            "multi_qubit_duration",
            self.multi_qubit_duration,
        )?;
        validate_non_negative_finite(
            "single_qubit_error",
            self.single_qubit_error,
        )?;
        validate_non_negative_finite(
            "two_qubit_error",
            self.two_qubit_error,
        )?;
        validate_non_negative_finite(
            "multi_qubit_error",
            self.multi_qubit_error,
        )?;
        validate_non_negative_finite(
            "single_qubit_energy",
            self.single_qubit_energy,
        )?;
        validate_non_negative_finite(
            "two_qubit_energy",
            self.two_qubit_energy,
        )?;
        validate_non_negative_finite(
            "multi_qubit_energy",
            self.multi_qubit_energy,
        )?;

        Ok(())
    }

    /// Calculates the resource contribution of one canonical IR gate.
    ///
    /// This method deliberately uses the canonical `GateKind` rather than a
    /// private optimizer gate representation.
    pub fn cost_for_gate(&self, kind: GateKind) -> GateCost {
        if kind.is_measurement() {
            return GateCost::measurement();
        }

        if kind.is_reset() {
            return GateCost::reset();
        }

        if kind.is_barrier() {
            return GateCost::barrier();
        }

        let mut cost = match kind.operand_count_value() {
            1 => {
                if kind == GateKind::T || kind == GateKind::Tdg {
                    GateCost::t_gate()
                } else if kind.is_clifford() {
                    GateCost::clifford_single_qubit()
                } else {
                    GateCost::single_qubit()
                }
            }

            2 => {
                if kind.is_clifford() {
                    GateCost::clifford_two_qubit()
                } else {
                    GateCost::two_qubit()
                }
            }

            _ => GateCost::multi_qubit(),
        };

        match kind.operand_count_value() {
            1 => {
                cost.duration = self.single_qubit_duration;
                cost.error = self.single_qubit_error;
                cost.energy = self.single_qubit_energy;
            }

            2 => {
                cost.duration = self.two_qubit_duration;
                cost.error = self.two_qubit_error;
                cost.energy = self.two_qubit_energy;
            }

            _ => {
                cost.duration = self.multi_qubit_duration;
                cost.error = self.multi_qubit_error;
                cost.energy = self.multi_qubit_energy;
            }
        }

        cost
    }
}

// =============================================================================
// Complete cost model
// =============================================================================

/// Complete logical optimization cost model.
///
/// This is the stable object future optimization context/configuration code can
/// own.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CostModel {
    /// Per-gate resource model.
    pub gate_costs: GateCostModel,

    /// Whether measurements contribute to gate count.
    ///
    /// This is enabled by default because a measurement is an executable IR
    /// operation and therefore consumes resources. Users can disable it when
    /// comparing purely unitary transformations.
    pub count_measurements: bool,

    /// Whether resets contribute to gate count.
    pub count_resets: bool,

    /// Whether barriers contribute to gate count.
    pub count_barriers: bool,

    /// Additional classical-side optimization cost.
    pub classical_cost_per_operation: f64,

    /// Whether logical width should be retained in resource vectors.
    pub count_qubits: bool,
}

impl CostModel {
    /// Creates the canonical hardware-independent logical model.
    #[must_use]
    pub const fn logical() -> Self {
        Self {
            gate_costs: GateCostModel::logical(),
            count_measurements: true,
            count_resets: true,
            count_barriers: true,
            classical_cost_per_operation: 0.0,
            count_qubits: true,
        }
    }

    /// Validates the model.
    pub fn validate(&self) -> CostResult<()> {
        self.gate_costs.validate()?;

        validate_non_negative_finite(
            "classical_cost_per_operation",
            self.classical_cost_per_operation,
        )?;

        Ok(())
    }

    /// Returns the cost of one gate.
    pub fn cost_for_gate(&self, kind: GateKind) -> CostResult<GateCost> {
        self.validate()?;

        let mut cost = self.gate_costs.cost_for_gate(kind);

        if kind.is_measurement() && !self.count_measurements {
            cost.gate_count = 0;
            cost.measurement_count = 0;
        }

        if kind.is_reset() && !self.count_resets {
            cost.gate_count = 0;
            cost.reset_count = 0;
        }

        if kind.is_barrier() && !self.count_barriers {
            cost.gate_count = 0;
            cost.barrier_count = 0;
        }

        cost.classical_cost = self.classical_cost_per_operation;

        Ok(cost)
    }

    /// Converts a gate sequence into an accumulated resource vector.
    ///
    /// This method accepts an iterator instead of a concrete circuit so that
    /// analysis code can feed it any canonical IR traversal without creating a
    /// second circuit abstraction here.
    pub fn cost_for_gates<I>(
        &self,
        gates: I,
    ) -> CostResult<CostVector>
    where
        I: IntoIterator<Item = GateKind>,
    {
        self.validate()?;

        let mut result = CostVector::zero();

        for kind in gates {
            let gate = self.cost_for_gate(kind)?;

            result.gate_count = checked_add(
                result.gate_count,
                gate.gate_count,
                "gate_count",
            )?;

            result.single_qubit_gate_count = checked_add(
                result.single_qubit_gate_count,
                gate.single_qubit_gate_count,
                "single_qubit_gate_count",
            )?;

            result.two_qubit_gate_count = checked_add(
                result.two_qubit_gate_count,
                gate.two_qubit_gate_count,
                "two_qubit_gate_count",
            )?;

            result.multi_qubit_gate_count = checked_add(
                result.multi_qubit_gate_count,
                gate.multi_qubit_gate_count,
                "multi_qubit_gate_count",
            )?;

            result.clifford_gate_count = checked_add(
                result.clifford_gate_count,
                gate.clifford_gate_count,
                "clifford_gate_count",
            )?;

            result.non_clifford_gate_count = checked_add(
                result.non_clifford_gate_count,
                gate.non_clifford_gate_count,
                "non_clifford_gate_count",
            )?;

            result.t_count = checked_add(
                result.t_count,
                gate.t_count,
                "t_count",
            )?;

            result.measurement_count = checked_add(
                result.measurement_count,
                gate.measurement_count,
                "measurement_count",
            )?;

            result.reset_count = checked_add(
                result.reset_count,
                gate.reset_count,
                "reset_count",
            )?;

            result.barrier_count = checked_add(
                result.barrier_count,
                gate.barrier_count,
                "barrier_count",
            )?;

            result.duration = checked_float_add(
                result.duration,
                gate.duration,
                "duration",
            )?;

            result.error = checked_float_add(
                result.error,
                gate.error,
                "error",
            )?;

            result.energy = checked_float_add(
                result.energy,
                gate.energy,
                "energy",
            )?;

            result.classical_cost = checked_float_add(
                result.classical_cost,
                gate.classical_cost,
                "classical_cost",
            )?;
        }

        Ok(result)
    }
}

// =============================================================================
// Complete metric set
// =============================================================================

/// All metrics currently defined by the optimization cost subsystem.
///
/// This is intentionally a static slice so adding analysis logic elsewhere
/// does not require a registry allocation.
pub const ALL_METRICS: &[CostMetric] = &[
    CostMetric::GateCount,
    CostMetric::SingleQubitGateCount,
    CostMetric::TwoQubitGateCount,
    CostMetric::MultiQubitGateCount,
    CostMetric::CliffordGateCount,
    CostMetric::NonCliffordGateCount,
    CostMetric::TCount,
    CostMetric::TDepth,
    CostMetric::Depth,
    CostMetric::TwoQubitDepth,
    CostMetric::MeasurementCount,
    CostMetric::ResetCount,
    CostMetric::BarrierCount,
    CostMetric::QubitCount,
    CostMetric::AncillaCount,
    CostMetric::Duration,
    CostMetric::Error,
    CostMetric::Energy,
    CostMetric::ClassicalCost,
];

// =============================================================================
// GateKind compatibility helpers
// =============================================================================
//
// The canonical IR currently exposes operand_count() as an OperandCount.
// Optimization cost accounting needs a compact integer classification. Keeping
// the conversion here prevents the cost subsystem from depending on the
// internal OperandCount representation beyond the public GateKind contract.

trait GateKindCostExt {
    fn operand_count_value(self) -> usize;
}

impl GateKindCostExt for GateKind {
    fn operand_count_value(self) -> usize {
        match self {
            GateKind::I
            | GateKind::X
            | GateKind::Y
            | GateKind::Z
            | GateKind::H
            | GateKind::S
            | GateKind::Sdg
            | GateKind::T
            | GateKind::Tdg
            | GateKind::V
            | GateKind::Vdg
            | GateKind::RX
            | GateKind::RY
            | GateKind::RZ
            | GateKind::Phase
            | GateKind::U1
            | GateKind::U2
            | GateKind::U3
            | GateKind::Measure
            | GateKind::Reset => 1,

            GateKind::CX
            | GateKind::CY
            | GateKind::CZ
            | GateKind::CH
            | GateKind::SWAP
            | GateKind::ISWAP
            | GateKind::ECR
            | GateKind::CRX
            | GateKind::CRY
            | GateKind::CRZ => 2,

            GateKind::CCX | GateKind::CSWAP => 3,

            GateKind::Barrier => 1,
        }
    }
}

// =============================================================================
// Internal helpers
// =============================================================================

fn checked_add(
    left: u128,
    right: u128,
    resource: &'static str,
) -> CostResult<u128> {
    left.checked_add(right).ok_or(CostError::ArithmeticOverflow {
        resource,
    })
}

fn checked_float_add(
    left: f64,
    right: f64,
    resource: &'static str,
) -> CostResult<f64> {
    if !left.is_finite() || !right.is_finite() {
        return Err(CostError::NonFiniteValue {
            field: resource,
        });
    }

    let result = left + right;

    if !result.is_finite() {
        return Err(CostError::NonFiniteValue {
            field: resource,
        });
    }

    Ok(result)
}

fn saturating_float_add(left: f64, right: f64) -> f64 {
    let result = left + right;

    if result.is_finite() {
        result
    } else {
        f64::MAX
    }
}

fn signed_difference(left: u128, right: u128) -> i128 {
    if left >= right {
        let difference = left - right;

        if difference > i128::MAX as u128 {
            i128::MAX
        } else {
            difference as i128
        }
    } else {
        let difference = right - left;

        if difference > i128::MAX as u128 {
            i128::MIN
        } else {
            -(difference as i128)
        }
    }
}

fn integer_to_f64(
    value: u128,
    resource: &'static str,
) -> CostResult<f64> {
    let converted = value as f64;

    if !converted.is_finite() {
        return Err(CostError::NonFiniteValue {
            field: resource,
        });
    }

    Ok(converted)
}

fn validate_weight(
    metric: &'static str,
    value: f64,
) -> CostResult<()> {
    if !value.is_finite() {
        return Err(CostError::InvalidWeight { metric });
    }

    if value < 0.0 {
        return Err(CostError::InvalidWeight { metric });
    }

    Ok(())
}

fn validate_non_negative_finite(
    field: &'static str,
    value: f64,
) -> CostResult<()> {
    if !value.is_finite() {
        return Err(CostError::NonFiniteValue { field });
    }

    if value < 0.0 {
        return Err(CostError::NegativeValue { field });
    }

    Ok(())
}

fn compare_f64(left: f64, right: f64) -> CostComparison {
    match left.total_cmp(&right) {
        Ordering::Less => CostComparison::Better,
        Ordering::Equal => CostComparison::Equivalent,
        Ordering::Greater => CostComparison::Worse,
    }
}

fn compare_metric(
    left: CostVector,
    right: CostVector,
    metric: CostMetric,
) -> CostComparison {
    match (left.get(metric), right.get(metric)) {
        (CostValue::Integer(left), CostValue::Integer(right)) => {
            match left.cmp(&right) {
                Ordering::Less => CostComparison::Better,
                Ordering::Equal => CostComparison::Equivalent,
                Ordering::Greater => CostComparison::Worse,
            }
        }

        (CostValue::Floating(left), CostValue::Floating(right)) => {
            compare_f64(left, right)
        }

        // This branch can only occur if CostMetric's representation is changed
        // inconsistently. Keeping it explicit prevents accidental coercion.
        _ => CostComparison::Incomparable,
    }
}

fn compare_lexicographic(
    left: CostVector,
    right: CostVector,
    metrics: &[CostMetric],
) -> CostComparison {
    for metric in metrics {
        let result = compare_metric(left, right, *metric);

        if result != CostComparison::Equivalent {
            return result;
        }
    }

    CostComparison::Equivalent
}

fn compare_pareto(
    left: CostVector,
    right: CostVector,
    metrics: &[CostMetric],
) -> CostComparison {
    let mut left_better = false;
    let mut right_better = false;

    for metric in metrics {
        match compare_metric(left, right, *metric) {
            CostComparison::Better => left_better = true,
            CostComparison::Worse => right_better = true,
            CostComparison::Equivalent
            | CostComparison::Incomparable => {}
        }
    }

    match (left_better, right_better) {
        (true, false) => CostComparison::Better,
        (false, true) => CostComparison::Worse,
        (false, false) => CostComparison::Equivalent,
        (true, true) => CostComparison::Incomparable,
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_cost_is_zero() {
        let cost = CostVector::zero();

        assert_eq!(cost.gate_count, 0);
        assert_eq!(cost.two_qubit_gate_count, 0);
        assert_eq!(cost.t_count, 0);
        assert_eq!(cost.depth, 0);
        assert_eq!(cost.duration, 0.0);
    }

    #[test]
    fn gate_cost_classification_is_correct() {
        let model = GateCostModel::logical();

        let h = model.cost_for_gate(GateKind::H);
        assert_eq!(h.gate_count, 1);
        assert_eq!(h.single_qubit_gate_count, 1);
        assert_eq!(h.clifford_gate_count, 1);
        assert_eq!(h.non_clifford_gate_count, 0);

        let cx = model.cost_for_gate(GateKind::CX);
        assert_eq!(cx.gate_count, 1);
        assert_eq!(cx.two_qubit_gate_count, 1);
        assert_eq!(cx.clifford_gate_count, 1);

        let t = model.cost_for_gate(GateKind::T);
        assert_eq!(t.gate_count, 1);
        assert_eq!(t.single_qubit_gate_count, 1);
        assert_eq!(t.non_clifford_gate_count, 1);
        assert_eq!(t.t_count, 1);
    }

    #[test]
    fn measurements_are_counted_separately() {
        let model = CostModel::logical();
        let cost = model.cost_for_gate(GateKind::Measure).unwrap();

        assert_eq!(cost.gate_count, 1);
        assert_eq!(cost.measurement_count, 1);
        assert_eq!(cost.single_qubit_gate_count, 0);
        assert_eq!(cost.two_qubit_gate_count, 0);
    }

    #[test]
    fn reset_is_counted_separately() {
        let model = CostModel::logical();
        let cost = model.cost_for_gate(GateKind::Reset).unwrap();

        assert_eq!(cost.gate_count, 1);
        assert_eq!(cost.reset_count, 1);
    }

    #[test]
    fn barrier_is_counted_separately() {
        let model = CostModel::logical();
        let cost = model.cost_for_gate(GateKind::Barrier).unwrap();

        assert_eq!(cost.gate_count, 1);
        assert_eq!(cost.barrier_count, 1);
    }

    #[test]
    fn gate_sequence_accumulates_without_overflow() {
        let model = CostModel::logical();

        let cost = model
            .cost_for_gates([
                GateKind::H,
                GateKind::CX,
                GateKind::T,
                GateKind::Tdg,
                GateKind::Measure,
            ])
            .unwrap();

        assert_eq!(cost.gate_count, 5);
        assert_eq!(cost.single_qubit_gate_count, 4);
        assert_eq!(cost.two_qubit_gate_count, 1);
        assert_eq!(cost.clifford_gate_count, 2);
        assert_eq!(cost.non_clifford_gate_count, 2);
        assert_eq!(cost.t_count, 2);
        assert_eq!(cost.measurement_count, 1);
    }

    #[test]
    fn weighted_gate_count_objective_works() {
        let objective = OptimizationObjective::gate_count();

        let small = CostVector {
            gate_count: 10,
            ..CostVector::zero()
        };

        let large = CostVector {
            gate_count: 20,
            ..CostVector::zero()
        };

        assert_eq!(
            compare_costs(small, large, &objective).unwrap(),
            CostComparison::Better
        );

        assert_eq!(
            compare_costs(large, small, &objective).unwrap(),
            CostComparison::Worse
        );
    }

    #[test]
    fn depth_objective_can_prefer_more_gates() {
        let objective = OptimizationObjective::depth();

        let more_gates = CostVector {
            gate_count: 20,
            depth: 3,
            ..CostVector::zero()
        };

        let fewer_gates = CostVector {
            gate_count: 10,
            depth: 8,
            ..CostVector::zero()
        };

        assert_eq!(
            compare_costs(more_gates, fewer_gates, &objective).unwrap(),
            CostComparison::Better
        );
    }

    #[test]
    fn two_qubit_objective_is_independent_of_total_gate_count() {
        let objective = OptimizationObjective::two_qubit_gates();

        let first = CostVector {
            gate_count: 100,
            two_qubit_gate_count: 2,
            ..CostVector::zero()
        };

        let second = CostVector {
            gate_count: 20,
            two_qubit_gate_count: 5,
            ..CostVector::zero()
        };

        assert_eq!(
            compare_costs(first, second, &objective).unwrap(),
            CostComparison::Better
        );
    }

    #[test]
    fn fault_tolerant_objective_prefers_lower_t_count() {
        let objective = OptimizationObjective::t_count();

        let first = CostVector {
            gate_count: 100,
            t_count: 2,
            ..CostVector::zero()
        };

        let second = CostVector {
            gate_count: 20,
            t_count: 10,
            ..CostVector::zero()
        };

        assert_eq!(
            compare_costs(first, second, &objective).unwrap(),
            CostComparison::Better
        );
    }

    #[test]
    fn lexicographic_objective_uses_tie_breakers() {
        let objective = OptimizationObjective::Lexicographic(vec![
            CostMetric::TwoQubitGateCount,
            CostMetric::Depth,
            CostMetric::GateCount,
        ]);

        let first = CostVector {
            two_qubit_gate_count: 2,
            depth: 10,
            gate_count: 100,
            ..CostVector::zero()
        };

        let second = CostVector {
            two_qubit_gate_count: 2,
            depth: 20,
            gate_count: 50,
            ..CostVector::zero()
        };

        assert_eq!(
            compare_costs(first, second, &objective).unwrap(),
            CostComparison::Better
        );
    }

    #[test]
    fn pareto_detects_incomparable_costs() {
        let objective = OptimizationObjective::Pareto(vec![
            CostMetric::GateCount,
            CostMetric::Depth,
        ]);

        let first = CostVector {
            gate_count: 10,
            depth: 20,
            ..CostVector::zero()
        };

        let second = CostVector {
            gate_count: 20,
            depth: 10,
            ..CostVector::zero()
        };

        assert_eq!(
            compare_costs(first, second, &objective).unwrap(),
            CostComparison::Incomparable
        );
    }

    #[test]
    fn pareto_detects_dominance() {
        let objective = OptimizationObjective::Pareto(vec![
            CostMetric::GateCount,
            CostMetric::Depth,
        ]);

        let first = CostVector {
            gate_count: 10,
            depth: 10,
            ..CostVector::zero()
        };

        let second = CostVector {
            gate_count: 20,
            depth: 20,
            ..CostVector::zero()
        };

        assert_eq!(
            compare_costs(first, second, &objective).unwrap(),
            CostComparison::Better
        );
    }

    #[test]
    fn zero_weight_objective_is_rejected() {
        let objective =
            OptimizationObjective::Weighted(ResourceWeights::zero());

        assert_eq!(
            objective.validate(),
            Err(CostError::EmptyObjective)
        );
    }

    #[test]
    fn negative_weight_is_rejected() {
        let weights = ResourceWeights::zero()
            .with(CostMetric::GateCount, -1.0);

        assert!(matches!(
            weights,
            Err(CostError::InvalidWeight {
                metric: "gate_count"
            })
        ));
    }

    #[test]
    fn_nan_weight_is_rejected() {
        let weights = ResourceWeights::zero()
            .with(CostMetric::GateCount, f64::NAN);

        assert!(matches!(
            weights,
            Err(CostError::InvalidWeight {
                metric: "gate_count"
            })
        ));
    }

    #[test]
    fn_infinite_weight_is_rejected() {
        let weights = ResourceWeights::zero()
            .with(CostMetric::GateCount, f64::INFINITY);

        assert!(matches!(
            weights,
            Err(CostError::InvalidWeight {
                metric: "gate_count"
            })
        ));
    }

    #[test]
    fn checked_integer_overflow_is_reported() {
        let left = CostVector {
            gate_count: u128::MAX,
            ..CostVector::zero()
        };

        let right = CostVector {
            gate_count: 1,
            ..CostVector::zero()
        };

        assert_eq!(
            left.checked_add(right),
            Err(CostError::ArithmeticOverflow {
                resource: "gate_count"
            })
        );
    }

    #[test]
    fn saturating_integer_addition_does_not_wrap() {
        let left = CostVector {
            gate_count: u128::MAX,
            ..CostVector::zero()
        };

        let right = CostVector {
            gate_count: 1,
            ..CostVector::zero()
        };

        let result = left.saturating_add(right);

        assert_eq!(result.gate_count, u128::MAX);
    }

    #[test]
    fn delta_reports_reduction_as_negative() {
        let before = CostVector {
            gate_count: 100,
            two_qubit_gate_count: 20,
            t_count: 10,
            ..CostVector::zero()
        };

        let after = CostVector {
            gate_count: 70,
            two_qubit_gate_count: 5,
            t_count: 2,
            ..CostVector::zero()
        };

        let delta = after.delta(before);

        assert_eq!(delta.gate_count, -30);
        assert_eq!(delta.two_qubit_gate_count, -15);
        assert_eq!(delta.t_count, -8);
    }

    #[test]
    fn weighted_cost_combines_multiple_dimensions() {
        let weights = ResourceWeights::zero()
            .with(CostMetric::GateCount, 1.0)
            .unwrap()
            .with(CostMetric::TwoQubitGateCount, 10.0)
            .unwrap();

        let cost = CostVector {
            gate_count: 10,
            two_qubit_gate_count: 3,
            ..CostVector::zero()
        };

        assert_eq!(cost.weighted_cost(weights).unwrap(), 40.0);
    }

    #[test]
    fn model_accepts_hardware_estimates() {
        let gate_model = GateCostModel {
            single_qubit_duration: 10.0,
            two_qubit_duration: 50.0,
            multi_qubit_duration: 100.0,
            single_qubit_error: 0.001,
            two_qubit_error: 0.01,
            multi_qubit_error: 0.02,
            single_qubit_energy: 1.0,
            two_qubit_energy: 5.0,
            multi_qubit_energy: 10.0,
        };

        assert!(gate_model.validate().is_ok());

        let model = CostModel {
            gate_costs: gate_model,
            count_measurements: true,
            count_resets: true,
            count_barriers: true,
            classical_cost_per_operation: 0.1,
            count_qubits: true,
        };

        let cost = model.cost_for_gate(GateKind::CX).unwrap();

        assert_eq!(cost.two_qubit_gate_count, 1);
        assert_eq!(cost.duration, 50.0);
        assert_eq!(cost.error, 0.01);
        assert_eq!(cost.energy, 5.0);
    }

    #[test]
    fn all_metrics_have_stable_names() {
        for metric in ALL_METRICS {
            assert!(!metric.as_str().is_empty());
        }
    }
}