//! Zamani Quantum Optimization — Fault-Tolerant Magic-State Resource Model
//!
//! Production-grade, backend-independent magic-state resource accounting for
//! fault-tolerant quantum computation.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                         quantum::ir
//!                              │
//!                              ▼
//!                    optimization::fault_tolerant
//!                              │
//!              ┌───────────────┼────────────────┐
//!              │               │                │
//!              ▼               ▼                ▼
//!          t_count         t_depth          magic_state
//!              │                                │
//!              │                                ├── demand
//!              │                                ├── factories
//!              │                                ├── yield
//!              │                                ├── throughput
//!              │                                ├── buffering
//!              │                                └── resource estimates
//!              │                                │
//!              └────────────────┬───────────────┘
//!                               ▼
//!                    fault-tolerant cost model
//!                               │
//!                               ▼
//!                     routing / scheduling
//!                               │
//!                               ▼
//!                            hardware
//! ```
//!
//! # Purpose
//!
//! This module models the logical magic-state resources required by
//! fault-tolerant quantum computation.
//!
//! The primary use case is Clifford+T computation where logical T/Tdg
//! operations consume non-Clifford resources commonly supplied by magic-state
//! preparation, injection, distillation, cultivation, or another equivalent
//! non-Clifford resource mechanism.
//!
//! This file deliberately separates:
//!
//! - logical T-resource demand;
//! - magic-state consumption;
//! - factory production;
//! - probabilistic factory yield;
//! - factory throughput;
//! - factory buffering;
//! - distillation input/output accounting;
//! - factory parallelism;
//! - lower-bound resource estimates;
//! - conservative integer accounting;
//!
//! from:
//!
//! - circuit rewriting;
//! - T-count optimization;
//! - T-depth optimization;
//! - QEC code implementation;
//! - physical routing;
//! - hardware topology;
//! - pulse scheduling;
//! - QPU execution;
//! - backend APIs;
//! - authentication;
//! - benchmarking orchestration.
//!
//! Those responsibilities belong to their owning subsystems.
//!
//! # Important semantic boundary
//!
//! A logical T gate and a particular physical magic-state factory are NOT the
//! same abstraction.
//!
//! A T operation creates a logical non-Clifford resource demand.
//!
//! A factory is one possible mechanism for supplying that demand.
//!
//! Therefore this module does not hard-code:
//!
//! - surface code;
//! - color code;
//! - a particular lattice-surgery implementation;
//! - a particular vendor;
//! - a particular distillation protocol;
//! - a particular physical gate set.
//!
//! Instead, callers provide a `MagicStateFactory` description.
//!
//! This makes the model suitable for:
//!
//! - surface-code architectures;
//! - color-code architectures;
//! - concatenated-code architectures;
//! - subsystem-code architectures;
//! - magic-state distillation;
//! - magic-state cultivation;
//! - injected resource states;
//! - future non-standard FTQC architectures.
//!
//! # Exactness policy
//!
//! Integer resource quantities are exact within the supplied model.
//!
//! Probabilistic quantities use exact rational numbers rather than floating
//! point.
//!
//! This avoids nondeterminism and threshold errors caused by floating-point
//! comparisons such as:
//!
//! ```text
//! 0.9999999999999999 < 1.0
//! ```
//!
//! Probabilities are represented by `Probability` as a reduced or reducible
//! numerator/denominator pair:
//!
//! ```text
//! 0 <= numerator <= denominator
//! denominator > 0
//! ```
//!
//! # Scaling
//!
//! Resource quantities use `u128`.
//!
//! This permits very large estimates while preserving checked arithmetic.
//!
//! The module does not impose an artificial circuit-size limit.
//!
//! Practical limits are determined by:
//!
//! - the canonical IR;
//! - optimizer limits;
//! - caller-selected resource bounds;
//! - available memory;
//! - available CPU;
//! - the representable `u128` range.
//!
//! No recursive algorithm is required for ordinary resource accounting.
//!
//! Distillation chains are represented iteratively and therefore do not grow
//! the call stack with factory depth.
//!
//! # Determinism
//!
//! The model is deterministic.
//!
//! It uses:
//!
//! - no random number generator;
//! - no global mutable state;
//! - no threads;
//! - no floating-point arithmetic;
//! - no hardware state;
//! - no wall-clock measurements.
//!
//! A factory yield is represented analytically by a probability rather than by
//! sampling.
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
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies.
//!
//! # Integration contract
//!
//! ## `fault_tolerant/mod.rs`
//!
//! Export:
//!
//! ```text
//! pub mod magic_state;
//! ```
//!
//! Recommended public re-exports:
//!
//! ```text
//! pub use magic_state::{
//!     MagicStateFactory,
//!     MagicStateFactorySet,
//!     MagicStateResourceModel,
//!     MagicStateResourceEstimate,
//!     MagicStateDemand,
//!     Probability,
//! };
//! ```
//!
//! ## `fault_tolerant/t_count.rs`
//!
//! `t_count.rs` owns exact structural T/Tdg accounting.
//!
//! This module intentionally does not duplicate circuit traversal.
//!
//! A caller should convert the result of T-count analysis into:
//!
//! ```text
//! MagicStateDemand::from_t_count(...)
//! ```
//!
//! The dependency direction is therefore:
//!
//! ```text
//! t_count → magic_state
//! ```
//!
//! and never:
//!
//! ```text
//! magic_state → t_count
//! ```
//!
//! This prevents a dependency cycle and permits the resource model to remain
//! independently testable.
//!
//! ## `fault_tolerant/t_depth.rs`
//!
//! T-depth remains a separate metric.
//!
//! This module may consume T-depth information when estimating peak demand or
//! factory throughput, but it does not own T-depth analysis.
//!
//! ## `fault_tolerant/t_gate_reduction.rs`
//!
//! T-gate reduction runs before this model when optimization is requested.
//!
//! This module never rewrites the circuit.
//!
//! Therefore:
//!
//! ```text
//! original circuit
//!      │
//!      ▼
//! T optimization
//!      │
//!      ▼
//! final circuit
//!      │
//!      ▼
//! T-count
//!      │
//!      ▼
//! magic-state resource model
//! ```
//!
//! ## `cost.rs`
//!
//! `cost.rs` can consume `MagicStateResourceEstimate` to construct a
//! target-specific fault-tolerant cost.
//!
//! This file does not depend on `cost.rs`.
//!
//! ## `context.rs`
//!
//! The immutable resource model types can be inserted into the typed analysis
//! cache when the pipeline wants to retain them.
//!
//! This file does not require `OptimizationContext`, avoiding an unnecessary
//! infrastructure dependency.
//!
//! ## `verification/*`
//!
//! Verification may validate the logical circuit transformation separately.
//!
//! Magic-state resource estimation is not a semantic equivalence proof.
//!
//! ## `routing`
//!
//! Routing may change physical resource requirements.
//!
//! This module therefore models logical/factory resources only unless the
//! caller explicitly supplies a physical-cost model.
//!
//! ## `scheduling`
//!
//! Scheduling owns execution timing.
//!
//! This file provides factory throughput and demand information that scheduling
//! may consume, but it does not produce a hardware schedule.
//!
//! ## `benchmarking`
//!
//! Benchmarking may consume the immutable result types.
//!
//! This module does not depend on benchmarking.
//!
//! -----------------------------------------------------------------------------
//! Implementation
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]

use std::fmt;

// =============================================================================
// Constants
// =============================================================================

/// Stable schema version for serialized/provenance resource estimates.
pub const MAGIC_STATE_SCHEMA_VERSION: u32 = 1;

