//! Zamani Quantum Noise (ZQN) — Routing Integration.
//!
//! `src/quantum/zqn/integration/routing.rs`
//!
//! # Purpose
//!
//! This module is the formal boundary between the canonical ZQN physical-noise
//! model and Zamani's quantum routing subsystem.
//!
//! It answers:
//!
//! > "Given a candidate routing operation and its logical/physical resources,
//! > what deterministic, backend-independent physical-noise information can
//! > routing use when selecting a placement or movement?"
//!
//! ZQN remains the owner of:
//!
//! - noise semantics;
//! - channels;
//! - faults;
//! - correlations;
//! - calibration;
//! - characterization;
//! - uncertainty;
//! - temporal noise;
//! - crosstalk;
//! - physical error models.
//!
//! Routing remains the owner of:
//!
//! - logical-to-physical placement;
//! - topology traversal;
//! - path finding;
//! - SWAP selection;
//! - movement selection;
//! - routing algorithms;
//! - candidate generation;
//! - layout mutation;
//! - routing verification;
//! - route construction.
//!
//! This file owns only the integration contract between those domains.
//!
//! # Architectural position
//!
//! ```text
//!                 canonical Quantum IR
//!                         │
//!                         │ OperationId
//!                         │ QubitId
//!                         │ PhysicalQubitId
//!                         ▼
//!                    ZQN integration
//!                  integration/routing.rs
//!                         │
//!                         │ RoutingNoiseRequest
//!                         ▼
//!                 RoutingNoiseProvider
//!                         │
//!                         ▼
//!                  RoutingNoiseEstimate
//!                         │
//!             ┌───────────┼────────────┐
//!             ▼           ▼            ▼
//!          routing      scheduling     analysis
//!             │
//!             ▼
//!       noise-aware routing
//! ```
//!
//! # Critical dependency rule
//!
//! This module MUST NOT import:
//!
//! - a concrete routing algorithm;
//! - SABRE implementation;
//! - topology storage implementation;
//! - mapping implementation;
//! - router implementation;
//! - hardware provider;
//! - vendor SDK;
//! - QPU transport;
//! - simulator implementation;
//! - QEC decoder;
//! - frontend AST.
//!
//! The integration boundary is intentionally based on stable data contracts
//! and a provider trait.
//!
//! # Canonical identities
//!
//! ZQN MUST use the canonical Quantum IR identity types:
//!
//! ```text
//! crate::quantum::ir::identity::OperationId
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This file therefore MUST NOT define another `QubitId`,
//! `PhysicalQubitId`, or `OperationId`.
//!
//! The distinction between logical and physical identity is preserved at the
//! type level.
//!
//! # Write once, scale everywhere
//!
//! There is deliberately no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_OPERATIONS
//! MAX_ROUTE_LENGTH
//! MAX_CORRELATED_QUBITS
//! MAX_DEVICES
//! ```
//!
//! in this file.
//!
//! A routing request may contain any number of operands and resource
//! references representable by the selected storage and permitted by the
//! caller's resource policy.
//!
//! "Infinity" means that the semantic contract does not impose an artificial
//! finite machine-size ceiling. Actual execution remains bounded by available
//! memory, compute, storage, topology, target capabilities and explicit
//! runtime resource policies.
//!
//! # Why this file does not directly use `f64`
//!
//! Routing decisions must be deterministic.
//!
//! Raw floating-point values introduce:
//!
//! - NaN ordering problems;
//! - infinity handling;
//! - platform-dependent edge cases;
//! - accidental invalid values;
//! - ambiguous comparison semantics.
//!
//! Therefore this integration boundary represents probabilities/fidelities as
//! parts-per-billion (`u64`) and accumulated scores as checked `u128` values.
//!
//! A ZQN implementation may internally use arbitrary precision or floating
//! point, but conversion into this routing contract MUST be explicit and
//! validated.
//!
//! # Important semantic distinction
//!
//! A routing noise estimate is NOT itself a noise model.
//!
//! ```text
//! NoiseModel
//!     describes physical uncertainty.
//!
//! RoutingNoiseEstimate
//!     is a routing-oriented projection of that uncertainty.
//!
//! RoutingCost
//!     is the deterministic numerical value used for candidate comparison.
//! ```
//!
//! This prevents routing from becoming a second owner of physical-noise
//! semantics.
//!
//! # Approximation policy
//!
//! A provider MUST explicitly declare whether an estimate is:
//!
//! - exact;
//! - approximate;
//! - bounded;
//! - statistical;
//! - unavailable.
//!
//! Routing MUST NOT silently treat an approximation as exact.
//!
//! # Unknown information
//!
//! Unknown calibration/noise information is represented explicitly.
//!
//! Unknown does not mean:
//!
//! ```text
//! zero error
//! ```
//!
//! nor:
//!
//! ```text
//! perfect fidelity
//! ```
//!
//! A caller chooses how unknown information is handled through
//! `UnknownNoisePolicy`.
//!
//! This prevents missing calibration data from accidentally producing an
//! unrealistically attractive route.
//!
//! # Determinism
//!
//! This module:
//!
//! - has no global RNG;
//! - has no hidden mutable state;
//! - has no wall-clock dependency;
//! - has no memory-address dependency;
//! - does not depend on hash-map iteration order;
//! - uses ordered collections where deterministic aggregation is required;
//! - preserves explicit operation and resource identities.
//!
//! The same request and provider semantics must produce the same estimate.
//!
//! # Parallelism
//!
//! Providers may be shared between routing workers when their implementation
//! is `Send + Sync`.
//!
//! The contract does not require a provider to maintain mutable global state.
//!
//! Parallel routing must not change the semantic result of a deterministic
//! provider.
//!
//! # Resource safety
//!
//! This file does not impose a semantic machine-size limit.
//!
//! It does, however, support explicit request limits through
//! `RoutingNoisePolicy`.
//!
//! Limits are safety/resource policies, not quantum-computing limits.
//!
//! A policy may restrict:
//!
//! - number of resources materialized in one request;
//! - number of accumulated components;
//! - maximum score magnitude;
//! - whether unknown values are accepted.
//!
//! `None` means that this integration layer imposes no limit for that field.
//!
//! # Security
//!
//! This integration contract grants no hardware capabilities.
//!
//! A `RoutingNoiseProvider` does not automatically receive:
//!
//! - QPU credentials;
//! - network access;
//! - filesystem access;
//! - calibration write access;
//! - process execution;
//! - hardware control.
//!
//! Such capabilities remain controlled by the runtime/security architecture.
//!
//! Untrusted noise data MUST be validated before it is projected into routing
//! costs.
//!
//! # Integration with existing routing
//!
//! The repository's routing subsystem already defines a routing-level
//! `NoiseModel` used by the noise-aware routing algorithm. That existing
//! contract operates on `RoutingOperation` and produces a routing-level
//! `NoiseEstimate`. The ZQN boundary here deliberately does not depend on that
//! algorithm implementation, because doing so would reverse the desired
//! dependency direction.
//!
//! The intended adapter is:
//!
//! ```text
//! ZQN RoutingNoiseProvider
//!         │
//!         ▼
//! RoutingNoiseEstimate
//!         │
//!         ▼
//! routing::algorithms::noise_aware::NoiseModel adapter
//!         │
//!         ▼
//! existing noise-aware router
//! ```
//!
//! The adapter belongs in the routing subsystem or a dedicated routing
//! compatibility layer. This file remains stable while routing algorithms
//! evolve.
//!
//! # Integration with canonical IR
//!
//! The caller obtains `OperationId`, `QubitId`, and `PhysicalQubitId` from the
//! canonical Quantum IR and places them into `RoutingNoiseRequest`.
//!
//! This file does not inspect or depend on a particular IR operation enum.
//!
//! That is intentional: future IR operation kinds must not require this file
//! to be rewritten.
//!
//! # Integration with calibration
//!
//! A ZQN provider may internally consult:
//!
//! ```text
//! calibration::snapshot
//! calibration::parameter
//! calibration::gate
//! calibration::readout
//! calibration::drift
//! ```
//!
//! This file does not depend on those concrete implementations.
//!
//! Calibration identity may be exposed through the provider's returned
//! provenance metadata.
//!
//! # Integration with characterization
//!
//! Characterization may produce the data consumed by a provider.
//!
//! ```text
//! hardware observations
//!        │
//!        ▼
//! characterization
//!        │
//!        ▼
//! calibrated noise model
//!        │
//!        ▼
//! RoutingNoiseProvider
//! ```
//!
//! # Integration with scheduling
//!
//! Routing should primarily use spatial/operation-level noise information.
//!
//! Scheduling may additionally consume the same ZQN model for duration and
//! temporal noise.
//!
//! This file therefore permits duration and temporal context in the request
//! without making scheduling a dependency of routing.
//!
//! # Integration with QEC
//!
//! ZQN remains the canonical physical-noise source.
//!
//! QEC may use the same underlying model, but this routing integration must not
//! import QEC types.
//!
//! # Integration with hardware
//!
//! Hardware adapters provide abstract capabilities/calibration/observations.
//!
//! They do not become dependencies of this file.
//!
//! # Integration with benchmarking
//!
//! Benchmarking may record routing-noise estimates for later analysis.
//!
//! The benchmark subsystem remains the owner of benchmark methodology.
//!
//! # Serialization
//!
//! This file does not define a wire format.
//!
//! The ZQN IO subsystem owns serialization.
//!
//! Any serialized routing-noise request/result MUST preserve:
//!
//! - identity domain;
//! - operation identity;
//! - logical resource identities;
//! - physical resource identities;
//! - semantic guarantee;
//! - uncertainty;
//! - provenance;
//! - cost components;
//! - policy where policy is part of reproducibility.
//!
//! Rust memory layout is not a serialization contract.
//!
//! # Thread safety
//!
//! All value types in this file are immutable after construction.
//!
//! `RoutingNoiseProvider` is `Send + Sync` so implementations can be safely
//! shared by concurrent routing workers when their implementation satisfies
//! those requirements.
//!
//! # Rust compatibility
//!
//! Target:
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
//! 1. canonical IR identity types are used;
//! 2. no ZQN-specific qubit identity exists;
//! 3. no machine-size ceiling exists;
//! 4. routing can consume physical-noise projections without owning noise
//!    semantics;
//! 5. unknown calibration is explicit;
//! 6. approximation is explicit;
//! 7. numeric ordering is deterministic;
//! 8. integer overflow is rejected;
//! 9. arbitrary operand counts are supported;
//! 10. providers are independent of concrete routing algorithms;
//! 11. the contract can be consumed by noise-aware routing;
//! 12. the same provider/request produces the same result;
//! 13. no hidden global state exists;
//! 14. no unsafe code exists;
//! 15. future noise models can be added without changing this file;
//! 16. future routing algorithms can be added without changing this file;
//! 17. hardware vendors do not require changes to this file.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;
use std::collections::BTreeSet;
use std::time::Duration;

