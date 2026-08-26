//! Zamani Quantum Benchmarking — Production Benchmark Configuration
//!
//! This module defines the stable, backend-independent configuration contract
//! shared by every quantum benchmark in Zamani.
//!
//! # Architectural role
//!
//! `config.rs` is deliberately a foundation module. It owns:
//!
//! - benchmark identity and versioning;
//! - workload-size ranges;
//! - circuit/depth ranges;
//! - shot/circuit limits;
//! - deterministic random seeds;
//! - statistical configuration;
//! - execution policy;
//! - backend selection;
//! - compiler configuration;
//! - validation policy;
//! - reporting policy;
//! - resource/time safety limits;
//! - user-defined benchmark metadata.
//!
//! It does NOT own:
//!
//! - circuit generation;
//! - circuit execution;
//! - backend implementations;
//! - statistical algorithms;
//! - metrics;
//! - benchmark results;
//! - benchmark protocols;
//! - Quantum IR.
//!
//! Those responsibilities belong to the downstream modules:
//!
//! ```text
//! core::config
//!      │
//!      ├──────────────► generators
//!      ├──────────────► execution
//!      ├──────────────► statistics
//!      ├──────────────► validation
//!      ├──────────────► protocols
//!      ├──────────────► applications
//!      ├──────────────► qec
//!      └──────────────► reporting
//! ```
//!
//! The configuration is intentionally backend-neutral. A benchmark may target
//! a CPU simulator, GPU simulator, emulator, QPU, analog system, annealer,
//! logical-qubit backend, or another future execution technology.
//!
//! # Stability contract
//!
//! Benchmark configuration is part of the reproducibility boundary.
//!
//! A configuration must therefore be:
//!
//! - deterministic;
//! - explicitly validated;
//! - bounded;
//! - serializable when the `full` feature is enabled;
//! - free from process-global mutable state;
//! - independent of wall-clock time;
//! - independent of hidden random-number generators.
//!
//! Configuration fingerprints are intentionally NOT calculated here. That
//! responsibility belongs to `core::reproducibility`, which can hash this
//! complete configuration representation without creating a dependency cycle.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97.1, Rust 2021.
//!
//! No nightly features are required.
//!
//! # Serialization
//!
//! `serde` derives are enabled only under Zamani's existing `full` feature,
//! matching the convention already used by the quantum benchmarking
//! foundation modules.
//!
//! # Security/resource policy
//!
//! Benchmark configuration is an untrusted input boundary whenever it is
//! eventually loaded from a Zamani program, JSON/TOML document, CLI, remote
//! benchmark request, or CI configuration.
//!
//! Therefore all externally controllable resource dimensions are explicitly
//! bounded before execution.

use core::fmt;
use std::collections::BTreeMap;
use std::str::FromStr;

#[cfg(feature = "full")]
use serde::{Deserialize, Serialize};

// =============================================================================
// Public constants
// =============================================================================

/// Stable schema identifier for benchmark configurations.
pub const BENCHMARK_CONFIG_SCHEMA: &str = "zamani.quantum.benchmark.config";

/// Current configuration schema version.
///
/// This is independent of the Cargo package version. Schema versions identify
/// serialized-data compatibility, not compiler releases.
pub const BENCHMARK_CONFIG_SCHEMA_VERSION: u32 = 1;

/// Default benchmark confidence level.
///
/// Individual protocols may define stricter protocol-specific requirements,
/// but they must override this explicitly rather than silently changing the
/// global meaning.
pub const DEFAULT_CONFIDENCE_LEVEL: f64 = 0.95;

/// Default number of shots for an executable benchmark.
pub const DEFAULT_SHOTS: usize = 1_000;

/// Default number of independently generated circuit instances.
pub const DEFAULT_CIRCUITS: usize = 1;

/// Default benchmark timeout in milliseconds.
///
/// This is a safety limit, not a promise that every backend supports this
/// duration.
pub const DEFAULT_TIMEOUT_MS: u64 = 300_000;

/// Default maximum number of benchmark circuits that one configuration may
/// request.
pub const DEFAULT_MAX_CIRCUITS: usize = 100_000;

/// Default maximum shots per circuit.
pub const DEFAULT_MAX_SHOTS_PER_CIRCUIT: usize = 10_000_000;

/// Default maximum total shots across the complete benchmark.
pub const DEFAULT_MAX_TOTAL_SHOTS: u64 = 1_000_000_000;

/// Default maximum generated circuits retained by one experiment.
pub const DEFAULT_MAX_GENERATED_CIRCUITS: usize = 100_000;

/// Default maximum benchmark metadata entries.
pub const DEFAULT_MAX_METADATA_ENTRIES: usize = 256;

/// Default maximum bytes for one benchmark identifier.
pub const MAX_BENCHMARK_ID_BYTES: usize = 128;

/// Default maximum bytes for a backend identifier.
pub const MAX_BACKEND_ID_BYTES: usize = 256;

/// Default maximum bytes for a configuration metadata key.
pub const MAX_METADATA_KEY_BYTES: usize = 128;

/// Default maximum bytes for a configuration metadata value.
pub const MAX_METADATA_VALUE_BYTES: usize = 4096;

/// Maximum number of explicit qubit sizes in one benchmark configuration.
pub const MAX_EXPLICIT_SIZES: usize = 4096;

/// Maximum number of explicit depth values in one benchmark configuration.
pub const MAX_EXPLICIT_DEPTHS: usize = 4096;

// =============================================================================
// Benchmark identity
// =============================================================================

/// Stable identity of a benchmark configuration.
///
/// The identifier is deliberately separate from the Rust type name. This
/// allows the Zamani language, JSON reports, CI systems, and external tools
/// to refer to benchmarks without knowing Rust implementation details.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "full", derive(Serialize, Deserialize))]
pub struct BenchmarkIdentity {
    /// Stable machine-readable benchmark identifier.
    pub id: String,

    /// Benchmark protocol/schema version.
    pub version: u32,
}

impl BenchmarkIdentity {
    /// Creates a validated benchmark identity.
    pub fn new<S: Into<String>>(id: S, version: u32) -> Result<Self, ConfigError> {
        let id = id.into();

        validate_identifier(
            &id,
            MAX_BENCHMARK_ID_BYTES,
            "benchmark identifier",
        )?;

        if version == 0 {
            return Err(ConfigError::InvalidVersion);
        }

        Ok(Self { id, version })
    }

    /// Returns the stable benchmark identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the benchmark protocol version.
    #[must_use]
    pub fn version(&self) -> u32 {
        self.version
    }
}

// =============================================================================
// Workload ranges
// =============================================================================

/// Inclusive integer range used for benchmark workload sizes.
///
/// This is intentionally not `std::ops::RangeInclusive` because benchmark
/// configurations need stable serialization, validation, and deterministic
/// expansion semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "full", derive(Serialize, Deserialize))]
pub struct InclusiveRange {
    /// Smallest value included in the range.
    pub start: usize,

    /// Largest value included in the range.
    pub end: usize,

    /// Increment between successive values.
    pub step: usize,
}

impl InclusiveRange {
    /// Creates a validated inclusive range.
    pub fn new(
        start: usize,
        end: usize,
        step: usize,
    ) -> Result<Self, ConfigError> {
        if start == 0 {
            return Err(ConfigError::ZeroRangeStart);
        }

        if end < start {
            return Err(ConfigError::InvalidRange {
                start,
                end,
            });
        }

        if step == 0 {
            return Err(ConfigError::ZeroRangeStep);
        }

        let range = Self { start, end, step };

        // Validate expansion cardinality now, rather than allowing a future
        // consumer to accidentally allocate an enormous vector.
        range.validate_cardinality()?;

        Ok(range)
    }

    /// Creates a single-value range.
    pub fn single(value: usize) -> Result<Self, ConfigError> {
        Self::new(value, value, 1)
    }

    /// Returns the number of values represented by the range.
    pub fn len(&self) -> usize {
        if self.start > self.end {
            return 0;
        }

        ((self.end - self.start) / self.step) + 1
    }

