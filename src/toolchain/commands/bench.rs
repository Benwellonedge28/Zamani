
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Benchmark Command
//!
//! Path: src/toolchain/commands/bench.rs
//! Edition: Rust 2021
//! Minimum Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## Mission
//!
//! Coordinate real benchmark operations across the Zamani ecosystem.
//!
//! This module provides a domain-neutral benchmark orchestration boundary
//! for classical, quantum, HDL, AI, tensor, hybrid, distributed and future
//! computational domains.
//!
//! It does not implement benchmark algorithms, hardware discovery,
//! compilation, physical resource allocation, process execution,
//! runtime scheduling or statistical research protocols.
//!
//! Those responsibilities remain with their authoritative subsystems.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Benchmark command requests.
//! - Request validation.
//! - Benchmark provider contracts.
//! - Benchmark workload lifecycle coordination.
//! - Warm-up and measurement orchestration.
//! - Streaming sample reporting.
//! - Online aggregate statistics.
//! - Benchmark result classification.
//! - Provider-response validation.
//!
//! It does not own:
//!
//! - Source-language grammar.
//! - Parsing or semantic analysis.
//! - Compiler execution implementation.
//! - Quantum benchmarking protocols.
//! - Hardware benchmarking algorithms.
//! - Physical device discovery.
//! - Resource allocation.
//! - External process execution.
//! - Runtime scheduling.
//! - Artifact generation.
//! - Environment fingerprint algorithms.
//! - Cryptographic provenance algorithms.
//!
//! ## Canonical integrations
//!
//! Compiler scalability:
//!     crate::compiler::testing::scalability
//!
//! Quantum benchmarking:
//!     crate::quantum::benchmarking
//!
//! Compiler regression infrastructure:
//!     crate::compiler::testing::regression
//!
//! Build orchestration:
//!     crate::toolchain::build
//!
//! Operational budgets:
//!     crate::toolchain::configuration::budget
//!
//! Cancellation:
//!     crate::toolchain::tasks::cancellation
//!
//! Backend process execution:
//!     crate::backend::process
//!
//! External tool discovery:
//!     crate::backend::toolchain
//!
//! Hardware:
//!     crate::zhal
//!
//! Runtime:
//!     crate::zur
//!
//! ## POCO-REAF
//!
//! One logical benchmark request may evaluate multiple target realizations
//! through independently prepared provider instances.
//!
//! A benchmark result is tied to its actual:
//!
//! - Workload.
//! - Domain.
//! - Provider.
//! - Implementation version.
//! - Target realization.
//! - Environment.
//! - Semantic input.
//! - Measurement configuration.
//!
//! Results from different environments must not automatically be treated
//! as directly comparable.
//!
//! ## Scalability
//!
//! No arbitrary universal ceiling is imposed on:
//!
//! - Benchmark samples.
//! - Iterations.
//! - Workload size.
//! - Source files.
//! - Targets.
//! - Providers.
//! - Computing domains.
//! - Processors.
//! - Accelerators.
//! - Quantum resources.
//! - Tensor dimensions.
//! - Distributed participants.
//!
//! Counters use checked u128 arithmetic.
//!
//! Samples are aggregated incrementally. The command does not retain an
//! unbounded vector of measurements.
//!
//! Actual execution remains subject to:
//!
//! - Available resources.
//! - Rust representation limits.
//! - Operating-system constraints.
//! - Provider capabilities.
//! - Explicit operational budgets.
//! - Cancellation.
//!
//! Literal infinite computation is not promised.
//!
//! ## Determinism
//!
//! Benchmark execution order follows the requested measurement sequence.
//!
//! Measured durations are observations, not deterministic semantic values.
//!
//! Benchmark execution must not change program semantics.
//!
//! Environment identity and benchmark configuration must accompany
//! measurements intended for reproducibility.
//!
//! ## Failure semantics
//!
//! The command must never:
//!
//! - Report an unexecuted workload as successful.
//! - Convert provider failure into benchmark success.
//! - Convert resource exhaustion into successful completion.
//! - Treat a cancelled partial result as a completed benchmark.
//! - Compare measurements with incompatible identities as equivalent.
//! - Invent unavailable target capabilities.
//!
//! ## Provider integration
//!
//! A production deployment must supply an actual
//! BenchmarkExecutionProvider.
//!
//! There is deliberately no fake default implementation.

