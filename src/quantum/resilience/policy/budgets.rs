//! Zamani Quantum Resilience — Policy Budgets
//!
//! Path:
//!     src/quantum/resilience/policy/budgets.rs
//!
//! Purpose:
//!     Defines the provider-independent budget model used by the resilience
//!     policy layer to constrain retries, execution time, shots, memory,
//!     compilation effort, recovery effort, mitigation overhead, resource
//!     consumption, energy, network usage, and other explicitly declared
//!     execution costs.
//!
//! Architectural position:
//!
//!     DETECT
//!        |
//!        v
//!     DIAGNOSE
//!        |
//!        v
//!     POLICY
//!        |
//!        +----> budgets.rs
//!        |
//!        v
//!     PLAN
//!        |
//!        v
//!     ADAPT / RECOVER / MITIGATE
//!
//! This module defines budget semantics and deterministic accounting.
//! It does NOT execute operations.
//!
//! -----------------------------------------------------------------------------
//! OWNERSHIP BOUNDARIES
//! -----------------------------------------------------------------------------
//!
//! This module MUST NOT own:
//!
//! - quantum program semantics;
//! - quantum gates;
//! - quantum operations;
//! - quantum circuits;
//! - logical/physical qubit identity;
//! - hardware discovery;
//! - hardware capabilities;
//! - hardware execution;
//! - routing;
//! - scheduling;
//! - optimization;
//! - compilation;
//! - QEC implementation;
//! - noise models;
//! - fault ontology;
//! - mitigation implementations;
//! - recovery implementations;
//! - telemetry collection;
//! - credentials;
//! - provider SDKs;
//! - network I/O;
//! - filesystem I/O;
//! - background threads;
//! - global mutable state.
//!
//! Canonical quantum identity remains:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! This file does not need either identity because budgets describe quantities
//! and accounting dimensions, not individual quantum resources.
//!
//! -----------------------------------------------------------------------------
//! BUDGET PRINCIPLE
//! -----------------------------------------------------------------------------
//!
//! A budget is an explicit bound on a measurable quantity.
//!
//! Examples:
//!
//! - retry attempts;
//! - elapsed execution time;
//! - shots;
//! - compilation effort;
//! - memory;
//! - QPU time;
//! - energy;
//! - network traffic;
//! - recovery operations;
//! - mitigation overhead;
//! - logical/physical resource usage;
//! - provider cost when such a cost is explicitly supplied by the caller or
//!   target environment.
//!
//! No architectural budget is hard-coded here.
//!
//! There is deliberately no:
//!
//!     MAX_RETRIES
//!     MAX_SHOTS
//!     MAX_QUBITS
//!     MAX_MEMORY
//!     MAX_RECOVERY_ATTEMPTS
//!     DEFAULT_TIMEOUT
//!
//! and no provider-specific budget.
//!
//! "Infinite" means that a dimension may be explicitly unbounded. It does not
//! imply physically infinite execution. Actual execution remains constrained by
//! available resources, caller policy, target capabilities, operating-system
//! limits, and external execution contracts.
//!
//! -----------------------------------------------------------------------------
//! REQUEST/POLICY SEPARATION
//! -----------------------------------------------------------------------------
//!
//! `api::request` owns request-facing DTOs such as `RetryBudget`.
//!
//! This module owns:
//!
//! - the generic policy budget dimension vocabulary;
//! - budget limits;
//! - consumption accounting;
//! - remaining-budget calculations;
//! - deterministic exhaustion checks;
//! - budget snapshots;
//! - budget ledgers;
//! - composition of multiple budget dimensions.
//!
//! The policy layer may translate request-local budget values into this model.
//! This file must not duplicate request DTOs.
//!
//! -----------------------------------------------------------------------------
//! HARD CONSTRAINT RULE
//! -----------------------------------------------------------------------------
//!
//! A budget is a hard resource constraint.
//!
//! It MUST NOT be treated as merely a preference.
//!
//! In particular:
//!
//!     budget exhausted
//!         !=
//!     "try anyway because availability is important"
//!
//! A planner may choose among actions that fit within the budget, but it may
//! never authorize an action solely because that action appears desirable.
//! Semantic, safety, capability, security, and verification gates remain
//! authoritative.
//!
//! -----------------------------------------------------------------------------
//! MONOTONICITY
//! -----------------------------------------------------------------------------
//!
//! Consumption is monotonic:
//!
//!     consumed_after >= consumed_before
//!
//! A budget cannot become larger because an execution failed.
//!
//! Rollback of an execution state does NOT implicitly refund a consumed budget.
//! A higher-level policy may explicitly construct a new budget scope, but this
//! accounting layer does not invent refunds.
//!
//! -----------------------------------------------------------------------------
//! DETERMINISM
//! -----------------------------------------------------------------------------
//!
//! Accounting is deterministic with respect to explicit inputs.
//!
//! This module does not:
//!
//! - read the clock;
//! - read environment variables;
//! - inspect global state;
//! - perform I/O;
//! - use randomness;
//! - spawn threads;
//! - depend on hash-map iteration order.
//!
//! Ordered maps are used where collections are required.
//!
//! -----------------------------------------------------------------------------
//! NUMERIC SAFETY
//! -----------------------------------------------------------------------------
//!
//! Budget quantities are represented with checked arithmetic.
//!
//! Arithmetic overflow is never silently wrapped.
//!
//! Floating-point values are deliberately avoided for budget quantities where
//! exact accounting is possible. Integer units are used instead.
//!
//! Time is represented by `Duration` and checked arithmetic.
//!
//! -----------------------------------------------------------------------------
//! RUST CONTRACT
//! -----------------------------------------------------------------------------
//!
//! - Rust 1.97 / 1.97.1
//! - Rust 2021
//! - stable Rust only
//! - `unsafe` forbidden
//! - no unsafe operations
//! - no hard-coded machine-size limits
//! - no provider-specific behavior
//! - no hidden I/O
//! - no hidden concurrency
//! - no hidden retries
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::fmt;
use std::time::Duration;

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier for policy budgets.
pub const RESILIENCE_BUDGET_SCHEMA_ID: &str =
    "zamani.quantum.resilience.policy.budgets";

/// Semantic schema version.
///
/// This version is independent of the Rust package version and the resilience
/// request schema version.
pub const RESILIENCE_BUDGET_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Budget errors
// =============================================================================

/// Errors produced by budget construction and accounting.
///
/// The policy layer may translate these into the repository's canonical
/// resilience error type when exposing them through a public orchestration API.
///
/// This module intentionally keeps the budget contract independently usable;
/// it does not depend on planner, recovery, or execution implementations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BudgetError {
    /// A supplied integer quantity is invalid for the requested operation.
    InvalidQuantity {
        /// Stable budget dimension name.
        dimension: BudgetDimension,
        /// Stable description of the invalid condition.
        reason: &'static str,
    },

    /// A supplied duration is invalid for the requested operation.
    InvalidDuration {
        /// Stable budget dimension name.
        dimension: BudgetDimension,
        /// Stable description of the invalid condition.
        reason: &'static str,
    },

    /// Arithmetic would overflow the representable accounting domain.
    ArithmeticOverflow {
        /// Stable budget dimension name.
        dimension: BudgetDimension,
    },

    /// A budget would be exceeded by a proposed consumption.
    Exhausted {
        /// Dimension that would be exhausted.
        dimension: BudgetDimension,
        /// Amount already consumed.
        consumed: u128,
        /// Maximum permitted amount.
        limit: u128,
        /// Requested additional amount.
        requested: u128,
    },

    /// A duration budget would be exceeded.
    TimeExhausted {
        /// Amount already consumed.
        consumed: Duration,
        /// Maximum permitted duration.
        limit: Duration,
        /// Requested additional duration.
        requested: Duration,
    },

    /// A dimension was supplied more than once while constructing a set.
    DuplicateDimension {
        /// Duplicated dimension.
        dimension: BudgetDimension,
    },

    /// The requested budget set is internally inconsistent.
    InvalidConfiguration {
        /// Human-readable static explanation.
        reason: &'static str,
    },
}

