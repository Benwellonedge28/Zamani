//! Zamani Quantum Scheduling — Hybrid Policy
//!
//! Production-grade, target-independent hybrid scheduling policy for the
//! Zamani quantum scheduling subsystem.
//!
//! # Purpose
//!
//! This module defines a composable policy that combines independently
//! calculated scheduling signals into one deterministic scheduling preference.
//!
//! A hybrid policy answers:
//!
//! > "When several legal scheduling candidates are available, how should
//! > their competing scheduling priorities be combined?"
//!
//! It does NOT answer:
//!
//! - which physical qubit a logical qubit maps to;
//! - how quantum operations are represented in canonical IR;
//! - how the dependency graph is constructed;
//! - when a resource is physically available;
//! - how a resource reservation is committed;
//! - how hardware is discovered;
//! - how hardware is controlled;
//! - how pulses are generated;
//! - how QEC is decoded;
//! - how a quantum program is executed.
//!
//! Those responsibilities belong to their owning subsystems.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!       |
//!       v
//! quantum::frontend
//!       |
//!       v
//! quantum::ir
//!       |
//!       v
//! optimization
//!       |
//!       v
//! routing
//!       |
//!       v
//! scheduling adapters
//!       |
//!       v
//! dependency/resource/timing analysis
//!       |
//!       +-----------------------------+
//!       |                             |
//!       v                             v
//! ASAP / ALAP / criticality     resource/fidelity/etc.
//!       |                             |
//!       +-------------+---------------+
//!                     |
//!                     v
//!              HybridPolicy
//!                     |
//!                     v
//!                  planner
//!                     |
//!                     v
//!                verification
//!                     |
//!                     v
//!               ScheduleResult
//! ```
//!
//! The hybrid policy is therefore a policy-composition layer, not a complete
//! scheduling algorithm.
//!
//! # Universal-program principle
//!
//! A Zamani program describes computation rather than machine size.
//!
//! This policy therefore contains no:
//!
//! - qubit count;
//! - physical qubit count;
//! - resource count;
//! - gate count;
//! - maximum schedule depth;
//! - maximum parallelism;
//! - topology dimensions;
//! - hardware vendor;
//! - hardware technology assumption;
//! - fixed channel count;
//! - fixed timing constant.
//!
//! The same hybrid policy can therefore be applied to:
//!
//! - a single quantum system;
//! - a small QPU;
//! - a large QPU;
//! - a modular quantum computer;
//! - a distributed quantum computer;
//! - a quantum network;
//! - a future architecture unknown to Zamani.
//!
//! "Infinity" in the Zamani architecture means that this policy introduces no
//! artificial finite machine-size ceiling. A concrete execution remains
//! bounded by the actual target, compiler process, operating system, memory,
//! address space, network, and execution resources.
//!
//! # Critical architectural distinction
//!
//! Hybrid scheduling does NOT mean:
//!
//! ```text
//! run every scheduling algorithm simultaneously
//! ```
//!
//! Instead, it means:
//!
//! ```text
//! independent scheduling signals
//!             |
//!             v
//!       normalized policy
//!             |
//!             v
//!       deterministic composition
//!             |
//!             v
//!       candidate preference
//! ```
//!
//! The planner remains responsible for legality and resource reservation.
//!
//! A hybrid policy MUST NEVER select an illegal operation merely because its
//! combined score is higher.
//!
//! # Separation of concerns
//!
//! The hybrid policy may combine signals representing:
//!
//! - ASAP urgency;
//! - ALAP urgency;
//! - critical-path urgency;
//! - explicit operation priority;
//! - resource pressure;
//! - resource footprint;
//! - fidelity preference;
//! - energy preference;
//! - deadline urgency;
//! - slack;
//! - communication pressure;
//! - QEC urgency;
//! - source order.
//!
//! The signals are supplied by upstream analyses.
//!
//! The policy does not calculate hardware characteristics itself.
//!
//! # Canonical identities
//!
//! Operation identity MUST remain the canonical Quantum IR identity:
//!
//! ```text
//! crate::quantum::ir::core::identity::OperationId
//! ```
//!
//! This file deliberately does not define another `OperationId`.
//!
//! Logical and physical qubit identity remain owned by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! The hybrid policy normally does not need to inspect qubits directly.
//! Resource analysis is responsible for converting qubit/resource information
//! into scheduling signals.
//!
//! # Integration with policy.rs
//!
//! `policy.rs` is the common policy vocabulary.
//!
//! In particular, this file consumes:
//!
//! ```text
//! SchedulingPolicy
//! SchedulingPolicyKind
//! SchedulingObjective
//! TieBreakRule
//! Determinism
//! ```
//!
//! The hybrid implementation does not redefine those types.
//!
//! The canonical hybrid policy kind is:
//!
//! ```text
//! SchedulingPolicyKind::CriticalPathResourceAware
//! ```
//!
//! A caller may also construct `HybridPolicy` explicitly when a planner wants
//! a more detailed combination of independent signals.
//!
//! # Integration with ASAP and ALAP
//!
//! ASAP and ALAP remain independent policies.
//!
//! HybridPolicy does not duplicate their scheduling calculations.
//!
//! A planner may provide:
//!
//! ```text
//! asap_signal
//! alap_signal
//! ```
//!
//! as normalized candidate preferences.
//!
//! HybridPolicy then combines them with other signals.
//!
//! This avoids coupling hybrid scheduling to the concrete representation of
//! `AsapDecision` or `AlapDecision`.
//!
//! # Integration with resource-aware scheduling
//!
//! Resource-aware analysis may provide:
//!
//! ```text
//! resource_pressure
//! resource_footprint
//! resource_wait
//! scarcity
//! ```
//!
//! These are signals, not resource reservations.
//!
//! HybridPolicy never modifies a resource calendar.
//!
//! # Integration with critical-path scheduling
//!
//! Critical-path analysis may provide:
//!
//! ```text
//! criticality
//! slack
//! remaining_work
//! downstream_weight
//! ```
//!
//! HybridPolicy can use these signals without knowing how the critical path
//! was calculated.
//!
//! # Integration with fidelity/noise
//!
//! The ZQN/noise subsystem may eventually provide estimated fidelity or
//! error-related scheduling signals.
//!
//! The dependency direction is:
//!
//! ```text
//! ZQN/noise
//!      |
//!      v
//! target-aware analysis
//!      |
//!      v
//! hybrid signal
//!      |
//!      v
//! HybridPolicy
//! ```
//!
//! This module does not implement noise modelling.
//!
//! # Integration with hardware
//!
//! Hardware information must arrive through the scheduling target/resource
//! adapter.
//!
//! HybridPolicy never queries a QPU.
//!
//! It must not contain:
//!
//! ```text
//! vendor == ...
//! qubits == ...
//! channels == ...
//! ```
//!
//! or any equivalent hardware-specific assumption.
//!
//! # Integration with routing
//!
//! Routing answers:
//!
//! > WHERE should an operation execute?
//!
//! Scheduling answers:
//!
//! > WHEN should it execute?
//!
//! HybridPolicy consumes already-derived scheduling information. It does not
//! perform logical-to-physical routing.
//!
//! # Integration with QEC
//!
//! QEC scheduling may provide urgency signals for:
//!
//! - syndrome extraction;
//! - ancilla preparation;
//! - stabilizer interactions;
//! - measurements;
//! - feedback;
//! - round boundaries.
//!
//! HybridPolicy does not implement a QEC decoder or surface-code algorithm.
//!
//! # Integration with dynamic scheduling
//!
//! Runtime schedulers may construct a new immutable `HybridCandidate` for each
//! scheduling epoch.
//!
//! No mutable runtime state is stored in `HybridPolicy`.
//!
//! # Integration with distributed scheduling
//!
//! Distributed schedulers may provide communication-related signals:
//!
//! - communication urgency;
//! - network contention;
//! - synchronization pressure;
//! - remote-resource scarcity;
//! - expected communication delay.
//!
//! The hybrid policy does not own the network graph.
//!
//! # Determinism
//!
//! Deterministic mode is supported explicitly.
//!
//! The hybrid score uses checked integer arithmetic and deterministic
//! lexicographic tie-breaking.
//!
//! No floating-point arithmetic is used.
//!
//! This is deliberate:
//!
//! - floating-point NaN ordering is undesirable in scheduling contracts;
//! - platform floating-point behaviour can complicate reproducibility;
//! - integer scoring is sufficient when normalized signal values are supplied;
//! - exact deterministic comparison is valuable for regression testing.
//!
//! # Signal model
//!
//! Each signal is represented as a non-negative normalized integer:
//!
//! ```text
//! 0 ..= signal::MAX
//! ```
//!
//! `MAX` is a mathematical normalization boundary, not a machine-size limit.
//!
//! The signal value has no physical unit.
//!
//! Higher values mean stronger preference for scheduling the candidate earlier.
//!
//! A signal provider is responsible for mapping its domain-specific metric into
//! this normalized representation.
//!
//! # Why normalized signals?
//!
//! Different scheduling quantities are not naturally comparable:
//!
//! ```text
//! nanoseconds
//! resource count
//! fidelity
//! critical-path length
//! priority
//! slack
//! ```
//!
//! HybridPolicy must not pretend these quantities share a physical unit.
//!
//! Instead, upstream analyses normalize them into dimensionless scheduling
//! preference signals.
//!
//! # Weight model
//!
//! Each signal receives an explicit non-negative weight.
//!
//! The policy calculates:
//!
//! ```text
//! score =
//!     ASAP            * asap_weight
//!   + ALAP            * alap_weight
//!   + Criticality     * criticality_weight
//!   + Priority        * priority_weight
//!   + ResourcePressure* resource_weight
//!   + ResourceFootprint* footprint_weight
//!   + Fidelity        * fidelity_weight
//!   + Energy          * energy_weight
//!   + Deadline        * deadline_weight
//!   + Communication   * communication_weight
//!   + QEC             * qec_weight
//! ```
//!
//! Not every signal needs a non-zero weight.
//!
//! A weight of zero disables that signal.
//!
//! No default weight represents a physical hardware assumption.
//!
//! # Overflow
//!
//! Score calculation uses checked arithmetic.
//!
//! An overflow is returned as a structured `HybridError`.
//!
//! The policy never wraps scores.
//!
//! # Legality
//!
//! `HybridPolicy` does not decide whether a candidate is legal.
//!
//! The candidate contains a `legal` flag supplied by the planner/constraint
//! layer.
//!
//! An illegal candidate is always ranked below a legal candidate.
//!
//! If no legal candidate exists, the policy reports that condition.
//!
//! This ensures that optimization preferences can never override correctness.
//!
//! # Tie-breaking
//!
//! After primary score comparison, the policy applies a deterministic
//! lexicographic tie-break sequence.
//!
//! The default sequence is:
//!
//! ```text
//! legal
//! score
//! deadline urgency
//! criticality
//! resource pressure
//! explicit priority
//! earliest start
//! source order
//! canonical OperationId
//! ```
//!
//! The final canonical `OperationId` tie-break guarantees a stable total order
//! when all other signals are equal.
//!
//! # No hidden machine-size state
//!
//! The policy stores only configuration.
//!
//! It does not store:
//!
//! - all qubits;
//! - all resources;
//! - a machine topology;
//! - a schedule timeline;
//! - a dependency graph;
//! - reservations;
//! - hardware handles.
//!
//! Therefore memory usage is O(1) per policy instance.
//!
//! # Complexity
//!
//! For one candidate, scoring is O(S), where S is the number of enabled signal
//! components.
//!
//! The built-in signal set is finite and policy-defined; it is not a machine
//! resource enumeration.
//!
//! No structure proportional to the number of qubits or hardware resources is
//! allocated.
//!
//! # Thread safety
//!
//! `HybridPolicy` is immutable and contains only plain value types.
//!
//! It has no global state and no interior mutability.
//!
//! It is therefore suitable for concurrent read-only use.
//!
//! # Serialization
//!
//! The policy is intentionally represented entirely by explicit value types.
//!
//! A future `serialization` adapter can serialize:
//!
//! - schema ID;
//! - schema version;
//! - policy mode;
//! - objective;
//! - weights;
//! - tie-break rule;
//! - deterministic mode.
//!
//! This file does not depend on serde or a particular wire format.
//!
//! # Frozen-file contract
//!
//! Downstream modules should consume this contract without modifying it merely
//! because another scheduler subsystem is added.
//!
//! New hardware/resource types should produce new normalized signals rather
//! than forcing this file to know the hardware type.
//!
//! If a future scheduling concern cannot be represented by the current signal
//! model, it should normally be implemented as an adapter-level transformation
//! before changing this foundational contract.
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
//! The safety boundary is compiler-enforced.
//!
//! =============================================================================
//! Safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::cmp::Ordering;
use core::fmt;

