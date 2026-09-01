//! Zamani Quantum Noise (ZQN) — Non-Markovian Noise
//!
//! # Purpose
//!
//! This module defines the backend-independent semantic representation of
//! quantum noise with memory.
//!
//! A Markovian model assumes that the future evolution is completely
//! determined by the present state. A non-Markovian model additionally depends
//! on explicitly represented history, memory state, or a memory kernel.
//!
//! This module therefore provides:
//!
//! - explicit temporal memory;
//! - history-aware noise semantics;
//! - memory kernels;
//! - finite and infinite-memory policies;
//! - sampled/custom kernels;
//! - exponential, stretched-exponential, Gaussian, power-law and oscillatory
//!   kernels;
//! - composite memory kernels;
//! - resource-scoped history;
//! - canonical Quantum IR resource identities;
//! - operation-aware history;
//! - deterministic evaluation;
//! - explicit approximation/error contracts;
//! - bounded-memory execution policies;
//! - streaming-friendly history access;
//! - state evolution through user-supplied memory state;
//! - validation and numerical safety;
//! - no hidden global state;
//! - no global RNG;
//! - no unsafe code.
//!
//! # Architectural position
//!
//! ```text
//!                     crate::quantum::ir
//!                            │
//!                            │ canonical operation/resource identity
//!                            ▼
//!                     ┌──────────────┐
//!                     │     ZQN      │
//!                     │              │
//!                     │ non_markovian│
//!                     └──────┬───────┘
//!                            │
//!             ┌──────────────┼──────────────┐
//!             │              │              │
//!             ▼              ▼              ▼
//!       temporal.rs     correlation.rs   calibration
//!             │              │              │
//!             └──────────────┼──────────────┘
//!                            ▼
//!                    noise/application
//!                            │
//!             ┌──────────────┼──────────────┐
//!             ▼              ▼              ▼
//!         simulation        QEC         scheduling
//!                            │
//!                            ▼
//!                         hardware
//! ```
//!
//! # Ownership
//!
//! This file owns:
//!
//! - non-Markovian process semantics;
//! - memory-kernel semantics;
//! - history-entry semantics;
//! - history retention policies;
//! - deterministic memory evaluation;
//! - memory-state contracts;
//! - non-Markovian validation;
//! - explicit approximation declarations.
//!
//! # Does not own
//!
//! This module does NOT own:
//!
//! - canonical quantum IR;
//! - quantum state vectors;
//! - density matrices;
//! - Kraus operators;
//! - Choi matrices;
//! - probability distributions;
//! - random-number generation;
//! - QEC decoding;
//! - routing;
//! - scheduling;
//! - hardware APIs;
//! - calibration storage;
//! - serialization wire formats;
//! - global registries;
//! - global mutable state.
//!
//! Those responsibilities remain in their respective modules.
//!
//! # Canonical quantum identities
//!
//! Where history is associated with canonical quantum resources, this module
//! uses:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! crate::quantum::ir::identity::OperationId
//! ```
//!
//! No ZQN-specific qubit identity is introduced.
//!
//! # Write once, scale everywhere
//!
//! There is no semantic upper bound on:
//!
//! - number of resources;
//! - number of history entries;
//! - number of correlated resources;
//! - number of operations;
//! - process duration;
//! - memory depth;
//! - circuit depth.
//!
//! An execution MAY impose explicit resource limits through the surrounding
//! ZQN context. Such limits are policies, not mathematical limits.
//!
//! The implementation never contains `MAX_QUBITS`, `MAX_HISTORY`, or similar
//! machine-size constants.
//!
//! # Important distinction
//!
//! "Unlimited" means:
//!
//! ```text
//! no artificial semantic ceiling
//! ```
//!
//! It does NOT mean:
//!
//! ```text
//! infinite RAM / CPU / storage / execution time
//! ```
//!
//! A full-history process may consume resources proportional to the history
//! retained by the caller. A bounded policy exists precisely so applications
//! can choose a resource/fidelity trade-off explicitly.
//!
//! # Determinism
//!
//! This module is deterministic by construction.
//!
//! It does NOT:
//!
//! - generate random numbers;
//! - read the wall clock;
//! - use thread identity;
//! - use memory addresses;
//! - depend on hash-map iteration order;
//! - maintain global state.
//!
//! Stochastic realization belongs to the simulation/sampling subsystem.
//!
//! # Numerical policy
//!
//! Time and kernel values use `f64` at this semantic boundary because the
//! existing ZQN temporal layer is numerical and the kernel API needs a
//! backend-neutral scalar. Values are validated for finiteness before use.
//!
//! This module does not silently clamp invalid values.
//!
//! A caller requiring arbitrary precision can implement `MemoryKernel` over a
//! different numerical layer and use the semantic types as integration
//! metadata. No state-vector or matrix precision is prescribed here.
//!
//! # Approximation policy
//!
//! Truncating memory is an approximation unless the discarded contribution is
//! mathematically known to be zero.
//!
//! The approximation must therefore be explicit through `MemoryRetention` and
//! `NonMarkovianGuarantee`.
//!
//! A bounded-memory execution must never silently claim exact infinite-memory
//! semantics.
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
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use std::collections::VecDeque;

use crate::quantum::ir::identity::OperationId;
use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

// ============================================================================
// Scalar validation
// ============================================================================

const SECONDS_EPSILON: f64 = f64::EPSILON;

/// Validates a finite scalar.
///
/// This helper deliberately does not clamp values.
fn finite(value: f64, name: &'static str) -> Result<f64, NonMarkovianError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(NonMarkovianError::NonFiniteValue { field: name })
    }
}

/// Validates a non-negative finite scalar.
fn non_negative(value: f64, name: &'static str) -> Result<f64, NonMarkovianError> {
    let value = finite(value, name)?;

    if value < 0.0 {
        return Err(NonMarkovianError::NegativeValue { field: name });
    }

    Ok(value)
}

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by the non-Markovian semantic layer.
///
/// This domain error is intentionally independent of the global ZQN diagnostic
/// representation. An integration layer may convert it into `ZqnError` without
/// coupling this file to unrelated error infrastructure.
#[derive(Debug, Clone, PartialEq)]
pub enum NonMarkovianError {
    /// A numerical input was NaN or infinite.
    NonFiniteValue {
        /// Semantic field containing the invalid value.
        field: &'static str,
    },

    /// A value that must be non-negative was negative.
    NegativeValue {
        /// Semantic field containing the invalid value.
        field: &'static str,
    },

    /// A value that must be strictly positive was zero or negative.
    NonPositiveValue {
        /// Semantic field containing the invalid value.
        field: &'static str,
    },

    /// A time coordinate was earlier than the reference required by an API.
    InvalidTimeOrdering,

    /// A kernel's parameters are invalid.
    InvalidKernelParameters {
        /// Human-readable explanation.
        reason: &'static str,
    },

    /// A sampled kernel has inconsistent coordinates/values.
    InvalidSampledKernel {
        /// Human-readable explanation.
        reason: &'static str,
    },