/// Stable module identifier.
pub const MODULE_ID: &str = "fault_tolerant.magic_state";

/// Standard logical resource consumed by a conventional T/Tdg injection model.
pub const DEFAULT_STATES_PER_T: u128 = 1;

/// Number of T/Tdg operations represented by one logical magic-state demand
/// unit under the standard one-state-per-injection model.
pub const DEFAULT_T_DEMAND_UNIT: u128 = 1;

// =============================================================================
// Result aliases
// =============================================================================

/// Result type for magic-state resource operations.
pub type MagicStateResult<T> = Result<T, MagicStateError>;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by the magic-state resource model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MagicStateError {
    /// A probability has an invalid denominator or numerator.
    InvalidProbability {
        numerator: u128,
        denominator: u128,
    },

    /// Arithmetic would overflow `u128`.
    ArithmeticOverflow {
        operation: &'static str,
    },

    /// A required resource quantity is zero.
    ZeroResource {
        resource: &'static str,
    },

    /// A factory has no useful output.
    ZeroFactoryOutput,

    /// A factory has an invalid number of input states.
    InvalidFactoryInputs,

    /// A factory has an invalid number of output states.
    InvalidFactoryOutputs,

    /// A factory has a zero or invalid production interval.
    InvalidProductionTime,

    /// A factory has an invalid parallelism value.
    InvalidParallelism,

    /// A distillation chain contains an invalid level.
    InvalidDistillationLevel {
        level: usize,
    },

    /// The requested demand cannot be represented.
    DemandOverflow,

    /// The requested number of factories cannot be represented.
    FactoryCountOverflow,

    /// A throughput calculation cannot be represented.
    ThroughputOverflow,

    /// A supplied factory set is empty.
    EmptyFactorySet,

    /// A factory identifier is empty.
    EmptyFactoryId,

    /// A factory identifier occurs more than once.
    DuplicateFactoryId,

    /// A requested magic-state kind is not supplied.
    UnsupportedStateKind {
        kind: MagicStateKind,
    },

    /// A requested factory does not exist.
    UnknownFactory {
        id: String,
    },

    /// A configuration is internally inconsistent.
    InvalidConfiguration {
        message: &'static str,
    },

    /// A resource model cannot satisfy a requested condition.
    Infeasible {
        reason: &'static str,
    },
}

impl fmt::Display for MagicStateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidProbability {
                numerator,
                denominator,
            } => write!(
                f,
                "invalid probability {numerator}/{denominator}; \
                 expected denominator > 0 and numerator <= denominator"
            ),

            Self::ArithmeticOverflow { operation } => {
                write!(f, "u128 arithmetic overflow during {operation}")
            }

            Self::ZeroResource { resource } => {
                write!(f, "{resource} must be greater than zero")
            }

            Self::ZeroFactoryOutput => {
                f.write_str("magic-state factory must produce at least one state")
            }

            Self::InvalidFactoryInputs => {
                f.write_str("magic-state factory input count is invalid")
            }

            Self::InvalidFactoryOutputs => {
                f.write_str("magic-state factory output count is invalid")
            }

            Self::InvalidProductionTime => {
                f.write_str("factory production time must be greater than zero")
            }

            Self::InvalidParallelism => {
                f.write_str("factory parallelism must be greater than zero")
            }

            Self::InvalidDistillationLevel { level } => {
                write!(f, "invalid distillation level {level}")
            }

            Self::DemandOverflow => {
                f.write_str("magic-state demand exceeds representable range")
            }

            Self::FactoryCountOverflow => {
                f.write_str("required factory count exceeds representable range")
            }

            Self::ThroughputOverflow => {
                f.write_str("factory throughput exceeds representable range")
            }

            Self::EmptyFactorySet => {
                f.write_str("magic-state factory set must not be empty")
            }

            Self::EmptyFactoryId => {
                f.write_str("magic-state factory identifier must not be empty")
            }

            Self::DuplicateFactoryId => {
                f.write_str("magic-state factory identifier must be unique")
            }

            Self::UnsupportedStateKind { kind } => {
                write!(f, "unsupported magic-state kind: {kind}")
            }

            Self::UnknownFactory { id } => {
                write!(f, "unknown magic-state factory `{id}`")
            }

            Self::InvalidConfiguration { message } => {
                write!(f, "invalid magic-state configuration: {message}")
            }

            Self::Infeasible { reason } => {
                write!(f, "magic-state resource model is infeasible: {reason}")
            }
        }
    }
}

impl std::error::Error for MagicStateError {}

// =============================================================================
// Probability
// =============================================================================

/// An exact probability represented as a rational number.
///
/// The invariant is:
///
/// ```text
/// denominator > 0
/// numerator <= denominator
/// ```
///
/// The value is not required to be reduced. This is intentional because
/// multiplication can otherwise require unnecessary GCD work on enormous
/// values. `normalized()` is available when canonical representation is
/// required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Probability {
    numerator: u128,
    denominator: u128,
}

impl Probability {
    /// Exactly zero.
    pub const ZERO: Self = Self {
        numerator: 0,
        denominator: 1,
    };

    /// Exactly one.
    pub const ONE: Self = Self {
        numerator: 1,
        denominator: 1,
    };

    /// Constructs a probability after validating its bounds.
    pub const fn new(
        numerator: u128,
        denominator: u128,
    ) -> MagicStateResult<Self> {
        if denominator == 0 || numerator > denominator {
            return Err(MagicStateError::InvalidProbability {
                numerator,
                denominator,
            });
        }

        Ok(Self {
            numerator,
            denominator,
        })
    }

    /// Returns the numerator.
    #[must_use]
    pub const fn numerator(self) -> u128 {
        self.numerator
    }

    /// Returns the denominator.
    #[must_use]
    pub const fn denominator(self) -> u128 {
        self.denominator
    }

    /// Returns whether the probability is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.numerator == 0
    }

    /// Returns whether the probability is exactly one.
    #[must_use]
    pub const fn is_one(self) -> bool {
        self.numerator == self.denominator
    }

    /// Returns the complement probability.
    pub fn complement(self) -> MagicStateResult<Self> {
        Self::new(
            self.denominator - self.numerator,
            self.denominator,
        )
    }

    /// Multiplies two probabilities exactly.
    pub fn multiply(self, other: Self) -> MagicStateResult<Self> {
        let numerator = self
            .numerator
            .checked_mul(other.numerator)
            .ok_or(MagicStateError::ArithmeticOverflow {
                operation: "probability numerator multiplication",
            })?;

        let denominator = self
            .denominator
            .checked_mul(other.denominator)
            .ok_or(MagicStateError::ArithmeticOverflow {
                operation: "probability denominator multiplication",
            })?;

        Self::new(numerator, denominator)
    }

    /// Raises a probability to an unsigned integer power.
    pub fn pow(self, exponent: u32) -> MagicStateResult<Self> {
        let mut result = Self::ONE;
        let mut base = self;
        let mut remaining = exponent;

        while remaining != 0 {
            if remaining & 1 == 1 {
                result = result.multiply(base)?;
            }

            remaining >>= 1;

            if remaining != 0 {
                base = base.multiply(base)?;
            }
        }

        Ok(result)
    }

    /// Returns a reduced rational representation.
    pub fn normalized(self) -> Self {
        if self.numerator == 0 {
            return Self::ZERO;
        }

        if self.numerator == self.denominator {
            return Self::ONE;
        }

        let divisor = gcd(self.numerator, self.denominator);

        Self {
            numerator: self.numerator / divisor,
            denominator: self.denominator / divisor,
        }
    }
}

