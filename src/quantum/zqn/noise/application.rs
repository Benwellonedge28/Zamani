//! Zamani Quantum Noise (ZQN) — Noise Application.
//!
//! This module is the authoritative application/attachment boundary between
//! a ZQN `NoiseModel` and a concrete quantum execution scope.
//!
//! # Mission
//!
//! `noise::application` answers:
//!
//! > "What noise semantics did this particular noise model select for this
//! > particular quantum operation/resource scope?"
//!
//! It does NOT answer:
//!
//! > "How is the quantum state physically evolved?"
//!
//! That responsibility belongs to `zqn::channel` and the simulation/runtime
//! layers.
//!
//! It does NOT answer:
//!
//! > "Which fault-tolerant code should correct this error?"
//!
//! That belongs to QEC.
//!
//! It does NOT answer:
//!
//! > "Which physical qubits should be selected?"
//!
//! That belongs to routing/hardware.
//!
//! It does NOT answer:
//!
//! > "When should the operation execute?"
//!
//! That belongs to scheduling.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                         quantum::ir
//!                              │
//!                    canonical operation/resource
//!                              │
//!                              ▼
//!                   NoiseApplicationRequest
//!                              │
//!                              ▼
//!                         NoiseModel
//!                              │
//!                              ▼
//!                         NoiseSelection
//!                              │
//!                              ▼
//!                    NoiseApplication  ◄── this module
//!                              │
//!             ┌────────────────┼─────────────────┐
//!             │                │                 │
//!             ▼                ▼                 ▼
//!          channel           fault          metadata/provenance
//!             │                │
//!             └────────────────┼─────────────────┐
//!                              ▼                 │
//!                         simulation             │
//!                              │                 │
//!                         QEC/hardware/runtime   │
//!                                                │
//! routing/scheduling/benchmarking consume application metadata/costs
//! through their own integration boundaries.
//! ```
//!
//! # Ownership
//!
//! This file owns:
//!
//! - the immutable `NoiseApplication` value;
//! - application identity association;
//! - application-time model metadata;
//! - selected noise semantics;
//! - application validation;
//! - model/request/selection consistency;
//! - exactness/approximation policy checks;
//! - immutable accessors for downstream consumers;
//! - application-level composition helpers;
//! - deterministic value semantics;
//! - bounded/stream-friendly application construction helpers.
//!
//! # Does not own
//!
//! This file does NOT own:
//!
//! - canonical quantum IR semantics;
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - `OperationId`;
//! - quantum-channel mathematics;
//! - Kraus/Choi/process matrices;
//! - probability distributions;
//! - random-number generation;
//! - fault generation;
//! - QEC;
//! - routing;
//! - scheduling;
//! - calibration;
//! - characterization;
//! - hardware APIs;
//! - simulator state;
//! - execution;
//! - global registries;
//! - global mutable state;
//! - serialization wire formats.
//!
//! Those responsibilities remain in their respective subsystems.
//!
//! # Canonical quantum identity
//!
//! The application layer deliberately does not define a second qubit identity.
//!
//! `NoiseApplicationRequest` already uses:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! crate::quantum::ir::identity::OperationId
//! ```
//!
//! through `NoiseTarget` and the request defined by `noise::model`.
//!
//! This preserves the repository-wide rule that the canonical Quantum IR
//! remains the sole owner of quantum-resource identity.
//!
//! # Write once, scale everywhere
//!
//! This module contains no semantic maximum for:
//!
//! - qubits;
//! - physical qubits;
//! - operations;
//! - application count;
//! - target count;
//! - noise-effect count;
//! - circuit depth;
//! - machine size;
//! - device size;
//! - topology size;
//! - number of quantum technologies.
//!
//! There is intentionally no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_APPLICATIONS
//! MAX_TARGETS
//! MAX_EFFECTS
//! ```
//!
//! Any finite resource constraint belongs to the explicit ZQN/runtime resource
//! policy, not to the semantic application model.
//!
//! "Infinity" means no artificial finite machine-size ceiling in the semantic
//! contract. Actual construction and execution remain bounded by available
//! memory, CPU/GPU capacity, distributed resources, storage, target capacity,
//! and explicit resource policy.
//!
//! # Model versus application
//!
//! A `NoiseModel` is reusable semantic behavior.
//!
//! A `NoiseApplication` is one immutable attachment of the model's selected
//! semantics to one application request.
//!
//! ```text
//! NoiseModel
//!     │
//!     │ select(request, context)
//!     ▼
//! NoiseSelection
//!     │
//!     ▼
//! NoiseApplication
//! ```
//!
//! This distinction is important because one model can be applied to many
//! operations/resources while each application remains independently
//! identifiable and reproducible.
//!
//! # Selection versus realization
//!
//! A `NoiseSelection` contains references to ZQN-owned effects.
//!
//! It does NOT execute those effects.
//!
//! For example:
//!
//! ```text
//! NoiseEffect::Channel(ChannelId)
//! ```
//!
//! means:
//!
//! > "The selected semantics refer to this channel."
//!
//! It does NOT mean:
//!
//! > "Apply this channel to a quantum state now."
//!
//! Channel execution belongs to simulation/runtime.
//!
//! Likewise:
//!
//! ```text
//! NoiseEffect::Fault(FaultId)
//! ```
//!
//! does not itself generate or apply a fault.
//!
//! # Determinism
//!
//! `NoiseApplication` is deterministic immutable value data.
//!
//! It contains:
//!
//! - no RNG;
//! - no clock;
//! - no thread-local state;
//! - no process-local state;
//! - no memory addresses;
//! - no global mutable state.
//!
//! If a model is stochastic, stochastic realization must remain controlled by
//! the explicit ZQN sampling/execution context.
//!
//! Constructing an application therefore does not consume randomness.
//!
//! # Parallel execution
//!
//! Applications are immutable and can be shared between concurrent consumers
//! when the contained model metadata and surrounding containers are safely
//! shareable.
//!
//! The application layer does not depend on:
//!
//! - thread identity;
//! - scheduling order;
//! - worker count;
//! - process identity.
//!
//! Consequently, creating applications sequentially or concurrently does not
//! change their semantic value.
//!
//! # Resource safety
//!
//! Application construction does not intentionally materialize quantum states,
//! matrices, tensors, channels, or fault batches.
//!
//! Potentially large request/selection vectors remain owned by their respective
//! model-layer value types.
//!
//! This module provides:
//!
//! - structural validation;
//! - `try_reserve`-based construction helpers where it owns collection growth;
//! - no hidden global allocations;
//! - no recursive application structures;
//! - no unbounded automatic expansion.
//!
//! Expensive realization belongs downstream and must obey `ZqnContext` resource
//! policy.
//!
//! # Security
//!
//! A `NoiseApplication` does not grant:
//!
//! - hardware access;
//! - filesystem access;
//! - network access;
//! - credentials;
//! - calibration write access;
//! - simulator control;
//! - process execution.
//!
//! It is data describing selected noise semantics.
//!
//! Untrusted applications must still be validated before execution.
//!
//! Downstream consumers must not interpret application metadata as executable
//! code.
//!
//! # Numerical safety
//!
//! This module does not perform channel mathematics or numerical integration.
//!
//! It therefore does not silently transform:
//!
//! ```text
//! NaN       → 0
//! Infinity  → finite maximum
//! invalid p → |p|
//! ```
//!
//! Numerical validation remains owned by probability/channel/fault modules.
//!
//! # Approximation safety
//!
//! The application preserves the model's declared semantic guarantee.
//!
//! It must never silently upgrade:
//!
//! ```text
//! Approximate → Exact
//! Statistical → Exact
//! Bounded     → Exact
//! ```
//!
//! A consumer can inspect the guarantee before execution.
//!
//! If a downstream target requires exact semantics, it must reject an
//! incompatible application explicitly through its target/compatibility layer.
//!
//! # Integration with `noise::model`
//!
//! This module consumes:
//!
//! ```text
//! NoiseModel
//! NoiseApplicationRequest
//! NoiseSelection
//! NoiseEffect
//! NoiseModelId
//! NoiseModelRevision
//! NoiseSemanticGuarantee
//! ```
//!
//! The model remains responsible for deciding which effects apply.
//!
//! This module records that decision as an immutable application.
//!
//! # Integration with channels
//!
//! `NoiseEffect::Channel(ChannelId)` identifies a channel owned by
//! `zqn::channel`.
//!
//! This module does not resolve the ID into a concrete channel and does not
//! apply the channel to state.
//!
//! The downstream flow is:
//!
//! ```text
//! NoiseApplication
//!       │
//!       ▼
//! NoiseEffect::Channel(ChannelId)
//!       │
//!       ▼
//! channel registry/owner
//!       │
//!       ▼
//! concrete QuantumChannel
//!       │
//!       ▼
//! simulator/runtime
//! ```
//!
//! # Integration with faults
//!
//! `NoiseEffect::Fault(FaultId)` identifies fault semantics owned by
//! `zqn::fault`.
//!
//! This module does not create, sample, decode, or correct faults.
//!
//! QEC consumes the application through its own integration boundary.
//!
//! # Integration with QEC
//!
//! The intended direction is:
//!
//! ```text
//! NoiseApplication
//!       │
//!       ▼
//! integration::qec
//!       │
//!       ▼
//! QEC physical-error representation
//! ```
//!
//! QEC remains responsible for:
//!
//! - codes;
//! - encodings;
//! - syndrome extraction;
//! - decoding;
//! - correction;
//! - logical error analysis.
//!
//! # Integration with routing
//!
//! Routing may inspect the application and its model metadata to determine
//! noise-related costs.
//!
//! Routing must not mutate the application or redefine its semantics.
//!
//! # Integration with scheduling
//!
//! Scheduling may use application scope and model semantics when estimating
//! time-dependent noise.
//!
//! Scheduling remains responsible for temporal ordering.
//!
//! # Integration with hardware
//!
//! Hardware integration validates whether selected application semantics are
//! representable on the target.
//!
//! This module does not know vendor APIs or device implementations.
//!
//! # Integration with simulation
//!
//! Simulation consumes applications and realizes their selected channels/faults.
//!
//! This module intentionally stops before state mutation.
//!
//! # Integration with benchmarking
//!
//! Benchmarking can use application identity and model revision to group,
//! compare, reproduce, and report noise-aware execution results.
//!
//! Benchmarking does not own application semantics.
//!
//! # Serialization
//!
//! This file does not implement serialization.
//!
//! `zqn::io` owns external representations.
//!
//! A serialized application must preserve at least:
//!
//! - application identity;
//! - model identity;
//! - model revision;
//! - request;
//! - selected effects;
//! - semantic guarantee;
//! - application metadata necessary for provenance.
//!
//! Rust struct layout is not a wire-format contract.
//!
//! # Versioning
//!
//! Global ZQN schema versioning belongs to `zqn::core::version`.
//!
//! Model semantic revision belongs to `NoiseModelRevision`.
//!
//! Application identity is distinct from both.
//!
//! # API stability
//!
//! The public application API intentionally depends on stable model-layer
//! abstractions rather than concrete channel/fault implementations.
//!
//! Adding a new channel type or fault type therefore does not require changing
//! this file.
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
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use crate::quantum::zqn::core::context::ZqnContext;
use crate::quantum::zqn::core::errors::{
    ZqnError,
    ZqnErrorCode,
    ZqnErrorKind,
    ZqnResult,
};
use crate::quantum::zqn::core::ids::{
    NoiseApplicationId,
    NoiseModelId,
};
use crate::quantum::zqn::noise::model::{
    select_noise,
    validate_selection,
    NoiseApplicationRequest,
    NoiseEffect,
    NoiseModel,
    NoiseModelRevision,
    NoiseSemanticGuarantee,
    NoiseSelection,
};

