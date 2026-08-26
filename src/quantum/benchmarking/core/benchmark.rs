//! Zamani Quantum Benchmarking — Core Benchmark Contract
//!
//! `benchmark.rs` is the authoritative orchestration contract for the
//! quantum benchmarking subsystem.
//!
//! # Architectural responsibility
//!
//! This module defines what a benchmark is and how a benchmark moves through
//! its lifecycle:
//!
//! ```text
//! BenchmarkConfig
//!       │
//!       ▼
//!   validation
//!       │
//!       ▼
//! Benchmark::generate
//!       │
//!       ▼
//!    Experiment
//!       │
//!       ▼
//! BenchmarkExecutor
//!       │
//!       ▼
//! ObservationSet
//!       │
//!       ▼
//! Benchmark::analyze
//!       │
//!       ▼
//! BenchmarkResult
//! ```
//!
//! The benchmark contract deliberately does NOT own:
//!
//! - Quantum IR semantics;
//! - circuit generation algorithms;
//! - backend communication;
//! - hardware topology;
//! - calibration;
//! - statistical algorithms;
//! - metric mathematics;
//! - reporting formats;
//! - protocol-specific implementation.
//!
//! Those responsibilities belong to their owning modules.
//!
//! # Dependency direction
//!
//! ```text
//! quantum::frontend
//!        │
//!        ▼
//! quantum::ir
//!        │
//!        ├───────────────┐
//!        ▼               ▼
//! quantum::algorithms   quantum::benchmarking
//!                            │
//!                ┌───────────┼────────────┐
//!                ▼           ▼            ▼
//!             generators execution    statistics
//!                │           │            │
//!                └───────────┼────────────┘
//!                            ▼
//!                       BenchmarkResult
//! ```
//!
//! Benchmarking may consume the canonical Quantum IR, but the IR must never
//! depend on benchmarking.
//!
//! # Production properties
//!
//! The contract is designed to guarantee:
//!
//! - deterministic benchmark identity;
//! - explicit validation before generation;
//! - explicit execution boundaries;
//! - backend independence;
//! - simulator/hardware independence;
//! - reproducible experiments;
//! - structured failures;
//! - cancellation propagation;
//! - bounded execution through configuration limits;
//! - no process-global benchmark state;
//! - no direct printing/logging from library code;
//! - reusable analysis of previously captured observations;
//! - support for hardware, simulator, QEC, application, volumetric and
//!   future benchmark families;
//! - object-safe benchmark registration;
//! - stable orchestration suitable for the Zamani language frontend.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97 / Rust 1.97.1, Rust 2021.
//!
//! No nightly features are required.
//!
//! # Integration contract
//!
//! This module expects the following sibling modules to provide the
//! corresponding stable contracts:
//!
//! - `config.rs` → `BenchmarkConfig`
//! - `errors.rs` → `BenchmarkError`
//! - `experiment.rs` → `BenchmarkExperiment`
//! - `execution.rs` → `BenchmarkExecutor`
//! - `observation.rs` → `BenchmarkObservationSet`
//! - `result.rs` → `BenchmarkResult`
//!
//! The benchmark protocol implementation should normally live under
//! `protocols/`, `applications/`, or `qec/` and implement this trait.
//!
//! The contract intentionally avoids depending on any concrete protocol.
//!
//! # Example
//!
//! A Quantum Volume implementation eventually looks conceptually like:
//!
//! ```text
//! QuantumVolumeBenchmark
//!        │
//!        ├── validate()
//!        ├── generate()
//!        ├── execute()
//!        └── analyze()
//!                  │
//!                  ▼
//!          QuantumVolumeResult
//! ```
//!
//! while the generic caller only knows about `dyn Benchmark`.
//!
//! This allows the same orchestration machinery to execute:
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
//! - random circuit sampling;
//! - mirror circuits;
//! - SPAM characterization;
//! - coherence characterization;
//! - crosstalk;
//! - drift;
//! - application benchmarks;
//! - QEC benchmarks;
//! - custom Zamani benchmarks.
//!
//! The benchmark contract therefore remains deliberately protocol-neutral.