    /// Returns whether the range contains no values.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.start > self.end
    }

    /// Returns whether a value belongs to this range.
    #[must_use]
    pub fn contains(&self, value: usize) -> bool {
        value >= self.start
            && value <= self.end
            && (value - self.start) % self.step == 0
    }

    /// Expands the range into a bounded vector.
    pub fn values(&self) -> Result<Vec<usize>, ConfigError> {
        self.validate_cardinality()?;

        let mut values = Vec::with_capacity(self.len());

        let mut value = self.start;

        loop {
            values.push(value);

            if value >= self.end {
                break;
            }

            value = value
                .checked_add(self.step)
                .ok_or(ConfigError::RangeOverflow)?;
        }

        Ok(values)
    }

    fn validate_cardinality(&self) -> Result<(), ConfigError> {
        if self.len() > MAX_EXPLICIT_SIZES {
            return Err(ConfigError::RangeTooLarge {
                requested: self.len(),
                maximum: MAX_EXPLICIT_SIZES,
            });
        }

        Ok(())
    }
}

/// Dimension specification for a benchmark.
///
/// A benchmark may use an automatically selected sequence or an explicit
/// bounded sequence.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "full", derive(Serialize, Deserialize))]
pub enum DimensionRange {
    /// Backend/protocol determines appropriate values subject to safety limits.
    Auto,

    /// One inclusive arithmetic progression.
    Range(InclusiveRange),

    /// Explicit values.
    ///
    /// Values are required to be non-zero and strictly increasing. Keeping
    /// them canonical here ensures deterministic configuration fingerprints.
    Explicit(Vec<usize>),
}

impl Default for DimensionRange {
    fn default() -> Self {
        Self::Auto
    }
}

impl DimensionRange {
    /// Creates an inclusive range.
    pub fn range(
        start: usize,
        end: usize,
        step: usize,
    ) -> Result<Self, ConfigError> {
        Ok(Self::Range(InclusiveRange::new(start, end, step)?))
    }

    /// Creates an explicit dimension list.
    pub fn explicit(values: Vec<usize>) -> Result<Self, ConfigError> {
        validate_explicit_values(&values)?;

        if values.len() > MAX_EXPLICIT_SIZES {
            return Err(ConfigError::TooManyExplicitValues {
                requested: values.len(),
                maximum: MAX_EXPLICIT_SIZES,
            });
        }

        Ok(Self::Explicit(values))
    }

    /// Returns the explicit values represented by the specification.
    ///
    /// `Auto` remains unresolved because selecting automatic sizes belongs to
    /// the protocol/generator layer.
    pub fn values(&self) -> Result<Option<Vec<usize>>, ConfigError> {
        match self {
            Self::Auto => Ok(None),
            Self::Range(range) => Ok(Some(range.values()?)),
            Self::Explicit(values) => Ok(Some(values.clone())),
        }
    }

    /// Returns whether the dimension is automatic.
    #[must_use]
    pub fn is_auto(&self) -> bool {
        matches!(self, Self::Auto)
    }
}

// =============================================================================
// Execution policy
// =============================================================================

/// Execution mode requested by the benchmark.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "full", derive(Serialize, Deserialize))]
pub enum ExecutionMode {
    /// Do not execute circuits. Generate/validate/analyze only.
    ///
    /// Useful for protocol planning, static resource analysis, and CI tests.
    PlanOnly,

    /// Execute against a deterministic simulator.
    Simulator,

    /// Execute against a hardware-emulating backend.
    Emulator,

    /// Execute against a physical quantum processing unit.
    Qpu,

    /// Permit the backend registry to choose a compatible execution target.
    Auto,
}

impl Default for ExecutionMode {
    fn default() -> Self {
        Self::Auto
    }
}

/// Policy controlling whether a benchmark may perform external execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "full", derive(Serialize, Deserialize))]
pub enum ExecutionPermission {
    /// Configuration may only be planned/validated.
    PlanOnly,

    /// Simulator/emulator execution is permitted, but physical QPU execution
    /// requires an explicit configuration.
    NonQpuOnly,

    /// Physical QPU execution is permitted when the backend is compatible.
    AllowQpu,
}

impl Default for ExecutionPermission {
    fn default() -> Self {
        Self::NonQpuOnly
    }
}

// =============================================================================
// Backend selection
// =============================================================================

/// Backend selection policy.
///
/// This type does not contain a backend implementation. It only describes
/// which execution target the execution/registry layer should resolve.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "full", derive(Serialize, Deserialize))]
pub enum BackendSelection {
    /// Let the backend registry select a compatible backend.
    Auto,

    /// Require a specific registered backend identifier.
    Named(String),

    /// Require a backend of a particular execution mode.
    Mode(ExecutionMode),
}

impl Default for BackendSelection {
    fn default() -> Self {
        Self::Auto
    }
}

impl BackendSelection {
    /// Creates a specific backend selection.
    pub fn named<S: Into<String>>(id: S) -> Result<Self, ConfigError> {
        let id = id.into();

        validate_identifier(&id, MAX_BACKEND_ID_BYTES, "backend identifier")?;

        Ok(Self::Named(id))
    }

    /// Returns a named backend if one was explicitly selected.
    #[must_use]
    pub fn backend_id(&self) -> Option<&str> {
        match self {
            Self::Named(id) => Some(id.as_str()),
            Self::Auto | Self::Mode(_) => None,
        }
    }
}

// =============================================================================
// Compiler policy
// =============================================================================

/// Optimization policy applied before benchmark execution.
///
/// Benchmarking must record this configuration because compiler optimization
/// can materially alter circuit depth, gate count, routing, and fidelity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "full", derive(Serialize, Deserialize))]
pub enum OptimizationLevel {
    /// No optional optimization.
    None,

    /// Lightweight local optimization.
    Basic,

    /// Standard production optimization.
    Standard,

    /// Aggressive optimization. May materially change circuit structure.
    Aggressive,
}

impl Default for OptimizationLevel {
    fn default() -> Self {
        Self::Standard
    }
}

/// Routing policy requested for benchmark execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "full", derive(Serialize, Deserialize))]
pub enum RoutingPolicy {
    /// Do not perform routing; require a compatible circuit.
    Disabled,

    /// Let the routing subsystem select an appropriate method.
    Auto,

    /// Prefer minimum additional depth.
    MinimizeDepth,

    /// Prefer minimum added two-qubit operations.
    MinimizeTwoQubitGates,
}

impl Default for RoutingPolicy {
    fn default() -> Self {
        Self::Auto
    }
}

/// Scheduling policy requested for benchmark execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "full", derive(Serialize, Deserialize))]
pub enum SchedulingPolicy {
    /// Do not schedule beyond the circuit's existing order.
    Disabled,

    /// Let the scheduling subsystem select a compatible strategy.
    Auto,

    /// Prefer minimum total duration.
    MinimizeDuration,

    /// Prefer maximum parallelism.
    MaximizeParallelism,
}

impl Default for SchedulingPolicy {
    fn default() -> Self {
        Self::Auto
    }
}

/// Compiler configuration recorded with every benchmark experiment.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "full", derive(Serialize, Deserialize))]
pub struct CompilerConfig {
    /// Compiler optimization level.
    pub optimization: OptimizationLevel,

    /// Routing policy.
    pub routing: RoutingPolicy,

    /// Scheduling policy.
    pub scheduling: SchedulingPolicy,

    /// Whether parameter binding should happen before execution.
    pub bind_parameters: bool,

    /// Whether measurement operations may be optimized/reordered where
    /// semantics permit it.
    pub optimize_measurements: bool,

    /// Optional compiler profile identifier.
    ///
    /// This allows the benchmark system to distinguish future compiler
    /// profiles without depending on compiler implementation types.
    pub profile: Option<String>,
}

impl Default for CompilerConfig {
    fn default() -> Self {
        Self {
            optimization: OptimizationLevel::Standard,
            routing: RoutingPolicy::Auto,
            scheduling: SchedulingPolicy::Auto,
            bind_parameters: true,
            optimize_measurements: false,
            profile: None,
        }
    }
}

