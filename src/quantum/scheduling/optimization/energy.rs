//! Zamani Quantum Scheduling — Energy Optimization
//!
//! Path:
//!     src/quantum/scheduling/optimization/energy.rs
//!
//! # Purpose
//!
//! This module provides the production energy-objective boundary for the
//! quantum scheduling subsystem.
//!
//! It answers:
//!
//!     "Given target-supplied energy estimates for a candidate schedule,
//!      how energetically expensive is that schedule, and which candidate
//!      should a scheduler prefer?"
//!
//! This module deliberately does NOT:
//!
//! - discover hardware;
//! - query hardware;
//! - define hardware power models;
//! - define pulse physics;
//! - define gate semantics;
//! - perform routing;
//! - perform scheduling;
//! - modify a schedule;
//! - define QubitId;
//! - define PhysicalQubitId;
//! - define resource topology;
//! - define noise models;
//! - perform QEC;
//! - execute quantum operations.
//!
//! Instead:
//!
//! ```text
//! hardware / calibration / pulse / noise model
//!                    |
//!                    v
//!          target-specific adapter
//!                    |
//!                    v
//!        EnergyContribution values
//!                    |
//!                    v
//!              energy.rs
//!                    |
//!             energy objective
//!                    |
//!                    v
//!          scheduling planner
//! ```
//!
//! # Architectural ownership
//!
//! The ownership boundary is:
//!
//! ```text
//! quantum::ir
//!      |
//!      | quantum semantics
//!      v
//! optimization / routing
//!      |
//!      | target-compatible program
//!      v
//! scheduling
//!      |
//!      | when / resource reservation / timing
//!      v
//! energy.rs
//!      |
//!      | objective evaluation only
//!      v
//! schedule candidate comparison
//! ```
//!
//! Routing answers:
//!
//!     WHERE?
//!
//! Scheduling answers:
//!
//!     WHEN?
//!
//! This module answers:
//!
//!     HOW ENERGETICALLY EXPENSIVE?
//!
//! It must never become a second scheduler.
//!
//! # Universal-program principle
//!
//! A Zamani program is written once.
//!
//! Its energy objective is evaluated against the target actually selected for
//! the invocation.
//!
//! Therefore this module contains no:
//!
//! - fixed qubit count;
//! - fixed gate count;
//! - fixed channel count;
//! - fixed device topology;
//! - fixed gate duration;
//! - fixed voltage;
//! - fixed current;
//! - fixed power;
//! - fixed clock;
//! - fixed energy-per-gate;
//! - fixed schedule depth;
//! - fixed machine size.
//!
//! A small machine and a very large machine can provide different energy
//! observations to the same evaluator without changing this file.
//!
//! "Infinity" means that this module imposes no artificial finite machine-size
//! ceiling. Actual resource availability remains bounded by the target,
//! operating environment, numeric representation, and explicit caller policy.
//!
//! # Physical-energy boundary
//!
//! This module distinguishes between:
//!
//! 1. measured energy;
//! 2. calibrated estimated energy;
//! 3. analytically estimated energy;
//! 4. externally supplied energy;
//! 5. schedule-derived energy.
//!
//! The module does not claim that an arbitrary floating-point value represents
//! physical joules unless the producing adapter explicitly declares its unit
//! and provenance.
//!
//! In particular:
//!
//!     waveform mathematical energy != automatically physical energy
//!
//! The upstream model must perform the physical interpretation.
//!
//! # Why the objective consumes contributions
//!
//! A scheduler must be able to evaluate energy from many possible sources:
//!
//! - gate execution;
//! - idle power;
//! - control electronics;
//! - readout;
//! - reset;
//! - cooling;
//! - communication;
//! - transport;
//! - entanglement generation;
//! - classical feedback;
//! - memory;
//! - synchronization;
//! - target-specific resources;
//! - user-defined resources.
//!
//! Hard-coding those sources into this module would make the scheduler
//! architecture vendor- and technology-dependent.
//!
//! Instead, an adapter emits normalized contributions.
//!
//! # Energy accounting model
//!
//! The fundamental quantity is:
//!
//!     energy >= 0
//!
//! Contributions may be represented directly in a common unit selected by the
//! caller.
//!
//! The evaluator never silently converts between units.
//!
//! Therefore:
//!
//!     joule + joule
//!
//! is valid,
//!
//! while:
//!
//!     joule + femtojoule
//!
//! must first be normalized by the producing adapter.
//!
//! This prevents hidden unit conversion and machine-specific assumptions.
//!
//! # Weighted optimization
//!
//! Energy may be one dimension of a larger objective:
//!
//!     total_cost =
//!         w_time   * time_cost
//!       + w_energy * energy_cost
//!       + w_fidelity * fidelity_cost
//!       + w_idle   * idle_cost
//!       + ...
//!
//! This file therefore exposes a normalized energy objective cost rather than
//! pretending that energy itself can be directly compared to unrelated
//! quantities such as nanoseconds or fidelity.
//!
//! Weights are caller-supplied.
//!
//! No hardware-specific weight is embedded here.
//!
//! # Schedule mutation
//!
//! This module does NOT move operations.
//!
//! A planner may use the result to compare candidates:
//!
//! ```text
//! candidate A -> energy.rs -> cost A
//! candidate B -> energy.rs -> cost B
//! candidate C -> energy.rs -> cost C
//!
//!                         |
//!                         v
//!                  planner chooses
//! ```
//!
//! The candidate schedule remains owned by the scheduling subsystem.
//!
//! # Sparse scalability
//!
//! The evaluator consumes only supplied contributions.
//!
//! It does not:
//!
//! - enumerate all target resources;
//! - allocate one entry per possible qubit;
//! - allocate one entry per possible channel;
//! - allocate one entry per unit of time;
//! - create a timeline proportional to machine size.
//!
//! Consequently a huge target with a small active workload remains sparse.
//!
//! # Streaming and distributed evaluation
//!
//! The primary evaluation API accepts an iterator.
//!
//! Therefore energy can be accumulated:
//!
//! - sequentially;
//! - from streamed schedules;
//! - from partitions;
//! - from distributed workers;
//! - from large generated schedules;
//! - without first materializing every contribution in a `Vec`.
//!
//! Partial results can be combined using [`EnergyScore::combine`].
//!
//! # Numerical safety
//!
//! Energy values must be:
//!
//! - finite;
//! - non-negative.
//!
//! NaN, positive infinity, and negative values are rejected.
//!
//! Accumulation uses checked floating-point validation and compensated
//! summation.
//!
//! A floating-point result that becomes non-finite is rejected.
//!
//! # Determinism
//!
//! The evaluator contains no random state.
//!
//! For mathematically identical input, accumulation order can still affect the
//! final least-significant floating-point bits. Callers requiring bitwise
//! reproducibility should provide contributions in a deterministic order or
//! combine deterministic partitions in a deterministic order.
//!
//! The module itself never iterates over a hash-based collection.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no unsafe code.
//!
//! # Frozen integration contract
//!
//! `energy.rs` intentionally has a narrow dependency surface.
//!
//! The following modules consume it:
//!
//! ```text
//! scheduling::optimization::multi_objective
//! scheduling::planners::*
//! scheduling::policies::*
//! scheduling::diagnostics::*
//! scheduling::result
//! scheduling::adapters::hardware
//! scheduling::adapters::routing
//! scheduling::adapters::qec
//! ```
//!
//! None of those modules should require this file to be rewritten merely
//! because another subsystem is implemented.
//!
//! The integration direction is:
//!
//! ```text
//! target-specific source
//!       |
//!       v
//! EnergyContribution
//!       |
//!       v
//! EnergyObjective
//!       |
//!       v
//! EnergyScore
//!       |
//!       +--------------------+
//!       |                    |
//!       v                    v
//! multi-objective       diagnostics
//!       |
//!       v
//! planner
//! ```
//!
//! # Important separation from `quantum::optimization::cost`
//!
//! The broader optimization cost subsystem already recognizes `Energy` as a
//! cost dimension. This scheduling module does not create a competing global
//! cost-vector abstraction.
//!
//! Instead, it provides the scheduling-specific energy evaluator and can be
//! adapted into the canonical optimization cost model at the integration
//! boundary.
//!
//! This avoids coupling the scheduling module to the implementation details
//! of the general optimization subsystem.
//!
//! # Important separation from hardware
//!
//! Hardware calibration determines energy estimates.
//!
//! This module consumes those estimates.
//!
//! It must never contain code such as:
//!
//!     power = 5.0;
//!     energy = power * duration;
//!
//! unless those values have explicitly arrived from a target model.
//!
//! The absence of such constants is intentional.
//!
//! # Important separation from QEC
//!
//! QEC may cause additional operations, ancilla activity, syndrome rounds,
//! measurement, reset, and classical processing.
//!
//! QEC adapters should express those effects as energy contributions.
//!
//! This file does not know what a stabilizer, surface code, repetition code,
//! color code, or future QEC architecture is.
//!
//! # Important separation from qubit identity
//!
//! This module does not import:
//!
//!     quantum::ir::qubit::QubitId
//!
//! because energy aggregation does not require ownership of qubit identity.
//!
//! If a future diagnostic layer needs to associate a contribution with a
//! canonical qubit, that association belongs in the adapter or diagnostic
//! record and must use the canonical:
//!
//!     quantum::ir::qubit::QubitId
//!
//! It must not introduce a scheduler-local qubit identity.
//!
//! # Testing requirements
//!
//! The embedded tests cover:
//!
//! - empty schedules;
//! - single contributions;
//! - multiple contributions;
//! - zero energy;
//! - large representable energy;
//! - invalid NaN;
//! - invalid infinity;
//! - invalid negative energy;
//! - invalid weights;
//! - weighted objectives;
//! - category weighting;
//! - partial-score combination;
//! - deterministic comparison;
//! - candidate improvement;
//! - streaming evaluation;
//! - zero artificial machine limits.
//!
//! Additional integration tests belong under:
//!
//!     src/quantum/scheduling/tests/
//!
//! and must exercise the real scheduling context, hardware adapter, and
//! multi-objective planner.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::error::Error;
use std::fmt;

