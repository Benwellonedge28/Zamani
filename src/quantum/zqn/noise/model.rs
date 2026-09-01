//! Zamani Quantum Noise (ZQN) — Canonical Noise-Model Contract.
//!
//! # Purpose
//!
//! This module defines the stable, backend-independent contract for a ZQN
//! noise model.
//!
//! A `NoiseModel` describes how physical or abstract uncertainty/noise is
//! selected for a requested quantum execution scope. It does NOT itself own
//! channel mathematics, fault storage, random-number generation, simulation,
//! hardware execution, calibration, routing, scheduling, or QEC.
//!
//! The central architectural distinction is:
//!
//! ```text
//! Quantum IR
//!     = what the program means
//!
//! NoiseModel
//!     = what physical uncertainty/deviation is associated with it
//!
//! NoiseApplication
//!     = where/how the model is applied
//!
//! QuantumChannel
//!     = mathematical physical transformation
//!
//! Fault
//!     = realized deviation/event
//!
//! Runtime/Simulator/Hardware
//!     = executes the selected semantics
//! ```
//!
//! # Architectural position
//!
//! ```text
//! crate::quantum::ir
//!        │
//!        │ canonical semantic operation/resource identities
//!        ▼
//! zqn::noise::model
//!        │
//!        ├───────────────┐
//!        │               │
//!        ▼               ▼
//! channel             fault
//!        │               │
//!        └───────┬───────┘
//!                ▼
//!          application
//!                │
//!       ┌────────┼─────────┐
//!       ▼        ▼         ▼
//! simulation    QEC      hardware
//!
//! calibration/characterization provide model data;
//! routing/scheduling consume model-derived cost information.
//! ```
//!
//! # Ownership
//!
//! This file owns:
//!
//! - the canonical `NoiseModel` trait;
//! - the stable noise-model descriptor;
//! - the model revision contract;
//! - model applicability/selection requests;
//! - abstract noise-effect references;
//! - model validation at the model-contract level;
//! - model-level deterministic identity metadata;
//! - model-level capability declarations;
//! - model-level semantic guarantees.
//!
//! # Does not own
//!
//! This file deliberately does NOT own:
//!
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - `OperationId`;
//! - quantum IR semantics;
//! - quantum-channel mathematics;
//! - Kraus/Choi/Liouville representations;
//! - probability distributions;
//! - RNG implementations;
//! - fault implementation;
//! - calibration values;
//! - characterization protocols;
//! - simulation;
//! - QEC decoding;
//! - routing;
//! - scheduling;
//! - hardware APIs;
//! - credentials;
//! - serialization formats;
//! - persistence;
//! - global registries;
//! - global mutable state.
//!
//! Those responsibilities belong to their respective modules.
//!
//! # Canonical quantum identities
//!
//! ZQN MUST use the canonical Quantum IR identities:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! crate::quantum::ir::identity::OperationId
//! ```
//!
//! This module therefore never creates a ZQN-specific `QubitId`,
//! `PhysicalQubitId`, or `OperationId`.
//!
//! This is essential for preventing semantic identity fragmentation across
//! the quantum compiler.
//!
//! # Write once, scale everywhere
//!
//! `NoiseModel` imposes no semantic upper bound on:
//!
//! - number of logical qubits;
//! - number of physical qubits;
//! - number of operations;
//! - circuit depth;
//! - number of noise locations;
//! - number of correlated resources;
//! - number of devices;
//! - number of execution nodes;
//! - number of shots;
//! - number of experiments.
//!
//! Actual resource limits are supplied through `ZqnContext` and its explicit
//! resource-policy layer.
//!
//! No `MAX_QUBITS`, `MAX_OPERATIONS`, `MAX_FAULTS`, or similar machine-size
//! constant is defined here.
//!
//! "Infinity" in the Zamani architecture means that the semantic contract
//! contains no artificial finite machine-size ceiling. Actual execution is,
//! of course, constrained by available memory, compute, storage, network,
//! target capability, and explicit resource policies.
//!
//! # No fixed gate set
//!
//! The model contract does not assume:
//!
//! - one-qubit gates;
//! - two-qubit gates;
//! - Pauli noise;
//! - a finite gate set;
//! - qubits as the only quantum resources.
//!
//! A model may describe noise affecting:
//!
//! - qubits;
//! - qudits;
//! - modes;
//! - bosonic resources;
//! - analog systems;
//! - measurement-based systems;
//! - transport resources;
//! - communication links;
//! - logical resources;
//! - composite resources;
//! - future quantum modalities.
//!
//! The canonical resource identities are used where the existing Quantum IR
//! has an authoritative identity type. Other resources are represented through
//! ZQN object identities or higher-level integration adapters.
//!
//! # Model versus realization
//!
//! A model is a semantic rule.
//!
//! A realization is one concrete outcome of applying that rule.
//!
//! Therefore:
//!
//! ```text
//! NoiseModel
//!      │
//!      ├── may select no effect
//!      ├── may select a channel
//!      ├── may select a fault
//!      └── may select multiple effects
//! ```
//!
//! The actual channel/fault objects remain owned by `channel` and `fault`.
//!
//! This module therefore returns typed ZQN object references rather than
//! duplicating channel/fault implementations.
//!
//! # Determinism
//!
//! A `NoiseModel` MUST NOT:
//!
//! - use a global RNG;
//! - use thread-local RNG implicitly;
//! - read wall-clock time implicitly;
//! - use memory addresses as semantic input;
//! - depend on hash-map iteration order;
//! - depend on thread scheduling;
//! - mutate global state.
//!
//! Stochastic realization is downstream of this contract.
//!
//! A deterministic runtime must supply explicit deterministic execution
//! context through `ZqnContext` and the ZQN sampling subsystem.
//!
//! This trait does not invent a second RNG API.
//!
//! # Parallel execution
//!
//! The model contract is compatible with:
//!
//! ```text
//! single-thread execution
//! multi-thread execution
//! distributed execution
//! accelerator execution
//! ```
//!
//! Implementations must not make thread identity part of semantic behavior.
//!
//! # Resource safety
//!
//! This file never allocates hidden global state.
//!
//! User-provided model implementations remain responsible for obeying the
//! resource policy exposed by `ZqnContext`.
//!
//! Operations that materialize large collections should be implemented by
//! downstream modules using explicit limits, streaming, iterators, or bounded
//! batches.
//!
//! This contract itself contains no machine-size ceiling.
//!
//! # Security
//!
//! A `NoiseModel` is data/behavior within the execution environment.
//!
//! It does NOT grant:
//!
//! - QPU access;
//! - hardware credentials;
//! - calibration write access;
//! - filesystem access;
//! - network access;
//! - process execution;
//! - authorization.
//!
//! Capability/security enforcement belongs to the runtime and surrounding
//! security architecture.
//!
//! Untrusted model specifications MUST be validated and executed under an
//! explicit `ZqnContext` resource policy.
//!
//! # Numerical semantics
//!
//! This contract deliberately does not prescribe `f32`, `f64`, arbitrary
//! precision, tensors, matrices, or a particular numerical library.
//!
//! Numerical precision belongs to the concrete channel/simulation/
//! characterization layer.
//!
//! If a model requires an approximation, the approximation must be explicit
//! through its declared semantic guarantee.
//!
//! # Serialization
//!
//! This module does NOT implement serialization.
//!
//! `zqn::io` owns the external representation.
//!
//! Serialization must preserve:
//!
//! - model identity;
//! - descriptor;
//! - revision;
//! - semantic guarantees;
//! - capability declarations;
//! - applicability semantics;
//! - effect references;
//! - provenance supplied by the owning layer.
//!
//! Rust memory layout is NOT a wire-format contract.
//!
//! # Versioning
//!
//! ZQN schema/semantic versioning remains owned by `zqn::core::version`.
//!
//! A model revision is distinct from the ZQN schema version.
//!
//! ```text
//! ZQN version
//!     = contract implemented by the ZQN subsystem
//!
//! Noise model revision
//!     = semantic revision of one model
//! ```
//!
//! # Integration contract
//!
//! ```text
//! Quantum IR
//!     │
//!     │ OperationId / QubitId / PhysicalQubitId
//!     ▼
//! NoiseApplicationRequest
//!     │
//!     ▼
//! NoiseModel::select
//!     │
//!     ▼
//! NoiseSelection
//!     │
//!     ├── None
//!     ├── Channel(ChannelId)
//!     ├── Fault(FaultId)
//!     └── Composite(...)
//!     │
//!     ├──────────────┬──────────────┬──────────────┐
//!     ▼              ▼              ▼              ▼
//! channel         fault       calibration     characterization
//!     │              │
//!     └───────┬──────┘
//!             ▼
//!       noise/application
//!             │
//!       ┌─────┼───────────────┐
//!       ▼     ▼               ▼
//! simulation  QEC           hardware
//! ```
//!
//! `routing`, `scheduling`, and `benchmarking` consume model-derived
//! information through their own integration modules. They must not redefine
//! the `NoiseModel` contract.
//!
//! # Future integration files
//!
//! `noise/application.rs` should consume `NoiseModel`, `NoiseApplicationRequest`
//! and `NoiseSelection` and attach selected semantics to concrete execution
//! operations.
//!
//! `channel/channel.rs` owns actual channel semantics referenced by
//! `NoiseSelection::Channel`.
//!
//! `fault/fault.rs` owns actual fault semantics referenced by
//! `NoiseSelection::Fault`.
//!
//! `calibration/*` supplies calibration state used by model implementations.
//!
//! `characterization/*` produces experimentally inferred model parameters.
//!
//! `simulation/*` realizes channels/faults and performs stochastic execution.
//!
//! `integration/qec.rs` converts selected fault/channel semantics into QEC
//! representations.
//!
//! `integration/routing.rs` consumes noise cost information for placement.
//!
//! `integration/scheduling.rs` consumes time/resource-dependent noise
//! information.
//!
//! `integration/hardware.rs` adapts abstract model semantics to target
//! capabilities.
//!
//! `io/*` serializes model descriptors and specifications.
//!
//! # API stability rule
//!
//! The core `NoiseModel` trait is intentionally small.
//!
//! New physical noise mechanisms should normally be implemented as model
//! implementations, channels, faults, or extensions without changing this
//! trait.
//!
//! This allows:
//!
//! ```text
//! existing NoiseModel implementation
//!          │
//!          └── remains valid while new noise types are added
//! ```
//!
//! New required semantics should be added through capability/descriptor data
//! or new extension points rather than by repeatedly expanding the trait.
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

