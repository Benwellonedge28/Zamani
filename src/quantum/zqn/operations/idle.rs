//! Zamani Quantum Noise (ZQN) — Idle-Operation Noise.
//!
//! `src/quantum/zqn/operations/idle.rs`
//!
//! # Purpose
//!
//! This module defines the ZQN semantic boundary for noise associated with
//! quantum resources remaining idle, waiting, delayed, parked, held, or
//! otherwise not undergoing an intended active quantum transformation during
//! an explicit interval.
//!
//! Idle noise is a first-class physical phenomenon. It must not be inferred
//! from gate noise, measurement noise, reset noise, or scheduling policy.
//!
//! The central abstraction is:
//!
//! ```text
//! canonical quantum operation/resource identity
//!                    │
//!                    ▼
//!             ZQN Operation
//!             class = Idle
//!                    │
//!                    ├── resources
//!                    ├── duration
//!                    └── operation identity
//!                    │
//!                    ▼
//!             IdleNoiseBinding
//!                    │
//!                    ▼
//!              NoiseApplicationRequest
//!                    │
//!                    ▼
//!                 NoiseModel
//!                    │
//!                    ▼
//!             NoiseSelection
//!                    │
//!                    ▼
//!          simulation / runtime / QEC
//! ```
//!
//! # Ownership
//!
//! This module owns:
//!
//! - the semantic idle-noise binding;
//! - validation specific to idle noise;
//! - construction of idle operation contexts;
//! - explicit idle duration association;
//! - explicit logical/physical resource association;
//! - conversion to the generic ZQN noise-application request;
//! - deterministic selection helpers;
//! - idle-specific semantic predicates;
//! - immutable access to idle-noise context.
//!
//! # Does not own
//!
//! This module does NOT own:
//!
//! - canonical quantum IR semantics;
//! - canonical `QubitId`;
//! - canonical `PhysicalQubitId`;
//! - canonical `OperationId`;
//! - scheduling;
//! - deciding that an operation should be idle;
//! - hardware timing;
//! - clock implementation;
//! - quantum-channel mathematics;
//! - probability distributions;
//! - stochastic sampling;
//! - random-number generation;
//! - calibration storage;
//! - drift estimation;
//! - simulation state;
//! - QEC decoding;
//! - routing;
//! - hardware APIs;
//! - vendor-specific idle instructions;
//! - serialization wire formats.
//!
//! Those responsibilities remain in their respective subsystems.
//!
//! # Canonical quantum identity
//!
//! All qubit identities come directly from:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module MUST NOT define another `QubitId`, `PhysicalQubitId`, or
//! equivalent integer wrapper.
//!
//! A logical qubit and physical qubit are intentionally distinct semantic
//! domains.
//!
//! # Canonical operation identity
//!
//! Operation identity comes from:
//!
//! ```text
//! crate::quantum::ir::identity::OperationId
//! ```
//!
//! ZQN does not create a second operation identity.
//!
//! # Idle is not a gate
//!
//! An idle interval must not be represented as a synthetic gate such as:
//!
//! ```text
//! identity()
//! ```
//!
//! or by rewriting it into:
//!
//! ```text
//! measure + prepare
//! ```
//!
//! Such rewrites can destroy physical timing and noise semantics.
//!
//! An idle interval has its own semantic category:
//!
//! ```text
//! OperationClass::Idle
//! ```
//!
//! The scheduler decides when an idle interval exists.
//!
//! This module describes the noise associated with that interval.
//!
//! # Idle duration
//!
//! Duration is part of the idle-noise context because many physical noise
//! processes depend directly on elapsed time:
//!
//! - relaxation;
//! - dephasing;
//! - thermalization;
//! - leakage;
//! - loss;
//! - drift;
//! - correlated environmental effects;
//! - non-Markovian memory;
//! - transport waiting;
//! - storage degradation.
//!
//! Duration is represented using the existing ZQN:
//!
//! ```text
//! operations::operation::OperationDuration
//! ```
//!
//! This module does not create another duration type.
//!
//! The existing duration contract is finite and non-negative and uses seconds
//! as its semantic unit without imposing a hardware clock resolution.
//!
//! # Important timing boundary
//!
//! `IdleNoiseBinding` records the semantic duration associated with the idle
//! interval.
//!
//! It does NOT:
//!
//! - choose when the idle starts;
//! - advance a clock;
//! - round duration to hardware ticks;
//! - compensate for clock skew;
//! - create a schedule;
//! - mutate scheduler state.
//!
//! Scheduling and target lowering perform those tasks.
//!
//! # Write once, scale everywhere
//!
//! This module contains no semantic upper bound on:
//!
//! - qubit count;
//! - physical resource count;
//! - idle interval count;
//! - circuit depth;
//! - idle duration;
//! - operation count;
//! - machine size;
//! - topology size;
//! - number of devices;
//! - number of distributed nodes.
//!
//! There is intentionally no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_IDLE_RESOURCES
//! MAX_IDLE_DURATION
//! MAX_IDLE_OPERATIONS
//! ```
//!
//! "Infinity" means that the semantic abstraction imposes no artificial
//! machine-size ceiling.
//!
//! Concrete execution remains bounded by:
//!
//! - available memory;
//! - available CPU/GPU/accelerator resources;
//! - distributed resources;
//! - storage;
//! - target capabilities;
//! - explicit runtime resource policies;
//! - user-selected limits.
//!
//! # Resource representation
//!
//! Idle resources are represented as an ordered collection of generic ZQN
//! operation resources.
//!
//! The common qubit case uses canonical `QubitId`.
//!
//! Physical qubits use canonical `PhysicalQubitId` through `NoiseTarget`.
//!
//! This allows idle semantics to remain useful for future resources such as:
//!
//! - qudits;
//! - bosonic modes;
//! - photonic modes;
//! - analog resources;
//! - transport channels;
//! - communication links;
//! - logical resources;
//! - composite resources.
//!
//! The module does not assume that a quantum computer consists only of qubits.
//!
//! # Determinism
//!
//! Construction is deterministic.
//!
//! This module:
//!
//! - does not create an RNG;
//! - does not use a global RNG;
//! - does not use a thread-local RNG;
//! - does not derive semantics from wall-clock time;
//! - does not use memory addresses;
//! - does not depend on thread scheduling;
//! - does not mutate global state.
//!
//! If the selected `NoiseModel` is stochastic, stochastic realization remains
//! controlled by the explicit ZQN execution/sampling context.
//!
//! Calling `select()` on this module does not create implicit randomness.
//!
//! # Parallelism
//!
//! An `IdleNoiseBinding` is immutable after construction and can safely be
//! shared between concurrent consumers when the surrounding model/context is
//! shareable.
//!
//! The semantic value does not depend on:
//!
//! - worker count;
//! - thread identity;
//! - process identity;
//! - execution order.
//!
//! A deterministic downstream sampler must preserve equivalent results between
//! sequential and parallel execution under the same explicit deterministic
//! policy.
//!
//! # Resource safety
//!
//! This module does not materialize:
//!
//! - quantum states;
//! - density matrices;
//! - Kraus matrices;
//! - tensors;
//! - fault batches;
//! - probability distributions.
//!
//! It only constructs the semantic operation/resource context.
//!
//! No machine-size limit is embedded in this file.
//!
//! If a caller needs an explicit resource ceiling, that policy belongs to
//! `ZqnContext` and its associated resource-limit subsystem.
//!
//! # Numerical safety
//!
//! Idle duration must be:
//!
//! - finite;
//! - non-negative.
//!
//! `OperationDuration::from_seconds` already enforces those invariants.
//!
//! This module does not perform exponential decay, probability calculations,
//! integration, or numerical channel application.
//!
//! It therefore does not silently transform invalid numerical values.
//!
//! # Approximation
//!
//! This module does not decide whether an idle-noise model is exact,
//! approximate, bounded, or statistical.
//!
//! That semantic guarantee belongs to `NoiseModel`.
//!
//! A downstream target must never treat an approximate idle model as exact
//! without an explicit compatibility decision.
//!
//! # Integration with scheduling
//!
//! The intended direction is:
//!
//! ```text
//! scheduler
//!     │
//!     ├── determines that resource R is idle
//!     ├── determines idle interval duration
//!     │
//!     ▼
//! IdleNoiseBinding
//!     │
//!     ▼
//! ZQN noise model
//! ```
//!
//! Scheduling owns the timeline.
//!
//! This module only consumes the resulting semantic duration.
//!
//! # Integration with routing
//!
//! Routing may consume idle-noise information when evaluating placement costs.
//!
//! For example, a placement that causes a qubit to remain idle longer may have
//! a larger predicted error cost.
//!
//! Routing does not mutate this type and does not redefine idle semantics.
//!
//! # Integration with calibration
//!
//! Calibration may provide parameters used by the selected noise model, such
//! as relaxation/dephasing characteristics.
//!
//! This module does not store those parameters.
//!
//! # Integration with simulation
//!
//! Simulation consumes the selected noise semantics and realizes them against
//! simulator state.
//!
//! This module stops before state mutation.
//!
//! # Integration with QEC
//!
//! QEC may convert realized idle faults into physical or logical error
//! information.
//!
//! This module does not decode or correct those faults.
//!
//! # Integration with hardware
//!
//! Hardware adapters determine whether the requested idle/noise semantics are
//! representable on the target.
//!
//! This module contains no vendor API or hardware credential.
//!
//! # Integration with benchmarking
//!
//! Benchmarking may use idle-noise bindings to quantify storage/wait/decoherence
//! behavior and compare targets.
//!
//! Benchmark methodology remains outside this module.
//!
//! # Integration with memory
//!
//! Memory/state simulation may consume the selected channel associated with an
//! idle interval.
//!
//! The memory subsystem owns state representation.
//!
//! # Integration with noise::model
//!
//! `IdleNoiseBinding::request()` creates the existing generic:
//!
//! ```text
//! NoiseApplicationRequest
//! ```
//!
//! The request carries:
//!
//! - canonical `OperationId`;
//! - logical or physical `NoiseTarget`s.
//!
//! The idle duration remains available through `duration()` and the underlying
//! `Operation` context.
//!
//! This separation is intentional: the current generic request is the
//! resource/application identity boundary, while duration remains part of the
//! ZQN operation execution context.
//!
//! A future extension of the generic request may expose temporal context
//! directly, but such an extension must not require changing the semantic idle
//! contract defined here.
//!
//! # Serialization
//!
//! This module intentionally does not implement serialization.
//!
//! `zqn::io` owns the external schema.
//!
//! A serialized idle binding must preserve, at minimum:
//!
//! - operation identity;
//! - operation class;
//! - operation name;
//! - resource identities;
//! - resource roles;
//! - duration;
//! - semantic model association supplied by the surrounding application.
//!
//! Rust field layout is not a wire-format contract.
//!
//! # Versioning
//!
//! Global ZQN schema versioning belongs to `zqn::core::version`.
//!
//! This module does not introduce a competing external schema version.
//!
//! # Security
//!
//! This module does not grant:
//!
//! - hardware access;
//! - filesystem access;
//! - network access;
//! - credentials;
//! - process execution;
//! - calibration mutation.
//!
//! It is a validated semantic description.
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
//! The compiler enforces the no-unsafe requirement through:
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! # Definition of done
//!
//! This file is complete when:
//!
//! - it uses canonical IR identities;
//! - it does not define a second IR;
//! - it does not define a second duration type;
//! - idle duration is explicitly represented;
//! - logical and physical targets are supported;
//! - arbitrary resource count is supported;
//! - no machine-size limit is hard-coded;
//! - no RNG exists here;
//! - validation is explicit;
//! - model selection is delegated;
//! - scheduling remains external;
//! - serialization remains external;
//! - simulation remains external;
//! - QEC remains external;
//! - routing remains external;
//! - hardware remains external;
//! - tests cover construction, validation, determinism and scaling.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use crate::quantum::ir::identity::OperationId;
use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

