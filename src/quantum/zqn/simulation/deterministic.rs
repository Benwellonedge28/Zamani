//! Zamani Quantum Noise (ZQN) — Deterministic Simulation Executor
//!
//! Production deterministic-execution boundary for ZQN.
//!
//! # Mission
//!
//! This module provides the deterministic execution adapter used by
//! `simulation::engine`.
//!
//! Its purpose is to guarantee that deterministic execution is driven by
//! explicit semantic execution coordinates rather than:
//!
//! - thread identity;
//! - worker identity;
//! - memory addresses;
//! - wall-clock time;
//! - iteration order of unordered collections;
//! - hidden global state;
//! - hidden global RNG state;
//! - scheduler timing.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - the deterministic executor adapter;
//! - the deterministic backend contract;
//! - deterministic execution validation;
//! - deterministic execution lifecycle forwarding;
//! - explicit execution-coordinate forwarding;
//! - deterministic backend capability declaration;
//! - deterministic failure boundaries;
//! - deterministic executor tests.
//!
//! # Does NOT own
//!
//! This module does NOT own:
//!
//! - canonical quantum IR;
//! - qubit identity;
//! - quantum state representations;
//! - density matrices;
//! - state-vector mathematics;
//! - tensor-network mathematics;
//! - stabilizer mathematics;
//! - channel mathematics;
//! - Kraus operators;
//! - Choi matrices;
//! - stochastic distributions;
//! - random-number generation algorithms;
//! - seed derivation;
//! - noise-model semantics;
//! - routing;
//! - scheduling;
//! - QEC;
//! - calibration;
//! - hardware APIs;
//! - QPU transport;
//! - benchmarking methodology;
//! - serialization formats.
//!
//! Those responsibilities remain in their owning modules.
//!
//! # Architectural position
//!
//! ```text
//!                    quantum::ir
//!                         |
//!                         v
//!                SimulationOperation
//!                         |
//!                         v
//!                simulation::engine
//!                         |
//!                         v
//!               SimulationCoordinates
//!                         |
//!                         v
//!              deterministic.rs
//!                         |
//!                         v
//!             DeterministicBackend
//!                         |
//!             +-----------+-----------+
//!             |                       |
//!             v                       v
//!       exact simulator          channel engine
//!       tensor backend           hardware adapter
//!       stabilizer backend       distributed backend
//! ```
//!
//! `SimulationEngine` remains the orchestration owner.
//!
//! `DeterministicExecutor` provides the deterministic implementation of the
//! `SimulationExecutor` contract.
//!
//! # Determinism contract
//!
//! A deterministic backend MUST derive all semantic execution behavior from
//! explicit inputs:
//!
//! ```text
//! ZqnContext
//! + SimulationOperation
//! + NoiseSelection
//! + SimulationCoordinates
//! + backend configuration
//! ```
//!
//! The backend MUST NOT derive semantic results from:
//!
//! ```text
//! thread ID
//! worker ID
//! pointer address
//! wall-clock time
//! task scheduling order
//! hash-map iteration order
//! process-local hidden state
//! global RNG state
//! ```
//!
//! # Parallel determinism
//!
//! Deterministic execution is defined by semantic coordinates, not execution
//! order.
//!
//! Therefore a compliant implementation must make:
//!
//! ```text
//! sequential execution
//!
//! and
//!
//! parallel execution
//! ```
//!
//! semantically equivalent when given the same deterministic execution
//! coordinates and configuration.
//!
//! The wrapper in this module does not itself create worker threads. This is
//! deliberate: concurrency belongs to the execution/runtime layer.
//!
//! # Reproducibility
//!
//! This module does not implement seed derivation.
//!
//! `simulation::reproducibility` owns deterministic stochastic-stream
//! derivation.
//!
//! A deterministic backend that needs randomness must consume explicit
//! reproducibility material supplied by the appropriate ZQN context/service.
//!
//! It must never introduce an implicit RNG.
//!
//! # No hidden state
//!
//! `DeterministicExecutor` owns exactly one backend value.
//!
//! It does not contain:
//!
//! - static mutable state;
//! - global caches;
//! - global RNGs;
//! - thread-local semantic state;
//! - process-global calibration;
//! - process-global configuration.
//!
//! Backend-local mutable state is permitted because simulation state itself must
//! be mutable during execution. Such state must remain semantically controlled
//! by the backend and must not affect deterministic results through unspecified
//! execution ordering.
//!
//! # Scaling
//!
//! There is no semantic limit in this module for:
//!
//! - qubits;
//! - qudits;
//! - modes;
//! - operations;
//! - circuit depth;
//! - shots;
//! - dimensions;
//! - state elements;
//! - nodes;
//! - devices.
//!
//! Resource limits remain governed by `ZqnContext` and `ZqnLimits`.
//!
//! The deterministic adapter itself does not materialize all operations or all
//! shots. `SimulationEngine` supplies operations through iterators and invokes
//! the executor one operation at a time.
//!
//! Consequently this module is compatible with:
//!
//! - tiny simulations;
//! - very large generated workloads;
//! - streaming execution;
//! - distributed execution;
//! - remote execution;
//! - hardware execution;
//! - sparse representations;
//! - tensor-network representations;
//! - trajectory representations;
//! - exact representations.
//!
//! Actual scalability is determined by the selected backend and available
//! resources, not by a semantic ceiling in this module.
//!
//! # State representation independence
//!
//! The backend may use any suitable representation:
//!
//! ```text
//! state vector
//! density matrix
//! sparse state
//! stabilizer/tableau
//! tensor network
//! matrix-product representation
//! trajectory
//! symbolic state
//! hardware-native state
//! distributed state
//! ```
//!
//! This module deliberately does not select one.
//!
//! # Approximation
//!
//! This module never silently changes representation or precision.
//!
//! If a deterministic backend uses approximation, the backend must expose its
//! approximation contract through its own capability/configuration API.
//!
//! Approximation is never inferred merely from the fact that execution is
//! deterministic.
//!
//! # Error semantics
//!
//! Backend errors are propagated unchanged through the ZQN `ZqnResult`
//! boundary.
//!
//! Cancellation is owned by `ZqnContext` and therefore remains an explicit
//! execution failure rather than a successful partial result.
//!
//! # Resource safety
//!
//! This module does not allocate according to machine size.
//!
//! It does not:
//!
//! - collect an entire circuit;
//! - collect all shots;
//! - collect all results;
//! - create one object per qubit;
//! - create one object per operation before execution.
//!
//! Backend implementations remain responsible for their own resource use and
//! must honor the resource policy exposed by `ZqnContext`.
//!
//! # Transaction semantics
//!
//! `SimulationEngine` considers a simulation successful only after the
//! requested work has completed.
//!
//! On failure, the engine invokes `abort` on the executor.
//!
//! This module forwards that lifecycle exactly.
//!
//! It does not manufacture a partial successful report.
//!
//! # Canonical quantum-resource identity
//!
//! This module intentionally does not import `QubitId` or `PhysicalQubitId`.
//!
//! Deterministic scheduling requires:
//!
//! - shot identity;
//! - operation position;
//! - canonical operation identity.
//!
//! Those are already represented by `SimulationCoordinates` in
//! `simulation::engine`.
//!
//! When a deterministic backend needs quantum resources, it must obtain the
//! canonical identities through the operation/state/hardware integration layer:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! A second ZQN qubit identity is forbidden.
//!
//! # Integration with simulation::engine
//!
//! `DeterministicExecutor` implements:
//!
//! ```text
//! simulation::engine::SimulationExecutor
//! ```
//!
//! Therefore it can be supplied directly to:
//!
//! ```text
//! SimulationEngine::run(...)
//! SimulationEngine::run_with_factory(...)
//! ```
//!
//! `SimulationEngine` remains responsible for:
//!
//! - shot iteration;
//! - operation iteration;
//! - noise selection;
//! - cancellation;
//! - resource-policy checks;
//! - aggregate report construction.
//!
//! This module remains responsible for deterministic backend delegation.
//!
//! # Integration with simulation::channel_engine
//!
//! A future channel executor can implement `DeterministicBackend` and then be
//! wrapped in `DeterministicExecutor`.
//!
//! The channel engine remains responsible for channel application.
//!
//! This module does not duplicate channel mathematics.
//!
//! # Integration with simulation::reproducibility
//!
//! A deterministic backend may consume explicit reproducibility information
//! from the ZQN reproducibility subsystem.
//!
//! Seed derivation remains outside this file.
//!
//! This separation prevents two competing deterministic algorithms from being
//! introduced accidentally.
//!
//! # Integration with simulation::sampler
//!
//! The sampler remains responsible for stochastic sampling.
//!
//! Deterministic execution means that stochastic sampling, when required, must
//! be reproducibly seeded; it does not mean that every noise model becomes
//! deterministic or that randomness is removed.
//!
//! # Integration with simulation::monte_carlo
//!
//! Monte Carlo execution may use this deterministic execution boundary when
//! each trial is assigned stable semantic coordinates.
//!
//! The Monte Carlo algorithm remains outside this module.
//!
//! # Integration with simulation::trajectory
//!
//! A trajectory backend can implement `DeterministicBackend` provided each
//! trajectory/shot receives stable execution coordinates.
//!
//! # Integration with memory
//!
//! Memory/state adapters remain outside this module.
//!
//! A backend may internally use:
//!
//! ```text
//! quantum::memory
//! ```
//!
//! without making `deterministic.rs` depend on a concrete memory
//! representation.
//!
//! # Integration with hardware
//!
//! Hardware adapters may implement `DeterministicBackend` if they can provide
//! the required reproducibility contract.
//!
//! Physical hardware may still contain environmental nondeterminism.
//!
//! Therefore the adapter must not claim mathematical determinism merely because
//! the execution request is deterministic. Hardware-specific reproducibility
//! guarantees must be explicitly declared by the hardware integration layer.
//!
//! # Integration with QEC
//!
//! QEC may use deterministic execution for:
//!
//! - reproducible fault injection;
//! - syndrome experiments;
//! - decoder regression tests;
//! - logical-error experiments.
//!
//! QEC semantics remain outside this file.
//!
//! # Integration with routing and scheduling
//!
//! Routing and scheduling do not depend directly on this executor.
//!
//! They may produce the canonical operations that eventually reach it.
//!
//! Execution coordinates remain stable only when the operation stream supplied
//! to the executor has stable semantic ordering.
//!
//! # Serialization
//!
//! `DeterministicExecutor` is intentionally not a wire-format type.
//!
//! Backend configuration serialization belongs to the backend/configuration
//! owner and `zqn::io`.
//!
//! Runtime executor objects, mutable state, open hardware sessions, and
//! cancellation state must not be serialized accidentally.
//!
//! # Thread safety
//!
//! This module does not impose `Send + Sync` on every backend because doing so
//! would unnecessarily constrain valid single-threaded deterministic
//! implementations.
//!
//! A backend intended for concurrent execution should independently satisfy
//! the appropriate `Send`/`Sync` requirements.
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
//! # Safety
//!
//! This file forbids unsafe Rust.
//!
//! No FFI, raw pointers, global mutable state, or unsafe synchronization is
//! required.
//!
//! # Definition of done
//!
//! This file is complete when:
//!
//! - `DeterministicBackend` is the only deterministic-backend contract;
//! - `DeterministicExecutor` implements `SimulationExecutor`;
//! - deterministic context validation is enforced;
//! - all execution coordinates are forwarded unchanged;
//! - lifecycle events are forwarded unchanged;
//! - backend failures are propagated;
//! - cancellation is checked through `ZqnContext`;
//! - no global RNG is created;
//! - no machine-size limit is introduced;
//! - no quantum-resource identity is duplicated;
//! - tests verify coordinate preservation and lifecycle behavior.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use crate::quantum::zqn::core::context::{
    ZqnContext,
    ZqnDeterminism,
};
use crate::quantum::zqn::core::errors::{
    ZqnError,
    ZqnErrorCode,
    ZqnErrorKind,
    ZqnResult,
};
use crate::quantum::zqn::noise::model::NoiseSelection;
use crate::quantum::zqn::simulation::engine::{
    SimulationConfig,
    SimulationCoordinates,
    SimulationExecutor,
    SimulationOperation,
    SimulationStepOutcome,
};

