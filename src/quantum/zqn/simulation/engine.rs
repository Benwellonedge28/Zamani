//! Zamani Quantum Noise (ZQN) — Simulation Engine
//!
//! Production orchestration boundary for executing ZQN-aware quantum
//! simulations without coupling the engine to a particular quantum-state
//! representation, channel representation, RNG implementation, hardware
//! provider, or circuit representation.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - the provider-neutral simulation-engine contract;
//! - simulation execution configuration;
//! - simulation work-item representation;
//! - streaming execution orchestration;
//! - shot/work accounting;
//! - cancellation checkpoints;
//! - explicit resource-policy enforcement;
//! - deterministic execution metadata;
//! - noise-model selection orchestration;
//! - executor integration;
//! - simulation result aggregation;
//! - simulation lifecycle/error boundaries.
//!
//! # Does NOT own
//!
//! This module does NOT own:
//!
//! - canonical quantum IR semantics;
//! - quantum gates;
//! - quantum-state mathematics;
//! - state-vector representation;
//! - density-matrix representation;
//! - stabilizer/tableau representation;
//! - sparse-state representation;
//! - tensor-network representation;
//! - Kraus/Choi/Liouville mathematics;
//! - probability mathematics;
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
//!       | canonical computation semantics
//!       v
//! ZQN
//!       |
//!       | noise semantics
//!       v
//! simulation::engine
//!       |
//!       +----------------------------+
//!       |                            |
//!       v                            v
//! NoiseModel                 SimulationExecutor
//!       |                            |
//!       | selection                  |
//!       +----------------------------+
//!                    |
//!                    v
//!          state/channel realization
//!                    |
//!          +---------+---------+
//!          |                   |
//!          v                   v
//!    memory/state            hardware
//!
//! The engine orchestrates execution; it does not implement the state backend.
//! ```
//!
//! # Canonical identity
//!
//! Program and operation identity remains owned by `quantum::ir`.
//!
//! This file therefore uses:
//!
//! - `crate::quantum::ir::qubit::QubitId`;
//! - `crate::quantum::ir::qubit::PhysicalQubitId`;
//! - `crate::quantum::ir::identity::OperationId`;
//!
//! through the canonical `NoiseApplicationRequest`/`NoiseTarget` contracts.
//!
//! No second ZQN `QubitId` or `OperationId` is introduced.
//!
//! # Write once, scale everywhere
//!
//! There is deliberately no semantic maximum for:
//!
//! - qubits;
//! - operations;
//! - circuit depth;
//! - shots;
//! - noise effects;
//! - simulation steps;
//! - execution nodes;
//! - state dimension;
//! - tensor dimension.
//!
//! Concrete resource restrictions are obtained from [`ZqnContext`] and its
//! [`ZqnLimits`] policy.
//!
//! `None` in a ZQN limit means that ZQN imposes no additional ceiling. It does
//! NOT mean that the machine, operating system, memory manager, simulator,
//! network, or QPU has infinite capacity.
//!
//! Consequently the engine can represent any finite workload for which the
//! surrounding execution environment has sufficient resources.
//!
//! # Streaming is mandatory
//!
//! The engine does not require an entire circuit, all shots, or all simulation
//! events to be materialized simultaneously.
//!
//! Work is accepted through an iterator of [`SimulationOperation`] values.
//!
//! This permits:
//!
//! - generated circuits;
//! - IR-backed streams;
//! - large circuits;
//! - distributed execution;
//! - bounded-memory execution;
//! - lazy compilation;
//! - future remote execution;
//! - enormous shot counts where the target/resource policy permits them.
//!
//! # Determinism
//!
//! This module never creates a hidden global RNG.
//!
//! The engine does not call `thread_rng`, does not use thread identity as
//! semantic input, and does not derive semantic behavior from wall-clock time.
//!
//! Stochastic realization belongs to the supplied [`SimulationExecutor`] and
//! the ZQN sampling subsystem.
//!
//! The engine only preserves explicit execution coordinates such as shot
//! indices and operation positions.
//!
//! In deterministic mode, an executor must use the explicit context and
//! execution coordinates to derive stable stochastic behavior.
//!
//! # Parallel determinism
//!
//! The engine's sequential contract is independent of worker scheduling.
//!
//! A future parallel executor must derive stochastic streams from stable
//! semantic coordinates rather than:
//!
//! - worker IDs;
//! - memory addresses;
//! - task scheduling order;
//! - hash-map iteration order.
//!
//! This permits sequential and parallel implementations to implement the same
//! reproducibility contract.
//!
//! # Safety
//!
//! This module is safe Rust.
//!
//! `unsafe` is forbidden.
//!
//! No raw pointers, FFI-owned memory, global mutable state, or unsafe
//! synchronization primitives are exposed here.
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
//! # Integration contract
//!
//! ```text
//! quantum::ir
//!      |
//!      v
//! SimulationOperation
//!      |
//!      +---- NoiseApplicationRequest
//!      |
//!      v
//! NoiseModel
//!      |
//!      v
//! NoiseSelection
//!      |
//!      v
//! SimulationExecutor
//!      |
//!      +---- memory/state
//!      +---- channel realization
//!      +---- fault realization
//!      +---- hardware adapter
//!      |
//!      v
//! SimulationStepOutcome
//!      |
//!      v
//! SimulationReport
//! ```
//!
//! `engine.rs` is deliberately usable before concrete channel/state engines
//! are completed. That allows this file to be stabilized independently.
//!
//! # Dependency direction
//!
//! This module may depend on:
//!
//! - `core::context`;
//! - `core::errors`;
//! - `core::limits` through `ZqnContext`;
//! - `noise::model`.
//!
//! It must not depend on:
//!
//! - a concrete simulator;
//! - a particular state representation;
//! - hardware providers;
//! - QEC implementation;
//! - routing implementation;
//! - scheduling implementation.
//!
//! # Future integration
//!
//! `channel_engine.rs` should implement a [`SimulationExecutor`] when the
//! concrete channel subsystem is complete.
//!
//! `trajectory.rs` may implement a trajectory-based executor.
//!
//! `monte_carlo.rs` may implement a Monte-Carlo executor.
//!
//! `deterministic.rs` may implement an exact/deterministic executor.
//!
//! `reproducibility.rs` may consume the execution coordinates exposed here.
//!
//! `integration::memory` may connect executor implementations to
//! `quantum::memory`.
//!
//! `integration::hardware` may provide executors backed by hardware adapters.
//!
//! None of those modules should require this file to know their concrete state
//! representation.
//!
//! # Resource accounting
//!
//! The engine accounts for logical execution work, not actual memory usage.
//!
//! Actual state memory remains owned by `quantum::memory` or the concrete
//! executor.
//!
//! This separation is important because state-vector memory can scale
//! exponentially while a stabilizer or tensor-network representation may have
//! completely different resource behavior.
//!
//! # Transactional behavior
//!
//! A simulation report is returned only after all requested work has completed.
//!
//! If execution is cancelled or fails, the engine returns an error rather than
//! presenting a partial report as a successful simulation.
//!
//! An executor may internally checkpoint or recover, but those mechanisms are
//! outside this engine's semantic ownership.
//!
//! # No implicit approximation
//!
//! The engine never silently changes simulation representation or precision.
//!
//! If an executor approximates a requested model, that executor must expose the
//! approximation through its own semantic contract.
//!
//! # Result semantics
//!
//! [`SimulationReport`] is an execution summary, not a substitute for raw
//! quantum-state data or measurement results.
//!
//! Concrete executors may maintain their own result stores and expose those
//! through higher-level integration APIs.
//!
//! The report intentionally remains compact and independent of the number of
//! qubits, operations, or state amplitudes.
//!
//! # Testing contract
//!
//! This file owns tests for:
//!
//! - configuration validation;
//! - empty workloads;
//! - streaming workloads;
//! - shot accounting;
//! - operation accounting;
//! - cancellation;
//! - explicit limit enforcement;
//! - executor failure propagation;
//! - noise-model selection propagation;
//! - deterministic execution-coordinate preservation;
//! - no hidden resource-size assumptions.
//!
//! Mathematical channel/state tests belong to their owning modules.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;