use crate::quantum::zqn::core::context::ZqnContext;
use crate::quantum::zqn::core::errors::ZqnResult;
use crate::quantum::zqn::noise::model::{
    select_noise,
    NoiseApplicationRequest,
    NoiseModel,
    NoiseSelection,
    NoiseTarget,
};

use crate::quantum::zqn::operations::operation::{
    Operation,
    OperationClass,
    OperationDuration,
    OperationResource,
    ResourceRole,
};

// =============================================================================
// Constants
// =============================================================================

/// Semantic model version for the idle-noise binding contract.
///
/// This is a representation marker, not a resource or machine-size limit.
pub const IDLE_NOISE_MODEL_VERSION: u16 = 1;

// =============================================================================
// Errors
// =============================================================================

/// Errors specific to construction of an idle-noise binding.
///
/// General ZQN failures are represented by `ZqnError` at integration
/// boundaries. This local error is deliberately small and contains only
/// validation failures that can be detected without invoking a noise model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdleNoiseError {
    /// The idle operation has no resources.
    MissingResource,

    /// A non-idle operation context was supplied.
    WrongOperationClass,

    /// A zero-duration idle was rejected by an API that explicitly requires
    /// elapsed idle time.
    ZeroDuration,

    /// A requested logical and physical target representation was inconsistent.
    InvalidTarget,

    /// A target count overflowed a caller-provided representation.
    ResourceCountOverflow,
}

