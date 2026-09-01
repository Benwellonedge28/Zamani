//! Zamani Quantum Noise (ZQN) — Reset-operation noise semantics.
//!
//! # Ownership
//!
//! This module owns the ZQN-specific description of noise associated with a
//! quantum reset operation.
//!
//! It provides:
//!
//! - reset-noise request construction;
//! - logical and physical reset targets;
//! - arbitrary-size reset target collections;
//! - binding of a reset operation to a canonical ZQN `NoiseModel`;
//! - construction of a `NoiseApplication` for reset;
//! - validation of reset-specific structural invariants;
//! - deterministic, immutable reset-noise request data;
//! - stable integration boundaries for simulation, QEC, scheduling,
//!   routing, hardware, calibration, and runtime consumers.
//!
//! # Non-ownership
//!
//! This module does NOT own:
//!
//! - the ideal reset operation;
//! - canonical quantum IR;
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - quantum state evolution;
//! - reset-state preparation mathematics;
//! - quantum channels;
//! - probability distributions;
//! - fault generation;
//! - stochastic sampling;
//! - random-number generation;
//! - noise-model semantics;
//! - calibration data;
//! - hardware APIs;
//! - routing;
//! - scheduling;
//! - QEC decoding/correction;
//! - simulator state;
//! - serialization schemas;
//! - global resource limits;
//! - global mutable state.
//!
//! The ideal reset operation remains owned by the canonical Quantum IR.
//!
//! ZQN only describes how noise semantics are attached to that reset.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! quantum::ir
//!      │
//!      │ canonical reset operation
//!      ▼
//! ZQN reset operation adapter
//!      │
//!      ▼
//! NoiseApplicationRequest
//!      │
//!      ▼
//! NoiseModel
//!      │
//!      ▼
//! NoiseApplication
//!      │
//! ┌────┼─────────────┬──────────────┐
//! ▼    ▼             ▼              ▼
//! simulation   scheduling          QEC       benchmarking
//! ```
//!
//! This module is therefore an operation-specific adapter, not a second
//! quantum IR.
//!
//! # Canonical reset semantics
//!
//! A reset is semantically distinct from:
//!
//! ```text
//! measurement + conditional X
//! measurement + discard
//! preparation
//! ```
//!
//! A consumer must not replace a reset with another sequence merely because
//! that sequence happens to produce an equivalent ideal-state result under a
//! restricted noise model.
//!
//! Reset noise may include effects that are not represented by an ideal
//! reset-state assignment, including:
//!
//! - incomplete reset;
//! - residual population;
//! - thermalization;
//! - leakage;
//! - state-dependent reset failure;
//! - correlated reset errors;
//! - crosstalk during reset;
//! - duration-dependent noise;
//! - calibration-dependent errors;
//! - non-Markovian effects;
//! - transport-related effects;
//! - environment-dependent effects.
//!
//! Those mechanisms are represented by the canonical ZQN noise/channel/fault
//! layers rather than duplicated here.
//!
//! # Why this module exists
//!
//! A reset is an operation boundary at which noise can be different from gate,
//! measurement, preparation, or idle noise.
//!
//! Therefore reset noise needs a first-class integration point without making
//! reset itself a new noise model.
//!
//! ```text
//! ideal reset
//!     │
//!     ├── semantic operation → quantum::ir
//!     │
//!     └── physical uncertainty → ZQN
//! ```
//!
//! # Write once, scale everywhere
//!
//! This module deliberately contains no machine-size assumptions.
//!
//! It does NOT contain:
//!
//! ```text
//! MAX_QUBITS
//! MAX_RESET_TARGETS
//! MAX_RESETS
//! MAX_BATCH_SIZE
//! ```
//!
//! A reset request can contain any number of canonical quantum resources that
//! the surrounding IR and execution environment can represent and the caller
//! permits through its resource policy.
//!
//! The number of targets is data, not a semantic constant.
//!
//! A single-qubit reset is therefore represented naturally as a one-element
//! target collection rather than as a fundamentally different abstraction.
//!
//! This permits the same abstraction to work for:
//!
//! - one physical qubit;
//! - many physical qubits;
//! - logical qubits;
//! - distributed resources;
//! - large generated programs;
//! - future resource models supported by the canonical ZQN target abstraction.
//!
//! # Important distinction about "infinity"
//!
//! "Infinity" means that this semantic API does not impose an artificial finite
//! machine-size ceiling.
//!
//! It does NOT mean that Rust collections, RAM, storage, CPU, a simulator,
//! network, or physical quantum hardware have infinite capacity.
//!
//! Actual resource governance belongs to the surrounding ZQN context/runtime
//! policy.
//!
//! # Canonical quantum identities
//!
//! This module uses the canonical identities from:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! Specifically:
//!
//! ```text
//! QubitId
//! PhysicalQubitId
//! ```
//!
//! ZQN MUST NOT define:
//!
//! ```text
//! ResetQubitId
//! ZqnQubitId
//! ResetPhysicalQubitId
//! ```
//!
//! or equivalent competing wrappers.
//!
//! The repository explicitly establishes `quantum::ir::qubit` as the
//! authoritative quantum-resource identity boundary. 
//!
//! # Logical versus physical reset
//!
//! A reset request may target either:
//!
//! ```text
//! LogicalQubit(QubitId)
//! PhysicalQubit(PhysicalQubitId)
//! ```
//!
//! Logical targets are appropriate before placement/routing.
//!
//! Physical targets are appropriate after placement/lowering or when a target
//! is already expressed in physical-resource terms.
//!
//! This module never performs logical-to-physical mapping.
//!
//! That responsibility belongs to routing/placement.
//!
//! # Operation identity
//!
//! When the reset corresponds to a concrete operation in the canonical IR,
//! callers should provide its canonical `OperationId`.
//!
//! The operation identity belongs to:
//!
//! ```text
//! crate::quantum::ir::identity::OperationId
//! ```
//!
//! ZQN does not create another operation identity.
//!
//! An absent operation ID is allowed for contexts where the reset is being
//! described before a concrete IR operation has been assigned an identity.
//!
//! Such a request remains declarative and does not imply execution.
//!
//! # Noise model integration
//!
//! Noise generation belongs to:
//!
//! ```text
//! crate::quantum::zqn::noise::model::NoiseModel
//! ```
//!
//! This module never implements a noise model.
//!
//! The integration is:
//!
//! ```text
//! ResetNoiseRequest
//!        │
//!        ▼
//! NoiseApplicationRequest
//!        │
//!        ▼
//! NoiseModel
//!        │
//!        ▼
//! NoiseApplication
//! ```
//!
//! This preserves one authoritative noise-model abstraction for gates,
//! preparation, reset, measurement, idle, pulse, transport, and future
//! operation classes.
//!
//! # Determinism
//!
//! This module contains no randomness.
//!
//! It MUST NOT:
//!
//! - call a global RNG;
//! - call a thread-local RNG;
//! - derive semantics from system time;
//! - derive semantics from memory addresses;
//! - use process identity as semantic input;
//! - use unordered hash-map iteration as semantic ordering;
//! - maintain global mutable state.
//!
//! Stochastic reset-noise generation is owned by the noise/simulation layers.
//!
//! The resulting `NoiseApplication` is deterministic for the same:
//!
//! ```text
//! model
//! + model revision
//! + request
//! + ZqnContext
//! ```
//!
//! subject to the deterministic guarantees declared by the selected model.
//!
//! # Parallel determinism
//!
//! This module does not depend on thread scheduling.
//!
//! A reset request is immutable value data and can safely be constructed and
//! inspected from multiple execution strategies.
//!
//! If a model performs stochastic realization, deterministic execution must be
//! governed by the repository's explicit ZQN reproducibility context rather
//! than by this module.
//!
//! # Resource safety
//!
//! This module does not impose a semantic maximum number of reset targets.
//!
//! A caller processing untrusted input should apply `ZqnContext` resource
//! policy before constructing arbitrarily large target collections.
//!
//! This prevents a semantic module from accidentally becoming a machine-size
//! policy module.
//!
//! # Numerical safety
//!
//! This module does not own probabilities or numerical noise parameters.
//!
//! Reset-specific numerical parameters belong to the selected noise model,
//! channel, calibration, or probability subsystem.
//!
//! Consequently this module does not silently clamp, normalize, round, or
//! reinterpret numerical values.
//!
//! # Validation
//!
//! The following structural conditions are enforced here:
//!
//! 1. A reset request must have at least one target.
//! 2. A reset target collection must not contain duplicate semantic targets.
//! 3. A concrete reset application must have a concrete `OperationId` when the
//!    caller explicitly requests operation-bound application.
//! 4. Target identity is preserved exactly.
//! 5. No logical/physical target conversion is performed.
//!
//! Whether a target actually exists is NOT validated here.
//!
//! Existence belongs to the canonical IR/program/hardware layers.
//!
//! # Serialization
//!
//! This module does not define a wire format.
//!
//! `ResetNoiseRequest` is intentionally composed of stable semantic values and
//! can therefore be serialized by the ZQN IO layer.
//!
//! The external schema must preserve:
//!
//! - operation identity when present;
//! - target domain;
//! - target identity;
//! - target ordering where ordering is semantically relevant.
//!
//! Serialization versioning belongs to `zqn::io`.
//!
//! # Thread safety
//!
//! `ResetNoiseRequest` contains immutable value data.
//!
//! It has no interior mutability and no global state.
//!
//! It is therefore safe to share between threads when the surrounding Rust
//! ownership model permits it.
//!
//! `NoiseModel` execution remains governed by the model's own `Send`/`Sync`
//! contract.
//!
//! # Security
//!
//! Reset-noise descriptions are data, not authorization.
//!
//! Possession of a reset request or noise application MUST NOT grant:
//!
//! - QPU access;
//! - calibration access;
//! - filesystem access;
//! - network access;
//! - execution credentials;
//! - hardware-control privileges.
//!
//! Untrusted reset target collections must be processed under the caller's
//! explicit resource and cancellation policy.
//!
//! # QEC integration
//!
//! QEC may consume reset noise through:
//!
//! ```text
//! ResetNoiseApplication
//!        │
//!        ▼
//! ZQN fault/channel semantics
//!        │
//!        ▼
//! QEC adapter
//!        │
//!        ▼
//! syndrome / decoding / correction
//! ```
//!
//! QEC remains responsible for code-specific interpretation.
//!
//! This is particularly important because the existing QEC subsystem already
//! has reset-fault concepts. The long-term architecture is for QEC to consume
//! the canonical ZQN noise/fault semantics rather than creating a second
//! universal reset-noise representation.
//!
//! # Scheduling integration
//!
//! The scheduler can associate reset timing with the resulting noise request.
//!
//! Reset duration is deliberately NOT stored here as an ad-hoc `f64` because
//! timing semantics belong to the canonical IR/hardware timing and ZQN context
//! layers.
//!
//! A model may inspect scheduling/context information when selecting noise.
//!
//! # Calibration integration
//!
//! Calibration is referenced by the model/context layer.
//!
//! This module does not embed mutable calibration state.
//!
//! A reset-noise application therefore remains reproducible against an
//! immutable calibration/noise snapshot supplied through the surrounding
//! context.
//!
//! # Hardware integration
//!
//! Hardware adapters expose abstract target capabilities and calibration state.
//!
//! This module contains no vendor-specific implementation.
//!
//! There must never be code here such as:
//!
//! ```text
//! if vendor == ...
//! ```
//!
//! Hardware-specific realization belongs outside ZQN.
//!
//! # Simulation integration
//!
//! Simulation consumes the `NoiseApplication` produced from a reset request.
//!
//! This module does not manipulate state vectors, density matrices, tensor
//! networks, trajectories, or hardware state.
//!
//! # Routing integration
//!
//! Routing can inspect reset-related noise after a logical-to-physical mapping
//! is available.
//!
//! This module does not perform placement.
//!
//! # Benchmarking integration
//!
//! Benchmarking can use reset applications/observations to calculate reset
//! error rates, fidelity, leakage, thermalization, or application-level
//! metrics.
//!
//! Benchmark methodology remains outside this module.
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
//! - no unsafe code.
//!
//! # Unsafe-code policy
//!
//! Unsafe Rust is forbidden.
//!
//! `#![forbid(unsafe_code)]` makes this a compile-time invariant.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use crate::quantum::ir::identity::OperationId;
use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::zqn::core::context::ZqnContext;
use crate::quantum::zqn::core::errors::{
    ZqnError,
    ZqnErrorCode,
    ZqnErrorKind,
    ZqnResult,
};
use crate::quantum::zqn::core::ids::NoiseApplicationId;
use crate::quantum::zqn::noise::application::NoiseApplication;
use crate::quantum::zqn::noise::model::{
    NoiseApplicationRequest,
    NoiseModel,
    NoiseTarget,
};