use serde::{Deserialize, Serialize};

use std::fmt;
use std::hint::black_box;
use std::time::Instant;

use crate::toolchain::errors::ToolchainError;
use crate::toolchain::result::ToolchainResult;
use crate::toolchain::tasks::cancellation::CancellationToken;

/// Stable schema identifier for benchmark reports.
pub const BENCHMARK_REPORT_SCHEMA: &str =
    "zamani.toolchain.benchmark-report.v1";

/// Public API version of the benchmark command.
pub const BENCHMARK_COMMAND_API_VERSION: u16 = 1;

// ============================================================================
// REQUEST
// ============================================================================

/// Immutable intent for one benchmark operation.
///
/// Domain, workload and target identifiers are open-ended strings.
/// They are resolved by authoritative registered providers.
///
/// The iteration and sample counts are requested workload parameters.
/// They are not universal implementation or hardware limits.
///
/// No upper bound is imposed by this request type.
///
/// The caller remains responsible for selecting workload sizes that
/// are appropriate for the configured operational budget.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BenchmarkRequest {
    /// Open computational-domain identifier.
    ///
    /// Examples include classical, quantum, hdl, ai, tensor, hybrid
    /// and distributed. Additional identifiers may be registered.
    pub domain: String,

    /// Provider-defined benchmark workload identifier.
    pub workload: String,

    /// Optional requested target selector.
    ///
    /// This is intent, not evidence of target feasibility.
    pub target: Option<String>,

    /// Number of warm-up executions.
    ///
    /// Warm-up executions are excluded from measured results.
    pub warmup_iterations: u128,

    /// Number of measured executions in each sample.
    pub iterations_per_sample: u128,

    /// Number of independent measured samples requested.
    pub samples: u128,
}

impl BenchmarkRequest {
    /// Construct and validate a benchmark request.
    pub fn new(
        domain: impl Into<String>,
        workload: impl Into<String>,
        warmup_iterations: u128,
        iterations_per_sample: u128,
        samples: u128,
    ) -> ToolchainResult<Self> {
        let request = Self {
            domain: domain.into(),
            workload: workload.into(),
            target: None,
            warmup_iterations,
            iterations_per_sample,
            samples,
        };

        request.validate()?;

        Ok(request)
    }

    /// Set an explicit target selector.
    pub fn with_target(
        mut self,
        target: impl Into<String>,
    ) -> ToolchainResult<Self> {
        self.target = Some(target.into());
        self.validate()?;
        Ok(self)
    }

    /// Validate the benchmark request.
    pub fn validate(&self) -> ToolchainResult<()> {
        validate_identifier("domain", &self.domain)?;
        validate_identifier("workload", &self.workload)?;

        if let Some(target) = &self.target {
            validate_identifier("target", target)?;
        }

        if self.iterations_per_sample == 0 {
            return Err(validation_error(
                "Measured iterations per sample must be greater than zero",
            ));
        }

        if self.samples == 0 {
            return Err(validation_error(
                "The requested number of samples must be greater than zero",
            ));
        }

        Ok(())
    }
}

// ============================================================================
// VALIDATION
// ============================================================================

fn validate_identifier(
    field: &str,
    value: &str,
) -> ToolchainResult<()> {
    if value.trim().is_empty() {
        return Err(validation_error(format!(
            "Benchmark {field} must not be empty"
        )));
    }

    if value != value.trim() {
        return Err(validation_error(format!(
            "Benchmark {field} must not contain surrounding whitespace"
        )));
    }

    if value.chars().any(char::is_control) {
        return Err(validation_error(format!(
            "Benchmark {field} must not contain control characters"
        )));
    }

    Ok(())
}

