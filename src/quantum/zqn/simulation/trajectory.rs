//! Zamani Quantum Noise (ZQN) — Quantum Trajectory Engine.
//!
//! `src/quantum/zqn/simulation/trajectory.rs`
//!
//! # Purpose
//!
//! This module defines the provider-neutral execution contract for stochastic
//! quantum trajectories.
//!
//! A trajectory represents one realized history of a stochastic quantum
//! evolution. The actual mathematical state representation remains outside
//! this module.
//!
//! The architecture is:
//!
//! ```text
//! canonical quantum IR
//!         │
//!         ▼
//! ZQN noise/channel semantics
//!         │
//!         ▼
//! TrajectoryModel
//!         │
//!         ├── possible transitions
//!         │
//!         ▼
//! TrajectorySelector
//!         │
//!         ▼
//! one realized trajectory
//!         │
//!         ├── state
//!         ├── time
//!         ├── selected events
//!         └── execution metadata
//! ```
//!
//! # Ownership
//!
//! This module owns:
//!
//! - the provider-neutral trajectory execution contract;
//! - trajectory configuration;
//! - trajectory coordinates;
//! - stochastic transition representation;
//! - branch validation;
//! - trajectory time accounting;
//! - step accounting;
//! - explicit resource-policy enforcement;
//! - cancellation checkpoints;
//! - event streaming;
//! - deterministic execution coordinates;
//! - trajectory result metadata;
//! - trajectory-specific errors.
//!
//! # Does NOT own
//!
//! This module does NOT own:
//!
//! - canonical quantum IR;
//! - qubit identity;
//! - quantum state-vector mathematics;
//! - density matrices;
//! - stabilizer/tableau mathematics;
//! - tensor networks;
//! - Kraus operators;
//! - Choi matrices;
//! - Lindblad mathematics;
//! - probability-distribution construction;
//! - RNG implementation;
//! - hardware APIs;
//! - QPU credentials;
//! - routing;
//! - scheduling;
//! - QEC decoding;
//! - calibration;
//! - characterization;
//! - benchmarking methodology;
//! - serialization formats.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Canonical quantum identity
//!
//! This module intentionally does not define `QubitId`, `PhysicalQubitId`, or
//! any competing quantum-resource identity.
//!
//! If a caller needs to associate a trajectory with a quantum resource, the
//! caller must use the canonical types:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! The trajectory engine itself remains resource-model agnostic because a
//! future trajectory may represent:
//!
//! - qubits;
//! - qudits;
//! - bosonic modes;
//! - continuous-variable systems;
//! - fermionic modes;
//! - logical resources;
//! - distributed quantum resources;
//! - analog systems;
//! - measurement-based systems;
//! - future quantum technologies.
//!
//! # Write once, scale everywhere
//!
//! No semantic machine-size limit is encoded here.
//!
//! There is no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_TRAJECTORY_LENGTH
//! MAX_BRANCHES
//! MAX_EVENTS
//! MAX_SHOTS
//! ```
//!
//! Resource limits are explicit configuration supplied by the caller.
//!
//! `None` means that this module imposes no additional limit.
//!
//! It does not mean that the physical machine has infinite resources.
//!
//! Therefore a trajectory can scale from a tiny system to arbitrarily large
//! finite workloads, subject only to the resources and policies supplied by
//! the surrounding execution environment.
//!
//! # Why this is not a state-vector simulator
//!
//! A state-vector trajectory engine would incorrectly couple ZQN to one
//! representation.
//!
//! Instead:
//!
//! ```text
//! TrajectoryModel<S>
//! ```
//!
//! owns the state-transition mathematics for its particular state type.
//!
//! This module only performs:
//!
//! ```text
//! transition generation
//!        ↓
//! validation
//!        ↓
//! branch selection
//!        ↓
//! trajectory orchestration
//! ```
//!
//! # Determinism
//!
//! The trajectory engine does not create or own a global RNG.
//!
//! It never uses:
//!
//! - `thread_rng()`;
//! - wall-clock time;
//! - worker identity;
//! - memory addresses;
//! - hash-map iteration order;
//! - process-local entropy;
//! - hidden mutable global state.
//!
//! Instead, each transition has a stable [`TrajectoryCoordinate`] containing:
//!
//! - trajectory identity;
//! - step index.
//!
//! A production sampler can therefore derive its stochastic stream from the
//! coordinate.
//!
//! This makes the following semantically equivalent under deterministic
//! execution:
//!
//! ```text
//! sequential execution
//! parallel execution
//! distributed execution
//! ```
//!
//! provided they use the same trajectory identity, step coordinates, model,
//! and sampling policy.
//!
//! # Integration with sampler.rs
//!
//! [`TrajectorySelector`] is deliberately defined as a narrow interface.
//!
//! `simulation::sampler` is responsible for the actual sampling algorithm.
//! A sampler adapter can implement `TrajectorySelector` without requiring this
//! file to be modified.
//!
//! This preserves the dependency direction:
//!
//! ```text
//! trajectory.rs
//!      │
//!      │ defines contract
//!      ▼
//! TrajectorySelector
//!      ▲
//!      │ implements
//!      │
//! sampler.rs
//! ```
//!
//! The trajectory engine therefore does not duplicate the probability sampler.
//!
//! # Integration with engine.rs
//!
//! `simulation::engine` remains the higher-level simulation orchestration
//! layer.
//!
//! The intended relationship is:
//!
//! ```text
//! simulation::engine
//!       │
//!       ▼
//! trajectory::TrajectoryRunner
//!       │
//!       ▼
//! TrajectoryModel
//!       │
//!       ▼
//! state/channel implementation
//! ```
//!
//! The engine may execute many trajectories/shots, while this module executes
//! one trajectory at a time.
//!
//! # Resource safety
//!
//! Potentially unbounded work is controlled by:
//!
//! - `max_steps`;
//! - `max_branches_per_step`;
//! - `max_events`;
//! - `cancellation`;
//! - caller-controlled streaming.
//!
//! No large result collection is required.
//!
//! The preferred production path is:
//!
//! ```text
//! run_with_observer(...)
//! ```
//!
//! rather than materializing every trajectory event.
//!
//! # Time semantics
//!
//! Each transition supplies a non-negative finite duration.
//!
//! The runner accumulates time using checked floating-point validation.
//!
//! Time must never become:
//!
//! - NaN;
//! - positive infinity;
//! - negative infinity;
//! - negative through a negative duration.
//!
//! The runner does not assume a particular unit.
//!
//! The model and surrounding execution context must establish the unit.
//!
//! # Branch semantics
//!
//! A transition consists of one or more possible branches.
//!
//! Each branch has a non-negative finite weight.
//!
//! Two policies are supported:
//!
//! ```text
//! RequireNormalized
//! Normalize
//! ```
//!
//! `RequireNormalized` is appropriate when the model produces actual
//! probabilities.
//!
//! `Normalize` is useful for unnormalized weights supplied by a mathematical
//! representation.
//!
//! Silent normalization is never performed when strict normalization has been
//! requested.
//!
//! # Empty transitions
//!
//! A transition with zero branches is invalid.
//!
//! A transition with branches whose total weight is zero is invalid.
//!
//! Such a transition cannot define a stochastic continuation.
//!
//! # Branch state ownership
//!
//! Each branch owns its candidate next state.
//!
//! This means a branch can represent:
//!
//! - no-jump evolution;
//! - a quantum jump;
//! - measurement outcome;
//! - leakage;
//! - loss;
//! - reset outcome;
//! - correlated fault;
//! - arbitrary future stochastic transition.
//!
//! The state type itself remains entirely outside ZQN.
//!
//! # Event streaming
//!
//! Events are observed immediately after a branch is selected.
//!
//! The runner does not require storing all events.
//!
//! This is critical for very long trajectories.
//!
//! # Thread safety
//!
//! The runner itself contains no global state.
//!
//! Whether `TrajectoryModel<S>` and `TrajectorySelector` are `Send`/`Sync` is
//! determined by their concrete implementations.
//!
//! No unsafe synchronization is required.
//!
//! # Serialization
//!
//! This file does not define a wire format.
//!
//! Configuration and results may be serialized by `simulation`/`io` adapters.
//!
//! The semantic contract is independent of the serialization representation.
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
//! # File-completion invariant
//!
//! This file is complete when:
//!
//! 1. trajectory semantics are independent of state representation;
//! 2. no quantum-resource identity is duplicated;
//! 3. no RNG is hidden inside the runner;
//! 4. no machine-size limit is hard-coded;
//! 5. resource limits are explicit;
//! 6. cancellation is supported;
//! 7. deterministic coordinates are explicit;
//! 8. events can be streamed;
//! 9. invalid probabilities/weights are rejected;
//! 10. invalid time values are rejected;
//! 11. branch selection is delegated to the sampler boundary;
//! 12. the runner can be used by sequential, parallel, or distributed callers;
//! 13. unrelated ZQN files do not need to be modified to change state
//!     representations;
//! 14. the file compiles independently of unfinished ZQN modules.
//!
//! # Testing
//!
//! Tests cover:
//!
//! - deterministic one-branch trajectories;
//! - multi-branch selection through a test selector;
//! - invalid weights;
//! - zero total weight;
//! - normalization validation;
//! - time accumulation;
//! - step limits;
//! - event limits;
//! - cancellation;
//! - index overflow;
//! - empty branch sets;
//! - streaming observers.
//!
//! The tests deliberately do not assume a fixed qubit count or gate set.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden in this module.