impl fmt::Display for BudgetError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidQuantity { dimension, reason } => {
                write!(
                    formatter,
                    "invalid quantity for budget dimension {}: {}",
                    dimension, reason
                )
            }
            Self::InvalidDuration { dimension, reason } => {
                write!(
                    formatter,
                    "invalid duration for budget dimension {}: {}",
                    dimension, reason
                )
            }
            Self::ArithmeticOverflow { dimension } => {
                write!(
                    formatter,
                    "budget arithmetic overflow for dimension {}",
                    dimension
                )
            }
            Self::Exhausted {
                dimension,
                consumed,
                limit,
                requested,
            } => {
                write!(
                    formatter,
                    "budget {} exhausted: consumed={}, limit={}, requested={}",
                    dimension, consumed, limit, requested
                )
            }
            Self::TimeExhausted {
                consumed,
                limit,
                requested,
            } => {
                write!(
                    formatter,
                    "time budget exhausted: consumed={:?}, limit={:?}, requested={:?}",
                    consumed, limit, requested
                )
            }
            Self::DuplicateDimension { dimension } => {
                write!(
                    formatter,
                    "duplicate budget dimension: {}",
                    dimension
                )
            }
            Self::InvalidConfiguration { reason } => {
                write!(formatter, "invalid budget configuration: {}", reason)
            }
        }
    }
}

impl std::error::Error for BudgetError {}

// =============================================================================
// Budget dimension
// =============================================================================

/// Canonical policy budget dimensions.
///
/// The enum contains only dimensions with generally useful resilience
/// semantics. It is intentionally extensible through `Custom`.
///
/// A dimension describes *what is being consumed*, not how it is obtained.
///
/// Provider-specific resources must use `Custom` only at an integration
/// boundary; core resilience code must remain provider-neutral.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BudgetDimension {
    /// Number of additional execution attempts.
    RetryAttempts,

    /// Total elapsed execution time attributable to the resilience scope.
    ExecutionTime,

    /// Quantum processor time when explicitly exposed by the target.
    QpuTime,

    /// Number of measurement shots.
    Shots,

    /// Compilation effort units.
    CompilationEffort,

    /// Memory consumption units.
    Memory,

    /// Energy consumption units.
    Energy,

    /// Network transfer units.
    Network,

    /// Total resource usage units.
    ResourceUsage,

    /// Physical resource usage units.
    PhysicalResourceUsage,

    /// Logical resource usage units.
    LogicalResourceUsage,

    /// Mitigation overhead units.
    Mitigation,

    /// Recovery-operation units.
    Recovery,

    /// Adaptation-operation units.
    Adaptation,

    /// Number of generated/consumed execution artifacts.
    Artifact,

    /// Explicit externally supplied financial/provider cost units.
    ///
    /// The unit is intentionally unspecified. The caller/environment must
    /// define its interpretation.
    FinancialCost,

    /// Extension point for future resource dimensions.
    Custom(String),
}

impl BudgetDimension {
    /// Stable machine-readable identifier.
    pub fn as_str(&self) -> &str {
        match self {
            Self::RetryAttempts => "retry_attempts",
            Self::ExecutionTime => "execution_time",
            Self::QpuTime => "qpu_time",
            Self::Shots => "shots",
            Self::CompilationEffort => "compilation_effort",
            Self::Memory => "memory",
            Self::Energy => "energy",
            Self::Network => "network",
            Self::ResourceUsage => "resource_usage",
            Self::PhysicalResourceUsage => "physical_resource_usage",
            Self::LogicalResourceUsage => "logical_resource_usage",
            Self::Mitigation => "mitigation",
            Self::Recovery => "recovery",
            Self::Adaptation => "adaptation",
            Self::Artifact => "artifact",
            Self::FinancialCost => "financial_cost",
            Self::Custom(name) => name.as_str(),
        }
    }

    /// Returns whether this dimension is duration-valued.
    pub const fn is_time(self: &Self) -> bool {
        matches!(self, Self::ExecutionTime | Self::QpuTime)
    }

    /// Returns whether this is a caller-defined extension dimension.
    pub const fn is_custom(self: &Self) -> bool {
        matches!(self, Self::Custom(_))
    }
}

impl fmt::Display for BudgetDimension {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Budget limit
// =============================================================================

/// A budget limit.
///
/// `Unlimited` means that this policy scope places no limit on the dimension.
/// It does not override physical, hardware, runtime, security, or other
/// independently enforced limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BudgetLimit {
    /// No request-local limit is imposed for this dimension.
    Unlimited,

    /// Finite quantity limit.
    Quantity(u128),
}

impl BudgetLimit {
    /// Creates an unlimited budget.
    pub const fn unlimited() -> Self {
        Self::Unlimited
    }

    /// Creates a finite quantity budget.
    pub const fn quantity(value: u128) -> Self {
        Self::Quantity(value)
    }

    /// Returns the finite quantity when present.
    pub const fn as_quantity(self) -> Option<u128> {
        match self {
            Self::Unlimited => None,
            Self::Quantity(value) => Some(value),
        }
    }

    /// Returns whether this budget is unlimited.
    pub const fn is_unlimited(self) -> bool {
        matches!(self, Self::Unlimited)
    }
}

// =============================================================================
// Duration budget limit
// =============================================================================

/// A duration-specific budget limit.
///
/// Kept separate from `BudgetLimit` so that time accounting cannot accidentally
/// mix nanoseconds with arbitrary quantity units.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeBudgetLimit {
    /// No request-local duration limit.
    Unlimited,

    /// Finite duration limit.
    Duration(Duration),
}

impl TimeBudgetLimit {
    /// Creates an unlimited time budget.
    pub const fn unlimited() -> Self {
        Self::Unlimited
    }

    /// Creates a finite duration budget.
    pub const fn duration(value: Duration) -> Self {
        Self::Duration(value)
    }

    /// Returns the finite duration when present.
    pub const fn as_duration(self) -> Option<Duration> {
        match self {
            Self::Unlimited => None,
            Self::Duration(value) => Some(value),
        }
    }

    /// Returns whether this budget is unlimited.
    pub const fn is_unlimited(self) -> bool {
        matches!(self, Self::Unlimited)
    }
}

// =============================================================================
// Budget value
// =============================================================================

/// A non-time budget consumption value.
///
/// `u128` is used to avoid artificial 32-bit/64-bit machine-size assumptions
/// in policy accounting while remaining compatible with stable Rust.
///
/// The value has no inherent physical unit; its unit is defined by the
/// associated `BudgetDimension`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BudgetValue(u128);

impl BudgetValue {
    /// Zero consumption.
    pub const ZERO: Self = Self(0);

    /// Creates a value.
    pub const fn new(value: u128) -> Self {
        Self(value)
    }

    /// Returns the raw quantity.
    pub const fn get(self) -> u128 {
        self.0
    }