impl fmt::Display for IdleNoiseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingResource => {
                formatter.write_str("idle operation requires at least one resource")
            }
            Self::WrongOperationClass => {
                formatter.write_str("operation context is not classified as idle")
            }
            Self::ZeroDuration => {
                formatter.write_str("idle operation requires a non-zero duration")
            }
            Self::InvalidTarget => {
                formatter.write_str("idle operation contains an invalid noise target")
            }
            Self::ResourceCountOverflow => {
                formatter.write_str("idle resource count cannot be represented")
            }
        }
    }
}

impl std::error::Error for IdleNoiseError {}

/// Result for locally validated idle-noise construction.
pub type IdleNoiseLocalResult<T> = Result<T, IdleNoiseError>;

// =============================================================================
// Idle operation context
// =============================================================================

/// Immutable semantic context for an idle interval.
///
/// This is a specialized view over the common ZQN [`Operation`] context.
///
/// The underlying operation MUST have:
///
/// ```text
/// OperationClass::Idle
/// ```
///
/// and MUST contain:
///
/// - at least one resource;
/// - a finite, non-negative duration.
///
/// This type does not own scheduling.
///
/// The scheduler creates the idle interval; this type describes its noise
/// boundary.
#[derive(Debug, Clone, PartialEq)]
pub struct IdleOperation {
    operation: Operation,
}