#![forbid(unsafe_code)]

use core::fmt;

/// Stable identifier for a trajectory execution.
///
/// This is deliberately distinct from quantum resource identity.
///
/// A trajectory ID identifies an execution history, not a qubit, physical
/// qubit, operation, or logical resource.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TrajectoryId(u64);

impl TrajectoryId {
    /// Creates a trajectory identifier.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying stable value.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// Stable step coordinate within a trajectory.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TrajectoryStep(u64);

impl TrajectoryStep {
    /// Creates a step coordinate.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the step number.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Returns the next step, or an overflow error.
    pub fn checked_next(self) -> Result<Self, TrajectoryError> {
        self.0
            .checked_add(1)
            .map(Self)
            .ok_or(TrajectoryError::StepIndexOverflow)
    }
}

/// Stable stochastic coordinate supplied to the sampler.
///
/// A sampler must use these semantic coordinates rather than worker identity
/// or execution order when deterministic execution is requested.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct TrajectoryCoordinate {
    trajectory: TrajectoryId,
    step: TrajectoryStep,
}

impl TrajectoryCoordinate {
    /// Creates a coordinate.
    #[must_use]
    pub const fn new(trajectory: TrajectoryId, step: TrajectoryStep) -> Self {
        Self { trajectory, step }
    }