fn validation_error(
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::validation_failed(message)
        .with_operation("commands.bench.validate")
}

// ============================================================================
// BENCHMARK PROVENANCE
// ============================================================================

/// Provenance describing the actual benchmark realization.
///
/// Fingerprint generation remains the responsibility of the
/// corresponding compiler, backend, environment and provider authorities.
///
/// The command records the supplied fingerprints without inventing
/// its own fingerprinting algorithm.
///
/// Fingerprints should identify immutable input snapshots or
/// versioned environment descriptions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BenchmarkMetadata {
    /// Actual provider identifier.
    pub provider_id: String,

    /// Computational-domain identifier.
    pub domain: String,

    /// Actual workload identifier.
    pub workload: String,

    /// Provider implementation version.
    pub implementation_version: String,

    /// Actual target realization identity, if applicable.
    pub target_realization: Option<String>,

    /// Fingerprint of the measured execution environment.
    pub environment_fingerprint: String,

    /// Fingerprint of the semantic workload inputs.
    pub input_fingerprint: String,

    /// Unit describing the measured operation.
    pub measurement_unit: String,
}

impl BenchmarkMetadata {
    /// Validate benchmark provenance.
    pub fn validate(&self) -> ToolchainResult<()> {
        validate_identifier("provider_id", &self.provider_id)?;
        validate_identifier("domain", &self.domain)?;
        validate_identifier("workload", &self.workload)?;

        validate_identifier(
            "implementation_version",
            &self.implementation_version,
        )?;

        validate_identifier(
            "environment_fingerprint",
            &self.environment_fingerprint,
        )?;

        validate_identifier(
            "input_fingerprint",
            &self.input_fingerprint,
        )?;

        validate_identifier(
            "measurement_unit",
            &self.measurement_unit,
        )?;

        if let Some(target) = &self.target_realization {
            validate_identifier("target_realization", target)?;
        }

        Ok(())
    }
}

// ============================================================================
// EXECUTION CONTRACT
// ============================================================================

/// One prepared, executable benchmark workload.
///
/// Implementations must perform the actual requested operation.
///
/// The operation may invoke the compiler, execute a verified artifact,
/// exercise a hardware provider, invoke a quantum benchmark protocol,
/// or evaluate another registered computing-domain workload.
///
/// The implementation must not return success without performing
/// the operation represented by the workload.
///
/// Long-running operations should observe cancellation internally.
pub trait BenchmarkWorkload: Send {
    /// Execute one actual benchmark iteration.
    ///
    /// The command measures the wall-clock time around this operation.
    ///
    /// The provider must ensure that the measured operation corresponds
    /// to the workload and that any required correctness validation
    /// occurs before success is returned.
    ///
    /// # Errors
    ///
    /// Returns the original structured failure if execution fails.
    fn execute_iteration(
        &mut self,
        cancellation: &CancellationToken,
    ) -> ToolchainResult<()>;
}

/// Complete prepared benchmark.
///
/// The metadata must describe the actual workload that will execute.
/// The workload implementation is injected by an authoritative provider.
pub struct PreparedBenchmark {
    metadata: BenchmarkMetadata,
    workload: Box<dyn BenchmarkWorkload>,
}

impl PreparedBenchmark {
    /// Construct a prepared benchmark.
    pub fn new<W>(
        metadata: BenchmarkMetadata,
        workload: W,
    ) -> ToolchainResult<Self>
    where
        W: BenchmarkWorkload + 'static,
    {
        metadata.validate()?;

        Ok(Self {
            metadata,
            workload: Box::new(workload),
        })
    }

    /// Borrow the validated benchmark metadata.
    #[must_use]
    pub fn metadata(&self) -> &BenchmarkMetadata {
        &self.metadata
    }
}