// =============================================================================
// Result aliases
// =============================================================================

/// Result type returned by energy optimization operations.
pub type EnergyResult<T> = Result<T, EnergyError>;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by the scheduling energy objective.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnergyError {
    /// An energy value is NaN or infinite.
    NonFiniteEnergy {
        /// Position of the contribution in the caller's logical sequence.
        ordinal: u64,

        /// Invalid value.
        value: f64,
    },

    /// An energy value is negative.
    NegativeEnergy {
        /// Position of the contribution in the caller's logical sequence.
        ordinal: u64,

        /// Invalid value.
        value: f64,
    },

    /// A weight is NaN or infinite.
    NonFiniteWeight {
        /// Category receiving the invalid weight.
        category: EnergyCategory,

        /// Invalid weight.
        value: f64,
    },

    /// A weight is negative.
    NegativeWeight {
        /// Category receiving the invalid weight.
        category: EnergyCategory,

        /// Invalid weight.
        value: f64,
    },

    /// A weighted accumulation became non-finite.
    AccumulationOverflow {
        /// Description of the calculation.
        calculation: &'static str,
    },

    /// Two scores could not be combined because their contribution counts
    /// overflowed the representable counter.
    ContributionCountOverflow,

    /// A score contains an invalid internal value.
    InvalidScore {
        /// Description of the invalid score.
        field: &'static str,
    },

    /// A candidate does not contain a valid energy score.
    InvalidCandidate {
        /// Description of the candidate.
        reason: &'static str,
    },
}

