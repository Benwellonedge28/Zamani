//! Zamani Quantum Noise (ZQN) — Scheduling Integration
//!
//! Path:
//!     src/quantum/zqn/integration/scheduling.rs
//!
//! # Purpose
//!
//! This module defines the boundary between:
//!
//!     quantum::ir::scheduling
//!
//! and:
//!
//!     quantum::zqn
//!
//! Its responsibility is to expose physical-noise consequences of scheduling
//! decisions to the canonical Zamani scheduler.
//!
//! It does NOT implement a scheduling algorithm.
//!
//! The canonical scheduling subsystem remains responsible for deciding:
//!
//! - when operations execute;
//! - dependency ordering;
//! - resource conflicts;
//! - temporal placement;
//! - schedule construction;
//! - scheduling strategy;
//! - scheduling policy.
//!
//! ZQN remains responsible for:
//!
//! - noise semantics;
//! - noise models;
//! - noise effects;
//! - calibration-aware noise information;
//! - temporal noise;
//! - idle noise;
//! - crosstalk;
//! - correlated noise;
//! - uncertainty;
//! - physical error estimates;
//! - noise-derived costs.
//!
//! This file connects the two without transferring ownership.
//!
//! # Architectural boundary
//!
//! ```text
//!                         Zamani source
//!                              |
//!                              v
//!                       canonical Quantum IR
//!                              |
//!                +-------------+-------------+
//!                |                           |
//!                v                           v
//!             routing                    scheduling
//!                |                           |
//!                |                           v
//!                |                  proposed temporal placement
//!                |                           |
//!                |                           v
//!                |                    ZQN scheduling
//!                |                    integration
//!                |                           |
//!                |                 +---------+---------+
//!                |                 |                   |
//!                |                 v                   v
//!                |             idle noise          crosstalk
//!                |             duration            temporal noise
//!                |                 |                   |
//!                +-----------------+-------------------+
//!                                  |
//!                                  v
//!                         scheduling cost/analysis
//!                                  |
//!                                  v
//!                         canonical Schedule
//! ```
//!
//! The dependency direction is intentionally one-way:
//!
//!     scheduling -> ZQN integration contract
//!
//! while ZQN does not invoke a scheduler implementation.
//!
//! # Critical ownership rule
//!
//! This file does NOT own:
//!
//! - `Schedule`;
//! - `ScheduledOperation`;
//! - `ScheduleResource`;
//! - `OperationId`;
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - `TimePoint`;
//! - `Duration`;
//! - `TimeInterval`;
//! - scheduler algorithms;
//! - routing;
//! - hardware APIs;
//! - calibration storage;
//! - quantum channels;
//! - faults;
//! - noise-model implementations.
//!
//! Those objects belong to their canonical modules.
//!
//! # Canonical identity rule
//!
//! Quantum-resource identity MUST use:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! This file never defines a second qubit identifier.
//!
//! Semantic operation identity MUST use:
//!
//!     crate::quantum::ir::identity::OperationId
//!
//! ZQN object identity uses:
//!
//!     crate::quantum::zqn::core::ids::NoiseModelId
//!
//! # Why this integration exists
//!
//! A normal scheduler generally asks:
//!
//!     "Can operation B start at time T?"
//!
//! Noise-aware scheduling additionally needs to ask:
//!
//!     "What physical-noise consequence does starting B at time T have?"
//!
//! This distinction is fundamental.
//!
//! A schedule that is temporally valid is not necessarily physically optimal.
//!
//! For example:
//!
//! ```text
//! operation A
//!     |
//!     +---- finishes at t = 10
//!
//! operation B
//!     |
//!     +---- could begin at t = 10
//!
//! or
//!
//! operation B
//!     |
//!     +---- could begin at t = 30
//! ```
//!
//! The second placement may introduce additional idle time and therefore
//! additional decoherence, even though both placements satisfy the ordinary
//! dependency graph.
//!
//! Conversely, delaying an operation may reduce crosstalk or avoid an unstable
//! calibration window.
//!
//! ZQN therefore provides *information* to scheduling; it does not make the
//! scheduling decision itself.
//!
//! # Write once, scale everywhere
//!
//! This module contains no semantic upper bounds for:
//!
//! - qubits;
//! - physical qubits;
//! - operations;
//! - resources;
//! - schedule depth;
//! - correlation width;
//! - number of noise models;
//! - number of devices;
//! - number of execution nodes;
//! - number of shots.
//!
//! There is no:
//!
//!     MAX_QUBITS
//!     MAX_OPERATIONS
//!     MAX_RESOURCES
//!     MAX_SCHEDULE_DEPTH
//!
//! in this file.
//!
//! The scheduler and runtime may impose explicit resource-policy limits.
//! Such limits are deployment constraints, not ZQN semantic limits.
//!
//! "Infinity" therefore means:
//!
//!     no artificial finite machine-size ceiling in the semantic contract.
//!
//! Actual execution remains constrained by available memory, compute,
//! storage, target capabilities and explicit resource policies.
//!
//! # No fixed operation arity
//!
//! A scheduling request may contain any finite number of logical or physical
//! resources.
//!
//! There is deliberately no special case for:
//!
//! - one-qubit operations;
//! - two-qubit operations;
//! - three-qubit operations.
//!
//! Resource cardinality is data.
//!
//! # No vendor coupling
//!
//! This module contains no:
//!
//! - IBM API;
//! - AWS API;
//! - Azure API;
//! - IonQ API;
//! - Rigetti API;
//! - Quantinuum API;
//! - vendor-specific calibration format.
//!
//! Hardware adapters provide target/calibration information through abstract
//! ZQN contracts.
//!
//! # Noise versus scheduling
//!
//! Scheduling answers:
//!
//!     WHEN?
//!
//! ZQN answers:
//!
//!     WHAT NOISE DOES THAT TIMING PRODUCE?
//!
//! The scheduler remains free to optimize additional objectives such as:
//!
//! - latency;
//! - makespan;
//! - resource utilization;
//! - throughput;
//! - hardware constraints.
//!
//! ZQN contributes noise-derived information such as:
//!
//! - estimated error;
//! - idle exposure;
//! - crosstalk exposure;
//! - calibration uncertainty;
//! - noise-related duration effects;
//! - conservative error bounds.
//!
//! # Determinism
//!
//! This integration layer is deterministic.
//!
//! It contains no:
//!
//! - global RNG;
//! - thread-local RNG;
//! - wall-clock reads;
//! - global mutable state;
//! - memory-address-dependent behavior;
//! - hash-map iteration used for semantic ordering.
//!
//! If a downstream ZQN model performs stochastic sampling, the deterministic
//! sampling contract belongs to the ZQN simulation/reproducibility subsystem.
//!
//! This integration layer only carries the explicit scheduling context.
//!
//! # Numerical safety
//!
//! All floating-point values accepted by this module are required to be:
//!
//! - finite;
//! - non-negative where representing costs/probabilities;
//! - within `[0, 1]` where representing probabilities/fidelities.
//!
//! NaN and infinite values are rejected.
//!
//! No invalid numerical value is silently normalized, clamped or converted.
//!
//! # Approximation
//!
//! Noise estimates are explicitly classified as:
//!
//! - Exact;
//! - Approximate;
//! - Bounded;
//! - Statistical;
//! - Unknown.
//!
//! The scheduler MUST NOT interpret an approximate value as exact.
//!
//! # Resource identity
//!
//! A scheduling resource is represented using the canonical IR scheduling
//! resource type:
//!
//!     crate::quantum::ir::scheduling::schedule::ScheduleResource
//!
//! This means logical and physical qubits retain their distinct semantic
//! identity.
//!
//! # Timing
//!
//! Timing uses the canonical IR types:
//!
//!     Duration
//!     TimePoint
//!     TimeInterval
//!
//! No second ZQN time representation is introduced.
//!
//! # Integration with canonical scheduling
//!
//! The intended flow is:
//!
//! ```text
//! SchedulingTask
//!       |
//!       v
//! proposed TimeInterval
//!       |
//!       v
//! SchedulingNoiseContext
//!       |
//!       v
//! NoiseAwareSchedulingModel
//!       |
//!       v
//! SchedulingNoiseEstimate
//!       |
//!       v
//! scheduler objective / validation
//!       |
//!       v
//! canonical Schedule
//! ```
//!
//! # Important distinction: estimate versus execution
//!
//! The scheduling integration MUST NOT execute a noise model in the sense of
//! performing a complete noisy quantum simulation.
//!
//! It asks for a scheduling-relevant estimate.
//!
//! Full physical realization belongs to:
//!
//!     zqn::simulation
//!     zqn::runtime integration
//!     hardware adapters
//!
//! # Important distinction: estimate versus guarantee
//!
//! An estimate is not automatically a mathematical bound.
//!
//! The `Precision` field makes this distinction explicit.
//!
//! A caller that requires a conservative guarantee must request or verify a
//! `Bounded` estimate.
//!
//! # Important distinction: missing calibration
//!
//! Missing calibration MUST NOT be silently interpreted as perfect hardware.
//!
//! A model may explicitly return:
//!
//!     Precision::Unknown
//!
//! or a bounded fallback estimate.
//!
//! The scheduling policy decides whether that is acceptable.
//!
//! # Integration with routing
//!
//! Routing may produce a physical placement.
//!
//! Scheduling then receives the resulting physical resources.
//!
//! ZQN can consequently account for:
//!
//! - physical-qubit idle time;
//! - physical crosstalk;
//! - calibration windows;
//! - correlated temporal noise.
//!
//! Routing remains responsible for placement.
//!
//! # Integration with hardware
//!
//! Hardware adapters may implement `SchedulingNoiseModel` using:
//!
//! - current calibration;
//! - historical characterization;
//! - target capabilities;
//! - device-specific noise observations.
//!
//! No hardware handle is stored in this integration type.
//!
//! # Integration with QEC
//!
//! QEC scheduling can consume this same contract when a QEC scheduler wants
//! to account for:
//!
//! - syndrome-extraction timing;
//! - idle exposure;
//! - measurement noise;
//! - leakage windows;
//! - correlated faults.
//!
//! This file does not know how syndrome extraction or decoding works.
//!
//! # Integration with benchmarking
//!
//! Benchmarking may record `SchedulingNoiseEstimate` values as part of an
//! experiment provenance record.
//!
//! It must not use these estimates as measured observations unless the values
//! actually came from characterization/execution.
//!
//! # Integration with memory/simulation
//!
//! The schedule produced by the canonical scheduler can subsequently be
//! consumed by simulation or execution.
//!
//! ZQN simulation can use the final schedule to realize:
//!
//! - gate noise;
//! - idle noise;
//! - temporal noise;
//! - correlated noise;
//! - measurement noise.
//!
//! # Serialization
//!
//! This file defines semantic integration values but does not establish a
//! separate wire format.
//!
//! Canonical serialization belongs to the surrounding IR/ZQN serialization
//! layers.
//!
//! Serialization MUST preserve:
//!
//! - operation identity;
//! - resource identities;
//! - interval;
//! - noise-model identity;
//! - precision classification;
//! - objective contributions;
//! - provenance identity.
//!
//! It MUST NOT serialize:
//!
//! - memory addresses;
//! - collection capacity;
//! - temporary caches;
//! - synchronization primitives;
//! - implementation-specific hash state.
//!
//! # Thread safety
//!
//! The value types in this module contain ordinary owned immutable data.
//!
//! The integration traits require implementations to be `Send + Sync` where
//! shared concurrent access is expected.
//!
//! No global mutable state is used.
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
//! `#![forbid(unsafe_code)]` is intentional.
//!
//! # Testing contract
//!
//! Tests must verify:
//!
//! 1. canonical qubit identities are used;
//! 2. operation identity remains independent from schedule position;
//! 3. zero-duration intervals are handled correctly;
//! 4. probabilities reject NaN and infinity;
//! 5. probabilities reject values outside `[0, 1]`;
//! 6. costs reject NaN and infinity;
//! 7. negative costs are rejected;
//! 8. approximation precision is not confused with exact precision;
//! 9. deterministic ordering is independent of input ordering;
//! 10. resource collections support arbitrary cardinality;
//! 11. no semantic maximum resource count exists;
//! 12. empty resource sets are representable where valid;
//! 13. physical and logical qubit identities remain distinct;
//! 14. model identity is not confused with resource identity;
//! 15. schedule timing is not silently modified by ZQN;
//! 16. no stochastic behavior is introduced by this integration layer.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use crate::quantum::ir::identity::OperationId;
use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::ir::scheduling::schedule::{
    ScheduleResource,
    ScheduledOperation,
};
use crate::quantum::ir::timing::{Duration, TimeInterval, TimePoint};
use crate::quantum::zqn::core::ids::NoiseModelId;