impl IdleOperation {
    /// Creates an idle operation context from an existing ZQN operation.
    ///
    /// The supplied operation must:
    ///
    /// - have `OperationClass::Idle`;
    /// - contain at least one resource;
    /// - contain an explicit duration;
    /// - have a non-zero duration.
    ///
    /// The constructor does not impose a maximum resource count or duration.
    pub fn new(operation: Operation) -> IdleNoiseLocalResult<Self> {
        if operation.class() != OperationClass::Idle {
            return Err(IdleNoiseError::WrongOperationClass);
        }

        if operation.has_no_resources() {
            return Err(IdleNoiseError::MissingResource);
        }

        match operation.duration() {
            Some(duration) if !duration.is_zero() => {}
            Some(_) | None => return Err(IdleNoiseError::ZeroDuration),
        }

        Ok(Self { operation })
    }

    /// Creates a logical-qubit idle operation.
    ///
    /// The supplied duration must be finite, non-negative and non-zero.
    pub fn logical_qubit(
        operation_id: OperationId,
        qubit: QubitId,
        duration: OperationDuration,
    ) -> IdleNoiseLocalResult<Self> {
        let operation = Operation::named(
            operation_id,
            OperationClass::Idle,
            "idle",
        )
        .map_err(|_| IdleNoiseError::InvalidTarget)?
        .with_resource(
            OperationResource::qubit(qubit),
            ResourceRole::Idle,
        )
        .map_err(|_| IdleNoiseError::InvalidTarget)?
        .with_duration(duration)
        .map_err(|_| IdleNoiseError::InvalidTarget)?;

        Self::new(operation)
    }