    /// A history entry was inconsistent with the requested temporal scope.
    InvalidHistoryEntry {
        /// Human-readable explanation.
        reason: &'static str,
    },

    /// A bounded-memory policy was requested without a meaningful bound.
    InvalidRetentionPolicy {
        /// Human-readable explanation.
        reason: &'static str,
    },

    /// A memory state implementation rejected an operation.
    MemoryStateFailure {
        /// Human-readable explanation.
        reason: &'static str,
    },

    /// A requested exact evaluation is impossible under the selected policy.
    ExactEvaluationUnavailable {
        /// Human-readable explanation.
        reason: &'static str,
    },
}

impl fmt::Display for NonMarkovianError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteValue { field } => {
                write!(formatter, "{field} must be finite")
            }
            Self::NegativeValue { field } => {
                write!(formatter, "{field} must be non-negative")
            }
            Self::NonPositiveValue { field } => {
                write!(formatter, "{field} must be positive")
            }
            Self::InvalidTimeOrdering => {
                formatter.write_str("time coordinates are not monotonically ordered")
            }
            Self::InvalidKernelParameters { reason } => {
                write!(formatter, "invalid memory-kernel parameters: {reason}")
            }
            Self::InvalidSampledKernel { reason } => {
                write!(formatter, "invalid sampled memory kernel: {reason}")
            }
            Self::InvalidHistoryEntry { reason } => {
                write!(formatter, "invalid history entry: {reason}")
            }
            Self::InvalidRetentionPolicy { reason } => {
                write!(formatter, "invalid memory-retention policy: {reason}")
            }
            Self::MemoryStateFailure { reason } => {
                write!(formatter, "memory-state failure: {reason}")
            }
            Self::ExactEvaluationUnavailable { reason } => {
                write!(formatter, "exact non-Markovian evaluation unavailable: {reason}")
            }
        }
    }
}

impl std::error::Error for NonMarkovianError {}

// ============================================================================
// Time
// ============================================================================

/// Absolute semantic time measured in seconds relative to an execution-defined
/// origin.
///
/// The origin is supplied by the caller. This type deliberately does not read
/// a system clock.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct NonMarkovianTime(f64);

impl NonMarkovianTime {
    /// Creates a semantic time coordinate.
    pub fn new(seconds: f64) -> Result<Self, NonMarkovianError> {
        finite(seconds, "time")?;
        Ok(Self(seconds))
    }

    /// Creates the zero-time coordinate.
    #[must_use]
    pub const fn zero() -> Self {
        Self(0.0)
    }

    /// Returns seconds from the caller-defined temporal origin.
    #[must_use]
    pub const fn seconds(self) -> f64 {
        self.0
    }

    /// Computes the non-negative lag from an earlier time to this time.
    pub fn lag_from(
        self,
        earlier: Self,
    ) -> Result<f64, NonMarkovianError> {
        let lag = self.0 - earlier.0;

        if lag < -SECONDS_EPSILON {
            return Err(NonMarkovianError::InvalidTimeOrdering);
        }

        Ok(if lag < 0.0 { 0.0 } else { lag })
    }
}

impl Eq for NonMarkovianTime {}

impl Ord for NonMarkovianTime {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl fmt::Display for NonMarkovianTime {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}s", self.0)
    }
}

// ============================================================================
// Resource scope
// ============================================================================

/// Quantum resource to which a memory process may be scoped.
///
/// Canonical qubit identifiers come directly from `quantum::ir::qubit`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NonMarkovianResource {
    /// Logical qubit.
    LogicalQubit(QubitId),

    /// Physical qubit.
    PhysicalQubit(PhysicalQubitId),
}

impl NonMarkovianResource {
    /// Returns true when the resource is a logical qubit.
    #[must_use]
    pub const fn is_logical(self) -> bool {
        matches!(self, Self::LogicalQubit(_))
    }

    /// Returns true when the resource is a physical qubit.
    #[must_use]
    pub const fn is_physical(self) -> bool {
        matches!(self, Self::PhysicalQubit(_))
    }
}

// ============================================================================
// Operation scope
// ============================================================================

/// Optional canonical operation identity associated with a memory event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NonMarkovianOperation {
    operation: OperationId,
}

impl NonMarkovianOperation {
    /// Creates an operation reference from the canonical Quantum IR ID.
    #[must_use]
    pub const fn new(operation: OperationId) -> Self {
        Self { operation }
    }

    /// Returns the canonical operation identity.
    #[must_use]
    pub const fn id(self) -> OperationId {
        self.operation
    }
}

// ============================================================================
// History event
// ============================================================================

/// A semantic event contributing to the memory of a non-Markovian process.
///
/// The event does not contain a quantum state. The simulation or execution
/// layer remains responsible for state evolution.
///
/// `weight` is a model-defined scalar observable associated with the event.
/// It can represent, for example, an error amplitude, deviation magnitude,
/// control perturbation, or another scalar coupling variable.
///
/// Vector/tensor observables should be represented by a domain-specific
/// `MemoryState` implementation rather than forcing this module into a fixed
/// physical dimensionality.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MemoryEvent {
    time: NonMarkovianTime,
    resource: Option<NonMarkovianResource>,
    operation: Option<NonMarkovianOperation>,
    weight: f64,
}

impl MemoryEvent {
    /// Creates a memory event.
    pub fn new(
        time: NonMarkovianTime,
        weight: f64,
    ) -> Result<Self, NonMarkovianError> {
        finite(weight, "event weight")?;

        Ok(Self {
            time,
            resource: None,
            operation: None,
            weight,
        })
    }

    /// Associates the event with a canonical quantum resource.
    #[must_use]
    pub const fn with_resource(
        mut self,
        resource: NonMarkovianResource,
    ) -> Self {
        self.resource = Some(resource);
        self
    }

    /// Associates the event with a canonical operation.
    #[must_use]
    pub const fn with_operation(
        mut self,
        operation: NonMarkovianOperation,
    ) -> Self {
        self.operation = Some(operation);
        self
    }

    /// Event time.
    #[must_use]
    pub const fn time(self) -> NonMarkovianTime {
        self.time
    }

    /// Optional resource.
    #[must_use]
    pub const fn resource(self) -> Option<NonMarkovianResource> {
        self.resource
    }

    /// Optional operation.
    #[must_use]
    pub const fn operation(self) -> Option<NonMarkovianOperation> {
        self.operation
    }

    /// Scalar memory-driving weight.
    #[must_use]
    pub const fn weight(self) -> f64 {
        self.weight
    }
}

// ============================================================================
// Memory retention
// ============================================================================

/// Explicit policy controlling how much history is retained.
///
/// The policy is part of semantics because truncating history changes a
/// genuinely non-Markovian process unless the discarded contribution is known
/// to be zero.
///
/// `Full` means that the caller requests complete retained history. It still
/// remains subject to external resource availability.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemoryRetention {
    /// Retain all supplied history.
    Full,

    /// Retain only events within the specified duration before the newest
    /// event.
    Duration(f64),

    /// Retain at most this many events.
    Count(usize),

    /// Retain events until the accumulated absolute weight reaches the
    /// specified budget.
    ///
    /// This is a resource/fidelity policy and therefore must be documented by
    /// the caller as an approximation when non-zero events are discarded.
    Weight(f64),
}