// ============================================================================
// Reset target
// ============================================================================

/// A canonical quantum resource targeted by a reset-noise request.
///
/// The resource identity is borrowed directly from the canonical Quantum IR.
/// No ZQN-specific qubit identity is introduced.
///
/// This enum intentionally mirrors the semantic distinction already present in
/// the ZQN `NoiseTarget` abstraction while giving reset callers a domain-safe
/// construction API.
///
/// # Scaling
///
/// The number of reset targets is represented by the containing collection,
/// not by this enum. There is no fixed one-/two-/N-qubit representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResetTarget {
    /// A logical qubit identified by the canonical Quantum IR.
    Logical(QubitId),

    /// A physical qubit identified by the canonical Quantum IR.
    Physical(PhysicalQubitId),
}

impl ResetTarget {
    /// Creates a logical reset target.
    #[must_use]
    pub const fn logical(qubit: QubitId) -> Self {
        Self::Logical(qubit)
    }

    /// Creates a physical reset target.
    #[must_use]
    pub const fn physical(qubit: PhysicalQubitId) -> Self {
        Self::Physical(qubit)
    }

    /// Converts this reset-specific target into the canonical ZQN
    /// `NoiseTarget`.
    #[must_use]
    pub const fn as_noise_target(self) -> NoiseTarget {
        match self {
            Self::Logical(qubit) => NoiseTarget::logical_qubit(qubit),
            Self::Physical(qubit) => NoiseTarget::physical_qubit(qubit),
        }
    }