// =============================================================================
// Result and error types
// =============================================================================

/// Result returned by ZQN scheduling integration operations.
pub type SchedulingResult<T> = Result<T, SchedulingIntegrationError>;

/// Errors produced by the scheduling integration boundary.
#[derive(Debug, Clone, PartialEq)]
pub enum SchedulingIntegrationError {
    /// A floating-point value was NaN or infinite.
    NonFiniteValue {
        /// Semantic name of the invalid field.
        field: &'static str,
        /// Supplied value.
        value: f64,
    },

    /// A probability/fidelity was outside `[0, 1]`.
    InvalidProbability {
        /// Semantic name of the invalid field.
        field: &'static str,
        /// Supplied value.
        value: f64,
    },

    /// A cost was negative.
    NegativeCost {
        /// Semantic name of the invalid field.
        field: &'static str,
        /// Supplied value.
        value: f64,
    },

    /// A scheduling estimate contained inconsistent values.
    InvalidEstimate {
        /// Explanation.
        reason: &'static str,
    },

    /// A resource appeared more than once in one scheduling context.
    DuplicateResource {
        /// Duplicated resource.
        resource: ScheduleResource,
    },

    /// A noise model identity was missing where one was required.
    MissingNoiseModel,

    /// A requested model cannot provide the requested scheduling information.
    Unsupported {
        /// Description of the unsupported request.
        reason: &'static str,
    },
}