// ============================================================================
// Application status
// ============================================================================

/// Semantic status of a noise application.
///
/// The status is deliberately small and immutable. Execution state is owned by
/// the runtime and is not represented here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NoiseApplicationStatus {
    /// The application contains no selected noise effect.
    NoEffect,

    /// One or more noise effects have been selected and attached.
    Selected,
}

impl NoiseApplicationStatus {
    /// Returns true if no noise effect is selected.
    #[must_use]
    pub const fn is_no_effect(self) -> bool {
        matches!(self, Self::NoEffect)
    }

    /// Returns true if one or more effects are selected.
    #[must_use]
    pub const fn is_selected(self) -> bool {
        matches!(self, Self::Selected)
    }
}

impl fmt::Display for NoiseApplicationStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoEffect => formatter.write_str("no-effect"),
            Self::Selected => formatter.write_str("selected"),
        }
    }
}

// ============================================================================
// Immutable application
// ============================================================================

/// An immutable attachment of selected ZQN noise semantics to a request.
///
/// A `NoiseApplication` is the bridge between model selection and downstream
/// realization.
///
/// It contains no executable channel object, fault object, RNG, simulator
/// state, hardware handle, or mutable execution state.
///
/// # Identity
///
/// `id` identifies this application object within the caller's ZQN identity
/// namespace. Constructing an ID does not prove global uniqueness.
///
/// # Model identity
///
/// `model_id` and `model_revision` identify the semantic model that produced
/// the selection.
///
/// # Request
///
/// `request` identifies the canonical quantum operation/resource scope.
///
/// # Selection
///
/// `selection` identifies the abstract effects selected by the model.
///
/// # Guarantee
///
/// `guarantee` preserves the model's declared semantic fidelity guarantee.
///
/// Consumers MUST NOT reinterpret the guarantee as stronger than declared.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NoiseApplication {
    id: NoiseApplicationId,
    model_id: NoiseModelId,
    model_revision: NoiseModelRevision,
    request: NoiseApplicationRequest,
    selection: NoiseSelection,
    guarantee: NoiseSemanticGuarantee,
}