use std::fmt;
use std::time::{Duration, Instant};

use super::config::BenchmarkConfig;
use super::errors::BenchmarkError;
use super::execution::BenchmarkExecutor;
use super::experiment::BenchmarkExperiment;
use super::observation::BenchmarkObservationSet;
use super::result::BenchmarkResult;

// =============================================================================
// Public constants
// =============================================================================

/// Stable schema version for the benchmark contract.
///
/// This is intentionally separate from individual protocol versions.
pub const BENCHMARK_CONTRACT_VERSION: &str = "1.0.0";

/// Stable identifier for the core benchmark abstraction.
pub const BENCHMARK_COMPONENT_ID: &str = "zamani.quantum.benchmark";

/// Maximum number of lifecycle phases retained by the orchestration layer.
///
/// This is deliberately small and fixed because lifecycle state is diagnostic
/// metadata, not an unbounded event log.
const MAX_LIFECYCLE_EVENTS: usize = 16;

// =============================================================================
// Benchmark identity
// =============================================================================

/// Stable identifier for a benchmark.
///
/// A benchmark ID is a machine-readable identity and MUST NOT contain
/// execution-specific information such as timestamps, random seeds, backend
/// IDs, or result hashes.
///
/// Examples:
///
/// - `quantum_volume`
/// - `randomized_benchmarking`
/// - `xeb`
/// - `vqe`
/// - `logical_error_rate`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BenchmarkId(String);

impl BenchmarkId {
    /// Creates a validated benchmark identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, BenchmarkError> {
        let value = value.into();

        validate_identifier(&value)?;

        Ok(Self(value))
    }

    /// Returns the identifier as a string slice.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its owned string.
    #[inline]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for BenchmarkId {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for BenchmarkId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Benchmark version
// =============================================================================

/// Semantic version of an individual benchmark protocol.
///
/// This version belongs to the benchmark implementation, not to the global
/// benchmarking contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BenchmarkVersion {
    /// Major version.
    pub major: u16,

    /// Minor version.
    pub minor: u16,

    /// Patch version.
    pub patch: u16,
}

impl BenchmarkVersion {
    /// Creates a semantic benchmark version.
    #[inline]
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the version as `(major, minor, patch)`.
    #[inline]
    pub const fn components(self) -> (u16, u16, u16) {
        (self.major, self.minor, self.patch)
    }
}

impl fmt::Display for BenchmarkVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

// =============================================================================
// Benchmark category
// =============================================================================

/// Broad benchmark domain.
///
/// This is intentionally broader than individual protocol names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BenchmarkCategory {
    /// Device/gate/readout characterization.
    Device,

    /// End-to-end computational workloads.
    Computation,

    /// System/compiler/runtime performance.
    System,

    /// Width/depth/volume scaling.
    Scaling,

    /// Error correction and logical computation.
    FaultTolerance,

    /// Application-level quantum workloads.
    Application,

    /// Hybrid quantum-classical workloads.
    Hybrid,

    /// Analog quantum workloads.
    Analog,

    /// Quantum annealing workloads.
    Annealing,

    /// Sampling workloads.
    Sampling,

    /// General/custom benchmark supplied by a user.
    Custom,
}

impl BenchmarkCategory {
    /// Returns a stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Device => "device",
            Self::Computation => "computation",
            Self::System => "system",
            Self::Scaling => "scaling",
            Self::FaultTolerance => "fault_tolerance",
            Self::Application => "application",
            Self::Hybrid => "hybrid",
            Self::Analog => "analog",
            Self::Annealing => "annealing",
            Self::Sampling => "sampling",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for BenchmarkCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Benchmark lifecycle
// =============================================================================

/// Lifecycle phase of a benchmark execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BenchmarkPhase {
    /// No lifecycle operation has started.
    Created,