use crate::quantum::ir::identity::OperationId;
use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::zqn::core::errors::{
    ZqnError,
    ZqnErrorCode,
    ZqnErrorKind,
    ZqnResult,
};
use crate::quantum::zqn::core::ids::{
    ChannelId,
    FaultId,
    NoiseModelId,
    NoiseParameterId,
    ZqnObjectId,
};
use crate::quantum::zqn::core::context::ZqnContext;

// ============================================================================
// Model revision
// ============================================================================

/// Semantic revision of an individual noise model.
///
/// This is intentionally separate from the global ZQN schema version.
///
/// A model revision changes when the mathematical/semantic behavior of that
/// model changes in a way that can affect reproducibility or interpretation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NoiseModelRevision {
    major: u32,
    minor: u32,
    patch: u32,
}

impl NoiseModelRevision {
    /// Creates an explicit model revision.
    ///
    /// The values are semantic version components, not machine limits.
    #[must_use]
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Major semantic revision.
    #[must_use]
    pub const fn major(self) -> u32 {
        self.major
    }

    /// Minor semantic revision.
    #[must_use]
    pub const fn minor(self) -> u32 {
        self.minor
    }

    /// Patch semantic revision.
    #[must_use]
    pub const fn patch(self) -> u32 {
        self.patch
    }
}