use crate::quantum::ir::core::identity::OperationId;

use super::policy::{
    Determinism,
    SchedulingObjective,
    SchedulingPolicy,
    SchedulingPolicyKind,
    TieBreakRule,
};

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier for the hybrid policy.
pub const HYBRID_POLICY_SCHEMA_ID: &str =
    "zamani.quantum.scheduling.policy.hybrid";

/// Semantic schema version for the hybrid policy.
pub const HYBRID_POLICY_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Normalized signal
// =============================================================================

/// Maximum normalized signal value.
///
/// This is a mathematical normalization boundary. It is NOT a hardware or
/// machine-size limit.
///
/// Signal providers may map their native metrics into the inclusive range
/// `0..=SIGNAL_MAX`.
pub const SIGNAL_MAX: u64 = u64::MAX;

/// A normalized, dimensionless scheduling preference signal.
///
/// Higher values indicate stronger preference for scheduling the candidate
/// earlier.
///
/// The value has no physical unit.
///
/// The type intentionally contains no hardware information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Signal(u64);

impl Signal {
    /// Lowest possible signal.
    pub const MIN: Self = Self(0);

    /// Highest possible signal.
    pub const MAX: Self = Self(SIGNAL_MAX);

    /// Creates a normalized signal.
    ///
    /// Every `u64` value is already inside the normalized domain.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the normalized signal value.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Returns whether the signal is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Returns whether the signal is maximal.
    #[must_use]
    pub const fn is_max(self) -> bool {
        self.0 == SIGNAL_MAX
    }

    /// Returns the complement of the signal.
    ///
    /// This is useful for metrics where a low native value represents a high
    /// scheduling preference.
    #[must_use]
    pub const fn complement(self) -> Self {
        Self(SIGNAL_MAX - self.0)
    }

    /// Adds two signals with saturation.
    ///
    /// This operation is intended only for diagnostic/normalization helpers.
    /// Hybrid score calculation itself uses checked arithmetic.
    #[must_use]
    pub const fn saturating_add(self, other: Self) -> Self {
        Self(self.0.saturating_add(other.0))
    }

    /// Multiplies a signal by a weight with checked arithmetic.
    #[must_use]
    pub const fn checked_mul(self, weight: Weight) -> Option<u128> {
        (self.0 as u128).checked_mul(weight.value() as u128)
    }
}

impl From<u64> for Signal {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<Signal> for u64 {
    fn from(signal: Signal) -> Self {
        signal.value()
    }
}

impl fmt::Display for Signal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

// =============================================================================
// Weight
// =============================================================================

/// Non-negative hybrid scoring weight.
///
/// Weights are dimensionless coefficients. They are not physical units and
/// do not encode hardware capacity.
///
/// `u64` provides a large configuration space while keeping the value compact
/// and deterministic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Weight(u64);

impl Weight {
    /// Zero weight disables a signal.
    pub const ZERO: Self = Self(0);