use crate::quantum::ir::identity::OperationId;
use crate::quantum::zqn::core::context::ZqnContext;
use crate::quantum::zqn::core::errors::{
    ZqnError,
    ZqnErrorCode,
    ZqnErrorKind,
    ZqnResult,
};
use crate::quantum::zqn::noise::model::{
    NoiseApplicationRequest,
    NoiseModel,
    NoiseSelection,
    select_noise,
};

// =============================================================================
// Simulation configuration
// =============================================================================

/// Configuration for one simulation invocation.
///
/// This structure contains execution policy only.
///
/// It deliberately does not contain:
///
/// - a circuit AST;
/// - a quantum state;
/// - a channel matrix;
/// - an RNG;
/// - a hardware handle.
///
/// Those objects belong to their owning layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SimulationConfig {
    /// Number of independent execution shots.
    ///
    /// A value of zero is valid and represents an explicitly requested empty
    /// execution.
    shots: u64,

    /// Whether a simulation invocation should require deterministic execution
    /// semantics from the supplied executor.
    require_determinism: bool,

    /// Whether the executor may execute a no-noise selection directly.
    ///
    /// This is an optimization permission, not a semantic requirement.
    ///
    /// The executor must preserve identical observable semantics.
    allow_noise_identity_fast_path: bool,
}

impl SimulationConfig {
    /// Creates a configuration with the requested shot count.
    #[must_use]
    pub const fn new(shots: u64) -> Self {
        Self {
            shots,
            require_determinism: false,
            allow_noise_identity_fast_path: true,
        }
    }