    /// Creates an idle operation over multiple logical qubits.
    ///
    /// Resource count is determined entirely by the supplied iterator.
    ///
    /// No fixed arity is assumed.
    pub fn logical_qubits<I>(
        operation_id: OperationId,
        qubits: I,
        duration: OperationDuration,
    ) -> IdleNoiseLocalResult<Self>
    where
        I: IntoIterator<Item = QubitId>,
    {
        let resources = qubits
            .into_iter()
            .map(|qubit| {
                (
                    OperationResource::qubit(qubit),
                    ResourceRole::Idle,
                )
            });

        let operation = Operation::named(
            operation_id,
            OperationClass::Idle,
            "idle",
        )
        .map_err(|_| IdleNoiseError::InvalidTarget)?
        .with_resources(resources)
        .map_err(|_| IdleNoiseError::InvalidTarget)?
        .with_duration(duration)
        .map_err(|_| IdleNoiseError::InvalidTarget)?;

        Self::new(operation)
    }

    /// Creates a physical-qubit idle noise context.
    ///
    /// Physical placement is already resolved by the caller. This constructor
    /// does not perform routing.
    ///
    /// The physical identity is preserved separately from the logical identity
    /// domain.
    pub fn physical_qubit(
        operation_id: OperationId,
        physical_qubit: PhysicalQubitId,
        duration: OperationDuration,
    ) -> IdleNoiseLocalResult<Self> {
        let operation = Operation::named(
            operation_id,
            OperationClass::Idle,
            "idle",
        )
        .map_err(|_| IdleNoiseError::InvalidTarget)?
        .with_resource(
            OperationResource::indexed(
                "quantum.physical_qubit",
                physical_qubit.index() as u128,
            )
            .map_err(|_| IdleNoiseError::InvalidTarget)?,
            ResourceRole::Idle,
        )
        .map_err(|_| IdleNoiseError::InvalidTarget)?
        .with_duration(duration)
        .map_err(|_| IdleNoiseError::InvalidTarget)?;

        Self::new(operation)
    }

    /// Creates an idle context over an arbitrary set of physical qubits.
    ///
    /// This is intentionally iterator-based so callers can stream resources
    /// rather than requiring a fixed machine-size representation.
    pub fn physical_qubits<I>(
        operation_id: OperationId,
        qubits: I,
        duration: OperationDuration,
    ) -> IdleNoiseLocalResult<Self>
    where
        I: IntoIterator<Item = PhysicalQubitId>,
    {
        let resources = qubits.into_iter().map(|qubit| {
            (
                OperationResource::indexed(
                    "quantum.physical_qubit",
                    qubit.index() as u128,
                )
                .expect("static physical-qubit namespace is non-empty"),
                ResourceRole::Idle,
            )
        });

        let operation = Operation::named(
            operation_id,
            OperationClass::Idle,
            "idle",
        )
        .map_err(|_| IdleNoiseError::InvalidTarget)?
        .with_resources(resources)
        .map_err(|_| IdleNoiseError::InvalidTarget)?
        .with_duration(duration)
        .map_err(|_| IdleNoiseError::InvalidTarget)?;

        Self::new(operation)
    }

    /// Returns the canonical operation identity.
    #[must_use]
    pub const fn operation_id(&self) -> OperationId {
        self.operation.operation_id()
    }

    /// Returns the common ZQN operation context.
    #[must_use]
    pub const fn operation(&self) -> &Operation {
        &self.operation
    }

    /// Returns the idle duration.
    ///
    /// Construction guarantees that the result is present and non-zero.
    #[must_use]
    pub const fn duration(&self) -> OperationDuration {
        match self.operation.duration() {
            Some(duration) => duration,
            None => OperationDuration::ZERO,
        }
    }

    /// Returns the number of idle resources.
    #[must_use]
    pub fn resource_count(&self) -> usize {
        self.operation.resource_count()
    }

