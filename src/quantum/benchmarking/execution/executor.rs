//! Zamani Quantum Benchmarking — Production Execution Orchestrator
//!
//! This module provides the production orchestration layer for benchmark
//! execution.
//!
//! # Architectural responsibility
//!
//! `core::execution` defines the canonical execution contract:
//!
//! ```text
//! ExecutionRequest
//!       │
//!       ▼
//! BenchmarkExecutor
//!       │
//!       ▼
//! ExecutionResponse
//! ```
//!
//! This module adds the lifecycle/orchestration layer around that contract:
//!
//! ```text
//! benchmark protocol
//!       │
//!       ▼
//! ExecutionOrchestrator
//!       │
//!       ├── validate request
//!       ├── validate executor/backend identity
//!       ├── validate capabilities
//!       ├── check cancellation
//!       ├── delegate execution
//!       ├── validate response correlation
//!       ├── validate response invariants
//!       ├── classify terminal state
//!       └── return raw ExecutionResponse
//! ```
//!
//! # What this module does NOT own
//!
//! This module deliberately does not:
//!
//! - generate circuits;
//! - compile circuits;
//! - transpile circuits;
//! - route circuits;
//! - schedule circuits;
//! - implement hardware-provider SDKs;
//! - implement simulators;
//! - calculate benchmark metrics;
//! - calculate Quantum Volume;
//! - calculate randomized-benchmarking statistics;
//! - calculate XEB;
//! - calculate fidelity;
//! - perform statistical fitting;
//! - modify Quantum IR;
//! - silently retry failed execution;
//! - silently change the requested shot count;
//! - silently change a requested seed;
//! - silently substitute a backend;
//! - print diagnostics;
//! - perform network I/O itself.
//!
//! Those responsibilities belong to their owning subsystems.
//!
//! # Dependency direction
//!
//! ```text
//! benchmarking::protocols
//!          │
//!          ▼
//! benchmarking::execution::executor
//!          │
//!          ▼
//! benchmarking::core::execution
//!          │
//!          ▼
//! concrete BenchmarkExecutor
//!          │
//!          ├── simulator
//!          ├── runtime
//!          └── hardware adapter
//! ```
//!
//! The canonical Quantum IR remains below the execution contract:
//!
//! ```text
//! quantum::ir
//!      ▲
//!      │
//! ExecutionRequest
//! ```
//!
//! Benchmarking consumes Quantum IR; it does not redefine it.
//!
//! # Production invariants
//!
//! This module enforces the following invariants:
//!
//! 1. The request is validated before provider execution.
//! 2. Executor/backend identity must match the request.
//! 3. Execution mode must match the selected executor.
//! 4. Cancellation is checked before provider execution.
//! 5. Capability requirements are checked before provider execution.
//! 6. The provider receives the exact validated request.
//! 7. No implicit retry occurs.
//! 8. No implicit shot modification occurs.
//! 9. No implicit seed modification occurs.
//! 10. No implicit backend substitution occurs.
//! 11. The response must correlate to the original request.
//! 12. Response shot counts must be valid.
//! 13. Provider metadata is preserved.
//! 14. Raw observations are preserved.
//! 15. Partial execution remains explicitly partial.
//! 16. Cancellation remains explicitly observable.
//! 17. Timeout remains explicitly observable.
//! 18. The execution layer never fabricates benchmark metrics.
//! 19. The execution layer never mutates Quantum IR.
//! 20. The execution layer contains no process-global mutable state.
//! 21. The execution layer requires no async runtime.
//! 22. The execution layer requires no unsafe code.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust only
//!
//! No additional dependency is required by this module.

use std::fmt;
use std::time::{Duration, Instant};

use super::super::core::execution::{
    BenchmarkExecutor,
    CancellationToken,
    ExecutionCapabilities,
    ExecutionError,
    ExecutionGuard,
    ExecutionMode,
    ExecutionRequest,
    ExecutionRequestId,
    ExecutionRequirements,
    ExecutionResponse,
    ExecutionStatus,
    ExecutionTiming,
    ExecutorMetadata,
};

// =============================================================================
// Constants
// =============================================================================

/// Stable orchestration API version.
///
/// This changes only when the semantics of this orchestration layer change.
pub const EXECUTION_ORCHESTRATOR_VERSION: u32 = 1;

/// Default maximum number of sequential execution attempts.
///
/// This is intentionally one.
///
/// The execution orchestrator never retries implicitly. A caller that wants
/// retries must explicitly construct a retry policy and make that policy part
/// of the benchmark experiment.
pub const DEFAULT_MAX_ATTEMPTS: usize = 1;

// =============================================================================
// Execution policy
// =============================================================================