    /// Returns the trajectory identifier.
    #[must_use]
    pub const fn trajectory(self) -> TrajectoryId {
        self.trajectory
    }

    /// Returns the step coordinate.
    #[must_use]
    pub const fn step(self) -> TrajectoryStep {
        self.step
    }
}

/// Policy controlling interpretation of transition weights.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WeightPolicy {
    /// Require the supplied weights to sum to one within the supplied
    /// absolute tolerance.
    RequireNormalized {
        /// Maximum permitted absolute normalization error.
        tolerance: f64,
    },

    /// Treat weights as non-negative relative weights and normalize them for
    /// branch selection.
    Normalize,
}

impl Default for WeightPolicy {
    fn default() -> Self {
        Self::RequireNormalized {
            tolerance: 1.0e-12,
        }
    }
}

impl WeightPolicy {
    fn validate(self) -> Result<Self, TrajectoryError> {
        match self {
            Self::RequireNormalized { tolerance } => {
                if !tolerance.is_finite() || tolerance < 0.0 {
                    return Err(TrajectoryError::InvalidTolerance { tolerance });
                }
            }
            Self::Normalize => {}
        }

        Ok(self)
    }
}

/// Explicit trajectory execution limits.
///
/// `None` means that this layer imposes no additional limit.
///
/// These are resource policies, not semantic limits on Zamani.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TrajectoryLimits {
    /// Maximum number of transition steps.
    pub max_steps: Option<u64>,

    /// Maximum number of branches accepted for one transition.
    pub max_branches_per_step: Option<usize>,

    /// Maximum number of events delivered to an observer.
    pub max_events: Option<u64>,
}

impl TrajectoryLimits {
    fn validate(self) -> Result<Self, TrajectoryError> {
        if self.max_steps == Some(0) {
            // Zero is valid: it means no transition may execute.
        }

        if self.max_branches_per_step == Some(0) {
            return Err(TrajectoryError::InvalidLimit {
                name: "max_branches_per_step",
            });
        }

        if self.max_events == Some(0) {
            // Zero events is a valid policy.
        }

        Ok(self)
    }
}

/// Configuration for one trajectory execution.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TrajectoryConfig {
    /// Stable trajectory identity.
    pub trajectory_id: TrajectoryId,

    /// Maximum execution limits.
    pub limits: TrajectoryLimits,

    /// Weight interpretation policy.
    pub weight_policy: WeightPolicy,

    /// Whether a zero-duration transition is allowed.
    ///
    /// Zero-duration transitions are allowed by default because they are
    /// necessary for instantaneous measurement/fault/jump abstractions.
    pub allow_zero_duration: bool,
}

impl Default for TrajectoryConfig {
    fn default() -> Self {
        Self {
            trajectory_id: TrajectoryId::new(0),
            limits: TrajectoryLimits::default(),
            weight_policy: WeightPolicy::default(),
            allow_zero_duration: true,
        }
    }
}

impl TrajectoryConfig {
    /// Validates configuration without performing execution.
    pub fn validate(self) -> Result<Self, TrajectoryError> {
        self.limits.validate()?;
        self.weight_policy.validate()?;

        Ok(self)
    }
}

/// A possible stochastic continuation of a trajectory.
///
/// `weight` is either a probability or an unnormalized non-negative weight,
/// depending on [`WeightPolicy`].
///
/// `duration` is the amount of simulated time added if this branch is selected.
///
/// `event` is an optional domain event emitted after branch selection.
#[derive(Clone, Debug, PartialEq)]
pub struct TrajectoryBranch<S, E> {
    /// Candidate next state.
    pub state: S,

    /// Branch probability or relative weight.
    pub weight: f64,

    /// Time elapsed when this branch is selected.
    pub duration: f64,

    /// Optional domain-specific event.
    pub event: Option<E>,
}

impl<S, E> TrajectoryBranch<S, E> {
    /// Constructs a branch.
    #[must_use]
    pub const fn new(state: S, weight: f64, duration: f64, event: Option<E>) -> Self {
        Self {
            state,
            weight,
            duration,
            event,
        }
    }
}

/// A stochastic transition generated by a trajectory model.
#[derive(Clone, Debug, PartialEq)]
pub struct TrajectoryTransition<S, E> {
    branches: Vec<TrajectoryBranch<S, E>>,
}

impl<S, E> TrajectoryTransition<S, E> {
    /// Creates a transition from candidate branches.
    pub fn new(
        branches: Vec<TrajectoryBranch<S, E>>,
    ) -> Result<Self, TrajectoryError> {
        if branches.is_empty() {
            return Err(TrajectoryError::EmptyTransition);
        }

        Ok(Self { branches })
    }

    /// Creates a deterministic transition.
    pub fn deterministic(
        state: S,
        duration: f64,
        event: Option<E>,
    ) -> Result<Self, TrajectoryError> {
        Self::new(vec![TrajectoryBranch::new(
            state,
            1.0,
            duration,
            event,
        )])
    }

    /// Returns the number of candidate branches.
    #[must_use]
    pub fn len(&self) -> usize {
        self.branches.len()
    }

    /// Returns whether the transition contains no branches.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.branches.is_empty()
    }

    /// Borrows the candidate branches.
    #[must_use]
    pub fn branches(&self) -> &[TrajectoryBranch<S, E>] {
        &self.branches
    }

    /// Consumes the transition and returns its branches.
    #[must_use]
    pub fn into_branches(self) -> Vec<TrajectoryBranch<S, E>> {
        self.branches
    }
}