// =============================================================================
// Deterministic backend contract
// =============================================================================

/// Contract implemented by a backend capable of deterministic ZQN execution.
///
/// The trait is deliberately smaller than [`SimulationExecutor`].
///
/// It describes the backend-specific portion of deterministic execution while
/// `DeterministicExecutor` supplies the common ZQN lifecycle and validation
/// boundary.
///
/// # Determinism requirement
///
/// Implementors MUST ensure that semantic execution depends only on explicit
/// arguments and backend configuration.
///
/// In particular, implementors MUST NOT use:
///
/// - thread IDs as semantic input;
/// - worker IDs as semantic input;
/// - wall-clock time as semantic input;
/// - memory addresses as semantic input;
/// - unordered iteration as semantic input;
/// - hidden global RNG state;
/// - hidden process-global mutable state.
///
/// # Stable coordinates
///
/// `coordinates` are the authoritative execution coordinates for the operation.
///
/// A backend requiring reproducible randomness must derive its stochastic
/// stream from stable semantic inputs, normally through
/// `simulation::reproducibility`.
pub trait DeterministicBackend {
    /// Validates backend-specific deterministic requirements.
    ///
    /// This method must not mutate backend state.
    fn validate(
        &self,
        context: &ZqnContext,
        config: &SimulationConfig,
    ) -> ZqnResult<()>;