/// Policy controlling what the orchestration layer accepts from an executor.
///
/// This is deliberately separate from backend capabilities. Capabilities say
/// what a backend can do; policy says what a benchmark invocation is willing
/// to accept.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExecutionPolicy {
    /// Whether a partial execution response may be returned as success of the
    /// orchestration call.
    ///
    /// This does NOT convert the response status from `Partial` to
    /// `Completed`. It only controls whether the orchestrator returns the
    /// response or rejects it.
    pub allow_partial: bool,

    /// Whether a cancelled execution response may be returned.
    pub allow_cancelled: bool,

    /// Whether a timed-out execution response may be returned.
    pub allow_timed_out: bool,

    /// Whether an empty-observation completed response is allowed.
    ///
    /// This is useful for execution modes whose useful result is represented
    /// outside the standard observation vector, but ordinary sampled
    /// benchmarks should normally leave this disabled.
    pub allow_empty_observations: bool,
}

impl Default for ExecutionPolicy {
    fn default() -> Self {
        Self {
            allow_partial: true,
            allow_cancelled: true,
            allow_timed_out: false,
            allow_empty_observations: false,
        }
    }
}

impl ExecutionPolicy {
    /// Policy suitable for ordinary benchmark execution.
    pub fn benchmark_default() -> Self {
        Self::default()
    }

    /// Strict policy requiring complete execution.
    pub fn strict() -> Self {
        Self {
            allow_partial: false,
            allow_cancelled: false,
            allow_timed_out: false,
            allow_empty_observations: false,
        }
    }

    /// Policy allowing incomplete execution so that the caller can inspect
    /// partial raw observations.
    pub fn analysis_friendly() -> Self {
        Self {
            allow_partial: true,
            allow_cancelled: true,
            allow_timed_out: true,
            allow_empty_observations: true,
        }
    }

    fn validate_response(
        &self,
        response: &ExecutionResponse,
    ) -> Result<(), ExecutionError> {
        match response.status {
            ExecutionStatus::Completed => {}

            ExecutionStatus::Partial if self.allow_partial => {}

            ExecutionStatus::Cancelled if self.allow_cancelled => {}

            ExecutionStatus::TimedOut if self.allow_timed_out => {}

            ExecutionStatus::Accepted | ExecutionStatus::Running => {
                return Err(ExecutionError::InvalidResponseStatus {
                    status: response.status,
                });
            }

            ExecutionStatus::Partial => {
                return Err(ExecutionError::PartialExecutionRejected);
            }

            ExecutionStatus::Cancelled => {
                return Err(ExecutionError::CancellationRejected);
            }

            ExecutionStatus::TimedOut => {
                return Err(ExecutionError::TimeoutRejected);
            }

            ExecutionStatus::Failed => {
                return Err(ExecutionError::ProviderExecutionFailed);
            }
        }

        if !self.allow_empty_observations
            && response.status.may_contain_observations()
            && response.observations.is_empty()
        {
            return Err(ExecutionError::EmptyObservationSet);
        }

        Ok(())
    }
}

// =============================================================================
// Execution outcome
// =============================================================================

/// Classification of an execution after provider completion.
///
/// This is intentionally not a benchmark result.
///
/// It describes execution lifecycle state only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExecutionOutcome {
    /// All requested shots completed successfully.
    Completed,

    /// Some but not all requested work completed.
    Partial,

    /// Execution was cancelled.
    Cancelled,

    /// Execution exceeded its configured timeout.
    TimedOut,
}

impl ExecutionOutcome {
    /// Creates an outcome from the canonical execution status.
    pub fn from_status(
        status: ExecutionStatus,
    ) -> Result<Self, ExecutionError> {
        match status {
            ExecutionStatus::Completed => Ok(Self::Completed),
            ExecutionStatus::Partial => Ok(Self::Partial),
            ExecutionStatus::Cancelled => Ok(Self::Cancelled),
            ExecutionStatus::TimedOut => Ok(Self::TimedOut),

            ExecutionStatus::Accepted | ExecutionStatus::Running => {
                Err(ExecutionError::InvalidResponseStatus { status })
            }

            ExecutionStatus::Failed => {
                Err(ExecutionError::ProviderExecutionFailed)
            }
        }
    }

    /// Returns whether the requested work completed fully.
    pub fn is_complete(self) -> bool {
        matches!(self, Self::Completed)
    }

    /// Returns whether useful partial raw observations may exist.
    pub fn may_contain_observations(self) -> bool {
        matches!(self, Self::Completed | Self::Partial)
    }
}

// =============================================================================
// Execution report
// =============================================================================

/// Production execution report.
///
/// This is the orchestration-layer result.
///
/// It intentionally contains the original `ExecutionResponse` so that raw
/// observations and provider metadata remain available for independent
/// statistical analysis.
///
/// No benchmark metric is stored here.
#[derive(Debug, Clone)]
pub struct ExecutionReport {
    /// Orchestration API version.
    pub orchestrator_version: u32,

    /// Request identity.
    pub request_id: ExecutionRequestId,

    /// Backend identity.
    pub backend_id: String,

    /// Execution mode.
    pub execution_mode: ExecutionMode,

    /// Classified execution outcome.
    pub outcome: ExecutionOutcome,

    /// Number of requested shots.
    pub requested_shots: usize,