    /// Configuration validation is running.
    Validating,

    /// Benchmark workloads are being generated.
    Generating,

    /// Generated workloads are being executed.
    Executing,

    /// Raw observations are being analyzed.
    Analyzing,

    /// Final result is being finalized.
    Finalizing,

    /// Benchmark completed successfully.
    Completed,

    /// Benchmark was cancelled.
    Cancelled,

    /// Benchmark failed.
    Failed,
}

impl BenchmarkPhase {
    /// Returns a stable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Validating => "validating",
            Self::Generating => "generating",
            Self::Executing => "executing",
            Self::Analyzing => "analyzing",
            Self::Finalizing => "finalizing",
            Self::Completed => "completed",
            Self::Cancelled => "cancelled",
            Self::Failed => "failed",
        }
    }
}

// =============================================================================
// Benchmark metadata
// =============================================================================

/// Immutable metadata describing a benchmark implementation.
///
/// Protocol implementations should construct this once and return a stable
/// reference from [`Benchmark::metadata`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BenchmarkMetadata {
    /// Stable benchmark identifier.
    pub id: BenchmarkId,

    /// Protocol implementation version.
    pub version: BenchmarkVersion,

    /// Broad benchmark category.
    pub category: BenchmarkCategory,

    /// Human-readable name.
    pub name: String,

    /// Human-readable description.
    pub description: String,

    /// Whether deterministic generation is supported.
    pub deterministic_generation: bool,

    /// Whether the benchmark can be analyzed without executing it again.
    pub offline_analysis: bool,

    /// Whether the benchmark can run against simulators.
    pub simulator_supported: bool,

    /// Whether the benchmark can run against physical hardware.
    pub hardware_supported: bool,
}

impl BenchmarkMetadata {
    /// Creates validated benchmark metadata.
    pub fn new(
        id: BenchmarkId,
        version: BenchmarkVersion,
        category: BenchmarkCategory,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Result<Self, BenchmarkError> {
        let name = name.into();
        let description = description.into();

        if name.trim().is_empty() {
            return Err(BenchmarkError::InvalidConfiguration {
                field: "benchmark.name".to_owned(),
                reason: "benchmark name must not be empty".to_owned(),
            });
        }

        if description.trim().is_empty() {
            return Err(BenchmarkError::InvalidConfiguration {
                field: "benchmark.description".to_owned(),
                reason: "benchmark description must not be empty".to_owned(),
            });
        }

        Ok(Self {
            id,
            version,
            category,
            name,
            description,
            deterministic_generation: false,
            offline_analysis: false,
            simulator_supported: false,
            hardware_supported: false,
        })
    }

    /// Marks the benchmark as supporting deterministic generation.
    #[must_use]
    pub fn with_deterministic_generation(mut self, supported: bool) -> Self {
        self.deterministic_generation = supported;
        self
    }

    /// Marks the benchmark as supporting offline analysis.
    #[must_use]
    pub fn with_offline_analysis(mut self, supported: bool) -> Self {
        self.offline_analysis = supported;
        self
    }

    /// Marks simulator support.
    #[must_use]
    pub fn with_simulator_support(mut self, supported: bool) -> Self {
        self.simulator_supported = supported;
        self
    }

    /// Marks physical-hardware support.
    #[must_use]
    pub fn with_hardware_support(mut self, supported: bool) -> Self {
        self.hardware_supported = supported;
        self
    }
}

// =============================================================================
// Lifecycle diagnostics
// =============================================================================

/// A bounded lifecycle event.
///
/// This is deliberately small and suitable for attaching to structured
/// diagnostics without creating an unbounded log in memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BenchmarkLifecycleEvent {
    /// Lifecycle phase.
    pub phase: BenchmarkPhase,

    /// Monotonic elapsed time from the beginning of the run.
    pub elapsed: Duration,
}

/// Bounded lifecycle information for one benchmark invocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BenchmarkLifecycle {
    phase: BenchmarkPhase,
    events: Vec<BenchmarkLifecycleEvent>,
}