/// Authoritative benchmark execution provider.
///
/// Implementations adapt the existing repository authorities.
///
/// A provider is responsible for:
///
/// - Validating that the requested workload exists.
/// - Resolving actual domain-specific benchmark implementations.
/// - Validating requested target feasibility.
/// - Preparing the actual workload.
/// - Obtaining authoritative environment fingerprints.
/// - Obtaining semantic input fingerprints.
/// - Respecting security and authorization policies.
/// - Respecting applicable shared operational budgets.
/// - Returning accurate provenance.
///
/// For compiler benchmarks, integrate the compiler API and build
/// infrastructure.
///
/// For quantum benchmarks, integrate quantum::benchmarking.
///
/// For hardware benchmarks, integrate ZHAL and registered providers.
///
/// For runtime benchmarks, integrate ZUR.
///
/// For external tools, reuse backend::toolchain and backend::process.
///
/// Do not implement these authorities again here.
pub trait BenchmarkExecutionProvider: Send + Sync {
    /// Prepare an actual benchmark workload.
    ///
    /// Preparation is outside the measured execution interval.
    ///
    /// # Errors
    ///
    /// Returns an explicit failure when the workload is unavailable,
    /// infeasible, unauthorized, incompatible or cannot be prepared.
    fn prepare(
        &self,
        request: &BenchmarkRequest,
        cancellation: &CancellationToken,
    ) -> ToolchainResult<PreparedBenchmark>;
}

// ============================================================================
// RESULT STATUS
// ============================================================================

/// Overall benchmark execution status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkStatus {
    /// Every requested sample completed.
    Completed,

    /// Cancellation prevented completion of the requested benchmark.
    Cancelled,
}

impl BenchmarkStatus {
    /// Return the stable machine-readable status.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Completed => "completed",
            Self::Cancelled => "cancelled",
        }
    }
}

// ============================================================================
// SAMPLE
// ============================================================================

/// One completed measured sample.
///
/// Sample indices are zero-based.
///
/// This report is emitted immediately after the sample completes.
/// The command does not retain all samples in memory.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BenchmarkSample {
    /// Zero-based sample index.
    pub index: u128,

    /// Number of measured iterations in this sample.
    pub iterations: u128,

    /// Total observed wall-clock duration in nanoseconds.
    pub elapsed_nanoseconds: u128,

    /// Observed average nanoseconds per iteration.
    pub nanoseconds_per_iteration: f64,
}

// ============================================================================
// STATISTICS
// ============================================================================

/// Online aggregate benchmark statistics.
///
/// Uses Welford's online algorithm for mean and variance.
///
/// Memory consumption is constant with respect to the number of samples.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BenchmarkStatistics {
    /// Number of completed measured samples.
    pub completed_samples: u128,

    /// Total number of measured iterations.
    pub total_iterations: u128,

    /// Minimum observed sample-average duration.
    pub minimum_nanoseconds_per_iteration: Option<f64>,

    /// Maximum observed sample-average duration.
    pub maximum_nanoseconds_per_iteration: Option<f64>,

    /// Arithmetic mean of sample-average durations.
    pub mean_nanoseconds_per_iteration: Option<f64>,

    /// Population standard deviation of sample-average durations.
    pub standard_deviation_nanoseconds: Option<f64>,

    /// Sum of observed sample durations in nanoseconds.
    pub total_elapsed_nanoseconds: u128,

    /// Internal online variance accumulator.
    #[serde(skip)]
    mean: f64,

    /// Internal online variance accumulator.
    #[serde(skip)]
    squared_deviation_sum: f64,
}

impl Default for BenchmarkStatistics {
    fn default() -> Self {
        Self {
            completed_samples: 0,
            total_iterations: 0,
            minimum_nanoseconds_per_iteration: None,
            maximum_nanoseconds_per_iteration: None,
            mean_nanoseconds_per_iteration: None,
            standard_deviation_nanoseconds: None,
            total_elapsed_nanoseconds: 0,
            mean: 0.0,
            squared_deviation_sum: 0.0,
        }
    }
}