    /// Number of completed shots.
    pub completed_shots: usize,

    /// Provider response.
    ///
    /// Raw observations are intentionally preserved here.
    pub response: ExecutionResponse,

    /// Time observed by the orchestration layer.
    ///
    /// This is separate from provider-reported timing in
    /// `response.timing.total_time`.
    pub orchestration_time: Duration,
}

impl ExecutionReport {
    /// Returns the number of incomplete shots.
    pub fn incomplete_shots(&self) -> usize {
        self.requested_shots
            .saturating_sub(self.completed_shots)
    }

    /// Returns the completion ratio.
    ///
    /// Returns `None` when the request had no shots. A valid
    /// `ExecutionRequest` always has at least one shot, but the method remains
    /// defensive because this is a public result type.
    pub fn completion_ratio(&self) -> Option<f64> {
        if self.requested_shots == 0 {
            return None;
        }

        Some(
            self.completed_shots as f64
                / self.requested_shots as f64,
        )
    }

    /// Returns whether the execution completed all requested shots.
    pub fn is_complete(&self) -> bool {
        self.outcome.is_complete()
    }

    /// Returns whether the response may contain usable raw observations.
    pub fn may_contain_observations(&self) -> bool {
        self.outcome.may_contain_observations()
    }
}

// =============================================================================
// Capability negotiation
// =============================================================================

/// Capability negotiation request.
///
/// This type makes protocol/backend negotiation explicit before provider
/// execution.
///
/// It deliberately contains no provider-specific SDK concepts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapabilityCheck {
    /// Requirements requested by the benchmark.
    pub requirements: ExecutionRequirements,

    /// Measurement mode requested by the execution request.
    pub measurement_mode: super::super::core::execution::MeasurementMode,
}

impl CapabilityCheck {
    /// Creates a capability check from an execution request.
    pub fn from_request(
        request: &ExecutionRequest,
        requirements: ExecutionRequirements,
    ) -> Self {
        Self {
            requirements,
            measurement_mode: request.measurement_mode,
        }
    }

    /// Validates requirements against executor capabilities.
    pub fn validate(
        &self,
        capabilities: &ExecutionCapabilities,
    ) -> Result<(), ExecutionError> {
        self.requirements.validate(capabilities)?;

        if !capabilities
            .supported_measurement_modes
            .contains(&self.measurement_mode)
        {
            return Err(ExecutionError::UnsupportedMeasurementMode {
                mode: self.measurement_mode,
            });
        }

        Ok(())
    }
}

// =============================================================================
// Execution orchestrator
// =============================================================================

/// Production benchmark execution orchestrator.
///
/// `ExecutionOrchestrator` is intentionally stateless.
///
/// It does not own a backend, queue, worker thread, runtime, network client,
/// or mutable execution registry.
///
/// A concrete `BenchmarkExecutor` is supplied for each execution operation.
///
/// This makes the type:
///
/// - deterministic;
/// - testable;
/// - safe to reuse;
/// - independent of an async runtime;
/// - independent of provider SDKs;
/// - suitable for local simulators and remote hardware alike.
#[derive(Debug, Clone, Copy, Default)]
pub struct ExecutionOrchestrator;

impl ExecutionOrchestrator {
    /// Creates a production execution orchestrator.
    pub const fn new() -> Self {
        Self
    }

    /// Returns the orchestration API version.
    pub const fn version() -> u32 {
        EXECUTION_ORCHESTRATOR_VERSION
    }

    // -------------------------------------------------------------------------
    // Basic execution
    // -------------------------------------------------------------------------

    /// Executes one validated request using the supplied executor.
    ///
    /// This is the primary entry point for benchmark protocols.
    ///
    /// The executor remains responsible for actual provider execution.
    ///
    /// The orchestrator is responsible for:
    ///
    /// - request validation;
    /// - executor validation;
    /// - cancellation pre-check;
    /// - delegation;
    /// - response correlation;
    /// - response validation;
    /// - outcome classification;
    /// - policy validation.
    pub fn execute<E>(
        &self,
        executor: &E,
        request: &ExecutionRequest,
    ) -> Result<ExecutionReport, ExecutionError>
    where
        E: BenchmarkExecutor + ?Sized,
    {
        self.execute_with_policy(
            executor,
            request,
            ExecutionPolicy::benchmark_default(),
        )
    }

    /// Executes one request using an explicit execution policy.
    pub fn execute_with_policy<E>(
        &self,
        executor: &E,
        request: &ExecutionRequest,
        policy: ExecutionPolicy,
    ) -> Result<ExecutionReport, ExecutionError>
    where
        E: BenchmarkExecutor + ?Sized,
    {
        let started = Instant::now();

        request.validate()?;

        executor.validate(request)?;

        validate_request_against_executor(
            request,
            executor.metadata(),
        )?;

        request.cancellation.check()?;

        let response = executor.execute(request)?;

        let orchestration_time = started.elapsed();

        response.validate_against(request)?;

        validate_response_invariants(
            request,
            &response,
        )?;

        policy.validate_response(&response)?;

        let outcome =
            ExecutionOutcome::from_status(response.status)?;

        Ok(ExecutionReport {
            orchestrator_version: EXECUTION_ORCHESTRATOR_VERSION,
            request_id: request.request_id.clone(),
            backend_id: request.backend_id.as_str().to_owned(),
            execution_mode: request.execution_mode,
            outcome,
            requested_shots: request.shots.get(),
            completed_shots: response.completed_shots,
            response,
            orchestration_time,
        })
    }