impl fmt::Display for EnergyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteEnergy { ordinal, value } => write!(
                formatter,
                "energy contribution {ordinal} is non-finite: {value}"
            ),

            Self::NegativeEnergy { ordinal, value } => write!(
                formatter,
                "energy contribution {ordinal} is negative: {value}"
            ),

            Self::NonFiniteWeight { category, value } => write!(
                formatter,
                "energy weight for category {category} is non-finite: {value}"
            ),

            Self::NegativeWeight { category, value } => write!(
                formatter,
                "energy weight for category {category} is negative: {value}"
            ),

            Self::AccumulationOverflow { calculation } => write!(
                formatter,
                "energy accumulation became non-finite during {calculation}"
            ),

            Self::ContributionCountOverflow => {
                formatter.write_str("energy contribution count overflowed")
            }

            Self::InvalidScore { field } => {
                write!(formatter, "invalid energy score field `{field}`")
            }

            Self::InvalidCandidate { reason } => {
                write!(formatter, "invalid energy candidate: {reason}")
            }
        }
    }
}

impl Error for EnergyError {}

// =============================================================================
// Energy category
// =============================================================================

/// Classification of an energy contribution.
///
/// Categories are deliberately generic enough to cover different quantum
/// technologies while remaining useful for multi-objective weighting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EnergyCategory {
    /// Energy associated with execution of a quantum operation.
    Operation,

    /// Energy associated with idle periods or maintaining a resource.
    Idle,

    /// Energy associated with measurement/readout.
    Measurement,

    /// Energy associated with reset/reinitialization.
    Reset,

    /// Energy associated with communication or transport.
    Communication,

    /// Energy associated with classical processing required by execution.
    Classical,

    /// Energy associated with generic target resources.
    Resource,

    /// Energy supplied directly by a target-level model.
    Target,

    /// Energy associated with cooling, environmental, or infrastructure
    /// overhead when the target model explicitly attributes it to the
    /// schedule.
    Infrastructure,

    /// User-defined scheduling energy category.
    ///
    /// The numeric identifier is an application-level namespace and is not a
    /// machine-size limit.
    Custom(u32),
}

impl EnergyCategory {
    /// Returns a stable machine-readable name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Operation => "operation",
            Self::Idle => "idle",
            Self::Measurement => "measurement",
            Self::Reset => "reset",
            Self::Communication => "communication",
            Self::Classical => "classical",
            Self::Resource => "resource",
            Self::Target => "target",
            Self::Infrastructure => "infrastructure",
            Self::Custom(_) => "custom",
        }
    }
}

impl fmt::Display for EnergyCategory {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Custom(identifier) => {
                write!(formatter, "custom({identifier})")
            }
            other => formatter.write_str(other.as_str()),
        }
    }
}

// =============================================================================
// Energy contribution
// =============================================================================

/// One normalized energy contribution to a schedule.
///
/// `energy` must already be expressed in the common unit selected by the
/// caller's target model. This module deliberately does not perform unit
/// conversion.
///
/// `ordinal` is a stable source-order identifier supplied by the producer. It
/// is not a quantum operation identity and does not replace canonical IR
/// identities.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EnergyContribution {
    /// Stable source-order position.
    ordinal: u64,

    /// Contribution category.
    category: EnergyCategory,

    /// Non-negative energy estimate in the caller-selected common unit.
    energy: f64,
}

impl EnergyContribution {
    /// Creates a validated energy contribution.
    pub fn new(
        ordinal: u64,
        category: EnergyCategory,
        energy: f64,
    ) -> EnergyResult<Self> {
        validate_energy(ordinal, energy)?;

        Ok(Self {
            ordinal,
            category,
            energy,
        })
    }

    /// Returns the contribution ordinal.
    #[must_use]
    pub const fn ordinal(self) -> u64 {
        self.ordinal
    }

    /// Returns the contribution category.
    #[must_use]
    pub const fn category(self) -> EnergyCategory {
        self.category
    }

    /// Returns the normalized energy value.
    #[must_use]
    pub const fn energy(self) -> f64 {
        self.energy
    }
}

// =============================================================================
// Energy weights
// =============================================================================

/// Weights applied to energy categories.
///
/// A weight of `0.0` disables the category for weighted objective comparison
/// while preserving its contribution in the unweighted energy total.
///
/// Negative weights are forbidden because they would make additional energy
/// beneficial.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EnergyWeights {
    operation: f64,
    idle: f64,
    measurement: f64,
    reset: f64,
    communication: f64,
    classical: f64,
    resource: f64,
    target: f64,
    infrastructure: f64,
    custom: f64,
}

impl Default for EnergyWeights {
    fn default() -> Self {
        Self::uniform(1.0)
    }
}

impl EnergyWeights {
    /// Creates equal weighting for every energy category.
    pub const fn uniform(weight: f64) -> Self {
        Self {
            operation: weight,
            idle: weight,
            measurement: weight,
            reset: weight,
            communication: weight,
            classical: weight,
            resource: weight,
            target: weight,
            infrastructure: weight,
            custom: weight,
        }
    }

    /// Creates zero weighting for all categories.
    pub const fn zero() -> Self {
        Self::uniform(0.0)
    }

    /// Validates the complete weight configuration.
    pub fn validate(self) -> EnergyResult<()> {
        let weights = [
            (EnergyCategory::Operation, self.operation),
            (EnergyCategory::Idle, self.idle),
            (EnergyCategory::Measurement, self.measurement),
            (EnergyCategory::Reset, self.reset),
            (EnergyCategory::Communication, self.communication),
            (EnergyCategory::Classical, self.classical),
            (EnergyCategory::Resource, self.resource),
            (EnergyCategory::Target, self.target),
            (
                EnergyCategory::Infrastructure,
                self.infrastructure,
            ),
            (EnergyCategory::Custom(0), self.custom),
        ];

        for (category, value) in weights {
            validate_weight(category, value)?;
        }

        Ok(())
    }

    /// Returns a copy with an operation weight.
    #[must_use]
    pub const fn with_operation(mut self, weight: f64) -> Self {
        self.operation = weight;
        self
    }