    /// Checked addition.
    pub const fn checked_add(self, other: Self) -> Option<Self> {
        match self.0.checked_add(other.0) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Checked subtraction.
    pub const fn checked_sub(self, other: Self) -> Option<Self> {
        match self.0.checked_sub(other.0) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl From<u64> for BudgetValue {
    fn from(value: u64) -> Self {
        Self(u128::from(value))
    }
}

impl From<u32> for BudgetValue {
    fn from(value: u32) -> Self {
        Self(u128::from(value))
    }
}

impl From<usize> for BudgetValue {
    fn from(value: usize) -> Self {
        Self(value as u128)
    }
}

impl fmt::Display for BudgetValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

// =============================================================================
// Budget consumption
// =============================================================================

/// A single consumption record.
///
/// A consumption record is immutable. The ledger stores cumulative state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BudgetConsumption {
    /// Amount consumed.
    value: BudgetValue,
}

impl BudgetConsumption {
    /// Creates a consumption record.
    pub const fn new(value: BudgetValue) -> Self {
        Self { value }
    }

    /// Returns the consumed quantity.
    pub const fn value(self) -> BudgetValue {
        self.value
    }
}

// =============================================================================
// Time consumption
// =============================================================================

/// Time consumption record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimeConsumption {
    /// Duration consumed.
    duration: Duration,
}

impl TimeConsumption {
    /// Creates a time consumption record.
    pub const fn new(duration: Duration) -> Self {
        Self { duration }
    }

    /// Returns consumed duration.
    pub const fn duration(self) -> Duration {
        self.duration
    }
}

// =============================================================================
// Budget specification
// =============================================================================

/// One policy budget specification.
///
/// This is immutable and safe to share between planning stages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BudgetSpec {
    /// Budget dimension.
    dimension: BudgetDimension,

    /// Quantity limit for non-time dimensions.
    limit: BudgetLimit,

    /// Time limit for time dimensions.
    time_limit: TimeBudgetLimit,

    /// Whether this budget is enabled.
    enabled: bool,
}

impl BudgetSpec {
    /// Creates an unlimited non-time budget.
    pub fn unlimited(dimension: BudgetDimension) -> Result<Self, BudgetError> {
        if dimension.is_time() {
            return Err(BudgetError::InvalidConfiguration {
                reason: "time dimensions require a TimeBudgetLimit",
            });
        }

        Ok(Self {
            dimension,
            limit: BudgetLimit::Unlimited,
            time_limit: TimeBudgetLimit::Unlimited,
            enabled: true,
        })
    }

    /// Creates a finite non-time budget.
    pub fn quantity(
        dimension: BudgetDimension,
        limit: u128,
    ) -> Result<Self, BudgetError> {
        if dimension.is_time() {
            return Err(BudgetError::InvalidConfiguration {
                reason: "time dimensions require a TimeBudgetLimit",
            });
        }

        Ok(Self {
            dimension,
            limit: BudgetLimit::Quantity(limit),
            time_limit: TimeBudgetLimit::Unlimited,
            enabled: true,
        })
    }

    /// Creates an unlimited time budget.
    pub fn unlimited_time(
        dimension: BudgetDimension,
    ) -> Result<Self, BudgetError> {
        if !dimension.is_time() {
            return Err(BudgetError::InvalidConfiguration {
                reason: "unlimited_time requires a time dimension",
            });
        }

        Ok(Self {
            dimension,
            limit: BudgetLimit::Unlimited,
            time_limit: TimeBudgetLimit::Unlimited,
            enabled: true,
        })
    }

    /// Creates a finite time budget.
    pub fn duration(
        dimension: BudgetDimension,
        limit: Duration,
    ) -> Result<Self, BudgetError> {
        if !dimension.is_time() {
            return Err(BudgetError::InvalidConfiguration {
                reason: "duration requires a time dimension",
            });
        }

        Ok(Self {
            dimension,
            limit: BudgetLimit::Unlimited,
            time_limit: TimeBudgetLimit::Duration(limit),
            enabled: true,
        })
    }

    /// Returns the dimension.
    pub fn dimension(&self) -> &BudgetDimension {
        &self.dimension
    }

    /// Returns the quantity limit.
    pub const fn limit(&self) -> BudgetLimit {
        self.limit
    }

    /// Returns the time limit.
    pub const fn time_limit(&self) -> TimeBudgetLimit {
        self.time_limit
    }

    /// Returns whether the budget is enabled.
    pub const fn enabled(&self) -> bool {
        self.enabled
    }

    /// Enables or disables this budget.
    pub const fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Returns whether this specification represents a time budget.
    pub const fn is_time(&self) -> bool {
        self.dimension.is_time()
    }
}

// =============================================================================
// Budget state
// =============================================================================

/// Current consumption state for a budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BudgetState {
    /// No consumption has been recorded.
    Unused,

    /// Some consumption has been recorded and the budget remains available.
    Available {
        /// Quantity consumed.
        consumed: BudgetValue,

        /// Quantity remaining when finite.
        remaining: Option<BudgetValue>,
    },

    /// The budget has been exhausted.
    Exhausted {
        /// Quantity consumed.
        consumed: BudgetValue,

        /// Finite configured limit.
        limit: BudgetValue,
    },

    /// Unlimited budget with recorded consumption.
    Unlimited {
        /// Quantity consumed.
        consumed: BudgetValue,
    },
}

impl BudgetState {
    /// Returns the quantity consumed.
    pub const fn consumed(self) -> BudgetValue {
        match self {
            Self::Unused => BudgetValue::ZERO,
            Self::Available { consumed, .. } => consumed,
            Self::Exhausted { consumed, .. } => consumed,
            Self::Unlimited { consumed } => consumed,
        }
    }

    /// Returns the remaining quantity when a finite budget remains available.
    pub const fn remaining(self) -> Option<BudgetValue> {
        match self {
            Self::Unused => None,
            Self::Available { remaining, .. } => remaining,
            Self::Exhausted { .. } => Some(BudgetValue::ZERO),
            Self::Unlimited { .. } => None,
        }
    }

    /// Returns whether no further consumption is permitted.
    pub const fn is_exhausted(self) -> bool {
        matches!(self, Self::Exhausted { .. })
    }

    /// Returns whether the budget is unlimited.
    pub const fn is_unlimited(self) -> bool {
        matches!(self, Self::Unlimited { .. })
    }
}

// =============================================================================
// Time budget state
// =============================================================================

/// Current state for a time budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeBudgetState {
    /// No time has been consumed.
    Unused,

    /// Time remains.
    Available {
        /// Consumed duration.
        consumed: Duration,

        /// Remaining duration.
        remaining: Option<Duration>,
    },

    /// Time budget exhausted.
    Exhausted {
        /// Consumed duration.
        consumed: Duration,

        /// Configured limit.
        limit: Duration,
    },

    /// Unlimited time budget.
    Unlimited {
        /// Consumed duration.
        consumed: Duration,
    },
}

impl TimeBudgetState {
    /// Returns consumed duration.
    pub const fn consumed(self) -> Duration {
        match self {
            Self::Unused => Duration::ZERO,
            Self::Available { consumed, .. } => consumed,
            Self::Exhausted { consumed, .. } => consumed,
            Self::Unlimited { consumed } => consumed,
        }
    }

    /// Returns remaining duration when finite.
    pub const fn remaining(self) -> Option<Duration> {
        match self {
            Self::Unused => None,
            Self::Available { remaining, .. } => remaining,
            Self::Exhausted { .. } => Some(Duration::ZERO),
            Self::Unlimited { .. } => None,
        }
    }

    /// Returns whether the budget is exhausted.
    pub const fn is_exhausted(self) -> bool {
        matches!(self, Self::Exhausted { .. })
    }

    /// Returns whether the budget is unlimited.
    pub const fn is_unlimited(self) -> bool {
        matches!(self, Self::Unlimited { .. })
    }
}

// =============================================================================
// Budget ledger
// =============================================================================

/// Deterministic, immutable-input budget ledger.
///
/// Internally uses `BTreeMap` so snapshots and iteration are stable and do not
/// depend on hash-map iteration order.
///
/// The ledger is mutable only through explicit methods on a value owned by the
/// caller. There is no global state.
#[derive(Debug, Clone, Default)]
pub struct BudgetLedger {
    specs: BTreeMap<BudgetDimension, BudgetSpec>,
    consumed: BTreeMap<BudgetDimension, BudgetValue>,
    time_consumed: BTreeMap<BudgetDimension, Duration>,
}