impl BenchmarkStatistics {
    /// Add one completed sample using checked counters.
    fn observe(
        &mut self,
        sample: &BenchmarkSample,
    ) -> ToolchainResult<()> {
        let next_samples = self
            .completed_samples
            .checked_add(1)
            .ok_or_else(|| {
                ToolchainError::resource_exhausted(
                    "Benchmark sample counter overflow",
                )
                .with_operation("commands.bench.statistics")
            })?;

        let next_iterations = self
            .total_iterations
            .checked_add(sample.iterations)
            .ok_or_else(|| {
                ToolchainError::resource_exhausted(
                    "Benchmark iteration counter overflow",
                )
                .with_operation("commands.bench.statistics")
            })?;

        let next_elapsed = self
            .total_elapsed_nanoseconds
            .checked_add(sample.elapsed_nanoseconds)
            .ok_or_else(|| {
                ToolchainError::resource_exhausted(
                    "Benchmark elapsed-time accounting overflow",
                )
                .with_operation("commands.bench.statistics")
            })?;

        let value = sample.nanoseconds_per_iteration;

        let delta = value - self.mean;
        let next_mean =
            self.mean + delta / next_samples as f64;

        let delta_after_mean = value - next_mean;

        let next_squared_deviation_sum =
            self.squared_deviation_sum
                + delta * delta_after_mean;

        self.completed_samples = next_samples;
        self.total_iterations = next_iterations;
        self.total_elapsed_nanoseconds = next_elapsed;

        self.mean = next_mean;
        self.squared_deviation_sum =
            next_squared_deviation_sum;

        self.minimum_nanoseconds_per_iteration =
            Some(
                self.minimum_nanoseconds_per_iteration
                    .map_or(value, |current| current.min(value)),
            );

        self.maximum_nanoseconds_per_iteration =
            Some(
                self.maximum_nanoseconds_per_iteration
                    .map_or(value, |current| current.max(value)),
            );

        self.mean_nanoseconds_per_iteration =
            Some(self.mean);

        self.standard_deviation_nanoseconds = Some(
            (self.squared_deviation_sum
                / next_samples as f64)
                .max(0.0)
                .sqrt(),
        );

        Ok(())
    }
}

// ============================================================================
// COMMAND RESULT
// ============================================================================

/// Structured result of a benchmark operation.
///
/// Cancelled results contain only measurements that actually completed.
/// They must not be interpreted as a completed benchmark.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BenchmarkCommandResult {
    /// Report schema.
    pub schema: String,

    /// Overall benchmark status.
    pub status: BenchmarkStatus,

    /// Actual workload provenance.
    pub metadata: BenchmarkMetadata,

    /// Requested measurement configuration.
    pub request: BenchmarkRequest,

    /// Aggregate completed measurements.
    pub statistics: BenchmarkStatistics,

    /// Whether every requested sample completed.
    pub completed: bool,
}

impl BenchmarkCommandResult {
    fn new(
        status: BenchmarkStatus,
        metadata: BenchmarkMetadata,
        request: BenchmarkRequest,
        statistics: BenchmarkStatistics,
        completed: bool,
    ) -> Self {
        Self {
            schema: BENCHMARK_REPORT_SCHEMA.to_owned(),
            status,
            metadata,
            request,
            statistics,
            completed,
        }
    }
}

// ============================================================================
// PROVIDER VALIDATION
// ============================================================================

fn validate_provider_result(
    request: &BenchmarkRequest,
    metadata: &BenchmarkMetadata,
) -> ToolchainResult<()> {
    metadata.validate()?;

    if metadata.domain != request.domain {
        return Err(
            ToolchainError::validation_failed(
                "Benchmark provider returned a different computational domain",
            )
            .with_operation("commands.bench.provider_validation"),
        );
    }

    if metadata.workload != request.workload {
        return Err(
            ToolchainError::validation_failed(
                "Benchmark provider returned a different workload",
            )
            .with_operation("commands.bench.provider_validation"),
        );
    }

    if let Some(requested_target) = &request.target {
        if metadata.target_realization.as_ref()
            != Some(requested_target)
        {
            return Err(
                ToolchainError::validation_failed(
                    "Benchmark provider did not establish the requested target realization",
                )
                .with_operation("commands.bench.provider_validation"),
            );
        }
    }

    Ok(())
}