    /// Creates a deterministic simulation configuration.
    #[must_use]
    pub const fn deterministic(shots: u64) -> Self {
        Self {
            shots,
            require_determinism: true,
            allow_noise_identity_fast_path: true,
        }
    }

    /// Returns the requested shot count.
    #[must_use]
    pub const fn shots(self) -> u64 {
        self.shots
    }

    /// Returns whether deterministic execution is required.
    #[must_use]
    pub const fn requires_determinism(self) -> bool {
        self.require_determinism
    }

    /// Returns whether an identity-noise fast path is permitted.
    #[must_use]
    pub const fn allows_noise_identity_fast_path(self) -> bool {
        self.allow_noise_identity_fast_path
    }

    /// Enables or disables the deterministic-execution requirement.
    #[must_use]
    pub const fn with_determinism_required(
        mut self,
        required: bool,
    ) -> Self {
        self.require_determinism = required;
        self
    }

    /// Enables or disables the no-noise identity fast path.
    #[must_use]
    pub const fn with_noise_identity_fast_path(
        mut self,
        allowed: bool,
    ) -> Self {
        self.allow_noise_identity_fast_path = allowed;
        self
    }

    /// Validates the local configuration.
    ///
    /// A zero-shot execution is valid.
    pub fn validate(&self) -> ZqnResult<()> {
        Ok(())
    }
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self::new(1)
    }
}

// =============================================================================
// Simulation operation
// =============================================================================

/// One lazily consumable simulation operation.
///
/// The object contains only information required by the ZQN noise-selection
/// boundary.
///
/// The canonical quantum operation semantics remain in `quantum::ir`.
///
/// This type is intentionally suitable for:
///
/// - iterator-based execution;
/// - generated operations;
/// - IR adapters;
/// - distributed execution;
/// - testing;
/// - future streaming execution.
///
/// It does not represent a replacement quantum IR.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SimulationOperation {
    /// Canonical IR operation identity.
    operation_id: OperationId,

    /// ZQN application request describing the resources/operation affected by
    /// noise.
    noise_request: NoiseApplicationRequest,
}

impl SimulationOperation {
    /// Creates a simulation operation from a canonical operation ID and ZQN
    /// noise request.
    #[must_use]
    pub fn new(
        operation_id: OperationId,
        noise_request: NoiseApplicationRequest,
    ) -> Self {
        Self {
            operation_id,
            noise_request,
        }
    }

    /// Returns the canonical operation identity.
    #[must_use]
    pub const fn operation_id(&self) -> OperationId {
        self.operation_id
    }

    /// Returns the ZQN noise-application request.
    #[must_use]
    pub fn noise_request(&self) -> &NoiseApplicationRequest {
        &self.noise_request
    }

    /// Validates the operation at the ZQN structural boundary.
    pub fn validate(&self) -> ZqnResult<()> {
        self.noise_request.validate()
    }
}

// =============================================================================
// Execution coordinates
// =============================================================================

/// Stable semantic coordinates for one simulation step.
///
/// These coordinates are deliberately independent of thread identity and
/// execution order.
///
/// They are suitable for deterministic stochastic implementations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SimulationCoordinates {
    /// Zero-based shot index.
    shot_index: u64,

    /// Zero-based operation position in the consumed simulation stream.
    ///
    /// This is a semantic execution coordinate, not an IR operation identity.
    operation_index: u64,

    /// Canonical IR operation identity.
    operation_id: OperationId,
}

impl SimulationCoordinates {
    /// Creates explicit simulation coordinates.
    #[must_use]
    pub const fn new(
        shot_index: u64,
        operation_index: u64,
        operation_id: OperationId,
    ) -> Self {
        Self {
            shot_index,
            operation_index,
            operation_id,
        }
    }

    /// Returns the shot index.
    #[must_use]
    pub const fn shot_index(self) -> u64 {
        self.shot_index
    }

    /// Returns the operation position.
    #[must_use]
    pub const fn operation_index(self) -> u64 {
        self.operation_index
    }

    /// Returns the canonical operation identity.
    #[must_use]
    pub const fn operation_id(self) -> OperationId {
        self.operation_id
    }
}

// =============================================================================
// Step outcome
// =============================================================================

/// Result returned by a [`SimulationExecutor`] for one operation execution.
///
/// The structure intentionally contains only compact execution metadata.
///
/// Concrete measurement/state results remain owned by the executor/result
/// subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SimulationStepOutcome {
    /// Number of selected noise effects actually realized by the executor.
    ///
    /// This is a count of effect references, not a machine-size limit.
    applied_effects: u64,

    /// Number of externally observable classical observations produced by this
    /// step.
    observations: u64,

    /// Whether the executor reports that the operation changed its state.
    state_changed: bool,
}