    /// Begins one deterministic shot.
    ///
    /// The default implementation performs no work.
    fn begin_shot(
        &mut self,
        _context: &ZqnContext,
        _shot_index: u64,
    ) -> ZqnResult<()> {
        Ok(())
    }

    /// Executes one deterministic operation.
    ///
    /// The backend receives the exact semantic coordinates generated by
    /// `SimulationEngine`.
    fn execute(
        &mut self,
        operation: &SimulationOperation,
        selection: &NoiseSelection,
        coordinates: SimulationCoordinates,
        context: &ZqnContext,
    ) -> ZqnResult<SimulationStepOutcome>;

    /// Ends one successfully completed deterministic shot.
    ///
    /// The default implementation performs no work.
    fn end_shot(
        &mut self,
        _context: &ZqnContext,
        _shot_index: u64,
    ) -> ZqnResult<()> {
        Ok(())
    }

    /// Aborts an in-progress deterministic execution.
    ///
    /// This hook is best-effort. The original execution error remains
    /// authoritative if an abort operation itself fails.
    fn abort(
        &mut self,
        _context: &ZqnContext,
    ) -> ZqnResult<()> {
        Ok(())
    }
}

// =============================================================================
// Deterministic executor
// =============================================================================

/// Deterministic implementation of the ZQN [`SimulationExecutor`] contract.
///
/// The wrapper is intentionally transparent:
///
/// ```text
/// SimulationEngine
///       │
///       ▼
/// SimulationExecutor
///       │
///       ▼
/// DeterministicExecutor
///       │
///       ▼
/// DeterministicBackend
/// ```
///
/// The wrapper does not alter operations, noise selections, coordinates, or
/// outcomes.
#[derive(Debug)]
pub struct DeterministicExecutor<B> {
    backend: B,
}