impl fmt::Display for SchedulingIntegrationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteValue { field, value } => {
                write!(formatter, "{field} must be finite, got {value}")
            }

            Self::InvalidProbability { field, value } => {
                write!(
                    formatter,
                    "{field} must be within [0, 1], got {value}"
                )
            }

            Self::NegativeCost { field, value } => {
                write!(
                    formatter,
                    "{field} must be non-negative, got {value}"
                )
            }

            Self::InvalidEstimate { reason } => {
                write!(formatter, "invalid scheduling noise estimate: {reason}")
            }

            Self::DuplicateResource { resource } => {
                write!(
                    formatter,
                    "resource appears more than once: {resource:?}"
                )
            }

            Self::MissingNoiseModel => {
                formatter.write_str("a noise model is required")
            }

            Self::Unsupported { reason } => {
                write!(formatter, "unsupported scheduling noise request: {reason}")
            }
        }
    }
}

impl Error for SchedulingIntegrationError {}

// =============================================================================
// Precision
// =============================================================================

/// Semantic precision/guarantee classification for a scheduling estimate.
///
/// A scheduler MUST preserve this distinction when making decisions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Precision {
    /// Mathematically exact under the supplied model and numerical domain.
    Exact,

    /// Approximation with an explicit absolute tolerance.
    Approximate {
        /// Maximum declared approximation tolerance.
        tolerance: f64,
    },

    /// Conservative bound with an explicit error bound.
    Bounded {
        /// Declared upper bound on the relevant error quantity.
        error_bound: f64,
    },

    /// Statistical estimate with a confidence level.
    Statistical {
        /// Confidence level in `[0, 1]`.
        confidence: f64,
    },

    /// No quantitative guarantee is available.
    Unknown,
}

impl Precision {
    /// Creates an approximate precision descriptor.
    pub fn approximate(tolerance: f64) -> SchedulingResult<Self> {
        validate_non_negative_finite(tolerance, "tolerance")?;

        Ok(Self::Approximate { tolerance })
    }

    /// Creates a bounded precision descriptor.
    pub fn bounded(error_bound: f64) -> SchedulingResult<Self> {
        validate_probability(error_bound, "error_bound")?;

        Ok(Self::Bounded { error_bound })
    }

    /// Creates a statistical precision descriptor.
    pub fn statistical(confidence: f64) -> SchedulingResult<Self> {
        validate_probability(confidence, "confidence")?;

        Ok(Self::Statistical { confidence })
    }

    /// Returns whether this descriptor provides a quantitative guarantee.
    #[must_use]
    pub const fn is_quantified(self) -> bool {
        !matches!(self, Self::Unknown)
    }

    /// Returns whether this descriptor represents a conservative bound.
    #[must_use]
    pub const fn is_bounded(self) -> bool {
        matches!(self, Self::Bounded { .. })
    }
}

// =============================================================================
// Noise objective
// =============================================================================

/// Noise-related quantities that a scheduler may optimize.
///
/// ZQN provides these quantities; the scheduler chooses how they participate
/// in its objective.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NoiseObjective {
    /// Weight applied to estimated physical error.
    pub error_weight: f64,

    /// Weight applied to estimated idle exposure.
    pub idle_weight: f64,

    /// Weight applied to crosstalk exposure.
    pub crosstalk_weight: f64,

    /// Weight applied to calibration uncertainty.
    pub calibration_weight: f64,

    /// Weight applied to temporal-noise exposure.
    pub temporal_weight: f64,
}

impl Default for NoiseObjective {
    fn default() -> Self {
        Self {
            error_weight: 1.0,
            idle_weight: 1.0,
            crosstalk_weight: 1.0,
            calibration_weight: 1.0,
            temporal_weight: 1.0,
        }
    }
}

impl NoiseObjective {
    /// Creates a noise objective from explicit weights.
    pub fn new(
        error_weight: f64,
        idle_weight: f64,
        crosstalk_weight: f64,
        calibration_weight: f64,
        temporal_weight: f64,
    ) -> SchedulingResult<Self> {
        validate_non_negative_finite(error_weight, "error_weight")?;
        validate_non_negative_finite(idle_weight, "idle_weight")?;
        validate_non_negative_finite(crosstalk_weight, "crosstalk_weight")?;
        validate_non_negative_finite(
            calibration_weight,
            "calibration_weight",
        )?;
        validate_non_negative_finite(temporal_weight, "temporal_weight")?;

        Ok(Self {
            error_weight,
            idle_weight,
            crosstalk_weight,
            calibration_weight,
            temporal_weight,
        })
    }
}