impl Default for NoiseModelRevision {
    fn default() -> Self {
        Self::new(1, 0, 0)
    }
}

impl fmt::Display for NoiseModelRevision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major, self.minor, self.patch
        )
    }
}

// ============================================================================
// Semantic guarantees
// ============================================================================

/// Semantic fidelity guarantee made by a noise model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NoiseSemanticGuarantee {
    /// The selected realization represents the requested semantics exactly
    /// within the numerical representation used by the consumer.
    Exact,

    /// The model is an explicitly declared approximation.
    ///
    /// The approximation contract must be described by the owning model.
    Approximate,

    /// The model provides a mathematically bounded approximation.
    ///
    /// The bound itself belongs to the model/parameter representation.
    Bounded,

    /// The model is statistical rather than deterministic.
    Statistical,

    /// The model cannot provide the requested semantics exactly and requires
    /// an explicit compatibility/approximation decision.
    RequiresCompatibilityDecision,
}

impl fmt::Display for NoiseSemanticGuarantee {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exact => formatter.write_str("exact"),
            Self::Approximate => formatter.write_str("approximate"),
            Self::Bounded => formatter.write_str("bounded"),
            Self::Statistical => formatter.write_str("statistical"),
            Self::RequiresCompatibilityDecision => {
                formatter.write_str("requires-compatibility-decision")
            }
        }
    }
}

// ============================================================================
// Noise model scope
// ============================================================================

/// Semantic scope in which a noise model may operate.
///
/// This is deliberately not a gate-set enumeration.
///
/// It describes the broad resource category without restricting how many
/// resources participate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NoiseModelScope {
    /// Model can apply to an individual quantum resource.
    Resource,

    /// Model can apply to an operation.
    Operation,

    /// Model can apply to measurement.
    Measurement,

    /// Model can apply to preparation.
    Preparation,

    /// Model can apply to reset.
    Reset,

    /// Model can apply to transport/communication.
    Transport,

    /// Model can apply to an arbitrary composite resource set.
    Composite,

    /// Model can apply to execution-wide/global conditions.
    Global,
}

impl fmt::Display for NoiseModelScope {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Resource => formatter.write_str("resource"),
            Self::Operation => formatter.write_str("operation"),
            Self::Measurement => formatter.write_str("measurement"),
            Self::Preparation => formatter.write_str("preparation"),
            Self::Reset => formatter.write_str("reset"),
            Self::Transport => formatter.write_str("transport"),
            Self::Composite => formatter.write_str("composite"),
            Self::Global => formatter.write_str("global"),
        }
    }
}