impl<B> DeterministicExecutor<B> {
    /// Creates a deterministic executor around `backend`.
    ///
    /// No validation is performed here because construction must remain cheap
    /// and side-effect free. Backend validation occurs through
    /// `SimulationExecutor::validate`.
    #[must_use]
    pub const fn new(backend: B) -> Self {
        Self { backend }
    }

    /// Returns an immutable reference to the backend.
    #[must_use]
    pub const fn backend(&self) -> &B {
        &self.backend
    }

    /// Returns a mutable reference to the backend.
    ///
    /// Mutation is intentionally exposed only through an explicit mutable
    /// reference so callers cannot accidentally create hidden global state.
    #[must_use]
    pub const fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
    }

    /// Consumes the executor and returns its backend.
    #[must_use]
    pub fn into_backend(self) -> B {
        self.backend
    }
}

impl<B> SimulationExecutor for DeterministicExecutor<B>
where
    B: DeterministicBackend,
{
    fn validate(
        &self,
        context: &ZqnContext,
        config: &SimulationConfig,
    ) -> ZqnResult<()> {
        validate_deterministic_context(context)?;

        self.backend.validate(context, config)
    }

    fn begin_shot(
        &mut self,
        context: &ZqnContext,
        shot_index: u64,
    ) -> ZqnResult<()> {
        context.check_cancellation()?;

        self.backend.begin_shot(
            context,
            shot_index,
        )
    }

    fn execute(
        &mut self,
        operation: &SimulationOperation,
        selection: &NoiseSelection,
        coordinates: SimulationCoordinates,
        context: &ZqnContext,
    ) -> ZqnResult<SimulationStepOutcome> {
        context.check_cancellation()?;

        // Never reconstruct or transform coordinates here.
        //
        // This is important for parallel determinism: the semantic coordinate
        // generated by SimulationEngine is the authoritative coordinate.
        self.backend.execute(
            operation,
            selection,
            coordinates,
            context,
        )
    }

    fn end_shot(
        &mut self,
        context: &ZqnContext,
        shot_index: u64,
    ) -> ZqnResult<()> {
        context.check_cancellation()?;

        self.backend.end_shot(
            context,
            shot_index,
        )
    }

    fn abort(
        &mut self,
        context: &ZqnContext,
    ) -> ZqnResult<()> {
        self.backend.abort(context)
    }
}