    /// Creates a weight.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the weight value.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Returns whether the weight is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }
}

impl From<u64> for Weight {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<Weight> for u64 {
    fn from(weight: Weight) -> Self {
        weight.value()
    }
}

impl fmt::Display for Weight {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

// =============================================================================
// Hybrid signal vector
// =============================================================================

/// Independent scheduling signals consumed by [`HybridPolicy`].
///
/// Every field is a normalized, dimensionless preference signal.
///
/// A value of zero means that the corresponding analysis contributes no
/// positive preference for this candidate.
///
/// Higher values mean stronger preference for scheduling the candidate earlier.
///
/// The structure deliberately contains no qubit, resource, topology, timing,
/// or hardware collections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct HybridSignals {
    /// Preference derived from ASAP analysis.
    asap: Signal,

    /// Preference derived from ALAP/slack analysis.
    alap: Signal,

    /// Critical-path urgency.
    criticality: Signal,

    /// Explicit operation priority.
    priority: Signal,

    /// Pressure caused by scarce resources.
    resource_pressure: Signal,

    /// Preference associated with a small/efficient resource footprint.
    resource_footprint: Signal,

    /// Estimated fidelity preference.
    fidelity: Signal,

    /// Estimated energy/resource-cost preference.
    energy: Signal,

    /// Deadline urgency.
    deadline: Signal,

    /// Communication/network urgency.
    communication: Signal,

    /// QEC urgency.
    qec: Signal,
}

impl HybridSignals {
    /// Creates an all-zero signal vector.
    #[must_use]
    pub const fn new() -> Self {
        Self::default()
    }

    /// Returns the ASAP signal.
    #[must_use]
    pub const fn asap(self) -> Signal {
        self.asap
    }

    /// Returns the ALAP signal.
    #[must_use]
    pub const fn alap(self) -> Signal {
        self.alap
    }

    /// Returns the criticality signal.
    #[must_use]
    pub const fn criticality(self) -> Signal {
        self.criticality
    }

    /// Returns the explicit priority signal.
    #[must_use]
    pub const fn priority(self) -> Signal {
        self.priority
    }

    /// Returns the resource-pressure signal.
    #[must_use]
    pub const fn resource_pressure(self) -> Signal {
        self.resource_pressure
    }

    /// Returns the resource-footprint signal.
    #[must_use]
    pub const fn resource_footprint(self) -> Signal {
        self.resource_footprint
    }

    /// Returns the fidelity signal.
    #[must_use]
    pub const fn fidelity(self) -> Signal {
        self.fidelity
    }

    /// Returns the energy signal.
    #[must_use]
    pub const fn energy(self) -> Signal {
        self.energy
    }

    /// Returns the deadline signal.
    #[must_use]
    pub const fn deadline(self) -> Signal {
        self.deadline
    }

    /// Returns the communication signal.
    #[must_use]
    pub const fn communication(self) -> Signal {
        self.communication
    }

    /// Returns the QEC signal.
    #[must_use]
    pub const fn qec(self) -> Signal {
        self.qec
    }

    /// Sets the ASAP signal.
    #[must_use]
    pub const fn with_asap(mut self, value: Signal) -> Self {
        self.asap = value;
        self
    }

    /// Sets the ALAP signal.
    #[must_use]
    pub const fn with_alap(mut self, value: Signal) -> Self {
        self.alap = value;
        self
    }

    /// Sets the criticality signal.
    #[must_use]
    pub const fn with_criticality(mut self, value: Signal) -> Self {
        self.criticality = value;
        self
    }

    /// Sets the explicit priority signal.
    #[must_use]
    pub const fn with_priority(mut self, value: Signal) -> Self {
        self.priority = value;
        self
    }

    /// Sets the resource-pressure signal.
    #[must_use]
    pub const fn with_resource_pressure(mut self, value: Signal) -> Self {
        self.resource_pressure = value;
        self
    }

    /// Sets the resource-footprint signal.
    #[must_use]
    pub const fn with_resource_footprint(mut self, value: Signal) -> Self {
        self.resource_footprint = value;
        self
    }

    /// Sets the fidelity signal.
    #[must_use]
    pub const fn with_fidelity(mut self, value: Signal) -> Self {
        self.fidelity = value;
        self
    }

    /// Sets the energy signal.
    #[must_use]
    pub const fn with_energy(mut self, value: Signal) -> Self {
        self.energy = value;
        self
    }

    /// Sets the deadline signal.
    #[must_use]
    pub const fn with_deadline(mut self, value: Signal) -> Self {
        self.deadline = value;
        self
    }

    /// Sets the communication signal.
    #[must_use]
    pub const fn with_communication(mut self, value: Signal) -> Self {
        self.communication = value;
        self
    }

    /// Sets the QEC signal.
    #[must_use]
    pub const fn with_qec(mut self, value: Signal) -> Self {
        self.qec = value;
        self
    }
}

// =============================================================================
// Hybrid weights
// =============================================================================

/// Weight vector used by [`HybridPolicy`].
///
/// All weights are explicit and independent of hardware capacity.
///
/// Zero is a valid value and disables the corresponding signal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HybridWeights {
    asap: Weight,
    alap: Weight,
    criticality: Weight,
    priority: Weight,
    resource_pressure: Weight,
    resource_footprint: Weight,
    fidelity: Weight,
    energy: Weight,
    deadline: Weight,
    communication: Weight,
    qec: Weight,
}

impl HybridWeights {
    /// Creates an all-zero weight vector.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            asap: Weight::ZERO,
            alap: Weight::ZERO,
            criticality: Weight::ZERO,
            priority: Weight::ZERO,
            resource_pressure: Weight::ZERO,
            resource_footprint: Weight::ZERO,
            fidelity: Weight::ZERO,
            energy: Weight::ZERO,
            deadline: Weight::ZERO,
            communication: Weight::ZERO,
            qec: Weight::ZERO,
        }
    }

    /// Returns the ASAP weight.
    #[must_use]
    pub const fn asap(self) -> Weight {
        self.asap
    }

    /// Returns the ALAP weight.
    #[must_use]
    pub const fn alap(self) -> Weight {
        self.alap
    }

    /// Returns the criticality weight.
    #[must_use]
    pub const fn criticality(self) -> Weight {
        self.criticality
    }

    /// Returns the explicit priority weight.
    #[must_use]
    pub const fn priority(self) -> Weight {
        self.priority
    }

    /// Returns the resource-pressure weight.
    #[must_use]
    pub const fn resource_pressure(self) -> Weight {
        self.resource_pressure
    }

    /// Returns the resource-footprint weight.
    #[must_use]
    pub const fn resource_footprint(self) -> Weight {
        self.resource_footprint
    }

    /// Returns the fidelity weight.
    #[must_use]
    pub const fn fidelity(self) -> Weight {
        self.fidelity
    }

    /// Returns the energy weight.
    #[must_use]
    pub const fn energy(self) -> Weight {
        self.energy
    }

    /// Returns the deadline weight.
    #[must_use]
    pub const fn deadline(self) -> Weight {
        self.deadline
    }

    /// Returns the communication weight.
    #[must_use]
    pub const fn communication(self) -> Weight {
        self.communication
    }

    /// Returns the QEC weight.
    #[must_use]
    pub const fn qec(self) -> Weight {
        self.qec
    }

    /// Sets the ASAP weight.
    #[must_use]
    pub const fn with_asap(mut self, value: Weight) -> Self {
        self.asap = value;
        self
    }

    /// Sets the ALAP weight.
    #[must_use]
    pub const fn with_alap(mut self, value: Weight) -> Self {
        self.alap = value;
        self
    }

    /// Sets the criticality weight.
    #[must_use]
    pub const fn with_criticality(mut self, value: Weight) -> Self {
        self.criticality = value;
        self
    }

    /// Sets the explicit priority weight.
    #[must_use]
    pub const fn with_priority(mut self, value: Weight) -> Self {
        self.priority = value;
        self
    }

    /// Sets the resource-pressure weight.
    #[must_use]
    pub const fn with_resource_pressure(mut self, value: Weight) -> Self {
        self.resource_pressure = value;
        self
    }

    /// Sets the resource-footprint weight.
    #[must_use]
    pub const fn with_resource_footprint(mut self, value: Weight) -> Self {
        self.resource_footprint = value;
        self
    }

    /// Sets the fidelity weight.
    #[must_use]
    pub const fn with_fidelity(mut self, value: Weight) -> Self {
        self.fidelity = value;
        self
    }

    /// Sets the energy weight.
    #[must_use]
    pub const fn with_energy(mut self, value: Weight) -> Self {
        self.energy = value;
        self
    }

    /// Sets the deadline weight.
    #[must_use]
    pub const fn with_deadline(mut self, value: Weight) -> Self {
        self.deadline = value;
        self
    }

    /// Sets the communication weight.
    #[must_use]
    pub const fn with_communication(mut self, value: Weight) -> Self {
        self.communication = value;
        self
    }

    /// Sets the QEC weight.
    #[must_use]
    pub const fn with_qec(mut self, value: Weight) -> Self {
        self.qec = value;
        self
    }

    /// Creates weights appropriate for a critical-path/resource-aware
    /// configuration using the caller's existing policy weights.
    ///
    /// No hardware assumptions are introduced.
    #[must_use]
    pub const fn from_policy(policy: &SchedulingPolicy) -> Self {
        Self {
            asap: Weight::new(policy.criticality_weight() as u64),
            alap: Weight::ZERO,
            criticality: Weight::new(policy.criticality_weight() as u64),
            priority: Weight::new(policy.priority_weight() as u64),
            resource_pressure: Weight::new(policy.resource_weight() as u64),
            resource_footprint: Weight::new(policy.resource_weight() as u64),
            fidelity: Weight::new(policy.fidelity_weight() as u64),
            energy: Weight::ZERO,
            deadline: Weight::new(policy.priority_weight() as u64),
            communication: Weight::new(policy.resource_weight() as u64),
            qec: Weight::new(policy.criticality_weight() as u64),
        }
    }
}