    /// Returns the logical qubit when this is a logical target.
    #[must_use]
    pub const fn logical_qubit(self) -> Option<QubitId> {
        match self {
            Self::Logical(qubit) => Some(qubit),
            Self::Physical(_) => None,
        }
    }

    /// Returns the physical qubit when this is a physical target.
    #[must_use]
    pub const fn physical_qubit(self) -> Option<PhysicalQubitId> {
        match self {
            Self::Logical(_) => None,
            Self::Physical(qubit) => Some(qubit),
        }
    }
}

// ============================================================================
// Reset noise request
// ============================================================================

/// Declarative request describing where reset noise is to be applied.
///
/// This type does not execute a reset and does not execute noise.
///
/// It is an operation-specific value object used to construct the canonical
/// ZQN `NoiseApplicationRequest`.
///
/// # Operation identity
///
/// `operation` is optional because a reset request may be constructed before
/// an IR operation receives its canonical identity.
///
/// Use [`Self::with_operation`] when the request is tied to a concrete IR
/// operation.
///
/// # Targets
///
/// Targets are stored in insertion order.
///
/// Ordering is preserved so that serialization and deterministic downstream
/// processing do not depend on hash-map iteration.
///
/// Duplicate targets are rejected during validation.
///
/// # Empty requests
///
/// Unlike the generic ZQN `NoiseApplicationRequest`, which can represent
/// execution-wide/global model requests, a reset-specific request cannot be
/// empty because reset noise is intrinsically attached to reset resources.
///
/// This is an important semantic distinction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResetNoiseRequest {
    operation: Option<OperationId>,
    targets: Vec<ResetTarget>,
}