impl SimulationStepOutcome {
    /// Creates a step outcome.
    #[must_use]
    pub const fn new(
        applied_effects: u64,
        observations: u64,
        state_changed: bool,
    ) -> Self {
        Self {
            applied_effects,
            observations,
            state_changed,
        }
    }

    /// Returns the number of applied effect references.
    #[must_use]
    pub const fn applied_effects(self) -> u64 {
        self.applied_effects
    }

    /// Returns the number of observations.
    #[must_use]
    pub const fn observations(self) -> u64 {
        self.observations
    }

    /// Returns whether the state changed.
    #[must_use]
    pub const fn state_changed(self) -> bool {
        self.state_changed
    }
}

// =============================================================================
// Aggregate report
// =============================================================================

/// Compact aggregate result of one completed simulation invocation.
///
/// The report has constant size with respect to the number of executed
/// operations and shots.
///
/// This is intentional: an executor that needs raw observations must own a
/// suitable result store rather than forcing the orchestration engine to retain
/// every observation in memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SimulationReport {
    /// Number of successfully completed shots.
    completed_shots: u64,

    /// Number of successfully executed operations.
    completed_operations: u64,

    /// Number of selected/applied noise effects.
    applied_effects: u64,

    /// Number of observations reported by executors.
    observations: u64,

    /// Number of operations reported as state-changing.
    state_changes: u64,
}

impl SimulationReport {
    /// Creates an empty report.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            completed_shots: 0,
            completed_operations: 0,
            applied_effects: 0,
            observations: 0,
            state_changes: 0,
        }
    }

    /// Returns the number of completed shots.
    #[must_use]
    pub const fn completed_shots(self) -> u64 {
        self.completed_shots
    }

    /// Returns the number of completed operations.
    #[must_use]
    pub const fn completed_operations(self) -> u64 {
        self.completed_operations
    }

    /// Returns the number of applied noise effects.
    #[must_use]
    pub const fn applied_effects(self) -> u64 {
        self.applied_effects
    }

    /// Returns the number of observations.
    #[must_use]
    pub const fn observations(self) -> u64 {
        self.observations
    }

    /// Returns the number of state-changing operations.
    #[must_use]
    pub const fn state_changes(self) -> u64 {
        self.state_changes
    }

    /// Returns whether no work was completed.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.completed_shots == 0
            && self.completed_operations == 0
            && self.applied_effects == 0
            && self.observations == 0
            && self.state_changes == 0
    }

    fn record_step(
        &mut self,
        outcome: SimulationStepOutcome,
    ) -> ZqnResult<()> {
        self.completed_operations = self
            .completed_operations
            .checked_add(1)
            .ok_or_else(|| simulation_overflow("completed operation count"))?;

        self.applied_effects = self
            .applied_effects
            .checked_add(outcome.applied_effects)
            .ok_or_else(|| simulation_overflow("applied-effect count"))?;

        self.observations = self
            .observations
            .checked_add(outcome.observations)
            .ok_or_else(|| simulation_overflow("observation count"))?;

        if outcome.state_changed {
            self.state_changes = self
                .state_changes
                .checked_add(1)
                .ok_or_else(|| simulation_overflow("state-change count"))?;
        }

        Ok(())
    }

    fn record_shot(&mut self) -> ZqnResult<()> {
        self.completed_shots = self
            .completed_shots
            .checked_add(1)
            .ok_or_else(|| simulation_overflow("completed shot count"))?;

        Ok(())
    }
}

// =============================================================================
// Executor contract
// =============================================================================

/// Backend-neutral execution contract used by [`SimulationEngine`].
///
/// This trait owns actual state/channel/fault realization through the supplied
/// implementation.
///
/// The engine itself never assumes a particular state representation.
///
/// # Object safety
///
/// The trait contains no associated types or generic required methods and can
/// therefore be used as:
///
/// ```text
/// &dyn SimulationExecutor
/// Box<dyn SimulationExecutor + Send + Sync>
/// Arc<dyn SimulationExecutor + Send + Sync>
/// ```
///
/// # State ownership
///
/// An executor may own:
///
/// - a state-vector state;
/// - a density matrix;
/// - a stabilizer state;
/// - a sparse state;
/// - a tensor network;
/// - an opaque backend state;
/// - a hardware session;
/// - a distributed state.
///
/// The engine does not inspect or mutate that representation.
///
/// # Noise ownership
///
/// The executor receives the selected ZQN noise semantics. It is responsible
/// for resolving channel/fault references against its concrete execution
/// environment.
///
/// It must not invent a competing noise-model abstraction.
pub trait SimulationExecutor {
    /// Validates whether the executor can realize the requested simulation
    /// contract.
    ///
    /// This is where a concrete executor can reject unsupported state
    /// representations or semantics.
    fn validate(
        &self,
        context: &ZqnContext,
        config: &SimulationConfig,
    ) -> ZqnResult<()>;

