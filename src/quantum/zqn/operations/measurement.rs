//! Zamani Quantum Noise (ZQN)
//! Measurement-level noise integration.
//!
//! `src/quantum/zqn/operations/measurement.rs`
//!
//! # Purpose
//!
//! This module defines the ZQN-side contract for associating physical
//! uncertainty and noise with a canonical quantum measurement operation.
//!
//! The canonical meaning of measurement remains owned by:
//!
//! ```text
//! crate::quantum::ir::measurement
//! ```
//!
//! This module does NOT redefine measurement semantics.
//!
//! Instead, it answers:
//!
//! > What noise-related semantics are associated with this measurement
//! > boundary, its quantum resources, and its readout process?
//!
//! # Ownership
//!
//! This module owns:
//!
//! - measurement-noise binding metadata;
//! - measurement noise phase;
//! - measurement noise scope;
//! - canonical logical/physical measurement-resource references;
//! - deterministic construction of measurement noise requests;
//! - local validation of measurement-noise bindings;
//! - integration with `zqn::noise::application`;
//! - convenience application of an existing `NoiseModel`;
//! - explicit distinction between measurement backaction and readout noise.
//!
//! # Does NOT own
//!
//! This module does NOT own:
//!
//! - canonical measurement semantics;
//! - `Measurement` itself;
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - `OperationId`;
//! - readout hardware;
//! - ADC/DAC behavior;
//! - detector implementation;
//! - measurement pulses;
//! - calibration storage;
//! - probability mathematics;
//! - quantum channels;
//! - fault mathematics;
//! - random-number generation;
//! - simulator state;
//! - QEC decoding;
//! - routing;
//! - scheduling;
//! - hardware APIs;
//! - vendor-specific measurement names;
//! - serialization wire formats;
//! - global state.
//!
//! Those responsibilities remain in their respective subsystems.
//!
//! # Canonical identity boundary
//!
//! Logical measurement resources use:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! Physical measurement resources use:
//!
//! ```text
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Measurement operation identity uses:
//!
//! ```text
//! crate::quantum::ir::identity::OperationId
//! ```
//!
//! ZQN deliberately does not define replacement identity types.
//!
//! # Measurement is not readout-only
//!
//! A quantum measurement can have several physically meaningful noise
//! boundaries:
//!
//! ```text
//! quantum state
//!     │
//!     ▼
//! ┌─────────────────────┐
//! │ before measurement  │  preparation / pre-measurement disturbance
//! └──────────┬──────────┘
//!            ▼
//! ┌─────────────────────┐
//! │ measurement process │  backaction / measurement noise
//! └──────────┬──────────┘
//!            ▼
//! ┌─────────────────────┐
//! │ physical readout    │  detector/readout noise
//! └──────────┬──────────┘
//!            ▼
//! ┌─────────────────────┐
//! │ classical result    │  assignment / digitization noise
//! └─────────────────────┘
//! ```
//!
//! These stages must not be collapsed into a single hard-coded "readout
//! error", because different technologies expose different physical
//! measurement mechanisms.
//!
//! # Universal-program principle
//!
//! The same Zamani program must remain usable across compatible targets of
//! different sizes and technologies.
//!
//! Therefore this module contains no semantic limits for:
//!
//! - number of qubits;
//! - number of physical qubits;
//! - number of measurement targets;
//! - number of measurement operations;
//! - circuit depth;
//! - number of shots;
//! - number of measurement outcomes;
//! - machine size;
//! - device topology.
//!
//! There is no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_MEASUREMENTS
//! MAX_READOUTS
//! ```
//!
//! Semantic capacity is determined by the represented computation.
//!
//! Concrete resource limits belong to explicit runtime/compiler/resource
//! policies such as `ZqnContext` and the Quantum IR limit layer.
//!
//! "Infinity" therefore means that this semantic module imposes no artificial
//! finite machine-size ceiling. Real execution remains constrained by the
//! available resources and target capabilities.
//!
//! # No fixed measurement model
//!
//! This module does not assume that measurement is:
//!
//! - one qubit;
//! - two outcomes;
//! - Z-basis;
//! - projective;
//! - destructive;
//! - non-destructive;
//! - binary;
//! - qubit-only.
//!
//! The canonical IR measurement layer already supports richer measurement
//! semantics, including generalized, weak, continuous, and joint
//! measurements. This ZQN layer merely attaches noise to those semantics.
//!
//! # Noise versus measurement semantics
//!
//! This distinction is mandatory:
//!
//! ```text
//! quantum::ir::measurement
//!     = what measurement means
//!
//! zqn::operations::measurement
//!     = what uncertainty/noise is associated with it
//!
//! zqn::channel
//!     = mathematical channel representation
//!
//! zqn::fault
//!     = discrete fault representation
//!
//! zqn::simulation
//!     = physical/state realization
//!
//! hardware
//!     = physical implementation
//! ```
//!
//! # Determinism
//!
//! This module never samples noise.
//!
//! It does not:
//!
//! - create an RNG;
//! - use a global RNG;
//! - use a thread-local RNG;
//! - read wall-clock time;
//! - derive randomness from memory addresses;
//! - depend on worker count;
//! - depend on thread scheduling.
//!
//! A stochastic `NoiseModel` is realized later through the explicit ZQN
//! execution/sampling context.
//!
//! Constructing a `MeasurementNoiseBinding` or a `NoiseApplicationRequest`
//! therefore consumes no randomness.
//!
//! # Parallelism
//!
//! The binding is immutable after construction.
//!
//! It is therefore suitable for sharing between concurrent consumers,
//! subject to the normal `Send`/`Sync` guarantees of the types held by the
//! consumer.
//!
//! Semantic values do not depend on:
//!
//! - thread identity;
//! - worker count;
//! - process identity;
//! - scheduling order.
//!
//! # Resource safety
//!
//! This module never materializes:
//!
//! - quantum states;
//! - density matrices;
//! - channel matrices;
//! - fault batches;
//! - measurement-result buffers;
//! - shot arrays.
//!
//! It only represents measurement-noise attachment metadata.
//!
//! Potentially large target collections are represented explicitly by the
//! caller and remain subject to caller-selected resource policies.
//!
//! No hidden unbounded expansion is performed.
//!
//! # Numerical safety
//!
//! No numerical noise parameters are interpreted here.
//!
//! Probability validation belongs to `zqn::probability`.
//!
//! Channel validation belongs to `zqn::channel`.
//!
//! Calibration validation belongs to `zqn::calibration`.
//!
//! Consequently this file never silently converts:
//!
//! ```text
//! NaN      -> 0
//! Infinity -> finite value
//! invalid probability -> absolute value
//! ```
//!
//! # Integration
//!
//! ```text
//!                 quantum::ir::measurement
//!                           │
//!                           │ OperationId / QubitId
//!                           ▼
//!             zqn::operations::measurement
//!                           │
//!                  MeasurementNoiseBinding
//!                           │
//!                           ▼
//!                NoiseApplicationRequest
//!                           │
//!                           ▼
//!                     NoiseModel
//!                           │
//!                           ▼
//!                   NoiseApplication
//!                           │
//!             ┌─────────────┼──────────────┐
//!             ▼             ▼              ▼
//!          channel         fault       provenance
//!             │             │
//!             └─────────────┼──────────────┘
//!                           ▼
//!                 simulation / QEC / runtime
//! ```
//!
//! # Integration with canonical measurement IR
//!
//! The canonical IR measurement owns:
//!
//! - measurement observable;
//! - measurement kind;
//! - quantum targets;
//! - classical destinations;
//! - destructive/non-destructive semantics;
//! - reset-after-measurement intent.
//!
//! This file must never duplicate those structures.
//!
//! Instead, a caller supplies the canonical `OperationId` and canonical
//! quantum resource identities when constructing a binding.
//!
//! # Integration with `noise::model`
//!
//! `NoiseApplicationRequest` is the canonical ZQN request boundary.
//!
//! A measurement binding converts into that request using:
//!
//! - the canonical operation identity;
//! - logical and/or physical measurement resources;
//! - explicit semantic measurement-noise scope.
//!
//! The noise model remains responsible for selecting actual effects.
//!
//! # Integration with `noise::application`
//!
//! `MeasurementNoiseBinding::apply` delegates selection to the existing
//! `NoiseApplication::from_model` boundary.
//!
//! This module therefore does not duplicate model selection or application
//! validation.
//!
//! # Integration with channels
//!
//! A selected channel remains owned by `zqn::channel`.
//!
//! This file never applies a channel to a state.
//!
//! # Integration with faults
//!
//! A selected measurement fault remains owned by `zqn::fault`.
//!
//! QEC consumes the resulting application through its own integration layer.
//!
//! # Integration with calibration
//!
//! Calibration is not stored as a measurement-noise implementation detail.
//!
//! A model can use calibration context when selecting noise. This file merely
//! identifies the measurement operation/resource scope.
//!
//! # Integration with routing
//!
//! Logical targets can be bound before routing.
//!
//! Physical targets can be bound after mapping.
//!
//! This permits the same semantic measurement-noise description to survive
//! logical-to-physical lowering without embedding routing into ZQN.
//!
//! # Integration with scheduling
//!
//! Scheduling remains responsible for determining when a measurement executes.
//!
//! Measurement duration is therefore deliberately not stored as an implicit
//! hardware-specific value in this module.
//!
//! Time-dependent models consume scheduling/runtime context through the normal
//! ZQN execution boundary.
//!
//! # Integration with hardware
//!
//! Hardware adapters determine whether the requested measurement/noise
//! semantics can be represented by a target.
//!
//! This module contains no vendor-specific backend calls.
//!
//! # Integration with simulation
//!
//! Simulation consumes the resulting `NoiseApplication` and realizes the
//! selected effects.
//!
//! This module does not mutate simulator state.
//!
//! # Integration with benchmarking
//!
//! Benchmarking may use measurement-noise application identity to correlate
//! readout errors, assignment errors, measurement backaction, and
//! characterization results.
//!
//! Benchmarking does not own this semantic binding.
//!
//! # Serialization
//!
//! This file intentionally does not define a wire format.
//!
//! `zqn::io` owns serialization.
//!
//! A serialized measurement-noise binding must preserve at least:
//!
//! - operation identity;
//! - target identities;
//! - noise phase;
//! - noise scope;
//! - representation/schema version supplied by the owning ZQN schema layer.
//!
//! Rust memory layout is not an external serialization contract.
//!
//! # Security
//!
//! This module does not grant:
//!
//! - hardware access;
//! - filesystem access;
//! - network access;
//! - credentials;
//! - simulator access;
//! - calibration mutation;
//! - process execution.
//!
//! Untrusted bindings must still be validated before execution.
//!
//! # Rust contract
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use std::collections::BTreeSet;