impl ResetNoiseRequest {
    /// Creates an empty reset request.
    ///
    /// The request is intentionally allowed to start empty so callers can use
    /// builder-style construction.
    ///
    /// Validation is performed by [`Self::validate`] and by
    /// [`Self::into_noise_request`].
    #[must_use]
    pub fn new() -> Self {
        Self {
            operation: None,
            targets: Vec::new(),
        }
    }

    /// Creates a request containing one reset target.
    ///
    /// This is a convenience constructor for the common one-resource case.
    ///
    /// It does not establish that the target exists.
    #[must_use]
    pub fn for_target(target: ResetTarget) -> Self {
        Self {
            operation: None,
            targets: vec![target],
        }
    }

    /// Creates a request from an explicit target collection.
    ///
    /// The collection is copied into the request so the resulting request is
    /// immutable with respect to the caller's original collection.
    ///
    /// Duplicate targets are rejected by [`Self::validate`].
    pub fn from_targets<I>(targets: I) -> ZqnResult<Self>
    where
        I: IntoIterator<Item = ResetTarget>,
    {
        let request = Self {
            operation: None,
            targets: targets.into_iter().collect(),
        };

        request.validate()?;
        Ok(request)
    }

    /// Associates the request with a canonical IR operation.
    #[must_use]
    pub const fn with_operation(mut self, operation: OperationId) -> Self {
        self.operation = Some(operation);
        self
    }