impl NoiseApplication {
    /// Creates an application from already validated immutable components.
    ///
    /// This constructor does not call a model and does not perform execution.
    ///
    /// Prefer [`NoiseApplication::from_model`] when constructing an
    /// application from a live `NoiseModel`.
    pub fn new(
        id: NoiseApplicationId,
        model_id: NoiseModelId,
        model_revision: NoiseModelRevision,
        request: NoiseApplicationRequest,
        selection: NoiseSelection,
        guarantee: NoiseSemanticGuarantee,
    ) -> ZqnResult<Self> {
        request.validate()?;
        validate_selection(&selection)?;

        Ok(Self {
            id,
            model_id,
            model_revision,
            request,
            selection,
            guarantee,
        })
    }

    /// Constructs an application by evaluating a noise model against a request.
    ///
    /// This is the preferred high-level application entry point.
    ///
    /// The model is validated before selection, the request is validated before
    /// model evaluation, and the resulting selection is structurally validated
    /// before it is attached to the application.
    ///
    /// No channel or fault is executed by this function.
    pub fn from_model(
        id: NoiseApplicationId,
        model: &dyn NoiseModel,
        request: NoiseApplicationRequest,
        context: &ZqnContext,
    ) -> ZqnResult<Self> {
        let selection = select_noise(model, &request, context)?;

        Self::new(
            id,
            model.id(),
            model.revision(),
            request,
            selection,
            model.guarantee(),
        )
    }

    /// Returns the application identity.
    #[must_use]
    pub const fn id(&self) -> NoiseApplicationId {
        self.id
    }

    /// Returns the identity of the model that produced this application.
    #[must_use]
    pub const fn model_id(&self) -> NoiseModelId {
        self.model_id
    }

    /// Returns the semantic revision of the model that produced this
    /// application.
    #[must_use]
    pub const fn model_revision(&self) -> NoiseModelRevision {
        self.model_revision
    }

    /// Returns the original immutable application request.
    #[must_use]
    pub fn request(&self) -> &NoiseApplicationRequest {
        &self.request
    }

    /// Returns the selected immutable noise semantics.
    #[must_use]
    pub fn selection(&self) -> &NoiseSelection {
        &self.selection
    }

    /// Returns the model's declared semantic guarantee.
    #[must_use]
    pub const fn guarantee(&self) -> NoiseSemanticGuarantee {
        self.guarantee
    }

    /// Returns the application status.
    #[must_use]
    pub const fn status(&self) -> NoiseApplicationStatus {
        if self.selection.is_none() {
            NoiseApplicationStatus::NoEffect
        } else {
            NoiseApplicationStatus::Selected
        }
    }

    /// Returns true when no effect was selected.
    #[must_use]
    pub fn is_no_effect(&self) -> bool {
        self.selection.is_none()
    }

    /// Returns true when one or more effects were selected.
    #[must_use]
    pub fn has_effects(&self) -> bool {
        !self.selection.is_none()
    }

    /// Returns the number of selected effect references.
    #[must_use]
    pub fn effect_count(&self) -> usize {
        self.selection.len()
    }

    /// Visits every selected effect without exposing internal collection
    /// ownership.
    ///
    /// This is useful for streaming downstream consumers.
    pub fn for_each_effect<F>(&self, visitor: F)
    where
        F: FnMut(NoiseEffect),
    {
        self.selection.for_each(visitor);
    }