impl BenchmarkLifecycle {
    /// Creates a new lifecycle state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            phase: BenchmarkPhase::Created,
            events: Vec::with_capacity(MAX_LIFECYCLE_EVENTS),
        }
    }

    /// Returns the current phase.
    #[inline]
    pub const fn phase(&self) -> BenchmarkPhase {
        self.phase
    }

    /// Records a phase transition.
    ///
    /// The caller supplies elapsed time so this structure remains independent
    /// from any clock implementation.
    pub fn transition(&mut self, phase: BenchmarkPhase, elapsed: Duration) {
        self.phase = phase;

        if self.events.len() < MAX_LIFECYCLE_EVENTS {
            self.events
                .push(BenchmarkLifecycleEvent { phase, elapsed });
        }
    }

    /// Returns the recorded lifecycle events.
    #[inline]
    pub fn events(&self) -> &[BenchmarkLifecycleEvent] {
        &self.events
    }
}

impl Default for BenchmarkLifecycle {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Benchmark execution outcome
// =============================================================================

/// Structured outcome of one benchmark execution.
///
/// The successful result itself remains owned by `BenchmarkResult`; this
/// wrapper adds lifecycle information without changing the scientific result
/// schema.
#[derive(Debug)]
pub struct BenchmarkRun {
    /// Final benchmark result.
    pub result: BenchmarkResult,

    /// Lifecycle information collected during execution.
    pub lifecycle: BenchmarkLifecycle,
}

impl BenchmarkRun {
    /// Creates a completed run.
    #[must_use]
    pub fn completed(result: BenchmarkResult, lifecycle: BenchmarkLifecycle) -> Self {
        Self {
            result,
            lifecycle,
        }
    }
}

// =============================================================================
// Benchmark trait
// =============================================================================

/// Stable production contract implemented by every Zamani benchmark.
///
/// # Object safety
///
/// This trait intentionally uses no associated types and no generic methods,
/// allowing it to be stored as:
///
/// ```text
/// Box<dyn Benchmark>
/// Arc<dyn Benchmark>
/// &dyn Benchmark
/// ```
///
/// That is required by the benchmark registry and Zamani's future runtime
/// integration.
///
/// # Separation of responsibilities
///
/// Implementations should follow this strict separation:
///
/// ```text
/// validate()
///     ↓
/// generate()
///     ↓
/// execute()       [generic orchestration; executor owns backend interaction]
///     ↓
/// analyze()
/// ```
///
/// Protocol implementations must not bypass the executor to communicate
/// directly with a backend.
pub trait Benchmark: Send + Sync {
    /// Returns immutable benchmark metadata.
    fn metadata(&self) -> &BenchmarkMetadata;

    /// Validates configuration against this benchmark's protocol requirements.
    ///
    /// This method MUST:
    ///
    /// - reject invalid values;
    /// - reject unsupported configurations;
    /// - reject resource requests exceeding configured limits;
    /// - reject incompatible execution requirements;
    /// - perform no external execution;
    /// - perform no irreversible mutation.
    fn validate(&self, config: &BenchmarkConfig) -> Result<(), BenchmarkError>;

    /// Generates a benchmark experiment.
    ///
    /// Generation MUST be deterministic when the configuration contains a
    /// deterministic seed and the benchmark advertises deterministic
    /// generation.
    ///
    /// Generation must not communicate with hardware.
    fn generate(
        &self,
        config: &BenchmarkConfig,
    ) -> Result<BenchmarkExperiment, BenchmarkError>;

    /// Executes an already-generated experiment through the supplied executor.
    ///
    /// The benchmark implementation does not own backend communication.
    ///
    /// The executor is responsible for:
    ///
    /// - backend capability negotiation;
    /// - submission;
    /// - batching;
    /// - timeout;
    /// - cancellation;
    /// - transport failures;
    /// - backend normalization.
    fn execute(
        &self,
        experiment: &BenchmarkExperiment,
        executor: &dyn BenchmarkExecutor,
    ) -> Result<BenchmarkObservationSet, BenchmarkError> {
        executor.execute(experiment)
    }