    /// Adds one reset target.
    ///
    /// Duplicate detection is deferred until validation so callers can build
    /// requests efficiently without repeatedly scanning a potentially large
    /// collection.
    #[must_use]
    pub fn with_target(mut self, target: ResetTarget) -> Self {
        self.targets.push(target);
        self
    }

    /// Adds many reset targets.
    ///
    /// The targets are appended in iterator order.
    ///
    /// This method does not impose a target-count ceiling.
    #[must_use]
    pub fn with_targets<I>(mut self, targets: I) -> Self
    where
        I: IntoIterator<Item = ResetTarget>,
    {
        self.targets.extend(targets);
        self
    }

    /// Returns the associated canonical IR operation identity, if present.
    #[must_use]
    pub const fn operation(&self) -> Option<OperationId> {
        self.operation
    }

    /// Returns the reset targets in deterministic insertion order.
    #[must_use]
    pub fn targets(&self) -> &[ResetTarget] {
        &self.targets
    }

    /// Returns the number of reset targets.
    ///
    /// This is descriptive data and must not be interpreted as a machine-size
    /// limit.
    #[must_use]
    pub fn target_count(&self) -> usize {
        self.targets.len()
    }

    /// Returns true if the request has no targets.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.targets.is_empty()
    }

    /// Validates the reset request's structural invariants.
    ///
    /// Validation does not check hardware or IR existence.
    pub fn validate(&self) -> ZqnResult<()> {
        if self.targets.is_empty() {
            return Err(ZqnError::new(
                ZqnErrorKind::Structure,
                ZqnErrorCode::InvalidStructure,
                "reset noise request must contain at least one target",
            ));
        }

        // `ResetTarget` implements `Ord`, allowing deterministic duplicate
        // detection without depending on randomized hash state.
        //
        // The copy is intentionally local. This validation is not performed
        // during ordinary target insertion so large request construction can
        // remain append-oriented.
        let mut canonical_targets = self.targets.clone();
        canonical_targets.sort_unstable();

        for pair in canonical_targets.windows(2) {
            if pair[0] == pair[1] {
                return Err(ZqnError::new(
                    ZqnErrorKind::Structure,
                    ZqnErrorCode::InvalidStructure,
                    "reset noise request contains duplicate targets",
                ));
            }
        }

        Ok(())
    }

    /// Converts this reset-specific request into the canonical ZQN noise
    /// application request.
    ///
    /// This is the primary integration boundary with `noise::model`.
    pub fn into_noise_request(self) -> ZqnResult<NoiseApplicationRequest> {
        self.validate()?;

        let mut request = NoiseApplicationRequest::new();

        if let Some(operation) = self.operation {
            request = request.with_operation(operation);
        }

        for target in self.targets {
            request = request.with_target(target.as_noise_target());
        }

        request.validate()?;

        Ok(request)
    }

    /// Returns a borrowed view suitable for conversion without consuming the
    /// reset request.
    ///
    /// This method avoids requiring callers to clone their semantic request
    /// merely to construct a model request.
    pub fn to_noise_request(&self) -> ZqnResult<NoiseApplicationRequest> {
        self.clone().into_noise_request()
    }

    /// Constructs a canonical ZQN noise application from this reset request.
    ///
    /// No channel, fault, or quantum state is executed here.
    ///
    /// The selected `NoiseModel` determines the actual reset-noise semantics.
    pub fn apply_model(
        &self,
        id: NoiseApplicationId,
        model: &dyn NoiseModel,
        context: &ZqnContext,
    ) -> ZqnResult<NoiseApplication> {
        self.validate()?;

        let request = self.to_noise_request()?;

        NoiseApplication::from_model(id, model, request, context)
    }

    /// Constructs a reset request for one logical qubit and one operation.
    ///
    /// This convenience function still uses the canonical IR identities and
    /// canonical `OperationId`.
    #[must_use]
    pub const fn logical(
        operation: OperationId,
        qubit: QubitId,
    ) -> Self {
        Self {
            operation: Some(operation),
            targets: vec![ResetTarget::Logical(qubit)],
        }
    }

    /// Constructs a reset request for one physical qubit and one operation.
    ///
    /// This is useful after placement/lowering.
    #[must_use]
    pub const fn physical(
        operation: OperationId,
        qubit: PhysicalQubitId,
    ) -> Self {
        Self {
            operation: Some(operation),
            targets: vec![ResetTarget::Physical(qubit)],
        }
    }
}

