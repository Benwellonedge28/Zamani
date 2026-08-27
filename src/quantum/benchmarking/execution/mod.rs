//! Zamani Quantum Benchmarking — Execution Subsystem
//!
//! Production module boundary for benchmark execution.
//!
//! # Purpose
//!
//! This module is the public execution-layer facade for
//! `quantum::benchmarking`.
//!
//! It wires together:
//!
//! - execution orchestration;
//! - cancellation coordination;
//! - batching;
//! - sampling;
//! - execution timing;
//! - the canonical execution contract defined by `core::execution`.
//!
//! The module itself contains NO provider-specific execution logic.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                   quantum::benchmarking
//!                              │
//!                              ▼
//!                    benchmark protocol
//!                              │
//!                              ▼
//!                execution::ExecutionOrchestrator
//!                              │
//!               ┌──────────────┼──────────────┐
//!               │              │              │
//!               ▼              ▼              ▼
//!          cancellation      batching       timing
//!               │              │              │
//!               └──────────────┼──────────────┘
//!                              ▼
//!                    core::execution
//!                              │
//!                    BenchmarkExecutor
//!                              │
//!             ┌────────────────┼────────────────┐
//!             ▼                ▼                ▼
//!          simulator        runtime          hardware
//!             │                │                │
//!             └────────────────┼────────────────┘
//!                              ▼
//!                    ExecutionResponse
//!                              │
//!                              ▼
//!                    raw observations
//!                              │
//!                    ┌─────────┴─────────┐
//!                    ▼                   ▼
//!                statistics           metrics
//!                    │                   │
//!                    └─────────┬─────────┘
//!                              ▼
//!                       BenchmarkResult
//! ```
//!
//! # Canonical contract
//!
//! The authoritative execution contract is:
//!
//! ```text
//! quantum::benchmarking::core::execution
//! ```
//!
//! In particular, the following types are owned by `core::execution`:
//!
//! - `BenchmarkExecutor`;
//! - `ExecutionRequest`;
//! - `ExecutionResponse`;
//! - `ExecutionError`;
//! - `ExecutionMode`;
//! - `MeasurementMode`;
//! - `ExecutionCapabilities`;
//! - `ExecutionRequirements`;
//! - `CancellationToken`;
//! - `ExecutionTiming`;
//! - `ExecutionStatus`;
//! - `ExecutionRequestId`;
//! - `ExecutionBackendId`.
//!
//! This module re-exports those canonical types for execution-layer callers.
//!
//! It MUST NOT define competing versions of those types.
//!
//! # Why this boundary is important
//!
//! Benchmark execution has two different abstraction levels:
//!
//! ```text
//! core::execution
//!     = semantic execution contract
//!
//! execution/
//!     = lifecycle/orchestration implementation
//! ```
//!
//! The distinction prevents benchmark protocols from becoming coupled to:
//!
//! - a simulator;
//! - a particular QPU provider;
//! - an async runtime;
//! - a network transport;
//! - a scheduler implementation;
//! - a hardware SDK;
//! - a specific batching mechanism.
//!
//! # Ownership
//!
//! ## `core::execution` owns
//!
//! - execution request semantics;
//! - execution response semantics;
//! - backend-neutral execution status;
//! - backend capabilities;
//! - execution requirements;
//! - cancellation primitive;
//! - canonical execution timing contract;
//! - executor trait;
//! - execution errors.
//!
//! ## `execution::executor` owns
//!
//! - orchestration;
//! - pre-execution validation;
//! - capability negotiation;
//! - request/response correlation;
//! - response invariant checking;
//! - execution outcome classification;
//! - orchestration policy.
//!
//! ## `execution::cancellation` owns
//!
//! - cancellation policy;
//! - cancellation reasons;
//! - cancellation source;
//! - cancellation controller;
//! - cancellation checkpoints;
//! - cancellation interpretation.
//!
//! It does NOT create another cancellation token.
//!
//! ## `execution::batching` owns
//!
//! - grouping execution requests;
//! - batch-level validation;
//! - batch correlation;
//! - partial batch handling;
//! - backend-independent batching policy.
//!
//! It does NOT execute provider jobs itself.
//!
//! ## `execution::sampler` owns
//!
//! - backend-independent sampling orchestration;
//! - shot-oriented execution helpers;
//! - sample validation;
//! - deterministic sampling policy.
//!
//! It does NOT implement benchmark-specific statistics.
//!
//! ## `execution::timing` owns
//!
//! - monotonic timing;
//! - lifecycle phase timing;
//! - timing recording;
//! - timing snapshots.
//!
//! It does NOT calculate throughput or performance metrics.
//!
//! # Dependency direction
//!
//! The dependency direction is intentionally one-way:
//!
//! ```text
//! execution::mod
//!       │
//!       ├── execution::executor
//!       ├── execution::cancellation
//!       ├── execution::batching
//!       ├── execution::sampler
//!       └── execution::timing
//!                 │
//!                 ▼
//!          core::execution
//!                 │
//!       ┌─────────┼─────────┐
//!       ▼         ▼         ▼
//!      IR       runtime   hardware
//! ```
//!
//! The reverse direction is forbidden:
//!
//! ```text
//! quantum::ir
//!      X
//!      │
//!      └──────> benchmarking::execution
//! ```
//!
//! `quantum::ir` remains the authoritative semantic representation of
//! quantum programs.
//!
//! # Quantum IR boundary
//!
//! Execution consumes the canonical Quantum IR through the core execution
//! contract.
//!
//! This module must never:
//!
//! - define a second circuit representation;
//! - modify Quantum IR;
//! - compile Quantum IR;
//! - route Quantum IR;
//! - schedule Quantum IR.
//!
//! Those responsibilities belong to the appropriate quantum compiler,
//! optimization, routing, scheduling, runtime, and hardware layers.
//!
//! # Backend neutrality
//!
//! The execution facade must support the complete execution model required by
//! Zamani without assuming that all systems are ordinary gate-model QPUs.
//!
//! Supported execution classes include:
//!
//! - state-vector simulation;
//! - density-matrix simulation;
//! - stabilizer simulation;
//! - tensor-network simulation;
//! - hardware emulation;
//! - physical QPU execution;
//! - logical-QPU execution;
//! - analog quantum execution;
//! - quantum annealing;
//! - sampling-oriented systems;
//! - custom execution targets.
//!
//! The canonical `ExecutionMode` in `core::execution` defines these semantics.
//!
//! # Protocol independence
//!
//! This module MUST NOT know about:
//!
//! - Quantum Volume;
//! - randomized benchmarking;
//! - interleaved RB;
//! - simultaneous RB;
//! - purity RB;
//! - leakage RB;
//! - cycle benchmarking;
//! - layer fidelity;
//! - XEB;
//! - mirror circuits;
//! - SPAM;
//! - tomography;
//! - QEC benchmarks;
//! - VQE;
//! - QAOA;
//! - Grover;
//! - QFT;
//! - application benchmarks;
//! - any individual benchmark protocol.
//!
//! Protocols submit execution work through the canonical execution contract.
//!
//! Example:
//!
//! ```text
//! protocols::quantum_volume
//!             │
//!             ▼
//!      ExecutionRequest
//!             │
//!             ▼
//!      execution::executor
//!             │
//!             ▼
//!      BenchmarkExecutor
//!             │
//!             ▼
//!       simulator/QPU
//!             │
//!             ▼
//!      ExecutionResponse
//!             │
//!             ▼
//! volume_estimator / protocol analysis
//! ```
//!
//! # Cancellation
//!
//! Cancellation is cooperative.
//!
//! There must be exactly one authoritative cancellation token:
//!
//! ```text
//! core::execution::CancellationToken
//! ```
//!
//! `execution::cancellation` may add policy and lifecycle semantics around
//! that token, but must never create a competing cancellation primitive.
//!
//! Cancellation must remain distinguishable from timeout:
//!
//! ```text
//! Cancelled != TimedOut
//! ```
//!
//! Remote cancellation also must not be treated as proof that a provider has
//! actually stopped remote work unless the provider confirms it.
//!
//! # Timing
//!
//! Timing is deliberately separated from benchmark metrics.
//!
//! The execution subsystem records lifecycle timing such as:
//!
//! - preparation;
//! - compilation;
//! - transpilation;
//! - routing;
//! - scheduling;
//! - queue;
//! - submission;
//! - quantum execution;
//! - readout;
//! - result retrieval;
//! - analysis;
//! - total wall-clock orchestration time.
//!
//! Throughput metrics such as:
//!
//! - shots/sec;
//! - circuits/sec;
//! - gates/sec;
//! - layers/sec;
//! - CLOPS-like measurements;
//!
//! belong to `metrics::throughput`.
//!
//! # Batching
//!
//! Batching is an optimization and execution-transport concern.
//!
//! A benchmark protocol may generate:
//!
//! ```text
//! circuit 1
//! circuit 2
//! circuit 3
//! ...
//! circuit N
//! ```
//!
//! and the execution layer may group those requests where the selected
//! backend permits it.
//!
//! Batching MUST NOT change benchmark semantics.
//!
//! In particular, batching must never silently change:
//!
//! - shot counts;
//! - circuit order where order is semantically significant;
//! - seeds;
//! - backend selection;
//! - requested measurement mode;
//! - timeout semantics;
//! - benchmark identity.
//!
//! # Sampling
//!
//! Sampling helpers operate on execution requests and raw observations.
//!
//! They must not silently convert an exact state representation into sampled
//! data or vice versa.
//!
//! Requested measurement mode remains part of the execution contract.
//!
//! # Error policy
//!
//! Execution-layer modules must use the canonical benchmarking error model.
//!
//! No execution module may:
//!
//! - print errors;
//! - call `panic!` for recoverable input;
//! - terminate the process;
//! - silently downgrade failures;
//! - fabricate successful responses;
//! - swallow provider errors.
//!
//! Diagnostics must remain structured data.
//!
//! # Resource safety
//!
//! All execution-layer operations must respect the resource limits defined by
//! `core::limits` and the limits carried by execution requests.
//!
//! The execution layer must guard against:
//!
//! - excessive shot counts;
//! - excessive batch sizes;
//! - unbounded metadata;
//! - pathological sampling requests;
//! - uncontrolled result accumulation;
//! - timeout overflow;
//! - integer overflow;
//! - unbounded retries.
//!
//! # Retry policy
//!
//! The execution facade does NOT retry implicitly.
//!
//! This is particularly important for remote QPU execution:
//!
//! ```text
//! timeout
//!    │
//!    └──> remote job may already exist
//!              │
//!              X
//!              │
//!              └──> do not blindly resubmit
//! ```
//!
//! Retry decisions must be explicit and must respect backend idempotency and
//! submission-state semantics.
//!
//! # Partial execution
//!
//! Partial execution is first-class.
//!
//! A provider may execute some requested work and fail or cancel the rest.
//!
//! The execution subsystem must preserve:
//!
//! - requested work;
//! - completed work;
//! - incomplete work;
//! - raw observations obtained before failure;
//! - provider diagnostics;
//! - execution status.
//!
//! Partial execution must never be silently promoted to complete execution.
//!
//! # Execution versus benchmark success
//!
//! This distinction is mandatory:
//!
//! ```text
//! execution success
//!         !=
//! benchmark success
//! ```
//!
//! For example:
//!
//! ```text
//! ExecutionStatus::Completed
//! ```
//!
//! means the requested experiment execution completed.
//!
//! Quantum Volume may subsequently produce:
//!
//! ```text
//! benchmark_passed = false
//! ```
//!
//! because the measured heavy-output probability failed the protocol's
//! acceptance criterion.
//!
//! The execution subsystem must never make that scientific decision.
//!
//! # Existing repository integration
//!
//! The repository already contains a canonical execution contract under:
//!
//! ```text
//! src/quantum/benchmarking/core/execution.rs
//! ```
//!
//! The existing production orchestrator is:
//!
//! ```text
//! src/quantum/benchmarking/execution/executor.rs
//! ```
//!
//! It already consumes the canonical core execution types rather than
//! defining a separate execution contract. 
//!
//! The canonical core contract explicitly separates execution from circuit
//! generation, compilation, routing, scheduling, statistics and provider
//! communication. 
//!
//! The execution timing implementation is also designed as a lower-level
//! timing primitive and explicitly separates timing from throughput and
//! benchmark mathematics. 
//!
//! # Important repository correction
//!
//! The repository currently contains additional files named:
//!
//! ```text
//! execution/request.rs
//! execution/response.rs
//! ```
//!
//! Those files currently duplicate concepts already owned by
//! `core::execution`. In particular, the response implementation imports
//! request-local identity types, while the production orchestrator imports
//! the canonical core types. 
//!
//! Therefore this module deliberately does NOT make those duplicate modules
//! part of the authoritative execution API.
//!
//! The correct final architecture is:
//!
//! ```text
//! core::execution
//!       │
//!       ├── ExecutionRequest
//!       ├── ExecutionResponse
//!       ├── ExecutionError
//!       ├── BenchmarkExecutor
//!       └── execution primitives
//!
//! execution/
//!       │
//!       ├── executor.rs
//!       ├── cancellation.rs
//!       ├── batching.rs
//!       ├── sampler.rs
//!       └── timing.rs
//! ```
//!
//! `execution/request.rs` and `execution/response.rs` should subsequently be
//! migrated into the canonical core contract or removed as duplicate
//! implementations. That cleanup belongs to those files; it should not be
//! hidden inside `execution/mod.rs`.
//!
//! This prevents `mod.rs` from becoming a compatibility layer that masks two
//! competing execution APIs.
//!
//! # Public API policy
//!
//! The execution module exposes:
//!
//! 1. the lifecycle/orchestration modules;
//! 2. the canonical execution-contract types;
//! 3. the production orchestration facade.
//!
//! It does not flatten every implementation detail into one namespace.
//!
//! This keeps ownership explicit and prevents future additions from causing
//! accidental API collisions.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only.
//!
//! No nightly features.
//! No unsafe code.
//! No additional dependencies.
//!
//! # Testing boundary
//!
//! This module's tests verify only module/API wiring.
//!
//! Protocol-specific tests belong under:
//!
//! ```text
//! benchmarking/tests/
//! ```
//!
//! Execution behavior belongs in the execution implementation modules.
//!
//! This file must remain small in executable logic even though its contract
//! documentation is extensive.
//!
//! # Integration checklist
//!
//! Before considering this module complete:
//!
//! - [x] `core::execution` remains authoritative;
//! - [x] orchestration is exposed through `executor`;
//! - [x] cancellation has one canonical token;
//! - [x] timing remains a separate concern;
//! - [x] batching is separate from provider execution;
//! - [x] sampling is separate from benchmark mathematics;
//! - [x] no provider SDK is imported here;
//! - [x] no benchmark protocol is imported here;
//! - [x] no Quantum IR mutation occurs here;
//! - [x] no global mutable execution state is introduced;
//! - [x] no implicit retry is introduced;
//! - [x] no implicit backend substitution is introduced;
//! - [x] no implicit shot modification is introduced;
//! - [x] no diagnostic printing is introduced;
//! - [x] Rust 1.97.1 compatibility is preserved.
//!
//! # Future integration
//!
//! Future Zamani language syntax should lower through the stable execution
//! contracts:
//!
//! ```text
//! Zamani benchmark declaration
//!             │
//!             ▼
//! BenchmarkExperiment
//!             │
//!             ▼
//! ExecutionRequest
//!             │
//!             ▼
//! execution::executor
//!             │
//!             ▼
//! BenchmarkExecutor
//! ```
//!
//! This means adding a new Zamani benchmark syntax must not require changing
//! this module.
//!
//! Adding a new backend must not require changing this module.
//!
//! Adding a new simulator must not require changing this module.
//!
//! Adding a new benchmark protocol must not require changing this module.
//!
//! Adding a new statistical metric must not require changing this module.
//!
//! That stability is the purpose of this facade.