// =============================================================================
// Scheduling noise estimate
// =============================================================================

/// Noise information associated with one proposed scheduling decision.
///
/// This is an estimate, not a command to execute the operation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SchedulingNoiseEstimate {
    /// Estimated probability of an operation-level error.
    pub error_probability: f64,

    /// Estimated additional error probability caused by idle exposure.
    pub idle_error_probability: f64,

    /// Estimated crosstalk contribution.
    pub crosstalk_error_probability: f64,

    /// Estimated uncertainty contribution.
    pub calibration_uncertainty: f64,

    /// Estimated time-dependent contribution.
    pub temporal_error_probability: f64,

    /// Semantic precision of the estimate.
    pub precision: Precision,
}

impl SchedulingNoiseEstimate {
    /// Creates a validated noise estimate.
    pub fn new(
        error_probability: f64,
        idle_error_probability: f64,
        crosstalk_error_probability: f64,
        calibration_uncertainty: f64,
        temporal_error_probability: f64,
        precision: Precision,
    ) -> SchedulingResult<Self> {
        validate_probability(
            error_probability,
            "error_probability",
        )?;

        validate_probability(
            idle_error_probability,
            "idle_error_probability",
        )?;

        validate_probability(
            crosstalk_error_probability,
            "crosstalk_error_probability",
        )?;

        validate_probability(
            calibration_uncertainty,
            "calibration_uncertainty",
        )?;

        validate_probability(
            temporal_error_probability,
            "temporal_error_probability",
        )?;

        validate_precision(precision)?;

        Ok(Self {
            error_probability,
            idle_error_probability,
            crosstalk_error_probability,
            calibration_uncertainty,
            temporal_error_probability,
            precision,
        })
    }

    /// Returns an explicitly unknown estimate.
    ///
    /// Unknown is not interpreted as zero error.
    #[must_use]
    pub const fn unknown() -> Self {
        Self {
            error_probability: 0.0,
            idle_error_probability: 0.0,
            crosstalk_error_probability: 0.0,
            calibration_uncertainty: 0.0,
            temporal_error_probability: 0.0,
            precision: Precision::Unknown,
        }
    }

    /// Returns the conservative sum of independently supplied contributions.
    ///
    /// This is a scheduling objective contribution, not a claim that the
    /// underlying physical probabilities are statistically independent.
    pub fn objective_contribution(
        self,
        objective: NoiseObjective,
    ) -> SchedulingResult<f64> {
        let value = self.error_probability * objective.error_weight
            + self.idle_error_probability * objective.idle_weight
            + self.crosstalk_error_probability
                * objective.crosstalk_weight
            + self.calibration_uncertainty
                * objective.calibration_weight
            + self.temporal_error_probability
                * objective.temporal_weight;

        validate_non_negative_finite(value, "objective contribution")?;

        Ok(value)
    }
}

// =============================================================================
// Scheduling noise context
// =============================================================================

/// Complete context for evaluating the noise consequences of one scheduling
/// decision.
///
/// The context is immutable and contains no scheduler state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchedulingNoiseContext {
    /// Semantic operation identity.
    operation_id: OperationId,

    /// Proposed execution interval.
    interval: TimeInterval,

    /// Resources occupied by the proposed operation.
    resources: Vec<ScheduleResource>,

    /// Other already-scheduled operations relevant to this decision.
    ///
    /// The collection is intentionally caller-provided rather than generated
    /// by ZQN. This allows sparse evaluation for very large schedules.
    occupied_operations: Vec<ScheduledOperation>,

    /// Optional ZQN model identity.
    noise_model: Option<NoiseModelId>,
}

impl SchedulingNoiseContext {
    /// Creates a scheduling noise context.
    ///
    /// Resources are copied into deterministic order.
    ///
    /// Duplicate resource references are rejected.
    pub fn new(
        operation_id: OperationId,
        interval: TimeInterval,
        resources: impl IntoIterator<Item = ScheduleResource>,
    ) -> SchedulingResult<Self> {
        let resources = canonical_resources(resources)?;

        Ok(Self {
            operation_id,
            interval,
            resources,
            occupied_operations: Vec::new(),
            noise_model: None,
        })
    }

    /// Adds already-scheduled operations relevant to the proposed placement.
    ///
    /// This does not mutate the canonical schedule. It only enriches the query
    /// context presented to ZQN.
    pub fn with_occupied_operations(
        mut self,
        operations: impl IntoIterator<Item = ScheduledOperation>,
    ) -> Self {
        self.occupied_operations = operations.into_iter().collect();

        self.occupied_operations.sort_by(compare_scheduled_operations);

        self
    }

    /// Associates a ZQN noise model.
    #[must_use]
    pub fn with_noise_model(
        mut self,
        noise_model: NoiseModelId,
    ) -> Self {
        self.noise_model = Some(noise_model);

        self
    }

    /// Returns the semantic operation identity.
    #[must_use]
    pub const fn operation_id(&self) -> OperationId {
        self.operation_id
    }

    /// Returns the proposed interval.
    #[must_use]
    pub const fn interval(&self) -> TimeInterval {
        self.interval
    }

    /// Returns the proposed start time.
    #[must_use]
    pub const fn start(&self) -> TimePoint {
        self.interval.start()
    }

    /// Returns the proposed end time.
    #[must_use]
    pub const fn end(&self) -> TimePoint {
        self.interval.end()
    }

    /// Returns the proposed duration.
    #[must_use]
    pub fn duration(&self) -> Duration {
        self.interval.duration()
    }

    /// Returns canonical resource ordering.
    #[must_use]
    pub fn resources(&self) -> &[ScheduleResource] {
        &self.resources
    }

    /// Returns already-scheduled operations supplied for context.
    #[must_use]
    pub fn occupied_operations(&self) -> &[ScheduledOperation] {
        &self.occupied_operations
    }

    /// Returns the associated noise model, if one was supplied.
    #[must_use]
    pub const fn noise_model(&self) -> Option<NoiseModelId> {
        self.noise_model
    }