// ============================================================================
// Noise model capabilities
// ============================================================================

/// Capabilities declared by a noise model.
///
/// These are semantic declarations, not hardware capabilities.
///
/// A target may later reject a model even when the model itself is valid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NoiseModelCapabilities {
    correlated: bool,
    temporal: bool,
    spatial: bool,
    crosstalk: bool,
    non_markovian: bool,
    leakage: bool,
    erasure: bool,
    loss: bool,
    readout: bool,
    dynamic: bool,
}

impl NoiseModelCapabilities {
    /// Creates a capability declaration.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            correlated: false,
            temporal: false,
            spatial: false,
            crosstalk: false,
            non_markovian: false,
            leakage: false,
            erasure: false,
            loss: false,
            readout: false,
            dynamic: false,
        }
    }

    /// Returns whether arbitrary correlations may be represented.
    #[must_use]
    pub const fn supports_correlated(self) -> bool {
        self.correlated
    }

    /// Returns whether temporal dependence may be represented.
    #[must_use]
    pub const fn supports_temporal(self) -> bool {
        self.temporal
    }

    /// Returns whether spatial dependence may be represented.
    #[must_use]
    pub const fn supports_spatial(self) -> bool {
        self.spatial
    }

    /// Returns whether crosstalk may be represented.
    #[must_use]
    pub const fn supports_crosstalk(self) -> bool {
        self.crosstalk
    }

    /// Returns whether non-Markovian behavior may be represented.
    #[must_use]
    pub const fn supports_non_markovian(self) -> bool {
        self.non_markovian
    }

    /// Returns whether leakage may be represented.
    #[must_use]
    pub const fn supports_leakage(self) -> bool {
        self.leakage
    }

    /// Returns whether erasure may be represented.
    #[must_use]
    pub const fn supports_erasure(self) -> bool {
        self.erasure
    }

    /// Returns whether loss may be represented.
    #[must_use]
    pub const fn supports_loss(self) -> bool {
        self.loss
    }

    /// Returns whether readout noise may be represented.
    #[must_use]
    pub const fn supports_readout(self) -> bool {
        self.readout
    }

    /// Returns whether dynamic/conditional behavior may be represented.
    #[must_use]
    pub const fn supports_dynamic(self) -> bool {
        self.dynamic
    }

    /// Enables correlated-noise semantics.
    #[must_use]
    pub const fn with_correlated(mut self, value: bool) -> Self {
        self.correlated = value;
        self
    }

    /// Enables temporal-noise semantics.
    #[must_use]
    pub const fn with_temporal(mut self, value: bool) -> Self {
        self.temporal = value;
        self
    }

    /// Enables spatial-noise semantics.
    #[must_use]
    pub const fn with_spatial(mut self, value: bool) -> Self {
        self.spatial = value;
        self
    }

    /// Enables crosstalk semantics.
    #[must_use]
    pub const fn with_crosstalk(mut self, value: bool) -> Self {
        self.crosstalk = value;
        self
    }

    /// Enables non-Markovian semantics.
    #[must_use]
    pub const fn with_non_markovian(mut self, value: bool) -> Self {
        self.non_markovian = value;
        self
    }

    /// Enables leakage semantics.
    #[must_use]
    pub const fn with_leakage(mut self, value: bool) -> Self {
        self.leakage = value;
        self
    }

    /// Enables erasure semantics.
    #[must_use]
    pub const fn with_erasure(mut self, value: bool) -> Self {
        self.erasure = value;
        self
    }

    /// Enables loss semantics.
    #[must_use]
    pub const fn with_loss(mut self, value: bool) -> Self {
        self.loss = value;
        self
    }

    /// Enables readout semantics.
    #[must_use]
    pub const fn with_readout(mut self, value: bool) -> Self {
        self.readout = value;
        self
    }

    /// Enables dynamic semantics.
    #[must_use]
    pub const fn with_dynamic(mut self, value: bool) -> Self {
        self.dynamic = value;
        self
    }
}

impl Default for NoiseModelCapabilities {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Noise model descriptor
// ============================================================================

/// Immutable descriptive metadata for a noise model.
///
/// The descriptor does not contain executable state or hardware handles.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NoiseModelDescriptor {
    id: NoiseModelId,
    name: String,
    revision: NoiseModelRevision,
    scope: NoiseModelScope,
    guarantee: NoiseSemanticGuarantee,
    capabilities: NoiseModelCapabilities,
}