impl fmt::Display for Probability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let normalized = self.normalized();

        if normalized.denominator == 1 {
            return write!(f, "{}", normalized.numerator);
        }

        write!(
            f,
            "{}/{}",
            normalized.numerator,
            normalized.denominator
        )
    }
}

// =============================================================================
// Magic-state kinds
// =============================================================================

/// Logical magic-state resource families.
///
/// `T` represents the standard |A> / T-state resource used by conventional
/// Clifford+T injection.
///
/// `H` represents a distinct Hadamard-type magic resource.
///
/// `Custom` allows future non-Clifford resource families without modifying
/// this enum's core semantics.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MagicStateKind {
    /// Standard T-state / |A> resource.
    T,

    /// Hadamard-type magic state.
    H,

    /// Named architecture-specific or algorithm-specific resource.
    Custom(String),
}

impl MagicStateKind {
    /// Returns the canonical stable identifier.
    #[must_use]
    pub fn identifier(&self) -> &str {
        match self {
            Self::T => "T",
            Self::H => "H",
            Self::Custom(value) => value.as_str(),
        }
    }
}

impl fmt::Display for MagicStateKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.identifier())
    }
}

// =============================================================================
// Logical magic-state demand
// =============================================================================

/// Logical magic-state demand before physical factory expansion.
///
/// This type is intentionally independent of a circuit representation.
///
/// A T-count analysis can construct it without requiring this module to know
/// how the circuit was represented or optimized.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MagicStateDemand {
    /// Number of logical T-family resource consumptions.
    t_states: u128,

    /// Number of H-type resource consumptions.
    h_states: u128,
}

impl MagicStateDemand {
    /// Creates a demand from explicit resource counts.
    pub const fn new(
        t_states: u128,
        h_states: u128,
    ) -> Self {
        Self {
            t_states,
            h_states,
        }
    }

    /// Creates demand from a T-family count.
    ///
    /// Under the conventional one-state-per-T/Tdg injection model:
    ///
    /// ```text
    /// T + Tdg = one magic-state demand each
    /// ```
    #[must_use]
    pub const fn from_t_count(t_count: u128) -> Self {
        Self {
            t_states: t_count,
            h_states: 0,
        }
    }

    /// Creates demand from separate T and Tdg counts.
    #[must_use]
    pub const fn from_t_and_tdg(
        t_count: u128,
        tdg_count: u128,
    ) -> MagicStateResult<Self> {
        let total = match t_count.checked_add(tdg_count) {
            Some(value) => value,
            None => {
                return Err(MagicStateError::DemandOverflow);
            }
        };

        Ok(Self::from_t_count(total))
    }

    /// Returns T-state demand.
    #[must_use]
    pub const fn t_states(self) -> u128 {
        self.t_states
    }

    /// Returns H-state demand.
    #[must_use]
    pub const fn h_states(self) -> u128 {
        self.h_states
    }

    /// Returns total logical magic-state demand.
    pub fn total_states(self) -> MagicStateResult<u128> {
        self.t_states
            .checked_add(self.h_states)
            .ok_or(MagicStateError::DemandOverflow)
    }

    /// Returns whether no magic states are required.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.t_states == 0 && self.h_states == 0
    }

    /// Adds another demand.
    pub fn checked_add(
        self,
        other: Self,
    ) -> MagicStateResult<Self> {
        Ok(Self {
            t_states: self
                .t_states
                .checked_add(other.t_states)
                .ok_or(MagicStateError::DemandOverflow)?,

            h_states: self
                .h_states
                .checked_add(other.h_states)
                .ok_or(MagicStateError::DemandOverflow)?,
        })
    }
}

// =============================================================================
// Factory description
// =============================================================================

/// A single logical magic-state factory description.
///
/// A factory consumes some number of input resources and attempts to produce
/// some number of output magic states.
///
/// The model is intentionally protocol-neutral.
///
/// For deterministic factories:
///
/// ```text
/// success_probability = 1
/// ```
///
/// For probabilistic distillation:
///
/// ```text
/// success_probability < 1
/// ```
///
/// `parallelism` represents the number of identical factory instances
/// represented by this factory configuration.
///
/// `production_time` is an abstract integer time unit. The interpretation of
/// that unit belongs to the caller/target profile.
///
/// This avoids coupling the optimization layer to seconds, cycles, lattice
/// surgery rounds, code cycles, or physical pulse durations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MagicStateFactory {
    id: String,
    input_kind: MagicStateKind,
    output_kind: MagicStateKind,
    input_states: u128,
    output_states: u128,
    success_probability: Probability,
    production_time: u128,
    parallelism: u128,
}

impl MagicStateFactory {
    /// Creates a validated factory description.
    pub fn new(
        id: impl Into<String>,
        input_kind: MagicStateKind,
        output_kind: MagicStateKind,
        input_states: u128,
        output_states: u128,
        success_probability: Probability,
        production_time: u128,
        parallelism: u128,
    ) -> MagicStateResult<Self> {
        let id = id.into();

        if id.is_empty() {
            return Err(MagicStateError::EmptyFactoryId);
        }

        if input_states == 0 {
            return Err(MagicStateError::InvalidFactoryInputs);
        }

        if output_states == 0 {
            return Err(MagicStateError::ZeroFactoryOutput);
        }

        if production_time == 0 {
            return Err(MagicStateError::InvalidProductionTime);
        }

        if parallelism == 0 {
            return Err(MagicStateError::InvalidParallelism);
        }

        Ok(Self {
            id,
            input_kind,
            output_kind,
            input_states,
            output_states,
            success_probability,
            production_time,
            parallelism,
        })
    }

    /// Returns the stable factory identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the input state kind.
    #[must_use]
    pub const fn input_kind(&self) -> &MagicStateKind {
        &self.input_kind
    }

    /// Returns the output state kind.
    #[must_use]
    pub const fn output_kind(&self) -> &MagicStateKind {
        &self.output_kind
    }

    /// Returns the number of input states consumed per attempt.
    #[must_use]
    pub const fn input_states(&self) -> u128 {
        self.input_states
    }

    /// Returns the number of output states produced per successful attempt.
    #[must_use]
    pub const fn output_states(&self) -> u128 {
        self.output_states
    }

    /// Returns the probability that one factory attempt succeeds.
    #[must_use]
    pub const fn success_probability(&self) -> Probability {
        self.success_probability
    }

    /// Returns the abstract production time of one attempt.
    #[must_use]
    pub const fn production_time(&self) -> u128 {
        self.production_time
    }

    /// Returns the number of parallel factory instances.
    #[must_use]
    pub const fn parallelism(&self) -> u128 {
        self.parallelism
    }

    /// Returns the expected successful output per factory attempt as a rational
    /// quantity.
    pub fn expected_output_per_attempt(
        &self,
    ) -> MagicStateResult<ProbabilityAmount> {
        ProbabilityAmount::new(
            self.output_states
                .checked_mul(self.success_probability.numerator())
                .ok_or(MagicStateError::ArithmeticOverflow {
                    operation: "factory expected output multiplication",
                })?,
            self.success_probability.denominator(),
        )
    }

    /// Returns the maximum deterministic output rate of all represented
    /// parallel instances, ignoring stochastic yield.
    ///
    /// This is a structural upper bound.
    pub fn peak_output_rate(
        &self,
    ) -> MagicStateResult<Rate> {
        let numerator = self
            .output_states
            .checked_mul(self.parallelism)
            .ok_or(MagicStateError::ThroughputOverflow)?;

        Rate::new(numerator, self.production_time)
    }