// =============================================================================
// Deterministic validation
// =============================================================================

/// Validates that a context explicitly requests deterministic execution.
fn validate_deterministic_context(
    context: &ZqnContext,
) -> ZqnResult<()> {
    match context.determinism() {
        ZqnDeterminism::Deterministic { .. } => Ok(()),

        ZqnDeterminism::Nondeterministic => Err(
            ZqnError::new(
                ZqnErrorKind::Determinism,
                ZqnErrorCode::DeterminismViolation,
                "deterministic execution requires a deterministic ZQN context",
            ),
        ),
    }
}

// =============================================================================
// Reference deterministic backend
// =============================================================================

/// Minimal deterministic backend useful for integration tests and adapter
/// development.
///
/// It intentionally performs no quantum-state evolution.
///
/// Its purpose is to provide a zero-semantic-cost backend for validating the
/// deterministic execution contract itself.
#[derive(Debug, Default, Clone, Copy)]
pub struct DeterministicNoopBackend;

impl DeterministicBackend for DeterministicNoopBackend {
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
        Ok(SimulationStepOutcome::new(
            selection.len() as u64,
            0,
            false,
        ))
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::identity::OperationId;
    use crate::quantum::zqn::noise::model::NoiseApplicationRequest;

    // -------------------------------------------------------------------------
    // Recording backend
    // -------------------------------------------------------------------------

    #[derive(Debug, Default)]
    struct RecordingBackend {
        validated: bool,
        begin_shots: Vec<u64>,
        coordinates: Vec<SimulationCoordinates>,
        end_shots: Vec<u64>,
        abort_called: bool,
    }

    impl DeterministicBackend for RecordingBackend {
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
            shot_index: u64,
        ) -> ZqnResult<()> {
            self.begin_shots.push(shot_index);
            Ok(())
        }

        fn execute(
            &mut self,
            _operation: &SimulationOperation,
            selection: &NoiseSelection,
            coordinates: SimulationCoordinates,
            _context: &ZqnContext,
        ) -> ZqnResult<SimulationStepOutcome> {
            self.coordinates.push(coordinates);

            Ok(SimulationStepOutcome::new(
                selection.len() as u64,
                0,
                false,
            ))
        }

        fn end_shot(
            &mut self,
            _context: &ZqnContext,
            shot_index: u64,
        ) -> ZqnResult<()> {
            self.end_shots.push(shot_index);
            Ok(())
        }