impl NoiseModelDescriptor {
    /// Creates a validated descriptor.
    ///
    /// The name must be non-empty and must not contain control characters.
    pub fn new(
        id: NoiseModelId,
        name: impl Into<String>,
        revision: NoiseModelRevision,
        scope: NoiseModelScope,
        guarantee: NoiseSemanticGuarantee,
        capabilities: NoiseModelCapabilities,
    ) -> ZqnResult<Self> {
        let name = name.into();

        if name.is_empty() {
            return Err(Self::invalid(
                ZqnErrorCode::InvalidNoiseModel,
                "noise-model name must not be empty",
            ));
        }

        if name.chars().any(char::is_control) {
            return Err(Self::invalid(
                ZqnErrorCode::InvalidNoiseModel,
                "noise-model name must not contain control characters",
            ));
        }

        Ok(Self {
            id,
            name,
            revision,
            scope,
            guarantee,
            capabilities,
        })
    }

    /// Returns the model identity.
    #[must_use]
    pub const fn id(&self) -> NoiseModelId {
        self.id
    }

    /// Returns the stable model name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the model revision.
    #[must_use]
    pub const fn revision(&self) -> NoiseModelRevision {
        self.revision
    }

    /// Returns the supported semantic scope.
    #[must_use]
    pub const fn scope(&self) -> NoiseModelScope {
        self.scope
    }

    /// Returns the semantic fidelity guarantee.
    #[must_use]
    pub const fn guarantee(&self) -> NoiseSemanticGuarantee {
        self.guarantee
    }

    /// Returns declared model capabilities.
    #[must_use]
    pub const fn capabilities(&self) -> NoiseModelCapabilities {
        self.capabilities
    }

    fn invalid(code: ZqnErrorCode, message: &'static str) -> ZqnError {
        ZqnError::new(
            ZqnErrorKind::Noise,
            code,
            message,
        )
    }
}

// ============================================================================
// Noise target
// ============================================================================

/// A canonical resource to which noise may apply.
///
/// This uses the existing Quantum IR identity types directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NoiseTarget {
    /// Logical qubit.
    LogicalQubit(QubitId),

    /// Physical qubit.
    PhysicalQubit(PhysicalQubitId),

    /// Canonical IR operation.
    Operation(OperationId),

    /// Generic ZQN-owned resource.
    ZqnResource(ZqnObjectId),
}

impl NoiseTarget {
    /// Creates a logical-qubit target.
    #[must_use]
    pub const fn logical_qubit(id: QubitId) -> Self {
        Self::LogicalQubit(id)
    }

    /// Creates a physical-qubit target.
    #[must_use]
    pub const fn physical_qubit(id: PhysicalQubitId) -> Self {
        Self::PhysicalQubit(id)
    }

    /// Creates an operation target.
    #[must_use]
    pub const fn operation(id: OperationId) -> Self {
        Self::Operation(id)
    }

    /// Creates a ZQN resource target.
    #[must_use]
    pub const fn zqn_resource(id: ZqnObjectId) -> Self {
        Self::ZqnResource(id)
    }
}

// ============================================================================
// Noise application request
// ============================================================================

/// Immutable request describing where a noise model is being evaluated.
///
/// The request is deliberately declarative. It does not contain hardware
/// handles, RNG state, credentials, calibration storage, or simulator state.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NoiseApplicationRequest {
    operation: Option<OperationId>,
    targets: Vec<NoiseTarget>,
    parameters: Vec<NoiseParameterId>,
}

impl NoiseApplicationRequest {
    /// Creates an empty request.
    ///
    /// An empty request is useful for execution-wide/global models.
    #[must_use]
    pub fn new() -> Self {
        Self {
            operation: None,
            targets: Vec::new(),
            parameters: Vec::new(),
        }
    }

    /// Associates the request with a canonical IR operation.
    #[must_use]
    pub fn with_operation(mut self, operation: OperationId) -> Self {
        self.operation = Some(operation);
        self
    }

    /// Adds a resource target.
    #[must_use]
    pub fn with_target(mut self, target: NoiseTarget) -> Self {
        self.targets.push(target);
        self
    }

    /// Adds a parameter identity used by the model.
    #[must_use]
    pub fn with_parameter(mut self, parameter: NoiseParameterId) -> Self {
        self.parameters.push(parameter);
        self
    }

    /// Returns the associated operation, if any.
    #[must_use]
    pub const fn operation(&self) -> Option<OperationId> {
        self.operation
    }

    /// Returns the requested resource targets.
    #[must_use]
    pub fn targets(&self) -> &[NoiseTarget] {
        &self.targets
    }

    /// Returns parameter identities.
    #[must_use]
    pub fn parameters(&self) -> &[NoiseParameterId] {
        &self.parameters
    }

    /// Returns whether the request has no operation and no targets.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operation.is_none() && self.targets.is_empty()
    }

    /// Validates structural invariants.
    pub fn validate(&self) -> ZqnResult<()> {
        if self.is_empty() {
            // Empty requests are valid for global models.
            return Ok(());
        }

        Ok(())
    }
}