    /// Returns the expected output rate accounting for success probability.
    pub fn expected_output_rate(
        &self,
    ) -> MagicStateResult<ProbabilityRate> {
        let numerator = self
            .output_states
            .checked_mul(self.parallelism)
            .ok_or(MagicStateError::ThroughputOverflow)?;

        ProbabilityRate::new(
            numerator
                .checked_mul(self.success_probability.numerator())
                .ok_or(MagicStateError::ThroughputOverflow)?,
            self.production_time
                .checked_mul(self.success_probability.denominator())
                .ok_or(MagicStateError::ThroughputOverflow)?,
        )
    }
}

// =============================================================================
// Probability amount
// =============================================================================

/// A non-negative rational resource amount.
///
/// This is used for expected resource production without converting to
/// floating-point numbers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProbabilityAmount {
    numerator: u128,
    denominator: u128,
}

impl ProbabilityAmount {
    /// Creates a rational amount.
    pub const fn new(
        numerator: u128,
        denominator: u128,
    ) -> MagicStateResult<Self> {
        if denominator == 0 {
            return Err(MagicStateError::InvalidProbability {
                numerator,
                denominator,
            });
        }

        Ok(Self {
            numerator,
            denominator,
        })
    }

    /// Returns numerator.
    #[must_use]
    pub const fn numerator(self) -> u128 {
        self.numerator
    }

    /// Returns denominator.
    #[must_use]
    pub const fn denominator(self) -> u128 {
        self.denominator
    }
}

// =============================================================================
// Rate
// =============================================================================

/// Deterministic resource production rate.
///
/// Represents:
///
/// ```text
/// numerator / denominator
/// ```
///
/// states per abstract time unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rate {
    numerator: u128,
    denominator: u128,
}

impl Rate {
    /// Creates a positive rate.
    pub const fn new(
        numerator: u128,
        denominator: u128,
    ) -> MagicStateResult<Self> {
        if numerator == 0 {
            return Err(MagicStateError::ZeroFactoryOutput);
        }

        if denominator == 0 {
            return Err(MagicStateError::InvalidProductionTime);
        }

        Ok(Self {
            numerator,
            denominator,
        })
    }

    /// Returns numerator.
    #[must_use]
    pub const fn numerator(self) -> u128 {
        self.numerator
    }

    /// Returns denominator.
    #[must_use]
    pub const fn denominator(self) -> u128 {
        self.denominator
    }
}

/// Expected resource production rate.
///
/// This may be fractional because factory success is probabilistic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProbabilityRate {
    numerator: u128,
    denominator: u128,
}

impl ProbabilityRate {
    /// Creates an expected rate.
    pub const fn new(
        numerator: u128,
        denominator: u128,
    ) -> MagicStateResult<Self> {
        if denominator == 0 {
            return Err(MagicStateError::InvalidProductionTime);
        }

        Ok(Self {
            numerator,
            denominator,
        })
    }

    /// Returns numerator.
    #[must_use]
    pub const fn numerator(self) -> u128 {
        self.numerator
    }

    /// Returns denominator.
    #[must_use]
    pub const fn denominator(self) -> u128 {
        self.denominator
    }
}

// =============================================================================
// Factory set
// =============================================================================

/// A validated collection of magic-state factories.
///
/// The collection owns no hardware resources. It is a pure logical resource
/// model.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MagicStateFactorySet {
    factories: Vec<MagicStateFactory>,
}

impl MagicStateFactorySet {
    /// Creates an empty factory set.
    ///
    /// This is useful while incrementally constructing a configuration.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            factories: Vec::new(),
        }
    }

    /// Adds a factory while enforcing unique identifiers.
    pub fn push(
        &mut self,
        factory: MagicStateFactory,
    ) -> MagicStateResult<()> {
        if self
            .factories
            .iter()
            .any(|existing| existing.id() == factory.id())
        {
            return Err(MagicStateError::DuplicateFactoryId);
        }

        self.factories.push(factory);

        Ok(())
    }

    /// Returns the number of configured factories.
    #[must_use]
    pub fn len(&self) -> usize {
        self.factories.len()
    }

    /// Returns whether no factories are configured.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.factories.is_empty()
    }

    /// Returns all factories.
    #[must_use]
    pub fn factories(&self) -> &[MagicStateFactory] {
        &self.factories
    }

    /// Finds a factory by stable identifier.
    #[must_use]
    pub fn get(&self, id: &str) -> Option<&MagicStateFactory> {
        self.factories.iter().find(|factory| factory.id() == id)
    }

    /// Finds a factory that produces the requested state kind.
    ///
    /// The first matching factory is returned. Policy-based selection belongs
    /// to `MagicStateResourceModel`.
    #[must_use]
    pub fn find_output(
        &self,
        kind: &MagicStateKind,
    ) -> Option<&MagicStateFactory> {
        self.factories
            .iter()
            .find(|factory| factory.output_kind() == kind)
    }
}

// =============================================================================
// Distillation stage
// =============================================================================

/// One stage in a magic-state distillation/cultivation chain.
///
/// A stage consumes `input_states` lower-fidelity states and attempts to
/// produce `output_states` higher-fidelity states.
///
/// The stage is intentionally generic and does not prescribe a named protocol.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DistillationStage {
    level: usize,
    input_kind: MagicStateKind,
    output_kind: MagicStateKind,
    input_states: u128,
    output_states: u128,
    success_probability: Probability,
}

impl DistillationStage {
    /// Creates a validated distillation stage.
    pub fn new(
        level: usize,
        input_kind: MagicStateKind,
        output_kind: MagicStateKind,
        input_states: u128,
        output_states: u128,
        success_probability: Probability,
    ) -> MagicStateResult<Self> {
        if input_states == 0 {
            return Err(MagicStateError::InvalidFactoryInputs);
        }

        if output_states == 0 {
            return Err(MagicStateError::InvalidFactoryOutputs);
        }

        Ok(Self {
            level,
            input_kind,
            output_kind,
            input_states,
            output_states,
            success_probability,
        })
    }

    /// Returns the chain level.
    #[must_use]
    pub const fn level(&self) -> usize {
        self.level
    }

    /// Returns the input resource kind.
    #[must_use]
    pub const fn input_kind(&self) -> &MagicStateKind {
        &self.input_kind
    }

    /// Returns the output resource kind.
    #[must_use]
    pub const fn output_kind(&self) -> &MagicStateKind {
        &self.output_kind
    }

    /// Returns input state count per attempt.
    #[must_use]
    pub const fn input_states(&self) -> u128 {
        self.input_states
    }

    /// Returns successful output state count.
    #[must_use]
    pub const fn output_states(&self) -> u128 {
        self.output_states
    }

    /// Returns success probability.
    #[must_use]
    pub const fn success_probability(&self) -> Probability {
        self.success_probability
    }
}

// =============================================================================
// Resource model
// =============================================================================

/// Complete logical magic-state resource model.
///
/// This is the primary high-level API for compiler/resource-estimation users.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MagicStateResourceModel {
    /// Logical resource demand.
    demand: MagicStateDemand,

    /// Number of input states required per requested output state.
    states_per_output: u128,

    /// Factory production configuration.
    factories: MagicStateFactorySet,

    /// Optional distillation chain.
    distillation: Vec<DistillationStage>,

    /// Optional required output-state buffer.
    buffer_capacity: u128,
}