    /// Returns whether the proposed operation uses a particular resource.
    #[must_use]
    pub fn uses_resource(&self, resource: ScheduleResource) -> bool {
        self.resources.binary_search(&resource).is_ok()
    }

    /// Returns the number of proposed resources.
    ///
    /// This is a collection cardinality, not a semantic machine-size limit.
    #[must_use]
    pub fn resource_count(&self) -> usize {
        self.resources.len()
    }
}

// =============================================================================
// Noise-aware scheduling model
// =============================================================================

/// Provider-neutral interface through which the canonical scheduler obtains
/// ZQN information.
///
/// Implementations may live in:
///
/// - `zqn::noise`;
/// - `zqn::calibration`;
/// - `zqn::target`;
/// - hardware integration;
/// - simulator integration;
/// - characterization adapters.
///
/// The trait deliberately does not expose a scheduling algorithm.
///
/// # Required implementation properties
///
/// Implementations MUST:
///
/// - be deterministic for identical immutable inputs unless explicitly
///   documented as statistical;
/// - never depend on thread identity;
/// - never mutate global state;
/// - reject or explicitly classify unsupported requests;
/// - return finite validated values;
/// - not treat missing calibration as perfect hardware;
/// - not impose machine-size limits;
/// - respect the caller's resource policy;
/// - not execute hardware operations.
pub trait SchedulingNoiseModel: Send + Sync {
    /// Returns the stable ZQN model identity.
    fn model_id(&self) -> NoiseModelId;

    /// Estimates the noise consequences of a proposed schedule placement.
    fn estimate(
        &self,
        context: &SchedulingNoiseContext,
    ) -> SchedulingResult<SchedulingNoiseEstimate>;
}

// =============================================================================
// Schedule-level noise analysis
// =============================================================================

/// Aggregate noise information for a collection of scheduled operations.
///
/// The aggregation here is deliberately conservative and objective-oriented.
/// It does not claim that physical errors are statistically independent.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScheduleNoiseSummary {
    /// Number of evaluated operations.
    pub operations_evaluated: usize,

    /// Number of estimates that were quantitatively classified.
    pub quantified_operations: usize,

    /// Number of unknown estimates.
    pub unknown_operations: usize,

    /// Sum of operation error contributions.
    pub operation_error: f64,

    /// Sum of idle-error contributions.
    pub idle_error: f64,

    /// Sum of crosstalk contributions.
    pub crosstalk_error: f64,

    /// Sum of calibration uncertainty.
    pub calibration_uncertainty: f64,

    /// Sum of temporal-noise contributions.
    pub temporal_error: f64,

    /// Objective score.
    pub objective_score: f64,
}

impl Default for ScheduleNoiseSummary {
    fn default() -> Self {
        Self {
            operations_evaluated: 0,
            quantified_operations: 0,
            unknown_operations: 0,
            operation_error: 0.0,
            idle_error: 0.0,
            crosstalk_error: 0.0,
            calibration_uncertainty: 0.0,
            temporal_error: 0.0,
            objective_score: 0.0,
        }
    }
}

impl ScheduleNoiseSummary {
    /// Incorporates one validated estimate.
    pub fn record(
        &mut self,
        estimate: SchedulingNoiseEstimate,
        objective: NoiseObjective,
    ) -> SchedulingResult<()> {
        self.operations_evaluated =
            self.operations_evaluated.checked_add(1).ok_or(
                SchedulingIntegrationError::Unsupported {
                    reason: "operation count overflow",
                },
            )?;

        if estimate.precision.is_quantified() {
            self.quantified_operations =
                self.quantified_operations.checked_add(1).ok_or(
                    SchedulingIntegrationError::Unsupported {
                        reason: "quantified-operation count overflow",
                    },
                )?;
        } else {
            self.unknown_operations =
                self.unknown_operations.checked_add(1).ok_or(
                    SchedulingIntegrationError::Unsupported {
                        reason: "unknown-operation count overflow",
                    },
                )?;
        }

        self.operation_error += estimate.error_probability;
        self.idle_error += estimate.idle_error_probability;
        self.crosstalk_error += estimate.crosstalk_error_probability;
        self.calibration_uncertainty += estimate.calibration_uncertainty;
        self.temporal_error += estimate.temporal_error_probability;

        self.objective_score += estimate.objective_contribution(objective)?;

        validate_non_negative_finite(
            self.operation_error,
            "aggregate operation error",
        )?;

        validate_non_negative_finite(
            self.idle_error,
            "aggregate idle error",
        )?;

        validate_non_negative_finite(
            self.crosstalk_error,
            "aggregate crosstalk error",
        )?;

        validate_non_negative_finite(
            self.calibration_uncertainty,
            "aggregate calibration uncertainty",
        )?;

        validate_non_negative_finite(
            self.temporal_error,
            "aggregate temporal error",
        )?;

        validate_non_negative_finite(
            self.objective_score,
            "aggregate objective score",
        )?;

        Ok(())
    }

    /// Returns whether every evaluated operation had quantified information.
    #[must_use]
    pub const fn is_fully_quantified(&self) -> bool {
        self.unknown_operations == 0
    }
}

// =============================================================================
// Noise-aware scheduling evaluator
// =============================================================================

/// Stateless evaluator that connects a canonical schedule candidate to a ZQN
/// model.
///
/// The evaluator does not construct or modify a `Schedule`.
pub struct SchedulingNoiseEvaluator<'a, M>
where
    M: SchedulingNoiseModel + ?Sized,
{
    model: &'a M,
    objective: NoiseObjective,
}