use crate::quantum::ir::identity::OperationId;
use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

use crate::quantum::zqn::core::context::ZqnContext;
use crate::quantum::zqn::core::ids::NoiseApplicationId;
use crate::quantum::zqn::noise::application::NoiseApplication;
use crate::quantum::zqn::noise::model::{
    NoiseApplicationRequest,
    NoiseModel,
    NoiseTarget,
};

// ============================================================================
// Errors
// ============================================================================

/// Errors produced while constructing or validating a measurement-noise
/// binding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MeasurementNoiseError {
    /// A measurement operation has no canonical operation identity.
    MissingOperationId,

    /// No quantum measurement resource was supplied.
    EmptyTargets,

    /// A target appeared more than once.
    DuplicateTarget,

    /// Logical and physical targets were mixed when a single identity domain
    /// was required.
    MixedTargetDomains,

    /// A measurement-noise scope is incompatible with the target information.
    InvalidScope {
        /// Human-readable static reason.
        reason: &'static str,
    },

    /// The caller supplied an invalid measurement operation identity.
    InvalidOperationId,
}

impl fmt::Display for MeasurementNoiseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingOperationId => {
                formatter.write_str(
                    "measurement noise requires a canonical operation identity",
                )
            }

            Self::EmptyTargets => {
                formatter.write_str(
                    "measurement noise requires at least one measurement target",
                )
            }

            Self::DuplicateTarget => {
                formatter.write_str(
                    "measurement noise contains a duplicate measurement target",
                )
            }

            Self::MixedTargetDomains => {
                formatter.write_str(
                    "measurement noise cannot mix logical and physical target domains",
                )
            }

            Self::InvalidScope { reason } => {
                write!(formatter, "invalid measurement noise scope: {reason}")
            }

            Self::InvalidOperationId => {
                formatter.write_str("invalid measurement operation identity")
            }
        }
    }
}