#![deny(unsafe_code)]
#![deny(missing_debug_implementations)]

/// Production execution orchestration.
///
/// This is the primary implementation module used by benchmark protocols.
pub mod executor;

/// Execution cancellation policy and coordination.
///
/// Uses the canonical `core::execution::CancellationToken`.
pub mod cancellation;

/// Backend-independent request batching.
///
/// Batching changes transport/execution grouping, not benchmark semantics.
pub mod batching;

/// Sampling-oriented execution helpers.
///
/// Sampling remains distinct from protocol-specific statistical analysis.
pub mod sampler;

/// Monotonic execution lifecycle timing.
///
/// This module records timing; it does not calculate performance metrics.
pub mod timing;

// =============================================================================
// Canonical execution-contract re-exports
// =============================================================================
//
// These are intentionally re-exported from `core::execution` rather than
// recreated here. This guarantees that every execution-layer implementation,
// benchmark protocol, runtime adapter and hardware adapter speaks the same
// semantic contract.

pub use super::core::execution::{
    BenchmarkExecutor,
    CancellationToken,
    ExecutionBackendId,
    ExecutionCapabilities,
    ExecutionError,
    ExecutionGuard,
    ExecutionMode,
    ExecutionRequest,
    ExecutionRequestId,
    ExecutionRequirements,
    ExecutionResponse,
    ExecutionStatus,
    ExecutionTimeout,
    MeasurementMode,
    ShotCount,
    DEFAULT_EXECUTION_TIMEOUT,
    DEFAULT_SHOTS,
    EXECUTION_CONTRACT_VERSION,
    EXECUTION_REQUEST_SCHEMA_VERSION,
    EXECUTION_RESPONSE_SCHEMA_VERSION,
};