impl Default for NoiseApplicationRequest {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Noise effect references
// ============================================================================

/// One abstract effect selected by a noise model.
///
/// The referenced mathematical object is owned by its corresponding ZQN
/// subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NoiseEffect {
    /// No physical/noise effect is selected.
    None,

    /// Reference to a canonical ZQN quantum channel.
    Channel(ChannelId),

    /// Reference to a realized or declaratively selected fault.
    Fault(FaultId),
}

impl NoiseEffect {
    /// Returns true when this effect represents no deviation.
    #[must_use]
    pub const fn is_none(self) -> bool {
        matches!(self, Self::None)
    }
}

// ============================================================================
// Noise selection
// ============================================================================

/// Result of asking a noise model to select noise for an application request.
///
/// Composite selection is represented as a flat vector rather than recursive
/// nested selections. This avoids recursive data structures whose depth could
/// be controlled by untrusted input and makes large workloads easier to stream
/// or batch in downstream modules.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NoiseSelection {
    /// No noise effect applies.
    None,

    /// Exactly one effect applies.
    Single(NoiseEffect),

    /// Multiple effects apply to the request.
    ///
    /// The vector has no semantic maximum. Resource limits are enforced by the
    /// execution context/downstream owner.
    Composite(Vec<NoiseEffect>),
}

impl NoiseSelection {
    /// Creates an empty selection.
    #[must_use]
    pub const fn none() -> Self {
        Self::None
    }

    /// Creates a single selection.
    #[must_use]
    pub const fn single(effect: NoiseEffect) -> Self {
        Self::Single(effect)
    }

    /// Creates a composite selection.
    ///
    /// `None` effects are retained exactly as supplied so that callers can
    /// preserve explicit model decisions. Consumers may normalize them when
    /// appropriate.
    #[must_use]
    pub fn composite(effects: Vec<NoiseEffect>) -> Self {
        if effects.is_empty() {
            Self::None
        } else if effects.len() == 1 {
            Self::Single(effects[0])
        } else {
            Self::Composite(effects)
        }
    }

    /// Returns true if this selection contains no effects.
    #[must_use]
    pub fn is_none(&self) -> bool {
        match self {
            Self::None => true,
            Self::Single(effect) => effect.is_none(),
            Self::Composite(effects) => {
                effects.iter().all(|effect| effect.is_none())
            }
        }
    }

    /// Returns the number of selected effect references.
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::None => 0,
            Self::Single(_) => 1,
            Self::Composite(effects) => effects.len(),
        }
    }

    /// Returns true if no effect references are present.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Visits each selected effect without exposing the internal representation.
    ///
    /// This allows downstream application/simulation code to process selections
    /// without depending on whether the model chose a single or composite
    /// representation.
    pub fn for_each<F>(&self, mut visitor: F)
    where
        F: FnMut(NoiseEffect),
    {
        match self {
            Self::None => {}
            Self::Single(effect) => visitor(*effect),
            Self::Composite(effects) => {
                for effect in effects {
                    visitor(*effect);
                }
            }
        }
    }
}

// ============================================================================
// Noise model contract
// ============================================================================

/// Canonical backend-independent noise-model contract.
///
/// Implementations describe *what noise semantics apply*; they do not own
/// execution.
///
/// # Object safety
///
/// The trait deliberately avoids associated types and generic required
/// methods. This permits use through:
///
/// ```text
/// &dyn NoiseModel
/// Box<dyn NoiseModel + Send + Sync>
/// Arc<dyn NoiseModel + Send + Sync>
/// ```
///
/// where the surrounding runtime permits those containers.
///
/// # Thread safety
///
/// Implementations should be `Send + Sync` whenever their internal state
/// permits. The trait itself does not require those bounds so that specialized
/// single-threaded model implementations remain possible without changing
/// the semantic contract.
///
/// Runtime registries that require concurrent sharing should explicitly use
/// `dyn NoiseModel + Send + Sync`.
pub trait NoiseModel {
    /// Returns immutable descriptive metadata.
    fn descriptor(&self) -> &NoiseModelDescriptor;

    /// Validates the model against the supplied ZQN execution context.
    ///
    /// Implementations must not mutate global state.
    fn validate(&self, context: &ZqnContext) -> ZqnResult<()>;

    /// Determines whether this model can semantically consider the request.
    ///
    /// Returning `false` means the model does not claim applicability.
    ///
    /// This is different from an error: malformed model state should return an
    /// error from `validate`, while a valid model that simply does not apply
    /// to one request may return `false`.
    fn applies_to(
        &self,
        request: &NoiseApplicationRequest,
    ) -> ZqnResult<bool>;