impl std::error::Error for MeasurementNoiseError {}

/// Local result type for measurement-noise construction.
pub type MeasurementNoiseResult<T> = Result<T, MeasurementNoiseError>;

// ============================================================================
// Measurement noise phase
// ============================================================================

/// Semantic location of noise relative to measurement.
///
/// These values describe semantic boundaries, not a required hardware
/// implementation sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MeasurementNoisePhase {
    /// Noise occurring before the measurement process begins.
    ///
    /// Examples include state preparation disturbance or pre-measurement
    /// decoherence.
    Before,

    /// Noise associated with the quantum measurement interaction itself.
    ///
    /// This includes measurement backaction and imperfections in the
    /// measurement process.
    During,

    /// Noise associated with the physical readout process after measurement.
    ///
    /// Examples include detector/readout assignment errors.
    Readout,

    /// Noise associated with the complete semantic measurement boundary.
    ///
    /// This is appropriate when the noise model itself owns the distinction
    /// between physical stages.
    WholeOperation,
}

impl Default for MeasurementNoisePhase {
    fn default() -> Self {
        Self::WholeOperation
    }
}

impl fmt::Display for MeasurementNoisePhase {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Before => formatter.write_str("before"),
            Self::During => formatter.write_str("during"),
            Self::Readout => formatter.write_str("readout"),
            Self::WholeOperation => formatter.write_str("whole-operation"),
        }
    }
}