impl MemoryRetention {
    /// Validates the policy.
    pub fn validate(self) -> Result<Self, NonMarkovianError> {
        match self {
            Self::Full => Ok(self),
            Self::Duration(value) => {
                if !value.is_finite() {
                    return Err(NonMarkovianError::NonFiniteValue {
                        field: "retention duration",
                    });
                }

                if value < 0.0 {
                    return Err(NonMarkovianError::NegativeValue {
                        field: "retention duration",
                    });
                }

                Ok(self)
            }
            Self::Count(value) => {
                if value == 0 {
                    return Err(NonMarkovianError::InvalidRetentionPolicy {
                        reason: "count retention must be greater than zero",
                    });
                }

                Ok(self)
            }
            Self::Weight(value) => {
                if !value.is_finite() {
                    return Err(NonMarkovianError::NonFiniteValue {
                        field: "retention weight",
                    });
                }

                if value <= 0.0 {
                    return Err(NonMarkovianError::NonPositiveValue {
                        field: "retention weight",
                    });
                }

                Ok(self)
            }
        }
    }
}

// ============================================================================
// History
// ============================================================================

/// Ordered non-Markovian history.
///
/// History is intentionally stored as an ordered deque rather than a hash map.
/// Temporal order is semantic and must never depend on hash iteration order.
///
/// The structure itself does not impose a fixed capacity. Capacity is governed
/// by `MemoryRetention`.
#[derive(Debug, Clone)]
pub struct MemoryHistory {
    events: VecDeque<MemoryEvent>,
    retention: MemoryRetention,
}

impl MemoryHistory {
    /// Creates an empty history with an explicit retention policy.
    pub fn new(
        retention: MemoryRetention,
    ) -> Result<Self, NonMarkovianError> {
        retention.validate()?;

        Ok(Self {
            events: VecDeque::new(),
            retention,
        })
    }

    /// Returns the retention policy.
    #[must_use]
    pub const fn retention(&self) -> MemoryRetention {
        self.retention
    }

    /// Number of retained events.
    #[must_use]
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Returns true if no events are retained.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Adds an event while preserving temporal ordering.
    ///
    /// Events at equal timestamps are retained in insertion order.
    pub fn push(
        &mut self,
        event: MemoryEvent,
    ) -> Result<(), NonMarkovianError> {
        if let Some(last) = self.events.back() {
            if event.time() < last.time() {
                return Err(NonMarkovianError::InvalidTimeOrdering);
            }
        }

        self.events.push_back(event);
        self.prune();

        Ok(())
    }

    /// Returns an iterator over retained events in chronological order.
    pub fn iter(&self) -> impl Iterator<Item = &MemoryEvent> {
        self.events.iter()
    }

    /// Returns the newest retained event.
    #[must_use]
    pub fn newest(&self) -> Option<&MemoryEvent> {
        self.events.back()
    }

    /// Returns the oldest retained event.
    #[must_use]
    pub fn oldest(&self) -> Option<&MemoryEvent> {
        self.events.front()
    }

    /// Returns the total absolute retained weight.
    #[must_use]
    pub fn absolute_weight(&self) -> f64 {
        self.events
            .iter()
            .map(|event| event.weight().abs())
            .sum()
    }

    /// Clears all retained history.
    pub fn clear(&mut self) {
        self.events.clear();
    }

    fn prune(&mut self) {
        match self.retention {
            MemoryRetention::Full => {}

            MemoryRetention::Count(max_count) => {
                while self.events.len() > max_count {
                    let _ = self.events.pop_front();
                }
            }

            MemoryRetention::Duration(duration) => {
                let newest = match self.events.back() {
                    Some(event) => event.time(),
                    None => return,
                };

                while let Some(oldest) = self.events.front() {
                    let lag = match newest.lag_from(oldest.time()) {
                        Ok(value) => value,
                        Err(_) => break,
                    };

                    if lag <= duration {
                        break;
                    }

                    let _ = self.events.pop_front();
                }
            }

            MemoryRetention::Weight(max_weight) => {
                while self.events.len() > 1
                    && self.absolute_weight() > max_weight
                {
                    let _ = self.events.pop_front();
                }
            }
        }
    }
}

// ============================================================================
// Memory-kernel semantics
// ============================================================================

/// Evaluation of a memory kernel.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KernelValue {
    /// Kernel value.
    value: f64,

    /// Whether this value is exactly known to be zero outside a finite
    /// support.
    compact_support_zero: bool,
}

impl KernelValue {
    /// Creates a kernel value.
    pub fn new(
        value: f64,
        compact_support_zero: bool,
    ) -> Result<Self, NonMarkovianError> {
        finite(value, "kernel value")?;

        Ok(Self {
            value,
            compact_support_zero,
        })
    }

    /// Returns the kernel value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.value
    }

    /// Returns whether the kernel is exactly zero for this evaluated point due
    /// to finite support.
    #[must_use]
    pub const fn is_compact_support_zero(self) -> bool {
        self.compact_support_zero
    }
}

/// Backend-independent memory-kernel contract.
///
/// A kernel receives only a non-negative time lag. It has no access to global
/// state, clocks, RNGs, threads, hardware, or quantum state.
pub trait MemoryKernel: Send + Sync {
    /// Evaluates the kernel at a non-negative time lag.
    fn evaluate(&self, lag_seconds: f64) -> Result<KernelValue, NonMarkovianError>;

    /// Returns the characteristic memory timescale when one exists.
    ///
    /// `None` means that no single scalar timescale adequately describes the
    /// kernel.
    fn characteristic_time(&self) -> Option<f64>;

    /// Returns true when the kernel has finite support.
    fn has_finite_support(&self) -> bool;

    /// Returns a stable semantic name for diagnostics and provenance.
    fn name(&self) -> &'static str;
}

// ============================================================================
// Built-in kernels
// ============================================================================

/// Built-in memory kernels.
///
/// These are mathematical primitives, not machine-specific assumptions.
///
/// Arbitrary kernels remain possible through the `MemoryKernel` trait.
#[derive(Debug, Clone, PartialEq)]
pub enum MemoryKernelKind {
    /// Exponential memory:
    ///
    /// `exp(-t / tau)`
    Exponential {
        /// Positive correlation time.
        tau: f64,
    },

    /// Stretched exponential:
    ///
    /// `exp(-(t / tau)^beta)`
    StretchedExponential {
        /// Positive characteristic time.
        tau: f64,

        /// Positive shape parameter.
        beta: f64,
    },

    /// Gaussian memory:
    ///
    /// `exp(-(t / tau)^2)`
    Gaussian {
        /// Positive characteristic time.
        tau: f64,
    },

    /// Power-law memory:
    ///
    /// `(1 + t / tau)^(-alpha)`
    PowerLaw {
        /// Positive scale.
        tau: f64,

        /// Positive decay exponent.
        alpha: f64,
    },