    /// Analyzes observations and produces the universal benchmark result.
    ///
    /// This MUST be a pure analysis operation from the benchmark's
    /// perspective: no backend execution, no hidden network access, and no
    /// mutation of global state.
    fn analyze(
        &self,
        config: &BenchmarkConfig,
        experiment: &BenchmarkExperiment,
        observations: &BenchmarkObservationSet,
    ) -> Result<BenchmarkResult, BenchmarkError>;

    /// Runs the complete benchmark lifecycle.
    ///
    /// This is the canonical high-level API for callers.
    ///
    /// The method deliberately performs generation before executor access,
    /// allowing configuration failures and generation failures to occur
    /// without contacting hardware.
    fn run(
        &self,
        config: &BenchmarkConfig,
        executor: &dyn BenchmarkExecutor,
    ) -> Result<BenchmarkRun, BenchmarkError> {
        let started = Instant::now();
        let mut lifecycle = BenchmarkLifecycle::new();

        lifecycle.transition(BenchmarkPhase::Validating, started.elapsed());

        self.validate(config)?;

        lifecycle.transition(BenchmarkPhase::Generating, started.elapsed());

        let experiment = self.generate(config)?;

        lifecycle.transition(BenchmarkPhase::Executing, started.elapsed());

        let observations = self.execute(&experiment, executor)?;

        lifecycle.transition(BenchmarkPhase::Analyzing, started.elapsed());

        let result = self.analyze(config, &experiment, &observations)?;

        lifecycle.transition(BenchmarkPhase::Finalizing, started.elapsed());

        validate_result_identity(self.metadata(), &result)?;

        lifecycle.transition(BenchmarkPhase::Completed, started.elapsed());

        Ok(BenchmarkRun::completed(result, lifecycle))
    }

    /// Performs analysis on previously captured observations.
    ///
    /// This is essential for scientific reproducibility:
    ///
    /// ```text
    /// hardware execution
    ///       ↓
    /// persisted observations
    ///       ↓
    /// later analysis
    ///       ↓
    /// BenchmarkResult
    /// ```
    ///
    /// A benchmark implementation that does not support offline analysis
    /// should return `BenchmarkError::UnsupportedOperation`.
    fn analyze_existing(
        &self,
        config: &BenchmarkConfig,
        experiment: &BenchmarkExperiment,
        observations: &BenchmarkObservationSet,
    ) -> Result<BenchmarkResult, BenchmarkError> {
        if !self.metadata().offline_analysis {
            return Err(BenchmarkError::UnsupportedOperation {
                operation: "offline benchmark analysis".to_owned(),
                benchmark: self.metadata().id.as_str().to_owned(),
            });
        }

        self.validate(config)?;
        self.analyze(config, experiment, observations)
    }

    /// Returns whether the benchmark can run without a physical backend.
    #[inline]
    fn supports_simulator(&self) -> bool {
        self.metadata().simulator_supported
    }

    /// Returns whether the benchmark supports physical hardware.
    #[inline]
    fn supports_hardware(&self) -> bool {
        self.metadata().hardware_supported
    }

    /// Returns whether deterministic generation is guaranteed when configured
    /// with a deterministic seed.
    #[inline]
    fn supports_deterministic_generation(&self) -> bool {
        self.metadata().deterministic_generation
    }