impl BudgetLedger {
    /// Creates an empty ledger.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a ledger from a collection of specifications.
    ///
    /// Duplicate dimensions are rejected deterministically.
    pub fn from_specs<I>(specs: I) -> Result<Self, BudgetError>
    where
        I: IntoIterator<Item = BudgetSpec>,
    {
        let mut ledger = Self::new();

        for spec in specs {
            ledger.insert(spec)?;
        }

        Ok(ledger)
    }

    /// Inserts one budget specification.
    pub fn insert(&mut self, spec: BudgetSpec) -> Result<(), BudgetError> {
        let dimension = spec.dimension.clone();

        if self.specs.contains_key(&dimension) {
            return Err(BudgetError::DuplicateDimension { dimension });
        }

        self.specs.insert(dimension, spec);
        Ok(())
    }

    /// Returns a budget specification.
    pub fn spec(&self, dimension: &BudgetDimension) -> Option<&BudgetSpec> {
        self.specs.get(dimension)
    }

    /// Returns the number of configured dimensions.
    pub fn len(&self) -> usize {
        self.specs.len()
    }

    /// Returns whether no dimensions are configured.
    pub fn is_empty(&self) -> bool {
        self.specs.is_empty()
    }

    /// Returns all configured specifications in deterministic order.
    pub fn specs(&self) -> impl Iterator<Item = &BudgetSpec> {
        self.specs.values()
    }

    /// Records non-time consumption.
    ///
    /// The operation is atomic with respect to this dimension: if the new
    /// consumption would exceed the finite limit, the ledger remains unchanged.
    pub fn consume(
        &mut self,
        dimension: &BudgetDimension,
        amount: BudgetValue,
    ) -> Result<(), BudgetError> {
        let spec = self
            .specs
            .get(dimension)
            .ok_or(BudgetError::InvalidConfiguration {
                reason: "budget dimension is not configured",
            })?;

        if !spec.enabled() {
            return Ok(());
        }

        if spec.is_time() {
            return Err(BudgetError::InvalidConfiguration {
                reason: "time budget requires consume_time",
            });
        }

        let previous = self
            .consumed
            .get(dimension)
            .copied()
            .unwrap_or(BudgetValue::ZERO);

        let new_value = previous
            .checked_add(amount)
            .ok_or_else(|| BudgetError::ArithmeticOverflow {
                dimension: dimension.clone(),
            })?;

        if let BudgetLimit::Quantity(limit) = spec.limit() {
            if new_value.get() > limit {
                return Err(BudgetError::Exhausted {
                    dimension: dimension.clone(),
                    consumed: previous.get(),
                    limit,
                    requested: amount.get(),
                });
            }
        }

        self.consumed.insert(dimension.clone(), new_value);
        Ok(())
    }

    /// Records time consumption.
    ///
    /// The operation is atomic: failed accounting does not alter the ledger.
    pub fn consume_time(
        &mut self,
        dimension: &BudgetDimension,
        amount: Duration,
    ) -> Result<(), BudgetError> {
        let spec = self
            .specs
            .get(dimension)
            .ok_or(BudgetError::InvalidConfiguration {
                reason: "budget dimension is not configured",
            })?;

        if !spec.enabled() {
            return Ok(());
        }

        if !spec.is_time() {
            return Err(BudgetError::InvalidConfiguration {
                reason: "non-time budget requires consume",
            });
        }

        let previous = self
            .time_consumed
            .get(dimension)
            .copied()
            .unwrap_or(Duration::ZERO);

        let new_value = previous
            .checked_add(amount)
            .ok_or_else(|| BudgetError::InvalidDuration {
                dimension: dimension.clone(),
                reason: "duration arithmetic overflow",
            })?;

        if let TimeBudgetLimit::Duration(limit) = spec.time_limit() {
            if new_value > limit {
                return Err(BudgetError::TimeExhausted {
                    consumed: previous,
                    limit,
                    requested: amount,
                });
            }
        }

        self.time_consumed.insert(dimension.clone(), new_value);
        Ok(())
    }

    /// Checks whether non-time consumption would fit without changing state.
    pub fn can_consume(
        &self,
        dimension: &BudgetDimension,
        amount: BudgetValue,
    ) -> Result<bool, BudgetError> {
        let spec = self
            .specs
            .get(dimension)
            .ok_or(BudgetError::InvalidConfiguration {
                reason: "budget dimension is not configured",
            })?;

        if !spec.enabled() {
            return Ok(true);
        }

        if spec.is_time() {
            return Err(BudgetError::InvalidConfiguration {
                reason: "time budget requires can_consume_time",
            });
        }

        let previous = self
            .consumed
            .get(dimension)
            .copied()
            .unwrap_or(BudgetValue::ZERO);

        let new_value = previous
            .checked_add(amount)
            .ok_or_else(|| BudgetError::ArithmeticOverflow {
                dimension: dimension.clone(),
            })?;

        Ok(match spec.limit() {
            BudgetLimit::Unlimited => true,
            BudgetLimit::Quantity(limit) => new_value.get() <= limit,
        })
    }

    /// Checks whether time consumption would fit without changing state.
    pub fn can_consume_time(
        &self,
        dimension: &BudgetDimension,
        amount: Duration,
    ) -> Result<bool, BudgetError> {
        let spec = self
            .specs
            .get(dimension)
            .ok_or(BudgetError::InvalidConfiguration {
                reason: "budget dimension is not configured",
            })?;

        if !spec.enabled() {
            return Ok(true);
        }

        if !spec.is_time() {
            return Err(BudgetError::InvalidConfiguration {
                reason: "non-time budget requires can_consume",
            });
        }

        let previous = self
            .time_consumed
            .get(dimension)
            .copied()
            .unwrap_or(Duration::ZERO);

        let new_value = previous
            .checked_add(amount)
            .ok_or_else(|| BudgetError::InvalidDuration {
                dimension: dimension.clone(),
                reason: "duration arithmetic overflow",
            })?;

        Ok(match spec.time_limit() {
            TimeBudgetLimit::Unlimited => true,
            TimeBudgetLimit::Duration(limit) => new_value <= limit,
        })
    }

    /// Returns current state for a non-time dimension.
    pub fn state(&self, dimension: &BudgetDimension) -> Option<BudgetState> {
        let spec = self.specs.get(dimension)?;

        if !spec.enabled() {
            return Some(BudgetState::Unlimited {
                consumed: BudgetValue::ZERO,
            });
        }

        if spec.is_time() {
            return None;
        }

        let consumed = self
            .consumed
            .get(dimension)
            .copied()
            .unwrap_or(BudgetValue::ZERO);

        Some(match spec.limit() {
            BudgetLimit::Unlimited => BudgetState::Unlimited { consumed },

            BudgetLimit::Quantity(limit) if consumed.get() < limit => {
                BudgetState::Available {
                    consumed,
                    remaining: Some(BudgetValue::new(
                        limit.saturating_sub(consumed.get()),
                    )),
                }
            }

            BudgetLimit::Quantity(limit) => BudgetState::Exhausted {
                consumed,
                limit: BudgetValue::new(limit),
            },
        })
    }

    /// Returns current state for a time dimension.
    pub fn time_state(
        &self,
        dimension: &BudgetDimension,
    ) -> Option<TimeBudgetState> {
        let spec = self.specs.get(dimension)?;

        if !spec.enabled() {
            return Some(TimeBudgetState::Unlimited {
                consumed: Duration::ZERO,
            });
        }

        if !spec.is_time() {
            return None;
        }

        let consumed = self
            .time_consumed
            .get(dimension)
            .copied()
            .unwrap_or(Duration::ZERO);

        Some(match spec.time_limit() {
            TimeBudgetLimit::Unlimited => {
                TimeBudgetState::Unlimited { consumed }
            }

            TimeBudgetLimit::Duration(limit) if consumed < limit => {
                TimeBudgetState::Available {
                    consumed,
                    remaining: Some(limit.saturating_sub(consumed)),
                }
            }

            TimeBudgetLimit::Duration(limit) => TimeBudgetState::Exhausted {
                consumed,
                limit,
            },
        })
    }