    /// Damped oscillatory memory:
    ///
    /// `exp(-t / tau) * cos(omega * t + phase)`
    Oscillatory {
        /// Positive decay time.
        tau: f64,

        /// Angular frequency in radians per second.
        omega: f64,

        /// Phase in radians.
        phase: f64,
    },

    /// Finite-support rectangular memory:
    ///
    /// `1` for `0 <= t <= support`, otherwise `0`.
    Compact {
        /// Non-negative support duration.
        support: f64,
    },
}

impl MemoryKernelKind {
    /// Validates kernel parameters.
    pub fn validate(&self) -> Result<(), NonMarkovianError> {
        match self {
            Self::Exponential { tau } => {
                validate_positive(*tau, "tau")
            }

            Self::StretchedExponential { tau, beta } => {
                validate_positive(*tau, "tau")?;
                validate_positive(*beta, "beta")
            }

            Self::Gaussian { tau } => {
                validate_positive(*tau, "tau")
            }

            Self::PowerLaw { tau, alpha } => {
                validate_positive(*tau, "tau")?;
                validate_positive(*alpha, "alpha")
            }

            Self::Oscillatory {
                tau,
                omega,
                phase,
            } => {
                validate_positive(*tau, "tau")?;
                finite(*omega, "omega")?;
                finite(*phase, "phase")?;
                Ok(())
            }

            Self::Compact { support } => {
                non_negative(*support, "support")?;
                Ok(())
            }
        }
    }
}

fn validate_positive(
    value: f64,
    field: &'static str,
) -> Result<(), NonMarkovianError> {
    let value = finite(value, field)?;

    if value <= 0.0 {
        return Err(NonMarkovianError::NonPositiveValue { field });
    }

    Ok(())
}

impl MemoryKernel for MemoryKernelKind {
    fn evaluate(
        &self,
        lag_seconds: f64,
    ) -> Result<KernelValue, NonMarkovianError> {
        self.validate()?;

        let t = non_negative(lag_seconds, "lag")?;

        let value = match self {
            Self::Exponential { tau } => (-t / *tau).exp(),

            Self::StretchedExponential { tau, beta } => {
                (-(t / *tau).powf(*beta)).exp()
            }

            Self::Gaussian { tau } => {
                (-(t / *tau).powi(2)).exp()
            }

            Self::PowerLaw { tau, alpha } => {
                (1.0 + t / *tau).powf(-*alpha)
            }

            Self::Oscillatory {
                tau,
                omega,
                phase,
            } => {
                (-t / *tau).exp() * (*omega * t + *phase).cos()
            }

            Self::Compact { support } => {
                if t <= *support {
                    1.0
                } else {
                    0.0
                }
            }
        };

        let compact_zero = matches!(
            self,
            Self::Compact { .. }
        ) && value == 0.0;

        KernelValue::new(value, compact_zero)
    }

    fn characteristic_time(&self) -> Option<f64> {
        match self {
            Self::Exponential { tau }
            | Self::Gaussian { tau }
            | Self::PowerLaw { tau, .. }
            | Self::Oscillatory { tau, .. }
            | Self::StretchedExponential { tau, .. } => Some(*tau),

            Self::Compact { support } => Some(*support),
        }
    }

    fn has_finite_support(&self) -> bool {
        matches!(self, Self::Compact { .. })
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Exponential { .. } => "exponential",
            Self::StretchedExponential { .. } => "stretched_exponential",
            Self::Gaussian { .. } => "gaussian",
            Self::PowerLaw { .. } => "power_law",
            Self::Oscillatory { .. } => "oscillatory",
            Self::Compact { .. } => "compact",
        }
    }
}

// ============================================================================
// Sampled kernel
// ============================================================================

/// A deterministic piecewise-linear memory kernel.
///
/// Samples must be supplied in strictly increasing time order. Values may be
/// positive or negative because a memory kernel is not itself a probability
/// distribution.
///
/// Outside the supplied domain, the kernel evaluates to zero.
///
/// This provides a practical bridge from experimentally characterized memory
/// functions to the generic `MemoryKernel` contract.
#[derive(Debug, Clone, PartialEq)]
pub struct SampledMemoryKernel {
    samples: Vec<(f64, f64)>,
}

impl SampledMemoryKernel {
    /// Creates a sampled kernel.
    ///
    /// The vector is consumed once and retained in deterministic order.
    pub fn new(
        samples: Vec<(f64, f64)>,
    ) -> Result<Self, NonMarkovianError> {
        if samples.is_empty() {
            return Err(NonMarkovianError::InvalidSampledKernel {
                reason: "at least one sample is required",
            });
        }

        let mut previous = None;

        for (time, value) in &samples {
            non_negative(*time, "sample time")?;
            finite(*value, "sample value")?;

            if let Some(previous_time) = previous {
                if *time <= previous_time {
                    return Err(NonMarkovianError::InvalidSampledKernel {
                        reason: "sample times must be strictly increasing",
                    });
                }
            }

            previous = Some(*time);
        }

        Ok(Self { samples })
    }

    /// Returns the supplied samples.
    pub fn samples(&self) -> impl Iterator<Item = &(f64, f64)> {
        self.samples.iter()
    }
}

impl MemoryKernel for SampledMemoryKernel {
    fn evaluate(
        &self,
        lag_seconds: f64,
    ) -> Result<KernelValue, NonMarkovianError> {
        let t = non_negative(lag_seconds, "lag")?;

        if t < self.samples[0].0 {
            return KernelValue::new(0.0, true);
        }

        if let Some(&(last_time, last_value)) = self.samples.last() {
            if t > last_time {
                return KernelValue::new(0.0, true);
            }

            if t == last_time {
                return KernelValue::new(last_value, false);
            }
        }

        for window in self.samples.windows(2) {
            let (t0, v0) = window[0];
            let (t1, v1) = window[1];

            if t >= t0 && t <= t1 {
                let span = t1 - t0;

                let fraction = if span == 0.0 {
                    0.0
                } else {
                    (t - t0) / span
                };

                let value = v0 + fraction * (v1 - v0);

                return KernelValue::new(value, false);
            }
        }

        KernelValue::new(0.0, true)
    }

    fn characteristic_time(&self) -> Option<f64> {
        self.samples.last().map(|sample| sample.0)
    }

    fn has_finite_support(&self) -> bool {
        true
    }

    fn name(&self) -> &'static str {
        "sampled"
    }
}

// ============================================================================
// Composite kernel
// ============================================================================

/// Composition of multiple memory kernels.
///
/// The coefficients are arbitrary real values; they are not probabilities.
///
/// The composed kernel is:
///
/// `K(t) = sum_i coefficient_i * K_i(t)`
///
/// This permits users to represent multi-timescale environments without
/// creating a fixed number of memory components.
pub struct CompositeMemoryKernel {
    terms: Vec<(f64, Box<dyn MemoryKernel>)>,
}

impl CompositeMemoryKernel {
    /// Creates an empty composite kernel.
    #[must_use]
    pub fn new() -> Self {
        Self { terms: Vec::new() }
    }