use crate::quantum::ir::identity::OperationId;
use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::zqn::core::errors::{
    ZqnError,
    ZqnErrorCode,
    ZqnErrorKind,
    ZqnResult,
};

// ============================================================================
// Constants
// ============================================================================

/// Parts-per-billion scale.
///
/// `1_000_000_000` represents one whole probability/fidelity value.
pub const PROBABILITY_SCALE: u64 = 1_000_000_000;

/// Maximum valid probability/fidelity representation.
pub const MAX_PROBABILITY_PPB: u64 = PROBABILITY_SCALE;

/// Zero probability.
pub const ZERO_PROBABILITY_PPB: u64 = 0;

/// Perfect fidelity.
pub const PERFECT_FIDELITY_PPB: u64 = PROBABILITY_SCALE;

// ============================================================================
// Error helpers
// ============================================================================

fn integration_error(
    code: ZqnErrorCode,
    message: &'static str,
) -> ZqnError {
    ZqnError::new(
        ZqnErrorKind::Integration,
        code,
        message,
    )
}

fn invariant_error(message: &'static str) -> ZqnError {
    ZqnError::new(
        ZqnErrorKind::Invariant,
        ZqnErrorCode::InvalidNoiseApplication,
        message,
    )
}

// ============================================================================
// Semantic guarantee
// ============================================================================

/// Fidelity guarantee of a routing-noise estimate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RoutingNoiseGuarantee {
    /// The estimate represents the requested semantics exactly.
    Exact,

    /// The estimate is an explicit approximation.
    Approximate,

    /// The estimate is associated with an explicit mathematical bound.
    Bounded,

    /// The estimate is statistically inferred.
    Statistical,

    /// No reliable estimate is currently available.
    Unknown,
}

impl RoutingNoiseGuarantee {
    /// Returns whether the estimate represents exact semantics.
    #[must_use]
    pub const fn is_exact(self) -> bool {
        matches!(self, Self::Exact)
    }

    /// Returns whether the estimate is explicitly approximate.
    #[must_use]
    pub const fn is_approximate(self) -> bool {
        matches!(self, Self::Approximate)
    }

    /// Returns whether the estimate carries a bound.
    #[must_use]
    pub const fn is_bounded(self) -> bool {
        matches!(self, Self::Bounded)
    }

    /// Returns whether the estimate is statistical.
    #[must_use]
    pub const fn is_statistical(self) -> bool {
        matches!(self, Self::Statistical)
    }