impl CompilerConfig {
    /// Validates compiler configuration.
    pub fn validate(&self) -> Result<(), ConfigError> {
        if let Some(profile) = &self.profile {
            validate_identifier(
                profile,
                MAX_BENCHMARK_ID_BYTES,
                "compiler profile",
            )?;
        }

        Ok(())
    }
}

// =============================================================================
// Statistical policy
// =============================================================================

/// Confidence interval method.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "full", derive(Serialize, Deserialize))]
pub enum ConfidenceMethod {
    /// Wilson score interval for binomial proportions.
    Wilson,

    /// Exact Clopper-Pearson binomial interval.
    ClopperPearson,

    /// Bootstrap confidence interval.
    Bootstrap,

    /// Normal approximation.
    ///
    /// Protocols should reject this method when its assumptions are not
    /// appropriate.
    Normal,
}

impl Default for ConfidenceMethod {
    fn default() -> Self {
        Self::Wilson
    }
}

/// Bootstrap strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "full", derive(Serialize, Deserialize))]
pub enum BootstrapMethod {
    /// Percentile bootstrap.
    Percentile,

    /// Basic bootstrap interval.
    Basic,

    /// Bias-corrected accelerated interval.
    Bca,
}

impl Default for BootstrapMethod {
    fn default() -> Self {
        Self::Percentile
    }
}

/// Statistical analysis configuration.
///
/// This structure describes statistical policy, not statistical results.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "full", derive(Serialize, Deserialize))]
pub struct StatisticsConfig {
    /// Confidence level in the open interval `(0, 1)`.
    pub confidence_level: f64,

    /// Confidence interval method.
    pub confidence_method: ConfidenceMethod,

    /// Bootstrap method.
    pub bootstrap_method: BootstrapMethod,

    /// Number of bootstrap resamples.
    pub bootstrap_samples: usize,

    /// Optional explicit statistical seed.
    ///
    /// If absent, the benchmark's master seed is used where deterministic
    /// resampling is possible.
    pub bootstrap_seed: Option<u64>,

    /// Minimum observations required for a statistical estimate.
    pub minimum_samples: usize,

    /// Whether statistically invalid fits are fatal.
    pub require_valid_fit: bool,

    /// Whether protocols may report estimates when confidence intervals are
    /// unavailable.
    pub allow_unbounded_estimates: bool,
}

impl Default for StatisticsConfig {
    fn default() -> Self {
        Self {
            confidence_level: DEFAULT_CONFIDENCE_LEVEL,
            confidence_method: ConfidenceMethod::Wilson,
            bootstrap_method: BootstrapMethod::Percentile,
            bootstrap_samples: 10_000,
            bootstrap_seed: None,
            minimum_samples: 1,
            require_valid_fit: true,
            allow_unbounded_estimates: false,
        }
    }
}

impl StatisticsConfig {
    /// Validates statistical configuration.
    pub fn validate(&self) -> Result<(), ConfigError> {
        if !self.confidence_level.is_finite()
            || !(0.0..1.0).contains(&self.confidence_level)
        {
            return Err(ConfigError::InvalidConfidenceLevel {
                value: self.confidence_level,
            });
        }

        if self.minimum_samples == 0 {
            return Err(ConfigError::InvalidMinimumSamples);
        }

        if matches!(
            self.confidence_method,
            ConfidenceMethod::Bootstrap
        ) && self.bootstrap_samples == 0
        {
            return Err(ConfigError::InvalidBootstrapSamples);
        }

        if self.bootstrap_samples > DEFAULT_MAX_CIRCUITS {
            return Err(ConfigError::BootstrapLimitExceeded {
                requested: self.bootstrap_samples,
                maximum: DEFAULT_MAX_CIRCUITS,
            });
        }

        Ok(())
    }
}

// =============================================================================
// Validation policy
// =============================================================================

/// Policy controlling benchmark validation strictness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "full", derive(Serialize, Deserialize))]
pub enum ValidationMode {
    /// Reject invalid or unsupported configuration before execution.
    Strict,

    /// Permit protocol-specific warnings where execution can still be safe.
    Permissive,

    /// Validate only structural constraints needed to construct a plan.
    Structural,
}

impl Default for ValidationMode {
    fn default() -> Self {
        Self::Strict
    }
}

/// Validation configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "full", derive(Serialize, Deserialize))]
pub struct ValidationConfig {
    /// Validation strictness.
    pub mode: ValidationMode,

    /// Require backend capability validation before submission.
    pub require_backend_capability_check: bool,

    /// Require all generated circuits to pass structural validation.
    pub require_circuit_validation: bool,

    /// Reject non-finite numerical observations during analysis.
    pub reject_non_finite_values: bool,

    /// Reject unsupported protocol/backend combinations before execution.
    pub reject_unsupported_backends: bool,

    /// Reject resource requests exceeding configured safety limits.
    pub enforce_resource_limits: bool,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            mode: ValidationMode::Strict,
            require_backend_capability_check: true,
            require_circuit_validation: true,
            reject_non_finite_values: true,
            reject_unsupported_backends: true,
            enforce_resource_limits: true,
        }
    }
}

// =============================================================================
// Resource safety
// =============================================================================

/// Hard resource safety limits for one benchmark configuration.
///
/// These are benchmark-level limits. Backend-specific limits are supplied by
/// the hardware/backend subsystem and checked later by execution validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "full", derive(Serialize, Deserialize))]
pub struct ResourceLimits {
    /// Maximum qubits permitted in one generated circuit.
    pub max_qubits: usize,

    /// Maximum circuit depth.
    pub max_depth: usize,

    /// Maximum operations in one circuit.
    pub max_operations: usize,

    /// Maximum shots per circuit.
    pub max_shots_per_circuit: usize,

    /// Maximum number of circuits in one experiment.
    pub max_circuits: usize,

    /// Maximum total shots across all circuits.
    pub max_total_shots: u64,

    /// Maximum number of statistical bootstrap samples.
    pub max_bootstrap_samples: usize,

    /// Maximum benchmark wall-clock duration.
    pub timeout_ms: u64,

    /// Maximum metadata entries.
    pub max_metadata_entries: usize,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_qubits: 1_024,
            max_depth: 1_000_000,
            max_operations: 10_000_000,
            max_shots_per_circuit: DEFAULT_MAX_SHOTS_PER_CIRCUIT,
            max_circuits: DEFAULT_MAX_CIRCUITS,
            max_total_shots: DEFAULT_MAX_TOTAL_SHOTS,
            max_bootstrap_samples: 100_000,
            timeout_ms: DEFAULT_TIMEOUT_MS,
            max_metadata_entries: DEFAULT_MAX_METADATA_ENTRIES,
        }
    }
}

impl ResourceLimits {
    /// Validates that all limits themselves are meaningful.
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.max_qubits == 0 {
            return Err(ConfigError::ZeroResourceLimit {
                field: "max_qubits",
            });
        }

        if self.max_depth == 0 {
            return Err(ConfigError::ZeroResourceLimit {
                field: "max_depth",
            });
        }

        if self.max_operations == 0 {
            return Err(ConfigError::ZeroResourceLimit {
                field: "max_operations",
            });
        }

        if self.max_shots_per_circuit == 0 {
            return Err(ConfigError::ZeroResourceLimit {
                field: "max_shots_per_circuit",
            });
        }

        if self.max_circuits == 0 {
            return Err(ConfigError::ZeroResourceLimit {
                field: "max_circuits",
            });
        }

        if self.max_total_shots == 0 {
            return Err(ConfigError::ZeroResourceLimit {
                field: "max_total_shots",
            });
        }

        if self.max_bootstrap_samples == 0 {
            return Err(ConfigError::ZeroResourceLimit {
                field: "max_bootstrap_samples",
            });
        }

        if self.timeout_ms == 0 {
            return Err(ConfigError::ZeroResourceLimit {
                field: "timeout_ms",
            });
        }

        if self.max_metadata_entries == 0 {
            return Err(ConfigError::ZeroResourceLimit {
                field: "max_metadata_entries",
            });
        }