    /// Returns whether every enabled budget is still available.
    pub fn all_available(&self) -> bool {
        self.specs.keys().all(|dimension| {
            if dimension.is_time() {
                !self
                    .time_state(dimension)
                    .map(TimeBudgetState::is_exhausted)
                    .unwrap_or(false)
            } else {
                !self
                    .state(dimension)
                    .map(BudgetState::is_exhausted)
                    .unwrap_or(false)
            }
        })
    }

    /// Returns all exhausted dimensions in deterministic order.
    pub fn exhausted_dimensions(&self) -> Vec<BudgetDimension> {
        self.specs
            .keys()
            .filter(|dimension| {
                if dimension.is_time() {
                    self.time_state(dimension)
                        .map(TimeBudgetState::is_exhausted)
                        .unwrap_or(false)
                } else {
                    self.state(dimension)
                        .map(BudgetState::is_exhausted)
                        .unwrap_or(false)
                }
            })
            .cloned()
            .collect()
    }

    /// Produces an immutable snapshot.
    pub fn snapshot(&self) -> BudgetSnapshot {
        BudgetSnapshot::from_ledger(self)
    }
}

// =============================================================================
// Budget snapshot
// =============================================================================

/// Immutable, deterministic view of budget state.
///
/// This is suitable for policy evaluation, planning, provenance and telemetry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BudgetSnapshot {
    entries: BTreeMap<BudgetDimension, BudgetSnapshotEntry>,
}

impl BudgetSnapshot {
    fn from_ledger(ledger: &BudgetLedger) -> Self {
        let mut entries = BTreeMap::new();

        for dimension in ledger.specs.keys() {
            let entry = if dimension.is_time() {
                BudgetSnapshotEntry::Time {
                    state: ledger
                        .time_state(dimension)
                        .unwrap_or(TimeBudgetState::Unused),
                }
            } else {
                BudgetSnapshotEntry::Quantity {
                    state: ledger
                        .state(dimension)
                        .unwrap_or(BudgetState::Unused),
                }
            };

            entries.insert(dimension.clone(), entry);
        }

        Self { entries }
    }

    /// Returns the number of dimensions represented.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether the snapshot contains no dimensions.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns one entry.
    pub fn get(&self, dimension: &BudgetDimension) -> Option<&BudgetSnapshotEntry> {
        self.entries.get(dimension)
    }

    /// Returns entries in deterministic order.
    pub fn entries(
        &self,
    ) -> impl Iterator<Item = (&BudgetDimension, &BudgetSnapshotEntry)> {
        self.entries.iter()
    }