    /// Returns whether the estimate is unknown.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

impl fmt::Display for RoutingNoiseGuarantee {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exact => formatter.write_str("exact"),
            Self::Approximate => formatter.write_str("approximate"),
            Self::Bounded => formatter.write_str("bounded"),
            Self::Statistical => formatter.write_str("statistical"),
            Self::Unknown => formatter.write_str("unknown"),
        }
    }
}

// ============================================================================
// Unknown-data policy
// ============================================================================

/// Policy controlling how routing handles missing physical-noise information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UnknownNoisePolicy {
    /// Reject a request if required noise information is unavailable.
    Reject,

    /// Allow unknown information and let the routing consumer decide how to
    /// score it.
    Allow,

    /// Treat unknown information as conservative rather than optimistic.
    ///
    /// The actual conservative penalty is supplied by the caller rather than
    /// hard-coded by this integration layer.
    Conservative,
}

impl Default for UnknownNoisePolicy {
    fn default() -> Self {
        Self::Conservative
    }
}

// ============================================================================
// Resource identity
// ============================================================================

/// Explicit logical/physical resource identity.
///
/// The enum prevents a logical qubit identifier from being silently consumed
/// as a physical hardware identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RoutingNoiseResource {
    /// Logical Quantum IR qubit.
    LogicalQubit(QubitId),

    /// Physical Quantum IR qubit.
    PhysicalQubit(PhysicalQubitId),
}

impl RoutingNoiseResource {
    /// Creates a logical resource reference.
    #[must_use]
    pub const fn logical(qubit: QubitId) -> Self {
        Self::LogicalQubit(qubit)
    }

    /// Creates a physical resource reference.
    #[must_use]
    pub const fn physical(qubit: PhysicalQubitId) -> Self {
        Self::PhysicalQubit(qubit)
    }

    /// Returns the logical identifier when this is a logical resource.
    #[must_use]
    pub const fn logical_id(self) -> Option<QubitId> {
        match self {
            Self::LogicalQubit(id) => Some(id),
            Self::PhysicalQubit(_) => None,
        }
    }

    /// Returns the physical identifier when this is a physical resource.
    #[must_use]
    pub const fn physical_id(self) -> Option<PhysicalQubitId> {
        match self {
            Self::PhysicalQubit(id) => Some(id),
            Self::LogicalQubit(_) => None,
        }
    }

    /// Returns whether this is a logical resource.
    #[must_use]
    pub const fn is_logical(self) -> bool {
        matches!(self, Self::LogicalQubit(_))
    }

    /// Returns whether this is a physical resource.
    #[must_use]
    pub const fn is_physical(self) -> bool {
        matches!(self, Self::PhysicalQubit(_))
    }
}

// ============================================================================
// Routing operation identity
// ============================================================================

/// Stable identity of a routing candidate operation.
///
/// `OperationId` remains owned by the canonical Quantum IR.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RoutingNoiseOperation {
    operation: OperationId,
}

impl RoutingNoiseOperation {
    /// Creates a routing-noise operation reference.
    #[must_use]
    pub const fn new(operation: OperationId) -> Self {
        Self { operation }
    }

    /// Returns the canonical IR operation identity.
    #[must_use]
    pub const fn operation_id(self) -> OperationId {
        self.operation
    }
}

// ============================================================================
// Routing noise request
// ============================================================================

/// Immutable request for a routing-oriented ZQN estimate.
///
/// The request contains semantic identities and context only. It does not
/// contain a routing algorithm or routing state.
///
/// # Arbitrary arity
///
/// `operands` is a vector rather than a pair or fixed-size array so that the
/// contract can represent:
///
/// - unary operations;
/// - binary operations;
/// - multi-body interactions;
/// - future non-gate quantum operations.
///
/// The number of operands is data, not architecture.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutingNoiseRequest {
    operation: RoutingNoiseOperation,
    operands: Vec<RoutingNoiseResource>,
    logical_operands: Vec<QubitId>,
    physical_operands: Vec<PhysicalQubitId>,
    duration: Option<Duration>,
}

impl RoutingNoiseRequest {
    /// Creates a request from a routing operation and resource list.
    ///
    /// The constructor performs structural validation but does not require
    /// logical and physical operands to have equal cardinality because routing
    /// may be evaluating a partially specified candidate.
    pub fn new(
        operation: OperationId,
        operands: Vec<RoutingNoiseResource>,
    ) -> ZqnResult<Self> {
        let mut logical = Vec::new();
        let mut physical = Vec::new();

        for operand in &operands {
            match *operand {
                RoutingNoiseResource::LogicalQubit(id) => logical.push(id),
                RoutingNoiseResource::PhysicalQubit(id) => physical.push(id),
            }
        }

        Self::from_parts(
            RoutingNoiseOperation::new(operation),
            operands,
            logical,
            physical,
            None,
        )
    }

    /// Creates a request with explicit logical and physical projections.
    pub fn from_parts(
        operation: RoutingNoiseOperation,
        operands: Vec<RoutingNoiseResource>,
        logical_operands: Vec<QubitId>,
        physical_operands: Vec<PhysicalQubitId>,
        duration: Option<Duration>,
    ) -> ZqnResult<Self> {
        let request = Self {
            operation,
            operands,
            logical_operands,
            physical_operands,
            duration,
        };

        request.validate()?;
        Ok(request)
    }

    /// Returns the operation identity.
    #[must_use]
    pub const fn operation(&self) -> RoutingNoiseOperation {
        self.operation
    }

    /// Returns the canonical operation identity.
    #[must_use]
    pub const fn operation_id(&self) -> OperationId {
        self.operation.operation_id()
    }

    /// Returns all resource operands.
    #[must_use]
    pub fn operands(&self) -> &[RoutingNoiseResource] {
        &self.operands
    }

    /// Returns logical operands.
    #[must_use]
    pub fn logical_operands(&self) -> &[QubitId] {
        &self.logical_operands
    }

    /// Returns physical operands.
    #[must_use]
    pub fn physical_operands(&self) -> &[PhysicalQubitId] {
        &self.physical_operands
    }

    /// Returns optional physical duration.
    #[must_use]
    pub const fn duration(&self) -> Option<Duration> {
        self.duration
    }

    /// Sets a physical-duration context without changing operation identity.
    pub fn with_duration(
        mut self,
        duration: Duration,
    ) -> ZqnResult<Self> {
        self.duration = Some(duration);
        self.validate()?;
        Ok(self)
    }