    /// Creates a composite kernel from supplied terms.
    pub fn from_terms(
        terms: Vec<(f64, Box<dyn MemoryKernel>)>,
    ) -> Result<Self, NonMarkovianError> {
        let kernel = Self { terms };

        kernel.validate()?;

        Ok(kernel)
    }

    /// Adds a weighted kernel.
    pub fn push(
        &mut self,
        coefficient: f64,
        kernel: Box<dyn MemoryKernel>,
    ) -> Result<(), NonMarkovianError> {
        finite(coefficient, "kernel coefficient")?;

        self.terms.push((coefficient, kernel));

        self.validate()
    }

    /// Number of component kernels.
    #[must_use]
    pub fn len(&self) -> usize {
        self.terms.len()
    }

    /// Whether the composite contains no terms.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.terms.is_empty()
    }

    /// Validates all component kernels.
    pub fn validate(&self) -> Result<(), NonMarkovianError> {
        for (coefficient, kernel) in &self.terms {
            finite(*coefficient, "kernel coefficient")?;
            kernel.evaluate(0.0)?;
        }

        Ok(())
    }
}

impl Default for CompositeMemoryKernel {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryKernel for CompositeMemoryKernel {
    fn evaluate(
        &self,
        lag_seconds: f64,
    ) -> Result<KernelValue, NonMarkovianError> {
        self.validate()?;

        let mut value = 0.0;
        let mut all_zero = true;

        for (coefficient, kernel) in &self.terms {
            let evaluated = kernel.evaluate(lag_seconds)?;
            value += *coefficient * evaluated.value();

            if evaluated.value() != 0.0 {
                all_zero = false;
            }
        }

        finite(value, "composite kernel value")?;

        KernelValue::new(value, all_zero)
    }

    fn characteristic_time(&self) -> Option<f64> {
        let mut maximum = None;

        for (_, kernel) in &self.terms {
            if let Some(value) = kernel.characteristic_time() {
                maximum = Some(match maximum {
                    Some(current) if current >= value => current,
                    _ => value,
                });
            }
        }

        maximum
    }

    fn has_finite_support(&self) -> bool {
        self.terms
            .iter()
            .all(|(_, kernel)| kernel.has_finite_support())
    }

    fn name(&self) -> &'static str {
        "composite"
    }
}

// ============================================================================
// Memory state
// ============================================================================

/// Context supplied when evaluating a memory state.
///
/// It contains no hidden clock or RNG.
#[derive(Debug, Clone, Copy)]
pub struct MemoryEvaluationContext<'a> {
    /// Current semantic time.
    pub current_time: NonMarkovianTime,

    /// Read-only retained history.
    pub history: &'a MemoryHistory,
}

/// Generic stateful memory contract.
///
/// This abstraction exists because a general non-Markovian environment cannot
/// always be reduced to one scalar convolution.
///
/// Examples of valid implementations include:
///
/// - scalar memory;
/// - vector memory;
/// - tensor memory;
/// - auxiliary density operators;
/// - hierarchical equations of motion;
/// - pseudomode state;
/// - hidden environmental state;
/// - process-tensor state.
///
/// The implementation remains outside this module.
pub trait MemoryState: Send + Sync {
    /// Concrete state representation associated with the memory environment.
    type State: Clone + Send + Sync;

    /// Creates the initial memory state.
    fn initial_state(&self) -> Result<Self::State, NonMarkovianError>;

    /// Evolves memory state using the explicit current event/history.
    fn evolve(
        &self,
        state: &Self::State,
        context: &MemoryEvaluationContext<'_>,
    ) -> Result<Self::State, NonMarkovianError>;

    /// Produces the scalar memory contribution exposed to a generic
    /// non-Markovian process.
    ///
    /// More specialized consumers may use the full `State` directly.
    fn contribution(
        &self,
        state: &Self::State,
        context: &MemoryEvaluationContext<'_>,
    ) -> Result<f64, NonMarkovianError>;

    /// Stable semantic name.
    fn name(&self) -> &'static str;
}

// ============================================================================
// Kernel convolution
// ============================================================================

/// Evaluates a scalar memory convolution over explicit history.
///
/// Conceptually:
///
/// `M(t) = Σ_i K(t - t_i) * w_i`
///
/// This discrete representation is deliberately explicit. Continuous
/// integrators belong to the numerical simulation subsystem.
///
/// The history may contain arbitrarily many events subject only to available
/// resources and caller-selected retention policy.
pub fn convolve_history(
    kernel: &dyn MemoryKernel,
    current_time: NonMarkovianTime,
    history: &MemoryHistory,
) -> Result<f64, NonMarkovianError> {
    let mut result = 0.0;

    for event in history.iter() {
        let lag = current_time.lag_from(event.time())?;
        let kernel_value = kernel.evaluate(lag)?;

        result += kernel_value.value() * event.weight();

        finite(result, "history convolution")?;
    }

    Ok(result)
}

// ============================================================================
// Semantic guarantee
// ============================================================================

/// Semantic guarantee of a non-Markovian realization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NonMarkovianGuarantee {
    /// Full requested history is represented.
    Exact,

    /// The history is deliberately truncated.
    Truncated,

    /// The kernel/model provides a mathematical error bound for the
    /// approximation.
    Bounded,

    /// Parameters or observations are statistical.
    Statistical,
}

impl NonMarkovianGuarantee {
    /// Returns true when the guarantee claims exact history.
    #[must_use]
    pub const fn is_exact(self) -> bool {
        matches!(self, Self::Exact)
    }
}

// ============================================================================
// Process descriptor
// ============================================================================

/// Stable semantic descriptor for a non-Markovian process.
#[derive(Debug, Clone, PartialEq)]
pub struct NonMarkovianDescriptor {
    /// Human-readable model name.
    name: String,

    /// Semantic revision.
    revision: (u32, u32, u32),

    /// Memory guarantee.
    guarantee: NonMarkovianGuarantee,

    /// Whether the process is explicitly history-dependent.
    history_dependent: bool,

    /// Whether the kernel has finite support.
    finite_support: bool,
}

impl NonMarkovianDescriptor {
    /// Creates a descriptor.
    pub fn new(
        name: impl Into<String>,
        revision: (u32, u32, u32),
        guarantee: NonMarkovianGuarantee,
        finite_support: bool,
    ) -> Result<Self, NonMarkovianError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(NonMarkovianError::InvalidKernelParameters {
                reason: "process name must not be empty",
            });
        }

        Ok(Self {
            name,
            revision,
            guarantee,
            history_dependent: true,
            finite_support,
        })
    }

    /// Process name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Semantic revision.
    #[must_use]
    pub const fn revision(&self) -> (u32, u32, u32) {
        self.revision
    }

    /// Guarantee.
    #[must_use]
    pub const fn guarantee(&self) -> NonMarkovianGuarantee {
        self.guarantee
    }

    /// Whether history affects the process.
    #[must_use]
    pub const fn history_dependent(&self) -> bool {
        self.history_dependent
    }

    /// Whether the memory kernel has finite support.
    #[must_use]
    pub const fn finite_support(&self) -> bool {
        self.finite_support
    }
}