        fn abort(
            &mut self,
            _context: &ZqnContext,
        ) -> ZqnResult<()> {
            self.abort_called = true;
            Ok(())
        }
    }

    // -------------------------------------------------------------------------
    // Helpers
    // -------------------------------------------------------------------------

    fn operation(index: u64) -> SimulationOperation {
        SimulationOperation::new(
            OperationId::new(index),
            NoiseApplicationRequest::new()
                .with_operation(OperationId::new(index)),
        )
    }

    // -------------------------------------------------------------------------
    // Constructor
    // -------------------------------------------------------------------------

    #[test]
    fn constructor_preserves_backend() {
        let executor = DeterministicExecutor::new(
            DeterministicNoopBackend,
        );

        assert_eq!(
            executor.backend(),
            &DeterministicNoopBackend
        );
    }

    #[test]
    fn into_backend_returns_original_backend() {
        let executor = DeterministicExecutor::new(
            DeterministicNoopBackend,
        );

        assert_eq!(
            executor.into_backend(),
            DeterministicNoopBackend
        );
    }

    // -------------------------------------------------------------------------
    // Context validation
    // -------------------------------------------------------------------------

    #[test]
    fn deterministic_executor_rejects_nondeterministic_context() {
        let context = nondeterministic_context_for_test();
        let config = SimulationConfig::deterministic(1);

        let executor = DeterministicExecutor::new(
            DeterministicNoopBackend,
        );

        let result = executor.validate(
            &context,
            &config,
        );

        assert!(result.is_err());
    }

    // -------------------------------------------------------------------------
    // Coordinate preservation
    // -------------------------------------------------------------------------

    #[test]
    fn coordinates_are_forwarded_without_transformation() {
        let mut executor = DeterministicExecutor::new(
            RecordingBackend::default(),
        );

        let context = deterministic_context_for_test();
        let operation = operation(17);

        let coordinates = SimulationCoordinates::new(
            4,
            9,
            OperationId::new(17),
        );

        let selection = NoiseSelection::none();

        executor
            .execute(
                &operation,
                &selection,
                coordinates,
                &context,
            )
            .expect("deterministic execution should succeed");

        assert_eq!(
            executor.backend().coordinates,
            vec![coordinates],
        );
    }

    // -------------------------------------------------------------------------
    // Lifecycle
    // -------------------------------------------------------------------------

    #[test]
    fn lifecycle_is_forwarded() {
        let mut executor = DeterministicExecutor::new(
            RecordingBackend::default(),
        );

        let context = deterministic_context_for_test();
        let config = SimulationConfig::deterministic(1);

        executor
            .validate(&context, &config)
            .expect("validation should succeed");

        executor
            .begin_shot(&context, 3)
            .expect("begin should succeed");

        executor
            .execute(
                &operation(7),
                &NoiseSelection::none(),
                SimulationCoordinates::new(
                    3,
                    0,
                    OperationId::new(7),
                ),
                &context,
            )
            .expect("execute should succeed");

        executor
            .end_shot(&context, 3)
            .expect("end should succeed");

        assert_eq!(
            executor.backend().begin_shots,
            vec![3],
        );

        assert_eq!(
            executor.backend().end_shots,
            vec![3],
        );
    }

    // -------------------------------------------------------------------------
    // No-op backend
    // -------------------------------------------------------------------------

    #[test]
    fn noop_backend_is_deterministic_adapter_compatible() {
        let mut executor = DeterministicExecutor::new(
            DeterministicNoopBackend,
        );

        let context = deterministic_context_for_test();

        let outcome = executor
            .execute(
                &operation(1),
                &NoiseSelection::none(),
                SimulationCoordinates::new(
                    0,
                    0,
                    OperationId::new(1),
                ),
                &context,
            )
            .expect("noop execution should succeed");

        assert_eq!(
            outcome.applied_effects(),
            0,
        );

        assert_eq!(
            outcome.observations(),
            0,
        );

        assert!(!outcome.state_changed());
    }

    // -------------------------------------------------------------------------
    // Abort
    // -------------------------------------------------------------------------

    #[test]
    fn abort_is_forwarded() {
        let mut executor = DeterministicExecutor::new(
            RecordingBackend::default(),
        );

        let context = deterministic_context_for_test();

        executor
            .abort(&context)
            .expect("abort should succeed");

        assert!(
            executor.backend().abort_called
        );
    }

    // -------------------------------------------------------------------------
    // Cancellation
    // -------------------------------------------------------------------------

    #[test]
    fn cancellation_is_checked_before_backend_execution() {
        let context = cancelled_deterministic_context_for_test();

        let mut executor = DeterministicExecutor::new(
            RecordingBackend::default(),
        );

        let result = executor.execute(
            &operation(1),
            &NoiseSelection::none(),
            SimulationCoordinates::new(
                0,
                0,
                OperationId::new(1),
            ),
            &context,
        );

        assert!(result.is_err());

        assert!(
            executor.backend().coordinates.is_empty(),
            "cancelled execution must not reach the backend"
        );
    }

    // -------------------------------------------------------------------------
    // Helpers for context construction
    // -------------------------------------------------------------------------

    fn deterministic_context_for_test() -> ZqnContext {
        /*
         * This helper intentionally delegates construction to the repository's
         * existing ZqnContext API rather than creating a second test-only
         * determinism representation.
         *
         * If the context constructor evolves, this is the only test-local
         * construction boundary that needs adjustment.
         */
        ZqnContext::deterministic_for_testing()
    }

    fn nondeterministic_context_for_test() -> ZqnContext {
        ZqnContext::nondeterministic_for_testing()
    }

    fn cancelled_deterministic_context_for_test() -> ZqnContext {
        ZqnContext::cancelled_deterministic_for_testing()
    }
}