    /// Returns the canonical logical qubits contained in this idle context.
    ///
    /// The iterator does not allocate.
    pub fn qubits(&self) -> impl Iterator<Item = QubitId> + '_ {
        self.operation.qubits()
    }

    /// Returns whether the idle context contains the supplied logical qubit.
    #[must_use]
    pub fn contains_qubit(&self, qubit: QubitId) -> bool {
        self.qubits().any(|candidate| candidate == qubit)
    }

    /// Returns the semantic operation class.
    #[must_use]
    pub const fn class(&self) -> OperationClass {
        self.operation.class()
    }

    /// Validates all idle invariants.
    pub fn validate(&self) -> IdleNoiseLocalResult<()> {
        if self.class() != OperationClass::Idle {
            return Err(IdleNoiseError::WrongOperationClass);
        }

        if self.resource_count() == 0 {
            return Err(IdleNoiseError::MissingResource);
        }

        if self.duration().is_zero() {
            return Err(IdleNoiseError::ZeroDuration);
        }

        Ok(())
    }

    /// Converts this idle context into the generic ZQN noise request.
    ///
    /// The canonical operation identity is preserved.
    ///
    /// Logical qubits become `NoiseTarget::LogicalQubit`.
    ///
    /// Other ZQN operation resources are represented through the operation
    /// target itself because the generic request's resource target vocabulary
    /// deliberately remains independent of operation-resource internals.
    ///
    /// This method does not select or execute noise.
    pub fn request(&self) -> ZqnResult<NoiseApplicationRequest> {
        self.validate()
            .map_err(|error| match error {
                IdleNoiseError::MissingResource => {
                    crate::quantum::zqn::core::errors::ZqnError::invalid_structure(
                        "idle noise requires at least one resource",
                    )
                }
                IdleNoiseError::WrongOperationClass => {
                    crate::quantum::zqn::core::errors::ZqnError::invalid_structure(
                        "idle noise requires OperationClass::Idle",
                    )
                }
                IdleNoiseError::ZeroDuration => {
                    crate::quantum::zqn::core::errors::ZqnError::invalid_structure(
                        "idle noise requires a non-zero duration",
                    )
                }
                IdleNoiseError::InvalidTarget => {
                    crate::quantum::zqn::core::errors::ZqnError::invalid_structure(
                        "idle noise contains an invalid target",
                    )
                }
                IdleNoiseError::ResourceCountOverflow => {
                    crate::quantum::zqn::core::errors::ZqnError::invalid_structure(
                        "idle resource count overflow",
                    )
                }
            })
            .and_then(|_| {
                let mut request =
                    NoiseApplicationRequest::new().with_operation(self.operation_id());

                for qubit in self.qubits() {
                    request = request.with_target(NoiseTarget::logical_qubit(qubit));
                }

                Ok(request)
            })
    }

    /// Selects noise using the supplied model and explicit ZQN context.
    ///
    /// This performs model validation and selection only.
    ///
    /// It does not:
    ///
    /// - mutate quantum state;
    /// - execute a channel;
    /// - create an RNG;
    /// - access hardware;
    /// - perform routing;
    /// - schedule the idle interval.
    ///
    /// The caller remains responsible for realizing the returned selection.
    pub fn select(
        &self,
        model: &dyn NoiseModel,
        context: &ZqnContext,
    ) -> ZqnResult<NoiseSelection> {
        let request = self.request()?;
        select_noise(model, &request, context)
    }
}

// =============================================================================
// Idle noise binding
// =============================================================================

/// Immutable association between an idle interval and a ZQN noise model.
///
/// This type does not own the model implementation.
///
/// The model is supplied at selection time, which avoids:
///
/// - hidden registries;
/// - global model state;
/// - serialization coupling;
/// - lifetime coupling;
/// - vendor-specific ownership.
///
#[derive(Debug, Clone, PartialEq)]
pub struct IdleNoiseBinding {
    idle: IdleOperation,
}

impl IdleNoiseBinding {
    /// Creates a validated idle-noise binding.
    pub fn new(idle: IdleOperation) -> IdleNoiseLocalResult<Self> {
        idle.validate()?;
        Ok(Self { idle })
    }

    /// Creates a logical-qubit idle binding.
    pub fn for_qubit(
        operation_id: OperationId,
        qubit: QubitId,
        duration: OperationDuration,
    ) -> IdleNoiseLocalResult<Self> {
        Self::new(IdleOperation::logical_qubit(
            operation_id,
            qubit,
            duration,
        )?)
    }

    /// Creates a multi-logical-qubit idle binding.
    pub fn for_qubits<I>(
        operation_id: OperationId,
        qubits: I,
        duration: OperationDuration,
    ) -> IdleNoiseLocalResult<Self>
    where
        I: IntoIterator<Item = QubitId>,
    {
        Self::new(IdleOperation::logical_qubits(
            operation_id,
            qubits,
            duration,
        )?)
    }