    // -------------------------------------------------------------------------
    // Capability-aware execution
    // -------------------------------------------------------------------------

    /// Executes one request after explicit capability negotiation.
    ///
    /// Benchmark protocols should normally use this method because it prevents
    /// unsupported work from reaching the backend.
    pub fn execute_with_requirements<E>(
        &self,
        executor: &E,
        request: &ExecutionRequest,
        requirements: ExecutionRequirements,
    ) -> Result<ExecutionReport, ExecutionError>
    where
        E: BenchmarkExecutor + ?Sized,
    {
        self.execute_with_requirements_and_policy(
            executor,
            request,
            requirements,
            ExecutionPolicy::benchmark_default(),
        )
    }

    /// Executes with explicit capability requirements and response policy.
    pub fn execute_with_requirements_and_policy<E>(
        &self,
        executor: &E,
        request: &ExecutionRequest,
        requirements: ExecutionRequirements,
        policy: ExecutionPolicy,
    ) -> Result<ExecutionReport, ExecutionError>
    where
        E: BenchmarkExecutor + ?Sized,
    {
        request.validate()?;

        executor.validate(request)?;

        validate_request_against_executor(
            request,
            executor.metadata(),
        )?;

        let capability_check =
            CapabilityCheck::from_request(request, requirements);

        capability_check.validate(
            &executor.metadata().capabilities,
        )?;

        self.execute_with_policy(
            executor,
            request,
            policy,
        )
    }

    // -------------------------------------------------------------------------
    // Cancellation
    // -------------------------------------------------------------------------

    /// Requests cancellation through the executor.
    ///
    /// The request's cancellation token is also checked by concrete
    /// implementations as appropriate.
    pub fn cancel<E>(
        &self,
        executor: &E,
        request: &ExecutionRequest,
    ) -> Result<(), ExecutionError>
    where
        E: BenchmarkExecutor + ?Sized,
    {
        request.validate()?;

        executor.validate(request)?;

        executor.cancel(&request.request_id)
    }

    /// Returns the request cancellation token.
    ///
    /// This convenience method does not mutate execution state.
    pub fn cancellation_token(
        &self,
        request: &ExecutionRequest,
    ) -> Result<CancellationToken, ExecutionError> {
        request.validate()?;

        Ok(request.cancellation.clone())
    }

    // -------------------------------------------------------------------------
    // Validation only
    // -------------------------------------------------------------------------

    /// Validates a request against an executor without executing it.
    ///
    /// This is useful for:
    ///
    /// - benchmark planning;
    /// - capability discovery;
    /// - CI validation;
    /// - dry-run commands;
    /// - frontend diagnostics;
    /// - hardware compatibility checks.
    pub fn validate<E>(
        &self,
        executor: &E,
        request: &ExecutionRequest,
    ) -> Result<(), ExecutionError>
    where
        E: BenchmarkExecutor + ?Sized,
    {
        request.validate()?;

        executor.validate(request)?;

        validate_request_against_executor(
            request,
            executor.metadata(),
        )
    }

    /// Validates both a request and its capability requirements without
    /// executing it.
    pub fn validate_with_requirements<E>(
        &self,
        executor: &E,
        request: &ExecutionRequest,
        requirements: ExecutionRequirements,
    ) -> Result<(), ExecutionError>
    where
        E: BenchmarkExecutor + ?Sized,
    {
        self.validate(executor, request)?;

        CapabilityCheck::from_request(
            request,
            requirements,
        )
        .validate(&executor.metadata().capabilities)
    }

    // -------------------------------------------------------------------------
    // Response validation
    // -------------------------------------------------------------------------

    /// Validates a provider response against its originating request.
    ///
    /// This method is public because stored/external execution responses may
    /// need to be validated before they are handed to benchmark analysis.
    pub fn validate_response(
        &self,
        request: &ExecutionRequest,
        response: &ExecutionResponse,
    ) -> Result<(), ExecutionError> {
        request.validate()?;

        response.validate_against(request)?;

        validate_response_invariants(
            request,
            response,
        )
    }

    /// Classifies an already validated execution response.
    pub fn classify_response(
        &self,
        request: &ExecutionRequest,
        response: &ExecutionResponse,
    ) -> Result<ExecutionOutcome, ExecutionError> {
        self.validate_response(request, response)?;

        ExecutionOutcome::from_status(response.status)
    }

    // -------------------------------------------------------------------------
    // Timing utilities
    // -------------------------------------------------------------------------