        Ok(())
    }

    /// Returns whether a single circuit fits within the configured limits.
    #[must_use]
    pub fn allows_circuit(
        &self,
        qubits: usize,
        depth: usize,
        operations: usize,
        shots: usize,
    ) -> bool {
        qubits <= self.max_qubits
            && depth <= self.max_depth
            && operations <= self.max_operations
            && shots <= self.max_shots_per_circuit
    }

    /// Checks the aggregate shot budget without overflowing.
    pub fn validate_total_shots(
        &self,
        circuits: usize,
        shots_per_circuit: usize,
    ) -> Result<u64, ConfigError> {
        let circuits = u64::try_from(circuits)
            .map_err(|_| ConfigError::IntegerConversion)?;

        let shots = u64::try_from(shots_per_circuit)
            .map_err(|_| ConfigError::IntegerConversion)?;

        let total = circuits
            .checked_mul(shots)
            .ok_or(ConfigError::TotalShotsOverflow)?;

        if total > self.max_total_shots {
            return Err(ConfigError::TotalShotsExceeded {
                requested: total,
                maximum: self.max_total_shots,
            });
        }

        Ok(total)
    }
}

// =============================================================================
// Reporting policy
// =============================================================================

/// Output formats supported by the reporting layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "full", derive(Serialize, Deserialize))]
pub enum ReportFormat {
    /// Canonical machine-readable JSON.
    Json,

    /// CSV for tabular scientific analysis.
    Csv,

    /// Markdown for humans/GitHub/CI.
    Markdown,
}

impl Default for ReportFormat {
    fn default() -> Self {
        Self::Json
    }
}

/// Reporting configuration.
///
/// The reporter implementation lives in `benchmarking::reporting`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "full", derive(Serialize, Deserialize))]
pub struct ReportingConfig {
    /// Requested report format.
    pub format: ReportFormat,

    /// Whether raw observations should be included.
    pub include_observations: bool,

    /// Whether provenance should be included.
    pub include_provenance: bool,

    /// Whether warnings should be included.
    pub include_warnings: bool,

    /// Whether intermediate statistical information should be included.
    pub include_statistics: bool,

    /// Maximum serialized report size in bytes.
    pub max_report_bytes: usize,
}

impl Default for ReportingConfig {
    fn default() -> Self {
        Self {
            format: ReportFormat::Json,
            include_observations: true,
            include_provenance: true,
            include_warnings: true,
            include_statistics: true,
            max_report_bytes: 64 * 1024 * 1024,
        }
    }
}

impl ReportingConfig {
    /// Validates reporting limits.
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.max_report_bytes == 0 {
            return Err(ConfigError::ZeroResourceLimit {
                field: "max_report_bytes",
            });
        }

        Ok(())
    }
}

// =============================================================================
// Benchmark configuration
// =============================================================================

/// Complete backend-independent Zamani quantum benchmark configuration.
///
/// This is the central configuration contract for the entire benchmarking
/// subsystem.
///
/// Future protocol-specific configuration should be layered on top of this
/// type rather than duplicating these fields.
///
/// For example:
///
/// ```text
/// BenchmarkConfig
///      │
///      ├── Quantum Volume protocol options
///      ├── RB protocol options
///      ├── XEB protocol options
///      ├── QAOA application options
///      └── QEC options
/// ```
///
/// Protocol-specific extensions should be represented by their own protocol
/// configuration types and composed by `BenchmarkSpec`/`Benchmark` rather
/// than added here every time a new benchmark is introduced.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "full", derive(Serialize, Deserialize))]
pub struct BenchmarkConfig {
    /// Serialized schema identifier.
    pub schema: String,

    /// Serialized schema version.
    pub schema_version: u32,

    /// Benchmark identity.
    pub benchmark: BenchmarkIdentity,

    /// Requested logical/workload qubit dimension.
    pub qubits: DimensionRange,

    /// Requested circuit-depth dimension.
    pub depth: DimensionRange,

    /// Number of independent circuits/instances.
    pub circuits: usize,

    /// Number of shots per circuit.
    pub shots: usize,

    /// Master deterministic benchmark seed.
    pub seed: u64,

    /// Execution mode.
    pub execution_mode: ExecutionMode,

    /// External execution permission.
    pub execution_permission: ExecutionPermission,

    /// Backend selection policy.
    pub backend: BackendSelection,

    /// Compiler configuration.
    pub compiler: CompilerConfig,

    /// Statistical configuration.
    pub statistics: StatisticsConfig,

    /// Validation configuration.
    pub validation: ValidationConfig,

    /// Resource safety limits.
    pub limits: ResourceLimits,

    /// Reporting configuration.
    pub reporting: ReportingConfig,

    /// Stable user-defined metadata.
    ///
    /// `BTreeMap` is deliberate: deterministic ordering is required for
    /// reproducibility and configuration fingerprints.
    pub metadata: BTreeMap<String, String>,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            schema: BENCHMARK_CONFIG_SCHEMA.to_owned(),
            schema_version: BENCHMARK_CONFIG_SCHEMA_VERSION,
            benchmark: BenchmarkIdentity {
                id: "custom".to_owned(),
                version: 1,
            },
            qubits: DimensionRange::Auto,
            depth: DimensionRange::Auto,
            circuits: DEFAULT_CIRCUITS,
            shots: DEFAULT_SHOTS,
            seed: 0,
            execution_mode: ExecutionMode::Auto,
            execution_permission: ExecutionPermission::NonQpuOnly,
            backend: BackendSelection::Auto,
            compiler: CompilerConfig::default(),
            statistics: StatisticsConfig::default(),
            validation: ValidationConfig::default(),
            limits: ResourceLimits::default(),
            reporting: ReportingConfig::default(),
            metadata: BTreeMap::new(),
        }
    }
}

impl BenchmarkConfig {
    /// Creates a new configuration for a named benchmark.
    pub fn new<S: Into<String>>(
        benchmark_id: S,
        benchmark_version: u32,
    ) -> Result<Self, ConfigError> {
        let benchmark =
            BenchmarkIdentity::new(benchmark_id, benchmark_version)?;

        let config = Self {
            benchmark,
            ..Self::default()
        };

        config.validate()?;

        Ok(config)
    }

    /// Returns a production-safe default custom benchmark configuration.
    #[must_use]
    pub fn custom() -> Self {
        Self::default()
    }

    /// Sets the qubit dimension.
    pub fn with_qubits(
        mut self,
        qubits: DimensionRange,
    ) -> Result<Self, ConfigError> {
        validate_dimension_range(&qubits, &self.limits, "qubits")?;
        self.qubits = qubits;
        Ok(self)
    }

    /// Sets the circuit-depth dimension.
    pub fn with_depth(
        mut self,
        depth: DimensionRange,
    ) -> Result<Self, ConfigError> {
        validate_dimension_range(&depth, &self.limits, "depth")?;
        self.depth = depth;
        Ok(self)
    }

    /// Sets the number of circuits.
    pub fn with_circuits(
        mut self,
        circuits: usize,
    ) -> Result<Self, ConfigError> {
        if circuits == 0 {
            return Err(ConfigError::ZeroCircuits);
        }

        if circuits > self.limits.max_circuits {
            return Err(ConfigError::CircuitLimitExceeded {
                requested: circuits,
                maximum: self.limits.max_circuits,
            });
        }

        self.circuits = circuits;

        self.validate_total_shots()?;

        Ok(self)
    }

    /// Sets the number of shots per circuit.
    pub fn with_shots(
        mut self,
        shots: usize,
    ) -> Result<Self, ConfigError> {
        if shots == 0 {
            return Err(ConfigError::ZeroShots);
        }

        if shots > self.limits.max_shots_per_circuit {
            return Err(ConfigError::ShotLimitExceeded {
                requested: shots,
                maximum: self.limits.max_shots_per_circuit,
            });
        }

        self.shots = shots;

        self.validate_total_shots()?;

        Ok(self)
    }

    /// Sets the master deterministic seed.
    #[must_use]
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    /// Sets the execution mode.
    #[must_use]
    pub fn with_execution_mode(
        mut self,
        mode: ExecutionMode,
    ) -> Self {
        self.execution_mode = mode;
        self
    }