    /// Returns all exhausted dimensions.
    pub fn exhausted_dimensions(&self) -> Vec<BudgetDimension> {
        self.entries
            .iter()
            .filter_map(|(dimension, entry)| {
                if entry.is_exhausted() {
                    Some(dimension.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Returns whether every represented budget remains available.
    pub fn all_available(&self) -> bool {
        self.entries.values().all(|entry| !entry.is_exhausted())
    }
}

/// One immutable budget snapshot entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BudgetSnapshotEntry {
    /// Quantity-valued budget.
    Quantity {
        /// Current state.
        state: BudgetState,
    },

    /// Time-valued budget.
    Time {
        /// Current state.
        state: TimeBudgetState,
    },
}

impl BudgetSnapshotEntry {
    /// Returns whether the represented budget is exhausted.
    pub const fn is_exhausted(self) -> bool {
        match self {
            Self::Quantity { state } => state.is_exhausted(),
            Self::Time { state } => state.is_exhausted(),
        }
    }
}

// =============================================================================
// Budget requirement
// =============================================================================

/// A proposed amount of budget consumption.
///
/// Planning uses requirements to perform feasibility checks before executing an
/// action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BudgetRequirement {
    /// Quantity requirement.
    Quantity {
        /// Budget dimension.
        dimension: BudgetDimension,

        /// Required amount.
        amount: BudgetValue,
    },

    /// Time requirement.
    Time {
        /// Budget dimension.
        dimension: BudgetDimension,

        /// Required duration.
        duration: Duration,
    },
}

impl BudgetRequirement {
    /// Creates a quantity requirement.
    pub const fn quantity(
        dimension: BudgetDimension,
        amount: BudgetValue,
    ) -> Self {
        Self::Quantity { dimension, amount }
    }

    /// Creates a time requirement.
    pub const fn time(
        dimension: BudgetDimension,
        duration: Duration,
    ) -> Self {
        Self::Time {
            dimension,
            duration,
        }
    }

    /// Returns the dimension.
    pub fn dimension(&self) -> &BudgetDimension {
        match self {
            Self::Quantity { dimension, .. } => dimension,
            Self::Time { dimension, .. } => dimension,
        }
    }
}

// =============================================================================
// Budget feasibility
// =============================================================================

/// Result of evaluating a collection of budget requirements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BudgetFeasibility {
    /// All requirements fit.
    Feasible,

    /// At least one requirement does not fit.
    Infeasible {
        /// Dimensions that prevent execution.
        exhausted: Vec<BudgetDimension>,
    },
}

impl BudgetFeasibility {
    /// Returns whether all requirements fit.
    pub const fn is_feasible(&self) -> bool {
        matches!(self, Self::Feasible)
    }

    /// Returns the blocking dimensions.
    pub fn exhausted(&self) -> &[BudgetDimension] {
        match self {
            Self::Feasible => &[],
            Self::Infeasible { exhausted } => exhausted,
        }
    }
}

// =============================================================================
// Budget requirement evaluation
// =============================================================================

impl BudgetLedger {
    /// Checks a collection of requirements without mutating the ledger.
    ///
    /// Requirements are evaluated in deterministic order as supplied by the
    /// caller. The returned blocking dimensions are sorted before being
    /// returned.
    pub fn evaluate_requirements<I>(
        &self,
        requirements: I,
    ) -> Result<BudgetFeasibility, BudgetError>
    where
        I: IntoIterator<Item = BudgetRequirement>,
    {
        let mut blocked = Vec::new();

        for requirement in requirements {
            let feasible = match requirement {
                BudgetRequirement::Quantity { dimension, amount } => {
                    self.can_consume(&dimension, amount)?
                }
                BudgetRequirement::Time {
                    dimension,
                    duration,
                } => self.can_consume_time(&dimension, duration)?,
            };

            if !feasible {
                blocked.push(requirement.dimension().clone());
            }
        }

        blocked.sort();
        blocked.dedup();

        if blocked.is_empty() {
            Ok(BudgetFeasibility::Feasible)
        } else {
            Ok(BudgetFeasibility::Infeasible {
                exhausted: blocked,
            })
        }
    }

    /// Atomically records a collection of requirements.
    ///
    /// First checks all requirements. If any requirement cannot be satisfied,
    /// no requirement is consumed.
    pub fn consume_requirements<I>(
        &mut self,
        requirements: I,
    ) -> Result<(), BudgetError>
    where
        I: IntoIterator<Item = BudgetRequirement>,
    {
        let requirements: Vec<BudgetRequirement> =
            requirements.into_iter().collect();

        match self.evaluate_requirements(requirements.iter().copied())? {
            BudgetFeasibility::Feasible => {}
            BudgetFeasibility::Infeasible { exhausted } => {
                if let Some(dimension) = exhausted.first() {
                    if dimension.is_time() {
                        let requirement = requirements.iter().find(|item| {
                            item.dimension() == dimension
                        });

                        if let Some(BudgetRequirement::Time { duration, .. }) =
                            requirement
                        {
                            let consumed = self
                                .time_consumed
                                .get(dimension)
                                .copied()
                                .unwrap_or(Duration::ZERO);

                            let limit = self
                                .spec(dimension)
                                .and_then(BudgetSpec::time_limit)
                                .as_duration()
                                .unwrap_or(Duration::ZERO);

                            return Err(BudgetError::TimeExhausted {
                                consumed,
                                limit,
                                requested: *duration,
                            });
                        }
                    } else if let Some(
                        BudgetRequirement::Quantity { amount, .. },
                    ) = requirements.iter().find(|item| {
                        item.dimension() == dimension
                    }) {
                        let consumed = self
                            .consumed
                            .get(dimension)
                            .copied()
                            .unwrap_or(BudgetValue::ZERO);

                        let limit = self
                            .spec(dimension)
                            .and_then(|spec| spec.limit().as_quantity())
                            .unwrap_or(0);

                        return Err(BudgetError::Exhausted {
                            dimension: dimension.clone(),
                            consumed: consumed.get(),
                            limit,
                            requested: amount.get(),
                        });
                    }
                }

                return Err(BudgetError::InvalidConfiguration {
                    reason: "budget requirement could not be resolved",
                });
            }
        }

        // All checks have succeeded, so all mutations are safe.
        for requirement in requirements {
            match requirement {
                BudgetRequirement::Quantity { dimension, amount } => {
                    self.consume(&dimension, amount)?;
                }
                BudgetRequirement::Time {
                    dimension,
                    duration,
                } => {
                    self.consume_time(&dimension, duration)?;
                }
            }
        }

        Ok(())
    }
}

// =============================================================================
// Budget policy
// =============================================================================

/// Complete policy budget configuration.
///
/// This is the object consumed by the policy evaluator and planner.
///
/// It deliberately contains no execution implementation.
#[derive(Debug, Clone, Default)]
pub struct BudgetPolicy {
    ledger: BudgetLedger,
}

impl BudgetPolicy {
    /// Creates an empty budget policy.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a policy from budget specifications.
    pub fn from_specs<I>(specs: I) -> Result<Self, BudgetError>
    where
        I: IntoIterator<Item = BudgetSpec>,
    {
        Ok(Self {
            ledger: BudgetLedger::from_specs(specs)?,
        })
    }

    /// Adds a budget specification.
    pub fn with_budget(
        mut self,
        spec: BudgetSpec,
    ) -> Result<Self, BudgetError> {
        self.ledger.insert(spec)?;
        Ok(self)
    }

    /// Returns the underlying immutable ledger.
    pub fn ledger(&self) -> &BudgetLedger {
        &self.ledger
    }

    /// Creates a mutable ledger for a concrete execution scope.
    ///
    /// Policy definitions remain immutable; accounting belongs to the
    /// invocation-specific ledger.
    pub fn new_ledger(&self) -> BudgetLedger {
        self.ledger.clone()
    }

    /// Returns a deterministic policy snapshot.
    pub fn snapshot(&self) -> BudgetSnapshot {
        self.ledger.snapshot()
    }

    /// Checks whether all configured budgets remain available.
    pub fn all_available(&self) -> bool {
        self.ledger.all_available()
    }
}

// =============================================================================
// Standard policy constructors
// =============================================================================

impl BudgetPolicy {
    /// Creates a policy with no request-local finite limits.
    ///
    /// This means the resilience policy does not add finite limits of its own.
    /// It does NOT bypass target/runtime/security limits.
    pub fn unlimited() -> Self {
        Self::new()
    }

    /// Creates a policy containing a retry-attempt budget.
    ///
    /// The supplied value is the complete caller-selected limit. No default
    /// retry count is introduced here.
    pub fn with_retry_attempts(
        self,
        limit: u128,
    ) -> Result<Self, BudgetError> {
        self.with_budget(BudgetSpec::quantity(
            BudgetDimension::RetryAttempts,
            limit,
        )?)
    }

    /// Creates a policy containing a shots budget.
    pub fn with_shots(self, limit: u128) -> Result<Self, BudgetError> {
        self.with_budget(BudgetSpec::quantity(
            BudgetDimension::Shots,
            limit,
        )?)
    }

    /// Creates a policy containing a compilation-effort budget.
    ///
    /// The unit is defined by the compiler integration contract.
    pub fn with_compilation_effort(
        self,
        limit: u128,
    ) -> Result<Self, BudgetError> {
        self.with_budget(BudgetSpec::quantity(
            BudgetDimension::CompilationEffort,
            limit,
        )?)
    }

    /// Creates a policy containing a memory budget.
    ///
    /// The unit is defined by the resource integration contract.
    pub fn with_memory(self, limit: u128) -> Result<Self, BudgetError> {
        self.with_budget(BudgetSpec::quantity(
            BudgetDimension::Memory,
            limit,
        )?)
    }

    /// Creates a policy containing an execution-time budget.
    pub fn with_execution_time(
        self,
        limit: Duration,
    ) -> Result<Self, BudgetError> {
        self.with_budget(BudgetSpec::duration(
            BudgetDimension::ExecutionTime,
            limit,
        )?)
    }

    /// Creates a policy containing a QPU-time budget.
    pub fn with_qpu_time(
        self,
        limit: Duration,
    ) -> Result<Self, BudgetError> {
        self.with_budget(BudgetSpec::duration(
            BudgetDimension::QpuTime,
            limit,
        )?)
    }

    /// Creates a policy containing an energy budget.
    ///
    /// The unit is supplied by the hardware/resource integration.
    pub fn with_energy(self, limit: u128) -> Result<Self, BudgetError> {
        self.with_budget(BudgetSpec::quantity(
            BudgetDimension::Energy,
            limit,
        )?)
    }

    /// Creates a policy containing a network budget.
    ///
    /// The unit is supplied by the runtime/network integration.
    pub fn with_network(self, limit: u128) -> Result<Self, BudgetError> {
        self.with_budget(BudgetSpec::quantity(
            BudgetDimension::Network,
            limit,
        )?)
    }

    /// Creates a policy containing a recovery-operation budget.
    pub fn with_recovery(
        self,
        limit: u128,
    ) -> Result<Self, BudgetError> {
        self.with_budget(BudgetSpec::quantity(
            BudgetDimension::Recovery,
            limit,
        )?)
    }

    /// Creates a policy containing a mitigation-overhead budget.
    pub fn with_mitigation(
        self,
        limit: u128,
    ) -> Result<Self, BudgetError> {
        self.with_budget(BudgetSpec::quantity(
            BudgetDimension::Mitigation,
            limit,
        )?)
    }

    /// Creates a policy containing a financial-cost budget.
    ///
    /// The currency/unit is intentionally not embedded in this module.
    pub fn with_financial_cost(
        self,
        limit: u128,
    ) -> Result<Self, BudgetError> {
        self.with_budget(BudgetSpec::quantity(
            BudgetDimension::FinancialCost,
            limit,
        )?)
    }
}

// =============================================================================
// Budget decision
// =============================================================================

/// Deterministic policy decision produced from a budget feasibility check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BudgetDecision {
    /// The proposed requirements fit all configured budgets.
    Allowed,

    /// The proposed requirements are blocked.
    Blocked {
        /// Dimensions preventing the proposed action.
        dimensions: Vec<BudgetDimension>,
    },
}

impl BudgetDecision {
    /// Returns whether the proposed action fits the budgets.
    pub const fn is_allowed(&self) -> bool {
        matches!(self, Self::Allowed)
    }