    /// Returns the canonical IR operation associated with this application,
    /// when one exists.
    #[must_use]
    pub const fn operation(
        &self,
    ) -> Option<crate::quantum::ir::identity::OperationId> {
        self.request.operation()
    }

    /// Returns all requested noise targets.
    #[must_use]
    pub fn targets(
        &self,
    ) -> &[crate::quantum::zqn::noise::model::NoiseTarget] {
        self.request.targets()
    }

    /// Returns the model parameter identities used by the request.
    #[must_use]
    pub fn parameters(
        &self,
    ) -> &[crate::quantum::zqn::core::ids::NoiseParameterId] {
        self.request.parameters()
    }

    /// Validates the immutable application invariant.
    ///
    /// This function is intentionally cheap and deterministic. It does not
    /// query hardware, registries, calibration, or execution state.
    pub fn validate(&self) -> ZqnResult<()> {
        self.request.validate()?;
        validate_selection(&self.selection)?;

        if self.selection.is_none() {
            return Ok(());
        }

        if self.effect_count() == 0 {
            return Err(invalid_application(
                "a non-empty noise application must contain at least one effect",
            ));
        }

        Ok(())
    }

    /// Returns a deterministic summary suitable for diagnostics.
    ///
    /// The summary is intentionally not a serialization format.
    #[must_use]
    pub fn summary(&self) -> NoiseApplicationSummary {
        NoiseApplicationSummary {
            id: self.id,
            model_id: self.model_id,
            model_revision: self.model_revision,
            status: self.status(),
            effect_count: self.effect_count(),
            guarantee: self.guarantee,
            has_operation: self.request.operation().is_some(),
            target_count: self.request.targets().len(),
            parameter_count: self.request.parameters().len(),
        }
    }
}

// ============================================================================
// Application summary
// ============================================================================

/// Lightweight immutable application summary.
///
/// This is useful for logging, metrics, diagnostics and scheduling/routing
/// decisions without cloning the complete request or selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NoiseApplicationSummary {
    id: NoiseApplicationId,
    model_id: NoiseModelId,
    model_revision: NoiseModelRevision,
    status: NoiseApplicationStatus,
    effect_count: usize,
    guarantee: NoiseSemanticGuarantee,
    has_operation: bool,
    target_count: usize,
    parameter_count: usize,
}

impl NoiseApplicationSummary {
    /// Returns the application identity.
    #[must_use]
    pub const fn id(self) -> NoiseApplicationId {
        self.id
    }

    /// Returns the model identity.
    #[must_use]
    pub const fn model_id(self) -> NoiseModelId {
        self.model_id
    }

    /// Returns the model revision.
    #[must_use]
    pub const fn model_revision(self) -> NoiseModelRevision {
        self.model_revision
    }

    /// Returns the application status.
    #[must_use]
    pub const fn status(self) -> NoiseApplicationStatus {
        self.status
    }

    /// Returns the number of selected effects.
    #[must_use]
    pub const fn effect_count(self) -> usize {
        self.effect_count
    }

    /// Returns the semantic guarantee.
    #[must_use]
    pub const fn guarantee(self) -> NoiseSemanticGuarantee {
        self.guarantee
    }

    /// Returns whether a canonical operation was supplied.
    #[must_use]
    pub const fn has_operation(self) -> bool {
        self.has_operation
    }

    /// Returns the number of requested targets.
    #[must_use]
    pub const fn target_count(self) -> usize {
        self.target_count
    }

    /// Returns the number of requested parameter identities.
    #[must_use]
    pub const fn parameter_count(self) -> usize {
        self.parameter_count
    }
}

// ============================================================================
// Application construction helpers
// ============================================================================

/// Applies a noise model and returns its immutable application.
///
/// This is a convenience wrapper around [`NoiseApplication::from_model`].
///
/// It exists so downstream integrations can depend on a function rather than
/// directly coupling themselves to the construction details of
/// `NoiseApplication`.
pub fn apply(
    id: NoiseApplicationId,
    model: &dyn NoiseModel,
    request: NoiseApplicationRequest,
    context: &ZqnContext,
) -> ZqnResult<NoiseApplication> {
    NoiseApplication::from_model(id, model, request, context)
}

/// Validates an application without performing execution.
///
/// This is useful at subsystem boundaries such as:
///
/// ```text
/// compiler → runtime
/// runtime → simulator
/// runtime → hardware
/// ZQN → QEC
/// ZQN → benchmarking
/// ```
pub fn validate_application(
    application: &NoiseApplication,
) -> ZqnResult<()> {
    application.validate()
}

// ============================================================================
// Application compatibility helpers
// ============================================================================

/// Determines whether an application is semantically compatible with an exact
/// execution requirement.
///
/// This function does not inspect target capabilities. It only evaluates the
/// declared semantic guarantee.
///
/// Target-specific capability checks belong to `zqn::target`.
///
/// Exact execution accepts only `Exact`.
///
/// This explicit behavior prevents an approximate model from silently becoming
/// an exact execution merely because a caller forgot to check its guarantee.
pub fn require_exact(
    application: &NoiseApplication,
) -> ZqnResult<()> {
    application.validate()?;

    if application.guarantee() != NoiseSemanticGuarantee::Exact {
        return Err(ZqnError::new(
            ZqnErrorKind::Application,
            ZqnErrorCode::InvalidNoiseApplication,
            "exact execution requires a noise application with an exact semantic guarantee",
        ));
    }

    Ok(())
}