    /// Returns whether observations can be analyzed independently of
    /// execution.
    #[inline]
    fn supports_offline_analysis(&self) -> bool {
        self.metadata().offline_analysis
    }
}

// =============================================================================
// Benchmark runner
// =============================================================================

/// Reusable benchmark runner.
///
/// This is intentionally stateless. It does not cache benchmark results,
/// maintain global executors, or hold backend connections.
///
/// That makes it safe to use from:
///
/// - CLI tools;
/// - tests;
/// - the Zamani runtime;
/// - language-level benchmark commands;
/// - CI;
/// - scheduled hardware experiments.
#[derive(Debug, Default, Clone, Copy)]
pub struct BenchmarkRunner;

impl BenchmarkRunner {
    /// Creates a stateless benchmark runner.
    #[inline]
    pub const fn new() -> Self {
        Self
    }

    /// Runs one benchmark.
    #[inline]
    pub fn run(
        &self,
        benchmark: &dyn Benchmark,
        config: &BenchmarkConfig,
        executor: &dyn BenchmarkExecutor,
    ) -> Result<BenchmarkRun, BenchmarkError> {
        benchmark.run(config, executor)
    }

    /// Validates a benchmark configuration without generating or executing
    /// anything.
    #[inline]
    pub fn validate(
        &self,
        benchmark: &dyn Benchmark,
        config: &BenchmarkConfig,
    ) -> Result<(), BenchmarkError> {
        benchmark.validate(config)
    }

    /// Generates a benchmark experiment without executing it.
    #[inline]
    pub fn generate(
        &self,
        benchmark: &dyn Benchmark,
        config: &BenchmarkConfig,
    ) -> Result<BenchmarkExperiment, BenchmarkError> {
        benchmark.validate(config)?;
        benchmark.generate(config)
    }