    /// Sets the execution permission.
    #[must_use]
    pub fn with_execution_permission(
        mut self,
        permission: ExecutionPermission,
    ) -> Self {
        self.execution_permission = permission;
        self
    }

    /// Sets backend selection.
    pub fn with_backend(
        mut self,
        backend: BackendSelection,
    ) -> Result<Self, ConfigError> {
        if let BackendSelection::Named(id) = &backend {
            validate_identifier(
                id,
                MAX_BACKEND_ID_BYTES,
                "backend identifier",
            )?;
        }

        self.backend = backend;

        Ok(self)
    }

    /// Sets compiler configuration.
    pub fn with_compiler(
        mut self,
        compiler: CompilerConfig,
    ) -> Result<Self, ConfigError> {
        compiler.validate()?;
        self.compiler = compiler;
        Ok(self)
    }

    /// Sets statistical configuration.
    pub fn with_statistics(
        mut self,
        statistics: StatisticsConfig,
    ) -> Result<Self, ConfigError> {
        statistics.validate()?;
        self.statistics = statistics;
        Ok(self)
    }

    /// Sets validation configuration.
    #[must_use]
    pub fn with_validation(
        mut self,
        validation: ValidationConfig,
    ) -> Self {
        self.validation = validation;
        self
    }

    /// Sets benchmark resource limits.
    pub fn with_limits(
        mut self,
        limits: ResourceLimits,
    ) -> Result<Self, ConfigError> {
        limits.validate()?;

        if self.circuits > limits.max_circuits {
            return Err(ConfigError::CircuitLimitExceeded {
                requested: self.circuits,
                maximum: limits.max_circuits,
            });
        }

        if self.shots > limits.max_shots_per_circuit {
            return Err(ConfigError::ShotLimitExceeded {
                requested: self.shots,
                maximum: limits.max_shots_per_circuit,
            });
        }

        limits.validate_total_shots(self.circuits, self.shots)?;

        self.limits = limits;

        Ok(self)
    }

    /// Sets reporting configuration.
    pub fn with_reporting(
        mut self,
        reporting: ReportingConfig,
    ) -> Result<Self, ConfigError> {
        reporting.validate()?;
        self.reporting = reporting;
        Ok(self)
    }

    /// Adds deterministic metadata.
    pub fn with_metadata<K, V>(
        mut self,
        key: K,
        value: V,
    ) -> Result<Self, ConfigError>
    where
        K: Into<String>,
        V: Into<String>,
    {
        let key = key.into();
        let value = value.into();

        validate_metadata_key(&key)?;
        validate_metadata_value(&value)?;

        if !self.metadata.contains_key(&key)
            && self.metadata.len() >= self.limits.max_metadata_entries
        {
            return Err(ConfigError::MetadataLimitExceeded {
                maximum: self.limits.max_metadata_entries,
            });
        }

        self.metadata.insert(key, value);

        Ok(self)
    }

    /// Validates the complete configuration.
    ///
    /// This method is intentionally deterministic and side-effect free.
    pub fn validate(&self) -> Result<(), ConfigError> {
        // ---------------------------------------------------------------------
        // Schema
        // ---------------------------------------------------------------------

        if self.schema != BENCHMARK_CONFIG_SCHEMA {
            return Err(ConfigError::UnsupportedSchema {
                schema: self.schema.clone(),
            });
        }

        if self.schema_version != BENCHMARK_CONFIG_SCHEMA_VERSION {
            return Err(ConfigError::UnsupportedSchemaVersion {
                version: self.schema_version,
            });
        }

        // ---------------------------------------------------------------------
        // Benchmark identity
        // ---------------------------------------------------------------------

        validate_identifier(
            &self.benchmark.id,
            MAX_BENCHMARK_ID_BYTES,
            "benchmark identifier",
        )?;

        if self.benchmark.version == 0 {
            return Err(ConfigError::InvalidVersion);
        }

        // ---------------------------------------------------------------------
        // Resource limits
        // ---------------------------------------------------------------------

        self.limits.validate()?;

        // ---------------------------------------------------------------------
        // Workload dimensions
        // ---------------------------------------------------------------------

        validate_dimension_range(
            &self.qubits,
            &self.limits,
            "qubits",
        )?;

        validate_dimension_range(
            &self.depth,
            &self.limits,
            "depth",
        )?;

        // ---------------------------------------------------------------------
        // Sampling
        // ---------------------------------------------------------------------

        if self.circuits == 0 {
            return Err(ConfigError::ZeroCircuits);
        }

        if self.circuits > self.limits.max_circuits {
            return Err(ConfigError::CircuitLimitExceeded {
                requested: self.circuits,
                maximum: self.limits.max_circuits,
            });
        }

        if self.shots == 0 {
            return Err(ConfigError::ZeroShots);
        }

        if self.shots > self.limits.max_shots_per_circuit {
            return Err(ConfigError::ShotLimitExceeded {
                requested: self.shots,
                maximum: self.limits.max_shots_per_circuit,
            });
        }

        self.validate_total_shots()?;

        // ---------------------------------------------------------------------
        // Backend
        // ---------------------------------------------------------------------

        if let BackendSelection::Named(id) = &self.backend {
            validate_identifier(
                id,
                MAX_BACKEND_ID_BYTES,
                "backend identifier",
            )?;
        }

        // ---------------------------------------------------------------------
        // Compiler/statistics/reporting
        // ---------------------------------------------------------------------

        self.compiler.validate()?;
        self.statistics.validate()?;
        self.reporting.validate()?;

        // ---------------------------------------------------------------------
        // Execution permission safety
        // ---------------------------------------------------------------------

        validate_execution_policy(
            self.execution_mode,
            self.execution_permission,
        )?;

        // ---------------------------------------------------------------------
        // Metadata
        // ---------------------------------------------------------------------

        if self.metadata.len() > self.limits.max_metadata_entries {
            return Err(ConfigError::MetadataLimitExceeded {
                maximum: self.limits.max_metadata_entries,
            });
        }

        for (key, value) in &self.metadata {
            validate_metadata_key(key)?;
            validate_metadata_value(value)?;
        }

        Ok(())
    }

    /// Validates aggregate shot consumption.
    pub fn validate_total_shots(&self) -> Result<u64, ConfigError> {
        self.limits
            .validate_total_shots(self.circuits, self.shots)
    }

    /// Returns the deterministic total number of requested shots.
    pub fn total_shots(&self) -> Result<u64, ConfigError> {
        self.validate_total_shots()
    }

    /// Returns whether this configuration permits physical-QPU execution.
    #[must_use]
    pub fn allows_qpu(&self) -> bool {
        matches!(
            self.execution_permission,
            ExecutionPermission::AllowQpu
        ) && matches!(
            self.execution_mode,
            ExecutionMode::Qpu | ExecutionMode::Auto
        )
    }

    /// Returns whether this configuration is planning-only.
    #[must_use]
    pub fn is_plan_only(&self) -> bool {
        matches!(self.execution_mode, ExecutionMode::PlanOnly)
            || matches!(
                self.execution_permission,
                ExecutionPermission::PlanOnly
            )
    }
}

// =============================================================================
// Builder
// =============================================================================

/// Mutable builder for `BenchmarkConfig`.
///
/// The builder provides a convenient API while the resulting
/// `BenchmarkConfig` remains immutable-by-convention and must pass complete
/// validation before execution.
#[derive(Debug, Clone)]
pub struct BenchmarkConfigBuilder {
    config: BenchmarkConfig,
}

impl BenchmarkConfigBuilder {
    /// Creates a builder for a named benchmark.
    pub fn new<S: Into<String>>(
        benchmark_id: S,
        benchmark_version: u32,
    ) -> Result<Self, ConfigError> {
        Ok(Self {
            config: BenchmarkConfig::new(
                benchmark_id,
                benchmark_version,
            )?,
        })
    }

    /// Starts from the custom benchmark defaults.
    #[must_use]
    pub fn custom() -> Self {
        Self {
            config: BenchmarkConfig::custom(),
        }
    }