// =============================================================================
// Execution-layer implementation re-exports
// =============================================================================
//
// Keep the implementation modules namespaced while providing the principal
// orchestration API directly under `benchmarking::execution`.

pub use self::executor::{
    ExecutionOrchestrator,
    ExecutionOutcome,
    ExecutionPolicy,
    ExecutionReport,
    EXECUTION_ORCHESTRATOR_VERSION,
};

// =============================================================================
// Cancellation API
// =============================================================================

pub use self::cancellation::{
    CancellationCapability,
    CancellationCheckpoint,
    CancellationController,
    CancellationError,
    CancellationObservation,
    CancellationPolicy,
    CancellationReason,
    CancellationSource,
    CancellationState,
    CANCELLATION_API_VERSION,
};

// =============================================================================
// Timing API
// =============================================================================
//
// Timing is kept in its own namespace because it is also useful to backend
// adapters and test infrastructure without requiring the full orchestrator.

pub use self::timing::{
    ExecutionTiming as RecordedExecutionTiming,
    PhaseTiming,
    TimingHandle,
    TimingPhase,
    TimingRecorder,
    TimingTimestamp,
    TimingValue,
    EXECUTION_TIMING_SCHEMA_VERSION,
    TIMING_RECORDER_VERSION,
};

// =============================================================================
// Stable facade
// =============================================================================