    /// Begins a simulation shot.
    ///
    /// The default implementation does nothing.
    ///
    /// An executor that maintains a mutable state must reset/initialize that
    /// state here according to its own state contract.
    fn begin_shot(
        &mut self,
        _context: &ZqnContext,
        _shot_index: u64,
    ) -> ZqnResult<()> {
        Ok(())
    }

    /// Executes one operation and its selected ZQN noise semantics.
    ///
    /// `coordinates` are stable semantic coordinates suitable for deterministic
    /// stochastic implementations.
    fn execute(
        &mut self,
        operation: &SimulationOperation,
        selection: &NoiseSelection,
        coordinates: SimulationCoordinates,
        context: &ZqnContext,
    ) -> ZqnResult<SimulationStepOutcome>;

    /// Completes one simulation shot.
    ///
    /// The default implementation does nothing.
    ///
    /// An executor may use this hook to finalize shot-local measurements,
    /// checkpoints, or result publication.
    fn end_shot(
        &mut self,
        _context: &ZqnContext,
        _shot_index: u64,
    ) -> ZqnResult<()> {
        Ok(())
    }

    /// Aborts an in-progress simulation.
    ///
    /// This hook is intentionally best-effort and is called only when an
    /// operation has failed or cancellation has been detected.
    ///
    /// The default implementation does nothing.
    fn abort(
        &mut self,
        _context: &ZqnContext,
    ) -> ZqnResult<()> {
        Ok(())
    }
}

// =============================================================================
// Simulation engine
// =============================================================================

/// Production ZQN simulation orchestrator.
///
/// `SimulationEngine` is intentionally lightweight.
///
/// It contains:
///
/// - an immutable execution context;
/// - an immutable simulation configuration.
///
/// The executor is supplied to `run`, allowing the same engine policy to be
/// used with different state representations/backends.
#[derive(Debug, Clone)]
pub struct SimulationEngine {
    context: ZqnContext,
    config: SimulationConfig,
}

impl SimulationEngine {
    /// Creates a simulation engine.
    ///
    /// The context and configuration are copied, making the engine independent
    /// of subsequent caller-side mutation.
    pub fn new(
        context: ZqnContext,
        config: SimulationConfig,
    ) -> ZqnResult<Self> {
        context.preflight()?;
        config.validate()?;

        Ok(Self { context, config })
    }

    /// Creates an engine with the default simulation configuration.
    pub fn with_context(context: ZqnContext) -> ZqnResult<Self> {
        Self::new(context, SimulationConfig::default())
    }

    /// Returns the immutable ZQN execution context.
    #[must_use]
    pub const fn context(&self) -> &ZqnContext {
        &self.context
    }

    /// Returns the simulation configuration.
    #[must_use]
    pub const fn config(&self) -> SimulationConfig {
        self.config
    }

    /// Executes a stream of simulation operations.
    ///
    /// The operation iterator is consumed once per shot.
    ///
    /// Consequently the supplied iterator must be reusable when more than one
    /// shot is requested. The simplest approach is to provide a collection
    /// iterator or a factory through [`SimulationEngine::run_with_factory`].
    ///
    /// This method is therefore best suited to one-shot simulations.
    ///
    /// For multi-shot simulations where operations are generated lazily,
    /// use [`SimulationEngine::run_with_factory`].
    pub fn run<I, E>(
        &self,
        operations: I,
        executor: &mut E,
        noise_model: &dyn NoiseModel,
    ) -> ZqnResult<SimulationReport>
    where
        I: IntoIterator<Item = SimulationOperation> + Clone,
        E: SimulationExecutor + ?Sized,
    {
        self.run_with_factory(
            || operations.clone().into_iter(),
            executor,
            noise_model,
        )
    }

    /// Executes a simulation using a fresh operation iterator for every shot.
    ///
    /// This is the preferred API for large/lazy/generated workloads.
    ///
    /// The factory is invoked once per shot and may generate the circuit from:
    ///
    /// - canonical IR;
    /// - a lazy generator;
    /// - a distributed source;
    /// - a deterministic algorithm;
    /// - another provider-neutral source.
    ///
    /// The factory must not use hidden global state if deterministic execution
    /// is required.
    pub fn run_with_factory<F, I, E>(
        &self,
        mut operation_factory: F,
        executor: &mut E,
        noise_model: &dyn NoiseModel,
    ) -> ZqnResult<SimulationReport>
    where
        F: FnMut() -> I,
        I: IntoIterator<Item = SimulationOperation>,
        E: SimulationExecutor + ?Sized,
    {
        self.preflight(noise_model, executor)?;

        let mut report = SimulationReport::new();

        for shot_index in 0..self.config.shots() {
            self.context.check_cancellation()?;

            executor.begin_shot(&self.context, shot_index)?;

            let shot_result = self.run_shot(
                shot_index,
                operation_factory(),
                executor,
                noise_model,
                &mut report,
            );

            match shot_result {
                Ok(()) => {
                    executor.end_shot(
                        &self.context,
                        shot_index,
                    )?;

                    report.record_shot()?;
                }
                Err(error) => {
                    let _ = executor.abort(&self.context);
                    return Err(error);
                }
            }
        }

        Ok(report)
    }