    /// Sets qubit dimensions.
    pub fn qubits(
        mut self,
        range: DimensionRange,
    ) -> Result<Self, ConfigError> {
        self.config = self.config.with_qubits(range)?;
        Ok(self)
    }

    /// Sets depth dimensions.
    pub fn depth(
        mut self,
        range: DimensionRange,
    ) -> Result<Self, ConfigError> {
        self.config = self.config.with_depth(range)?;
        Ok(self)
    }

    /// Sets circuit count.
    pub fn circuits(
        mut self,
        circuits: usize,
    ) -> Result<Self, ConfigError> {
        self.config = self.config.with_circuits(circuits)?;
        Ok(self)
    }

    /// Sets shots per circuit.
    pub fn shots(
        mut self,
        shots: usize,
    ) -> Result<Self, ConfigError> {
        self.config = self.config.with_shots(shots)?;
        Ok(self)
    }

    /// Sets deterministic seed.
    #[must_use]
    pub fn seed(mut self, seed: u64) -> Self {
        self.config = self.config.with_seed(seed);
        self
    }

    /// Sets execution mode.
    #[must_use]
    pub fn execution_mode(
        mut self,
        mode: ExecutionMode,
    ) -> Self {
        self.config = self.config.with_execution_mode(mode);
        self
    }

    /// Sets execution permission.
    #[must_use]
    pub fn execution_permission(
        mut self,
        permission: ExecutionPermission,
    ) -> Self {
        self.config =
            self.config.with_execution_permission(permission);
        self
    }

    /// Sets backend.
    pub fn backend(
        mut self,
        backend: BackendSelection,
    ) -> Result<Self, ConfigError> {
        self.config = self.config.with_backend(backend)?;
        Ok(self)
    }

    /// Sets compiler configuration.
    pub fn compiler(
        mut self,
        compiler: CompilerConfig,
    ) -> Result<Self, ConfigError> {
        self.config = self.config.with_compiler(compiler)?;
        Ok(self)
    }

    /// Sets statistical configuration.
    pub fn statistics(
        mut self,
        statistics: StatisticsConfig,
    ) -> Result<Self, ConfigError> {
        self.config =
            self.config.with_statistics(statistics)?;
        Ok(self)
    }

    /// Sets resource limits.
    pub fn limits(
        mut self,
        limits: ResourceLimits,
    ) -> Result<Self, ConfigError> {
        self.config = self.config.with_limits(limits)?;
        Ok(self)
    }

    /// Sets reporting configuration.
    pub fn reporting(
        mut self,
        reporting: ReportingConfig,
    ) -> Result<Self, ConfigError> {
        self.config =
            self.config.with_reporting(reporting)?;
        Ok(self)
    }

    /// Adds metadata.
    pub fn metadata<K, V>(
        mut self,
        key: K,
        value: V,
    ) -> Result<Self, ConfigError>
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.config = self.config.with_metadata(key, value)?;
        Ok(self)
    }

    /// Finalizes and validates the configuration.
    pub fn build(self) -> Result<BenchmarkConfig, ConfigError> {
        self.config.validate()?;
        Ok(self.config)
    }
}

// =============================================================================
// Parsing helpers
// =============================================================================

/// Parses an execution mode from a stable machine identifier.
impl FromStr for ExecutionMode {
    type Err = ConfigError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "plan_only" => Ok(Self::PlanOnly),
            "simulator" => Ok(Self::Simulator),
            "emulator" => Ok(Self::Emulator),
            "qpu" => Ok(Self::Qpu),
            "auto" => Ok(Self::Auto),
            _ => Err(ConfigError::UnknownExecutionMode {
                value: value.to_owned(),
            }),
        }
    }
}

/// Parses optimization levels from stable identifiers.
impl FromStr for OptimizationLevel {
    type Err = ConfigError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "none" => Ok(Self::None),
            "basic" => Ok(Self::Basic),
            "standard" => Ok(Self::Standard),
            "aggressive" => Ok(Self::Aggressive),
            _ => Err(ConfigError::UnknownOptimizationLevel {
                value: value.to_owned(),
            }),
        }
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_identifier(
    value: &str,
    maximum_bytes: usize,
    field: &'static str,
) -> Result<(), ConfigError> {
    if value.is_empty() {
        return Err(ConfigError::EmptyIdentifier { field });
    }

    if value.len() > maximum_bytes {
        return Err(ConfigError::IdentifierTooLong {
            field,
            length: value.len(),
            maximum: maximum_bytes,
        });
    }

    let bytes = value.as_bytes();

    if !bytes[0].is_ascii_lowercase() {
        return Err(ConfigError::InvalidIdentifier {
            field,
            reason: "identifier must begin with a lowercase ASCII letter",
        });
    }

    for byte in bytes.iter().copied() {
        if !(byte.is_ascii_lowercase()
            || byte.is_ascii_digit()
            || byte == b'_'
            || byte == b'-')
        {
            return Err(ConfigError::InvalidIdentifier {
                field,
                reason:
                    "identifier may contain only lowercase ASCII letters, digits, '-' or '_'",
            });
        }
    }

    Ok(())
}

fn validate_metadata_key(
    key: &str,
) -> Result<(), ConfigError> {
    validate_identifier(
        key,
        MAX_METADATA_KEY_BYTES,
        "metadata key",
    )
}

fn validate_metadata_value(
    value: &str,
) -> Result<(), ConfigError> {
    if value.len() > MAX_METADATA_VALUE_BYTES {
        return Err(ConfigError::MetadataValueTooLong {
            length: value.len(),
            maximum: MAX_METADATA_VALUE_BYTES,
        });
    }

    // NUL is not useful in configuration metadata and can cause problems when
    // crossing C/CLI/filesystem boundaries.
    if value.as_bytes().contains(&0) {
        return Err(ConfigError::InvalidMetadataValue);
    }

    Ok(())
}

fn validate_explicit_values(
    values: &[usize],
) -> Result<(), ConfigError> {
    if values.is_empty() {
        return Err(ConfigError::EmptyExplicitValues);
    }

    let mut previous = 0usize;

    for &value in values {
        if value == 0 {
            return Err(ConfigError::ZeroDimensionValue);
        }

        if previous != 0 && value <= previous {
            return Err(ConfigError::NonIncreasingDimensionValues);
        }

        previous = value;
    }

    Ok(())
}

fn validate_dimension_range(
    range: &DimensionRange,
    limits: &ResourceLimits,
    field: &'static str,
) -> Result<(), ConfigError> {
    match range {
        DimensionRange::Auto => Ok(()),

        DimensionRange::Range(range) => {
            range.validate_cardinality()?;

            if range.end > dimension_limit(field, limits) {
                return Err(ConfigError::DimensionLimitExceeded {
                    field,
                    requested: range.end,
                    maximum: dimension_limit(field, limits),
                });
            }

            Ok(())
        }

        DimensionRange::Explicit(values) => {
            if values.len() > MAX_EXPLICIT_SIZES {
                return Err(ConfigError::TooManyExplicitValues {
                    requested: values.len(),
                    maximum: MAX_EXPLICIT_SIZES,
                });
            }

            validate_explicit_values(values)?;

            if let Some(&maximum) = values.iter().max() {
                let limit = dimension_limit(field, limits);

                if maximum > limit {
                    return Err(ConfigError::DimensionLimitExceeded {
                        field,
                        requested: maximum,
                        maximum: limit,
                    });
                }
            }

            Ok(())
        }
    }
}

fn dimension_limit(
    field: &'static str,
    limits: &ResourceLimits,
) -> usize {
    match field {
        "qubits" => limits.max_qubits,
        "depth" => limits.max_depth,
        _ => limits.max_qubits,
    }
}

fn validate_execution_policy(
    mode: ExecutionMode,
    permission: ExecutionPermission,
) -> Result<(), ConfigError> {
    if matches!(permission, ExecutionPermission::PlanOnly)
        && !matches!(mode, ExecutionMode::PlanOnly)
    {
        return Err(ConfigError::ExecutionPermissionConflict);
    }

    if matches!(
        permission,
        ExecutionPermission::NonQpuOnly
    ) && matches!(mode, ExecutionMode::Qpu)
    {
        return Err(ConfigError::QpuExecutionNotPermitted);
    }

    Ok(())
}