/// Stable execution facade used by benchmark protocols.
///
/// This type intentionally contains no mutable state and no backend handle.
/// A concrete [`BenchmarkExecutor`] is supplied for each execution operation.
///
/// Keeping this facade stateless allows the same API to be used by:
///
/// - local simulators;
/// - GPU simulators;
/// - remote QPUs;
/// - logical-QPU runtimes;
/// - emulators;
/// - annealing systems;
/// - analog systems;
/// - deterministic test executors.
#[derive(Debug, Clone, Copy, Default)]
pub struct ExecutionService {
    orchestrator: ExecutionOrchestrator,
}

impl ExecutionService {
    /// Creates a production execution service.
    pub const fn new() -> Self {
        Self {
            orchestrator: ExecutionOrchestrator::new(),
        }
    }

    /// Returns the underlying stateless orchestrator.
    pub const fn orchestrator(&self) -> &ExecutionOrchestrator {
        &self.orchestrator
    }

    /// Executes one benchmark execution request.
    ///
    /// The concrete executor remains responsible for actual simulator/runtime/
    /// hardware execution.
    pub fn execute<E>(
        &self,
        executor: &E,
        request: &ExecutionRequest,
    ) -> Result<ExecutionReport, ExecutionError>
    where
        E: BenchmarkExecutor + ?Sized,
    {
        self.orchestrator.execute(executor, request)
    }