// ============================================================================
// EXECUTION
// ============================================================================

/// Execute a benchmark through an authoritative provider.
///
/// The operation has the following stages:
///
/// 1. Validate the request.
/// 2. Check cancellation.
/// 3. Prepare the actual workload.
/// 4. Validate workload provenance.
/// 5. Execute warm-up iterations.
/// 6. Execute measured samples.
/// 7. Aggregate statistics incrementally.
/// 8. Stream completed samples to the caller.
/// 9. Return the actual completion status.
///
/// Preparation and warm-up are excluded from measured durations.
///
/// The caller controls report delivery through `on_sample`.
/// The callback may serialize results, store them or stream them
/// to another consumer.
///
/// The command does not select a filesystem or network transport.
///
/// # Errors
///
/// Returns the original structured error when:
///
/// - Request validation fails.
/// - Provider preparation fails.
/// - Provider provenance is inconsistent.
/// - Workload execution fails.
/// - Statistics accounting overflows.
/// - Sample reporting fails.
///
/// Cancellation between samples returns a structured partial result.
/// Cancellation during a provider operation is cooperative and must
/// be observed by that provider.
///
/// # Scalability
///
/// Samples are processed sequentially and streamed.
/// Aggregate memory consumption is independent of sample count.
///
/// Parallel benchmarking is delegated to provider-owned execution
/// infrastructure and must not be introduced by this command
/// through an arbitrary fixed worker pool.
pub fn execute_benchmark<P, F>(
    request: &BenchmarkRequest,
    provider: &P,
    cancellation: &CancellationToken,
    mut on_sample: F,
) -> ToolchainResult<BenchmarkCommandResult>
where
    P: BenchmarkExecutionProvider + ?Sized,
    F: FnMut(&BenchmarkSample) -> ToolchainResult<()>,
{
    request.validate()?;

    cancellation.check_cancelled()?;

    let mut prepared = provider
        .prepare(request, cancellation)
        .map_err(|error| {
            error.with_operation("commands.bench.prepare")
        })?;

    validate_provider_result(
        request,
        prepared.metadata(),
    )?;

    cancellation.check_cancelled()?;

    // Warm-up is deliberately excluded from all measured samples.
    for _ in 0..request.warmup_iterations {
        cancellation.check_cancelled()?;

        prepared
            .workload
            .execute_iteration(cancellation)
            .map_err(|error| {
                error.with_operation("commands.bench.warmup")
            })?;
    }

    let mut statistics = BenchmarkStatistics::default();

    for sample_index in 0..request.samples {
        // Cancellation is checked at sample boundaries. Providers
        // must also observe the token during long-running iterations.
        if cancellation.is_cancelled() {
            return Ok(BenchmarkCommandResult::new(
                BenchmarkStatus::Cancelled,
                prepared.metadata.clone(),
                request.clone(),
                statistics,
                false,
            ));
        }

        let started = Instant::now();

        for _ in 0..request.iterations_per_sample {
            prepared
                .workload
                .execute_iteration(cancellation)
                .map_err(|error| {
                    error.with_operation("commands.bench.measure")
                })?;
        }

        let elapsed = started.elapsed();
        let elapsed_nanoseconds = elapsed.as_nanos();

        let sample = BenchmarkSample {
            index: sample_index,
            iterations: request.iterations_per_sample,
            elapsed_nanoseconds,
            nanoseconds_per_iteration:
                elapsed_nanoseconds as f64
                    / request.iterations_per_sample as f64,
        };

        statistics.observe(&sample)?;

        on_sample(&sample).map_err(|error| {
            error.with_operation("commands.bench.report_sample")
        })?;
    }

    // Cancellation can arrive while the last sample is completing.
    // Do not claim completion if cancellation was already requested.
    if cancellation.is_cancelled() {
        return Ok(BenchmarkCommandResult::new(
            BenchmarkStatus::Cancelled,
            prepared.metadata,
            request.clone(),
            statistics,
            false,
        ));
    }

    Ok(BenchmarkCommandResult::new(
        BenchmarkStatus::Completed,
        prepared.metadata,
        request.clone(),
        statistics,
        true,
    ))
}