    /// Returns a copy with an idle-energy weight.
    #[must_use]
    pub const fn with_idle(mut self, weight: f64) -> Self {
        self.idle = weight;
        self
    }

    /// Returns a copy with a measurement-energy weight.
    #[must_use]
    pub const fn with_measurement(mut self, weight: f64) -> Self {
        self.measurement = weight;
        self
    }

    /// Returns a copy with a reset-energy weight.
    #[must_use]
    pub const fn with_reset(mut self, weight: f64) -> Self {
        self.reset = weight;
        self
    }

    /// Returns a copy with a communication-energy weight.
    #[must_use]
    pub const fn with_communication(mut self, weight: f64) -> Self {
        self.communication = weight;
        self
    }

    /// Returns a copy with a classical-energy weight.
    #[must_use]
    pub const fn with_classical(mut self, weight: f64) -> Self {
        self.classical = weight;
        self
    }

    /// Returns a copy with a resource-energy weight.
    #[must_use]
    pub const fn with_resource(mut self, weight: f64) -> Self {
        self.resource = weight;
        self
    }

    /// Returns a copy with a target-energy weight.
    #[must_use]
    pub const fn with_target(mut self, weight: f64) -> Self {
        self.target = weight;
        self
    }

    /// Returns a copy with an infrastructure-energy weight.
    #[must_use]
    pub const fn with_infrastructure(mut self, weight: f64) -> Self {
        self.infrastructure = weight;
        self
    }

    /// Returns a copy with a custom-category weight.
    #[must_use]
    pub const fn with_custom(mut self, weight: f64) -> Self {
        self.custom = weight;
        self
    }

    /// Returns the weight for one category.
    #[must_use]
    pub const fn weight_for(self, category: EnergyCategory) -> f64 {
        match category {
            EnergyCategory::Operation => self.operation,
            EnergyCategory::Idle => self.idle,
            EnergyCategory::Measurement => self.measurement,
            EnergyCategory::Reset => self.reset,
            EnergyCategory::Communication => self.communication,
            EnergyCategory::Classical => self.classical,
            EnergyCategory::Resource => self.resource,
            EnergyCategory::Target => self.target,
            EnergyCategory::Infrastructure => self.infrastructure,
            EnergyCategory::Custom(_) => self.custom,
        }
    }

    /// Returns whether at least one objective dimension is active.
    #[must_use]
    pub fn has_active_dimension(self) -> bool {
        [
            self.operation,
            self.idle,
            self.measurement,
            self.reset,
            self.communication,
            self.classical,
            self.resource,
            self.target,
            self.infrastructure,
            self.custom,
        ]
        .iter()
        .any(|value| *value > 0.0)
    }
}

// =============================================================================
// Energy breakdown
// =============================================================================

/// Aggregated energy by category.
///
/// The values are ordinary sums, not weighted objective costs.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EnergyBreakdown {
    /// Operation energy.
    pub operation: f64,

    /// Idle energy.
    pub idle: f64,

    /// Measurement energy.
    pub measurement: f64,

    /// Reset energy.
    pub reset: f64,

    /// Communication energy.
    pub communication: f64,

    /// Classical processing energy.
    pub classical: f64,

    /// Generic resource energy.
    pub resource: f64,

    /// Target-supplied energy.
    pub target: f64,

    /// Infrastructure energy.
    pub infrastructure: f64,

    /// Custom-category energy.
    pub custom: f64,
}

impl Default for EnergyBreakdown {
    fn default() -> Self {
        Self::zero()
    }
}

impl EnergyBreakdown {
    /// Creates an empty breakdown.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            operation: 0.0,
            idle: 0.0,
            measurement: 0.0,
            reset: 0.0,
            communication: 0.0,
            classical: 0.0,
            resource: 0.0,
            target: 0.0,
            infrastructure: 0.0,
            custom: 0.0,
        }
    }

    /// Adds one validated contribution.
    fn add(&mut self, contribution: EnergyContribution) -> EnergyResult<()> {
        let field = contribution.category;

        let target = match field {
            EnergyCategory::Operation => &mut self.operation,
            EnergyCategory::Idle => &mut self.idle,
            EnergyCategory::Measurement => &mut self.measurement,
            EnergyCategory::Reset => &mut self.reset,
            EnergyCategory::Communication => &mut self.communication,
            EnergyCategory::Classical => &mut self.classical,
            EnergyCategory::Resource => &mut self.resource,
            EnergyCategory::Target => &mut self.target,
            EnergyCategory::Infrastructure => &mut self.infrastructure,
            EnergyCategory::Custom(_) => &mut self.custom,
        };

        let next = *target + contribution.energy;

        if !next.is_finite() {
            return Err(EnergyError::AccumulationOverflow {
                calculation: "category energy accumulation",
            });
        }

        *target = next;

        Ok(())
    }

    /// Returns the category energy for the supplied category.
    #[must_use]
    pub const fn energy_for(self, category: EnergyCategory) -> f64 {
        match category {
            EnergyCategory::Operation => self.operation,
            EnergyCategory::Idle => self.idle,
            EnergyCategory::Measurement => self.measurement,
            EnergyCategory::Reset => self.reset,
            EnergyCategory::Communication => self.communication,
            EnergyCategory::Classical => self.classical,
            EnergyCategory::Resource => self.resource,
            EnergyCategory::Target => self.target,
            EnergyCategory::Infrastructure => self.infrastructure,
            EnergyCategory::Custom(_) => self.custom,
        }
    }

    /// Returns the total unweighted energy.
    pub fn total(self) -> EnergyResult<f64> {
        let mut accumulator = CompensatedSum::default();

        accumulator.add(self.operation)?;
        accumulator.add(self.idle)?;
        accumulator.add(self.measurement)?;
        accumulator.add(self.reset)?;
        accumulator.add(self.communication)?;
        accumulator.add(self.classical)?;
        accumulator.add(self.resource)?;
        accumulator.add(self.target)?;
        accumulator.add(self.infrastructure)?;
        accumulator.add(self.custom)?;

        accumulator.finish("breakdown total")
    }

    /// Computes weighted energy from this breakdown.
    pub fn weighted(self, weights: EnergyWeights) -> EnergyResult<f64> {
        weights.validate()?;

        let mut accumulator = CompensatedSum::default();

        accumulator.add(self.operation * weights.operation)?;
        accumulator.add(self.idle * weights.idle)?;
        accumulator.add(self.measurement * weights.measurement)?;
        accumulator.add(self.reset * weights.reset)?;
        accumulator.add(self.communication * weights.communication)?;
        accumulator.add(self.classical * weights.classical)?;
        accumulator.add(self.resource * weights.resource)?;
        accumulator.add(self.target * weights.target)?;
        accumulator.add(
            self.infrastructure * weights.infrastructure,
        )?;
        accumulator.add(self.custom * weights.custom)?;

        accumulator.finish("weighted energy")
    }

    /// Combines two breakdowns.
    pub fn combine(self, other: Self) -> EnergyResult<Self> {
        let mut combined = self;

        combined.operation = checked_add(
            combined.operation,
            other.operation,
            "operation energy",
        )?;

        combined.idle =
            checked_add(combined.idle, other.idle, "idle energy")?;

        combined.measurement = checked_add(
            combined.measurement,
            other.measurement,
            "measurement energy",
        )?;

        combined.reset =
            checked_add(combined.reset, other.reset, "reset energy")?;

        combined.communication = checked_add(
            combined.communication,
            other.communication,
            "communication energy",
        )?;

        combined.classical = checked_add(
            combined.classical,
            other.classical,
            "classical energy",
        )?;

        combined.resource = checked_add(
            combined.resource,
            other.resource,
            "resource energy",
        )?;

        combined.target =
            checked_add(combined.target, other.target, "target energy")?;

        combined.infrastructure = checked_add(
            combined.infrastructure,
            other.infrastructure,
            "infrastructure energy",
        )?;

        combined.custom =
            checked_add(combined.custom, other.custom, "custom energy")?;

        Ok(combined)
    }
}