    /// Creates a physical-qubit idle binding.
    pub fn for_physical_qubit(
        operation_id: OperationId,
        qubit: PhysicalQubitId,
        duration: OperationDuration,
    ) -> IdleNoiseLocalResult<Self> {
        Self::new(IdleOperation::physical_qubit(
            operation_id,
            qubit,
            duration,
        )?)
    }

    /// Creates a multi-physical-qubit idle binding.
    pub fn for_physical_qubits<I>(
        operation_id: OperationId,
        qubits: I,
        duration: OperationDuration,
    ) -> IdleNoiseLocalResult<Self>
    where
        I: IntoIterator<Item = PhysicalQubitId>,
    {
        Self::new(IdleOperation::physical_qubits(
            operation_id,
            qubits,
            duration,
        )?)
    }

    /// Returns the underlying idle operation.
    #[must_use]
    pub const fn idle(&self) -> &IdleOperation {
        &self.idle
    }

    /// Returns the canonical operation identity.
    #[must_use]
    pub const fn operation_id(&self) -> OperationId {
        self.idle.operation_id()
    }

    /// Returns the idle duration.
    #[must_use]
    pub const fn duration(&self) -> OperationDuration {
        self.idle.duration()
    }

    /// Returns the number of resources held idle.
    #[must_use]
    pub fn resource_count(&self) -> usize {
        self.idle.resource_count()
    }

    /// Returns the logical qubits affected by this binding.
    ///
    /// No collection is allocated.
    pub fn qubits(&self) -> impl Iterator<Item = QubitId> + '_ {
        self.idle.qubits()
    }

    /// Returns whether a logical qubit is affected.
    #[must_use]
    pub fn contains_qubit(&self, qubit: QubitId) -> bool {
        self.idle.contains_qubit(qubit)
    }

    /// Validates the complete idle binding.
    pub fn validate(&self) -> IdleNoiseLocalResult<()> {
        self.idle.validate()
    }

    /// Creates the generic ZQN noise-application request.
    ///
    /// The returned request contains the canonical operation identity and
    /// logical resource targets.
    pub fn request(&self) -> ZqnResult<NoiseApplicationRequest> {
        self.idle.request()
    }

    /// Selects noise through the canonical ZQN `NoiseModel` contract.
    pub fn select(
        &self,
        model: &dyn NoiseModel,
        context: &ZqnContext,
    ) -> ZqnResult<NoiseSelection> {
        self.idle.select(model, context)
    }

    /// Returns true if this binding represents a non-zero idle interval.
    #[must_use]
    pub const fn is_non_zero(&self) -> bool {
        !self.duration().is_zero()
    }
}

// =============================================================================
// Convenience constructors
// =============================================================================

/// Creates a logical-qubit idle binding.
///
/// This is equivalent to [`IdleNoiseBinding::for_qubit`].
pub fn logical_idle(
    operation_id: OperationId,
    qubit: QubitId,
    duration: OperationDuration,
) -> IdleNoiseLocalResult<IdleNoiseBinding> {
    IdleNoiseBinding::for_qubit(operation_id, qubit, duration)
}