    /// Validates the structural request invariants.
    pub fn validate(&self) -> ZqnResult<()> {
        let mut logical = BTreeSet::new();
        let mut physical = BTreeSet::new();

        for resource in &self.operands {
            match *resource {
                RoutingNoiseResource::LogicalQubit(id) => {
                    logical.insert(id);
                }
                RoutingNoiseResource::PhysicalQubit(id) => {
                    physical.insert(id);
                }
            }
        }

        if logical.len() != self.logical_operands.len() {
            return Err(integration_error(
                ZqnErrorCode::DuplicateIdentifier,
                "routing-noise logical operand projection contains duplicates",
            ));
        }

        if physical.len() != self.physical_operands.len() {
            return Err(integration_error(
                ZqnErrorCode::DuplicateIdentifier,
                "routing-noise physical operand projection contains duplicates",
            ));
        }

        Ok(())
    }
}

// ============================================================================
// Probability / fidelity representation
// ============================================================================

/// Validated probability represented in parts per billion.
///
/// This type deliberately does not use floating point.
///
/// ```text
/// 0              = 0%
/// 500_000_000    = 50%
/// 1_000_000_000  = 100%
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProbabilityPpb(u64);

impl ProbabilityPpb {
    /// Creates a probability from parts-per-billion units.
    pub const fn new(value: u64) -> ZqnResult<Self> {
        if value > MAX_PROBABILITY_PPB {
            return Err(ZqnError::new(
                ZqnErrorKind::Integration,
                ZqnErrorCode::InvalidProbability,
                "probability exceeds the [0, 1] domain",
            ));
        }

        Ok(Self(value))
    }

    /// Creates zero probability.
    #[must_use]
    pub const fn zero() -> Self {
        Self(0)
    }

    /// Creates unit probability.
    #[must_use]
    pub const fn one() -> Self {
        Self(MAX_PROBABILITY_PPB)
    }

    /// Returns the raw parts-per-billion value.
    #[must_use]
    pub const fn ppb(self) -> u64 {
        self.0
    }

    /// Returns the complement.
    #[must_use]
    pub const fn complement(self) -> Self {
        Self(MAX_PROBABILITY_PPB - self.0)
    }
}

impl fmt::Display for ProbabilityPpb {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} ppb", self.0)
    }
}

/// Validated fidelity represented in parts per billion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FidelityPpb(u64);

impl FidelityPpb {
    /// Creates fidelity from parts-per-billion units.
    pub const fn new(value: u64) -> ZqnResult<Self> {
        if value > MAX_PROBABILITY_PPB {
            return Err(ZqnError::new(
                ZqnErrorKind::Integration,
                ZqnErrorCode::InvalidProbability,
                "fidelity exceeds the [0, 1] domain",
            ));
        }

        Ok(Self(value))
    }

    /// Creates zero fidelity.
    #[must_use]
    pub const fn zero() -> Self {
        Self(0)
    }

    /// Creates perfect fidelity.
    #[must_use]
    pub const fn one() -> Self {
        Self(MAX_PROBABILITY_PPB)
    }

    /// Returns the raw parts-per-billion value.
    #[must_use]
    pub const fn ppb(self) -> u64 {
        self.0
    }
}

impl fmt::Display for FidelityPpb {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} ppb", self.0)
    }
}

// ============================================================================
// Noise cost components
// ============================================================================

/// Individual physical-noise contribution relevant to routing.
///
/// The enum is intentionally technology-neutral.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RoutingNoiseComponent {
    /// Gate/process error probability.
    OperationError,

    /// Idle/decoherence error contribution.
    IdleError,

    /// Readout/measurement error contribution.
    ReadoutError,

    /// Preparation/reset error contribution.
    PreparationError,

    /// Transport/movement error contribution.
    TransportError,

    /// Crosstalk contribution.
    CrosstalkError,

    /// Correlated-noise contribution.
    CorrelationError,

    /// Calibration uncertainty contribution.
    CalibrationUncertainty,

    /// Temporal/drift contribution.
    TemporalUncertainty,

    /// Technology-specific physical contribution represented through the
    /// generic routing-noise boundary.
    Other,
}

impl fmt::Display for RoutingNoiseComponent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::OperationError => "operation_error",
            Self::IdleError => "idle_error",
            Self::ReadoutError => "readout_error",
            Self::PreparationError => "preparation_error",
            Self::TransportError => "transport_error",
            Self::CrosstalkError => "crosstalk_error",
            Self::CorrelationError => "correlation_error",
            Self::CalibrationUncertainty => "calibration_uncertainty",
            Self::TemporalUncertainty => "temporal_uncertainty",
            Self::Other => "other",
        };

        formatter.write_str(value)
    }
}

/// A single immutable routing-noise contribution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RoutingNoiseContribution {
    component: RoutingNoiseComponent,
    probability: ProbabilityPpb,
}

impl RoutingNoiseContribution {
    /// Creates a contribution.
    #[must_use]
    pub const fn new(
        component: RoutingNoiseComponent,
        probability: ProbabilityPpb,
    ) -> Self {
        Self {
            component,
            probability,
        }
    }

    /// Returns the contribution category.
    #[must_use]
    pub const fn component(self) -> RoutingNoiseComponent {
        self.component
    }

    /// Returns the contribution probability.
    #[must_use]
    pub const fn probability(self) -> ProbabilityPpb {
        self.probability
    }
}

// ============================================================================
// Routing noise estimate
// ============================================================================

/// Immutable physical-noise estimate projected into routing semantics.
///
/// The estimate is deliberately richer than one scalar error probability so
/// that routing may choose an objective appropriate to the compilation
/// problem.
///
/// The estimate does NOT decide which objective routing should optimize.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutingNoiseEstimate {
    guarantee: RoutingNoiseGuarantee,
    error_probability: ProbabilityPpb,
    fidelity: FidelityPpb,
    duration: Duration,
    contributions: Vec<RoutingNoiseContribution>,
    unknown: bool,
}

impl RoutingNoiseEstimate {
    /// Creates a fully specified estimate.
    pub fn new(
        guarantee: RoutingNoiseGuarantee,
        error_probability: ProbabilityPpb,
        fidelity: FidelityPpb,
        duration: Duration,
        contributions: Vec<RoutingNoiseContribution>,
    ) -> ZqnResult<Self> {
        let estimate = Self {
            guarantee,
            error_probability,
            fidelity,
            duration,
            contributions,
            unknown: guarantee.is_unknown(),
        };

        estimate.validate()?;
        Ok(estimate)
    }

    /// Creates an explicitly unknown estimate.
    ///
    /// Unknown is represented with zero numerical error/fidelity values because
    /// those numbers are not intended to be consumed as if they were actual
    /// physical measurements. Consumers MUST inspect `guarantee`/`is_unknown`.
    #[must_use]
    pub fn unknown() -> Self {
        Self {
            guarantee: RoutingNoiseGuarantee::Unknown,
            error_probability: ProbabilityPpb::zero(),
            fidelity: FidelityPpb::zero(),
            duration: Duration::ZERO,
            contributions: Vec::new(),
            unknown: true,
        }
    }