impl MagicStateResourceModel {
    /// Creates a model using one magic state per logical T-family operation.
    pub fn new(
        demand: MagicStateDemand,
        factories: MagicStateFactorySet,
    ) -> MagicStateResult<Self> {
        if factories.is_empty() {
            return Err(MagicStateError::EmptyFactorySet);
        }

        Ok(Self {
            demand,
            states_per_output: DEFAULT_STATES_PER_T,
            factories,
            distillation: Vec::new(),
            buffer_capacity: 0,
        })
    }

    /// Sets the logical states consumed by each output-level operation.
    pub fn with_states_per_output(
        mut self,
        states_per_output: u128,
    ) -> MagicStateResult<Self> {
        if states_per_output == 0 {
            return Err(MagicStateError::ZeroResource {
                resource: "states_per_output",
            });
        }

        self.states_per_output = states_per_output;

        Ok(self)
    }

    /// Adds a distillation stage.
    pub fn with_distillation_stage(
        mut self,
        stage: DistillationStage,
    ) -> MagicStateResult<Self> {
        if let Some(previous) = self.distillation.last() {
            if stage.level() <= previous.level() {
                return Err(MagicStateError::InvalidConfiguration {
                    message: "distillation stages must have strictly increasing levels",
                });
            }

            if stage.input_kind() != previous.output_kind() {
                return Err(MagicStateError::InvalidConfiguration {
                    message: "distillation stages must form a compatible resource chain",
                });
            }
        }

        self.distillation.push(stage);

        Ok(self)
    }

    /// Sets the desired logical buffer capacity.
    #[must_use]
    pub fn with_buffer_capacity(
        mut self,
        buffer_capacity: u128,
    ) -> Self {
        self.buffer_capacity = buffer_capacity;
        self
    }

    /// Returns the logical demand.
    #[must_use]
    pub const fn demand(&self) -> MagicStateDemand {
        self.demand
    }

    /// Returns the configured factories.
    #[must_use]
    pub fn factories(&self) -> &MagicStateFactorySet {
        &self.factories
    }

    /// Returns the configured distillation chain.
    #[must_use]
    pub fn distillation(&self) -> &[DistillationStage] {
        &self.distillation
    }

    /// Returns configured buffer capacity.
    #[must_use]
    pub const fn buffer_capacity(&self) -> u128 {
        self.buffer_capacity
    }

    /// Returns the number of logical T-state outputs required.
    ///
    /// This accounts only for logical demand and the configured
    /// `states_per_output`.
    pub fn required_t_state_inputs(&self) -> MagicStateResult<u128> {
        self.demand
            .t_states()
            .checked_mul(self.states_per_output)
            .ok_or(MagicStateError::DemandOverflow)
    }

    /// Returns a conservative lower bound on the number of successful factory
    /// output states required.
    ///
    /// The lower bound ignores stochastic failures and therefore cannot
    /// underestimate the number of *successful* output states required.
    pub fn required_successful_outputs(
        &self,
    ) -> MagicStateResult<u128> {
        self.required_t_state_inputs()
    }

    /// Returns the expected number of factory attempts required to produce the
    /// requested number of outputs from a selected factory.
    ///
    /// For a factory producing `k` states with success probability `p`:
    ///
    /// ```text
    /// expected attempts =
    ///     ceil(required_outputs / (k * p))
    /// ```
    ///
    /// The calculation remains exact by using rational arithmetic.
    pub fn expected_attempts(
        &self,
        factory_id: &str,
    ) -> MagicStateResult<u128> {
        let factory = self
            .factories
            .get(factory_id)
            .ok_or_else(|| MagicStateError::UnknownFactory {
                id: factory_id.to_owned(),
            })?;

        let required = self.required_successful_outputs()?;

        if required == 0 {
            return Ok(0);
        }

        let expected_numerator = factory
            .output_states()
            .checked_mul(factory.success_probability().numerator())
            .ok_or(MagicStateError::ArithmeticOverflow {
                operation: "expected factory output numerator",
            })?;

        let expected_denominator =
            factory.success_probability().denominator();

        if expected_numerator == 0 {
            return Err(MagicStateError::Infeasible {
                reason: "factory has zero expected successful output",
            });
        }

        let numerator = required
            .checked_mul(expected_denominator)
            .ok_or(MagicStateError::FactoryCountOverflow)?;

        Ok(ceil_div(numerator, expected_numerator))
    }

    /// Returns a deterministic lower bound on factory attempts.
    ///
    /// This ignores failed attempts and therefore represents the absolute
    /// optimistic lower bound.
    pub fn optimistic_attempts(
        &self,
        factory_id: &str,
    ) -> MagicStateResult<u128> {
        let factory = self
            .factories
            .get(factory_id)
            .ok_or_else(|| MagicStateError::UnknownFactory {
                id: factory_id.to_owned(),
            })?;

        let required = self.required_successful_outputs()?;

        if required == 0 {
            return Ok(0);
        }

        Ok(ceil_div(
            required,
            factory.output_states(),
        ))
    }

    /// Returns the number of input states consumed by the selected factory
    /// under the expected-attempt model.
    pub fn expected_input_states(
        &self,
        factory_id: &str,
    ) -> MagicStateResult<u128> {
        let factory = self
            .factories
            .get(factory_id)
            .ok_or_else(|| MagicStateError::UnknownFactory {
                id: factory_id.to_owned(),
            })?;

        let attempts = self.expected_attempts(factory_id)?;

        attempts
            .checked_mul(factory.input_states())
            .ok_or(MagicStateError::ArithmeticOverflow {
                operation: "expected factory input-state consumption",
            })
    }

    /// Estimates the minimum number of parallel factory instances required to
    /// sustain a requested demand rate.
    ///
    /// `demand_numerator / demand_denominator` is the requested logical
    /// state-production rate.
    ///
    /// Factory expected output rate is:
    ///
    /// ```text
    /// parallelism * output_states * success_probability
    /// -------------------------------------------------
    /// production_time
    /// ```
    pub fn required_parallel_factories_for_rate(
        &self,
        factory_id: &str,
        demand_numerator: u128,
        demand_denominator: u128,
    ) -> MagicStateResult<u128> {
        if demand_denominator == 0 {
            return Err(MagicStateError::InvalidConfiguration {
                message: "demand rate denominator must be non-zero",
            });
        }

        if demand_numerator == 0 {
            return Ok(0);
        }

        let factory = self
            .factories
            .get(factory_id)
            .ok_or_else(|| MagicStateError::UnknownFactory {
                id: factory_id.to_owned(),
            })?;

        let success_numerator = factory.success_probability().numerator();

        if success_numerator == 0 {
            return Err(MagicStateError::Infeasible {
                reason: "factory cannot produce successful states",
            });
        }

        let per_instance_numerator = factory
            .output_states()
            .checked_mul(success_numerator)
            .ok_or(MagicStateError::ThroughputOverflow)?;

        let per_instance_denominator = factory
            .production_time()
            .checked_mul(
                factory.success_probability().denominator(),
            )
            .ok_or(MagicStateError::ThroughputOverflow)?;

        let required_numerator = demand_numerator
            .checked_mul(per_instance_denominator)
            .ok_or(MagicStateError::FactoryCountOverflow)?;

        let required_denominator = demand_denominator
            .checked_mul(per_instance_numerator)
            .ok_or(MagicStateError::FactoryCountOverflow)?;

        if required_denominator == 0 {
            return Err(MagicStateError::FactoryCountOverflow);
        }

        Ok(ceil_div(
            required_numerator,
            required_denominator,
        ))
    }