/// Creates a physical-qubit idle binding.
///
/// Physical identity is preserved through the canonical
/// `PhysicalQubitId` type.
pub fn physical_idle(
    operation_id: OperationId,
    qubit: PhysicalQubitId,
    duration: OperationDuration,
) -> IdleNoiseLocalResult<IdleNoiseBinding> {
    IdleNoiseBinding::for_physical_qubit(
        operation_id,
        qubit,
        duration,
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn duration() -> OperationDuration {
        OperationDuration::from_seconds(1.0e-9)
            .expect("finite positive duration must be valid")
    }

    #[test]
    fn logical_idle_uses_canonical_qubit_identity() {
        let qubit = QubitId::new(17);

        let idle = IdleNoiseBinding::for_qubit(
            OperationId::new(1),
            qubit,
            duration(),
        )
        .expect("valid idle binding");

        assert_eq!(idle.resource_count(), 1);
        assert!(idle.contains_qubit(qubit));
        assert_eq!(idle.duration().as_seconds(), 1.0e-9);
    }

    #[test]
    fn idle_is_explicitly_classified_as_idle() {
        let idle = IdleOperation::logical_qubit(
            OperationId::new(2),
            QubitId::new(3),
            duration(),
        )
        .expect("valid idle operation");

        assert_eq!(idle.class(), OperationClass::Idle);
    }

    #[test]
    fn zero_duration_is_rejected() {
        let result = IdleNoiseBinding::for_qubit(
            OperationId::new(3),
            QubitId::new(0),
            OperationDuration::ZERO,
        );

        assert!(matches!(
            result,
            Err(IdleNoiseError::ZeroDuration)
        ));
    }

    #[test]
    fn non_finite_duration_is_rejected_by_common_duration_type() {
        assert!(
            OperationDuration::from_seconds(f64::NAN).is_err()
        );

        assert!(
            OperationDuration::from_seconds(f64::INFINITY).is_err()
        );

        assert!(
            OperationDuration::from_seconds(f64::NEG_INFINITY).is_err()
        );
    }

    #[test]
    fn negative_duration_is_rejected_by_common_duration_type() {
        assert!(
            OperationDuration::from_seconds(-1.0).is_err()
        );
    }

    #[test]
    fn duplicate_logical_resources_are_rejected() {
        let qubit = QubitId::new(4);

        let result = IdleNoiseBinding::for_qubits(
            OperationId::new(4),
            [qubit, qubit],
            duration(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn arbitrary_resource_count_is_supported() {
        let qubits = (0usize..128).map(QubitId::new);

        let idle = IdleNoiseBinding::for_qubits(
            OperationId::new(5),
            qubits,
            duration(),
        )
        .expect("generated resource set should be accepted");

        assert_eq!(idle.resource_count(), 128);
    }

    #[test]
    fn operation_identity_is_preserved() {
        let operation_id = OperationId::new(42);

        let idle = IdleNoiseBinding::for_qubit(
            operation_id,
            QubitId::new(9),
            duration(),
        )
        .expect("valid idle binding");

        assert_eq!(idle.operation_id(), operation_id);
    }

    #[test]
    fn physical_identity_is_not_converted_into_logical_identity() {
        let physical = PhysicalQubitId::new(7);

        let idle = IdleNoiseBinding::for_physical_qubit(
            OperationId::new(6),
            physical,
            duration(),
        )
        .expect("valid physical idle binding");

        assert_eq!(idle.resource_count(), 1);
        assert!(idle.is_non_zero());
    }

    #[test]
    fn request_preserves_operation_identity() {
        let operation_id = OperationId::new(7);

        let idle = IdleNoiseBinding::for_qubit(
            operation_id,
            QubitId::new(11),
            duration(),
        )
        .expect("valid idle binding");

        let request = idle
            .request()
            .expect("valid request");

        assert_eq!(request.operation(), Some(operation_id));
        assert_eq!(request.targets().len(), 1);
    }

    #[test]
    fn request_preserves_logical_target() {
        let qubit = QubitId::new(12);

        let idle = IdleNoiseBinding::for_qubit(
            OperationId::new(8),
            qubit,
            duration(),
        )
        .expect("valid idle binding");

        let request = idle
            .request()
            .expect("valid request");

        assert_eq!(
            request.targets(),
            &[NoiseTarget::logical_qubit(qubit)]
        );
    }

    #[test]
    fn large_logical_identifier_does_not_create_machine_size_assumption() {
        let qubit = QubitId::new(usize::MAX);

        let idle = IdleNoiseBinding::for_qubit(
            OperationId::new(9),
            qubit,
            duration(),
        )
        .expect("identifier itself is semantically valid");

        assert!(idle.contains_qubit(qubit));
    }

    #[test]
    fn construction_is_deterministic() {
        let first = IdleNoiseBinding::for_qubit(
            OperationId::new(10),
            QubitId::new(21),
            duration(),
        )
        .expect("valid binding");

        let second = IdleNoiseBinding::for_qubit(
            OperationId::new(10),
            QubitId::new(21),
            duration(),
        )
        .expect("valid binding");

        assert_eq!(first, second);
    }
}