    /// Creates an execution guard for a provider implementation.
    ///
    /// Concrete `BenchmarkExecutor` implementations can use this helper when
    /// implementing long-running execution loops.
    pub fn guard(
        &self,
        request: &ExecutionRequest,
    ) -> Result<ExecutionGuard, ExecutionError> {
        request.validate()?;

        Ok(ExecutionGuard::start(
            request.timeout,
            request.cancellation.clone(),
        ))
    }

    /// Creates timing information for a local execution that has just
    /// completed.
    pub fn completed_timing(
        elapsed: Duration,
    ) -> ExecutionTiming {
        ExecutionTiming::completed(elapsed)
    }
}

// =============================================================================
// Request/executor validation
// =============================================================================

/// Validates request identity and execution-mode compatibility against an
/// executor.
///
/// This is intentionally separate from `BenchmarkExecutor::validate` so that
/// the orchestration layer has one final defensive validation boundary.
fn validate_request_against_executor(
    request: &ExecutionRequest,
    metadata: &ExecutorMetadata,
) -> Result<(), ExecutionError> {
    if request.backend_id != metadata.backend_id {
        return Err(ExecutionError::BackendMismatch {
            expected: metadata.backend_id.clone(),
            actual: request.backend_id.clone(),
        });
    }

    if request.execution_mode != metadata.execution_mode {
        return Err(ExecutionError::ExecutionModeMismatch {
            expected: metadata.execution_mode,
            actual: request.execution_mode,
        });
    }

    Ok(())
}

// =============================================================================
// Response validation
// =============================================================================

/// Performs invariant validation that is stronger than simple request/response
/// correlation.
///
/// This function never derives benchmark metrics.
fn validate_response_invariants(
    request: &ExecutionRequest,
    response: &ExecutionResponse,
) -> Result<(), ExecutionError> {
    // -------------------------------------------------------------------------
    // Request identity
    // -------------------------------------------------------------------------

    if response.request_id != request.request_id {
        return Err(ExecutionError::RequestResponseMismatch {
            expected: request.request_id.clone(),
            actual: response.request_id.clone(),
        });
    }

    // -------------------------------------------------------------------------
    // Backend identity
    // -------------------------------------------------------------------------

    if response.backend_id != request.backend_id {
        return Err(ExecutionError::BackendResponseMismatch {
            expected: request.backend_id.clone(),
            actual: response.backend_id.clone(),
        });
    }

    // -------------------------------------------------------------------------
    // Execution mode
    // -------------------------------------------------------------------------

    if response.execution_mode != request.execution_mode {
        return Err(ExecutionError::ResponseExecutionModeMismatch {
            expected: request.execution_mode,
            actual: response.execution_mode,
        });
    }

    // -------------------------------------------------------------------------
    // Shot invariants
    // -------------------------------------------------------------------------

    if response.requested_shots != request.shots.get() {
        return Err(ExecutionError::RequestedShotMismatch {
            expected: request.shots.get(),
            actual: response.requested_shots,
        });
    }

    if response.completed_shots > response.requested_shots {
        return Err(ExecutionError::InvalidCompletedShotCount {
            requested: response.requested_shots,
            completed: response.completed_shots,
        });
    }

    // -------------------------------------------------------------------------
    // Status/shot consistency
    // -------------------------------------------------------------------------

    match response.status {
        ExecutionStatus::Completed => {
            if response.completed_shots != response.requested_shots {
                return Err(
                    ExecutionError::CompletedStatusWithIncompleteShots {
                        requested: response.requested_shots,
                        completed: response.completed_shots,
                    },
                );
            }
        }

        ExecutionStatus::Partial => {
            if response.completed_shots
                >= response.requested_shots
            {
                return Err(
                    ExecutionError::PartialStatusWithCompleteShots {
                        requested: response.requested_shots,
                        completed: response.completed_shots,
                    },
                );
            }
        }

        ExecutionStatus::Cancelled
        | ExecutionStatus::TimedOut
        | ExecutionStatus::Failed
        | ExecutionStatus::Accepted
        | ExecutionStatus::Running => {}
    }

    // -------------------------------------------------------------------------
    // Timing invariants
    // -------------------------------------------------------------------------

    if response.timing.total_time.is_zero()
        && response.status == ExecutionStatus::Completed
    {
        return Err(ExecutionError::InvalidExecutionTiming);
    }

    // -------------------------------------------------------------------------
    // Observation invariants
    // -------------------------------------------------------------------------

    validate_observations(
        request,
        response,
    )?;

    Ok(())
}