impl Default for ResetNoiseRequest {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Reset-noise application
// ============================================================================

/// A reset-specific wrapper around the canonical ZQN `NoiseApplication`.
///
/// This wrapper is intentionally thin.
///
/// The canonical application remains the authoritative representation of:
///
/// - model identity;
/// - model revision;
/// - noise request;
/// - selected effects;
/// - semantic guarantee.
///
/// `ResetNoiseApplication` exists to give reset-specific downstream code a
/// domain-safe type without duplicating the universal application model.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResetNoiseApplication {
    request: ResetNoiseRequest,
    application: NoiseApplication,
}

impl ResetNoiseApplication {
    /// Creates a reset-noise application from an already constructed canonical
    /// ZQN application.
    ///
    /// The application must have the same semantic targets as `request`.
    pub fn new(
        request: ResetNoiseRequest,
        application: NoiseApplication,
    ) -> ZqnResult<Self> {
        request.validate()?;

        let canonical_request = request.to_noise_request()?;

        if application.request() != &canonical_request {
            return Err(ZqnError::new(
                ZqnErrorKind::Application,
                ZqnErrorCode::InvalidNoiseApplication,
                "reset noise application does not match its reset request",
            ));
        }

        Ok(Self {
            request,
            application,
        })
    }

    /// Constructs reset noise by evaluating a canonical ZQN noise model.
    pub fn from_model(
        request: ResetNoiseRequest,
        id: NoiseApplicationId,
        model: &dyn NoiseModel,
        context: &ZqnContext,
    ) -> ZqnResult<Self> {
        let application = request.apply_model(id, model, context)?;

        Self::new(request, application)
    }

    /// Returns the reset request.
    #[must_use]
    pub fn request(&self) -> &ResetNoiseRequest {
        &self.request
    }

    /// Returns the canonical ZQN noise application.
    #[must_use]
    pub fn application(&self) -> &NoiseApplication {
        &self.application
    }

    /// Returns the canonical noise-application identity.
    #[must_use]
    pub const fn id(&self) -> NoiseApplicationId {
        self.application.id()
    }

    /// Returns the model identity used for this reset application.
    #[must_use]
    pub const fn model_id(&self) -> crate::quantum::zqn::core::ids::NoiseModelId {
        self.application.model_id()
    }

    /// Returns the model revision used for this reset application.
    #[must_use]
    pub const fn model_revision(
        &self,
    ) -> crate::quantum::zqn::noise::model::NoiseModelRevision {
        self.application.model_revision()
    }