    /// Returns the blocking dimensions.
    pub fn blocked_dimensions(&self) -> &[BudgetDimension] {
        match self {
            Self::Allowed => &[],
            Self::Blocked { dimensions } => dimensions,
        }
    }
}

// =============================================================================
// Budget evaluator
// =============================================================================

/// Stateless budget evaluator.
///
/// This is deliberately a zero-state type so policy evaluation cannot
/// accidentally introduce global mutable state.
#[derive(Debug, Clone, Copy, Default)]
pub struct BudgetEvaluator;

impl BudgetEvaluator {
    /// Creates an evaluator.
    pub const fn new() -> Self {
        Self
    }

    /// Evaluates requirements against a ledger.
    pub fn evaluate<I>(
        &self,
        ledger: &BudgetLedger,
        requirements: I,
    ) -> Result<BudgetDecision, BudgetError>
    where
        I: IntoIterator<Item = BudgetRequirement>,
    {
        match ledger.evaluate_requirements(requirements)? {
            BudgetFeasibility::Feasible => Ok(BudgetDecision::Allowed),
            BudgetFeasibility::Infeasible { exhausted } => {
                Ok(BudgetDecision::Blocked {
                    dimensions: exhausted,
                })
            }
        }
    }
}

// =============================================================================
// Budget accounting helper
// =============================================================================

/// Explicit accounting helper for a single execution/recovery scope.
///
/// The helper does not know what the execution is doing. It only records the
/// quantities supplied by the caller/integration layer.
#[derive(Debug, Clone)]
pub struct BudgetAccountant {
    ledger: BudgetLedger,
}

impl BudgetAccountant {
    /// Creates an accountant from a policy.
    pub fn from_policy(policy: &BudgetPolicy) -> Self {
        Self {
            ledger: policy.new_ledger(),
        }
    }

    /// Creates an accountant from an existing ledger.
    pub fn new(ledger: BudgetLedger) -> Self {
        Self { ledger }
    }

    /// Records one quantity.
    pub fn consume(
        &mut self,
        dimension: &BudgetDimension,
        amount: BudgetValue,
    ) -> Result<(), BudgetError> {
        self.ledger.consume(dimension, amount)
    }

    /// Records one duration.
    pub fn consume_time(
        &mut self,
        dimension: &BudgetDimension,
        amount: Duration,
    ) -> Result<(), BudgetError> {
        self.ledger.consume_time(dimension, amount)
    }

    /// Records an atomic set of requirements.
    pub fn consume_requirements<I>(
        &mut self,
        requirements: I,
    ) -> Result<(), BudgetError>
    where
        I: IntoIterator<Item = BudgetRequirement>,
    {
        self.ledger.consume_requirements(requirements)
    }

    /// Checks requirements without consuming them.
    pub fn evaluate<I>(
        &self,
        requirements: I,
    ) -> Result<BudgetDecision, BudgetError>
    where
        I: IntoIterator<Item = BudgetRequirement>,
    {
        BudgetEvaluator::new().evaluate(&self.ledger, requirements)
    }

    /// Returns the current snapshot.
    pub fn snapshot(&self) -> BudgetSnapshot {
        self.ledger.snapshot()
    }

    /// Returns the current ledger.
    pub fn ledger(&self) -> &BudgetLedger {
        &self.ledger
    }

    /// Consumes the accountant and returns the updated ledger.
    pub fn into_ledger(self) -> BudgetLedger {
        self.ledger
    }
}

// =============================================================================
// Budget scope
// =============================================================================

/// Identifies the semantic scope to which accounting belongs.
///
/// This is deliberately not tied to a quantum provider or physical qubit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BudgetScope {
    /// Entire resilience invocation.
    Invocation,

    /// One execution attempt.
    ExecutionAttempt,

    /// One adaptation operation.
    Adaptation,

    /// One recovery operation.
    Recovery,

    /// One mitigation operation.
    Mitigation,

    /// One compilation/recompilation operation.
    Compilation,

    /// One user-defined scope.
    Custom,
}

impl BudgetScope {
    /// Stable machine-readable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Invocation => "invocation",
            Self::ExecutionAttempt => "execution_attempt",
            Self::Adaptation => "adaptation",
            Self::Recovery => "recovery",
            Self::Mitigation => "mitigation",
            Self::Compilation => "compilation",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for BudgetScope {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Budget exhaustion policy
// =============================================================================

/// Defines what the policy layer may do after a budget is exhausted.
///
/// This is not execution logic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum BudgetExhaustion {
    /// Stop the current autonomous operation and require policy escalation.
    #[default]
    Escalate,

    /// Reject the current execution/result.
    Reject,

    /// Continue only if another independent policy/constraint layer explicitly
    /// establishes that the budget does not apply to the proposed action.
    ///
    /// This does not authorize bypassing the budget itself.
    ReevaluateScope,
}

impl BudgetExhaustion {
    /// Stable machine-readable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Escalate => "escalate",
            Self::Reject => "reject",
            Self::ReevaluateScope => "reevaluate_scope",
        }
    }
}

impl fmt::Display for BudgetExhaustion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Budget policy configuration
// =============================================================================

/// Complete behavior associated with budget exhaustion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BudgetPolicyConfig {
    /// Budget definitions.
    policy: BudgetPolicy,

    /// Action to take when a budget is exhausted.
    exhaustion: BudgetExhaustion,
}

impl Default for BudgetPolicyConfig {
    fn default() -> Self {
        Self {
            policy: BudgetPolicy::new(),
            exhaustion: BudgetExhaustion::default(),
        }
    }
}

impl BudgetPolicyConfig {
    /// Creates a configuration from a budget policy.
    pub const fn new(
        policy: BudgetPolicy,
        exhaustion: BudgetExhaustion,
    ) -> Self {
        Self {
            policy,
            exhaustion,
        }
    }

    /// Returns the configured policy.
    pub fn policy(&self) -> &BudgetPolicy {
        &self.policy
    }

    /// Returns exhaustion behavior.
    pub const fn exhaustion(&self) -> BudgetExhaustion {
        self.exhaustion
    }

    /// Returns a new configuration with different exhaustion behavior.
    pub const fn with_exhaustion(
        mut self,
        exhaustion: BudgetExhaustion,
    ) -> Self {
        self.exhaustion = exhaustion;
        self
    }
}

// =============================================================================
// Standard dimensions
// =============================================================================