    /// Produces a complete immutable resource estimate using one selected
    /// factory.
    pub fn estimate(
        &self,
        factory_id: &str,
    ) -> MagicStateResult<MagicStateResourceEstimate> {
        let factory = self
            .factories
            .get(factory_id)
            .ok_or_else(|| MagicStateError::UnknownFactory {
                id: factory_id.to_owned(),
            })?;

        let successful_outputs =
            self.required_successful_outputs()?;

        let optimistic_attempts =
            self.optimistic_attempts(factory_id)?;

        let expected_attempts =
            self.expected_attempts(factory_id)?;

        let expected_inputs =
            self.expected_input_states(factory_id)?;

        let peak_rate =
            factory.peak_output_rate()?;

        let expected_rate =
            factory.expected_output_rate()?;

        let expected_time_numerator = expected_attempts
            .checked_mul(factory.production_time())
            .ok_or(MagicStateError::ArithmeticOverflow {
                operation: "expected factory production time",
            })?;

        let total_parallelism =
            factory.parallelism();

        let factories_needed = self
            .required_parallel_factories_for_rate(
                factory_id,
                successful_outputs,
                1,
            )?;

        Ok(MagicStateResourceEstimate {
            factory_id: factory.id().to_owned(),
            state_kind: factory.output_kind().clone(),
            logical_demand: self.demand,
            successful_output_states: successful_outputs,
            optimistic_attempts,
            expected_attempts,
            expected_input_states: expected_inputs,
            factory_parallelism: total_parallelism,
            minimum_parallel_factories: factories_needed,
            optimistic_production_time: optimistic_attempts
                .checked_mul(factory.production_time())
                .ok_or(MagicStateError::ArithmeticOverflow {
                    operation: "optimistic production time",
                })?,
            expected_production_time: expected_time_numerator,
            production_time_unit: factory.production_time(),
            peak_output_rate: peak_rate,
            expected_output_rate: expected_rate,
            buffer_capacity: self.buffer_capacity,
        })
    }
}

// =============================================================================
// Resource estimate
// =============================================================================

/// Immutable result of magic-state resource estimation.
///
/// The result contains no references to the mutable optimizer context or
/// circuit. It can therefore safely be retained for provenance, benchmarking,
/// diagnostics, or later scheduling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MagicStateResourceEstimate {
    factory_id: String,
    state_kind: MagicStateKind,
    logical_demand: MagicStateDemand,
    successful_output_states: u128,
    optimistic_attempts: u128,
    expected_attempts: u128,
    expected_input_states: u128,
    factory_parallelism: u128,
    minimum_parallel_factories: u128,
    optimistic_production_time: u128,
    expected_production_time: u128,
    production_time_unit: u128,
    peak_output_rate: Rate,
    expected_output_rate: ProbabilityRate,
    buffer_capacity: u128,
}

impl MagicStateResourceEstimate {
    /// Returns selected factory identifier.
    #[must_use]
    pub fn factory_id(&self) -> &str {
        &self.factory_id
    }

    /// Returns produced state kind.
    #[must_use]
    pub const fn state_kind(&self) -> &MagicStateKind {
        &self.state_kind
    }

    /// Returns logical demand.
    #[must_use]
    pub const fn logical_demand(&self) -> MagicStateDemand {
        self.logical_demand
    }

    /// Returns required successful outputs.
    #[must_use]
    pub const fn successful_output_states(&self) -> u128 {
        self.successful_output_states
    }

    /// Returns optimistic attempt count.
    #[must_use]
    pub const fn optimistic_attempts(&self) -> u128 {
        self.optimistic_attempts
    }

    /// Returns expected attempt count.
    #[must_use]
    pub const fn expected_attempts(&self) -> u128 {
        self.expected_attempts
    }

    /// Returns expected input-state consumption.
    #[must_use]
    pub const fn expected_input_states(&self) -> u128 {
        self.expected_input_states
    }

    /// Returns configured factory parallelism.
    #[must_use]
    pub const fn factory_parallelism(&self) -> u128 {
        self.factory_parallelism
    }

    /// Returns minimum parallel factory count required by the supplied
    /// instantaneous demand-rate model.
    #[must_use]
    pub const fn minimum_parallel_factories(&self) -> u128 {
        self.minimum_parallel_factories
    }

    /// Returns optimistic production time in the factory's abstract time unit.
    #[must_use]
    pub const fn optimistic_production_time(&self) -> u128 {
        self.optimistic_production_time
    }

    /// Returns expected production time.
    #[must_use]
    pub const fn expected_production_time(&self) -> u128 {
        self.expected_production_time
    }

    /// Returns the production-time unit.
    #[must_use]
    pub const fn production_time_unit(&self) -> u128 {
        self.production_time_unit
    }

    /// Returns the deterministic peak output rate.
    #[must_use]
    pub const fn peak_output_rate(&self) -> Rate {
        self.peak_output_rate
    }

    /// Returns the expected output rate.
    #[must_use]
    pub const fn expected_output_rate(&self) -> ProbabilityRate {
        self.expected_output_rate
    }

    /// Returns buffer capacity.
    #[must_use]
    pub const fn buffer_capacity(&self) -> u128 {
        self.buffer_capacity
    }

    /// Returns whether the configured factory parallelism is sufficient for
    /// the calculated minimum.
    #[must_use]
    pub fn configured_parallelism_is_sufficient(&self) -> bool {
        self.factory_parallelism >= self.minimum_parallel_factories
    }
}

// =============================================================================
// Demand-rate model
// =============================================================================

/// Represents a logical magic-state demand rate.
///
/// This type is deliberately independent of scheduling.
///
/// It expresses a requirement, not an execution schedule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MagicStateDemandRate {
    numerator: u128,
    denominator: u128,
}

impl MagicStateDemandRate {
    /// Creates a demand rate.
    pub const fn new(
        numerator: u128,
        denominator: u128,
    ) -> MagicStateResult<Self> {
        if denominator == 0 {
            return Err(MagicStateError::InvalidConfiguration {
                message: "demand-rate denominator must be non-zero",
            });
        }

        Ok(Self {
            numerator,
            denominator,
        })
    }

    /// Zero demand rate.
    pub const fn zero() -> Self {
        Self {
            numerator: 0,
            denominator: 1,
        }
    }

    /// Returns numerator.
    #[must_use]
    pub const fn numerator(self) -> u128 {
        self.numerator
    }

    /// Returns denominator.
    #[must_use]
    pub const fn denominator(self) -> u128 {
        self.denominator
    }

    /// Returns whether demand is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.numerator == 0
    }
}

// =============================================================================
// Factory burst analysis
// =============================================================================

/// Result of comparing a finite magic-state demand burst against a factory
/// configuration.
///
/// This is deliberately a structural estimate, not a schedule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MagicStateBurstAnalysis {
    demand: u128,
    initial_buffer: u128,
    produced_before_demand_exhaustion: u128,
    remaining_demand: u128,
    required_attempts: u128,
    optimistic_time: u128,
}