    /// Returns the semantic guarantee declared by the selected noise model.
    #[must_use]
    pub const fn guarantee(
        &self,
    ) -> crate::quantum::zqn::noise::model::NoiseSemanticGuarantee {
        self.application.guarantee()
    }

    /// Returns the canonical ZQN application status.
    #[must_use]
    pub const fn status(
        &self,
    ) -> crate::quantum::zqn::noise::application::NoiseApplicationStatus {
        self.application.status()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_request_is_rejected() {
        let request = ResetNoiseRequest::new();

        let result = request.validate();

        assert!(result.is_err());
    }

    #[test]
    fn logical_target_uses_canonical_qubit_identity() {
        let qubit = QubitId::new(7);
        let target = ResetTarget::logical(qubit);

        assert_eq!(target.logical_qubit(), Some(qubit));
        assert_eq!(target.physical_qubit(), None);

        assert_eq!(
            target.as_noise_target(),
            NoiseTarget::logical_qubit(qubit)
        );
    }

    #[test]
    fn physical_target_uses_canonical_physical_qubit_identity() {
        let qubit = PhysicalQubitId::new(7);
        let target = ResetTarget::physical(qubit);

        assert_eq!(target.logical_qubit(), None);
        assert_eq!(target.physical_qubit(), Some(qubit));

        assert_eq!(
            target.as_noise_target(),
            NoiseTarget::physical_qubit(qubit)
        );
    }

    #[test]
    fn duplicate_targets_are_rejected() {
        let qubit = QubitId::new(3);

        let request = ResetNoiseRequest::new()
            .with_target(ResetTarget::logical(qubit))
            .with_target(ResetTarget::logical(qubit));

        assert!(request.validate().is_err());
    }

    #[test]
    fn distinct_logical_targets_are_accepted() {
        let request = ResetNoiseRequest::new()
            .with_target(ResetTarget::logical(QubitId::new(1)))
            .with_target(ResetTarget::logical(QubitId::new(2)))
            .with_target(ResetTarget::logical(QubitId::new(3)));

        assert!(request.validate().is_ok());
        assert_eq!(request.target_count(), 3);
    }

    #[test]
    fn logical_and_physical_identity_domains_remain_distinct() {
        let logical = ResetTarget::logical(QubitId::new(9));
        let physical = ResetTarget::physical(PhysicalQubitId::new(9));

        assert_ne!(
            format!("{logical:?}"),
            format!("{physical:?}")
        );
    }

    #[test]
    fn request_preserves_operation_identity() {
        let operation = OperationId::new(42);
        let request = ResetNoiseRequest::logical(
            operation,
            QubitId::new(4),
        );

        assert_eq!(request.operation(), Some(operation));
    }

    #[test]
    fn arbitrary_target_count_is_data_driven() {
        let targets = (0usize..128usize)
            .map(QubitId::new)
            .map(ResetTarget::logical);

        let request = ResetNoiseRequest::from_targets(targets)
            .expect("generated distinct targets should validate");

        assert_eq!(request.target_count(), 128);
    }

    #[test]
    fn target_order_is_preserved() {
        let first = ResetTarget::logical(QubitId::new(9));
        let second = ResetTarget::logical(QubitId::new(2));
        let third = ResetTarget::logical(QubitId::new(7));

        let request = ResetNoiseRequest::new()
            .with_target(first)
            .with_target(second)
            .with_target(third);

        assert_eq!(request.targets(), &[first, second, third]);
    }

    #[test]
    fn request_conversion_preserves_targets() {
        let request = ResetNoiseRequest::new()
            .with_target(ResetTarget::logical(QubitId::new(1)))
            .with_target(ResetTarget::physical(PhysicalQubitId::new(8)));

        let noise_request = request
            .to_noise_request()
            .expect("valid reset request should convert");

        assert_eq!(noise_request.targets().len(), 2);
        assert_eq!(
            noise_request.targets()[0],
            NoiseTarget::logical_qubit(QubitId::new(1))
        );
        assert_eq!(
            noise_request.targets()[1],
            NoiseTarget::physical_qubit(PhysicalQubitId::new(8))
        );
    }
}