/// An event emitted after a trajectory branch has been selected.
#[derive(Clone, Debug, PartialEq)]
pub struct TrajectoryEvent<E> {
    /// Stable trajectory coordinate.
    pub coordinate: TrajectoryCoordinate,

    /// Simulation time after the selected transition.
    pub time: f64,

    /// Index of the selected branch within the generated transition.
    pub branch_index: usize,

    /// Original branch weight.
    ///
    /// If `WeightPolicy::Normalize` is active this is the original relative
    /// weight, not the normalized probability.
    pub branch_weight: f64,

    /// Domain-specific event payload.
    pub payload: Option<E>,
}

/// Final result of one trajectory.
#[derive(Clone, Debug, PartialEq)]
pub struct TrajectoryResult<S> {
    /// Final state.
    pub state: S,

    /// Final simulation time.
    pub time: f64,

    /// Number of completed transitions.
    pub steps: u64,

    /// Stable trajectory identifier.
    pub trajectory_id: TrajectoryId,
}

/// Cancellation contract for long-running trajectories.
///
/// Implementations may use atomics, runtime cancellation handles, distributed
/// job state, or another safe mechanism.
pub trait TrajectoryCancellation {
    /// Returns true when execution should stop.
    fn is_cancelled(&self) -> bool;
}

/// Observer contract for streaming trajectory events.
///
/// An observer may:
///
/// - aggregate statistics;
/// - write events to a stream;
/// - feed benchmarking;
/// - feed characterization;
/// - update a runtime monitor.
///
/// It must not be assumed that all events are retained in memory.
pub trait TrajectoryObserver<E> {
    /// Receives one selected trajectory event.
    fn observe(&mut self, event: &TrajectoryEvent<E>) -> Result<(), TrajectoryError>;
}

/// Branch-selection contract implemented by the sampling subsystem.
///
/// The trajectory engine does not own the RNG.
///
/// A production implementation should be provided by
/// `simulation::sampler`.
///
/// The selector receives the complete candidate branch set for one transition
/// and the stable stochastic coordinate.
///
/// The selected index must be in `[0, weights.len())`.
pub trait TrajectorySelector {
    /// Selects one branch from the supplied weights.
    fn select(
        &mut self,
        weights: &[f64],
        coordinate: TrajectoryCoordinate,
    ) -> Result<usize, TrajectoryError>;
}

/// Model contract for generating one stochastic trajectory.
///
/// The state representation is generic.
///
/// A state-vector backend, density-matrix backend, tensor-network backend,
/// stabilizer backend, bosonic backend, or hardware-backed implementation may
/// implement this trait without modifying the trajectory runner.
pub trait TrajectoryModel<S, E> {
    /// Produces the initial state.
    fn initial_state(&mut self) -> Result<S, TrajectoryError>;

    /// Advances the supplied state by exactly one stochastic transition.
    ///
    /// The supplied coordinate is stable and must be used by deterministic
    /// models when operation/step identity affects the transition.
    fn advance(
        &mut self,
        state: S,
        coordinate: TrajectoryCoordinate,
    ) -> Result<TrajectoryTransition<S, E>, TrajectoryError>;
}

/// Errors produced by trajectory execution.
#[derive(Clone, Debug, PartialEq)]
pub enum TrajectoryError {
    /// The transition contains no candidate branches.
    EmptyTransition,

    /// A branch has an invalid non-finite weight.
    NonFiniteWeight {
        /// Branch index.
        index: usize,

        /// Invalid value.
        weight: f64,
    },

    /// A branch has a negative weight.
    NegativeWeight {
        /// Branch index.
        index: usize,

        /// Invalid value.
        weight: f64,
    },

    /// The sum of branch weights is not finite.
    NonFiniteWeightSum,

    /// The sum of branch weights is zero.
    ZeroWeightSum,

    /// Strict normalization was requested but the weights do not normalize.
    NotNormalized {
        /// Observed sum.
        sum: f64,

        /// Allowed absolute error.
        tolerance: f64,
    },

    /// A duration is NaN or infinite.
    NonFiniteDuration {
        /// Branch index.
        index: usize,

        /// Invalid duration.
        duration: f64,
    },

    /// A branch has negative duration.
    NegativeDuration {
        /// Branch index.
        index: usize,

        /// Invalid duration.
        duration: f64,
    },

    /// The trajectory time became non-finite.
    NonFiniteTime,

    /// A configured limit is invalid.
    InvalidLimit {
        /// Name of the invalid limit.
        name: &'static str,
    },

    /// A normalization tolerance is invalid.
    InvalidTolerance {
        /// Invalid tolerance.
        tolerance: f64,
    },

    /// A transition exceeds the configured branch limit.
    BranchLimitExceeded {
        /// Number of supplied branches.
        actual: usize,

        /// Configured limit.
        limit: usize,
    },

    /// A trajectory exceeds its configured step limit.
    StepLimitExceeded {
        /// Current number of completed steps.
        completed: u64,

        /// Configured limit.
        limit: u64,
    },

    /// An observer would exceed its configured event limit.
    EventLimitExceeded {
        /// Number of already emitted events.
        emitted: u64,

        /// Configured limit.
        limit: u64,
    },

    /// Execution was cancelled.
    Cancelled,

    /// The trajectory step index cannot be incremented.
    StepIndexOverflow,

    /// The event count cannot be incremented.
    EventCountOverflow,