impl MagicStateBurstAnalysis {
    /// Analyze a finite demand burst.
    pub fn analyze(
        demand: u128,
        initial_buffer: u128,
        factory: &MagicStateFactory,
    ) -> MagicStateResult<Self> {
        let available_from_buffer = initial_buffer.min(demand);

        let remaining_demand = demand - available_from_buffer;

        if remaining_demand == 0 {
            return Ok(Self {
                demand,
                initial_buffer,
                produced_before_demand_exhaustion: 0,
                remaining_demand: 0,
                required_attempts: 0,
                optimistic_time: 0,
            });
        }

        let required_attempts = ceil_div(
            remaining_demand,
            factory.output_states(),
        );

        let produced = required_attempts
            .checked_mul(factory.output_states())
            .ok_or(MagicStateError::ArithmeticOverflow {
                operation: "burst produced-state calculation",
            })?;

        let optimistic_time = required_attempts
            .checked_mul(factory.production_time())
            .ok_or(MagicStateError::ArithmeticOverflow {
                operation: "burst optimistic production time",
            })?;

        Ok(Self {
            demand,
            initial_buffer,
            produced_before_demand_exhaustion: produced,
            remaining_demand,
            required_attempts,
            optimistic_time,
        })
    }

    /// Returns original burst demand.
    #[must_use]
    pub const fn demand(self) -> u128 {
        self.demand
    }

    /// Returns initial buffer.
    #[must_use]
    pub const fn initial_buffer(self) -> u128 {
        self.initial_buffer
    }

    /// Returns optimistic produced states.
    #[must_use]
    pub const fn produced_before_demand_exhaustion(self) -> u128 {
        self.produced_before_demand_exhaustion
    }

    /// Returns demand not covered by the initial buffer.
    #[must_use]
    pub const fn remaining_demand(self) -> u128 {
        self.remaining_demand
    }

    /// Returns optimistic attempts.
    #[must_use]
    pub const fn required_attempts(self) -> u128 {
        self.required_attempts
    }

    /// Returns optimistic production time.
    #[must_use]
    pub const fn optimistic_time(self) -> u128 {
        self.optimistic_time
    }

    /// Returns whether the initial buffer alone covers the burst.
    #[must_use]
    pub const fn buffer_satisfies_demand(self) -> bool {
        self.remaining_demand == 0
    }
}

// =============================================================================
// Resource conversion helpers
// =============================================================================

/// Converts T-count and T-dagger count into standard T-state demand.
///
/// This function is deliberately independent of `t_count.rs` concrete result
/// types so this module does not need to be rewritten if T-count's result
/// structure evolves.
pub const fn demand_from_t_family_counts(
    t_count: u128,
    tdg_count: u128,
) -> MagicStateResult<MagicStateDemand> {
    MagicStateDemand::from_t_and_tdg(t_count, tdg_count)
}

/// Converts a T-family count into standard T-state demand.
#[must_use]
pub const fn demand_from_t_family_count(
    t_family_count: u128,
) -> MagicStateDemand {
    MagicStateDemand::from_t_count(t_family_count)
}

// =============================================================================
// Distillation chain accounting
// =============================================================================

/// Calculates the expected number of input states required by a sequence of
/// distillation stages.
///
/// The calculation proceeds from final demand backwards.
///
/// For each stage:
///
/// ```text
/// required_input =
///     ceil(required_output / expected_output_per_attempt)
///     × input_states
/// ```
///
/// This is an analytical expectation/lower-bound model. It does not claim to
/// model finite-shot tail probabilities, queueing, correlated failures, or
/// adaptive factory scheduling.
pub fn estimate_distillation_inputs(
    output_demand: u128,
    stages: &[DistillationStage],
) -> MagicStateResult<u128> {
    if output_demand == 0 {
        return Ok(0);
    }

    let mut required_outputs = output_demand;

    for stage in stages.iter().rev() {
        let expected_output_numerator = stage
            .output_states()
            .checked_mul(stage.success_probability().numerator())
            .ok_or(MagicStateError::ArithmeticOverflow {
                operation: "distillation expected output numerator",
            })?;

        let expected_output_denominator =
            stage.success_probability().denominator();

        if expected_output_numerator == 0 {
            return Err(MagicStateError::Infeasible {
                reason: "distillation stage has zero expected successful output",
            });
        }

        let attempts_numerator = required_outputs
            .checked_mul(expected_output_denominator)
            .ok_or(MagicStateError::ArithmeticOverflow {
                operation: "distillation attempt numerator",
            })?;

        let attempts = ceil_div(
            attempts_numerator,
            expected_output_numerator,
        );

        required_outputs = attempts
            .checked_mul(stage.input_states())
            .ok_or(MagicStateError::ArithmeticOverflow {
                operation: "distillation input-state expansion",
            })?;
    }

    Ok(required_outputs)
}

// =============================================================================
// Aggregate factory resource accounting
// =============================================================================

/// Aggregates the expected input-state consumption of several identical
/// factory stages.
///
/// This helper is useful when a compiler has already selected the number of
/// parallel factory instances.
pub fn aggregate_factory_attempts(
    required_outputs: u128,
    factory: &MagicStateFactory,
    factory_instances: u128,
) -> MagicStateResult<u128> {
    if required_outputs == 0 {
        return Ok(0);
    }

    if factory_instances == 0 {
        return Err(MagicStateError::ZeroResource {
            resource: "factory_instances",
        });
    }

    let attempts_per_instance = ceil_div(
        required_outputs,
        factory
            .output_states()
            .checked_mul(factory_instances)
            .ok_or(MagicStateError::FactoryCountOverflow)?,
    );

    attempts_per_instance
        .checked_mul(factory_instances)
        .ok_or(MagicStateError::ArithmeticOverflow {
            operation: "aggregate factory attempts",
        })
}

// =============================================================================
// Utility functions
// =============================================================================

/// Greatest common divisor using the Euclidean algorithm.
///
/// Iterative implementation avoids recursion for very large integers.
const fn gcd(
    mut a: u128,
    mut b: u128,
) -> u128 {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }

    a
}