// =============================================================================
// Errors
// =============================================================================

/// Complete configuration error vocabulary.
///
/// This type is intentionally local to `config.rs` during the foundation
/// phase. Once `core::errors` is implemented, it can wrap or absorb these
/// variants without changing the public semantics of the configuration API.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "full", derive(Serialize, Deserialize))]
pub enum ConfigError {
    /// Unsupported configuration schema.
    UnsupportedSchema {
        schema: String,
    },

    /// Unsupported serialized schema version.
    UnsupportedSchemaVersion {
        version: u32,
    },

    /// Version zero is reserved/invalid.
    InvalidVersion,

    /// Identifier was empty.
    EmptyIdentifier {
        field: &'static str,
    },

    /// Identifier exceeded its maximum size.
    IdentifierTooLong {
        field: &'static str,
        length: usize,
        maximum: usize,
    },

    /// Identifier contained unsupported characters.
    InvalidIdentifier {
        field: &'static str,
        reason: &'static str,
    },

    /// Generic invalid inclusive range.
    InvalidRange {
        start: usize,
        end: usize,
    },

    /// Range started at zero.
    ZeroRangeStart,

    /// Range step was zero.
    ZeroRangeStep,

    /// Range expansion overflowed.
    RangeOverflow,

    /// Range contained too many values.
    RangeTooLarge {
        requested: usize,
        maximum: usize,
    },

    /// Explicit dimension list was empty.
    EmptyExplicitValues,

    /// Explicit dimension contained zero.
    ZeroDimensionValue,

    /// Explicit dimensions were not strictly increasing.
    NonIncreasingDimensionValues,

    /// Too many explicit dimension values.
    TooManyExplicitValues {
        requested: usize,
        maximum: usize,
    },

    /// Dimension exceeded configured resource limits.
    DimensionLimitExceeded {
        field: &'static str,
        requested: usize,
        maximum: usize,
    },

    /// Zero circuits requested.
    ZeroCircuits,

    /// Circuit count exceeds configured safety limit.
    CircuitLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    /// Zero shots requested.
    ZeroShots,

    /// Shot count exceeds configured safety limit.
    ShotLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    /// Aggregate shot multiplication overflowed.
    TotalShotsOverflow,

    /// Aggregate shot count exceeds configured safety limit.
    TotalShotsExceeded {
        requested: u64,
        maximum: u64,
    },

    /// Confidence level is invalid.
    InvalidConfidenceLevel {
        value: f64,
    },

    /// Minimum sample count cannot be zero.
    InvalidMinimumSamples,

    /// Bootstrap sample count is invalid.
    InvalidBootstrapSamples,

    /// Bootstrap request exceeds safety limits.
    BootstrapLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    /// Resource limit itself was zero.
    ZeroResourceLimit {
        field: &'static str,
    },

    /// Metadata count exceeded its limit.
    MetadataLimitExceeded {
        maximum: usize,
    },

    /// Metadata value exceeded its byte limit.
    MetadataValueTooLong {
        length: usize,
        maximum: usize,
    },

    /// Metadata value contained a NUL byte.
    InvalidMetadataValue,

    /// Conflicting execution policy.
    ExecutionPermissionConflict,

    /// QPU execution was not permitted.
    QpuExecutionNotPermitted,

    /// Unknown execution mode.
    UnknownExecutionMode {
        value: String,
    },

    /// Unknown optimization level.
    UnknownOptimizationLevel {
        value: String,
    },

    /// Platform integer conversion failed.
    IntegerConversion,
}

impl fmt::Display for ConfigError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::UnsupportedSchema { schema } => write!(
                f,
                "unsupported benchmark configuration schema '{}'",
                schema
            ),

            Self::UnsupportedSchemaVersion { version } => write!(
                f,
                "unsupported benchmark configuration schema version {}",
                version
            ),

            Self::InvalidVersion => {
                f.write_str("benchmark version must be greater than zero")
            }

            Self::EmptyIdentifier { field } => {
                write!(f, "{} cannot be empty", field)
            }

            Self::IdentifierTooLong {
                field,
                length,
                maximum,
            } => write!(
                f,
                "{} is {} bytes; maximum is {} bytes",
                field, length, maximum
            ),

            Self::InvalidIdentifier { field, reason } => {
                write!(f, "invalid {}: {}", field, reason)
            }

            Self::InvalidRange { start, end } => {
                write!(
                    f,
                    "invalid inclusive range: start {} exceeds end {}",
                    start, end
                )
            }

            Self::ZeroRangeStart => {
                f.write_str("benchmark dimension range cannot start at zero")
            }

            Self::ZeroRangeStep => {
                f.write_str("benchmark dimension range step cannot be zero")
            }

            Self::RangeOverflow => {
                f.write_str("benchmark dimension range expansion overflowed")
            }

            Self::RangeTooLarge {
                requested,
                maximum,
            } => write!(
                f,
                "benchmark dimension range expands to {}; maximum is {}",
                requested, maximum
            ),

            Self::EmptyExplicitValues => {
                f.write_str("explicit benchmark dimension values cannot be empty")
            }

            Self::ZeroDimensionValue => {
                f.write_str("benchmark dimension values must be greater than zero")
            }

            Self::NonIncreasingDimensionValues => {
                f.write_str(
                    "explicit benchmark dimension values must be strictly increasing",
                )
            }

            Self::TooManyExplicitValues {
                requested,
                maximum,
            } => write!(
                f,
                "benchmark contains {} explicit dimension values; maximum is {}",
                requested, maximum
            ),

            Self::DimensionLimitExceeded {
                field,
                requested,
                maximum,
            } => write!(
                f,
                "{} dimension {} exceeds configured maximum {}",
                field, requested, maximum
            ),

            Self::ZeroCircuits => {
                f.write_str("benchmark circuit count must be greater than zero")
            }

            Self::CircuitLimitExceeded {
                requested,
                maximum,
            } => write!(
                f,
                "benchmark requests {} circuits; maximum is {}",
                requested, maximum
            ),

            Self::ZeroShots => {
                f.write_str("benchmark shots must be greater than zero")
            }

            Self::ShotLimitExceeded {
                requested,
                maximum,
            } => write!(
                f,
                "benchmark requests {} shots per circuit; maximum is {}",
                requested, maximum
            ),

            Self::TotalShotsOverflow => {
                f.write_str("total benchmark shot count overflowed")
            }

            Self::TotalShotsExceeded {
                requested,
                maximum,
            } => write!(
                f,
                "benchmark requests {} total shots; maximum is {}",
                requested, maximum
            ),

            Self::InvalidConfidenceLevel { value } => write!(
                f,
                "confidence level {} must be finite and strictly between 0 and 1",
                value
            ),

            Self::InvalidMinimumSamples => {
                f.write_str("minimum statistical sample count must be greater than zero")
            }

            Self::InvalidBootstrapSamples => {
                f.write_str("bootstrap sample count must be greater than zero")
            }

            Self::BootstrapLimitExceeded {
                requested,
                maximum,
            } => write!(
                f,
                "bootstrap requests {} resamples; maximum is {}",
                requested, maximum
            ),

            Self::ZeroResourceLimit { field } => {
                write!(f, "{} resource limit must be greater than zero", field)
            }

            Self::MetadataLimitExceeded { maximum } => write!(
                f,
                "benchmark metadata exceeds maximum entry count {}",
                maximum
            ),

            Self::MetadataValueTooLong {
                length,
                maximum,
            } => write!(
                f,
                "benchmark metadata value is {} bytes; maximum is {} bytes",
                length, maximum
            ),

            Self::InvalidMetadataValue => {
                f.write_str("benchmark metadata value contains a NUL byte")
            }

            Self::ExecutionPermissionConflict => {
                f.write_str(
                    "plan-only execution permission requires plan-only execution mode",
                )
            }

            Self::QpuExecutionNotPermitted => {
                f.write_str("physical QPU execution is not permitted by this configuration")
            }

            Self::UnknownExecutionMode { value } => {
                write!(f, "unknown benchmark execution mode '{}'", value)
            }

            Self::UnknownOptimizationLevel { value } => {
                write!(f, "unknown optimization level '{}'", value)
            }

            Self::IntegerConversion => {
                f.write_str("integer conversion failed")
            }
        }
    }
}