impl Default for HybridWeights {
    fn default() -> Self {
        Self {
            asap: Weight::new(1),
            alap: Weight::ZERO,
            criticality: Weight::new(1),
            priority: Weight::new(1),
            resource_pressure: Weight::new(1),
            resource_footprint: Weight::new(1),
            fidelity: Weight::new(1),
            energy: Weight::ZERO,
            deadline: Weight::new(1),
            communication: Weight::new(1),
            qec: Weight::new(1),
        }
    }
}

// =============================================================================
// Candidate metadata
// =============================================================================

/// Optional deterministic source-order position.
///
/// This is not a machine-size quantity. It is supplied by the planner when
/// source/program order is meaningful.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SourceOrder(u128);

impl SourceOrder {
    /// Creates a source-order value.
    #[must_use]
    pub const fn new(value: u128) -> Self {
        Self(value)
    }

    /// Returns the source-order value.
    #[must_use]
    pub const fn value(self) -> u128 {
        self.0
    }
}

/// Candidate metadata consumed by the hybrid policy.
///
/// The planner is responsible for establishing these values.
///
/// In particular, `legal` MUST be based on actual scheduler constraints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HybridCandidate {
    operation: OperationId,
    legal: bool,
    signals: HybridSignals,
    earliest_start: u128,
    source_order: Option<SourceOrder>,
}

impl HybridCandidate {
    /// Creates a candidate.
    ///
    /// `earliest_start` is an abstract scheduler coordinate. It does not
    /// represent a physical unit.
    #[must_use]
    pub const fn new(
        operation: OperationId,
        legal: bool,
        signals: HybridSignals,
        earliest_start: u128,
    ) -> Self {
        Self {
            operation,
            legal,
            signals,
            earliest_start,
            source_order: None,
        }
    }

    /// Returns the canonical operation identity.
    #[must_use]
    pub const fn operation(&self) -> OperationId {
        self.operation
    }

    /// Returns whether the planner has determined this candidate to be legal.
    #[must_use]
    pub const fn legal(&self) -> bool {
        self.legal
    }

    /// Returns the normalized signals.
    #[must_use]
    pub const fn signals(&self) -> HybridSignals {
        self.signals
    }

    /// Returns the earliest legal start supplied by the planner.
    #[must_use]
    pub const fn earliest_start(&self) -> u128 {
        self.earliest_start
    }

    /// Returns the optional source-order position.
    #[must_use]
    pub const fn source_order(&self) -> Option<SourceOrder> {
        self.source_order
    }

    /// Adds source-order metadata.
    #[must_use]
    pub const fn with_source_order(mut self, value: SourceOrder) -> Self {
        self.source_order = Some(value);
        self
    }
}

// =============================================================================
// Score
// =============================================================================

/// Checked hybrid score.
///
/// Scores are unsigned because the policy represents positive preference.
///
/// The internal width is `u128` so individual normalized `u64` signals can be
/// multiplied by `u64` weights without immediately truncating the result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct HybridScore(u128);

impl HybridScore {
    /// Zero score.
    pub const ZERO: Self = Self(0);

    /// Creates a score from an explicitly calculated value.
    #[must_use]
    pub const fn new(value: u128) -> Self {
        Self(value)
    }

    /// Returns the score value.
    #[must_use]
    pub const fn value(self) -> u128 {
        self.0
    }
}

impl fmt::Display for HybridScore {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

// =============================================================================
// Score contribution
// =============================================================================

/// One component of a hybrid score.
///
/// This is retained for diagnostics and explainability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScoreContribution {
    signal: SignalKind,
    signal_value: Signal,
    weight: Weight,
    contribution: u128,
}

impl ScoreContribution {
    /// Creates a contribution.
    #[must_use]
    pub const fn new(
        signal: SignalKind,
        signal_value: Signal,
        weight: Weight,
        contribution: u128,
    ) -> Self {
        Self {
            signal,
            signal_value,
            weight,
            contribution,
        }
    }

    /// Returns the signal kind.
    #[must_use]
    pub const fn signal(&self) -> SignalKind {
        self.signal
    }

    /// Returns the raw normalized signal.
    #[must_use]
    pub const fn signal_value(&self) -> Signal {
        self.signal_value
    }

    /// Returns the weight.
    #[must_use]
    pub const fn weight(&self) -> Weight {
        self.weight
    }

    /// Returns the weighted contribution.
    #[must_use]
    pub const fn contribution(&self) -> u128 {
        self.contribution
    }
}

/// Signal category used for diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SignalKind {
    /// ASAP-derived preference.
    Asap,

    /// ALAP-derived preference.
    Alap,

    /// Critical-path urgency.
    Criticality,

    /// Explicit priority.
    Priority,

    /// Resource pressure.
    ResourcePressure,

    /// Resource footprint.
    ResourceFootprint,

    /// Fidelity preference.
    Fidelity,

    /// Energy preference.
    Energy,

    /// Deadline urgency.
    Deadline,

    /// Communication urgency.
    Communication,

    /// QEC urgency.
    Qec,
}