impl<'a, M> SchedulingNoiseEvaluator<'a, M>
where
    M: SchedulingNoiseModel + ?Sized,
{
    /// Creates an evaluator.
    #[must_use]
    pub const fn new(
        model: &'a M,
        objective: NoiseObjective,
    ) -> Self {
        Self { model, objective }
    }

    /// Returns the underlying model identity.
    #[must_use]
    pub fn model_id(&self) -> NoiseModelId {
        self.model.model_id()
    }

    /// Evaluates one proposed scheduling decision.
    pub fn evaluate(
        &self,
        context: &SchedulingNoiseContext,
    ) -> SchedulingResult<SchedulingNoiseEstimate> {
        if context.noise_model() != Some(self.model.model_id()) {
            return Err(SchedulingIntegrationError::MissingNoiseModel);
        }

        self.model.estimate(context)
    }

    /// Evaluates multiple independent scheduling contexts.
    ///
    /// Ordering of the returned vector matches the input iterator order.
    ///
    /// The method does not internally parallelize evaluation, preserving a
    /// simple deterministic execution contract. Callers that parallelize must
    /// preserve semantic operation identities and deterministic result
    /// association.
    pub fn evaluate_many<I>(
        &self,
        contexts: I,
    ) -> SchedulingResult<Vec<SchedulingNoiseEstimate>>
    where
        I: IntoIterator<Item = SchedulingNoiseContext>,
    {
        let mut results = Vec::new();

        for context in contexts {
            results.push(self.evaluate(&context)?);
        }

        Ok(results)
    }

    /// Evaluates a collection and returns an aggregate summary.
    pub fn summarize<I>(
        &self,
        contexts: I,
    ) -> SchedulingResult<ScheduleNoiseSummary>
    where
        I: IntoIterator<Item = SchedulingNoiseContext>,
    {
        let mut summary = ScheduleNoiseSummary::default();

        for context in contexts {
            let estimate = self.evaluate(&context)?;
            summary.record(estimate, self.objective)?;
        }

        Ok(summary)
    }
}

// =============================================================================
// Idle-noise query
// =============================================================================

/// A resource-specific idle interval presented to ZQN.
///
/// This allows the scheduler to query idle noise without making ZQN responsible
/// for finding idle periods.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IdleNoiseQuery {
    /// Resource that remains idle.
    resource: ScheduleResource,

    /// Idle interval.
    interval: TimeInterval,
}

impl IdleNoiseQuery {
    /// Creates an idle-noise query.
    #[must_use]
    pub const fn new(
        resource: ScheduleResource,
        interval: TimeInterval,
    ) -> Self {
        Self { resource, interval }
    }

    /// Returns the resource.
    #[must_use]
    pub const fn resource(&self) -> ScheduleResource {
        self.resource
    }

    /// Returns the interval.
    #[must_use]
    pub const fn interval(&self) -> TimeInterval {
        self.interval
    }

    /// Returns the idle duration.
    #[must_use]
    pub fn duration(&self) -> Duration {
        self.interval.duration()
    }
}

/// Interface for models that can specifically evaluate idle exposure.
///
/// This is separate from `SchedulingNoiseModel` so a model that has no
/// specialized idle-noise representation can still provide ordinary operation
/// estimates.
pub trait IdleNoiseModel: SchedulingNoiseModel {
    /// Estimates noise generated by keeping one resource idle.
    fn estimate_idle(
        &self,
        query: &IdleNoiseQuery,
    ) -> SchedulingResult<f64>;
}

// =============================================================================
// Crosstalk query
// =============================================================================

/// Query describing simultaneous resource activity relevant to crosstalk.
///
/// The collection is arbitrary in cardinality.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrosstalkQuery {
    /// Proposed interval during which simultaneous activity is evaluated.
    interval: TimeInterval,

    /// Resources participating in the proposed activity.
    resources: Vec<ScheduleResource>,
}

impl CrosstalkQuery {
    /// Creates a crosstalk query.
    pub fn new(
        interval: TimeInterval,
        resources: impl IntoIterator<Item = ScheduleResource>,
    ) -> SchedulingResult<Self> {
        let resources = canonical_resources(resources)?;

        Ok(Self {
            interval,
            resources,
        })
    }

    /// Returns the interval.
    #[must_use]
    pub const fn interval(&self) -> TimeInterval {
        self.interval
    }

    /// Returns the resources.
    #[must_use]
    pub fn resources(&self) -> &[ScheduleResource] {
        &self.resources
    }

    /// Returns whether the supplied resource participates.
    #[must_use]
    pub fn contains(&self, resource: ScheduleResource) -> bool {
        self.resources.binary_search(&resource).is_ok()
    }
}

/// Optional crosstalk-specific interface.
pub trait CrosstalkNoiseModel: SchedulingNoiseModel {
    /// Estimates crosstalk contribution for simultaneous activity.
    fn estimate_crosstalk(
        &self,
        query: &CrosstalkQuery,
    ) -> SchedulingResult<f64>;
}

// =============================================================================
// Temporal-noise query
// =============================================================================

/// Query for time-dependent noise at a proposed scheduling interval.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemporalNoiseQuery {
    /// Operation associated with the interval.
    operation_id: OperationId,

    /// Proposed interval.
    interval: TimeInterval,

    /// Resources involved.
    resources: Vec<ScheduleResource>,
}

impl TemporalNoiseQuery {
    /// Creates a temporal-noise query.
    pub fn new(
        operation_id: OperationId,
        interval: TimeInterval,
        resources: impl IntoIterator<Item = ScheduleResource>,
    ) -> SchedulingResult<Self> {
        Ok(Self {
            operation_id,
            interval,
            resources: canonical_resources(resources)?,
        })
    }

    /// Returns the operation identity.
    #[must_use]
    pub const fn operation_id(&self) -> OperationId {
        self.operation_id
    }

    /// Returns the interval.
    #[must_use]
    pub const fn interval(&self) -> TimeInterval {
        self.interval
    }

    /// Returns the resources.
    #[must_use]
    pub fn resources(&self) -> &[ScheduleResource] {
        &self.resources
    }
}

/// Optional temporal-noise interface.
pub trait TemporalNoiseModel: SchedulingNoiseModel {
    /// Estimates time-dependent noise.
    fn estimate_temporal(
        &self,
        query: &TemporalNoiseQuery,
    ) -> SchedulingResult<f64>;
}

// =============================================================================
// Calibration-aware scheduling
// =============================================================================

/// Calibration validity state relevant to scheduling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalibrationState {
    /// Calibration is explicitly valid for the proposed placement.
    Valid,

    /// Calibration may be usable but has an explicit uncertainty.
    Uncertain,

    /// Calibration is outside its declared validity interval.
    Expired,

    /// No calibration information is available.
    Unknown,
}

impl CalibrationState {
    /// Returns whether calibration is known to be valid.
    #[must_use]
    pub const fn is_valid(self) -> bool {
        matches!(self, Self::Valid)
    }
}