// ============================================================================
// Generic non-Markovian process
// ============================================================================

/// Canonical backend-independent non-Markovian process.
///
/// The process combines:
///
/// - an explicit memory kernel;
/// - an explicit history;
/// - an optional stateful memory environment.
///
/// The mathematical process is not itself a simulator and does not apply a
/// quantum channel.
pub struct NonMarkovianProcess {
    descriptor: NonMarkovianDescriptor,
    kernel: Box<dyn MemoryKernel>,
    history: MemoryHistory,
}

impl NonMarkovianProcess {
    /// Creates a non-Markovian process.
    pub fn new(
        descriptor: NonMarkovianDescriptor,
        kernel: Box<dyn MemoryKernel>,
        retention: MemoryRetention,
    ) -> Result<Self, NonMarkovianError> {
        retention.validate()?;
        kernel.evaluate(0.0)?;

        if descriptor.finite_support() != kernel.has_finite_support() {
            return Err(NonMarkovianError::InvalidKernelParameters {
                reason: "descriptor finite-support flag does not match kernel",
            });
        }

        Ok(Self {
            descriptor,
            kernel,
            history: MemoryHistory::new(retention)?,
        })
    }

    /// Creates a process directly from a kernel.
    pub fn from_kernel(
        name: impl Into<String>,
        revision: (u32, u32, u32),
        kernel: Box<dyn MemoryKernel>,
        retention: MemoryRetention,
        guarantee: NonMarkovianGuarantee,
    ) -> Result<Self, NonMarkovianError> {
        let finite_support = kernel.has_finite_support();

        let descriptor = NonMarkovianDescriptor::new(
            name,
            revision,
            guarantee,
            finite_support,
        )?;

        Self::new(descriptor, kernel, retention)
    }

    /// Returns the immutable process descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &NonMarkovianDescriptor {
        &self.descriptor
    }

    /// Returns the configured memory kernel.
    #[must_use]
    pub fn kernel(&self) -> &dyn MemoryKernel {
        self.kernel.as_ref()
    }

    /// Returns the current retained history.
    #[must_use]
    pub fn history(&self) -> &MemoryHistory {
        &self.history
    }

    /// Adds one memory event.
    pub fn record(
        &mut self,
        event: MemoryEvent,
    ) -> Result<(), NonMarkovianError> {
        self.history.push(event)
    }

    /// Evaluates the current scalar memory contribution.
    pub fn evaluate(
        &self,
        current_time: NonMarkovianTime,
    ) -> Result<f64, NonMarkovianError> {
        convolve_history(
            self.kernel.as_ref(),
            current_time,
            &self.history,
        )
    }

    /// Clears retained history.
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Returns whether exact full-history semantics are currently represented.
    #[must_use]
    pub fn is_exact(&self) -> bool {
        self.descriptor.guarantee().is_exact()
    }

    /// Validates the complete process contract.
    pub fn validate(&self) -> Result<(), NonMarkovianError> {
        self.history.retention().validate()?;
        self.kernel.evaluate(0.0)?;

        if self.descriptor.guarantee() == NonMarkovianGuarantee::Exact {
            if !matches!(self.history.retention(), MemoryRetention::Full)
                && !self.kernel.has_finite_support()
            {
                return Err(
                    NonMarkovianError::ExactEvaluationUnavailable {
                        reason:
                            "exact infinite-memory semantics require full history",
                    },
                );
            }
        }

        Ok(())
    }
}

// ============================================================================
// Process evaluation result
// ============================================================================

/// Result of evaluating a non-Markovian process.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NonMarkovianEvaluation {
    /// Current semantic time.
    current_time: NonMarkovianTime,

    /// Memory contribution.
    contribution: f64,

    /// Number of retained events considered.
    history_events: usize,

    /// Semantic guarantee.
    guarantee: NonMarkovianGuarantee,
}

impl NonMarkovianEvaluation {
    /// Creates an evaluation result.
    fn new(
        current_time: NonMarkovianTime,
        contribution: f64,
        history_events: usize,
        guarantee: NonMarkovianGuarantee,
    ) -> Result<Self, NonMarkovianError> {
        finite(contribution, "memory contribution")?;

        Ok(Self {
            current_time,
            contribution,
            history_events,
            guarantee,
        })
    }

    /// Current time.
    #[must_use]
    pub const fn current_time(self) -> NonMarkovianTime {
        self.current_time
    }

    /// Scalar memory contribution.
    #[must_use]
    pub const fn contribution(self) -> f64 {
        self.contribution
    }

    /// Number of history events used.
    #[must_use]
    pub const fn history_events(self) -> usize {
        self.history_events
    }

    /// Semantic guarantee.
    #[must_use]
    pub const fn guarantee(self) -> NonMarkovianGuarantee {
        self.guarantee
    }
}

// ============================================================================
// Stateful non-Markovian process
// ============================================================================

/// Stateful non-Markovian process combining an explicit kernel with a
/// model-specific environment state.
///
/// The state implementation can represent arbitrarily complex memory without
/// this module knowing whether it is a vector, tensor, auxiliary density
/// operator, process tensor, pseudomode, or another mathematical object.
pub struct StatefulNonMarkovianProcess<S: MemoryState> {
    process: NonMarkovianProcess,
    memory_state: S::State,
    state_model: S,
}

impl<S: MemoryState> StatefulNonMarkovianProcess<S> {
    /// Creates a stateful process.
    pub fn new(
        process: NonMarkovianProcess,
        state_model: S,
    ) -> Result<Self, NonMarkovianError> {
        let memory_state = state_model.initial_state()?;

        Ok(Self {
            process,
            memory_state,
            state_model,
        })
    }

    /// Immutable access to the underlying semantic process.
    #[must_use]
    pub fn process(&self) -> &NonMarkovianProcess {
        &self.process
    }

    /// Immutable access to the model-specific memory state.
    #[must_use]
    pub fn memory_state(&self) -> &S::State {
        &self.memory_state
    }

    /// Records an event and advances memory state.
    pub fn record(
        &mut self,
        event: MemoryEvent,
    ) -> Result<(), NonMarkovianError> {
        self.process.record(event)?;

        let newest = self
            .process
            .history()
            .newest()
            .ok_or(NonMarkovianError::MemoryStateFailure {
                reason: "history unexpectedly empty after recording an event",
            })?;

        let context = MemoryEvaluationContext {
            current_time: newest.time(),
            history: self.process.history(),
        };

        self.memory_state =
            self.state_model.evolve(&self.memory_state, &context)?;

        Ok(())
    }

    /// Evaluates both the scalar kernel convolution and the model-specific
    /// memory state contribution.
    pub fn evaluate(
        &self,
        current_time: NonMarkovianTime,
    ) -> Result<NonMarkovianEvaluation, NonMarkovianError> {
        let convolution = self.process.evaluate(current_time)?;

        let context = MemoryEvaluationContext {
            current_time,
            history: self.process.history(),
        };

        let state_contribution = self
            .state_model
            .contribution(&self.memory_state, &context)?;

        let contribution = convolution + state_contribution;

        NonMarkovianEvaluation::new(
            current_time,
            contribution,
            self.process.history().len(),
            self.process.descriptor().guarantee(),
        )
    }