// ============================================================================
// Measurement noise scope
// ============================================================================

/// Semantic category of measurement-related noise.
///
/// This is deliberately broader than classical readout error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MeasurementNoiseScope {
    /// Noise affecting the quantum state immediately before measurement.
    PreMeasurement,

    /// Noise generated by the measurement interaction/backaction.
    MeasurementProcess,

    /// Noise affecting physical readout.
    Readout,

    /// Noise affecting conversion/assignment of physical results to logical
    /// classical outcomes.
    Assignment,

    /// Noise covering the complete measurement operation.
    WholeMeasurement,
}

impl Default for MeasurementNoiseScope {
    fn default() -> Self {
        Self::WholeMeasurement
    }
}

impl MeasurementNoiseScope {
    /// Returns the canonical phase corresponding to this scope.
    #[must_use]
    pub const fn default_phase(self) -> MeasurementNoisePhase {
        match self {
            Self::PreMeasurement => MeasurementNoisePhase::Before,
            Self::MeasurementProcess => MeasurementNoisePhase::During,
            Self::Readout | Self::Assignment => MeasurementNoisePhase::Readout,
            Self::WholeMeasurement => MeasurementNoisePhase::WholeOperation,
        }
    }

    /// Returns whether this scope represents readout/assignment behavior.
    #[must_use]
    pub const fn is_readout_related(self) -> bool {
        matches!(self, Self::Readout | Self::Assignment)
    }