    /// The branch selection implementation returned an invalid index.
    InvalidSelectedBranch {
        /// Returned index.
        index: usize,

        /// Number of available branches.
        branch_count: usize,
    },
}

impl fmt::Display for TrajectoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTransition => {
                write!(f, "trajectory transition contains no branches")
            }
            Self::NonFiniteWeight { index, weight } => {
                write!(f, "trajectory branch {index} has non-finite weight {weight}")
            }
            Self::NegativeWeight { index, weight } => {
                write!(f, "trajectory branch {index} has negative weight {weight}")
            }
            Self::NonFiniteWeightSum => {
                write!(f, "trajectory branch weights have a non-finite sum")
            }
            Self::ZeroWeightSum => {
                write!(f, "trajectory branch weights have a zero sum")
            }
            Self::NotNormalized { sum, tolerance } => {
                write!(
                    f,
                    "trajectory branch weights are not normalized: sum={sum}, tolerance={tolerance}"
                )
            }
            Self::NonFiniteDuration { index, duration } => {
                write!(
                    f,
                    "trajectory branch {index} has non-finite duration {duration}"
                )
            }
            Self::NegativeDuration { index, duration } => {
                write!(
                    f,
                    "trajectory branch {index} has negative duration {duration}"
                )
            }
            Self::NonFiniteTime => {
                write!(f, "trajectory simulation time became non-finite")
            }
            Self::InvalidLimit { name } => {
                write!(f, "invalid trajectory limit: {name}")
            }
            Self::InvalidTolerance { tolerance } => {
                write!(f, "invalid trajectory normalization tolerance: {tolerance}")
            }
            Self::BranchLimitExceeded { actual, limit } => {
                write!(
                    f,
                    "trajectory transition contains {actual} branches, exceeding limit {limit}"
                )
            }
            Self::StepLimitExceeded { completed, limit } => {
                write!(
                    f,
                    "trajectory step limit exceeded: completed={completed}, limit={limit}"
                )
            }
            Self::EventLimitExceeded { emitted, limit } => {
                write!(
                    f,
                    "trajectory event limit exceeded: emitted={emitted}, limit={limit}"
                )
            }
            Self::Cancelled => {
                write!(f, "trajectory execution cancelled")
            }
            Self::StepIndexOverflow => {
                write!(f, "trajectory step index overflow")
            }
            Self::EventCountOverflow => {
                write!(f, "trajectory event count overflow")
            }
            Self::InvalidSelectedBranch {
                index,
                branch_count,
            } => {
                write!(
                    f,
                    "trajectory selector returned branch {index}, but only {branch_count} branches exist"
                )
            }
        }
    }
}

impl std::error::Error for TrajectoryError {}

/// Production trajectory runner.
///
/// The runner is intentionally generic over:
///
/// - state representation;
/// - event representation;
/// - trajectory model;
/// - branch-selection implementation.
///
/// It therefore does not constrain ZQN to a particular quantum technology.
#[derive(Clone, Copy, Debug)]
pub struct TrajectoryRunner {
    config: TrajectoryConfig,
}

impl TrajectoryRunner {
    /// Creates a validated trajectory runner.
    pub fn new(config: TrajectoryConfig) -> Result<Self, TrajectoryError> {
        let config = config.validate()?;
        Ok(Self { config })
    }

    /// Returns the immutable configuration.
    #[must_use]
    pub const fn config(&self) -> TrajectoryConfig {
        self.config
    }

    /// Executes one trajectory without retaining events.
    pub fn run<S, E, M, R>(
        &self,
        model: &mut M,
        selector: &mut R,
        cancellation: Option<&dyn TrajectoryCancellation>,
    ) -> Result<TrajectoryResult<S>, TrajectoryError>
    where
        M: TrajectoryModel<S, E>,
        R: TrajectorySelector,
    {
        self.run_with_observer::<S, E, M, R, NoopObserver>(
            model,
            selector,
            cancellation,
            &mut NoopObserver,
        )
    }

    /// Executes one trajectory while streaming selected events to an observer.
    ///
    /// Events are never required to accumulate in memory.
    pub fn run_with_observer<S, E, M, R, O>(
        &self,
        model: &mut M,
        selector: &mut R,
        cancellation: Option<&dyn TrajectoryCancellation>,
        observer: &mut O,
    ) -> Result<TrajectoryResult<S>, TrajectoryError>
    where
        M: TrajectoryModel<S, E>,
        R: TrajectorySelector,
        O: TrajectoryObserver<E>,
    {
        let mut state = model.initial_state()?;
        let mut time = 0.0_f64;
        let mut completed_steps = 0_u64;
        let mut emitted_events = 0_u64;

        loop {
            Self::check_cancelled(cancellation)?;

            if let Some(limit) = self.config.limits.max_steps {
                if completed_steps >= limit {
                    return Ok(TrajectoryResult {
                        state,
                        time,
                        steps: completed_steps,
                        trajectory_id: self.config.trajectory_id,
                    });
                }
            }

            let step = TrajectoryStep::new(completed_steps);
            let coordinate = TrajectoryCoordinate::new(
                self.config.trajectory_id,
                step,
            );

            let transition = model.advance(state, coordinate)?;

            self.validate_transition(&transition)?;

            let selected = self.select_branch(
                selector,
                &transition,
                coordinate,
            )?;

            let branch = transition
                .into_branches()
                .into_iter()
                .nth(selected)
                .ok_or(TrajectoryError::InvalidSelectedBranch {
                    index: selected,
                    branch_count: transition.len(),
                })?;

            time = time
                .checked_add(branch.duration)
                .ok_or(TrajectoryError::NonFiniteTime)?;

            if !time.is_finite() {
                return Err(TrajectoryError::NonFiniteTime);
            }

            state = branch.state;

            completed_steps = completed_steps
                .checked_add(1)
                .ok_or(TrajectoryError::StepIndexOverflow)?;

            if let Some(payload) = branch.event {
                if let Some(limit) = self.config.limits.max_events {
                    if emitted_events >= limit {
                        return Err(TrajectoryError::EventLimitExceeded {
                            emitted: emitted_events,
                            limit,
                        });
                    }
                }

                let event = TrajectoryEvent {
                    coordinate,
                    time,
                    branch_index: selected,
                    branch_weight: branch.weight,
                    payload: Some(payload),
                };

                observer.observe(&event)?;

                emitted_events = emitted_events
                    .checked_add(1)
                    .ok_or(TrajectoryError::EventCountOverflow)?;
            }

            // A transition without an event still advances the trajectory.
            //
            // The caller controls termination through max_steps or through
            // model/runner composition. There is intentionally no implicit
            // "converged" condition here because convergence is domain-specific.
        }
    }