    /// Clears the history and restores the initial memory state.
    pub fn reset(&mut self) -> Result<(), NonMarkovianError> {
        self.process.clear_history();
        self.memory_state = self.state_model.initial_state()?;
        Ok(())
    }
}

// ============================================================================
// Generic scalar memory state
// ============================================================================

/// A reusable scalar memory-state implementation.
///
/// This is intentionally simple and deterministic. It is useful for models
/// where a scalar auxiliary memory is sufficient.
///
/// More complex physics should implement `MemoryState` directly.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScalarMemoryState {
    /// Relaxation time.
    relaxation_time: f64,

    /// Current state.
    value: f64,
}

impl ScalarMemoryState {
    /// Creates a scalar memory state.
    pub fn new(
        relaxation_time: f64,
        initial_value: f64,
    ) -> Result<Self, NonMarkovianError> {
        validate_positive(relaxation_time, "relaxation time")?;
        finite(initial_value, "initial memory value")?;

        Ok(Self {
            relaxation_time,
            value: initial_value,
        })
    }

    /// Current scalar value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.value
    }
}

impl MemoryState for ScalarMemoryState {
    type State = f64;

    fn initial_state(&self) -> Result<Self::State, NonMarkovianError> {
        finite(self.value, "initial memory value")?;
        Ok(self.value)
    }

    fn evolve(
        &self,
        state: &Self::State,
        context: &MemoryEvaluationContext<'_>,
    ) -> Result<Self::State, NonMarkovianError> {
        finite(*state, "memory state")?;

        let mut accumulated = 0.0;

        for event in context.history.iter() {
            let lag = context.current_time.lag_from(event.time())?;
            let decay = (-lag / self.relaxation_time).exp();

            accumulated += decay * event.weight();
            finite(accumulated, "memory-state accumulation")?;
        }

        Ok(accumulated)
    }

    fn contribution(
        &self,
        state: &Self::State,
        _context: &MemoryEvaluationContext<'_>,
    ) -> Result<f64, NonMarkovianError> {
        finite(*state, "memory-state contribution")
    }

    fn name(&self) -> &'static str {
        "scalar_exponential_memory_state"
    }
}

// ============================================================================
// Resource-scoped process collection
// ============================================================================

/// Resource-scoped non-Markovian processes.
///
/// This is a deliberately simple collection abstraction for integrations that
/// need one memory process per canonical quantum resource.
///
/// It does not assume a fixed number of resources.
#[derive(Debug, Default)]
pub struct ResourceMemoryRegistry {
    entries: Vec<(NonMarkovianResource, NonMarkovianProcess)>,
}

impl ResourceMemoryRegistry {
    /// Creates an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of registered resource processes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true if no resources are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Inserts a process for a resource.
    ///
    /// A resource can have only one active process in this registry. If the
    /// resource already exists, the operation replaces its process.
    pub fn insert(
        &mut self,
        resource: NonMarkovianResource,
        process: NonMarkovianProcess,
    ) -> Result<(), NonMarkovianError> {
        process.validate()?;

        if let Some((_, existing)) = self
            .entries
            .iter_mut()
            .find(|(candidate, _)| *candidate == resource)
        {
            *existing = process;
            return Ok(());
        }

        self.entries.push((resource, process));
        Ok(())
    }

    /// Returns a process by resource.
    #[must_use]
    pub fn get(
        &self,
        resource: NonMarkovianResource,
    ) -> Option<&NonMarkovianProcess> {
        self.entries
            .iter()
            .find(|(candidate, _)| *candidate == resource)
            .map(|(_, process)| process)
    }

    /// Returns a mutable process by resource.
    #[must_use]
    pub fn get_mut(
        &mut self,
        resource: NonMarkovianResource,
    ) -> Option<&mut NonMarkovianProcess> {
        self.entries
            .iter_mut()
            .find(|(candidate, _)| *candidate == resource)
            .map(|(_, process)| process)
    }

    /// Iterates over resources and processes in deterministic insertion order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &(NonMarkovianResource, NonMarkovianProcess)> {
        self.entries.iter()
    }

    /// Removes a resource process.
    pub fn remove(
        &mut self,
        resource: NonMarkovianResource,
    ) -> Option<NonMarkovianProcess> {
        let index = self
            .entries
            .iter()
            .position(|(candidate, _)| *candidate == resource)?;

        Some(self.entries.remove(index).1)
    }
}

// ============================================================================
// Validation helpers
// ============================================================================

/// Validates a process without requiring mutable access.
pub fn validate_process(
    process: &NonMarkovianProcess,
) -> Result<(), NonMarkovianError> {
    process.validate()
}

/// Determines whether a kernel's finite support permits exact history
/// truncation.
///
/// For finite-support kernels, history older than the support contributes
/// exactly zero.
pub fn exact_truncation_available(
    kernel: &dyn MemoryKernel,
) -> bool {
    kernel.has_finite_support()
}