    /// Returns the semantic guarantee.
    #[must_use]
    pub const fn guarantee(&self) -> RoutingNoiseGuarantee {
        self.guarantee
    }

    /// Returns the estimated aggregate error probability.
    #[must_use]
    pub const fn error_probability(&self) -> ProbabilityPpb {
        self.error_probability
    }

    /// Returns the estimated fidelity.
    #[must_use]
    pub const fn fidelity(&self) -> FidelityPpb {
        self.fidelity
    }

    /// Returns estimated physical duration.
    #[must_use]
    pub const fn duration(&self) -> Duration {
        self.duration
    }

    /// Returns the individual physical-noise contributions.
    #[must_use]
    pub fn contributions(&self) -> &[RoutingNoiseContribution] {
        &self.contributions
    }

    /// Returns whether the estimate is unknown.
    #[must_use]
    pub const fn is_unknown(&self) -> bool {
        self.unknown
    }

    /// Validates the estimate.
    pub fn validate(&self) -> ZqnResult<()> {
        if self.unknown && !self.guarantee.is_unknown() {
            return Err(invariant_error(
                "unknown routing-noise state must use the Unknown guarantee",
            ));
        }

        if !self.unknown && self.guarantee.is_unknown() {
            return Err(invariant_error(
                "known routing-noise state cannot use the Unknown guarantee",
            ));
        }

        for contribution in &self.contributions {
            if contribution.probability().ppb() > MAX_PROBABILITY_PPB {
                return Err(ZqnError::new(
                    ZqnErrorKind::Integration,
                    ZqnErrorCode::InvalidProbability,
                    "routing-noise contribution probability is out of range",
                ));
            }
        }

        Ok(())
    }
}

// ============================================================================
// Routing noise policy
// ============================================================================

/// Explicit safety and interpretation policy for routing-noise integration.
///
/// These are runtime/resource policies, not semantic machine-size limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoutingNoisePolicy {
    /// Optional maximum number of resource operands materialized in one
    /// request.
    pub max_operands: Option<u64>,

    /// Optional maximum number of noise contributions materialized in one
    /// estimate.
    pub max_contributions: Option<u64>,

    /// Policy for unknown noise information.
    pub unknown_policy: UnknownNoisePolicy,

    /// Whether approximate estimates are permitted.
    pub allow_approximate: bool,

    /// Whether statistical estimates are permitted.
    pub allow_statistical: bool,
}

impl Default for RoutingNoisePolicy {
    fn default() -> Self {
        Self {
            max_operands: None,
            max_contributions: None,
            unknown_policy: UnknownNoisePolicy::Conservative,
            allow_approximate: true,
            allow_statistical: true,
        }
    }
}

impl RoutingNoisePolicy {
    /// Creates an unrestricted semantic policy.
    ///
    /// "Unrestricted" means no limit imposed by this integration layer.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            max_operands: None,
            max_contributions: None,
            unknown_policy: UnknownNoisePolicy::Conservative,
            allow_approximate: true,
            allow_statistical: true,
        }
    }

    /// Validates a request against the configured resource policy.
    pub fn validate_request(
        &self,
        request: &RoutingNoiseRequest,
    ) -> ZqnResult<()> {
        if let Some(limit) = self.max_operands {
            if request.operands().len() as u128 > limit as u128 {
                return Err(ZqnError::new(
                    ZqnErrorKind::Limits,
                    ZqnErrorCode::LimitExceeded,
                    "routing-noise request exceeds the configured operand policy",
                ));
            }
        }

        Ok(())
    }

    /// Validates an estimate against the configured interpretation policy.
    pub fn validate_estimate(
        &self,
        estimate: &RoutingNoiseEstimate,
    ) -> ZqnResult<()> {
        if let Some(limit) = self.max_contributions {
            if estimate.contributions().len() as u128 > limit as u128 {
                return Err(ZqnError::new(
                    ZqnErrorKind::Limits,
                    ZqnErrorCode::LimitExceeded,
                    "routing-noise estimate exceeds the configured contribution policy",
                ));
            }
        }

        if estimate.guarantee() == RoutingNoiseGuarantee::Approximate
            && !self.allow_approximate
        {
            return Err(ZqnError::new(
                ZqnErrorKind::Compatibility,
                ZqnErrorCode::UnrepresentableChannel,
                "approximate routing-noise estimates are disabled by policy",
            ));
        }

        if estimate.guarantee() == RoutingNoiseGuarantee::Statistical
            && !self.allow_statistical
        {
            return Err(ZqnError::new(
                ZqnErrorKind::Compatibility,
                ZqnErrorCode::InsufficientObservations,
                "statistical routing-noise estimates are disabled by policy",
            ));
        }

        if estimate.is_unknown()
            && matches!(self.unknown_policy, UnknownNoisePolicy::Reject)
        {
            return Err(ZqnError::new(
                ZqnErrorKind::Compatibility,
                ZqnErrorCode::UnknownResource,
                "routing-noise information is unavailable and policy requires rejection",
            ));
        }

        Ok(())
    }
}

// ============================================================================
// Routing cost
// ============================================================================

/// Immutable deterministic routing cost projection.
///
/// This is intentionally separate from the existing routing `cost.rs`
/// implementation. ZQN supplies physical-noise information; routing's cost
/// subsystem decides how that information participates in its overall
/// objective.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RoutingNoiseCost {
    /// Aggregate error contribution.
    pub error_ppb: u64,

    /// Aggregate duration.
    pub duration_ns: u64,

    /// Aggregate crosstalk contribution.
    pub crosstalk_ppb: u64,

    /// Aggregate correlation contribution.
    pub correlation_ppb: u64,

    /// Aggregate uncertainty contribution.
    pub uncertainty_ppb: u64,
}