    /// Returns whether this scope represents quantum measurement backaction.
    #[must_use]
    pub const fn is_measurement_process(self) -> bool {
        matches!(self, Self::MeasurementProcess)
    }
}

impl fmt::Display for MeasurementNoiseScope {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PreMeasurement => formatter.write_str("pre-measurement"),
            Self::MeasurementProcess => formatter.write_str("measurement-process"),
            Self::Readout => formatter.write_str("readout"),
            Self::Assignment => formatter.write_str("assignment"),
            Self::WholeMeasurement => formatter.write_str("whole-measurement"),
        }
    }
}

// ============================================================================
// Measurement target
// ============================================================================

/// Canonical quantum resource associated with a measurement-noise binding.
///
/// The variants directly use the canonical Quantum IR identity types.
///
/// No ZQN-specific qubit identity is introduced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MeasurementNoiseTarget {
    /// Logical quantum resource.
    Logical(QubitId),

    /// Physical quantum resource.
    Physical(PhysicalQubitId),
}

impl MeasurementNoiseTarget {
    /// Creates a logical measurement target.
    #[must_use]
    pub const fn logical(qubit: QubitId) -> Self {
        Self::Logical(qubit)
    }

    /// Creates a physical measurement target.
    #[must_use]
    pub const fn physical(qubit: PhysicalQubitId) -> Self {
        Self::Physical(qubit)
    }

    /// Returns the logical qubit if this is a logical target.
    #[must_use]
    pub const fn logical_qubit(self) -> Option<QubitId> {
        match self {
            Self::Logical(qubit) => Some(qubit),
            Self::Physical(_) => None,
        }
    }

    /// Returns the physical qubit if this is a physical target.
    #[must_use]
    pub const fn physical_qubit(self) -> Option<PhysicalQubitId> {
        match self {
            Self::Logical(_) => None,
            Self::Physical(qubit) => Some(qubit),
        }
    }

    /// Returns whether the target is logical.
    #[must_use]
    pub const fn is_logical(self) -> bool {
        matches!(self, Self::Logical(_))
    }

    /// Returns whether the target is physical.
    #[must_use]
    pub const fn is_physical(self) -> bool {
        matches!(self, Self::Physical(_))
    }

    fn as_noise_target(self) -> NoiseTarget {
        match self {
            Self::Logical(qubit) => NoiseTarget::logical_qubit(qubit),
            Self::Physical(qubit) => NoiseTarget::physical_qubit(qubit),
        }
    }
}

impl From<QubitId> for MeasurementNoiseTarget {
    fn from(qubit: QubitId) -> Self {
        Self::Logical(qubit)
    }
}

impl From<PhysicalQubitId> for MeasurementNoiseTarget {
    fn from(qubit: PhysicalQubitId) -> Self {
        Self::Physical(qubit)
    }
}

impl fmt::Display for MeasurementNoiseTarget {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Logical(qubit) => write!(formatter, "{qubit}"),
            Self::Physical(qubit) => write!(formatter, "{qubit}"),
        }
    }
}

// ============================================================================
// Binding
// ============================================================================

/// Immutable measurement-noise binding.
///
/// This type connects a canonical measurement operation to the semantic
/// measurement resources and the scope at which noise is being requested.
///
/// It deliberately does not contain a concrete channel or fault. The
/// `NoiseModel` selects those later.
///
/// # Identity
///
/// `operation_id` identifies the canonical measurement operation.
///
/// # Targets
///
/// `targets` contains canonical logical or physical quantum resources.
///
/// # Scope
///
/// `scope` describes what part of the measurement is affected.
///
/// # Phase
///
/// `phase` describes the semantic temporal boundary.
///
/// # Ordering
///
/// Target order is preserved exactly. This is important for joint or
/// correlated measurement models where operand order may carry semantic
/// information.
///
/// The constructor rejects duplicate resources but never sorts the supplied
/// targets.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MeasurementNoiseBinding {
    operation_id: OperationId,
    targets: Vec<MeasurementNoiseTarget>,
    scope: MeasurementNoiseScope,
    phase: MeasurementNoisePhase,
}