impl std::error::Error for ConfigError {}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_configuration_is_valid() {
        let config = BenchmarkConfig::default();

        assert!(config.validate().is_ok());
        assert_eq!(
            config.schema,
            BENCHMARK_CONFIG_SCHEMA
        );
        assert_eq!(
            config.schema_version,
            BENCHMARK_CONFIG_SCHEMA_VERSION
        );
    }

    #[test]
    fn named_configuration_is_valid() {
        let config =
            BenchmarkConfig::new("quantum_volume", 1).unwrap();

        assert_eq!(config.benchmark.id, "quantum_volume");
        assert_eq!(config.benchmark.version, 1);
    }

    #[test]
    fn invalid_identifier_is_rejected() {
        assert!(
            BenchmarkIdentity::new("QuantumVolume", 1).is_err()
        );

        assert!(
            BenchmarkIdentity::new("quantum volume", 1).is_err()
        );

        assert!(
            BenchmarkIdentity::new("quantum_volume", 0).is_err()
        );
    }

    #[test]
    fn range_is_deterministic() {
        let range =
            InclusiveRange::new(2, 10, 2).unwrap();

        assert_eq!(
            range.values().unwrap(),
            vec![2, 4, 6, 8, 10]
        );
    }

    #[test]
    fn range_membership_is_correct() {
        let range =
            InclusiveRange::new(2, 10, 2).unwrap();

        assert!(range.contains(2));
        assert!(range.contains(6));
        assert!(range.contains(10));
        assert!(!range.contains(3));
        assert!(!range.contains(11));
    }

    #[test]
    fn zero_dimension_is_rejected() {
        assert!(
            InclusiveRange::new(0, 10, 1).is_err()
        );

        assert!(
            DimensionRange::explicit(vec![0, 1]).is_err()
        );
    }

    #[test]
    fn explicit_dimensions_must_be_strictly_increasing() {
        assert!(
            DimensionRange::explicit(vec![2, 4, 8]).is_ok()
        );

        assert!(
            DimensionRange::explicit(vec![2, 2, 4]).is_err()
        );

        assert!(
            DimensionRange::explicit(vec![4, 2]).is_err()
        );
    }

    #[test]
    fn total_shots_are_checked_without_overflow() {
        let config = BenchmarkConfig::new("test", 1)
            .unwrap()
            .with_circuits(10)
            .unwrap()
            .with_shots(100)
            .unwrap();

        assert_eq!(
            config.total_shots().unwrap(),
            1_000
        );
    }

    #[test]
    fn excessive_total_shots_are_rejected() {
        let limits = ResourceLimits {
            max_total_shots: 1_000,
            ..ResourceLimits::default()
        };

        let result =
            BenchmarkConfig::new("test", 1)
                .unwrap()
                .with_limits(limits);

        assert!(result.is_ok());

        let result = result
            .unwrap()
            .with_circuits(20)
            .unwrap()
            .with_shots(100);

        assert!(result.is_err());
    }

    #[test]
    fn qpu_permission_is_explicit() {
        let config =
            BenchmarkConfig::new("test", 1)
                .unwrap()
                .with_execution_mode(ExecutionMode::Qpu)
                .with_execution_permission(
                    ExecutionPermission::AllowQpu,
                );

        assert!(config.is_ok());
        assert!(config.unwrap().allows_qpu());
    }

    #[test]
    fn qpu_is_rejected_without_permission() {
        let result =
            BenchmarkConfig::new("test", 1)
                .unwrap()
                .with_execution_mode(ExecutionMode::Qpu)
                .validate();

        assert!(result.is_err());
    }

    #[test]
    fn plan_only_policy_is_consistent() {
        let config =
            BenchmarkConfig::new("test", 1)
                .unwrap()
                .with_execution_mode(
                    ExecutionMode::PlanOnly,
                )
                .with_execution_permission(
                    ExecutionPermission::PlanOnly,
                )
                .validate();

        assert!(config.is_ok());
    }

    #[test]
    fn metadata_is_bounded() {
        let config =
            BenchmarkConfig::new("test", 1)
                .unwrap()
                .with_metadata(
                    "application",
                    "vqe",
                );

        assert!(config.is_ok());

        let config = config.unwrap();

        assert_eq!(
            config.metadata.get("application"),
            Some(&"vqe".to_owned())
        );
    }

    #[test]
    fn metadata_keys_are_canonical() {
        assert!(
            BenchmarkConfig::new("test", 1)
                .unwrap()
                .with_metadata(
                    "benchmark_family",
                    "application",
                )
                .is_ok()
        );

        assert!(
            BenchmarkConfig::new("test", 1)
                .unwrap()
                .with_metadata(
                    "Benchmark Family",
                    "application",
                )
                .is_err()
        );
    }

    #[test]
    fn builder_produces_valid_configuration() {
        let config =
            BenchmarkConfigBuilder::new(
                "quantum_volume",
                1,
            )
            .unwrap()
            .qubits(
                DimensionRange::range(
                    2,
                    10,
                    1,
                )
                .unwrap(),
            )
            .unwrap()
            .depth(
                DimensionRange::range(
                    2,
                    10,
                    1,
                )
                .unwrap(),
            )
            .unwrap()
            .circuits(100)
            .unwrap()
            .shots(1_000)
            .unwrap()
            .seed(42)
            .execution_mode(
                ExecutionMode::Simulator,
            )
            .backend(
                BackendSelection::named(
                    "local_simulator",
                )
                .unwrap(),
            )
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(config.benchmark.id, "quantum_volume");
        assert_eq!(config.circuits, 100);
        assert_eq!(config.shots, 1_000);
        assert_eq!(config.seed, 42);
        assert_eq!(
            config.total_shots().unwrap(),
            100_000
        );
    }

    #[test]
    fn default_statistics_are_valid() {
        assert!(
            StatisticsConfig::default()
                .validate()
                .is_ok()
        );
    }

    #[test]
    fn invalid_confidence_is_rejected() {
        let statistics = StatisticsConfig {
            confidence_level: 1.0,
            ..StatisticsConfig::default()
        };

        assert!(
            statistics.validate().is_err()
        );
    }

    #[test]
    fn deterministic_metadata_order_is_used() {
        let mut first = BenchmarkConfig::new("test", 1).unwrap();

        first = first
            .with_metadata("z", "last")
            .unwrap()
            .with_metadata("a", "first")
            .unwrap();

        let mut second =
            BenchmarkConfig::new("test", 1).unwrap();

        second = second
            .with_metadata("a", "first")
            .unwrap()
            .with_metadata("z", "last")
            .unwrap();

        assert_eq!(first.metadata, second.metadata);
    }

    #[test]
    fn backend_identifier_is_validated() {
        assert!(
            BackendSelection::named(
                "local_simulator"
            )
            .is_ok()
        );

        assert!(
            BackendSelection::named(
                "Local Simulator"
            )
            .is_err()
        );
    }

    #[test]
    fn execution_mode_parses_stably() {
        assert_eq!(
            "simulator"
                .parse::<ExecutionMode>()
                .unwrap(),
            ExecutionMode::Simulator
        );

        assert_eq!(
            "qpu"
                .parse::<ExecutionMode>()
                .unwrap(),
            ExecutionMode::Qpu
        );

        assert!(
            "physical_qpu"
                .parse::<ExecutionMode>()
                .is_err()
        );
    }

    #[test]
    fn optimization_level_parses_stably() {
        assert_eq!(
            "standard"
                .parse::<OptimizationLevel>()
                .unwrap(),
            OptimizationLevel::Standard
        );

        assert_eq!(
            "aggressive"
                .parse::<OptimizationLevel>()
                .unwrap(),
            OptimizationLevel::Aggressive
        );
    }
}