/// Optional calibration-aware scheduling interface.
pub trait CalibrationAwareSchedulingModel: SchedulingNoiseModel {
    /// Returns the calibration state for a proposed scheduling placement.
    fn calibration_state(
        &self,
        context: &SchedulingNoiseContext,
    ) -> SchedulingResult<CalibrationState>;
}

// =============================================================================
// Deterministic candidate ordering
// =============================================================================

/// A scheduling candidate paired with its noise estimate.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NoiseScoredCandidate {
    /// Operation identity.
    pub operation_id: OperationId,

    /// Candidate start time.
    pub start: TimePoint,

    /// Candidate end time.
    pub end: TimePoint,

    /// Noise-derived score.
    pub score: f64,

    /// Associated estimate.
    pub estimate: SchedulingNoiseEstimate,
}

impl NoiseScoredCandidate {
    /// Creates a validated scored candidate.
    pub fn new(
        operation_id: OperationId,
        start: TimePoint,
        end: TimePoint,
        score: f64,
        estimate: SchedulingNoiseEstimate,
    ) -> SchedulingResult<Self> {
        validate_non_negative_finite(score, "candidate score")?;

        Ok(Self {
            operation_id,
            start,
            end,
            score,
            estimate,
        })
    }
}

impl Eq for NoiseScoredCandidate {}

impl Ord for NoiseScoredCandidate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start
            .cmp(&other.start)
            .then_with(|| self.end.cmp(&other.end))
            .then_with(|| self.score.total_cmp(&other.score))
            .then_with(|| self.operation_id.cmp(&other.operation_id))
    }
}