// =============================================================================
// Energy score
// =============================================================================

/// Result of evaluating the energy objective.
///
/// `total_energy` is the physical/normalized energy value supplied by the
/// target model.
///
/// `weighted_cost` is an optimization scalar and must not be interpreted as a
/// physical energy value when weights differ from one.
///
/// Lower values are better.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EnergyScore {
    /// Unweighted total energy.
    total_energy: f64,

    /// Weighted objective cost.
    weighted_cost: f64,

    /// Per-category breakdown.
    breakdown: EnergyBreakdown,

    /// Number of evaluated contributions.
    contribution_count: u64,
}

impl EnergyScore {
    /// Creates a validated score.
    pub fn new(
        total_energy: f64,
        weighted_cost: f64,
        breakdown: EnergyBreakdown,
        contribution_count: u64,
    ) -> EnergyResult<Self> {
        validate_non_negative_finite(
            total_energy,
            "total energy",
        )?;

        validate_non_negative_finite(
            weighted_cost,
            "weighted energy cost",
        )?;

        let breakdown_total = breakdown.total()?;

        if (breakdown_total - total_energy).abs()
            > energy_consistency_tolerance(total_energy)
        {
            return Err(EnergyError::InvalidScore {
                field: "breakdown does not agree with total energy",
            });
        }

        Ok(Self {
            total_energy,
            weighted_cost,
            breakdown,
            contribution_count,
        })
    }

    /// Returns the total unweighted energy.
    #[must_use]
    pub const fn total_energy(self) -> f64 {
        self.total_energy
    }

    /// Returns the scalar weighted objective cost.
    #[must_use]
    pub const fn weighted_cost(self) -> f64 {
        self.weighted_cost
    }

    /// Returns the energy breakdown.
    #[must_use]
    pub const fn breakdown(self) -> EnergyBreakdown {
        self.breakdown
    }

    /// Returns the number of source contributions.
    #[must_use]
    pub const fn contribution_count(self) -> u64 {
        self.contribution_count
    }

    /// Returns true when this score is strictly better than another score.
    ///
    /// Energy optimization minimizes the weighted objective cost.
    #[must_use]
    pub fn is_better_than(self, other: Self) -> bool {
        self.weighted_cost < other.weighted_cost
    }

    /// Combines two independent partial scores.
    ///
    /// This is useful for partitioned or distributed scheduling.
    pub fn combine(self, other: Self) -> EnergyResult<Self> {
        let contribution_count = self
            .contribution_count
            .checked_add(other.contribution_count)
            .ok_or(EnergyError::ContributionCountOverflow)?;

        let total_energy = checked_add(
            self.total_energy,
            other.total_energy,
            "total energy",
        )?;

        let weighted_cost = checked_add(
            self.weighted_cost,
            other.weighted_cost,
            "weighted energy objective",
        )?;

        let breakdown = self.breakdown.combine(other.breakdown)?;

        Self::new(
            total_energy,
            weighted_cost,
            breakdown,
            contribution_count,
        )
    }
}

// =============================================================================
// Objective configuration
// =============================================================================

/// Configuration of the energy objective.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EnergyObjectiveConfig {
    /// Category weights.
    weights: EnergyWeights,

    /// Whether a zero-energy schedule is accepted.
    ///
    /// Zero is a valid physical/estimated value and is therefore accepted by
    /// default.
    allow_zero: bool,
}

impl Default for EnergyObjectiveConfig {
    fn default() -> Self {
        Self {
            weights: EnergyWeights::default(),
            allow_zero: true,
        }
    }
}