impl MeasurementNoiseBinding {
    /// Creates a measurement-noise binding for an existing canonical
    /// measurement operation.
    ///
    /// At least one quantum target is required.
    ///
    /// No machine-size limit is imposed.
    pub fn new(
        operation_id: OperationId,
        targets: Vec<MeasurementNoiseTarget>,
    ) -> MeasurementNoiseResult<Self> {
        Self::with_scope(
            operation_id,
            targets,
            MeasurementNoiseScope::default(),
            MeasurementNoisePhase::default(),
        )
    }

    /// Creates a binding with an explicit noise scope.
    ///
    /// The phase defaults to the natural phase for the selected scope.
    pub fn with_scope(
        operation_id: OperationId,
        targets: Vec<MeasurementNoiseTarget>,
        scope: MeasurementNoiseScope,
        phase: MeasurementNoisePhase,
    ) -> MeasurementNoiseResult<Self> {
        if targets.is_empty() {
            return Err(MeasurementNoiseError::EmptyTargets);
        }

        validate_targets(&targets)?;

        Ok(Self {
            operation_id,
            targets,
            scope,
            phase,
        })
    }

    /// Creates a logical-qubit measurement binding.
    #[must_use]
    pub fn logical(
        operation_id: OperationId,
        qubits: Vec<QubitId>,
    ) -> MeasurementNoiseResult<Self> {
        let targets = qubits
            .into_iter()
            .map(MeasurementNoiseTarget::Logical)
            .collect();

        Self::new(operation_id, targets)
    }

    /// Creates a physical-qubit measurement binding.
    #[must_use]
    pub fn physical(
        operation_id: OperationId,
        qubits: Vec<PhysicalQubitId>,
    ) -> MeasurementNoiseResult<Self> {
        let targets = qubits
            .into_iter()
            .map(MeasurementNoiseTarget::Physical)
            .collect();

        Self::new(operation_id, targets)
    }

    /// Creates a logical readout-noise binding.
    #[must_use]
    pub fn logical_readout(
        operation_id: OperationId,
        qubits: Vec<QubitId>,
    ) -> MeasurementNoiseResult<Self> {
        Self::with_scope(
            operation_id,
            qubits
                .into_iter()
                .map(MeasurementNoiseTarget::Logical)
                .collect(),
            MeasurementNoiseScope::Readout,
            MeasurementNoisePhase::Readout,
        )
    }

    /// Creates a physical readout-noise binding.
    #[must_use]
    pub fn physical_readout(
        operation_id: OperationId,
        qubits: Vec<PhysicalQubitId>,
    ) -> MeasurementNoiseResult<Self> {
        Self::with_scope(
            operation_id,
            qubits
                .into_iter()
                .map(MeasurementNoiseTarget::Physical)
                .collect(),
            MeasurementNoiseScope::Readout,
            MeasurementNoisePhase::Readout,
        )
    }

    /// Returns the canonical measurement operation identity.
    #[must_use]
    pub const fn operation_id(&self) -> OperationId {
        self.operation_id
    }

    /// Returns the measurement-noise scope.
    #[must_use]
    pub const fn scope(&self) -> MeasurementNoiseScope {
        self.scope
    }

    /// Returns the semantic noise phase.
    #[must_use]
    pub const fn phase(&self) -> MeasurementNoisePhase {
        self.phase
    }

    /// Returns all measurement resources in their original semantic order.
    #[must_use]
    pub fn targets(&self) -> &[MeasurementNoiseTarget] {
        &self.targets
    }

    /// Returns the number of measurement resources.
    #[must_use]
    pub fn target_count(&self) -> usize {
        self.targets.len()
    }

    /// Returns whether all targets are logical.
    #[must_use]
    pub fn is_logical(&self) -> bool {
        self.targets
            .iter()
            .all(MeasurementNoiseTarget::is_logical)
    }