impl SignalKind {
    /// Returns a stable machine-readable name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Asap => "asap",
            Self::Alap => "alap",
            Self::Criticality => "criticality",
            Self::Priority => "priority",
            Self::ResourcePressure => "resource_pressure",
            Self::ResourceFootprint => "resource_footprint",
            Self::Fidelity => "fidelity",
            Self::Energy => "energy",
            Self::Deadline => "deadline",
            Self::Communication => "communication",
            Self::Qec => "qec",
        }
    }
}

impl fmt::Display for SignalKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Evaluation result
// =============================================================================

/// Detailed result of evaluating one hybrid candidate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HybridEvaluation {
    operation: OperationId,
    legal: bool,
    score: HybridScore,
    contributions: Vec<ScoreContribution>,
}

impl HybridEvaluation {
    /// Creates an evaluation.
    fn new(
        operation: OperationId,
        legal: bool,
        score: HybridScore,
        contributions: Vec<ScoreContribution>,
    ) -> Self {
        Self {
            operation,
            legal,
            score,
            contributions,
        }
    }

    /// Returns the operation identity.
    #[must_use]
    pub const fn operation(&self) -> OperationId {
        self.operation
    }

    /// Returns whether the candidate is legal.
    #[must_use]
    pub const fn legal(&self) -> bool {
        self.legal
    }

    /// Returns the total score.
    #[must_use]
    pub const fn score(&self) -> HybridScore {
        self.score
    }

    /// Returns the individual score contributions.
    #[must_use]
    pub fn contributions(&self) -> &[ScoreContribution] {
        &self.contributions
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by hybrid policy evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HybridError {
    /// The policy configuration is incompatible with the hybrid evaluator.
    InvalidPolicy {
        /// Stable explanation.
        reason: &'static str,
    },

    /// A weighted signal contribution overflowed `u128`.
    ScoreOverflow {
        /// Signal responsible for the overflow.
        signal: SignalKind,

        /// Operation being evaluated.
        operation: OperationId,
    },

    /// No candidate supplied to a selection operation.
    EmptyCandidateSet,

    /// Candidates were supplied but all were illegal.
    NoLegalCandidate,

    /// Two candidates had the same identity.
    DuplicateOperation {
        /// Duplicated operation identity.
        operation: OperationId,
    },
}

impl fmt::Display for HybridError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPolicy { reason } => {
                write!(formatter, "invalid hybrid scheduling policy: {reason}")
            }

            Self::ScoreOverflow {
                signal,
                operation,
            } => write!(
                formatter,
                "hybrid score overflow while evaluating signal `{signal}` for operation `{operation}`"
            ),

            Self::EmptyCandidateSet => {
                formatter.write_str("hybrid policy received an empty candidate set")
            }

            Self::NoLegalCandidate => {
                formatter.write_str("hybrid policy received no legal scheduling candidate")
            }

            Self::DuplicateOperation { operation } => write!(
                formatter,
                "hybrid policy received duplicate operation `{operation}`"
            ),
        }
    }
}

impl std::error::Error for HybridError {}

// =============================================================================
// Hybrid policy
// =============================================================================

/// Production hybrid scheduling policy.
///
/// The policy is immutable and stateless with respect to a scheduling
/// invocation.
///
/// It can therefore be:
///
/// - constructed per scheduling request;
/// - shared across threads;
/// - embedded in a planner;
/// - used in deterministic scheduling;
/// - serialized through an external adapter.
///
/// The policy does not own resource state, dependency state, or hardware state.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HybridPolicy {
    weights: HybridWeights,
    tie_break: TieBreakRule,
    determinism: Determinism,
    objective: SchedulingObjective,
}

impl HybridPolicy {
    /// Creates a hybrid policy from the production policy descriptor.
    ///
    /// The descriptor must identify a hybrid-compatible policy kind.
    pub fn from_policy(policy: &SchedulingPolicy) -> Result<Self, HybridError> {
        if !Self::is_hybrid_kind(policy.kind()) {
            return Err(HybridError::InvalidPolicy {
                reason: "the supplied SchedulingPolicyKind is not a hybrid policy kind",
            });
        }

        Ok(Self {
            weights: HybridWeights::from_policy(policy),
            tie_break: policy.tie_break(),
            determinism: policy.determinism(),
            objective: policy.objective(),
        })
    }

    /// Creates an explicitly configured hybrid policy.
    #[must_use]
    pub const fn new(weights: HybridWeights) -> Self {
        Self {
            weights,
            tie_break: TieBreakRule::DeterministicDefault,
            determinism: Determinism::Deterministic,
            objective: SchedulingObjective::MinimizeMakespan,
        }
    }

    /// Creates the canonical critical-path/resource-aware hybrid policy.
    #[must_use]
    pub const fn critical_path_resource_aware() -> Self {
        Self {
            weights: HybridWeights {
                asap: Weight::new(1),
                alap: Weight::ZERO,
                criticality: Weight::new(2),
                priority: Weight::new(1),
                resource_pressure: Weight::new(2),
                resource_footprint: Weight::new(1),
                fidelity: Weight::new(1),
                energy: Weight::ZERO,
                deadline: Weight::new(2),
                communication: Weight::new(1),
                qec: Weight::new(1),
            },
            tie_break: TieBreakRule::DeterministicDefault,
            determinism: Determinism::Deterministic,
            objective: SchedulingObjective::MinimizeMakespan,
        }
    }

    /// Returns whether a policy kind is hybrid-compatible.
    #[must_use]
    pub const fn is_hybrid_kind(kind: SchedulingPolicyKind) -> bool {
        matches!(
            kind,
            SchedulingPolicyKind::CriticalPathResourceAware
                | SchedulingPolicyKind::Adaptive
        )
    }