impl RoutingNoiseCost {
    /// Creates zero cost.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            error_ppb: 0,
            duration_ns: 0,
            crosstalk_ppb: 0,
            correlation_ppb: 0,
            uncertainty_ppb: 0,
        }
    }

    /// Converts an estimate into a routing-noise cost.
    ///
    /// Contributions are classified deterministically.
    pub fn from_estimate(
        estimate: &RoutingNoiseEstimate,
    ) -> ZqnResult<Self> {
        let mut cost = Self {
            error_ppb: estimate.error_probability().ppb(),
            duration_ns: duration_to_u64(estimate.duration())?,
            crosstalk_ppb: 0,
            correlation_ppb: 0,
            uncertainty_ppb: 0,
        };

        for contribution in estimate.contributions() {
            match contribution.component() {
                RoutingNoiseComponent::CrosstalkError => {
                    cost.crosstalk_ppb = checked_add_u64(
                        cost.crosstalk_ppb,
                        contribution.probability().ppb(),
                    )?;
                }

                RoutingNoiseComponent::CorrelationError => {
                    cost.correlation_ppb = checked_add_u64(
                        cost.correlation_ppb,
                        contribution.probability().ppb(),
                    )?;
                }

                RoutingNoiseComponent::CalibrationUncertainty
                | RoutingNoiseComponent::TemporalUncertainty => {
                    cost.uncertainty_ppb = checked_add_u64(
                        cost.uncertainty_ppb,
                        contribution.probability().ppb(),
                    )?;
                }

                RoutingNoiseComponent::OperationError
                | RoutingNoiseComponent::IdleError
                | RoutingNoiseComponent::ReadoutError
                | RoutingNoiseComponent::PreparationError
                | RoutingNoiseComponent::TransportError
                | RoutingNoiseComponent::Other => {}
            }
        }

        Ok(cost)
    }

    /// Returns a checked aggregate scalar score.
    ///
    /// The caller supplies weights so this integration layer does not
    /// hard-code a routing objective.
    pub fn weighted_score(
        self,
        weights: RoutingNoiseWeights,
    ) -> ZqnResult<u128> {
        let mut score = 0u128;

        score = checked_add_u128(
            score,
            checked_mul_u128(
                self.error_ppb as u128,
                weights.error_weight as u128,
            )?,
        )?;

        score = checked_add_u128(
            score,
            checked_mul_u128(
                self.duration_ns as u128,
                weights.duration_weight as u128,
            )?,
        )?;

        score = checked_add_u128(
            score,
            checked_mul_u128(
                self.crosstalk_ppb as u128,
                weights.crosstalk_weight as u128,
            )?,
        )?;

        score = checked_add_u128(
            score,
            checked_mul_u128(
                self.correlation_ppb as u128,
                weights.correlation_weight as u128,
            )?,
        )?;

        score = checked_add_u128(
            score,
            checked_mul_u128(
                self.uncertainty_ppb as u128,
                weights.uncertainty_weight as u128,
            )?,
        )?;

        Ok(score)
    }

    /// Adds two costs using checked arithmetic.
    pub fn checked_add(
        self,
        other: Self,
    ) -> ZqnResult<Self> {
        Ok(Self {
            error_ppb: checked_add_u64(
                self.error_ppb,
                other.error_ppb,
            )?,
            duration_ns: checked_add_u64(
                self.duration_ns,
                other.duration_ns,
            )?,
            crosstalk_ppb: checked_add_u64(
                self.crosstalk_ppb,
                other.crosstalk_ppb,
            )?,
            correlation_ppb: checked_add_u64(
                self.correlation_ppb,
                other.correlation_ppb,
            )?,
            uncertainty_ppb: checked_add_u64(
                self.uncertainty_ppb,
                other.uncertainty_ppb,
            )?,
        })
    }
}

/// Caller-controlled weights for a routing-noise objective.
///
/// No default machine-specific weighting is imposed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RoutingNoiseWeights {
    /// Weight for error probability.
    pub error_weight: u64,

    /// Weight for physical duration.
    pub duration_weight: u64,

    /// Weight for crosstalk.
    pub crosstalk_weight: u64,

    /// Weight for correlation.
    pub correlation_weight: u64,

    /// Weight for uncertainty.
    pub uncertainty_weight: u64,
}

impl RoutingNoiseWeights {
    /// Creates explicit weights.
    #[must_use]
    pub const fn new(
        error_weight: u64,
        duration_weight: u64,
        crosstalk_weight: u64,
        correlation_weight: u64,
        uncertainty_weight: u64,
    ) -> Self {
        Self {
            error_weight,
            duration_weight,
            crosstalk_weight,
            correlation_weight,
            uncertainty_weight,
        }
    }

    /// Creates an objective that considers only error.
    #[must_use]
    pub const fn error_only() -> Self {
        Self {
            error_weight: 1,
            duration_weight: 0,
            crosstalk_weight: 0,
            correlation_weight: 0,
            uncertainty_weight: 0,
        }
    }

    /// Creates an objective that considers only duration.
    #[must_use]
    pub const fn duration_only() -> Self {
        Self {
            error_weight: 0,
            duration_weight: 1,
            crosstalk_weight: 0,
            correlation_weight: 0,
            uncertainty_weight: 0,
        }
    }
}

// ============================================================================
// Provider trait
// ============================================================================

/// Stable ZQN-to-routing noise provider contract.
///
/// A provider projects the full ZQN noise semantics into information useful
/// for routing.
///
/// Implementations may be backed by:
///
/// - analytical noise models;
//! - calibration snapshots;
//! - characterization results;
//! - simulated noise;
//! - measured hardware data;
//! - composed ZQN models;
//! - technology-specific adapters.
///
/// The implementation must not change the meaning of the request based on
/// thread identity or hidden global state.
pub trait RoutingNoiseProvider: Send + Sync {
    /// Estimates physical noise for one routing candidate.
    fn estimate(
        &self,
        request: &RoutingNoiseRequest,
    ) -> ZqnResult<RoutingNoiseEstimate>;

    /// Returns a stable provider identity.
    ///
    /// This is an identity for provenance/cache correctness, not a hardware
    /// credential.
    fn provider_id(&self) -> &str;

    /// Returns the provider's semantic revision.
    fn revision(&self) -> u64 {
        1
    }
}

// ============================================================================
// Validated provider wrapper
// ============================================================================

/// Policy-enforcing wrapper around a routing-noise provider.
///
/// This type centralizes:
///
/// - request validation;
//! - resource-policy enforcement;
//! - estimate validation;
//! - unknown-data handling.
///
/// Routing algorithms therefore do not have to duplicate policy checks.
pub struct ValidatedRoutingNoiseProvider<P> {
    provider: P,
    policy: RoutingNoisePolicy,
}

impl<P> ValidatedRoutingNoiseProvider<P> {
    /// Creates a policy-enforcing provider wrapper.
    #[must_use]
    pub const fn new(
        provider: P,
        policy: RoutingNoisePolicy,
    ) -> Self {
        Self { provider, policy }
    }

    /// Returns the configured policy.
    #[must_use]
    pub const fn policy(&self) -> RoutingNoisePolicy {
        self.policy
    }

    /// Returns a reference to the underlying provider.
    #[must_use]
    pub const fn provider(&self) -> &P {
        &self.provider
    }

    /// Consumes the wrapper and returns the underlying provider.
    #[must_use]
    pub fn into_inner(self) -> P {
        self.provider
    }
}