/// Validates raw observations without interpreting them as benchmark metrics.
fn validate_observations(
    request: &ExecutionRequest,
    response: &ExecutionResponse,
) -> Result<(), ExecutionError> {
    for observation in &response.observations {
        match observation {
            super::super::core::execution::ExecutionObservation::Counts(
                counts,
            ) => {
                let total = counts.values().try_fold(
                    0usize,
                    |acc, value| acc.checked_add(*value),
                );

                let total = total.ok_or(
                    ExecutionError::ObservationCountOverflow,
                )?;

                if total > response.completed_shots {
                    return Err(
                        ExecutionError::ObservationShotCountExceeded {
                            completed: response.completed_shots,
                            observed: total,
                        },
                    );
                }
            }

            super::super::core::execution::ExecutionObservation::Probabilities(
                probabilities,
            ) => {
                let mut total = 0.0f64;

                for probability in probabilities.values() {
                    if !probability.is_finite()
                        || *probability < 0.0
                        || *probability > 1.0
                    {
                        return Err(
                            ExecutionError::InvalidObservationProbability,
                        );
                    }

                    total += *probability;
                }

                if !total.is_finite() {
                    return Err(
                        ExecutionError::InvalidObservationProbability,
                    );
                }

                // We intentionally do not require exactly 1.0 here.
                //
                // Some backends may return a filtered, post-selected or
                // partial probability distribution. Protocol analysis decides
                // whether normalization is required.
            }

            super::super::core::execution::ExecutionObservation::ExpectationValues(
                values,
            ) => {
                for value in values.values() {
                    if !value.is_finite() {
                        return Err(
                            ExecutionError::InvalidObservationValue,
                        );
                    }
                }
            }

            super::super::core::execution::ExecutionObservation::StateVector(
                amplitudes,
            ) => {
                for (real, imaginary) in amplitudes {
                    if !real.is_finite()
                        || !imaginary.is_finite()
                    {
                        return Err(
                            ExecutionError::InvalidObservationValue,
                        );
                    }
                }
            }

            super::super::core::execution::ExecutionObservation::DensityMatrix(
                entries,
            ) => {
                for (real, imaginary) in entries {
                    if !real.is_finite()
                        || !imaginary.is_finite()
                    {
                        return Err(
                            ExecutionError::InvalidObservationValue,
                        );
                    }
                }
            }

            super::super::core::execution::ExecutionObservation::AnalogSamples(
                values,
            ) => {
                for value in values {
                    if !value.is_finite() {
                        return Err(
                            ExecutionError::InvalidObservationValue,
                        );
                    }
                }
            }

            super::super::core::execution::ExecutionObservation::AnnealingSamples(
                samples,
            ) => {
                for sample in samples {
                    if let Some(energy) = sample.energy {
                        if !energy.is_finite() {
                            return Err(
                                ExecutionError::InvalidObservationValue,
                            );
                        }
                    }

                    if let Some(occurrences) = sample.occurrences {
                        if occurrences == 0 {
                            return Err(
                                ExecutionError::InvalidObservationOccurrenceCount,
                            );
                        }
                    }
                }
            }

            super::super::core::execution::ExecutionObservation::Syndrome(
                syndrome,
            ) => {
                if syndrome.samples.is_empty() {
                    return Err(
                        ExecutionError::EmptySyndromeObservation,
                    );
                }

                if let Some(rounds) = syndrome.rounds {
                    if rounds == 0 {
                        return Err(
                            ExecutionError::InvalidSyndromeRounds,
                        );
                    }
                }
            }

            super::super::core::execution::ExecutionObservation::BackendNative {
                format,
                payload,
            } => {
                if format.trim().is_empty() {
                    return Err(
                        ExecutionError::InvalidBackendNativeFormat,
                    );
                }

                // Empty provider-native payloads are permitted because some
                // providers use metadata/status fields for zero-data
                // responses. The provider contract decides whether such a
                // response is useful.
                let _ = payload;
            }
        }
    }

    // `request` is intentionally retained in the signature because future
    // observation validation may need request-specific measurement semantics.
    // Keeping this boundary stable prevents protocol code from reimplementing
    // observation validation.
    let _ = request;

    Ok(())
}

// =============================================================================
// Dry-run planning
// =============================================================================

/// Result of a validation-only execution plan.
///
/// This lets frontend, CLI and benchmark registry code determine whether an
/// experiment is executable without submitting work.
#[derive(Debug, Clone)]
pub struct ExecutionPlan {
    /// Orchestration version.
    pub orchestrator_version: u32,

    /// Executor identity.
    pub executor_id: String,

    /// Backend identity.
    pub backend_id: String,

    /// Execution mode.
    pub execution_mode: ExecutionMode,

    /// Requested shot count.
    pub shots: usize,

    /// Whether the request supplies a deterministic seed.
    pub deterministic_seed_requested: bool,

    /// Timeout.
    pub timeout: Duration,

    /// Capability requirements.
    pub requirements: ExecutionRequirements,
}

impl ExecutionPlan {
    /// Returns whether the execution request has an explicit seed.
    pub fn is_seeded(&self) -> bool {
        self.deterministic_seed_requested
    }
}