// ============================================================================
// COMPARABILITY
// ============================================================================

/// Determine whether two benchmark metadata records describe
/// sufficiently identical realization identities for direct comparison.
///
/// This is a conservative identity check, not proof of statistical
/// equivalence or experimental validity.
///
/// Providers remain responsible for documenting additional conditions
/// such as calibration, thermal state, simulator configuration,
/// measurement methodology and external system load.
#[must_use]
pub fn have_comparable_identity(
    left: &BenchmarkMetadata,
    right: &BenchmarkMetadata,
) -> bool {
    left.domain == right.domain
        && left.workload == right.workload
        && left.implementation_version == right.implementation_version
        && left.target_realization == right.target_realization
        && left.environment_fingerprint == right.environment_fingerprint
        && left.input_fingerprint == right.input_fingerprint
        && left.measurement_unit == right.measurement_unit
}

// ============================================================================
// API INFORMATION
// ============================================================================

/// Return the benchmark command API version.
#[must_use]
pub const fn api_version() -> u16 {
    BENCHMARK_COMMAND_API_VERSION
}

/// Return the stable report schema identifier.
#[must_use]
pub const fn report_schema() -> &'static str {
    BENCHMARK_REPORT_SCHEMA
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::toolchain::tasks::cancellation::CancellationSource;

    use std::sync::atomic::{
        AtomicUsize,
        Ordering,
    };
    use std::sync::Arc;

    struct CountingWorkload {
        executions: Arc<AtomicUsize>,
    }

    impl BenchmarkWorkload for CountingWorkload {
        fn execute_iteration(
            &mut self,
            cancellation: &CancellationToken,
        ) -> ToolchainResult<()> {
            cancellation.check_cancelled()?;

            self.executions.fetch_add(
                1,
                Ordering::Relaxed,
            );

            black_box(42_u64);

            Ok(())
        }
    }

    struct CountingProvider {
        executions: Arc<AtomicUsize>,
    }

    impl BenchmarkExecutionProvider for CountingProvider {
        fn prepare(
            &self,
            request: &BenchmarkRequest,
            _: &CancellationToken,
        ) -> ToolchainResult<PreparedBenchmark> {
            PreparedBenchmark::new(
                BenchmarkMetadata {
                    provider_id: "test.provider".to_owned(),
                    domain: request.domain.clone(),
                    workload: request.workload.clone(),
                    implementation_version: "1".to_owned(),
                    target_realization: request.target.clone(),
                    environment_fingerprint:
                        "environment-test".to_owned(),
                    input_fingerprint: "input-test".to_owned(),
                    measurement_unit: "nanoseconds_per_iteration"
                        .to_owned(),
                },
                CountingWorkload {
                    executions: Arc::clone(&self.executions),
                },
            )
        }
    }

    fn test_request() -> BenchmarkRequest {
        BenchmarkRequest::new(
            "classical",
            "addition",
            2,
            3,
            4,
        )
        .expect("valid benchmark request")
    }

    #[test]
    fn request_rejects_zero_measured_iterations() {
        assert!(
            BenchmarkRequest::new(
                "classical",
                "addition",
                0,
                0,
                1,
            )
            .is_err()
        );
    }

    #[test]
    fn request_rejects_zero_samples() {
        assert!(
            BenchmarkRequest::new(
                "classical",
                "addition",
                0,
                1,
                0,
            )
            .is_err()
        );
    }

    #[test]
    fn request_accepts_open_domain_identifiers() {
        let request = BenchmarkRequest::new(
            "future.experimental.domain",
            "provider.custom.workload",
            0,
            1,
            1,
        )
        .expect("open identifiers must be accepted");

        assert_eq!(
            request.domain,
            "future.experimental.domain"
        );
    }

    #[test]
    fn benchmark_executes_real_provider_iterations() {
        let executions = Arc::new(AtomicUsize::new(0));

        let provider = CountingProvider {
            executions: Arc::clone(&executions),
        };

        let cancellation = CancellationSource::new();

        let result = execute_benchmark(
            &test_request(),
            &provider,
            &cancellation.token(),
            |_| Ok(()),
        )
        .expect("benchmark should complete");

        // Two warm-ups plus four samples of three iterations.
        assert_eq!(
            executions.load(Ordering::Relaxed),
            14
        );

        assert_eq!(
            result.status,
            BenchmarkStatus::Completed
        );

        assert!(result.completed);

        assert_eq!(
            result.statistics.completed_samples,
            4
        );

        assert_eq!(
            result.statistics.total_iterations,
            12
        );
    }

    #[test]
    fn samples_are_streamed_without_retaining_them() {
        let executions = Arc::new(AtomicUsize::new(0));

        let provider = CountingProvider {
            executions,
        };

        let cancellation = CancellationSource::new();

        let observed = AtomicUsize::new(0);

        let result = execute_benchmark(
            &test_request(),
            &provider,
            &cancellation.token(),
            |_| {
                observed.fetch_add(1, Ordering::Relaxed);
                Ok(())
            },
        )
        .expect("benchmark should complete");

        assert_eq!(
            observed.load(Ordering::Relaxed),
            4
        );

        assert_eq!(
            result.statistics.completed_samples,
            4
        );
    }

    #[test]
    fn cancelled_benchmark_is_not_reported_as_completed() {
        let executions = Arc::new(AtomicUsize::new(0));

        let provider = CountingProvider {
            executions,
        };

        let cancellation = CancellationSource::new();

        let token = cancellation.token();

        let result = execute_benchmark(
            &test_request(),
            &provider,
            &token,
            |_| {
                cancellation.cancel("test cancellation");
                Ok(())
            },
        )
        .expect("cooperative cancellation returns partial report");

        assert_eq!(
            result.status,
            BenchmarkStatus::Cancelled
        );

        assert!(!result.completed);

        assert_eq!(
            result.statistics.completed_samples,
            1
        );
    }

    #[test]
    fn metadata_comparability_requires_matching_environment() {
        let metadata = BenchmarkMetadata {
            provider_id: "provider".to_owned(),
            domain: "classical".to_owned(),
            workload: "addition".to_owned(),
            implementation_version: "1".to_owned(),
            target_realization: None,
            environment_fingerprint: "environment-a".to_owned(),
            input_fingerprint: "input-a".to_owned(),
            measurement_unit: "nanoseconds_per_iteration".to_owned(),
        };

        let mut other = metadata.clone();

        assert!(have_comparable_identity(
            &metadata,
            &other
        ));

        other.environment_fingerprint =
            "environment-b".to_owned();

        assert!(!have_comparable_identity(
            &metadata,
            &other
        ));
    }

    #[test]
    fn statistics_use_online_aggregation() {
        let mut statistics = BenchmarkStatistics::default();

        let first = BenchmarkSample {
            index: 0,
            iterations: 1,
            elapsed_nanoseconds: 10,
            nanoseconds_per_iteration: 10.0,
        };

        let second = BenchmarkSample {
            index: 1,
            iterations: 1,
            elapsed_nanoseconds: 20,
            nanoseconds_per_iteration: 20.0,
        };

        statistics.observe(&first).expect("first sample");

        statistics.observe(&second).expect("second sample");

        assert_eq!(statistics.completed_samples, 2);

        assert_eq!(
            statistics.mean_nanoseconds_per_iteration,
            Some(15.0)
        );

        assert_eq!(
            statistics.minimum_nanoseconds_per_iteration,
            Some(10.0)
        );

        assert_eq!(
            statistics.maximum_nanoseconds_per_iteration,
            Some(20.0)
        );
    }

    #[test]
    fn api_contract_is_versioned() {
        assert_eq!(api_version(), 1);

        assert_eq!(
            report_schema(),
            "zamani.toolchain.benchmark-report.v1"
        );
    }
}