    /// Returns whether all targets are physical.
    #[must_use]
    pub fn is_physical(&self) -> bool {
        self.targets
            .iter()
            .all(MeasurementNoiseTarget::is_physical)
    }

    /// Returns whether this binding describes readout/assignment noise.
    #[must_use]
    pub const fn is_readout_noise(&self) -> bool {
        self.scope.is_readout_related()
    }

    /// Returns whether this binding describes measurement-process noise.
    #[must_use]
    pub const fn is_measurement_process_noise(&self) -> bool {
        self.scope.is_measurement_process()
    }

    /// Validates this binding.
    pub fn validate(&self) -> MeasurementNoiseResult<()> {
        if self.targets.is_empty() {
            return Err(MeasurementNoiseError::EmptyTargets);
        }

        validate_targets(&self.targets)
    }

    /// Converts this binding to the canonical ZQN noise-model request.
    ///
    /// The request contains the canonical operation identity and target
    /// identities. It does not select or execute noise.
    ///
    /// The measurement scope and phase remain available through the binding
    /// itself. They are deliberately not encoded as vendor-specific strings
    /// in the generic request.
    pub fn to_request(&self) -> MeasurementNoiseResult<NoiseApplicationRequest> {
        self.validate()?;

        let mut request =
            NoiseApplicationRequest::new().with_operation(self.operation_id);

        for target in &self.targets {
            request = request.with_target(target.as_noise_target());
        }

        Ok(request)
    }

    /// Applies a noise model to this measurement binding.
    ///
    /// This performs model selection only through the established
    /// `NoiseApplication` integration boundary.
    ///
    /// No channel, fault, simulator state, hardware operation, or RNG is
    /// executed here.
    pub fn apply(
        &self,
        application_id: NoiseApplicationId,
        model: &dyn NoiseModel,
        context: &ZqnContext,
    ) -> Result<NoiseApplication, crate::quantum::zqn::core::errors::ZqnError> {
        let request = self.to_request().map_err(|error| {
            crate::quantum::zqn::core::errors::ZqnError::new(
                crate::quantum::zqn::core::errors::ZqnErrorKind::Noise,
                crate::quantum::zqn::core::errors::ZqnErrorCode::InvalidNoiseModel,
                error.to_string(),
            )
        })?;

        NoiseApplication::from_model(
            application_id,
            model,
            request,
            context,
        )
    }
}

// ============================================================================
// Validation
// ============================================================================