    /// Returns the stable policy name.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        "hybrid"
    }

    /// Returns the stable schema identifier.
    #[must_use]
    pub const fn schema_id(&self) -> &'static str {
        HYBRID_POLICY_SCHEMA_ID
    }

    /// Returns the schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u16 {
        HYBRID_POLICY_SCHEMA_VERSION
    }

    /// Returns the configured signal weights.
    #[must_use]
    pub const fn weights(&self) -> HybridWeights {
        self.weights
    }

    /// Returns the configured tie-break rule.
    #[must_use]
    pub const fn tie_break(&self) -> TieBreakRule {
        self.tie_break
    }

    /// Returns the configured determinism mode.
    #[must_use]
    pub const fn determinism(&self) -> Determinism {
        self.determinism
    }

    /// Returns whether deterministic behaviour is required.
    #[must_use]
    pub const fn deterministic(&self) -> bool {
        self.determinism.is_required()
    }

    /// Returns the configured primary objective.
    #[must_use]
    pub const fn objective(&self) -> SchedulingObjective {
        self.objective
    }

    /// Replaces the tie-break rule.
    #[must_use]
    pub const fn with_tie_break(mut self, tie_break: TieBreakRule) -> Self {
        self.tie_break = tie_break;
        self
    }

    /// Replaces the determinism mode.
    #[must_use]
    pub const fn with_determinism(mut self, determinism: Determinism) -> Self {
        self.determinism = determinism;
        self
    }

    /// Replaces the primary objective.
    #[must_use]
    pub const fn with_objective(
        mut self,
        objective: SchedulingObjective,
    ) -> Self {
        self.objective = objective;
        self
    }

    /// Validates the policy-local configuration.
    pub fn validate(&self) -> Result<(), HybridError> {
        // The hybrid policy permits an all-zero weight vector. Such a policy
        // remains valid because deterministic tie-breaking still defines a
        // total preference order.
        //
        // This is important for callers that intentionally want pure
        // deterministic ordering without weighted optimization.
        Ok(())
    }

    /// Calculates the weighted score for one candidate.
    ///
    /// This method does not determine legality and does not reserve resources.
    ///
    /// A caller must still perform full planner/verification checks.
    pub fn evaluate(
        &self,
        candidate: &HybridCandidate,
    ) -> Result<HybridEvaluation, HybridError> {
        self.validate()?;

        let mut total = 0_u128;
        let mut contributions = Vec::new();

        self.add_contribution(
            &mut total,
            &mut contributions,
            candidate.operation(),
            SignalKind::Asap,
            candidate.signals().asap(),
            self.weights.asap(),
        )?;

        self.add_contribution(
            &mut total,
            &mut contributions,
            candidate.operation(),
            SignalKind::Alap,
            candidate.signals().alap(),
            self.weights.alap,
        )?;

        self.add_contribution(
            &mut total,
            &mut contributions,
            candidate.operation(),
            SignalKind::Criticality,
            candidate.signals().criticality(),
            self.weights.criticality,
        )?;

        self.add_contribution(
            &mut total,
            &mut contributions,
            candidate.operation(),
            SignalKind::Priority,
            candidate.signals().priority(),
            self.weights.priority,
        )?;

        self.add_contribution(
            &mut total,
            &mut contributions,
            candidate.operation(),
            SignalKind::ResourcePressure,
            candidate.signals().resource_pressure(),
            self.weights.resource_pressure,
        )?;

        self.add_contribution(
            &mut total,
            &mut contributions,
            candidate.operation(),
            SignalKind::ResourceFootprint,
            candidate.signals().resource_footprint(),
            self.weights.resource_footprint,
        )?;

        self.add_contribution(
            &mut total,
            &mut contributions,
            candidate.operation(),
            SignalKind::Fidelity,
            candidate.signals().fidelity(),
            self.weights.fidelity,
        )?;

        self.add_contribution(
            &mut total,
            &mut contributions,
            candidate.operation(),
            SignalKind::Energy,
            candidate.signals().energy(),
            self.weights.energy,
        )?;

        self.add_contribution(
            &mut total,
            &mut contributions,
            candidate.operation(),
            SignalKind::Deadline,
            candidate.signals().deadline(),
            self.weights.deadline,
        )?;

        self.add_contribution(
            &mut total,
            &mut contributions,
            candidate.operation(),
            SignalKind::Communication,
            candidate.signals().communication(),
            self.weights.communication,
        )?;

        self.add_contribution(
            &mut total,
            &mut contributions,
            candidate.operation(),
            SignalKind::Qec,
            candidate.signals().qec(),
            self.weights.qec,
        )?;

        Ok(HybridEvaluation::new(
            candidate.operation(),
            candidate.legal(),
            HybridScore::new(total),
            contributions,
        ))
    }

    /// Compares two candidates.
    ///
    /// `Ordering::Greater` means the left candidate has stronger scheduling
    /// preference.
    ///
    /// Legality always dominates optimization score.
    #[must_use]
    pub fn compare(
        &self,
        left: &HybridCandidate,
        right: &HybridCandidate,
    ) -> Result<Ordering, HybridError> {
        let left_evaluation = self.evaluate(left)?;
        let right_evaluation = self.evaluate(right)?;

        Ok(self.compare_evaluations(
            left,
            &left_evaluation,
            right,
            &right_evaluation,
        ))
    }

    /// Returns whether the left candidate is preferred.
    pub fn prefers(
        &self,
        left: &HybridCandidate,
        right: &HybridCandidate,
    ) -> Result<bool, HybridError> {
        Ok(self.compare(left, right)? == Ordering::Greater)
    }

    /// Selects the strongest candidate from an iterable.
    ///
    /// The iterable is consumed exactly once.
    ///
    /// This function does not require storing the complete candidate set,
    /// making it suitable for planners whose ready set is large or generated
    /// incrementally.
    pub fn select<'a, I>(
        &self,
        candidates: I,
    ) -> Result<&'a HybridCandidate, HybridError>
    where
        I: IntoIterator<Item = &'a HybridCandidate>,
    {
        let mut selected: Option<&'a HybridCandidate> = None;
        let mut selected_evaluation: Option<HybridEvaluation> = None;

        for candidate in candidates {
            let evaluation = self.evaluate(candidate)?;

            match (&selected, &selected_evaluation) {
                (None, None) => {
                    selected = Some(candidate);
                    selected_evaluation = Some(evaluation);
                }

                (Some(current), Some(current_evaluation)) => {
                    let ordering = self.compare_evaluations(
                        candidate,
                        &evaluation,
                        current,
                        current_evaluation,
                    );

                    if ordering == Ordering::Greater {
                        selected = Some(candidate);
                        selected_evaluation = Some(evaluation);
                    }
                }

                _ => {
                    // Internal state is deliberately kept consistent.
                    // This branch is unreachable through safe construction.
                    return Err(HybridError::EmptyCandidateSet);
                }
            }
        }

        match selected {
            Some(candidate) => {
                if !candidate.legal() {
                    return Err(HybridError::NoLegalCandidate);
                }

                Ok(candidate)
            }

            None => Err(HybridError::EmptyCandidateSet),
        }
    }

    /// Evaluates every candidate while retaining deterministic input order.
    ///
    /// This is intended for diagnostics, planner introspection, and testing.
    ///
    /// Unlike `select`, this method necessarily materializes one evaluation per
    /// supplied candidate and should therefore be used only where that memory
    /// trade-off is intentional.
    pub fn evaluate_all<'a, I>(
        &self,
        candidates: I,
    ) -> Result<Vec<HybridEvaluation>, HybridError>
    where
        I: IntoIterator<Item = &'a HybridCandidate>,
    {
        let mut evaluations = Vec::new();

        for candidate in candidates {
            evaluations.push(self.evaluate(candidate)?);
        }

        Ok(evaluations)
    }

    /// Returns the highest-priority legal candidate while rejecting duplicate
    /// operation identities.
    ///
    /// Duplicate detection is optional at the planner level, but this method
    /// provides a safe boundary for callers that need it.
    pub fn select_unique<'a, I>(
        &self,
        candidates: I,
    ) -> Result<&'a HybridCandidate, HybridError>
    where
        I: IntoIterator<Item = &'a HybridCandidate>,
    {
        use std::collections::BTreeSet;

        let mut seen = BTreeSet::new();
        let mut selected: Option<&'a HybridCandidate> = None;
        let mut selected_evaluation: Option<HybridEvaluation> = None;

        for candidate in candidates {
            if !seen.insert(candidate.operation()) {
                return Err(HybridError::DuplicateOperation {
                    operation: candidate.operation(),
                });
            }

            let evaluation = self.evaluate(candidate)?;

            match (&selected, &selected_evaluation) {
                (None, None) => {
                    selected = Some(candidate);
                    selected_evaluation = Some(evaluation);
                }

                (Some(current), Some(current_evaluation)) => {
                    if self.compare_evaluations(
                        candidate,
                        &evaluation,
                        current,
                        current_evaluation,
                    ) == Ordering::Greater
                    {
                        selected = Some(candidate);
                        selected_evaluation = Some(evaluation);
                    }
                }

                _ => {
                    return Err(HybridError::EmptyCandidateSet);
                }
            }
        }

        match selected {
            Some(candidate) if candidate.legal() => Ok(candidate),
            Some(_) => Err(HybridError::NoLegalCandidate),
            None => Err(HybridError::EmptyCandidateSet),
        }
    }

    // -------------------------------------------------------------------------
    // Internal scoring
    // -------------------------------------------------------------------------

    fn add_contribution(
        &self,
        total: &mut u128,
        contributions: &mut Vec<ScoreContribution>,
        operation: OperationId,
        signal: SignalKind,
        value: Signal,
        weight: Weight,
    ) -> Result<(), HybridError> {
        if weight.is_zero() || value.is_zero() {
            return Ok(());
        }

        let contribution = value
            .checked_mul(weight)
            .ok_or(HybridError::ScoreOverflow {
                signal,
                operation,
            })?;

        *total = total
            .checked_add(contribution)
            .ok_or(HybridError::ScoreOverflow {
                signal,
                operation,
            })?;

        contributions.push(ScoreContribution::new(
            signal,
            value,
            weight,
            contribution,
        ));

        Ok(())
    }

    fn compare_evaluations(
        &self,
        left: &HybridCandidate,
        left_evaluation: &HybridEvaluation,
        right: &HybridCandidate,
        right_evaluation: &HybridEvaluation,
    ) -> Ordering {
        // Legality is an absolute correctness boundary.
        match (left_evaluation.legal(), right_evaluation.legal()) {
            (true, false) => return Ordering::Greater,
            (false, true) => return Ordering::Less,
            (false, false) => {
                return self.compare_legal_tie_breakers(
                    left,
                    right,
                );
            }
            (true, true) => {}
        }

        // Higher hybrid score is preferred.
        let score_order = left_evaluation
            .score()
            .cmp(&right_evaluation.score());

        if score_order != Ordering::Equal {
            return score_order;
        }

        self.compare_legal_tie_breakers(left, right)
    }

    fn compare_legal_tie_breakers(
        &self,
        left: &HybridCandidate,
        right: &HybridCandidate,
    ) -> Ordering {
        let left_signals = left.signals();
        let right_signals = right.signals();

        // The configured tie-break rule is respected first.
        let configured = match self.tie_break {
            TieBreakRule::EarliestStart => left
                .earliest_start()
                .cmp(&right.earliest_start())
                .reverse(),

            TieBreakRule::Priority => left_signals
                .priority()
                .cmp(&right_signals.priority()),

            TieBreakRule::Criticality => left_signals
                .criticality()
                .cmp(&right_signals.criticality()),

            TieBreakRule::OperationId => {
                right.operation().cmp(&left.operation())
            }

            TieBreakRule::SourceOrder => self.compare_source_order(left, right),

            TieBreakRule::ResourceFootprint => left_signals
                .resource_footprint()
                .cmp(&right_signals.resource_footprint()),

            TieBreakRule::DeterministicDefault => Ordering::Equal,
        };

        if configured != Ordering::Equal {
            return configured;
        }

        // The deterministic default is intentionally lexicographic and
        // independent of hash-map iteration or thread timing.
        //
        // Higher urgency wins.
        let deadline = left_signals
            .deadline()
            .cmp(&right_signals.deadline());

        if deadline != Ordering::Equal {
            return deadline;
        }

        let criticality = left_signals
            .criticality()
            .cmp(&right_signals.criticality());

        if criticality != Ordering::Equal {
            return criticality;
        }

        let resource_pressure = left_signals
            .resource_pressure()
            .cmp(&right_signals.resource_pressure());

        if resource_pressure != Ordering::Equal {
            return resource_pressure;
        }

        let priority = left_signals
            .priority()
            .cmp(&right_signals.priority());

        if priority != Ordering::Equal {
            return priority;
        }

        // Earlier start is preferred.
        let earliest_start = right
            .earliest_start()
            .cmp(&left.earliest_start());

        if earliest_start != Ordering::Equal {
            return earliest_start;
        }

        let source_order = self.compare_source_order(left, right);

        if source_order != Ordering::Equal {
            return source_order;
        }

        // Final stable total-order tie-break.
        right.operation().cmp(&left.operation())
    }

    fn compare_source_order(
        &self,
        left: &HybridCandidate,
        right: &HybridCandidate,
    ) -> Ordering {
        match (left.source_order(), right.source_order()) {
            (Some(left_order), Some(right_order)) => {
                right_order.value().cmp(&left_order.value())
            }

            (Some(_), None) => Ordering::Greater,

            (None, Some(_)) => Ordering::Less,

            (None, None) => Ordering::Equal,
        }
    }
}