impl ExecutionOrchestrator {
    /// Builds a dry-run execution plan.
    pub fn plan<E>(
        &self,
        executor: &E,
        request: &ExecutionRequest,
        requirements: ExecutionRequirements,
    ) -> Result<ExecutionPlan, ExecutionError>
    where
        E: BenchmarkExecutor + ?Sized,
    {
        self.validate_with_requirements(
            executor,
            request,
            requirements,
        )?;

        Ok(ExecutionPlan {
            orchestrator_version: EXECUTION_ORCHESTRATOR_VERSION,
            executor_id: executor.metadata().id.clone(),
            backend_id: request.backend_id.as_str().to_owned(),
            execution_mode: request.execution_mode,
            shots: request.shots.get(),
            deterministic_seed_requested: request.seed.is_some(),
            timeout: request.timeout.duration(),
            requirements,
        })
    }
}

// =============================================================================
// Error helpers
// =============================================================================

/// Converts an execution error into a stable human-readable classification.
///
/// This function does not log or print.
pub fn classify_error(
    error: &ExecutionError,
) -> ExecutionErrorClass {
    match error {
        ExecutionError::InvalidRequestId
        | ExecutionError::InvalidBackendId
        | ExecutionError::InvalidExecutorIdentifier { .. }
        | ExecutionError::EmptyCircuit
        | ExecutionError::InvalidShotCount
        | ExecutionError::InvalidTimeout
        | ExecutionError::UnsupportedSchemaVersion { .. } => {
            ExecutionErrorClass::InvalidRequest
        }

        ExecutionError::BackendMismatch { .. }
        | ExecutionError::ExecutionModeMismatch { .. }
        | ExecutionError::UnsupportedCapability { .. }
        | ExecutionError::UnsupportedMeasurementMode { .. } => {
            ExecutionErrorClass::CapabilityOrCompatibility
        }

        ExecutionError::Cancelled
        | ExecutionError::CancellationRejected => {
            ExecutionErrorClass::Cancelled
        }

        ExecutionError::TimedOut { .. }
        | ExecutionError::TimeoutRejected => {
            ExecutionErrorClass::TimedOut
        }

        ExecutionError::PartialExecutionRejected
        | ExecutionError::PartialStatusWithCompleteShots { .. }
        | ExecutionError::CompletedStatusWithIncompleteShots { .. } => {
            ExecutionErrorClass::PartialOrCompletionInvariant
        }

        ExecutionError::RequestResponseMismatch { .. }
        | ExecutionError::BackendResponseMismatch { .. }
        | ExecutionError::RequestedShotMismatch { .. }
        | ExecutionError::ResponseExecutionModeMismatch { .. }
        | ExecutionError::InvalidCompletedShotCount { .. }
        | ExecutionError::InvalidResponseStatus { .. } => {
            ExecutionErrorClass::ResponseCorrelation
        }

        ExecutionError::ObservationCountOverflow
        | ExecutionError::ObservationShotCountExceeded { .. }
        | ExecutionError::InvalidObservationProbability
        | ExecutionError::InvalidObservationValue
        | ExecutionError::InvalidObservationOccurrenceCount
        | ExecutionError::EmptySyndromeObservation
        | ExecutionError::InvalidSyndromeRounds
        | ExecutionError::InvalidBackendNativeFormat
        | ExecutionError::EmptyObservationSet
        | ExecutionError::InvalidExecutionTiming => {
            ExecutionErrorClass::InvalidResponseData
        }

        ExecutionError::ProviderExecutionFailed => {
            ExecutionErrorClass::ProviderFailure
        }

        _ => ExecutionErrorClass::Other,
    }
}