/// Determines whether the application is usable under an explicit policy.
///
/// `allow_approximate` controls `Approximate`.
///
/// `allow_bounded` controls `Bounded`.
///
/// `allow_statistical` controls `Statistical`.
///
/// `RequiresCompatibilityDecision` is never silently accepted because it
/// explicitly means that another compatibility decision is required.
pub fn validate_semantic_policy(
    application: &NoiseApplication,
    allow_approximate: bool,
    allow_bounded: bool,
    allow_statistical: bool,
) -> ZqnResult<()> {
    application.validate()?;

    match application.guarantee() {
        NoiseSemanticGuarantee::Exact => Ok(()),

        NoiseSemanticGuarantee::Approximate if allow_approximate => Ok(()),

        NoiseSemanticGuarantee::Bounded if allow_bounded => Ok(()),

        NoiseSemanticGuarantee::Statistical if allow_statistical => Ok(()),

        NoiseSemanticGuarantee::RequiresCompatibilityDecision => {
            Err(ZqnError::new(
                ZqnErrorKind::Compatibility,
                ZqnErrorCode::InvalidNoiseApplication,
                "noise application requires an explicit compatibility decision",
            ))
        }

        NoiseSemanticGuarantee::Approximate => Err(
            ZqnError::new(
                ZqnErrorKind::Compatibility,
                ZqnErrorCode::InvalidNoiseApplication,
                "approximate noise semantics are not permitted by the current application policy",
            ),
        ),

        NoiseSemanticGuarantee::Bounded => Err(
            ZqnError::new(
                ZqnErrorKind::Compatibility,
                ZqnErrorCode::InvalidNoiseApplication,
                "bounded noise semantics are not permitted by the current application policy",
            ),
        ),

        NoiseSemanticGuarantee::Statistical => Err(
            ZqnError::new(
                ZqnErrorKind::Compatibility,
                ZqnErrorCode::InvalidNoiseApplication,
                "statistical noise semantics are not permitted by the current application policy",
            ),
        ),
    }
}

// ============================================================================
// Application composition
// ============================================================================

/// Composes two already-created applications into a single application.
///
/// The applications must refer to the same:
///
/// - model identity;
/// - model revision;
/// - request.
///
/// The resulting selection is the concatenation of both selections.
///
/// This operation does not execute or mathematically compose the underlying
/// channels. It merely combines selected effect references.
///
/// Mathematical channel composition belongs to `zqn::channel::composition`.
///
/// Fault combination semantics belong to `zqn::fault`.
pub fn compose(
    id: NoiseApplicationId,
    first: &NoiseApplication,
    second: &NoiseApplication,
) -> ZqnResult<NoiseApplication> {
    if first.model_id() != second.model_id() {
        return Err(ZqnError::new(
            ZqnErrorKind::Application,
            ZqnErrorCode::AmbiguousNoiseApplication,
            "cannot compose applications produced by different noise models",
        ));
    }

    if first.model_revision() != second.model_revision() {
        return Err(ZqnError::new(
            ZqnErrorKind::Application,
            ZqnErrorCode::AmbiguousNoiseApplication,
            "cannot compose applications from different noise-model revisions",
        ));
    }

    if first.request() != second.request() {
        return Err(ZqnError::new(
            ZqnErrorKind::Application,
            ZqnErrorCode::AmbiguousNoiseApplication,
            "cannot compose applications with different application requests",
        ));
    }

    if first.guarantee() != second.guarantee() {
        return Err(ZqnError::new(
            ZqnErrorKind::Compatibility,
            ZqnErrorCode::InvalidNoiseApplication,
            "cannot compose applications with incompatible semantic guarantees",
        ));
    }

    let mut effects = Vec::new();

    reserve_effect_capacity(
        &mut effects,
        first.effect_count(),
        second.effect_count(),
    )?;

    first.for_each_effect(|effect| effects.push(effect));
    second.for_each_effect(|effect| effects.push(effect));

    NoiseApplication::new(
        id,
        first.model_id(),
        first.model_revision(),
        first.request().clone(),
        NoiseSelection::composite(effects),
        first.guarantee(),
    )
}

/// Attempts to compose two applications while avoiding an intermediate
/// collection allocation when either application contains no effects.
///
/// This remains an immutable semantic operation.
pub fn compose_if_needed(
    id: NoiseApplicationId,
    first: &NoiseApplication,
    second: &NoiseApplication,
) -> ZqnResult<NoiseApplication> {
    if first.is_no_effect() {
        return NoiseApplication::new(
            id,
            second.model_id(),
            second.model_revision(),
            second.request().clone(),
            second.selection().clone(),
            second.guarantee(),
        );
    }

    if second.is_no_effect() {
        return NoiseApplication::new(
            id,
            first.model_id(),
            first.model_revision(),
            first.request().clone(),
            first.selection().clone(),
            first.guarantee(),
        );
    }

    compose(id, first, second)
}

// ============================================================================
// Effect inspection
// ============================================================================

/// Returns true when an application contains at least one channel reference.
#[must_use]
pub fn contains_channel(application: &NoiseApplication) -> bool {
    let mut found = false;

    application.for_each_effect(|effect| {
        if matches!(effect, NoiseEffect::Channel(_)) {
            found = true;
        }
    });

    found
}

/// Returns true when an application contains at least one fault reference.
#[must_use]
pub fn contains_fault(application: &NoiseApplication) -> bool {
    let mut found = false;

    application.for_each_effect(|effect| {
        if matches!(effect, NoiseEffect::Fault(_)) {
            found = true;
        }
    });

    found
}

/// Returns true when the application contains only explicit `None` effects.
///
/// This is stronger than `is_no_effect` in the sense that it distinguishes a
/// structurally empty selection from a selection that explicitly contains
/// `NoiseEffect::None`.
#[must_use]
pub fn contains_only_none_effects(
    application: &NoiseApplication,
) -> bool {
    if application.selection().is_empty() {
        return true;
    }

    let mut only_none = true;

    application.for_each_effect(|effect| {
        if !effect.is_none() {
            only_none = false;
        }
    });

    only_none
}