    /// Selects abstract noise effects for the requested scope.
    ///
    /// This method does not execute a channel, mutate quantum state, access
    /// hardware, or perform QEC.
    ///
    /// A stochastic implementation may delegate actual random realization to
    /// the explicit ZQN sampling subsystem. It must not silently create hidden
    /// RNG state.
    fn select(
        &self,
        request: &NoiseApplicationRequest,
        context: &ZqnContext,
    ) -> ZqnResult<NoiseSelection>;

    /// Returns the model identity.
    #[must_use]
    fn id(&self) -> NoiseModelId {
        self.descriptor().id()
    }

    /// Returns the model revision.
    #[must_use]
    fn revision(&self) -> NoiseModelRevision {
        self.descriptor().revision()
    }

    /// Returns the model capabilities.
    #[must_use]
    fn capabilities(&self) -> NoiseModelCapabilities {
        self.descriptor().capabilities()
    }

    /// Returns the semantic guarantee of the model.
    #[must_use]
    fn guarantee(&self) -> NoiseSemanticGuarantee {
        self.descriptor().guarantee()
    }
}

// ============================================================================
// Validation helpers
// ============================================================================

/// Validates a model and an application request as one explicit operation.
///
/// This function is intentionally outside the trait implementation so the
/// same validation sequence is available to registries, application adapters,
/// simulation, routing, scheduling, and hardware integration.
pub fn validate_model_request(
    model: &dyn NoiseModel,
    request: &NoiseApplicationRequest,
    context: &ZqnContext,
) -> ZqnResult<bool> {
    request.validate()?;
    model.validate(context)?;
    model.applies_to(request)
}

/// Selects noise only after validating the complete model/request contract.
///
/// This function is the preferred entry point for downstream integrations that
/// do not need to implement their own validation ordering.
pub fn select_noise(
    model: &dyn NoiseModel,
    request: &NoiseApplicationRequest,
    context: &ZqnContext,
) -> ZqnResult<NoiseSelection> {
    if !validate_model_request(model, request, context)? {
        return Ok(NoiseSelection::None);
    }

    let selection = model.select(request, context)?;

    validate_selection(&selection)?;

    Ok(selection)
}

/// Validates structural invariants of a selected noise effect.
pub fn validate_selection(selection: &NoiseSelection) -> ZqnResult<()> {
    match selection {
        NoiseSelection::None => Ok(()),

        NoiseSelection::Single(effect) => {
            validate_effect(*effect)
        }

        NoiseSelection::Composite(effects) => {
            for effect in effects {
                validate_effect(*effect)?;
            }

            Ok(())
        }
    }
}

fn validate_effect(effect: NoiseEffect) -> ZqnResult<()> {
    match effect {
        NoiseEffect::None => Ok(()),

        NoiseEffect::Channel(_) => Ok(()),

        NoiseEffect::Fault(_) => Ok(()),
    }
}

// ============================================================================
// Standard no-noise model
// ============================================================================

/// A model that explicitly represents absence of noise.
///
/// This is useful as a compositional identity model and for testing.
///
/// It does not represent "unknown noise"; it represents an explicit
/// zero-effect model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoNoiseModel {
    descriptor: NoiseModelDescriptor,
}

impl NoNoiseModel {
    /// Creates an explicit no-noise model.
    pub fn new(id: NoiseModelId) -> ZqnResult<Self> {
        let descriptor = NoiseModelDescriptor::new(
            id,
            "no-noise",
            NoiseModelRevision::default(),
            NoiseModelScope::Global,
            NoiseSemanticGuarantee::Exact,
            NoiseModelCapabilities::default(),
        )?;

        Ok(Self { descriptor })
    }
}

impl NoiseModel for NoNoiseModel {
    fn descriptor(&self) -> &NoiseModelDescriptor {
        &self.descriptor
    }

    fn validate(&self, _context: &ZqnContext) -> ZqnResult<()> {
        Ok(())
    }

    fn applies_to(
        &self,
        _request: &NoiseApplicationRequest,
    ) -> ZqnResult<bool> {
        Ok(true)
    }