/// High-level classification of execution failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExecutionErrorClass {
    /// Request itself is invalid.
    InvalidRequest,

    /// Backend/executor cannot satisfy the request.
    CapabilityOrCompatibility,

    /// Execution was cancelled.
    Cancelled,

    /// Execution exceeded its timeout.
    TimedOut,

    /// Partial/completion invariants were violated or rejected.
    PartialOrCompletionInvariant,

    /// Response does not correlate correctly with the request.
    ResponseCorrelation,

    /// Response data is malformed or inconsistent.
    InvalidResponseData,

    /// Provider reported an execution failure.
    ProviderFailure,

    /// Unclassified execution error.
    Other,
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeMap;
    use std::sync::Arc;

    use crate::quantum::benchmarking::core::execution::{
        BenchmarkExecutor,
        ExecutionBackendId,
        ExecutionCapabilities,
        ExecutionObservation,
        ExecutionRequest,
        ExecutionResponse,
        ExecutionTiming,
        ExecutorMetadata,
        MeasurementMode,
    };

    /// Minimal deterministic executor used only for orchestration tests.
    struct TestExecutor {
        metadata: ExecutorMetadata,
    }

    impl TestExecutor {
        fn new() -> Self {
            let backend_id =
                ExecutionBackendId::new("test.backend")
                    .expect("valid backend ID");

            let capabilities =
                ExecutionCapabilities::default();

            let metadata = ExecutorMetadata::new(
                "test.executor",
                "Zamani Test Executor",
                "1.0.0",
                backend_id,
                ExecutionMode::StateVectorSimulator,
                capabilities,
            )
            .expect("valid executor metadata");

            Self { metadata }
        }
    }

    impl BenchmarkExecutor for TestExecutor {
        fn metadata(&self) -> &ExecutorMetadata {
            &self.metadata
        }

        fn execute(
            &self,
            request: &ExecutionRequest,
        ) -> Result<ExecutionResponse, ExecutionError> {
            let mut counts = BTreeMap::new();

            counts.insert(
                "0".to_owned(),
                request.shots.get(),
            );

            ExecutionResponse::completed(
                request,
                vec![ExecutionObservation::Counts(
                    counts,
                )],
                ExecutionTiming::completed(
                    Duration::from_nanos(1),
                ),
            )
        }
    }

    fn test_request() -> ExecutionRequest {
        // This test intentionally avoids constructing a concrete Quantum IR
        // circuit here because the orchestration layer's behavior is tested
        // through the canonical request contract.
        //
        // The repository's QuantumCircuit constructor is owned by
        // quantum::ir. Production executor integrations must construct the
        // request through the canonical IR API.
        //
        // This helper is therefore only compiled when the repository's
        // existing QuantumCircuit default constructor is available.
        let circuit = Arc::new(
            crate::quantum::ir::QuantumCircuit::default(),
        );

        ExecutionRequest::new(
            crate::quantum::benchmarking::core::execution::ExecutionRequestId::new(
                "test-request",
            )
            .expect("valid request ID"),
            crate::quantum::benchmarking::core::execution::ExecutionBackendId::new(
                "test.backend",
            )
            .expect("valid backend ID"),
            ExecutionMode::StateVectorSimulator,
            circuit,
        )
        .expect("valid request")
        .with_shots(10)
        .expect("valid shots")
        .with_measurement_mode(MeasurementMode::Counts)
    }

    #[test]
    fn orchestrator_has_stable_version() {
        assert_eq!(
            ExecutionOrchestrator::version(),
            EXECUTION_ORCHESTRATOR_VERSION
        );
    }

    #[test]
    fn default_policy_allows_partial_execution() {
        let policy = ExecutionPolicy::default();

        assert!(policy.allow_partial);
        assert!(policy.allow_cancelled);
        assert!(!policy.allow_timed_out);
    }

    #[test]
    fn strict_policy_rejects_partial_execution() {
        let policy = ExecutionPolicy::strict();

        assert!(!policy.allow_partial);
        assert!(!policy.allow_cancelled);
        assert!(!policy.allow_timed_out);
    }

    #[test]
    fn execution_outcome_maps_completed() {
        assert_eq!(
            ExecutionOutcome::from_status(
                ExecutionStatus::Completed
            )
            .expect("completed"),
            ExecutionOutcome::Completed
        );
    }

    #[test]
    fn execution_outcome_maps_partial() {
        assert_eq!(
            ExecutionOutcome::from_status(
                ExecutionStatus::Partial
            )
            .expect("partial"),
            ExecutionOutcome::Partial
        );
    }

    #[test]
    fn execution_outcome_maps_cancelled() {
        assert_eq!(
            ExecutionOutcome::from_status(
                ExecutionStatus::Cancelled
            )
            .expect("cancelled"),
            ExecutionOutcome::Cancelled
        );
    }

    #[test]
    fn execution_outcome_maps_timeout() {
        assert_eq!(
            ExecutionOutcome::from_status(
                ExecutionStatus::TimedOut
            )
            .expect("timeout"),
            ExecutionOutcome::TimedOut
        );
    }

    #[test]
    fn capability_check_accepts_default_count_capability() {
        let request = test_request();

        let check = CapabilityCheck::from_request(
            &request,
            ExecutionRequirements::sampled_circuit(),
        );

        let capabilities =
            ExecutionCapabilities::default();

        assert!(
            check.validate(&capabilities).is_ok()
        );
    }

    #[test]
    fn orchestrator_can_validate_executor_without_execution() {
        let executor = TestExecutor::new();
        let orchestrator = ExecutionOrchestrator::new();

        let request = test_request();

        assert!(
            orchestrator
                .validate(&executor, &request)
                .is_ok()
        );
    }

    #[test]
    fn orchestrator_can_create_execution_plan() {
        let executor = TestExecutor::new();
        let orchestrator = ExecutionOrchestrator::new();

        let request = test_request();

        let plan = orchestrator
            .plan(
                &executor,
                &request,
                ExecutionRequirements::sampled_circuit(),
            )
            .expect("valid execution plan");

        assert_eq!(plan.shots, 10);
        assert!(!plan.is_seeded());
    }

    #[test]
    fn cancellation_token_is_cooperative() {
        let token = CancellationToken::new();

        assert!(!token.is_cancelled());
        assert!(token.check().is_ok());

        token.cancel();

        assert!(token.is_cancelled());
        assert!(matches!(
            token.check(),
            Err(ExecutionError::Cancelled)
        ));
    }

    #[test]
    fn error_classification_is_stable() {
        let error = ExecutionError::Cancelled;

        assert_eq!(
            classify_error(&error),
            ExecutionErrorClass::Cancelled
        );
    }
}