impl PartialOrd for NoiseScoredCandidate {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// =============================================================================
// Utility validation
// =============================================================================

fn validate_non_negative_finite(
    value: f64,
    field: &'static str,
) -> SchedulingResult<()> {
    if !value.is_finite() {
        return Err(SchedulingIntegrationError::NonFiniteValue {
            field,
            value,
        });
    }

    if value < 0.0 {
        return Err(SchedulingIntegrationError::NegativeCost {
            field,
            value,
        });
    }

    Ok(())
}

fn validate_probability(
    value: f64,
    field: &'static str,
) -> SchedulingResult<()> {
    if !value.is_finite() {
        return Err(SchedulingIntegrationError::NonFiniteValue {
            field,
            value,
        });
    }

    if !(0.0..=1.0).contains(&value) {
        return Err(SchedulingIntegrationError::InvalidProbability {
            field,
            value,
        });
    }

    Ok(())
}

fn validate_precision(
    precision: Precision,
) -> SchedulingResult<()> {
    match precision {
        Precision::Exact | Precision::Unknown => Ok(()),

        Precision::Approximate { tolerance } => {
            validate_non_negative_finite(tolerance, "tolerance")
        }

        Precision::Bounded { error_bound } => {
            validate_probability(error_bound, "error_bound")
        }

        Precision::Statistical { confidence } => {
            validate_probability(confidence, "confidence")
        }
    }
}

fn canonical_resources(
    resources: impl IntoIterator<Item = ScheduleResource>,
) -> SchedulingResult<Vec<ScheduleResource>> {
    let mut set = BTreeSet::new();

    for resource in resources {
        if !set.insert(resource) {
            return Err(
                SchedulingIntegrationError::DuplicateResource { resource },
            );
        }
    }

    Ok(set.into_iter().collect())
}

fn compare_scheduled_operations(
    left: &ScheduledOperation,
    right: &ScheduledOperation,
) -> Ordering {
    left.start()
        .cmp(&right.start())
        .then_with(|| left.end().cmp(&right.end()))
        .then_with(|| left.operation_id().cmp(&right.operation_id()))
        .then_with(|| compare_resources(left.resources(), right.resources()))
}

fn compare_resources(
    left: &[ScheduleResource],
    right: &[ScheduleResource],
) -> Ordering {
    left.iter()
        .cmp(right.iter())
}

// =============================================================================
// Canonical resource helpers
// =============================================================================

/// Returns the canonical logical-qubit resource representation.
#[must_use]
pub const fn logical_qubit_resource(
    qubit: QubitId,
) -> ScheduleResource {
    ScheduleResource::LogicalQubit(qubit)
}

/// Returns the canonical physical-qubit resource representation.
#[must_use]
pub const fn physical_qubit_resource(
    qubit: PhysicalQubitId,
) -> ScheduleResource {
    ScheduleResource::PhysicalQubit(qubit)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestNoiseModel {
        id: NoiseModelId,
    }

    impl SchedulingNoiseModel for TestNoiseModel {
        fn model_id(&self) -> NoiseModelId {
            self.id
        }

        fn estimate(
            &self,
            _context: &SchedulingNoiseContext,
        ) -> SchedulingResult<SchedulingNoiseEstimate> {
            SchedulingNoiseEstimate::new(
                0.1,
                0.2,
                0.05,
                0.01,
                0.02,
                Precision::Exact,
            )
        }
    }

    #[test]
    fn rejects_non_finite_probability() {
        let result = SchedulingNoiseEstimate::new(
            f64::NAN,
            0.0,
            0.0,
            0.0,
            0.0,
            Precision::Exact,
        );

        assert!(matches!(
            result,
            Err(
                SchedulingIntegrationError::NonFiniteValue {
                    field: "error_probability",
                    ..
                }
            )
        ));
    }

    #[test]
    fn rejects_infinite_probability() {
        let result = SchedulingNoiseEstimate::new(
            f64::INFINITY,
            0.0,
            0.0,
            0.0,
            0.0,
            Precision::Exact,
        );

        assert!(matches!(
            result,
            Err(
                SchedulingIntegrationError::NonFiniteValue {
                    field: "error_probability",
                    ..
                }
            )
        ));
    }

    #[test]
    fn rejects_probability_above_one() {
        let result = SchedulingNoiseEstimate::new(
            1.1,
            0.0,
            0.0,
            0.0,
            0.0,
            Precision::Exact,
        );

        assert!(matches!(
            result,
            Err(
                SchedulingIntegrationError::InvalidProbability {
                    field: "error_probability",
                    ..
                }
            )
        ));
    }

    #[test]
    fn rejects_negative_weight() {
        let result = NoiseObjective::new(
            -1.0,
            1.0,
            1.0,
            1.0,
            1.0,
        );

        assert!(matches!(
            result,
            Err(
                SchedulingIntegrationError::NegativeCost {
                    field: "error_weight",
                    ..
                }
            )
        ));
    }

    #[test]
    fn approximate_precision_is_explicit() {
        let precision = Precision::approximate(1.0e-6).unwrap();

        assert!(precision.is_quantified());
        assert!(!precision.is_bounded());
    }

    #[test]
    fn bounded_precision_is_explicit() {
        let precision = Precision::bounded(1.0e-3).unwrap();

        assert!(precision.is_quantified());
        assert!(precision.is_bounded());
    }

    #[test]
    fn unknown_does_not_mean_zero_precision() {
        let estimate = SchedulingNoiseEstimate::unknown();

        assert!(!estimate.precision.is_quantified());
    }

    #[test]
    fn duplicate_resources_are_rejected() {
        let resource =
            logical_qubit_resource(QubitId::new(0));

        let result = SchedulingNoiseContext::new(
            OperationId::new(1),
            TimeInterval::at(TimePoint::ZERO),
            [resource, resource],
        );

        assert!(matches!(
            result,
            Err(
                SchedulingIntegrationError::DuplicateResource {
                    ..
                }
            )
        ));
    }

    #[test]
    fn resources_are_canonicalized() {
        let first =
            logical_qubit_resource(QubitId::new(0));

        let second =
            physical_qubit_resource(PhysicalQubitId::new(0));

        let context = SchedulingNoiseContext::new(
            OperationId::new(1),
            TimeInterval::at(TimePoint::ZERO),
            [second, first],
        )
        .unwrap();

        assert_eq!(context.resources().len(), 2);

        let mut expected = vec![first, second];
        expected.sort();

        assert_eq!(context.resources(), expected.as_slice());
    }

    #[test]
    fn evaluator_requires_matching_model_identity() {
        let model_id = NoiseModelId::new(7);

        let model = TestNoiseModel { id: model_id };

        let context = SchedulingNoiseContext::new(
            OperationId::new(1),
            TimeInterval::at(TimePoint::ZERO),
            [],
        )
        .unwrap();

        let evaluator =
            SchedulingNoiseEvaluator::new(
                &model,
                NoiseObjective::default(),
            );

        let result = evaluator.evaluate(&context);

        assert!(matches!(
            result,
            Err(SchedulingIntegrationError::MissingNoiseModel)
        ));
    }

    #[test]
    fn evaluator_accepts_matching_model_identity() {
        let model_id = NoiseModelId::new(7);

        let model = TestNoiseModel { id: model_id };

        let context = SchedulingNoiseContext::new(
            OperationId::new(1),
            TimeInterval::at(TimePoint::ZERO),
            [],
        )
        .unwrap()
        .with_noise_model(model_id);

        let evaluator =
            SchedulingNoiseEvaluator::new(
                &model,
                NoiseObjective::default(),
            );

        let estimate = evaluator.evaluate(&context).unwrap();

        assert_eq!(estimate.error_probability, 0.1);
    }

    #[test]
    fn objective_is_deterministic() {
        let estimate =
            SchedulingNoiseEstimate::new(
                0.1,
                0.2,
                0.3,
                0.1,
                0.05,
                Precision::Exact,
            )
            .unwrap();

        let objective = NoiseObjective::default();

        let first =
            estimate.objective_contribution(objective).unwrap();

        let second =
            estimate.objective_contribution(objective).unwrap();

        assert_eq!(first, second);
    }

    #[test]
    fn summary_counts_unknown_estimates() {
        let mut summary =
            ScheduleNoiseSummary::default();

        summary
            .record(
                SchedulingNoiseEstimate::unknown(),
                NoiseObjective::default(),
            )
            .unwrap();

        assert_eq!(summary.operations_evaluated, 1);
        assert_eq!(summary.unknown_operations, 1);
        assert_eq!(summary.quantified_operations, 0);
    }

    #[test]
    fn summary_counts_quantified_estimates() {
        let mut summary =
            ScheduleNoiseSummary::default();

        let estimate =
            SchedulingNoiseEstimate::new(
                0.01,
                0.01,
                0.01,
                0.01,
                0.01,
                Precision::Bounded {
                    error_bound: 0.01,
                },
            )
            .unwrap();

        summary
            .record(
                estimate,
                NoiseObjective::default(),
            )
            .unwrap();

        assert_eq!(summary.operations_evaluated, 1);
        assert_eq!(summary.quantified_operations, 1);
        assert_eq!(summary.unknown_operations, 0);
    }

    #[test]
    fn logical_and_physical_resources_remain_distinct() {
        let logical =
            logical_qubit_resource(QubitId::new(7));

        let physical =
            physical_qubit_resource(PhysicalQubitId::new(7));

        assert_ne!(logical, physical);
    }

    #[test]
    fn scored_candidates_have_deterministic_ordering() {
        let estimate =
            SchedulingNoiseEstimate::unknown();

        let a = NoiseScoredCandidate::new(
            OperationId::new(1),
            TimePoint::ZERO,
            TimePoint::ZERO,
            1.0,
            estimate,
        )
        .unwrap();

        let b = NoiseScoredCandidate::new(
            OperationId::new(2),
            TimePoint::ZERO,
            TimePoint::ZERO,
            1.0,
            estimate,
        )
        .unwrap();

        assert!(a < b);
    }

    #[test]
    fn idle_query_preserves_resource_identity() {
        let resource =
            logical_qubit_resource(QubitId::new(3));

        let interval =
            TimeInterval::at(TimePoint::ZERO);

        let query =
            IdleNoiseQuery::new(resource, interval);

        assert_eq!(query.resource(), resource);
        assert_eq!(query.interval(), interval);
    }

    #[test]
    fn crosstalk_query_has_arbitrary_resource_cardinality() {
        let resources = (0_u64..128)
            .map(QubitId::new)
            .map(logical_qubit_resource)
            .collect::<Vec<_>>();

        let query = CrosstalkQuery::new(
            TimeInterval::at(TimePoint::ZERO),
            resources.clone(),
        )
        .unwrap();

        assert_eq!(
            query.resources().len(),
            resources.len()
        );
    }
}