impl EnergyObjectiveConfig {
    /// Creates the default configuration.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            weights: EnergyWeights::uniform(1.0),
            allow_zero: true,
        }
    }

    /// Creates a configuration with explicit weights.
    #[must_use]
    pub const fn with_weights(
        mut self,
        weights: EnergyWeights,
    ) -> Self {
        self.weights = weights;
        self
    }

    /// Enables or disables zero-energy schedules.
    #[must_use]
    pub const fn allow_zero(
        mut self,
        allow_zero: bool,
    ) -> Self {
        self.allow_zero = allow_zero;
        self
    }

    /// Returns the configured weights.
    #[must_use]
    pub const fn weights(self) -> EnergyWeights {
        self.weights
    }

    /// Returns whether zero energy is accepted.
    #[must_use]
    pub const fn zero_allowed(self) -> bool {
        self.allow_zero
    }

    /// Validates this objective configuration.
    pub fn validate(self) -> EnergyResult<()> {
        self.weights.validate()?;

        if !self.weights.has_active_dimension() {
            // A zero-weight objective is allowed as a configuration object,
            // but it cannot be used as the active energy objective.
            return Err(EnergyError::InvalidCandidate {
                reason: "energy objective has no active weight",
            });
        }

        Ok(())
    }
}

// =============================================================================
// Energy objective
// =============================================================================

/// Production energy objective evaluator.
///
/// This type is stateless apart from immutable configuration and is therefore
/// safe to create per scheduling invocation.
///
/// It contains no global state and no hardware-specific state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EnergyObjective {
    config: EnergyObjectiveConfig,
}

impl Default for EnergyObjective {
    fn default() -> Self {
        Self::new()
    }
}

impl EnergyObjective {
    /// Creates the default energy objective.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            config: EnergyObjectiveConfig::new(),
        }
    }

    /// Creates an energy objective from explicit configuration.
    pub fn from_config(
        config: EnergyObjectiveConfig,
    ) -> EnergyResult<Self> {
        config.validate()?;

        Ok(Self { config })
    }

    /// Returns the immutable configuration.
    #[must_use]
    pub const fn config(self) -> EnergyObjectiveConfig {
        self.config
    }

    /// Evaluates a stream of energy contributions.
    ///
    /// No collection proportional to the number of contributions is required.
    ///
    /// The caller remains responsible for ensuring that all contributions use
    /// the same normalized energy unit.
    pub fn evaluate<I>(
        &self,
        contributions: I,
    ) -> EnergyResult<EnergyScore>
    where
        I: IntoIterator<Item = EnergyContribution>,
    {
        self.config.validate()?;

        let mut total = CompensatedSum::default();
        let mut weighted = CompensatedSum::default();
        let mut breakdown = EnergyBreakdown::zero();
        let mut count = 0_u64;

        for contribution in contributions {
            validate_energy(
                contribution.ordinal,
                contribution.energy,
            )?;

            if !self.config.allow_zero && contribution.energy == 0.0 {
                return Err(EnergyError::InvalidCandidate {
                    reason: "zero-energy contribution is disabled",
                });
            }

            let category_weight =
                self.config.weights.weight_for(
                    contribution.category,
                );

            validate_weight(
                contribution.category,
                category_weight,
            )?;

            total.add(contribution.energy)?;

            weighted.add(
                contribution.energy * category_weight,
            )?;

            breakdown.add(contribution)?;

            count = count
                .checked_add(1)
                .ok_or(EnergyError::ContributionCountOverflow)?;
        }

        let total_energy = total.finish("total energy")?;
        let weighted_cost =
            weighted.finish("weighted energy objective")?;

        EnergyScore::new(
            total_energy,
            weighted_cost,
            breakdown,
            count,
        )
    }

    /// Evaluates a single contribution.
    pub fn evaluate_one(
        &self,
        contribution: EnergyContribution,
    ) -> EnergyResult<EnergyScore> {
        self.evaluate(std::iter::once(contribution))
    }

    /// Returns the objective cost of a score under the configured weights.
    ///
    /// This method is primarily useful when a score has been transported or
    /// stored and needs to be interpreted by the same objective configuration.
    pub fn cost(
        &self,
        score: EnergyScore,
    ) -> EnergyResult<f64> {
        self.config.validate()?;
        score.breakdown.weighted(self.config.weights)
    }

    /// Compares two already evaluated candidates.
    ///
    /// Returns `Ordering::Less` when `left` is preferred, `Greater` when
    /// `right` is preferred, and `Equal` when their objective costs are equal.
    #[must_use]
    pub fn compare(
        &self,
        left: EnergyScore,
        right: EnergyScore,
    ) -> std::cmp::Ordering {
        left.weighted_cost.total_cmp(&right.weighted_cost)
    }

    /// Returns whether `candidate` is better than `incumbent`.
    #[must_use]
    pub fn is_better(
        &self,
        candidate: EnergyScore,
        incumbent: EnergyScore,
    ) -> bool {
        self.compare(candidate, incumbent)
            == std::cmp::Ordering::Less
    }
}

// =============================================================================
// Candidate comparison
// =============================================================================

/// A lightweight energy candidate record.
///
/// This deliberately does not contain a schedule. The scheduling subsystem
/// owns schedules; this type only associates a caller-owned candidate identity
/// with an evaluated energy score.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EnergyCandidate {
    /// Caller-defined candidate ordinal.
    ordinal: u64,

    /// Evaluated energy score.
    score: EnergyScore,
}

impl EnergyCandidate {
    /// Creates a candidate.
    pub fn new(
        ordinal: u64,
        score: EnergyScore,
    ) -> EnergyResult<Self> {
        validate_non_negative_finite(
            score.total_energy,
            "candidate total energy",
        )?;

        validate_non_negative_finite(
            score.weighted_cost,
            "candidate weighted energy cost",
        )?;

        Ok(Self { ordinal, score })
    }

    /// Returns the candidate ordinal.
    #[must_use]
    pub const fn ordinal(self) -> u64 {
        self.ordinal
    }

    /// Returns the candidate score.
    #[must_use]
    pub const fn score(self) -> EnergyScore {
        self.score
    }
}

// =============================================================================
// Compensated summation
// =============================================================================

/// Kahan-style compensated summation.
///
/// This improves accumulation stability for very different contribution
/// magnitudes without introducing an external numerical dependency.
#[derive(Debug, Clone, Copy, Default)]
struct CompensatedSum {
    sum: f64,
    compensation: f64,
}