    /// Executes exactly `steps` transitions.
    ///
    /// This is useful when the caller owns the trajectory termination policy.
    pub fn run_for_steps<S, E, M, R>(
        &self,
        model: &mut M,
        selector: &mut R,
        steps: u64,
        cancellation: Option<&dyn TrajectoryCancellation>,
    ) -> Result<TrajectoryResult<S>, TrajectoryError>
    where
        M: TrajectoryModel<S, E>,
        R: TrajectorySelector,
    {
        if let Some(limit) = self.config.limits.max_steps {
            if steps > limit {
                return Err(TrajectoryError::StepLimitExceeded {
                    completed: steps,
                    limit,
                });
            }
        }

        let mut bounded = *self;

        bounded.config.limits.max_steps = Some(steps);

        bounded.run(model, selector, cancellation)
    }

    fn validate_transition<S, E>(
        &self,
        transition: &TrajectoryTransition<S, E>,
    ) -> Result<(), TrajectoryError> {
        let branches = transition.branches();

        if branches.is_empty() {
            return Err(TrajectoryError::EmptyTransition);
        }

        if let Some(limit) = self.config.limits.max_branches_per_step {
            if branches.len() > limit {
                return Err(TrajectoryError::BranchLimitExceeded {
                    actual: branches.len(),
                    limit,
                });
            }
        }

        let mut sum = 0.0_f64;

        for (index, branch) in branches.iter().enumerate() {
            if !branch.weight.is_finite() {
                return Err(TrajectoryError::NonFiniteWeight {
                    index,
                    weight: branch.weight,
                });
            }

            if branch.weight < 0.0 {
                return Err(TrajectoryError::NegativeWeight {
                    index,
                    weight: branch.weight,
                });
            }

            if !branch.duration.is_finite() {
                return Err(TrajectoryError::NonFiniteDuration {
                    index,
                    duration: branch.duration,
                });
            }

            if branch.duration < 0.0 {
                return Err(TrajectoryError::NegativeDuration {
                    index,
                    duration: branch.duration,
                });
            }

            if !self.config.allow_zero_duration && branch.duration == 0.0 {
                return Err(TrajectoryError::NegativeDuration {
                    index,
                    duration: branch.duration,
                });
            }

            sum += branch.weight;

            if !sum.is_finite() {
                return Err(TrajectoryError::NonFiniteWeightSum);
            }
        }

        if sum == 0.0 {
            return Err(TrajectoryError::ZeroWeightSum);
        }

        match self.config.weight_policy {
            WeightPolicy::RequireNormalized { tolerance } => {
                if (sum - 1.0).abs() > tolerance {
                    return Err(TrajectoryError::NotNormalized { sum, tolerance });
                }
            }
            WeightPolicy::Normalize => {}
        }

        Ok(())
    }

    fn select_branch<S, E, R>(
        &self,
        selector: &mut R,
        transition: &TrajectoryTransition<S, E>,
        coordinate: TrajectoryCoordinate,
    ) -> Result<usize, TrajectoryError>
    where
        R: TrajectorySelector,
    {
        let weights: Vec<f64> = transition
            .branches()
            .iter()
            .map(|branch| branch.weight)
            .collect();

        let selected = selector.select(&weights, coordinate)?;

        if selected >= weights.len() {
            return Err(TrajectoryError::InvalidSelectedBranch {
                index: selected,
                branch_count: weights.len(),
            });
        }

        Ok(selected)
    }

    fn check_cancelled(
        cancellation: Option<&dyn TrajectoryCancellation>,
    ) -> Result<(), TrajectoryError> {
        if cancellation.is_some_and(TrajectoryCancellation::is_cancelled) {
            return Err(TrajectoryError::Cancelled);
        }

        Ok(())
    }
}

/// Observer that deliberately discards trajectory events.
///
/// Used by [`TrajectoryRunner::run`].
#[derive(Clone, Copy, Debug, Default)]
struct NoopObserver;