/// Returns all standard non-custom dimensions.
///
/// The returned vector is deterministic.
///
/// This function is informational; it is NOT a machine-size limit.
pub fn standard_dimensions() -> Vec<BudgetDimension> {
    vec![
        BudgetDimension::RetryAttempts,
        BudgetDimension::ExecutionTime,
        BudgetDimension::QpuTime,
        BudgetDimension::Shots,
        BudgetDimension::CompilationEffort,
        BudgetDimension::Memory,
        BudgetDimension::Energy,
        BudgetDimension::Network,
        BudgetDimension::ResourceUsage,
        BudgetDimension::PhysicalResourceUsage,
        BudgetDimension::LogicalResourceUsage,
        BudgetDimension::Mitigation,
        BudgetDimension::Recovery,
        BudgetDimension::Adaptation,
        BudgetDimension::Artifact,
        BudgetDimension::FinancialCost,
    ]
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quantity_budget_accepts_consumption_within_limit() {
        let mut ledger = BudgetLedger::from_specs([
            BudgetSpec::quantity(BudgetDimension::Shots, 100).unwrap(),
        ])
        .unwrap();

        ledger
            .consume(&BudgetDimension::Shots, BudgetValue::new(40))
            .unwrap();

        assert_eq!(
            ledger.state(&BudgetDimension::Shots),
            Some(BudgetState::Available {
                consumed: BudgetValue::new(40),
                remaining: Some(BudgetValue::new(60)),
            })
        );
    }

    #[test]
    fn quantity_budget_rejects_overconsumption_atomically() {
        let mut ledger = BudgetLedger::from_specs([
            BudgetSpec::quantity(BudgetDimension::Shots, 100).unwrap(),
        ])
        .unwrap();

        let result =
            ledger.consume(&BudgetDimension::Shots, BudgetValue::new(101));

        assert!(matches!(
            result,
            Err(BudgetError::Exhausted { .. })
        ));

        assert_eq!(
            ledger.state(&BudgetDimension::Shots),
            Some(BudgetState::Available {
                consumed: BudgetValue::ZERO,
                remaining: Some(BudgetValue::new(100)),
            })
        );
    }

    #[test]
    fn unlimited_budget_never_exhausts() {
        let mut ledger = BudgetLedger::from_specs([
            BudgetSpec::unlimited(BudgetDimension::Shots).unwrap(),
        ])
        .unwrap();

        ledger
            .consume(
                &BudgetDimension::Shots,
                BudgetValue::new(u128::MAX),
            )
            .unwrap();

        assert!(matches!(
            ledger.state(&BudgetDimension::Shots),
            Some(BudgetState::Unlimited { .. })
        ));
    }

    #[test]
    fn time_budget_is_checked_without_wall_clock_access() {
        let mut ledger = BudgetLedger::from_specs([
            BudgetSpec::duration(
                BudgetDimension::ExecutionTime,
                Duration::from_secs(10),
            )
            .unwrap(),
        ])
        .unwrap();

        ledger
            .consume_time(
                &BudgetDimension::ExecutionTime,
                Duration::from_secs(4),
            )
            .unwrap();

        assert_eq!(
            ledger.time_state(&BudgetDimension::ExecutionTime),
            Some(TimeBudgetState::Available {
                consumed: Duration::from_secs(4),
                remaining: Some(Duration::from_secs(6)),
            })
        );
    }

    #[test]
    fn time_budget_rejects_overconsumption_atomically() {
        let mut ledger = BudgetLedger::from_specs([
            BudgetSpec::duration(
                BudgetDimension::ExecutionTime,
                Duration::from_secs(10),
            )
            .unwrap(),
        ])
        .unwrap();

        let result = ledger.consume_time(
            &BudgetDimension::ExecutionTime,
            Duration::from_secs(11),
        );

        assert!(matches!(
            result,
            Err(BudgetError::TimeExhausted { .. })
        ));

        assert_eq!(
            ledger.time_state(&BudgetDimension::ExecutionTime),
            Some(TimeBudgetState::Available {
                consumed: Duration::ZERO,
                remaining: Some(Duration::from_secs(10)),
            })
        );
    }

    #[test]
    fn duplicate_dimensions_are_rejected() {
        let result = BudgetLedger::from_specs([
            BudgetSpec::quantity(BudgetDimension::Shots, 10).unwrap(),
            BudgetSpec::quantity(BudgetDimension::Shots, 20).unwrap(),
        ]);

        assert!(matches!(
            result,
            Err(BudgetError::DuplicateDimension { .. })
        ));
    }

    #[test]
    fn time_and_quantity_dimensions_cannot_be_mixed() {
        let result =
            BudgetSpec::quantity(BudgetDimension::ExecutionTime, 10);

        assert!(matches!(
            result,
            Err(BudgetError::InvalidConfiguration { .. })
        ));
    }

    #[test]
    fn atomic_requirement_consumption() {
        let mut ledger = BudgetLedger::from_specs([
            BudgetSpec::quantity(BudgetDimension::Shots, 100).unwrap(),
            BudgetSpec::quantity(BudgetDimension::Recovery, 1).unwrap(),
        ])
        .unwrap();

        let result = ledger.consume_requirements([
            BudgetRequirement::quantity(
                BudgetDimension::Shots,
                BudgetValue::new(50),
            ),
            BudgetRequirement::quantity(
                BudgetDimension::Recovery,
                BudgetValue::new(2),
            ),
        ]);

        assert!(result.is_err());

        assert_eq!(
            ledger.state(&BudgetDimension::Shots),
            Some(BudgetState::Available {
                consumed: BudgetValue::ZERO,
                remaining: Some(BudgetValue::new(100)),
            })
        );
    }

    #[test]
    fn deterministic_snapshot_order() {
        let ledger = BudgetLedger::from_specs([
            BudgetSpec::quantity(BudgetDimension::Shots, 10).unwrap(),
            BudgetSpec::quantity(BudgetDimension::Recovery, 2).unwrap(),
            BudgetSpec::quantity(
                BudgetDimension::CompilationEffort,
                100,
            )
            .unwrap(),
        ])
        .unwrap();

        let snapshot = ledger.snapshot();

        let dimensions: Vec<BudgetDimension> = snapshot
            .entries()
            .map(|(dimension, _)| dimension.clone())
            .collect();

        let mut sorted = dimensions.clone();
        sorted.sort();

        assert_eq!(dimensions, sorted);
    }

    #[test]
    fn feasibility_does_not_mutate() {
        let ledger = BudgetLedger::from_specs([
            BudgetSpec::quantity(BudgetDimension::Shots, 100).unwrap(),
        ])
        .unwrap();

        let result = ledger
            .evaluate_requirements([BudgetRequirement::quantity(
                BudgetDimension::Shots,
                BudgetValue::new(50),
            )])
            .unwrap();

        assert_eq!(result, BudgetFeasibility::Feasible);

        assert_eq!(
            ledger.state(&BudgetDimension::Shots),
            Some(BudgetState::Unused)
        );
    }

    #[test]
    fn exhaustion_is_explicit() {
        let mut ledger = BudgetLedger::from_specs([
            BudgetSpec::quantity(BudgetDimension::RetryAttempts, 2).unwrap(),
        ])
        .unwrap();

        ledger
            .consume(
                &BudgetDimension::RetryAttempts,
                BudgetValue::new(2),
            )
            .unwrap();

        assert_eq!(
            ledger.state(&BudgetDimension::RetryAttempts),
            Some(BudgetState::Exhausted {
                consumed: BudgetValue::new(2),
                limit: BudgetValue::new(2),
            })
        );
    }

    #[test]
    fn policy_can_build_without_fixed_defaults() {
        let policy = BudgetPolicy::unlimited();

        assert!(policy.all_available());
        assert_eq!(policy.snapshot().len(), 0);
    }

    #[test]
    fn custom_dimensions_are_supported() {
        let dimension =
            BudgetDimension::Custom(String::from("future_resource"));

        let spec = BudgetSpec::quantity(dimension.clone(), 42).unwrap();

        let ledger = BudgetLedger::from_specs([spec]).unwrap();

        assert_eq!(
            ledger.spec(&dimension).map(BudgetSpec::limit),
            Some(BudgetLimit::Quantity(42))
        );
    }

    #[test]
    fn overflow_is_rejected() {
        let mut ledger = BudgetLedger::from_specs([
            BudgetSpec::unlimited(BudgetDimension::Shots).unwrap(),
        ])
        .unwrap();

        ledger
            .consume(
                &BudgetDimension::Shots,
                BudgetValue::new(u128::MAX),
            )
            .unwrap();

        let result = ledger.consume(
            &BudgetDimension::Shots,
            BudgetValue::new(1),
        );

        assert!(matches!(
            result,
            Err(BudgetError::ArithmeticOverflow { .. })
        ));
    }

    #[test]
    fn standard_dimensions_are_deterministic() {
        let dimensions = standard_dimensions();

        let mut sorted = dimensions.clone();
        sorted.sort();

        assert_eq!(dimensions.len(), 16);

        // The list itself is deliberately fixed as a vocabulary, but does not
        // constrain how many custom dimensions may be added.
        assert!(dimensions.iter().all(|dimension| !dimension.is_custom()));

        // Verify that sorting is stable independently of map implementation.
        assert_eq!(
            dimensions
                .iter()
                .map(BudgetDimension::as_str)
                .collect::<Vec<_>>()
                .len(),
            sorted.len()
        );
    }
}