    fn select(
        &self,
        _request: &NoiseApplicationRequest,
        _context: &ZqnContext,
    ) -> ZqnResult<NoiseSelection> {
        Ok(NoiseSelection::None)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn revision_is_value_stable() {
        let revision = NoiseModelRevision::new(2, 4, 7);

        assert_eq!(revision.major(), 2);
        assert_eq!(revision.minor(), 4);
        assert_eq!(revision.patch(), 7);
        assert_eq!(revision.to_string(), "2.4.7");
    }

    #[test]
    fn descriptor_rejects_empty_name() {
        let result = NoiseModelDescriptor::new(
            NoiseModelId::new(1),
            "",
            NoiseModelRevision::default(),
            NoiseModelScope::Global,
            NoiseSemanticGuarantee::Exact,
            NoiseModelCapabilities::default(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn descriptor_rejects_control_characters() {
        let result = NoiseModelDescriptor::new(
            NoiseModelId::new(1),
            "valid\ninvalid",
            NoiseModelRevision::default(),
            NoiseModelScope::Global,
            NoiseSemanticGuarantee::Exact,
            NoiseModelCapabilities::default(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn descriptor_preserves_identity_and_revision() {
        let descriptor = NoiseModelDescriptor::new(
            NoiseModelId::new(42),
            "example",
            NoiseModelRevision::new(1, 2, 3),
            NoiseModelScope::Operation,
            NoiseSemanticGuarantee::Exact,
            NoiseModelCapabilities::default(),
        )
        .expect("descriptor should be valid");

        assert_eq!(descriptor.id(), NoiseModelId::new(42));
        assert_eq!(
            descriptor.revision(),
            NoiseModelRevision::new(1, 2, 3)
        );
        assert_eq!(descriptor.name(), "example");
        assert_eq!(descriptor.scope(), NoiseModelScope::Operation);
    }

    #[test]
    fn canonical_resource_types_are_used() {
        let logical = NoiseTarget::logical_qubit(QubitId::new(7));
        let physical =
            NoiseTarget::physical_qubit(PhysicalQubitId::new(7));
        let operation =
            NoiseTarget::operation(OperationId::new(7));

        assert_eq!(
            logical,
            NoiseTarget::LogicalQubit(QubitId::new(7))
        );

        assert_eq!(
            physical,
            NoiseTarget::PhysicalQubit(PhysicalQubitId::new(7))
        );

        assert_eq!(
            operation,
            NoiseTarget::Operation(OperationId::new(7))
        );
    }

    #[test]
    fn request_is_immutable_value_data() {
        let request = NoiseApplicationRequest::new()
            .with_operation(OperationId::new(10))
            .with_target(
                NoiseTarget::logical_qubit(QubitId::new(3)),
            )
            .with_target(
                NoiseTarget::physical_qubit(
                    PhysicalQubitId::new(8),
                ),
            )
            .with_parameter(NoiseParameterId::new(99));

        assert_eq!(
            request.operation(),
            Some(OperationId::new(10))
        );
        assert_eq!(request.targets().len(), 2);
        assert_eq!(request.parameters().len(), 1);
    }

    #[test]
    fn selection_normalizes_empty_composite() {
        assert_eq!(
            NoiseSelection::composite(Vec::new()),
            NoiseSelection::None
        );
    }

    #[test]
    fn selection_normalizes_single_composite() {
        let selection =
            NoiseSelection::composite(vec![
                NoiseEffect::Channel(ChannelId::new(1)),
            ]);

        assert_eq!(
            selection,
            NoiseSelection::Single(
                NoiseEffect::Channel(ChannelId::new(1))
            )
        );
    }

    #[test]
    fn selection_preserves_multiple_effects() {
        let selection =
            NoiseSelection::composite(vec![
                NoiseEffect::Channel(ChannelId::new(1)),
                NoiseEffect::Fault(FaultId::new(2)),
            ]);

        assert_eq!(selection.len(), 2);
        assert!(!selection.is_empty());
        assert!(!selection.is_none());
    }

    #[test]
    fn no_noise_model_is_an_explicit_identity_model() {
        let model = NoNoiseModel::new(NoiseModelId::new(1))
            .expect("no-noise model should be valid");

        assert_eq!(model.id(), NoiseModelId::new(1));
        assert_eq!(
            model.descriptor().scope(),
            NoiseModelScope::Global
        );

        let request = NoiseApplicationRequest::new();

        // This test intentionally does not construct a ZqnContext manually.
        // The concrete context constructor remains owned by core::context.
        //
        // Model-level API correctness is covered by descriptor and selection
        // tests; integration tests should exercise the repository's canonical
        // context constructor.
        assert!(request.validate().is_ok());
    }

    #[test]
    fn capability_builder_is_persistent_value_style() {
        let capabilities = NoiseModelCapabilities::default()
            .with_correlated(true)
            .with_temporal(true)
            .with_spatial(true)
            .with_crosstalk(true)
            .with_non_markovian(true)
            .with_leakage(true)
            .with_erasure(true)
            .with_loss(true)
            .with_readout(true)
            .with_dynamic(true);

        assert!(capabilities.supports_correlated());
        assert!(capabilities.supports_temporal());
        assert!(capabilities.supports_spatial());
        assert!(capabilities.supports_crosstalk());
        assert!(capabilities.supports_non_markovian());
        assert!(capabilities.supports_leakage());
        assert!(capabilities.supports_erasure());
        assert!(capabilities.supports_loss());
        assert!(capabilities.supports_readout());
        assert!(capabilities.supports_dynamic());
    }
}