    /// Analyzes previously captured observations.
    #[inline]
    pub fn analyze_existing(
        &self,
        benchmark: &dyn Benchmark,
        config: &BenchmarkConfig,
        experiment: &BenchmarkExperiment,
        observations: &BenchmarkObservationSet,
    ) -> Result<BenchmarkResult, BenchmarkError> {
        benchmark.analyze_existing(config, experiment, observations)
    }
}

// =============================================================================
// Internal validation helpers
// =============================================================================

/// Validates a benchmark identifier.
///
/// The identifier deliberately follows a conservative ASCII format so that
/// it can safely become:
///
/// - a registry key;
/// - a JSON field;
/// - a filesystem component;
/// - a CLI identifier;
/// - a Zamani-language benchmark identifier.
fn validate_identifier(value: &str) -> Result<(), BenchmarkError> {
    if value.is_empty() {
        return Err(BenchmarkError::InvalidConfiguration {
            field: "benchmark.id".to_owned(),
            reason: "benchmark identifier must not be empty".to_owned(),
        });
    }

    if value.len() > 128 {
        return Err(BenchmarkError::InvalidConfiguration {
            field: "benchmark.id".to_owned(),
            reason: "benchmark identifier exceeds the 128-byte limit".to_owned(),
        });
    }

    let bytes = value.as_bytes();

    for (index, byte) in bytes.iter().copied().enumerate() {
        let valid = byte.is_ascii_lowercase()
            || byte.is_ascii_digit()
            || byte == b'_'
            || byte == b'-'
            || byte == b'.';

        if !valid {
            return Err(BenchmarkError::InvalidConfiguration {
                field: "benchmark.id".to_owned(),
                reason: format!(
                    "invalid character at byte {}: benchmark identifiers may \
                     contain only lowercase ASCII letters, digits, '_', '-' \
                     and '.'",
                    index
                ),
            });
        }
    }

    Ok(())
}

/// Ensures that the analyzed result still belongs to the benchmark that
/// produced it.
///
/// This protects against a subtle but serious production error: passing the
/// observations from benchmark A to benchmark B and accidentally accepting a
/// result whose protocol identity does not match.
fn validate_result_identity(
    metadata: &BenchmarkMetadata,
    result: &BenchmarkResult,
) -> Result<(), BenchmarkError> {
    if result.benchmark_id() != metadata.id.as_str() {
        return Err(BenchmarkError::ResultIdentityMismatch {
            expected: metadata.id.as_str().to_owned(),
            actual: result.benchmark_id().to_owned(),
        });
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn benchmark_identifier_accepts_canonical_identifier() {
        let id = BenchmarkId::new("quantum_volume").unwrap();

        assert_eq!(id.as_str(), "quantum_volume");
    }

    #[test]
    fn benchmark_identifier_accepts_namespaced_identifier() {
        let id = BenchmarkId::new("zamani.qc.quantum_volume").unwrap();

        assert_eq!(id.as_str(), "zamani.qc.quantum_volume");
    }

    #[test]
    fn benchmark_identifier_rejects_empty_identifier() {
        assert!(BenchmarkId::new("").is_err());
    }

    #[test]
    fn benchmark_identifier_rejects_uppercase_identifier() {
        assert!(BenchmarkId::new("QuantumVolume").is_err());
    }

    #[test]
    fn benchmark_identifier_rejects_whitespace() {
        assert!(BenchmarkId::new("quantum volume").is_err());
    }

    #[test]
    fn benchmark_identifier_rejects_invalid_symbols() {
        assert!(BenchmarkId::new("quantum/volume").is_err());
    }

    #[test]
    fn benchmark_version_is_stable() {
        let version = BenchmarkVersion::new(1, 2, 3);

        assert_eq!(version.components(), (1, 2, 3));
        assert_eq!(version.to_string(), "1.2.3");
    }

    #[test]
    fn benchmark_category_has_stable_identifier() {
        assert_eq!(
            BenchmarkCategory::FaultTolerance.as_str(),
            "fault_tolerance"
        );
        assert_eq!(
            BenchmarkCategory::Application.as_str(),
            "application"
        );
    }

    #[test]
    fn lifecycle_starts_created() {
        let lifecycle = BenchmarkLifecycle::new();

        assert_eq!(lifecycle.phase(), BenchmarkPhase::Created);
        assert!(lifecycle.events().is_empty());
    }

    #[test]
    fn lifecycle_records_bounded_events() {
        let mut lifecycle = BenchmarkLifecycle::new();

        for _ in 0..64 {
            lifecycle.transition(
                BenchmarkPhase::Validating,
                Duration::from_millis(1),
            );
        }

        assert_eq!(lifecycle.phase(), BenchmarkPhase::Validating);
        assert_eq!(
            lifecycle.events().len(),
            MAX_LIFECYCLE_EVENTS
        );
    }

    #[test]
    fn metadata_rejects_empty_name() {
        let id = BenchmarkId::new("test").unwrap();

        let result = BenchmarkMetadata::new(
            id,
            BenchmarkVersion::new(1, 0, 0),
            BenchmarkCategory::Custom,
            "",
            "test benchmark",
        );

        assert!(result.is_err());
    }

    #[test]
    fn metadata_rejects_empty_description() {
        let id = BenchmarkId::new("test").unwrap();

        let result = BenchmarkMetadata::new(
            id,
            BenchmarkVersion::new(1, 0, 0),
            BenchmarkCategory::Custom,
            "Test",
            "",
        );

        assert!(result.is_err());
    }

    #[test]
    fn metadata_capability_flags_are_explicit() {
        let id = BenchmarkId::new("test").unwrap();

        let metadata = BenchmarkMetadata::new(
            id,
            BenchmarkVersion::new(1, 0, 0),
            BenchmarkCategory::Custom,
            "Test",
            "Test benchmark",
        )
        .unwrap()
        .with_deterministic_generation(true)
        .with_offline_analysis(true)
        .with_simulator_support(true)
        .with_hardware_support(true);

        assert!(metadata.deterministic_generation);
        assert!(metadata.offline_analysis);
        assert!(metadata.simulator_supported);
        assert!(metadata.hardware_supported);
    }

    #[test]
    fn contract_version_is_stable() {
        assert_eq!(
            BENCHMARK_CONTRACT_VERSION,
            "1.0.0"
        );

        assert_eq!(
            BENCHMARK_COMPONENT_ID,
            "zamani.quantum.benchmark"
        );
    }
}