/// Validates target structure.
///
/// The target collection must:
///
/// - be non-empty;
/// - contain no duplicate logical resources;
/// - contain no duplicate physical resources;
/// - use exactly one identity domain.
///
/// Mixing logical and physical resources is rejected because it makes the
/// semantic scope ambiguous: routing may not yet have happened, or a caller
/// may accidentally have combined pre- and post-routing identities.
fn validate_targets(
    targets: &[MeasurementNoiseTarget],
) -> MeasurementNoiseResult<()> {
    if targets.is_empty() {
        return Err(MeasurementNoiseError::EmptyTargets);
    }

    let logical = targets
        .iter()
        .any(MeasurementNoiseTarget::is_logical);

    let physical = targets
        .iter()
        .any(MeasurementNoiseTarget::is_physical);

    if logical && physical {
        return Err(MeasurementNoiseError::MixedTargetDomains);
    }

    if logical {
        let mut seen = BTreeSet::new();

        for target in targets {
            if let MeasurementNoiseTarget::Logical(qubit) = target {
                if !seen.insert(*qubit) {
                    return Err(MeasurementNoiseError::DuplicateTarget);
                }
            }
        }
    } else {
        let mut seen = BTreeSet::new();

        for target in targets {
            if let MeasurementNoiseTarget::Physical(qubit) = target {
                if !seen.insert(*qubit) {
                    return Err(MeasurementNoiseError::DuplicateTarget);
                }
            }
        }
    }

    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::identity::OperationId;

    #[test]
    fn logical_binding_preserves_target_order() {
        let binding = MeasurementNoiseBinding::logical(
            OperationId::new(10),
            vec![
                QubitId::new(7),
                QubitId::new(2),
                QubitId::new(19),
            ],
        )
        .expect("valid measurement binding");

        assert_eq!(
            binding.targets(),
            &[
                MeasurementNoiseTarget::Logical(QubitId::new(7)),
                MeasurementNoiseTarget::Logical(QubitId::new(2)),
                MeasurementNoiseTarget::Logical(QubitId::new(19)),
            ]
        );
    }

    #[test]
    fn physical_binding_uses_canonical_physical_identity() {
        let binding = MeasurementNoiseBinding::physical(
            OperationId::new(11),
            vec![
                PhysicalQubitId::new(4),
                PhysicalQubitId::new(8),
            ],
        )
        .expect("valid measurement binding");

        assert!(binding.is_physical());
        assert!(!binding.is_logical());
        assert_eq!(binding.target_count(), 2);
    }

    #[test]
    fn duplicate_logical_targets_are_rejected() {
        let result = MeasurementNoiseBinding::logical(
            OperationId::new(12),
            vec![
                QubitId::new(1),
                QubitId::new(1),
            ],
        );

        assert_eq!(
            result,
            Err(MeasurementNoiseError::DuplicateTarget)
        );
    }

    #[test]
    fn duplicate_physical_targets_are_rejected() {
        let result = MeasurementNoiseBinding::physical(
            OperationId::new(13),
            vec![
                PhysicalQubitId::new(3),
                PhysicalQubitId::new(3),
            ],
        );

        assert_eq!(
            result,
            Err(MeasurementNoiseError::DuplicateTarget)
        );
    }

    #[test]
    fn empty_targets_are_rejected() {
        let result = MeasurementNoiseBinding::new(
            OperationId::new(14),
            Vec::new(),
        );

        assert_eq!(
            result,
            Err(MeasurementNoiseError::EmptyTargets)
        );
    }

    #[test]
    fn logical_and_physical_targets_are_not_mixed() {
        let result = MeasurementNoiseBinding::new(
            OperationId::new(15),
            vec![
                MeasurementNoiseTarget::Logical(QubitId::new(1)),
                MeasurementNoiseTarget::Physical(
                    PhysicalQubitId::new(1),
                ),
            ],
        );

        assert_eq!(
            result,
            Err(MeasurementNoiseError::MixedTargetDomains)
        );
    }

    #[test]
    fn readout_scope_has_readout_phase() {
        let binding = MeasurementNoiseBinding::logical_readout(
            OperationId::new(16),
            vec![QubitId::new(0)],
        )
        .expect("valid readout binding");

        assert_eq!(
            binding.scope(),
            MeasurementNoiseScope::Readout
        );
        assert_eq!(
            binding.phase(),
            MeasurementNoisePhase::Readout
        );
        assert!(binding.is_readout_noise());
    }

    #[test]
    fn measurement_process_scope_is_distinct_from_readout() {
        let binding = MeasurementNoiseBinding::with_scope(
            OperationId::new(17),
            vec![MeasurementNoiseTarget::Logical(
                QubitId::new(0),
            )],
            MeasurementNoiseScope::MeasurementProcess,
            MeasurementNoisePhase::During,
        )
        .expect("valid measurement-process binding");

        assert!(binding.is_measurement_process_noise());
        assert!(!binding.is_readout_noise());
    }

    #[test]
    fn request_contains_canonical_operation_and_targets() {
        let binding = MeasurementNoiseBinding::logical(
            OperationId::new(18),
            vec![
                QubitId::new(3),
                QubitId::new(8),
            ],
        )
        .expect("valid measurement binding");

        let request = binding
            .to_request()
            .expect("request construction must succeed");

        assert_eq!(
            request.operation(),
            Some(OperationId::new(18))
        );

        assert_eq!(request.targets().len(), 2);
    }
}