impl<P> RoutingNoiseProvider for ValidatedRoutingNoiseProvider<P>
where
    P: RoutingNoiseProvider,
{
    fn estimate(
        &self,
        request: &RoutingNoiseRequest,
    ) -> ZqnResult<RoutingNoiseEstimate> {
        self.policy.validate_request(request)?;

        let estimate = self.provider.estimate(request)?;

        self.policy.validate_estimate(&estimate)?;

        Ok(estimate)
    }

    fn provider_id(&self) -> &str {
        self.provider.provider_id()
    }

    fn revision(&self) -> u64 {
        self.provider.revision()
    }
}

// ============================================================================
// Deterministic estimate aggregation
// ============================================================================

/// Aggregates estimates deterministically.
///
/// This is useful when a routing candidate consists of multiple physical
/// operations.
///
/// Aggregation is intentionally independent of routing algorithms.
pub fn aggregate_estimates<'a, I>(
    estimates: I,
) -> ZqnResult<RoutingNoiseEstimate>
where
    I: IntoIterator<Item = &'a RoutingNoiseEstimate>,
{
    let mut guarantee = RoutingNoiseGuarantee::Exact;
    let mut error = 0u64;
    let mut duration_ns = 0u64;
    let mut contributions = Vec::new();
    let mut unknown = false;

    for estimate in estimates {
        guarantee = combine_guarantees(
            guarantee,
            estimate.guarantee(),
        );

        error = checked_add_u64(
            error,
            estimate.error_probability().ppb(),
        )?;

        duration_ns = checked_add_u64(
            duration_ns,
            duration_to_u64(estimate.duration())?,
        )?;

        if estimate.is_unknown() {
            unknown = true;
        }

        for contribution in estimate.contributions() {
            contributions.push(*contribution);
        }
    }

    let fidelity =
        ProbabilityPpb::new(error.min(MAX_PROBABILITY_PPB))?
            .complement();

    let fidelity = FidelityPpb::new(fidelity.ppb())?;

    RoutingNoiseEstimate::new(
        if unknown {
            RoutingNoiseGuarantee::Unknown
        } else {
            guarantee
        },
        ProbabilityPpb::new(error.min(MAX_PROBABILITY_PPB))?,
        fidelity,
        Duration::from_nanos(duration_ns),
        contributions,
    )
}

fn combine_guarantees(
    left: RoutingNoiseGuarantee,
    right: RoutingNoiseGuarantee,
) -> RoutingNoiseGuarantee {
    use RoutingNoiseGuarantee::*;

    match (left, right) {
        (Unknown, _) | (_, Unknown) => Unknown,
        (Statistical, _) | (_, Statistical) => Statistical,
        (Approximate, _) | (_, Approximate) => Approximate,
        (Bounded, _) | (_, Bounded) => Bounded,
        _ => Exact,
    }
}

// ============================================================================
// Numeric helpers
// ============================================================================

fn checked_add_u64(
    left: u64,
    right: u64,
) -> ZqnResult<u64> {
    left.checked_add(right).ok_or_else(|| {
        ZqnError::new(
            ZqnErrorKind::Limits,
            ZqnErrorCode::ResourceOverflow,
            "routing-noise u64 accumulation overflowed",
        )
    })
}

fn checked_add_u128(
    left: u128,
    right: u128,
) -> ZqnResult<u128> {
    left.checked_add(right).ok_or_else(|| {
        ZqnError::new(
            ZqnErrorKind::Limits,
            ZqnErrorCode::ResourceOverflow,
            "routing-noise u128 accumulation overflowed",
        )
    })
}

fn checked_mul_u128(
    left: u128,
    right: u128,
) -> ZqnResult<u128> {
    left.checked_mul(right).ok_or_else(|| {
        ZqnError::new(
            ZqnErrorKind::Limits,
            ZqnErrorCode::ResourceOverflow,
            "routing-noise weighted score multiplication overflowed",
        )
    })
}

fn duration_to_u64(
    duration: Duration,
) -> ZqnResult<u64> {
    let nanos = duration.as_nanos();

    if nanos > u64::MAX as u128 {
        return Err(ZqnError::new(
            ZqnErrorKind::Limits,
            ZqnErrorCode::SizeOverflow,
            "routing-noise duration exceeds the representable nanosecond range",
        ));
    }

    Ok(nanos as u64)
}

// ============================================================================
// Built-in neutral provider
// ============================================================================

/// Provider returning explicit unknown information.
///
/// This is useful when a routing pipeline has no ZQN calibration/model
/// available. It deliberately does NOT claim perfect hardware.
#[derive(Debug, Clone, Copy, Default)]
pub struct UnknownRoutingNoiseProvider;

impl RoutingNoiseProvider for UnknownRoutingNoiseProvider {
    fn estimate(
        &self,
        _request: &RoutingNoiseRequest,
    ) -> ZqnResult<RoutingNoiseEstimate> {
        Ok(RoutingNoiseEstimate::unknown())
    }

    fn provider_id(&self) -> &str {
        "zamani.zqn.unknown"
    }

    fn revision(&self) -> u64 {
        1
    }
}

// ============================================================================
// Conversion helpers for existing routing
// ============================================================================

/// Stable routing-facing scalar view.
///
/// This structure is useful for an adapter into the existing routing
/// `NoiseEstimate` contract without making ZQN depend on that implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RoutingNoiseScalarView {
    /// Error probability in parts per billion.
    pub error_ppb: u64,

    /// Fidelity in parts per billion.
    pub fidelity_ppb: u64,

    /// Duration in nanoseconds.
    pub duration_ns: u64,

    /// Semantic guarantee.
    pub guarantee: RoutingNoiseGuarantee,
}