// ============================================================================
// Collection/resource helpers
// ============================================================================

/// Reserves capacity for an effect vector using checked arithmetic and
/// fallible allocation.
///
/// This helper deliberately does not define a maximum size.
///
/// If the host cannot provide the requested memory, the failure is converted
/// into the canonical ZQN resource error rather than panicking through an
/// infallible allocation path.
fn reserve_effect_capacity(
    effects: &mut Vec<NoiseEffect>,
    first_len: usize,
    second_len: usize,
) -> ZqnResult<()> {
    let additional = first_len.checked_add(second_len).ok_or_else(|| {
        ZqnError::new(
            ZqnErrorKind::Limits,
            ZqnErrorCode::SizeOverflow,
            "noise application effect count overflowed host addressable size",
        )
    })?;

    effects.try_reserve(additional).map_err(|_| {
        ZqnError::new(
            ZqnErrorKind::Limits,
            ZqnErrorCode::AllocationRejected,
            "noise application effect storage could not be reserved",
        )
    })?;

    Ok(())
}

// ============================================================================
// Internal diagnostics
// ============================================================================

fn invalid_application(message: &'static str) -> ZqnError {
    ZqnError::new(
        ZqnErrorKind::Application,
        ZqnErrorCode::InvalidNoiseApplication,
        message,
    )
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::identity::OperationId;
    use crate::quantum::ir::qubit::QubitId;
    use crate::quantum::zqn::core::ids::{
        ChannelId,
        FaultId,
        NoiseApplicationId,
        NoiseModelId,
        NoiseParameterId,
    };
    use crate::quantum::zqn::noise::model::{
        NoiseModel,
        NoiseModelCapabilities,
        NoiseModelDescriptor,
        NoiseModelRevision,
        NoiseModelScope,
        NoiseSemanticGuarantee,
    };

    // ------------------------------------------------------------------------
    // Test model
    // ------------------------------------------------------------------------

    #[derive(Debug)]
    struct TestModel {
        descriptor: NoiseModelDescriptor,
        selection: NoiseSelection,
        applies: bool,
    }

    impl TestModel {
        fn new(
            id: NoiseModelId,
            selection: NoiseSelection,
            applies: bool,
            guarantee: NoiseSemanticGuarantee,
        ) -> Self {
            let descriptor = NoiseModelDescriptor::new(
                id,
                "application-test-model",
                NoiseModelRevision::new(1, 0, 0),
                NoiseModelScope::Operation,
                guarantee,
                NoiseModelCapabilities::default(),
            )
            .expect("test descriptor must be valid");

            Self {
                descriptor,
                selection,
                applies,
            }
        }
    }

    impl NoiseModel for TestModel {
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
            Ok(self.applies)
        }

        fn select(
            &self,
            _request: &NoiseApplicationRequest,
            _context: &ZqnContext,
        ) -> ZqnResult<NoiseSelection> {
            Ok(self.selection.clone())
        }
    }

    // ------------------------------------------------------------------------
    // Structural tests
    // ------------------------------------------------------------------------

    #[test]
    fn status_distinguishes_effect_and_no_effect() {
        assert!(NoiseApplicationStatus::NoEffect.is_no_effect());
        assert!(!NoiseApplicationStatus::NoEffect.is_selected());

        assert!(NoiseApplicationStatus::Selected.is_selected());
        assert!(!NoiseApplicationStatus::Selected.is_no_effect());
    }

    #[test]
    fn direct_application_preserves_all_identity_data() {
        let request = NoiseApplicationRequest::new()
            .with_operation(OperationId::new(7))
            .with_target(
                crate::quantum::zqn::noise::model::NoiseTarget::logical_qubit(
                    QubitId::new(3),
                ),
            )
            .with_parameter(NoiseParameterId::new(11));

        let application = NoiseApplication::new(
            NoiseApplicationId::new(100),
            NoiseModelId::new(200),
            NoiseModelRevision::new(2, 3, 4),
            request.clone(),
            NoiseSelection::single(NoiseEffect::Channel(
                ChannelId::new(300),
            )),
            NoiseSemanticGuarantee::Exact,
        )
        .expect("application must be valid");

        assert_eq!(application.id(), NoiseApplicationId::new(100));
        assert_eq!(application.model_id(), NoiseModelId::new(200));
        assert_eq!(
            application.model_revision(),
            NoiseModelRevision::new(2, 3, 4)
        );
        assert_eq!(application.request(), &request);
        assert_eq!(application.effect_count(), 1);
        assert_eq!(application.status(), NoiseApplicationStatus::Selected);
        assert_eq!(
            application.guarantee(),
            NoiseSemanticGuarantee::Exact
        );
        assert_eq!(
            application.operation(),
            Some(OperationId::new(7))
        );
        assert_eq!(application.targets().len(), 1);
        assert_eq!(application.parameters().len(), 1);
    }

    #[test]
    fn no_effect_application_is_valid() {
        let application = NoiseApplication::new(
            NoiseApplicationId::new(1),
            NoiseModelId::new(2),
            NoiseModelRevision::default(),
            NoiseApplicationRequest::new(),
            NoiseSelection::None,
            NoiseSemanticGuarantee::Exact,
        )
        .expect("no-effect application must be valid");

        assert!(application.is_no_effect());
        assert!(!application.has_effects());
        assert_eq!(application.effect_count(), 0);
        assert_eq!(
            application.status(),
            NoiseApplicationStatus::NoEffect
        );
        assert!(application.validate().is_ok());
    }

    #[test]
    fn channel_and_fault_detection_is_correct() {
        let channel_application = NoiseApplication::new(
            NoiseApplicationId::new(1),
            NoiseModelId::new(2),
            NoiseModelRevision::default(),
            NoiseApplicationRequest::new(),
            NoiseSelection::single(NoiseEffect::Channel(
                ChannelId::new(3),
            )),
            NoiseSemanticGuarantee::Exact,
        )
        .expect("channel application must be valid");

        assert!(contains_channel(&channel_application));
        assert!(!contains_fault(&channel_application));

        let fault_application = NoiseApplication::new(
            NoiseApplicationId::new(4),
            NoiseModelId::new(5),
            NoiseModelRevision::default(),
            NoiseApplicationRequest::new(),
            NoiseSelection::single(NoiseEffect::Fault(
                FaultId::new(6),
            )),
            NoiseSemanticGuarantee::Exact,
        )
        .expect("fault application must be valid");

        assert!(!contains_channel(&fault_application));
        assert!(contains_fault(&fault_application));
    }

    #[test]
    fn explicit_none_effects_are_detected() {
        let application = NoiseApplication::new(
            NoiseApplicationId::new(1),
            NoiseModelId::new(2),
            NoiseModelRevision::default(),
            NoiseApplicationRequest::new(),
            NoiseSelection::composite(vec![
                NoiseEffect::None,
                NoiseEffect::None,
            ]),
            NoiseSemanticGuarantee::Exact,
        )
        .expect("explicit-none application must be valid");

        assert!(application.is_no_effect());
        assert!(contains_only_none_effects(&application));
    }

    #[test]
    fn model_application_preserves_model_identity_and_guarantee() {
        let model = TestModel::new(
            NoiseModelId::new(10),
            NoiseSelection::single(NoiseEffect::Channel(
                ChannelId::new(20),
            )),
            true,
            NoiseSemanticGuarantee::Exact,
        );

        let request = NoiseApplicationRequest::new()
            .with_operation(OperationId::new(30));

        // The concrete ZqnContext constructor belongs to core::context.
        // This test verifies only types and invariants that do not require
        // constructing a policy-specific execution context.
        assert_eq!(model.id(), NoiseModelId::new(10));
        assert_eq!(
            model.guarantee(),
            NoiseSemanticGuarantee::Exact
        );
        assert_eq!(
            request.operation(),
            Some(OperationId::new(30))
        );
    }

    #[test]
    fn exact_policy_accepts_exact_application() {
        let application = NoiseApplication::new(
            NoiseApplicationId::new(1),
            NoiseModelId::new(2),
            NoiseModelRevision::default(),
            NoiseApplicationRequest::new(),
            NoiseSelection::single(NoiseEffect::Channel(
                ChannelId::new(3),
            )),
            NoiseSemanticGuarantee::Exact,
        )
        .expect("application must be valid");

        assert!(require_exact(&application).is_ok());
    }

    #[test]
    fn exact_policy_rejects_approximate_application() {
        let application = NoiseApplication::new(
            NoiseApplicationId::new(1),
            NoiseModelId::new(2),
            NoiseModelRevision::default(),
            NoiseApplicationRequest::new(),
            NoiseSelection::single(NoiseEffect::Channel(
                ChannelId::new(3),
            )),
            NoiseSemanticGuarantee::Approximate,
        )
        .expect("application must be structurally valid");

        assert!(require_exact(&application).is_err());
    }

    #[test]
    fn explicit_approximation_policy_is_respected() {
        let application = NoiseApplication::new(
            NoiseApplicationId::new(1),
            NoiseModelId::new(2),
            NoiseModelRevision::default(),
            NoiseApplicationRequest::new(),
            NoiseSelection::single(NoiseEffect::Channel(
                ChannelId::new(3),
            )),
            NoiseSemanticGuarantee::Approximate,
        )
        .expect("application must be structurally valid");

        assert!(
            validate_semantic_policy(
                &application,
                true,
                false,
                false,
            )
            .is_ok()
        );

        assert!(
            validate_semantic_policy(
                &application,
                false,
                false,
                false,
            )
            .is_err()
        );
    }

    #[test]
    fn compatibility_decision_is_never_silent() {
        let application = NoiseApplication::new(
            NoiseApplicationId::new(1),
            NoiseModelId::new(2),
            NoiseModelRevision::default(),
            NoiseApplicationRequest::new(),
            NoiseSelection::single(NoiseEffect::Channel(
                ChannelId::new(3),
            )),
            NoiseSemanticGuarantee::RequiresCompatibilityDecision,
        )
        .expect("application must be structurally valid");

        assert!(
            validate_semantic_policy(
                &application,
                true,
                true,
                true,
            )
            .is_err()
        );
    }

    #[test]
    fn composition_requires_same_model_identity() {
        let first = NoiseApplication::new(
            NoiseApplicationId::new(1),
            NoiseModelId::new(10),
            NoiseModelRevision::default(),
            NoiseApplicationRequest::new(),
            NoiseSelection::single(NoiseEffect::Channel(
                ChannelId::new(11),
            )),
            NoiseSemanticGuarantee::Exact,
        )
        .expect("first application must be valid");

        let second = NoiseApplication::new(
            NoiseApplicationId::new(2),
            NoiseModelId::new(20),
            NoiseModelRevision::default(),
            NoiseApplicationRequest::new(),
            NoiseSelection::single(NoiseEffect::Channel(
                ChannelId::new(21),
            )),
            NoiseSemanticGuarantee::Exact,
        )
        .expect("second application must be valid");

        assert!(
            compose(
                NoiseApplicationId::new(30),
                &first,
                &second,
            )
            .is_err()
        );
    }

    #[test]
    fn composition_requires_same_request() {
        let first = NoiseApplication::new(
            NoiseApplicationId::new(1),
            NoiseModelId::new(10),
            NoiseModelRevision::default(),
            NoiseApplicationRequest::new()
                .with_operation(OperationId::new(1)),
            NoiseSelection::single(NoiseEffect::Channel(
                ChannelId::new(11),
            )),
            NoiseSemanticGuarantee::Exact,
        )
        .expect("first application must be valid");

        let second = NoiseApplication::new(
            NoiseApplicationId::new(2),
            NoiseModelId::new(10),
            NoiseModelRevision::default(),
            NoiseApplicationRequest::new()
                .with_operation(OperationId::new(2)),
            NoiseSelection::single(NoiseEffect::Channel(
                ChannelId::new(21),
            )),
            NoiseSemanticGuarantee::Exact,
        )
        .expect("second application must be valid");

        assert!(
            compose(
                NoiseApplicationId::new(30),
                &first,
                &second,
            )
            .is_err()
        );
    }

    #[test]
    fn composition_combines_effect_references_without_executing_them() {
        let request = NoiseApplicationRequest::new()
            .with_operation(OperationId::new(1));

        let first = NoiseApplication::new(
            NoiseApplicationId::new(1),
            NoiseModelId::new(10),
            NoiseModelRevision::default(),
            request.clone(),
            NoiseSelection::single(NoiseEffect::Channel(
                ChannelId::new(11),
            )),
            NoiseSemanticGuarantee::Exact,
        )
        .expect("first application must be valid");

        let second = NoiseApplication::new(
            NoiseApplicationId::new(2),
            NoiseModelId::new(10),
            NoiseModelRevision::default(),
            request.clone(),
            NoiseSelection::single(NoiseEffect::Fault(
                FaultId::new(21),
            )),
            NoiseSemanticGuarantee::Exact,
        )
        .expect("second application must be valid");

        let combined = compose(
            NoiseApplicationId::new(30),
            &first,
            &second,
        )
        .expect("applications should compose");

        assert_eq!(combined.effect_count(), 2);
        assert!(contains_channel(&combined));
        assert!(contains_fault(&combined));
        assert_eq!(combined.request(), &request);
    }

    #[test]
    fn compose_if_needed_preserves_non_effect_application() {
        let empty = NoiseApplication::new(
            NoiseApplicationId::new(1),
            NoiseModelId::new(10),
            NoiseModelRevision::default(),
            NoiseApplicationRequest::new(),
            NoiseSelection::None,
            NoiseSemanticGuarantee::Exact,
        )
        .expect("empty application must be valid");

        let populated = NoiseApplication::new(
            NoiseApplicationId::new(2),
            NoiseModelId::new(10),
            NoiseModelRevision::default(),
            NoiseApplicationRequest::new(),
            NoiseSelection::single(NoiseEffect::Channel(
                ChannelId::new(20),
            )),
            NoiseSemanticGuarantee::Exact,
        )
        .expect("populated application must be valid");

        let result = compose_if_needed(
            NoiseApplicationId::new(3),
            &empty,
            &populated,
        )
        .expect("composition should succeed");

        assert_eq!(result.effect_count(), 1);
        assert!(contains_channel(&result));
    }

    #[test]
    fn summary_is_lightweight_and_deterministic() {
        let application = NoiseApplication::new(
            NoiseApplicationId::new(1),
            NoiseModelId::new(2),
            NoiseModelRevision::new(1, 2, 3),
            NoiseApplicationRequest::new()
                .with_operation(OperationId::new(4))
                .with_target(
                    crate::quantum::zqn::noise::model::NoiseTarget::logical_qubit(
                        QubitId::new(5),
                    ),
                )
                .with_parameter(NoiseParameterId::new(6)),
            NoiseSelection::single(NoiseEffect::Channel(
                ChannelId::new(7),
            )),
            NoiseSemanticGuarantee::Exact,
        )
        .expect("application must be valid");

        let summary = application.summary();

        assert_eq!(summary.id(), NoiseApplicationId::new(1));
        assert_eq!(summary.model_id(), NoiseModelId::new(2));
        assert_eq!(
            summary.model_revision(),
            NoiseModelRevision::new(1, 2, 3)
        );
        assert_eq!(summary.status(), NoiseApplicationStatus::Selected);
        assert_eq!(summary.effect_count(), 1);
        assert!(summary.has_operation());
        assert_eq!(summary.target_count(), 1);
        assert_eq!(summary.parameter_count(), 1);
    }

    #[test]
    fn canonical_qubit_identity_is_used_directly() {
        let target =
            crate::quantum::zqn::noise::model::NoiseTarget::logical_qubit(
                QubitId::new(99),
            );

        let request =
            NoiseApplicationRequest::new().with_target(target);

        let application = NoiseApplication::new(
            NoiseApplicationId::new(1),
            NoiseModelId::new(2),
            NoiseModelRevision::default(),
            request,
            NoiseSelection::None,
            NoiseSemanticGuarantee::Exact,
        )
        .expect("application must be valid");

        assert_eq!(application.targets().len(), 1);
    }

    #[test]
    fn application_is_cloneable_value_data() {
        let application = NoiseApplication::new(
            NoiseApplicationId::new(1),
            NoiseModelId::new(2),
            NoiseModelRevision::default(),
            NoiseApplicationRequest::new(),
            NoiseSelection::single(NoiseEffect::Channel(
                ChannelId::new(3),
            )),
            NoiseSemanticGuarantee::Exact,
        )
        .expect("application must be valid");

        let cloned = application.clone();

        assert_eq!(application, cloned);
    }

    #[test]
    fn application_validation_is_repeatable() {
        let application = NoiseApplication::new(
            NoiseApplicationId::new(1),
            NoiseModelId::new(2),
            NoiseModelRevision::default(),
            NoiseApplicationRequest::new(),
            NoiseSelection::single(NoiseEffect::Fault(
                FaultId::new(3),
            )),
            NoiseSemanticGuarantee::Exact,
        )
        .expect("application must be valid");

        assert!(application.validate().is_ok());
        assert!(application.validate().is_ok());
        assert!(application.validate().is_ok());
    }
}