// =============================================================================
// Capability metadata
// =============================================================================

/// Static capabilities of [`HybridPolicy`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HybridCapabilities {
    /// Can combine ASAP-derived information.
    pub asap: bool,

    /// Can combine ALAP-derived information.
    pub alap: bool,

    /// Can combine critical-path information.
    pub criticality: bool,

    /// Can combine explicit operation priority.
    pub priority: bool,

    /// Can combine resource pressure.
    pub resource_pressure: bool,

    /// Can combine resource footprint.
    pub resource_footprint: bool,

    /// Can combine fidelity estimates.
    pub fidelity: bool,

    /// Can combine energy estimates.
    pub energy: bool,

    /// Can combine deadline urgency.
    pub deadline: bool,

    /// Can combine communication urgency.
    pub communication: bool,

    /// Can combine QEC urgency.
    pub qec: bool,

    /// Does not contain a hardware-size limitation.
    pub target_independent: bool,

    /// Uses no floating-point scheduling semantics.
    pub exact_integer_scoring: bool,

    /// Supports deterministic evaluation.
    pub deterministic: bool,
}

impl Default for HybridCapabilities {
    fn default() -> Self {
        Self {
            asap: true,
            alap: true,
            criticality: true,
            priority: true,
            resource_pressure: true,
            resource_footprint: true,
            fidelity: true,
            energy: true,
            deadline: true,
            communication: true,
            qec: true,
            target_independent: true,
            exact_integer_scoring: true,
            deterministic: true,
        }
    }
}

impl HybridPolicy {
    /// Returns static capabilities of this policy.
    #[must_use]
    pub const fn capabilities(&self) -> HybridCapabilities {
        HybridCapabilities {
            asap: true,
            alap: true,
            criticality: true,
            priority: true,
            resource_pressure: true,
            resource_footprint: true,
            fidelity: true,
            energy: true,
            deadline: true,
            communication: true,
            qec: true,
            target_independent: true,
            exact_integer_scoring: true,
            deterministic: true,
        }
    }