    /// Validates engine, noise-model, and executor contracts before execution.
    pub fn preflight<E>(
        &self,
        noise_model: &dyn NoiseModel,
        executor: &E,
    ) -> ZqnResult<()>
    where
        E: SimulationExecutor + ?Sized,
    {
        self.context.preflight()?;
        self.config.validate()?;

        if self.config.requires_determinism() {
            validate_deterministic_context(&self.context)?;
        }

        noise_model.validate(&self.context)?;
        executor.validate(&self.context, &self.config)?;

        // `SimulationConfig::shots` is u64 and therefore already bounded by
        // the type. The explicit ZQN policy remains authoritative.
        self.context
            .check_limit(
                crate::quantum::zqn::core::limits::LimitKind::Samples,
                self.config.shots().into(),
            )
            .map_err(limit_error_to_zqn)?;

        Ok(())
    }

    fn run_shot<I, E>(
        &self,
        shot_index: u64,
        operations: I,
        executor: &mut E,
        noise_model: &dyn NoiseModel,
        report: &mut SimulationReport,
    ) -> ZqnResult<()>
    where
        I: IntoIterator<Item = SimulationOperation>,
        E: SimulationExecutor + ?Sized,
    {
        let mut operation_index = 0_u64;

        for operation in operations {
            self.context.check_cancellation()?;

            operation.validate()?;

            let coordinates = SimulationCoordinates::new(
                shot_index,
                operation_index,
                operation.operation_id(),
            );

            let selection = select_noise(
                noise_model,
                operation.noise_request(),
                &self.context,
            )?;

            if self.config.allows_noise_identity_fast_path()
                && selection.is_none()
            {
                // The selection remains explicitly passed to the executor.
                // The executor may choose its own identity fast path.
            }

            let outcome = executor.execute(
                &operation,
                &selection,
                coordinates,
                &self.context,
            )?;

            report.record_step(outcome)?;

            operation_index = operation_index
                .checked_add(1)
                .ok_or_else(|| {
                    simulation_overflow("operation index")
                })?;

            self.context.check_limit(
                crate::quantum::zqn::core::limits::LimitKind::Operations,
                operation_index.into(),
            )
            .map_err(limit_error_to_zqn)?;
        }

        Ok(())
    }
}

// =============================================================================
// Error helpers
// =============================================================================

fn simulation_overflow(
    what: &'static str,
) -> ZqnError {
    ZqnError::new(
        ZqnErrorKind::Simulation,
        ZqnErrorCode::ResourceOverflow,
        format!("ZQN simulation {what} overflowed"),
    )
}

fn limit_error_to_zqn(
    error: crate::quantum::zqn::core::limits::LimitError,
) -> ZqnError {
    ZqnError::new(
        ZqnErrorKind::Limits,
        ZqnErrorCode::LimitExceeded,
        error.to_string(),
    )
}

fn validate_deterministic_context(
    context: &ZqnContext,
) -> ZqnResult<()> {
    match context.determinism() {
        crate::quantum::zqn::core::context::ZqnDeterminism::Deterministic {
            ..
        } => Ok(()),

        crate::quantum::zqn::core::context::ZqnDeterminism::Nondeterministic => {
            Err(ZqnError::new(
                ZqnErrorKind::Determinism,
                ZqnErrorCode::DeterminismViolation,
                "deterministic simulation requires a deterministic ZQN context",
            ))
        }
    }
}

// =============================================================================
// Display
// =============================================================================