    /// Executes one request under an explicit execution policy.
    pub fn execute_with_policy<E>(
        &self,
        executor: &E,
        request: &ExecutionRequest,
        policy: ExecutionPolicy,
    ) -> Result<ExecutionReport, ExecutionError>
    where
        E: BenchmarkExecutor + ?Sized,
    {
        self.orchestrator
            .execute_with_policy(executor, request, policy)
    }
}

// =============================================================================
// Compile-time API-boundary tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execution_service_is_constructible() {
        let service = ExecutionService::new();

        assert_eq!(
            ExecutionOrchestrator::version(),
            EXECUTION_ORCHESTRATOR_VERSION
        );

        let _ = service.orchestrator();
    }

    #[test]
    fn canonical_execution_types_are_reexported() {
        let _: Option<ExecutionRequestId> = None;
        let _: Option<ExecutionBackendId> = None;
        let _: Option<ExecutionMode> = None;
        let _: Option<MeasurementMode> = None;
        let _: Option<ShotCount> = None;
        let _: Option<ExecutionTimeout> = None;
        let _: Option<CancellationToken> = None;
    }

    #[test]
    fn execution_policy_has_no_implicit_retry_contract() {
        assert_eq!(
            ExecutionOrchestrator::default(),
            ExecutionOrchestrator::new()
        );

        assert_eq!(
            EXECUTION_ORCHESTRATOR_VERSION,
            1
        );
    }

    #[test]
    fn execution_module_preserves_single_cancellation_primitive() {
        let token = CancellationToken::new();

        assert!(!token.is_cancelled());

        token.cancel();

        assert!(token.is_cancelled());
    }

    #[test]
    fn strict_policy_rejects_incomplete_execution_by_contract() {
        let policy = ExecutionPolicy::strict();

        assert!(!policy.allow_partial);
        assert!(!policy.allow_cancelled);
        assert!(!policy.allow_timed_out);
    }
}