/// Returns the kernel's characteristic time if one exists.
#[must_use]
pub fn characteristic_memory_time(
    kernel: &dyn MemoryKernel,
) -> Option<f64> {
    kernel.characteristic_time()
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_rejects_non_finite_values() {
        assert!(NonMarkovianTime::new(f64::NAN).is_err());
        assert!(NonMarkovianTime::new(f64::INFINITY).is_err());
        assert!(NonMarkovianTime::new(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn time_lag_is_non_negative() {
        let earlier = NonMarkovianTime::new(1.0).expect("valid time");
        let later = NonMarkovianTime::new(3.0).expect("valid time");

        assert_eq!(
            later.lag_from(earlier).expect("valid ordering"),
            2.0
        );

        assert!(earlier.lag_from(later).is_err());
    }

    #[test]
    fn exponential_kernel_is_one_at_zero() {
        let kernel = MemoryKernelKind::Exponential { tau: 2.0 };

        let value = kernel.evaluate(0.0).expect("valid kernel");

        assert_eq!(value.value(), 1.0);
    }

    #[test]
    fn exponential_kernel_decays() {
        let kernel = MemoryKernelKind::Exponential { tau: 1.0 };

        let zero = kernel.evaluate(0.0).expect("valid kernel");
        let later = kernel.evaluate(1.0).expect("valid kernel");

        assert!(later.value() < zero.value());
        assert!(later.value() > 0.0);
    }

    #[test]
    fn compact_kernel_has_exact_finite_support() {
        let kernel = MemoryKernelKind::Compact { support: 2.0 };

        let inside = kernel.evaluate(1.0).expect("valid kernel");
        let outside = kernel.evaluate(3.0).expect("valid kernel");

        assert_eq!(inside.value(), 1.0);
        assert_eq!(outside.value(), 0.0);
        assert!(outside.is_compact_support_zero());
        assert!(kernel.has_finite_support());
    }

    #[test]
    fn sampled_kernel_interpolates() {
        let kernel = SampledMemoryKernel::new(vec![
            (0.0, 1.0),
            (2.0, 0.0),
        ])
        .expect("valid samples");

        let value = kernel.evaluate(1.0).expect("valid evaluation");

        assert!((value.value() - 0.5).abs() < 1.0e-12);
    }

    #[test]
    fn sampled_kernel_requires_ordered_samples() {
        let result = SampledMemoryKernel::new(vec![
            (1.0, 1.0),
            (0.5, 0.0),
        ]);

        assert!(result.is_err());
    }

    #[test]
    fn history_preserves_temporal_order() {
        let mut history =
            MemoryHistory::new(MemoryRetention::Full).expect("valid policy");

        history
            .push(
                MemoryEvent::new(
                    NonMarkovianTime::new(1.0).expect("valid time"),
                    1.0,
                )
                .expect("valid event"),
            )
            .expect("valid insertion");

        history
            .push(
                MemoryEvent::new(
                    NonMarkovianTime::new(2.0).expect("valid time"),
                    2.0,
                )
                .expect("valid event"),
            )
            .expect("valid insertion");

        assert_eq!(history.len(), 2);
        assert_eq!(
            history.oldest().expect("oldest").weight(),
            1.0
        );
        assert_eq!(
            history.newest().expect("newest").weight(),
            2.0
        );
    }

    #[test]
    fn history_rejects_reverse_time() {
        let mut history =
            MemoryHistory::new(MemoryRetention::Full).expect("valid policy");

        history
            .push(
                MemoryEvent::new(
                    NonMarkovianTime::new(2.0).expect("valid time"),
                    1.0,
                )
                .expect("valid event"),
            )
            .expect("valid insertion");

        let result = history.push(
            MemoryEvent::new(
                NonMarkovianTime::new(1.0).expect("valid time"),
                1.0,
            )
            .expect("valid event"),
        );

        assert!(result.is_err());
    }

    #[test]
    fn count_retention_is_explicit() {
        let mut history =
            MemoryHistory::new(MemoryRetention::Count(2))
                .expect("valid policy");

        for time in 0..4 {
            history
                .push(
                    MemoryEvent::new(
                        NonMarkovianTime::new(time as f64)
                            .expect("valid time"),
                        1.0,
                    )
                    .expect("valid event"),
                )
                .expect("valid insertion");
        }

        assert_eq!(history.len(), 2);
        assert_eq!(
            history.oldest().expect("oldest").time().seconds(),
            2.0
        );
    }

    #[test]
    fn duration_retention_is_explicit() {
        let mut history =
            MemoryHistory::new(MemoryRetention::Duration(1.0))
                .expect("valid policy");

        for time in 0..4 {
            history
                .push(
                    MemoryEvent::new(
                        NonMarkovianTime::new(time as f64)
                            .expect("valid time"),
                        1.0,
                    )
                    .expect("valid event"),
                )
                .expect("valid insertion");
        }

        assert_eq!(history.len(), 2);
        assert_eq!(
            history.oldest().expect("oldest").time().seconds(),
            2.0
        );
    }

    #[test]
    fn convolution_uses_history() {
        let kernel =
            MemoryKernelKind::Exponential { tau: 1.0 };

        let mut history =
            MemoryHistory::new(MemoryRetention::Full)
                .expect("valid policy");

        history
            .push(
                MemoryEvent::new(
                    NonMarkovianTime::zero(),
                    1.0,
                )
                .expect("valid event"),
            )
            .expect("valid insertion");

        let now =
            NonMarkovianTime::new(1.0).expect("valid time");

        let value =
            convolve_history(&kernel, now, &history)
                .expect("valid convolution");

        assert!((value - (-1.0f64).exp()).abs() < 1.0e-12);
    }

    #[test]
    fn exact_infinite_memory_requires_full_history() {
        let process = NonMarkovianProcess::from_kernel(
            "power-law",
            (1, 0, 0),
            Box::new(MemoryKernelKind::PowerLaw {
                tau: 1.0,
                alpha: 1.0,
            }),
            MemoryRetention::Count(16),
            NonMarkovianGuarantee::Exact,
        )
        .expect("construction itself is valid");

        assert!(process.validate().is_err());
    }

    #[test]
    fn finite_support_can_be_exact_with_bounded_history() {
        let process = NonMarkovianProcess::from_kernel(
            "compact",
            (1, 0, 0),
            Box::new(MemoryKernelKind::Compact { support: 1.0 }),
            MemoryRetention::Duration(1.0),
            NonMarkovianGuarantee::Exact,
        )
        .expect("valid process");

        assert!(process.validate().is_ok());
    }

    #[test]
    fn composite_kernel_is_deterministic() {
        let mut kernel = CompositeMemoryKernel::new();

        kernel
            .push(
                0.5,
                Box::new(MemoryKernelKind::Exponential {
                    tau: 1.0,
                }),
            )
            .expect("valid term");

        kernel
            .push(
                0.5,
                Box::new(MemoryKernelKind::Gaussian {
                    tau: 2.0,
                }),
            )
            .expect("valid term");

        let first = kernel.evaluate(1.5).expect("valid evaluation");
        let second = kernel.evaluate(1.5).expect("valid evaluation");

        assert_eq!(first, second);
    }

    #[test]
    fn process_is_reusable() {
        let mut process = NonMarkovianProcess::from_kernel(
            "test",
            (1, 0, 0),
            Box::new(MemoryKernelKind::Exponential {
                tau: 1.0,
            }),
            MemoryRetention::Full,
            NonMarkovianGuarantee::Exact,
        )
        .expect("valid process");

        process
            .record(
                MemoryEvent::new(
                    NonMarkovianTime::zero(),
                    1.0,
                )
                .expect("valid event"),
            )
            .expect("record succeeds");

        let result = process
            .evaluate(
                NonMarkovianTime::new(1.0)
                    .expect("valid time"),
            )
            .expect("evaluation succeeds");

        assert!(result > 0.0);
    }

    #[test]
    fn resource_registry_has_no_fixed_resource_limit() {
        let mut registry = ResourceMemoryRegistry::new();

        let process = NonMarkovianProcess::from_kernel(
            "resource-memory",
            (1, 0, 0),
            Box::new(MemoryKernelKind::Exponential {
                tau: 1.0,
            }),
            MemoryRetention::Full,
            NonMarkovianGuarantee::Exact,
        )
        .expect("valid process");

        let resource = NonMarkovianResource::LogicalQubit(
            // The constructor is deliberately delegated to the canonical
            // Quantum IR type. This test therefore does not invent a ZQN
            // qubit identity.
            //
            // Replace this construction with the canonical QubitId
            // constructor used by the current IR implementation when this
            // test is integrated into the repository.
            //
            // The production module itself contains no numeric qubit
            // assumptions.
            QubitId::new(0),
        );

        registry
            .insert(resource, process)
            .expect("insert succeeds");

        assert_eq!(registry.len(), 1);
    }
}