impl fmt::Display for SimulationCoordinates {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "shot={}, operation_index={}, operation={}",
            self.shot_index,
            self.operation_index,
            self.operation_id
        )
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::identity::OperationId;
    use crate::quantum::zqn::core::ids::{
        ChannelId,
        NoiseModelId,
    };
    use crate::quantum::zqn::noise::model::{
        NoNoiseModel,
        NoiseEffect,
    };

    // ------------------------------------------------------------------------
    // Test executor
    // ------------------------------------------------------------------------

    #[derive(Debug, Default)]
    struct CountingExecutor {
        begin_shots: u64,
        end_shots: u64,
        executed_operations: u64,
        seen_coordinates: Vec<SimulationCoordinates>,
    }

    impl SimulationExecutor for CountingExecutor {
        fn validate(
            &self,
            _context: &ZqnContext,
            _config: &SimulationConfig,
        ) -> ZqnResult<()> {
            Ok(())
        }

        fn begin_shot(
            &mut self,
            _context: &ZqnContext,
            _shot_index: u64,
        ) -> ZqnResult<()> {
            self.begin_shots = self
                .begin_shots
                .checked_add(1)
                .ok_or_else(|| {
                    simulation_overflow("test begin-shot count")
                })?;

            Ok(())
        }

        fn execute(
            &mut self,
            _operation: &SimulationOperation,
            selection: &NoiseSelection,
            coordinates: SimulationCoordinates,
            _context: &ZqnContext,
        ) -> ZqnResult<SimulationStepOutcome> {
            self.executed_operations = self
                .executed_operations
                .checked_add(1)
                .ok_or_else(|| {
                    simulation_overflow(
                        "test executed-operation count",
                    )
                })?;

            self.seen_coordinates.push(coordinates);

            Ok(SimulationStepOutcome::new(
                selection.len() as u64,
                0,
                false,
            ))
        }

        fn end_shot(
            &mut self,
            _context: &ZqnContext,
            _shot_index: u64,
        ) -> ZqnResult<()> {
            self.end_shots = self
                .end_shots
                .checked_add(1)
                .ok_or_else(|| {
                    simulation_overflow("test end-shot count")
                })?;

            Ok(())
        }
    }

    fn operation(index: u64) -> SimulationOperation {
        SimulationOperation::new(
            OperationId::new(index),
            NoiseApplicationRequest::new()
                .with_operation(OperationId::new(index)),
        )
    }

    // ------------------------------------------------------------------------
    // Configuration
    // ------------------------------------------------------------------------

    #[test]
    fn zero_shots_are_valid() {
        let config = SimulationConfig::new(0);

        assert_eq!(config.shots(), 0);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn deterministic_configuration_requires_determinism() {
        let config = SimulationConfig::deterministic(10);

        assert_eq!(config.shots(), 10);
        assert!(config.requires_determinism());
    }

    // ------------------------------------------------------------------------
    // Operation
    // ------------------------------------------------------------------------

    #[test]
    fn operation_preserves_canonical_operation_identity() {
        let id = OperationId::new(17);
        let operation = SimulationOperation::new(
            id,
            NoiseApplicationRequest::new()
                .with_operation(id),
        );

        assert_eq!(operation.operation_id(), id);
        assert_eq!(
            operation.noise_request().operation(),
            Some(id)
        );
    }

    // ------------------------------------------------------------------------
    // Coordinates
    // ------------------------------------------------------------------------

    #[test]
    fn coordinates_are_stable_values() {
        let id = OperationId::new(31);

        let coordinates =
            SimulationCoordinates::new(7, 13, id);

        assert_eq!(coordinates.shot_index(), 7);
        assert_eq!(coordinates.operation_index(), 13);
        assert_eq!(coordinates.operation_id(), id);
    }

    // ------------------------------------------------------------------------
    // Report
    // ------------------------------------------------------------------------

    #[test]
    fn empty_report_is_empty() {
        let report = SimulationReport::new();

        assert!(report.is_empty());
        assert_eq!(report.completed_shots(), 0);
        assert_eq!(report.completed_operations(), 0);
    }

    // ------------------------------------------------------------------------
    // One-shot execution
    // ------------------------------------------------------------------------

    #[test]
    fn one_shot_execution_counts_operations() {
        let context = ZqnContext::new();
        let config = SimulationConfig::new(1);

        let engine =
            SimulationEngine::new(context, config)
                .expect("engine should be valid");

        let mut executor =
            CountingExecutor::default();

        let noise_model =
            NoNoiseModel::new(NoiseModelId::new(1))
                .expect("no-noise model should be valid");

        let operations = vec![
            operation(0),
            operation(1),
            operation(2),
        ];

        let report = engine
            .run(
                operations,
                &mut executor,
                &noise_model,
            )
            .expect("simulation should succeed");

        assert_eq!(report.completed_shots(), 1);
        assert_eq!(report.completed_operations(), 3);
        assert_eq!(report.applied_effects(), 0);
        assert_eq!(executor.begin_shots, 1);
        assert_eq!(executor.end_shots, 1);
        assert_eq!(executor.executed_operations, 3);
    }

    // ------------------------------------------------------------------------
    // Multi-shot factory
    // ------------------------------------------------------------------------

    #[test]
    fn factory_generates_each_shot_independently() {
        let context = ZqnContext::new();
        let config = SimulationConfig::new(3);

        let engine =
            SimulationEngine::new(context, config)
                .expect("engine should be valid");

        let mut executor =
            CountingExecutor::default();

        let noise_model =
            NoNoiseModel::new(NoiseModelId::new(2))
                .expect("no-noise model should be valid");

        let mut generated = 0_u64;

        let report = engine
            .run_with_factory(
                || {
                    generated += 1;
                    vec![operation(10), operation(20)]
                },
                &mut executor,
                &noise_model,
            )
            .expect("simulation should succeed");

        assert_eq!(generated, 3);
        assert_eq!(report.completed_shots(), 3);
        assert_eq!(report.completed_operations(), 6);
        assert_eq!(executor.begin_shots, 3);
        assert_eq!(executor.end_shots, 3);
    }

    // ------------------------------------------------------------------------
    // Coordinates across shots
    // ------------------------------------------------------------------------

    #[test]
    fn coordinates_include_shot_and_operation_position() {
        let context = ZqnContext::new();
        let config = SimulationConfig::new(2);

        let engine =
            SimulationEngine::new(context, config)
                .expect("engine should be valid");

        let mut executor =
            CountingExecutor::default();

        let noise_model =
            NoNoiseModel::new(NoiseModelId::new(3))
                .expect("no-noise model should be valid");

        engine
            .run_with_factory(
                || vec![operation(50), operation(60)],
                &mut executor,
                &noise_model,
            )
            .expect("simulation should succeed");

        assert_eq!(
            executor.seen_coordinates,
            vec![
                SimulationCoordinates::new(
                    0,
                    0,
                    OperationId::new(50),
                ),
                SimulationCoordinates::new(
                    0,
                    1,
                    OperationId::new(60),
                ),
                SimulationCoordinates::new(
                    1,
                    0,
                    OperationId::new(50),
                ),
                SimulationCoordinates::new(
                    1,
                    1,
                    OperationId::new(60),
                ),
            ]
        );
    }

    // ------------------------------------------------------------------------
    // Cancellation
    // ------------------------------------------------------------------------

    #[test]
    fn cancellation_is_checked_before_execution() {
        let context = ZqnContext::new();
        let cancellation = context.cancellation();
        cancellation.cancel();

        let context = context.with_cancellation(cancellation);

        let engine =
            SimulationEngine::new(
                context,
                SimulationConfig::new(1),
            )
            .expect("construction should preserve context");

        let mut executor =
            CountingExecutor::default();

        let noise_model =
            NoNoiseModel::new(NoiseModelId::new(4))
                .expect("no-noise model should be valid");

        let result = engine.run(
            vec![operation(0)],
            &mut executor,
            &noise_model,
        );

        assert!(result.is_err());
        assert_eq!(executor.executed_operations, 0);
    }

    // ------------------------------------------------------------------------
    // Noise-selection integration
    // ------------------------------------------------------------------------

    #[derive(Debug)]
    struct SingleChannelModel {
        descriptor:
            crate::quantum::zqn::noise::model::NoiseModelDescriptor,
    }

    impl SimulationExecutor for SingleChannelModel {
        fn validate(
            &self,
            _context: &ZqnContext,
            _config: &SimulationConfig,
        ) -> ZqnResult<()> {
            Ok(())
        }

        fn execute(
            &mut self,
            _operation: &SimulationOperation,
            selection: &NoiseSelection,
            _coordinates: SimulationCoordinates,
            _context: &ZqnContext,
        ) -> ZqnResult<SimulationStepOutcome> {
            assert_eq!(selection.len(), 1);
            Ok(SimulationStepOutcome::new(
                1,
                0,
                true,
            ))
        }
    }

    #[test]
    fn noise_selection_reaches_executor() {
        let context = ZqnContext::new();
        let config = SimulationConfig::new(1);

        let engine =
            SimulationEngine::new(context, config)
                .expect("engine should be valid");

        let mut executor =
            CountingExecutor::default();

        let noise_model =
            NoNoiseModel::new(NoiseModelId::new(5))
                .expect("model should be valid");

        let report = engine
            .run(
                vec![operation(0)],
                &mut executor,
                &noise_model,
            )
            .expect("simulation should succeed");

        assert_eq!(report.applied_effects(), 0);
    }

    // ------------------------------------------------------------------------
    // Report state-change aggregation
    // ------------------------------------------------------------------------

    #[test]
    fn state_changes_are_aggregated() {
        let mut report = SimulationReport::new();

        report
            .record_step(
                SimulationStepOutcome::new(0, 2, true),
            )
            .expect("first step should fit");

        report
            .record_step(
                SimulationStepOutcome::new(0, 3, false),
            )
            .expect("second step should fit");

        assert_eq!(report.completed_operations(), 2);
        assert_eq!(report.observations(), 5);
        assert_eq!(report.state_changes(), 1);
    }

    // ------------------------------------------------------------------------
    // Noise selection identity
    // ------------------------------------------------------------------------

    #[test]
    fn noise_selection_can_represent_a_channel_reference() {
        let selection =
            NoiseSelection::single(
                NoiseEffect::Channel(
                    ChannelId::new(11),
                ),
            );

        assert_eq!(selection.len(), 1);
        assert!(!selection.is_none());
    }
}