/// Ceiling division.
///
/// The caller must provide a non-zero denominator.
const fn ceil_div(
    numerator: u128,
    denominator: u128,
) -> u128 {
    if numerator == 0 {
        return 0;
    }

    ((numerator - 1) / denominator) + 1
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn t_factory() -> MagicStateFactory {
        MagicStateFactory::new(
            "t_factory",
            MagicStateKind::T,
            MagicStateKind::T,
            15,
            1,
            Probability::ONE,
            10,
            1,
        )
        .expect("test factory must be valid")
    }

    #[test]
    fn probability_validates_bounds() {
        assert!(Probability::new(1, 0).is_err());
        assert!(Probability::new(2, 1).is_err());
        assert!(Probability::new(1, 2).is_ok());
        assert!(Probability::new(0, 1).is_ok());
        assert!(Probability::new(1, 1).is_ok());
    }

    #[test]
    fn probability_normalizes() {
        let probability =
            Probability::new(2, 4).expect("valid probability");

        assert_eq!(
            probability.normalized(),
            Probability::new(1, 2).expect("valid probability")
        );
    }

    #[test]
    fn probability_complement_is_exact() {
        let probability =
            Probability::new(1, 4).expect("valid probability");

        assert_eq!(
            probability.complement().expect("valid complement"),
            Probability::new(3, 4).expect("valid probability")
        );
    }

    #[test]
    fn probability_multiplication_is_exact() {
        let a =
            Probability::new(1, 2).expect("valid probability");

        let b =
            Probability::new(2, 3).expect("valid probability");

        assert_eq!(
            a.multiply(b).expect("valid multiplication").normalized(),
            Probability::new(1, 3).expect("valid probability")
        );
    }

    #[test]
    fn t_count_maps_one_to_one() {
        let demand = MagicStateDemand::from_t_count(100);

        assert_eq!(demand.t_states(), 100);
        assert_eq!(demand.h_states(), 0);
        assert_eq!(
            demand.total_states().expect("no overflow"),
            100
        );
    }

    #[test]
    fn t_and_tdg_are_both_magic_state_demands() {
        let demand =
            MagicStateDemand::from_t_and_tdg(17, 23)
                .expect("no overflow");

        assert_eq!(demand.t_states(), 40);
    }

    #[test]
    fn factory_validates_zero_output() {
        assert!(
            MagicStateFactory::new(
                "bad",
                MagicStateKind::T,
                MagicStateKind::T,
                15,
                0,
                Probability::ONE,
                10,
                1,
            )
            .is_err()
        );
    }

    #[test]
    fn factory_expected_output_is_exact() {
        let factory =
            MagicStateFactory::new(
                "probabilistic",
                MagicStateKind::T,
                MagicStateKind::T,
                15,
                1,
                Probability::new(1, 2)
                    .expect("valid probability"),
                10,
                1,
            )
            .expect("valid factory");

        let output = factory
            .expected_output_per_attempt()
            .expect("valid expected output");

        assert_eq!(output.numerator(), 1);
        assert_eq!(output.denominator(), 2);
    }

    #[test]
    fn factory_peak_rate_is_exact() {
        let factory = t_factory();

        let rate =
            factory.peak_output_rate().expect("valid rate");

        assert_eq!(rate.numerator(), 1);
        assert_eq!(rate.denominator(), 10);
    }

    #[test]
    fn factory_set_rejects_duplicate_ids() {
        let factory = t_factory();

        let mut set = MagicStateFactorySet::new();

        set.push(factory.clone())
            .expect("first factory insertion succeeds");

        assert!(
            set.push(factory).is_err()
        );
    }

    #[test]
    fn resource_model_estimates_deterministic_factory() {
        let factory = t_factory();

        let mut factories = MagicStateFactorySet::new();

        factories
            .push(factory)
            .expect("factory insertion succeeds");

        let model = MagicStateResourceModel::new(
            MagicStateDemand::from_t_count(100),
            factories,
        )
        .expect("valid model");

        let estimate =
            model.estimate("t_factory")
                .expect("valid estimate");

        assert_eq!(
            estimate.successful_output_states(),
            100
        );

        assert_eq!(
            estimate.optimistic_attempts(),
            100
        );

        assert_eq!(
            estimate.expected_attempts(),
            100
        );

        assert_eq!(
            estimate.expected_input_states(),
            1500
        );

        assert_eq!(
            estimate.optimistic_production_time(),
            1000
        );
    }

    #[test]
    fn probabilistic_factory_requires_more_expected_attempts() {
        let factory =
            MagicStateFactory::new(
                "probabilistic",
                MagicStateKind::T,
                MagicStateKind::T,
                15,
                1,
                Probability::new(1, 2)
                    .expect("valid probability"),
                10,
                1,
            )
            .expect("valid factory");

        let mut factories = MagicStateFactorySet::new();

        factories
            .push(factory)
            .expect("factory insertion succeeds");

        let model = MagicStateResourceModel::new(
            MagicStateDemand::from_t_count(100),
            factories,
        )
        .expect("valid model");

        let estimate =
            model.estimate("probabilistic")
                .expect("valid estimate");

        assert_eq!(
            estimate.optimistic_attempts(),
            100
        );

        assert_eq!(
            estimate.expected_attempts(),
            200
        );

        assert_eq!(
            estimate.expected_input_states(),
            3000
        );
    }

    #[test]
    fn rate_factory_parallelism_is_calculated() {
        let factory =
            MagicStateFactory::new(
                "rate",
                MagicStateKind::T,
                MagicStateKind::T,
                15,
                1,
                Probability::ONE,
                10,
                1,
            )
            .expect("valid factory");

        let mut factories = MagicStateFactorySet::new();

        factories
            .push(factory)
            .expect("factory insertion succeeds");

        let model = MagicStateResourceModel::new(
            MagicStateDemand::from_t_count(1),
            factories,
        )
        .expect("valid model");

        let required = model
            .required_parallel_factories_for_rate(
                "rate",
                1,
                1,
            )
            .expect("valid rate");

        assert_eq!(required, 10);
    }

    #[test]
    fn burst_buffer_can_cover_demand() {
        let factory = t_factory();

        let result =
            MagicStateBurstAnalysis::analyze(
                100,
                100,
                &factory,
            )
            .expect("valid burst analysis");

        assert!(result.buffer_satisfies_demand());
        assert_eq!(result.required_attempts(), 0);
    }

    #[test]
    fn burst_requires_factory_when_buffer_is_insufficient() {
        let factory = t_factory();

        let result =
            MagicStateBurstAnalysis::analyze(
                100,
                20,
                &factory,
            )
            .expect("valid burst analysis");

        assert!(!result.buffer_satisfies_demand());
        assert_eq!(result.remaining_demand(), 80);
        assert_eq!(result.required_attempts(), 80);
    }

    #[test]
    fn distillation_chain_expands_demand() {
        let stage =
            DistillationStage::new(
                1,
                MagicStateKind::T,
                MagicStateKind::T,
                15,
                1,
                Probability::ONE,
            )
            .expect("valid stage");

        let required =
            estimate_distillation_inputs(
                100,
                &[stage],
            )
            .expect("valid estimate");

        assert_eq!(required, 1500);
    }

    #[test]
    fn probabilistic_distillation_is_conservative() {
        let stage =
            DistillationStage::new(
                1,
                MagicStateKind::T,
                MagicStateKind::T,
                15,
                1,
                Probability::new(1, 2)
                    .expect("valid probability"),
            )
            .expect("valid stage");

        let required =
            estimate_distillation_inputs(
                100,
                &[stage],
            )
            .expect("valid estimate");

        assert_eq!(required, 3000);
    }

    #[test]
    fn multiple_distillation_levels_are_applied_backwards() {
        let first =
            DistillationStage::new(
                1,
                MagicStateKind::T,
                MagicStateKind::T,
                15,
                1,
                Probability::ONE,
            )
            .expect("valid stage");

        let second =
            DistillationStage::new(
                2,
                MagicStateKind::T,
                MagicStateKind::T,
                15,
                1,
                Probability::ONE,
            )
            .expect("valid stage");

        let required =
            estimate_distillation_inputs(
                1,
                &[first, second],
            )
            .expect("valid estimate");

        assert_eq!(required, 225);
    }

    #[test]
    fn zero_demand_requires_zero_inputs() {
        let stage =
            DistillationStage::new(
                1,
                MagicStateKind::T,
                MagicStateKind::T,
                15,
                1,
                Probability::ONE,
            )
            .expect("valid stage");

        assert_eq!(
            estimate_distillation_inputs(
                0,
                &[stage],
            )
            .expect("valid estimate"),
            0
        );
    }

    #[test]
    fn custom_state_kinds_are_supported() {
        let factory =
            MagicStateFactory::new(
                "custom",
                MagicStateKind::Custom("input".into()),
                MagicStateKind::Custom("output".into()),
                3,
                1,
                Probability::ONE,
                10,
                1,
            )
            .expect("valid custom factory");

        assert_eq!(
            factory.output_kind().identifier(),
            "output"
        );
    }

    #[test]
    fn ceiling_division_is_correct() {
        assert_eq!(ceil_div(0, 5), 0);
        assert_eq!(ceil_div(1, 5), 1);
        assert_eq!(ceil_div(5, 5), 1);
        assert_eq!(ceil_div(6, 5), 2);
        assert_eq!(ceil_div(10, 5), 2);
        assert_eq!(ceil_div(11, 5), 3);
    }
}