impl CompensatedSum {
    /// Adds a finite non-negative value.
    fn add(&mut self, value: f64) -> EnergyResult<()> {
        if !value.is_finite() {
            return Err(EnergyError::AccumulationOverflow {
                calculation: "compensated sum",
            });
        }

        if value < 0.0 {
            return Err(EnergyError::AccumulationOverflow {
                calculation: "negative compensated sum contribution",
            });
        }

        let adjusted = value - self.compensation;
        let next = self.sum + adjusted;

        if !next.is_finite() {
            return Err(EnergyError::AccumulationOverflow {
                calculation: "compensated sum",
            });
        }

        self.compensation = (next - self.sum) - adjusted;
        self.sum = next;

        Ok(())
    }

    /// Finishes the sum.
    fn finish(
        self,
        calculation: &'static str,
    ) -> EnergyResult<f64> {
        let result = self.sum + self.compensation;

        if !result.is_finite() || result < 0.0 {
            return Err(EnergyError::AccumulationOverflow {
                calculation,
            });
        }

        Ok(result)
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

/// Validates an energy contribution.
fn validate_energy(
    ordinal: u64,
    energy: f64,
) -> EnergyResult<()> {
    if !energy.is_finite() {
        return Err(EnergyError::NonFiniteEnergy {
            ordinal,
            value: energy,
        });
    }

    if energy < 0.0 {
        return Err(EnergyError::NegativeEnergy {
            ordinal,
            value: energy,
        });
    }

    Ok(())
}

/// Validates a weight.
fn validate_weight(
    category: EnergyCategory,
    weight: f64,
) -> EnergyResult<()> {
    if !weight.is_finite() {
        return Err(EnergyError::NonFiniteWeight {
            category,
            value: weight,
        });
    }

    if weight < 0.0 {
        return Err(EnergyError::NegativeWeight {
            category,
            value: weight,
        });
    }

    Ok(())
}

/// Validates a generic finite non-negative value.
fn validate_non_negative_finite(
    value: f64,
    field: &'static str,
) -> EnergyResult<()> {
    if !value.is_finite() || value < 0.0 {
        return Err(EnergyError::InvalidScore { field });
    }

    Ok(())
}

/// Adds two validated non-negative floating-point values.
fn checked_add(
    left: f64,
    right: f64,
    calculation: &'static str,
) -> EnergyResult<f64> {
    if !left.is_finite() || !right.is_finite() {
        return Err(EnergyError::AccumulationOverflow {
            calculation,
        });
    }

    let result = left + right;

    if !result.is_finite() || result < 0.0 {
        return Err(EnergyError::AccumulationOverflow {
            calculation,
        });
    }

    Ok(result)
}

/// Provides a scale-aware tolerance for validating a breakdown against the
/// total.
///
/// This is only a consistency guard against floating-point accumulation; it
/// is not a physical measurement tolerance.
fn energy_consistency_tolerance(
    total: f64,
) -> f64 {
    let scale = total.abs().max(1.0);

    64.0 * f64::EPSILON * scale
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn contribution(
        ordinal: u64,
        category: EnergyCategory,
        energy: f64,
    ) -> EnergyContribution {
        EnergyContribution::new(
            ordinal,
            category,
            energy,
        )
        .expect("test contribution must be valid")
    }

    #[test]
    fn empty_schedule_has_zero_energy() {
        let objective = EnergyObjective::new();

        let score = objective
            .evaluate(std::iter::empty())
            .expect("empty schedule must evaluate");

        assert_eq!(score.total_energy(), 0.0);
        assert_eq!(score.weighted_cost(), 0.0);
        assert_eq!(score.contribution_count(), 0);
    }

    #[test]
    fn single_contribution_is_evaluated() {
        let objective = EnergyObjective::new();

        let score = objective
            .evaluate_one(contribution(
                0,
                EnergyCategory::Operation,
                10.0,
            ))
            .expect("valid contribution");

        assert_eq!(score.total_energy(), 10.0);
        assert_eq!(score.weighted_cost(), 10.0);
        assert_eq!(score.contribution_count(), 1);
    }

    #[test]
    fn categories_are_accumulated_independently() {
        let objective = EnergyObjective::new();

        let contributions = [
            contribution(0, EnergyCategory::Operation, 10.0),
            contribution(1, EnergyCategory::Idle, 5.0),
            contribution(2, EnergyCategory::Measurement, 2.0),
            contribution(3, EnergyCategory::Reset, 3.0),
        ];

        let score = objective
            .evaluate(contributions)
            .expect("valid contributions");

        assert_eq!(score.total_energy(), 20.0);

        let breakdown = score.breakdown();

        assert_eq!(breakdown.operation, 10.0);
        assert_eq!(breakdown.idle, 5.0);
        assert_eq!(breakdown.measurement, 2.0);
        assert_eq!(breakdown.reset, 3.0);
    }

    #[test]
    fn category_weights_change_only_weighted_cost() {
        let weights = EnergyWeights::uniform(1.0)
            .with_operation(2.0)
            .with_idle(0.5);

        let config = EnergyObjectiveConfig::new()
            .with_weights(weights);

        let objective = EnergyObjective::from_config(config)
            .expect("valid configuration");

        let contributions = [
            contribution(0, EnergyCategory::Operation, 10.0),
            contribution(1, EnergyCategory::Idle, 10.0),
        ];

        let score = objective
            .evaluate(contributions)
            .expect("valid contributions");

        assert_eq!(score.total_energy(), 20.0);
        assert_eq!(score.weighted_cost(), 25.0);
    }

    #[test]
    fn zero_weight_preserves_unweighted_energy() {
        let weights = EnergyWeights::uniform(0.0)
            .with_operation(1.0);

        let objective = EnergyObjective::from_config(
            EnergyObjectiveConfig::new()
                .with_weights(weights),
        )
        .expect("operation weight is active");

        let contributions = [
            contribution(0, EnergyCategory::Operation, 10.0),
            contribution(1, EnergyCategory::Idle, 100.0),
        ];

        let score = objective
            .evaluate(contributions)
            .expect("valid contributions");

        assert_eq!(score.total_energy(), 110.0);
        assert_eq!(score.weighted_cost(), 10.0);
    }

    #[test]
    fn zero_energy_is_valid() {
        let objective = EnergyObjective::new();

        let score = objective
            .evaluate_one(contribution(
                0,
                EnergyCategory::Operation,
                0.0,
            ))
            .expect("zero energy is valid");

        assert_eq!(score.total_energy(), 0.0);
    }

    #[test]
    fn negative_energy_is_rejected() {
        let result = EnergyContribution::new(
            0,
            EnergyCategory::Operation,
            -1.0,
        );

        assert!(matches!(
            result,
            Err(EnergyError::NegativeEnergy { .. })
        ));
    }

    #[test]
    fn nan_energy_is_rejected() {
        let result = EnergyContribution::new(
            0,
            EnergyCategory::Operation,
            f64::NAN,
        );

        assert!(matches!(
            result,
            Err(EnergyError::NonFiniteEnergy { .. })
        ));
    }

    #[test]
    fn infinite_energy_is_rejected() {
        let result = EnergyContribution::new(
            0,
            EnergyCategory::Operation,
            f64::INFINITY,
        );

        assert!(matches!(
            result,
            Err(EnergyError::NonFiniteEnergy { .. })
        ));
    }

    #[test]
    fn negative_weight_is_rejected() {
        let weights =
            EnergyWeights::uniform(-1.0);

        assert!(matches!(
            weights.validate(),
            Err(EnergyError::NegativeWeight { .. })
        ));
    }

    #[test]
    fn nan_weight_is_rejected() {
        let weights =
            EnergyWeights::uniform(f64::NAN);

        assert!(matches!(
            weights.validate(),
            Err(EnergyError::NonFiniteWeight { .. })
        ));
    }

    #[test]
    fn zero_active_weights_are_rejected_for_objective() {
        let config =
            EnergyObjectiveConfig::new()
                .with_weights(EnergyWeights::zero());

        assert!(EnergyObjective::from_config(config).is_err());
    }

    #[test]
    fn partial_scores_can_be_combined() {
        let objective = EnergyObjective::new();

        let left = objective
            .evaluate([
                contribution(
                    0,
                    EnergyCategory::Operation,
                    10.0,
                ),
            ])
            .expect("left score");

        let right = objective
            .evaluate([
                contribution(
                    1,
                    EnergyCategory::Measurement,
                    5.0,
                ),
            ])
            .expect("right score");

        let combined = left
            .combine(right)
            .expect("scores must combine");

        assert_eq!(combined.total_energy(), 15.0);
        assert_eq!(combined.weighted_cost(), 15.0);
        assert_eq!(combined.contribution_count(), 2);
    }

    #[test]
    fn lower_energy_candidate_is_better() {
        let objective = EnergyObjective::new();

        let lower = objective
            .evaluate_one(contribution(
                0,
                EnergyCategory::Operation,
                10.0,
            ))
            .expect("lower candidate");

        let higher = objective
            .evaluate_one(contribution(
                1,
                EnergyCategory::Operation,
                20.0,
            ))
            .expect("higher candidate");

        assert!(objective.is_better(lower, higher));
        assert!(!objective.is_better(higher, lower));
    }

    #[test]
    fn candidate_identity_is_not_semantic_operation_identity() {
        let objective = EnergyObjective::new();

        let score = objective
            .evaluate_one(contribution(
                42,
                EnergyCategory::Operation,
                1.0,
            ))
            .expect("score");

        let candidate =
            EnergyCandidate::new(100, score)
                .expect("candidate");

        assert_eq!(candidate.ordinal(), 100);
        assert_eq!(candidate.score().total_energy(), 1.0);
    }

    #[test]
    fn custom_category_is_supported() {
        let weights = EnergyWeights::uniform(1.0)
            .with_custom(3.0);

        let objective = EnergyObjective::from_config(
            EnergyObjectiveConfig::new()
                .with_weights(weights),
        )
        .expect("valid objective");

        let score = objective
            .evaluate_one(contribution(
                0,
                EnergyCategory::Custom(77),
                5.0,
            ))
            .expect("custom category");

        assert_eq!(score.total_energy(), 5.0);
        assert_eq!(score.weighted_cost(), 15.0);
    }

    #[test]
    fn iterator_evaluation_is_streaming() {
        let objective = EnergyObjective::new();

        let score = objective
            .evaluate(
                (0_u64..10_000)
                    .map(|ordinal| {
                        contribution(
                            ordinal,
                            EnergyCategory::Operation,
                            0.001,
                        )
                    }),
            )
            .expect("streaming evaluation");

        assert_eq!(score.contribution_count(), 10_000);
        assert!((score.total_energy() - 10.0).abs() < 1e-12);
    }

    #[test]
    fn ordering_uses_total_cmp() {
        let objective = EnergyObjective::new();

        let left = objective
            .evaluate_one(contribution(
                0,
                EnergyCategory::Operation,
                1.0,
            ))
            .expect("left");

        let right = objective
            .evaluate_one(contribution(
                1,
                EnergyCategory::Operation,
                2.0,
            ))
            .expect("right");

        assert_eq!(
            objective.compare(left, right),
            std::cmp::Ordering::Less
        );
    }

    #[test]
    fn no_fixed_machine_limit_exists() {
        let objective = EnergyObjective::new();

        let contributions = (0_u64..100_000)
            .map(|ordinal| {
                contribution(
                    ordinal,
                    EnergyCategory::Operation,
                    1.0,
                )
            });

        let score = objective
            .evaluate(contributions)
            .expect("large stream");

        assert_eq!(score.contribution_count(), 100_000);
        assert_eq!(score.total_energy(), 100_000.0);
    }
}