impl RoutingNoiseScalarView {
    /// Creates a scalar view from an estimate.
    pub fn from_estimate(
        estimate: &RoutingNoiseEstimate,
    ) -> ZqnResult<Self> {
        Ok(Self {
            error_ppb: estimate.error_probability().ppb(),
            fidelity_ppb: estimate.fidelity().ppb(),
            duration_ns: duration_to_u64(estimate.duration())?,
            guarantee: estimate.guarantee(),
        })
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn operation_id() -> OperationId {
        OperationId::new(1)
    }

    #[test]
    fn probability_rejects_values_above_one() {
        let result = ProbabilityPpb::new(
            MAX_PROBABILITY_PPB + 1,
        );

        assert!(result.is_err());
    }

    #[test]
    fn probability_complement_is_deterministic() {
        let probability =
            ProbabilityPpb::new(250_000_000).expect("valid probability");

        assert_eq!(
            probability.complement().ppb(),
            750_000_000
        );
    }

    #[test]
    fn logical_and_physical_identity_are_distinct() {
        let logical =
            RoutingNoiseResource::logical(QubitId::new(7));

        let physical =
            RoutingNoiseResource::physical(PhysicalQubitId::new(7));

        assert_ne!(logical, physical);
        assert!(logical.is_logical());
        assert!(physical.is_physical());
    }

    #[test]
    fn request_supports_arbitrary_operand_count() {
        let request = RoutingNoiseRequest::new(
            operation_id(),
            vec![
                RoutingNoiseResource::logical(QubitId::new(0)),
                RoutingNoiseResource::logical(QubitId::new(1)),
                RoutingNoiseResource::logical(QubitId::new(2)),
                RoutingNoiseResource::logical(QubitId::new(3)),
                RoutingNoiseResource::logical(QubitId::new(4)),
            ],
        )
        .expect("request should be valid");

        assert_eq!(request.operands().len(), 5);
        assert_eq!(request.logical_operands().len(), 5);
    }

    #[test]
    fn request_rejects_duplicate_projection_entries() {
        let result = RoutingNoiseRequest::from_parts(
            RoutingNoiseOperation::new(operation_id()),
            vec![
                RoutingNoiseResource::logical(QubitId::new(0)),
            ],
            vec![
                QubitId::new(0),
                QubitId::new(0),
            ],
            Vec::new(),
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn unknown_provider_never_claims_perfect_hardware() {
        let provider = UnknownRoutingNoiseProvider;

        let request = RoutingNoiseRequest::new(
            operation_id(),
            vec![
                RoutingNoiseResource::physical(
                    PhysicalQubitId::new(0),
                ),
            ],
        )
        .expect("request should be valid");

        let estimate = provider
            .estimate(&request)
            .expect("provider should return an explicit unknown");

        assert!(estimate.is_unknown());
        assert_eq!(
            estimate.guarantee(),
            RoutingNoiseGuarantee::Unknown
        );
    }

    #[test]
    fn unknown_provider_is_deterministic() {
        let provider = UnknownRoutingNoiseProvider;

        let request = RoutingNoiseRequest::new(
            operation_id(),
            vec![
                RoutingNoiseResource::physical(
                    PhysicalQubitId::new(0),
                ),
            ],
        )
        .expect("request should be valid");

        let first = provider
            .estimate(&request)
            .expect("first estimate");

        let second = provider
            .estimate(&request)
            .expect("second estimate");

        assert_eq!(first, second);
    }

    #[test]
    fn weighted_score_uses_checked_integer_arithmetic() {
        let cost = RoutingNoiseCost {
            error_ppb: 100,
            duration_ns: 200,
            crosstalk_ppb: 300,
            correlation_ppb: 400,
            uncertainty_ppb: 500,
        };

        let weights =
            RoutingNoiseWeights::new(1, 2, 3, 4, 5);

        let score = cost
            .weighted_score(weights)
            .expect("score should fit");

        assert_eq!(
            score,
            100
                + 400
                + 900
                + 1_600
                + 2_500
        );
    }

    #[test]
    fn cost_addition_is_checked() {
        let left = RoutingNoiseCost {
            error_ppb: 10,
            duration_ns: 20,
            crosstalk_ppb: 30,
            correlation_ppb: 40,
            uncertainty_ppb: 50,
        };

        let right = RoutingNoiseCost {
            error_ppb: 1,
            duration_ns: 2,
            crosstalk_ppb: 3,
            correlation_ppb: 4,
            uncertainty_ppb: 5,
        };

        let total = left
            .checked_add(right)
            .expect("addition should fit");

        assert_eq!(total.error_ppb, 11);
        assert_eq!(total.duration_ns, 22);
        assert_eq!(total.crosstalk_ppb, 33);
        assert_eq!(total.correlation_ppb, 44);
        assert_eq!(total.uncertainty_ppb, 55);
    }

    #[test]
    fn aggregation_is_deterministic() {
        let a = RoutingNoiseEstimate::new(
            RoutingNoiseGuarantee::Exact,
            ProbabilityPpb::new(10).expect("valid"),
            FidelityPpb::new(999_999_990).expect("valid"),
            Duration::from_nanos(10),
            vec![
                RoutingNoiseContribution::new(
                    RoutingNoiseComponent::OperationError,
                    ProbabilityPpb::new(10).expect("valid"),
                ),
            ],
        )
        .expect("estimate");

        let b = RoutingNoiseEstimate::new(
            RoutingNoiseGuarantee::Bounded,
            ProbabilityPpb::new(20).expect("valid"),
            FidelityPpb::new(999_999_980).expect("valid"),
            Duration::from_nanos(20),
            vec![
                RoutingNoiseContribution::new(
                    RoutingNoiseComponent::CrosstalkError,
                    ProbabilityPpb::new(20).expect("valid"),
                ),
            ],
        )
        .expect("estimate");

        let first =
            aggregate_estimates([&a, &b]).expect("aggregate");

        let second =
            aggregate_estimates([&a, &b]).expect("aggregate");

        assert_eq!(first, second);
        assert_eq!(
            first.guarantee(),
            RoutingNoiseGuarantee::Bounded
        );
        assert_eq!(
            first.duration(),
            Duration::from_nanos(30)
        );
    }

    #[test]
    fn approximate_policy_can_reject_approximation() {
        let policy = RoutingNoisePolicy {
            allow_approximate: false,
            ..RoutingNoisePolicy::default()
        };

        let estimate = RoutingNoiseEstimate::new(
            RoutingNoiseGuarantee::Approximate,
            ProbabilityPpb::new(10).expect("valid"),
            FidelityPpb::new(999_999_990).expect("valid"),
            Duration::ZERO,
            Vec::new(),
        )
        .expect("estimate");

        assert!(
            policy.validate_estimate(&estimate).is_err()
        );
    }

    #[test]
    fn unlimited_policy_has_no_machine_size_ceiling() {
        let policy = RoutingNoisePolicy::unrestricted();

        assert_eq!(policy.max_operands, None);
        assert_eq!(policy.max_contributions, None);
    }

    #[test]
    fn scalar_view_preserves_semantics() {
        let estimate = RoutingNoiseEstimate::new(
            RoutingNoiseGuarantee::Exact,
            ProbabilityPpb::new(100).expect("valid"),
            FidelityPpb::new(999_999_900).expect("valid"),
            Duration::from_nanos(42),
            Vec::new(),
        )
        .expect("estimate");

        let view =
            RoutingNoiseScalarView::from_estimate(&estimate)
                .expect("view");

        assert_eq!(view.error_ppb, 100);
        assert_eq!(view.fidelity_ppb, 999_999_900);
        assert_eq!(view.duration_ns, 42);
        assert_eq!(
            view.guarantee,
            RoutingNoiseGuarantee::Exact
        );
    }
}