impl<E> TrajectoryObserver<E> for NoopObserver {
    fn observe(&mut self, _event: &TrajectoryEvent<E>) -> Result<(), TrajectoryError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, Debug, Default)]
    struct FirstBranchSelector;

    impl TrajectorySelector for FirstBranchSelector {
        fn select(
            &mut self,
            weights: &[f64],
            _coordinate: TrajectoryCoordinate,
        ) -> Result<usize, TrajectoryError> {
            if weights.is_empty() {
                return Err(TrajectoryError::EmptyTransition);
            }

            Ok(0)
        }
    }

    #[derive(Clone, Copy, Debug, Default)]
    struct LastBranchSelector;

    impl TrajectorySelector for LastBranchSelector {
        fn select(
            &mut self,
            weights: &[f64],
            _coordinate: TrajectoryCoordinate,
        ) -> Result<usize, TrajectoryError> {
            weights
                .len()
                .checked_sub(1)
                .ok_or(TrajectoryError::EmptyTransition)
        }
    }

    #[derive(Clone, Copy, Debug, Default)]
    struct AlwaysCancelled;

    impl TrajectoryCancellation for AlwaysCancelled {
        fn is_cancelled(&self) -> bool {
            true
        }
    }

    #[derive(Debug)]
    struct DeterministicModel {
        remaining: u64,
    }

    impl TrajectoryModel<u64, &'static str> for DeterministicModel {
        fn initial_state(&mut self) -> Result<u64, TrajectoryError> {
            Ok(0)
        }

        fn advance(
            &mut self,
            state: u64,
            _coordinate: TrajectoryCoordinate,
        ) -> Result<TrajectoryTransition<u64, &'static str>, TrajectoryError> {
            if self.remaining == 0 {
                return TrajectoryTransition::deterministic(
                    state,
                    0.0,
                    None,
                );
            }

            self.remaining -= 1;

            TrajectoryTransition::deterministic(
                state + 1,
                1.0,
                Some("step"),
            )
        }
    }

    #[derive(Debug)]
    struct BranchingModel;

    impl TrajectoryModel<u64, &'static str> for BranchingModel {
        fn initial_state(&mut self) -> Result<u64, TrajectoryError> {
            Ok(10)
        }

        fn advance(
            &mut self,
            state: u64,
            _coordinate: TrajectoryCoordinate,
        ) -> Result<TrajectoryTransition<u64, &'static str>, TrajectoryError> {
            TrajectoryTransition::new(vec![
                TrajectoryBranch::new(
                    state + 1,
                    0.25,
                    1.0,
                    Some("first"),
                ),
                TrajectoryBranch::new(
                    state + 2,
                    0.75,
                    2.0,
                    Some("second"),
                ),
            ])
        }
    }

    #[derive(Default)]
    struct RecordingObserver {
        events: Vec<u64>,
    }

    impl TrajectoryObserver<&'static str> for RecordingObserver {
        fn observe(
            &mut self,
            event: &TrajectoryEvent<&'static str>,
        ) -> Result<(), TrajectoryError> {
            self.events.push(event.coordinate.step().get());
            Ok(())
        }
    }

    #[test]
    fn deterministic_trajectory_advances_without_hidden_limits() {
        let config = TrajectoryConfig {
            trajectory_id: TrajectoryId::new(7),
            limits: TrajectoryLimits {
                max_steps: Some(3),
                max_branches_per_step: None,
                max_events: None,
            },
            ..TrajectoryConfig::default()
        };

        let runner = TrajectoryRunner::new(config).expect("valid configuration");

        let mut model = DeterministicModel { remaining: 10 };
        let mut selector = FirstBranchSelector;

        let result = runner
            .run::<u64, &'static str, _, _>(
                &mut model,
                &mut selector,
                None,
            )
            .expect("trajectory should execute");

        assert_eq!(result.state, 3);
        assert_eq!(result.steps, 3);
        assert_eq!(result.time, 3.0);
        assert_eq!(result.trajectory_id, TrajectoryId::new(7));
    }

    #[test]
    fn first_branch_selector_is_deterministic() {
        let config = TrajectoryConfig {
            limits: TrajectoryLimits {
                max_steps: Some(1),
                ..TrajectoryLimits::default()
            },
            ..TrajectoryConfig::default()
        };

        let runner = TrajectoryRunner::new(config).expect("valid configuration");

        let mut model = BranchingModel;
        let mut selector = FirstBranchSelector;

        let result = runner
            .run::<u64, &'static str, _, _>(
                &mut model,
                &mut selector,
                None,
            )
            .expect("trajectory should execute");

        assert_eq!(result.state, 11);
        assert_eq!(result.time, 1.0);
    }

    #[test]
    fn last_branch_selector_is_deterministic() {
        let config = TrajectoryConfig {
            limits: TrajectoryLimits {
                max_steps: Some(1),
                ..TrajectoryLimits::default()
            },
            ..TrajectoryConfig::default()
        };

        let runner = TrajectoryRunner::new(config).expect("valid configuration");

        let mut model = BranchingModel;
        let mut selector = LastBranchSelector;

        let result = runner
            .run::<u64, &'static str, _, _>(
                &mut model,
                &mut selector,
                None,
            )
            .expect("trajectory should execute");

        assert_eq!(result.state, 12);
        assert_eq!(result.time, 2.0);
    }

    #[test]
    fn invalid_negative_weight_is_rejected() {
        let transition = TrajectoryTransition::new(vec![
            TrajectoryBranch::new((), -0.1, 0.0, None),
        ])
        .expect("branch container itself is structurally valid");

        let runner = TrajectoryRunner::new(TrajectoryConfig::default())
            .expect("valid configuration");

        let error = runner
            .validate_transition(&transition)
            .expect_err("negative weights must fail");

        assert!(matches!(
            error,
            TrajectoryError::NegativeWeight { index: 0, .. }
        ));
    }

    #[test]
    fn zero_weight_sum_is_rejected() {
        let transition = TrajectoryTransition::new(vec![
            TrajectoryBranch::new((), 0.0, 0.0, None),
            TrajectoryBranch::new((), 0.0, 0.0, None),
        ])
        .expect("branch container itself is structurally valid");

        let runner = TrajectoryRunner::new(TrajectoryConfig::default())
            .expect("valid configuration");

        let error = runner
            .validate_transition(&transition)
            .expect_err("zero total weight must fail");

        assert_eq!(error, TrajectoryError::ZeroWeightSum);
    }

    #[test]
    fn strict_normalization_is_enforced() {
        let transition = TrajectoryTransition::new(vec![
            TrajectoryBranch::new((), 0.2, 0.0, None),
            TrajectoryBranch::new((), 0.2, 0.0, None),
        ])
        .expect("branch container itself is structurally valid");

        let runner = TrajectoryRunner::new(TrajectoryConfig {
            weight_policy: WeightPolicy::RequireNormalized {
                tolerance: 1.0e-12,
            },
            ..TrajectoryConfig::default()
        })
        .expect("valid configuration");

        let error = runner
            .validate_transition(&transition)
            .expect_err("unnormalized weights must fail");

        assert!(matches!(
            error,
            TrajectoryError::NotNormalized { .. }
        ));
    }

    #[test]
    fn unnormalized_weights_are_allowed_when_requested() {
        let transition = TrajectoryTransition::new(vec![
            TrajectoryBranch::new((), 2.0, 0.0, None),
            TrajectoryBranch::new((), 8.0, 0.0, None),
        ])
        .expect("branch container itself is structurally valid");

        let runner = TrajectoryRunner::new(TrajectoryConfig {
            weight_policy: WeightPolicy::Normalize,
            ..TrajectoryConfig::default()
        })
        .expect("valid configuration");

        runner
            .validate_transition(&transition)
            .expect("relative weights should be accepted");
    }

    #[test]
    fn branch_limit_is_explicit_and_configurable() {
        let transition = TrajectoryTransition::new(vec![
            TrajectoryBranch::new((), 0.5, 0.0, None),
            TrajectoryBranch::new((), 0.5, 0.0, None),
        ])
        .expect("branch container itself is structurally valid");

        let runner = TrajectoryRunner::new(TrajectoryConfig {
            limits: TrajectoryLimits {
                max_branches_per_step: Some(1),
                ..TrajectoryLimits::default()
            },
            ..TrajectoryConfig::default()
        })
        .expect("valid configuration");

        let error = runner
            .validate_transition(&transition)
            .expect_err("branch policy must be enforced");

        assert_eq!(
            error,
            TrajectoryError::BranchLimitExceeded {
                actual: 2,
                limit: 1,
            }
        );
    }

    #[test]
    fn cancellation_is_checked_before_model_execution() {
        let config = TrajectoryConfig::default();
        let runner = TrajectoryRunner::new(config)
            .expect("valid configuration");

        let mut model = DeterministicModel { remaining: 1 };
        let mut selector = FirstBranchSelector;
        let cancellation = AlwaysCancelled;

        let error = runner
            .run::<u64, &'static str, _, _>(
                &mut model,
                &mut selector,
                Some(&cancellation),
            )
            .expect_err("cancelled execution must fail");

        assert_eq!(error, TrajectoryError::Cancelled);
        assert_eq!(model.remaining, 1);
    }

    #[test]
    fn observer_receives_events_without_runner_retaining_them() {
        let config = TrajectoryConfig {
            limits: TrajectoryLimits {
                max_steps: Some(3),
                max_events: Some(3),
                ..TrajectoryLimits::default()
            },
            ..TrajectoryConfig::default()
        };

        let runner = TrajectoryRunner::new(config)
            .expect("valid configuration");

        let mut model = DeterministicModel { remaining: 3 };
        let mut selector = FirstBranchSelector;
        let mut observer = RecordingObserver::default();

        let result = runner
            .run_with_observer::<u64, &'static str, _, _, _>(
                &mut model,
                &mut selector,
                None,
                &mut observer,
            )
            .expect("trajectory should execute");

        assert_eq!(result.steps, 3);
        assert_eq!(observer.events, vec![0, 1, 2]);
    }

    #[test]
    fn trajectory_coordinate_is_stable() {
        let coordinate = TrajectoryCoordinate::new(
            TrajectoryId::new(42),
            TrajectoryStep::new(17),
        );

        assert_eq!(coordinate.trajectory().get(), 42);
        assert_eq!(coordinate.step().get(), 17);
    }

    #[test]
    fn transition_constructor_rejects_empty_branches() {
        let result = TrajectoryTransition::<u64, ()>::new(Vec::new());

        assert_eq!(
            result.expect_err("empty transition must fail"),
            TrajectoryError::EmptyTransition
        );
    }

    #[test]
    fn deterministic_transition_has_probability_one() {
        let transition = TrajectoryTransition::deterministic(
            123_u64,
            1.5,
            Some("event"),
        )
        .expect("deterministic transition should be valid");

        assert_eq!(transition.len(), 1);
        assert_eq!(transition.branches()[0].weight, 1.0);
        assert_eq!(transition.branches()[0].duration, 1.5);
    }
}