    /// Returns the policy requirements that should normally be prepared by a
    /// planner before hybrid evaluation.
    ///
    /// This deliberately returns stable policy vocabulary rather than
    /// importing planner internals.
    pub fn requirements(
        &self,
    ) -> Vec<super::policy::PolicyRequirement> {
        use super::policy::PolicyRequirement;

        let mut requirements = Vec::new();

        requirements.push(PolicyRequirement::Dependencies);
        requirements.push(PolicyRequirement::ForwardTiming);
        requirements.push(PolicyRequirement::Resources);
        requirements.push(PolicyRequirement::Priorities);
        requirements.push(PolicyRequirement::CriticalPath);

        if self.determinism.is_required() {
            // Determinism is represented by the policy itself; no additional
            // requirement is needed.
        }

        match self.objective {
            SchedulingObjective::MinimizeMakespan
            | SchedulingObjective::MinimizeDepth
            | SchedulingObjective::MinimizeIdleTime
            | SchedulingObjective::MaximizeEstimatedFidelity
            | SchedulingObjective::MinimizeEnergy
            | SchedulingObjective::PreserveOrder
            | SchedulingObjective::Composite => {}
        }

        requirements
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn operation(value: u64) -> OperationId {
        OperationId::from(value)
    }

    #[test]
    fn signal_is_exact_integer_value() {
        let signal = Signal::new(42);

        assert_eq!(signal.value(), 42);
        assert!(!signal.is_zero());
        assert!(!signal.is_max());
    }

    #[test]
    fn signal_complement_is_exact() {
        let signal = Signal::new(10);
        let complement = signal.complement();

        assert_eq!(complement.value(), u64::MAX - 10);
    }

    #[test]
    fn zero_weight_disables_signal() {
        let weights = HybridWeights::new()
            .with_priority(Weight::new(100));

        let policy = HybridPolicy::new(weights);

        let signals = HybridSignals::new()
            .with_asap(Signal::MAX)
            .with_priority(Signal::new(10));

        let candidate = HybridCandidate::new(
            operation(1),
            true,
            signals,
            0,
        );

        let evaluation = policy
            .evaluate(&candidate)
            .expect("evaluation must succeed");

        assert_eq!(
            evaluation.score().value(),
            10_u128 * 100_u128
        );
    }

    #[test]
    fn weighted_score_uses_checked_integer_arithmetic() {
        let weights = HybridWeights::new()
            .with_priority(Weight::new(2))
            .with_criticality(Weight::new(3));

        let policy = HybridPolicy::new(weights);

        let signals = HybridSignals::new()
            .with_priority(Signal::new(10))
            .with_criticality(Signal::new(20));

        let candidate = HybridCandidate::new(
            operation(2),
            true,
            signals,
            0,
        );

        let evaluation = policy
            .evaluate(&candidate)
            .expect("evaluation must succeed");

        assert_eq!(
            evaluation.score().value(),
            10_u128 * 2_u128 + 20_u128 * 3_u128
        );
    }

    #[test]
    fn legal_candidate_always_beats_illegal_candidate() {
        let policy = HybridPolicy::new(
            HybridWeights::new()
                .with_priority(Weight::new(1)),
        );

        let illegal = HybridCandidate::new(
            operation(1),
            false,
            HybridSignals::new()
                .with_priority(Signal::MAX),
            0,
        );

        let legal = HybridCandidate::new(
            operation(2),
            true,
            HybridSignals::new()
                .with_priority(Signal::MIN),
            0,
        );

        assert_eq!(
            policy
                .compare(&legal, &illegal)
                .expect("comparison must succeed"),
            Ordering::Greater
        );
    }

    #[test]
    fn higher_score_is_preferred() {
        let policy = HybridPolicy::new(
            HybridWeights::new()
                .with_priority(Weight::new(1)),
        );

        let lower = HybridCandidate::new(
            operation(1),
            true,
            HybridSignals::new()
                .with_priority(Signal::new(10)),
            0,
        );

        let higher = HybridCandidate::new(
            operation(2),
            true,
            HybridSignals::new()
                .with_priority(Signal::new(20)),
            0,
        );

        assert!(
            policy
                .prefers(&higher, &lower)
                .expect("comparison must succeed")
        );
    }

    #[test]
    fn final_operation_id_tie_break_is_deterministic() {
        let policy = HybridPolicy::new(HybridWeights::new());

        let first = HybridCandidate::new(
            operation(1),
            true,
            HybridSignals::new(),
            0,
        );

        let second = HybridCandidate::new(
            operation(2),
            true,
            HybridSignals::new(),
            0,
        );

        assert_eq!(
            policy
                .compare(&first, &second)
                .expect("comparison must succeed"),
            Ordering::Greater
        );
    }

    #[test]
    fn earliest_start_tie_break_prefers_earlier_candidate() {
        let policy = HybridPolicy::new(
            HybridWeights::new(),
        )
        .with_tie_break(TieBreakRule::EarliestStart);

        let early = HybridCandidate::new(
            operation(1),
            true,
            HybridSignals::new(),
            10,
        );

        let late = HybridCandidate::new(
            operation(2),
            true,
            HybridSignals::new(),
            20,
        );

        assert!(
            policy
                .prefers(&early, &late)
                .expect("comparison must succeed")
        );
    }

    #[test]
    fn source_order_is_supported() {
        let policy = HybridPolicy::new(HybridWeights::new())
            .with_tie_break(TieBreakRule::SourceOrder);

        let first = HybridCandidate::new(
            operation(1),
            true,
            HybridSignals::new(),
            0,
        )
        .with_source_order(SourceOrder::new(1));

        let second = HybridCandidate::new(
            operation(2),
            true,
            HybridSignals::new(),
            0,
        )
        .with_source_order(SourceOrder::new(2));

        assert!(
            policy
                .prefers(&first, &second)
                .expect("comparison must succeed")
        );
    }

    #[test]
    fn select_does_not_require_materializing_all_candidates() {
        let policy = HybridPolicy::new(
            HybridWeights::new()
                .with_priority(Weight::new(1)),
        );

        let candidates = [
            HybridCandidate::new(
                operation(1),
                true,
                HybridSignals::new()
                    .with_priority(Signal::new(1)),
                0,
            ),
            HybridCandidate::new(
                operation(2),
                true,
                HybridSignals::new()
                    .with_priority(Signal::new(100)),
                0,
            ),
            HybridCandidate::new(
                operation(3),
                true,
                HybridSignals::new()
                    .with_priority(Signal::new(50)),
                0,
            ),
        ];

        let selected = policy
            .select(candidates.iter())
            .expect("a legal candidate must exist");

        assert_eq!(selected.operation(), operation(2));
    }

    #[test]
    fn select_rejects_all_illegal_candidates() {
        let policy = HybridPolicy::new(HybridWeights::new());

        let candidates = [
            HybridCandidate::new(
                operation(1),
                false,
                HybridSignals::new(),
                0,
            ),
            HybridCandidate::new(
                operation(2),
                false,
                HybridSignals::new(),
                0,
            ),
        ];

        assert_eq!(
            policy.select(candidates.iter()),
            Err(HybridError::NoLegalCandidate)
        );
    }

    #[test]
    fn select_rejects_empty_input() {
        let policy = HybridPolicy::new(HybridWeights::new());

        let candidates: [HybridCandidate; 0] = [];

        assert_eq!(
            policy.select(candidates.iter()),
            Err(HybridError::EmptyCandidateSet)
        );
    }

    #[test]
    fn unique_selection_rejects_duplicate_operations() {
        let policy = HybridPolicy::new(HybridWeights::new());

        let candidates = [
            HybridCandidate::new(
                operation(1),
                true,
                HybridSignals::new(),
                0,
            ),
            HybridCandidate::new(
                operation(1),
                true,
                HybridSignals::new(),
                1,
            ),
        ];

        assert_eq!(
            policy.select_unique(candidates.iter()),
            Err(HybridError::DuplicateOperation {
                operation: operation(1),
            })
        );
    }

    #[test]
    fn canonical_hybrid_kind_is_recognized() {
        assert!(
            HybridPolicy::is_hybrid_kind(
                SchedulingPolicyKind::CriticalPathResourceAware
            )
        );

        assert!(
            HybridPolicy::is_hybrid_kind(
                SchedulingPolicyKind::Adaptive
            )
        );

        assert!(
            !HybridPolicy::is_hybrid_kind(
                SchedulingPolicyKind::AsSoonAsPossible
            )
        );
    }

    #[test]
    fn critical_path_resource_policy_can_construct_hybrid_policy() {
        let descriptor = SchedulingPolicy::new(
            SchedulingPolicyKind::CriticalPathResourceAware,
        );

        let policy = HybridPolicy::from_policy(&descriptor)
            .expect("hybrid policy must be accepted");

        assert_eq!(policy.name(), "hybrid");
        assert!(policy.deterministic());
    }

    #[test]
    fn non_hybrid_policy_is_rejected() {
        let descriptor = SchedulingPolicy::new(
            SchedulingPolicyKind::AsSoonAsPossible,
        );

        assert_eq!(
            HybridPolicy::from_policy(&descriptor),
            Err(HybridError::InvalidPolicy {
                reason:
                    "the supplied SchedulingPolicyKind is not a hybrid policy kind",
            })
        );
    }

    #[test]
    fn all_zero_weights_are_valid() {
        let policy = HybridPolicy::new(HybridWeights::new());

        policy
            .validate()
            .expect("all-zero weights remain a valid deterministic policy");
    }

    #[test]
    fn candidate_preserves_canonical_operation_identity() {
        let candidate = HybridCandidate::new(
            operation(123),
            true,
            HybridSignals::new(),
            0,
        );

        assert_eq!(candidate.operation(), operation(123));
    }
}