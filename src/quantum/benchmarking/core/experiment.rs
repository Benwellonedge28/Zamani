//! Zamani Quantum Benchmarking — Experiment Model
//!
//! Defines the canonical, validated representation of a quantum benchmarking
//! experiment.
//!
//! # Architectural responsibility
//!
//! `Experiment` is the boundary between:
//!
//! ```text
//! Benchmark configuration
//!        +
//! Workload definition
//!        +
//! Circuit/workload instances
//!        +
//! Execution policy
//!        +
//! Reproducibility identity
//!        +
//! Resource limits
//!        ↓
//! Benchmark Experiment
//!        ↓
//! execution / analysis / reporting
//! ```
//!
//! This module deliberately does NOT:
//!
//! - execute quantum circuits;
//! - communicate with hardware;
//! - perform statistical analysis;
//! - calculate benchmark metrics;
//! - generate random circuits;
//! - compile/transpile circuits;
//! - perform routing;
//! - schedule circuits;
//! - parse Zamani source code;
//! - own the canonical Quantum IR.
//!
//! Those responsibilities belong to the appropriate benchmarking and quantum
//! subsystems.
//!
//! # Canonical dependency direction
//!
//! ```text
//! quantum::frontend
//!        │
//!        ▼
//! quantum::ir
//!        │
//!        ├───────────────┐
//!        ▼               ▼
//! algorithms        compiler pipeline
//!        │               │
//!        └──────┬────────┘
//!               ▼
//!        benchmarking::core
//!               │
//!        ┌──────┼───────────┐
//!        ▼      ▼           ▼
//! execution statistics   reporting
//! ```
//!
//! The benchmark experiment may consume Quantum IR, but Quantum IR must never
//! depend on benchmarking.
//!
//! # Production invariants
//!
//! A valid experiment:
//!
//! 1. has a non-empty stable identity;
//! 2. has a non-empty benchmark identifier;
//! 3. has a valid configuration;
//! 4. has a valid workload;
//! 5. has at least one executable workload item;
//! 6. has bounded execution requirements;
//! 7. has an explicit reproducibility seed;
//! 8. has deterministic experiment identity;
//! 9. cannot silently change after validation;
//! 10. cannot request zero shots;
//! 11. cannot request unbounded circuit/shot execution;
//! 12. cannot contain duplicate workload IDs;
//! 13. cannot contain duplicate circuit IDs;
//! 14. cannot contain invalid dimensions;
//! 15. records whether execution may be partial;
//! 16. keeps execution policy separate from backend implementation.
//!
//! # Integration contracts
//!
//! This file is intentionally written against the following stable contracts:
//!
//! - `core::config::BenchmarkConfig`
//! - `core::workload::BenchmarkWorkload`
//! - `core::circuit::BenchmarkCircuit`
//! - `core::errors::BenchmarkError`
//! - `core::limits::BenchmarkLimits`
//!
//! These types are owned by their respective modules. They must not be
//! reimplemented here.
//!
//! The experiment itself is the orchestration object; it does not become the
//! owner of those domain concepts.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97.1 / Rust 2021.
//!
//! No nightly features are required.

use std::collections::BTreeMap;
use std::fmt;
use std::time::Duration;

use super::circuit::BenchmarkCircuit;
use super::config::BenchmarkConfig;
use super::errors::BenchmarkError;
use super::limits::BenchmarkLimits;
use super::workload::BenchmarkWorkload;

// =============================================================================
// Constants
// =============================================================================

/// Maximum length of an experiment identifier.
///
/// This protects logs, serialized results, registry keys, and downstream
/// systems from pathological identifiers.
pub const MAX_EXPERIMENT_ID_LENGTH: usize = 256;

/// Maximum length of a benchmark identifier.
pub const MAX_BENCHMARK_ID_LENGTH: usize = 256;

/// Maximum length of a workload identifier.
pub const MAX_WORKLOAD_ID_LENGTH: usize = 256;

/// Maximum number of circuits in a single experiment.
///
/// The authoritative resource ceiling remains `BenchmarkLimits`; this value
/// is an additional structural sanity bound for experiment definitions.
pub const MAX_EXPERIMENT_CIRCUITS: usize = 10_000_000;

/// Maximum number of arbitrary experiment tags.
pub const MAX_EXPERIMENT_TAGS: usize = 256;

/// Maximum length of a tag key.
pub const MAX_TAG_KEY_LENGTH: usize = 128;

/// Maximum length of a tag value.
pub const MAX_TAG_VALUE_LENGTH: usize = 4096;

/// Maximum number of user-defined metadata entries.
pub const MAX_METADATA_ENTRIES: usize = 512;

/// Maximum metadata key length.
pub const MAX_METADATA_KEY_LENGTH: usize = 256;

/// Maximum metadata value length.
pub const MAX_METADATA_VALUE_LENGTH: usize = 16 * 1024;

// =============================================================================
// Experiment identity
// =============================================================================

/// Stable identifier for a benchmark experiment.
///
/// An experiment ID identifies a concrete experiment definition, not merely a
/// benchmark family.
///
/// For example:
///
/// ```text
/// qv-superconducting-20q-seed42
/// ```
///
/// The identifier must remain stable once an experiment has been submitted.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExperimentId(String);

impl ExperimentId {
    /// Creates a validated experiment identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, BenchmarkError> {
        let value = value.into();

        validate_identifier(
            &value,
            "experiment ID",
            MAX_EXPERIMENT_ID_LENGTH,
        )?;

        Ok(Self(value))
    }

    /// Returns the identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for ExperimentId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ExperimentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Benchmark identity
// =============================================================================

/// Stable identifier for a benchmark protocol.
///
/// Examples:
///
/// - `quantum_volume`
/// - `randomized_benchmarking`
/// - `xeb`
/// - `cycle_benchmarking`
/// - `vqe`
/// - `qaoa`
/// - `logical_error_rate`
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BenchmarkId(String);

impl BenchmarkId {
    /// Creates a validated benchmark identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, BenchmarkError> {
        let value = value.into();

        validate_identifier(
            &value,
            "benchmark ID",
            MAX_BENCHMARK_ID_LENGTH,
        )?;

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for BenchmarkId {
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
// Workload identity
// =============================================================================

/// Stable identifier for a workload instance.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorkloadId(String);

impl WorkloadId {
    pub fn new(value: impl Into<String>) -> Result<Self, BenchmarkError> {
        let value = value.into();

        validate_identifier(
            &value,
            "workload ID",
            MAX_WORKLOAD_ID_LENGTH,
        )?;

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for WorkloadId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for WorkloadId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Experiment lifecycle
// =============================================================================

/// Lifecycle state of an experiment.
///
/// The experiment definition itself remains immutable. Lifecycle state is
/// metadata describing execution, not mutable benchmark semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExperimentState {
    /// Experiment has been constructed but not submitted.
    Defined,

    /// Experiment has passed all validation.
    Validated,

    /// Execution has begun.
    Running,

    /// Execution completed successfully.
    Completed,

    /// Execution was cancelled.
    Cancelled,

    /// Execution failed.
    Failed,

    /// Execution completed but one or more workload items failed.
    PartiallyCompleted,
}

impl ExperimentState {
    /// Returns whether execution may be started from this state.
    pub fn can_start(self) -> bool {
        matches!(self, Self::Defined | Self::Validated)
    }

    /// Returns whether this state represents terminal execution.
    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Completed
                | Self::Cancelled
                | Self::Failed
                | Self::PartiallyCompleted
        )
    }

    /// Returns whether execution has started.
    pub fn has_started(self) -> bool {
        !matches!(self, Self::Defined | Self::Validated)
    }
}

// =============================================================================
// Execution mode
// =============================================================================

/// Execution strategy requested by the experiment.
///
/// The experiment requests a mode; the backend/executor decides whether it
/// can satisfy that mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExecutionMode {
    /// Execute through a local simulator.
    Simulator,

    /// Execute through an emulator.
    Emulator,

    /// Execute on a physical quantum processor.
    Hardware,

    /// Permit the executor to choose an appropriate backend.
    Auto,

    /// Generate and validate the experiment without executing it.
    GenerateOnly,
}

impl ExecutionMode {
    /// Returns whether this mode requires an actual execution backend.
    pub fn requires_execution(self) -> bool {
        !matches!(self, Self::GenerateOnly)
    }

    /// Returns whether this mode explicitly requires physical hardware.
    pub fn requires_hardware(self) -> bool {
        matches!(self, Self::Hardware)
    }
}

// =============================================================================
// Failure policy
// =============================================================================

/// Defines how execution handles individual workload failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FailurePolicy {
    /// Stop immediately when any workload item fails.
    FailFast,

    /// Continue independent workload items and report all failures.
    ContinueIndependent,

    /// Continue execution but mark the final result as partial.
    BestEffort,
}

impl Default for FailurePolicy {
    fn default() -> Self {
        Self::FailFast
    }
}

// =============================================================================
// Reproducibility
// =============================================================================

/// Deterministic seed associated with an experiment.
///
/// A seed is part of the experiment definition rather than executor-local
/// state. This ensures that random circuit generation can be reproduced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExperimentSeed(u64);

impl ExperimentSeed {
    pub const ZERO: Self = Self(0);

    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

impl Default for ExperimentSeed {
    fn default() -> Self {
        Self::ZERO
    }
}

impl fmt::Display for ExperimentSeed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// =============================================================================
// Execution policy
// =============================================================================

/// Execution policy attached to an experiment.
///
/// This describes what the experiment requests. It does not implement
/// execution and does not contain backend-specific API objects.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExperimentExecutionPolicy {
    /// Requested execution mode.
    pub mode: ExecutionMode,

    /// Requested number of shots for each circuit unless overridden by the
    /// benchmark protocol.
    pub shots: usize,

    /// Maximum total circuits that may be submitted.
    pub max_circuits: usize,

    /// Maximum total shots across the entire experiment.
    pub max_total_shots: usize,

    /// Optional wall-clock execution timeout.
    pub timeout: Option<Duration>,

    /// Failure behavior.
    pub failure_policy: FailurePolicy,

    /// Whether partial results may be retained after cancellation/failure.
    pub preserve_partial_results: bool,

    /// Maximum number of concurrently submitted circuits.
    pub max_parallelism: usize,
}

impl ExperimentExecutionPolicy {
    /// Creates a conservative execution policy.
    pub fn new(shots: usize) -> Result<Self, BenchmarkError> {
        if shots == 0 {
            return Err(BenchmarkError::InvalidExperiment(
                "execution shots must be greater than zero".to_owned(),
            ));
        }

        Ok(Self {
            mode: ExecutionMode::Auto,
            shots,
            max_circuits: 1,
            max_total_shots: shots,
            timeout: None,
            failure_policy: FailurePolicy::FailFast,
            preserve_partial_results: true,
            max_parallelism: 1,
        })
    }

    /// Changes execution mode.
    pub fn with_mode(mut self, mode: ExecutionMode) -> Self {
        self.mode = mode;
        self
    }

    /// Sets the maximum circuit count.
    pub fn with_max_circuits(
        mut self,
        value: usize,
    ) -> Result<Self, BenchmarkError> {
        if value == 0 {
            return Err(BenchmarkError::InvalidExperiment(
                "max_circuits must be greater than zero".to_owned(),
            ));
        }

        self.max_circuits = value;
        Ok(self)
    }

    /// Sets the total shot ceiling.
    pub fn with_max_total_shots(
        mut self,
        value: usize,
    ) -> Result<Self, BenchmarkError> {
        if value == 0 {
            return Err(BenchmarkError::InvalidExperiment(
                "max_total_shots must be greater than zero".to_owned(),
            ));
        }

        self.max_total_shots = value;
        Ok(self)
    }

    /// Sets an execution timeout.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Sets the failure policy.
    pub fn with_failure_policy(
        mut self,
        policy: FailurePolicy,
    ) -> Self {
        self.failure_policy = policy;
        self
    }

    /// Sets whether partial observations should be preserved.
    pub fn preserve_partial_results(
        mut self,
        preserve: bool,
    ) -> Self {
        self.preserve_partial_results = preserve;
        self
    }

    /// Sets execution parallelism.
    pub fn with_max_parallelism(
        mut self,
        value: usize,
    ) -> Result<Self, BenchmarkError> {
        if value == 0 {
            return Err(BenchmarkError::InvalidExperiment(
                "max_parallelism must be greater than zero".to_owned(),
            ));
        }

        self.max_parallelism = value;
        Ok(self)
    }

    /// Validates the execution policy.
    pub fn validate(&self) -> Result<(), BenchmarkError> {
        if self.shots == 0 {
            return Err(BenchmarkError::InvalidExperiment(
                "shots must be greater than zero".to_owned(),
            ));
        }

        if self.max_circuits == 0 {
            return Err(BenchmarkError::InvalidExperiment(
                "max_circuits must be greater than zero".to_owned(),
            ));
        }

        if self.max_total_shots == 0 {
            return Err(BenchmarkError::InvalidExperiment(
                "max_total_shots must be greater than zero".to_owned(),
            ));
        }

        if self.max_parallelism == 0 {
            return Err(BenchmarkError::InvalidExperiment(
                "max_parallelism must be greater than zero".to_owned(),
            ));
        }

        if self.max_total_shots < self.shots {
            return Err(BenchmarkError::InvalidExperiment(
                "max_total_shots cannot be smaller than shots"
                    .to_owned(),
            ));
        }

        if self.max_circuits > MAX_EXPERIMENT_CIRCUITS {
            return Err(BenchmarkError::InvalidExperiment(
                "max_circuits exceeds the structural experiment limit"
                    .to_owned(),
            ));
        }

        if self.max_parallelism > self.max_circuits {
            return Err(BenchmarkError::InvalidExperiment(
                "max_parallelism cannot exceed max_circuits".to_owned(),
            ));
        }

        Ok(())
    }
}

// =============================================================================
// Circuit assignment
// =============================================================================

/// A concrete circuit belonging to an experiment.
///
/// `BenchmarkCircuit` remains the owner of circuit-level information.
/// `ExperimentCircuit` only associates it with the experiment and workload.
#[derive(Debug, Clone, PartialEq)]
pub struct ExperimentCircuit {
    /// Workload that owns this circuit.
    pub workload_id: WorkloadId,

    /// Concrete benchmark circuit.
    pub circuit: BenchmarkCircuit,

    /// Shot override for this circuit.
    ///
    /// `None` means the experiment execution policy supplies the shot count.
    pub shots_override: Option<usize>,

    /// Stable ordinal inside the experiment.
    pub ordinal: usize,
}

impl ExperimentCircuit {
    pub fn new(
        workload_id: WorkloadId,
        circuit: BenchmarkCircuit,
        ordinal: usize,
    ) -> Result<Self, BenchmarkError> {
        if ordinal == 0 {
            return Err(BenchmarkError::InvalidExperiment(
                "experiment circuit ordinal must be greater than zero"
                    .to_owned(),
            ));
        }

        Ok(Self {
            workload_id,
            circuit,
            shots_override: None,
            ordinal,
        })
    }

    pub fn with_shots(
        mut self,
        shots: usize,
    ) -> Result<Self, BenchmarkError> {
        if shots == 0 {
            return Err(BenchmarkError::InvalidExperiment(
                "circuit shot override must be greater than zero"
                    .to_owned(),
            ));
        }

        self.shots_override = Some(shots);
        Ok(self)
    }

    /// Returns the effective shot count.
    pub fn effective_shots(&self, policy: &ExperimentExecutionPolicy) -> usize {
        self.shots_override.unwrap_or(policy.shots)
    }
}

// =============================================================================
// Experiment metadata
// =============================================================================

/// Immutable experiment metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExperimentMetadata {
    /// Human-readable experiment name.
    pub name: String,

    /// Optional description.
    pub description: Option<String>,

    /// Arbitrary deterministic tags.
    pub tags: BTreeMap<String, String>,

    /// User-defined metadata.
    pub metadata: BTreeMap<String, String>,
}

impl ExperimentMetadata {
    /// Creates empty metadata.
    pub fn new() -> Self {
        Self {
            name: String::new(),
            description: None,
            tags: BTreeMap::new(),
            metadata: BTreeMap::new(),
        }
    }

    /// Sets the human-readable name.
    pub fn with_name(
        mut self,
        name: impl Into<String>,
    ) -> Result<Self, BenchmarkError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(BenchmarkError::InvalidExperiment(
                "experiment name cannot be empty".to_owned(),
            ));
        }

        self.name = name;
        Ok(self)
    }

    /// Sets a description.
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Adds a deterministic tag.
    pub fn with_tag(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, BenchmarkError> {
        insert_bounded_map_entry(
            &mut self.tags,
            key.into(),
            value.into(),
            MAX_EXPERIMENT_TAGS,
            MAX_TAG_KEY_LENGTH,
            MAX_TAG_VALUE_LENGTH,
            "experiment tag",
        )?;

        Ok(self)
    }

    /// Adds user metadata.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, BenchmarkError> {
        insert_bounded_map_entry(
            &mut self.metadata,
            key.into(),
            value.into(),
            MAX_METADATA_ENTRIES,
            MAX_METADATA_KEY_LENGTH,
            MAX_METADATA_VALUE_LENGTH,
            "experiment metadata",
        )?;

        Ok(self)
    }

    fn validate(&self) -> Result<(), BenchmarkError> {
        if self.name.len() > MAX_METADATA_VALUE_LENGTH {
            return Err(BenchmarkError::InvalidExperiment(
                "experiment name exceeds maximum length".to_owned(),
            ));
        }

        if self.name.trim().is_empty() {
            return Err(BenchmarkError::InvalidExperiment(
                "experiment name cannot be empty".to_owned(),
            ));
        }

        validate_bounded_map(
            &self.tags,
            MAX_EXPERIMENT_TAGS,
            MAX_TAG_KEY_LENGTH,
            MAX_TAG_VALUE_LENGTH,
            "experiment tag",
        )?;

        validate_bounded_map(
            &self.metadata,
            MAX_METADATA_ENTRIES,
            MAX_METADATA_KEY_LENGTH,
            MAX_METADATA_VALUE_LENGTH,
            "experiment metadata",
        )?;

        Ok(())
    }
}

impl Default for ExperimentMetadata {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Experiment
// =============================================================================

/// Canonical immutable quantum benchmarking experiment definition.
///
/// An `Experiment` is complete enough to be handed to the execution layer
/// without requiring another semantic edit to this file.
///
/// Construction should normally happen through [`ExperimentBuilder`].
#[derive(Debug, Clone, PartialEq)]
pub struct Experiment {
    /// Stable experiment identity.
    id: ExperimentId,

    /// Benchmark protocol identity.
    benchmark_id: BenchmarkId,

    /// Benchmark configuration.
    config: BenchmarkConfig,

    /// Workload definition.
    workload: BenchmarkWorkload,

    /// Concrete circuits generated for this experiment.
    circuits: Vec<ExperimentCircuit>,

    /// Execution policy.
    execution: ExperimentExecutionPolicy,

    /// Reproducibility seed.
    seed: ExperimentSeed,

    /// Human-readable and user-defined metadata.
    metadata: ExperimentMetadata,

    /// Lifecycle state.
    state: ExperimentState,

    /// Optional deterministic configuration fingerprint.
    ///
    /// This field is populated by the reproducibility layer once its
    /// canonical hashing implementation is available. The experiment model
    /// does not invent a hash algorithm.
    configuration_fingerprint: Option<String>,
}

impl Experiment {
    /// Creates an experiment builder.
    pub fn builder() -> ExperimentBuilder {
        ExperimentBuilder::new()
    }

    /// Returns the experiment ID.
    pub fn id(&self) -> &ExperimentId {
        &self.id
    }

    /// Returns the benchmark ID.
    pub fn benchmark_id(&self) -> &BenchmarkId {
        &self.benchmark_id
    }

    /// Returns the benchmark configuration.
    pub fn config(&self) -> &BenchmarkConfig {
        &self.config
    }

    /// Returns the workload.
    pub fn workload(&self) -> &BenchmarkWorkload {
        &self.workload
    }

    /// Returns all concrete circuits.
    pub fn circuits(&self) -> &[ExperimentCircuit] {
        &self.circuits
    }

    /// Returns the execution policy.
    pub fn execution(&self) -> &ExperimentExecutionPolicy {
        &self.execution
    }

    /// Returns the experiment seed.
    pub fn seed(&self) -> ExperimentSeed {
        self.seed
    }

    /// Returns metadata.
    pub fn metadata(&self) -> &ExperimentMetadata {
        &self.metadata
    }

    /// Returns lifecycle state.
    pub fn state(&self) -> ExperimentState {
        self.state
    }

    /// Returns whether this experiment requires actual execution.
    pub fn requires_execution(&self) -> bool {
        self.execution.mode.requires_execution()
    }

    /// Returns whether physical hardware is explicitly required.
    pub fn requires_hardware(&self) -> bool {
        self.execution.mode.requires_hardware()
    }

    /// Returns a reproducibility fingerprint if one has been assigned.
    pub fn configuration_fingerprint(&self) -> Option<&str> {
        self.configuration_fingerprint.as_deref()
    }

    /// Returns total circuit count.
    pub fn circuit_count(&self) -> usize {
        self.circuits.len()
    }

    /// Calculates total effective shots with checked arithmetic.
    pub fn total_shots(&self) -> Result<usize, BenchmarkError> {
        let mut total = 0usize;

        for circuit in &self.circuits {
            let shots = circuit.effective_shots(&self.execution);

            total = total.checked_add(shots).ok_or_else(|| {
                BenchmarkError::InvalidExperiment(
                    "total experiment shots overflow usize".to_owned(),
                )
            })?;
        }

        Ok(total)
    }

    /// Validates the entire experiment against the supplied benchmark limits.
    ///
    /// This method performs structural validation only. Backend capability
    /// validation remains the responsibility of the hardware/execution layer.
    pub fn validate(
        &self,
        limits: &BenchmarkLimits,
    ) -> Result<(), BenchmarkError> {
        validate_identifier(
            self.id.as_str(),
            "experiment ID",
            MAX_EXPERIMENT_ID_LENGTH,
        )?;

        validate_identifier(
            self.benchmark_id.as_str(),
            "benchmark ID",
            MAX_BENCHMARK_ID_LENGTH,
        )?;

        self.config.validate()?;
        self.workload.validate()?;
        self.execution.validate()?;
        self.metadata.validate()?;

        if self.circuits.is_empty() {
            return Err(BenchmarkError::InvalidExperiment(
                "experiment must contain at least one circuit".to_owned(),
            ));
        }

        if self.circuits.len() > MAX_EXPERIMENT_CIRCUITS {
            return Err(BenchmarkError::InvalidExperiment(
                "experiment contains too many circuits".to_owned(),
            ));
        }

        if self.circuits.len() > self.execution.max_circuits {
            return Err(BenchmarkError::InvalidExperiment(
                "circuit count exceeds execution max_circuits".to_owned(),
            ));
        }

        let total_shots = self.total_shots()?;

        if total_shots > self.execution.max_total_shots {
            return Err(BenchmarkError::InvalidExperiment(
                "total experiment shots exceed max_total_shots".to_owned(),
            ));
        }

        validate_circuit_ordinals(&self.circuits)?;

        validate_workload_references(
            &self.circuits,
            self.workload.ids(),
        )?;

        validate_unique_circuit_ids(&self.circuits)?;

        validate_against_limits(
            self,
            limits,
            total_shots,
        )?;

        Ok(())
    }

    /// Returns a validated immutable experiment.
    ///
    /// This does not execute the experiment.
    pub fn validate_and_freeze(
        mut self,
        limits: &BenchmarkLimits,
    ) -> Result<Self, BenchmarkError> {
        self.validate(limits)?;

        self.state = ExperimentState::Validated;

        Ok(self)
    }

    /// Returns a copy marked as running.
    ///
    /// Only the execution coordinator should perform this transition.
    pub fn mark_running(mut self) -> Result<Self, BenchmarkError> {
        if !self.state.can_start() {
            return Err(BenchmarkError::InvalidExperiment(
                "experiment cannot transition to running from its current state"
                    .to_owned(),
            ));
        }

        self.state = ExperimentState::Running;

        Ok(self)
    }

    /// Returns a copy marked as successfully completed.
    pub fn mark_completed(mut self) -> Result<Self, BenchmarkError> {
        if self.state != ExperimentState::Running {
            return Err(BenchmarkError::InvalidExperiment(
                "only a running experiment can be marked completed"
                    .to_owned(),
            ));
        }

        self.state = ExperimentState::Completed;

        Ok(self)
    }

    /// Returns a copy marked as cancelled.
    pub fn mark_cancelled(mut self) -> Result<Self, BenchmarkError> {
        if !self.state.has_started() || self.state.is_terminal() {
            return Err(BenchmarkError::InvalidExperiment(
                "experiment cannot transition to cancelled from its current state"
                    .to_owned(),
            ));
        }

        self.state = ExperimentState::Cancelled;

        Ok(self)
    }

    /// Returns a copy marked as failed.
    pub fn mark_failed(mut self) -> Result<Self, BenchmarkError> {
        if !self.state.has_started() || self.state.is_terminal() {
            return Err(BenchmarkError::InvalidExperiment(
                "experiment cannot transition to failed from its current state"
                    .to_owned(),
            ));
        }

        self.state = ExperimentState::Failed;

        Ok(self)
    }

    /// Returns a copy marked as partially completed.
    pub fn mark_partially_completed(
        mut self,
    ) -> Result<Self, BenchmarkError> {
        if self.state != ExperimentState::Running {
            return Err(BenchmarkError::InvalidExperiment(
                "only a running experiment can be marked partially completed"
                    .to_owned(),
            ));
        }

        self.state = ExperimentState::PartiallyCompleted;

        Ok(self)
    }

    /// Attaches the canonical reproducibility fingerprint.
    ///
    /// The hashing algorithm is deliberately owned by
    /// `core::reproducibility`.
    pub fn with_configuration_fingerprint(
        mut self,
        fingerprint: impl Into<String>,
    ) -> Result<Self, BenchmarkError> {
        let fingerprint = fingerprint.into();

        if fingerprint.trim().is_empty() {
            return Err(BenchmarkError::InvalidExperiment(
                "configuration fingerprint cannot be empty".to_owned(),
            ));
        }

        self.configuration_fingerprint = Some(fingerprint);

        Ok(self)
    }
}

// =============================================================================
// Builder
// =============================================================================

/// Builder for [`Experiment`].
///
/// The builder exists so incomplete experiment definitions cannot accidentally
/// be passed to execution.
#[derive(Debug, Default)]
pub struct ExperimentBuilder {
    id: Option<ExperimentId>,
    benchmark_id: Option<BenchmarkId>,
    config: Option<BenchmarkConfig>,
    workload: Option<BenchmarkWorkload>,
    circuits: Vec<ExperimentCircuit>,
    execution: Option<ExperimentExecutionPolicy>,
    seed: ExperimentSeed,
    metadata: ExperimentMetadata,
}

impl ExperimentBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            benchmark_id: None,
            config: None,
            workload: None,
            circuits: Vec::new(),
            execution: None,
            seed: ExperimentSeed::default(),
            metadata: ExperimentMetadata::default(),
        }
    }

    pub fn id(
        mut self,
        value: impl Into<String>,
    ) -> Result<Self, BenchmarkError> {
        self.id = Some(ExperimentId::new(value)?);
        Ok(self)
    }

    pub fn benchmark(
        mut self,
        value: impl Into<String>,
    ) -> Result<Self, BenchmarkError> {
        self.benchmark_id = Some(BenchmarkId::new(value)?);
        Ok(self)
    }

    pub fn config(mut self, config: BenchmarkConfig) -> Self {
        self.config = Some(config);
        self
    }

    pub fn workload(mut self, workload: BenchmarkWorkload) -> Self {
        self.workload = Some(workload);
        self
    }

    pub fn execution(
        mut self,
        execution: ExperimentExecutionPolicy,
    ) -> Self {
        self.execution = Some(execution);
        self
    }

    pub fn seed(mut self, seed: ExperimentSeed) -> Self {
        self.seed = seed;
        self
    }

    pub fn metadata(mut self, metadata: ExperimentMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    /// Adds a concrete circuit.
    pub fn add_circuit(
        mut self,
        circuit: ExperimentCircuit,
    ) -> Result<Self, BenchmarkError> {
        if self.circuits.len() >= MAX_EXPERIMENT_CIRCUITS {
            return Err(BenchmarkError::InvalidExperiment(
                "experiment contains too many circuits".to_owned(),
            ));
        }

        self.circuits.push(circuit);

        Ok(self)
    }

    /// Adds multiple circuits while checking the structural limit before
    /// extending the vector.
    pub fn add_circuits<I>(
        mut self,
        circuits: I,
    ) -> Result<Self, BenchmarkError>
    where
        I: IntoIterator<Item = ExperimentCircuit>,
    {
        for circuit in circuits {
            self = self.add_circuit(circuit)?;
        }

        Ok(self)
    }

    /// Constructs the experiment.
    ///
    /// This validates the structural invariants available without a
    /// `BenchmarkLimits` instance.
    pub fn build(self) -> Result<Experiment, BenchmarkError> {
        let id = self.id.ok_or_else(|| {
            BenchmarkError::InvalidExperiment(
                "experiment ID is required".to_owned(),
            )
        })?;

        let benchmark_id = self.benchmark_id.ok_or_else(|| {
            BenchmarkError::InvalidExperiment(
                "benchmark ID is required".to_owned(),
            )
        })?;

        let config = self.config.ok_or_else(|| {
            BenchmarkError::InvalidExperiment(
                "benchmark configuration is required".to_owned(),
            )
        })?;

        let workload = self.workload.ok_or_else(|| {
            BenchmarkError::InvalidExperiment(
                "benchmark workload is required".to_owned(),
            )
        })?;

        let execution = self.execution.ok_or_else(|| {
            BenchmarkError::InvalidExperiment(
                "execution policy is required".to_owned(),
            )
        })?;

        let experiment = Experiment {
            id,
            benchmark_id,
            config,
            workload,
            circuits: self.circuits,
            execution,
            seed: self.seed,
            metadata: self.metadata,
            state: ExperimentState::Defined,
            configuration_fingerprint: None,
        };

        /*
         * Perform all structural checks immediately.
         *
         * Backend-specific validation is intentionally deferred until the
         * caller provides BenchmarkLimits and, later, a backend capability
         * profile.
         */
        experiment.config.validate()?;
        experiment.workload.validate()?;
        experiment.execution.validate()?;
        experiment.metadata.validate()?;

        if experiment.circuits.is_empty() {
            return Err(BenchmarkError::InvalidExperiment(
                "experiment must contain at least one circuit".to_owned(),
            ));
        }

        if experiment.circuits.len() > experiment.execution.max_circuits {
            return Err(BenchmarkError::InvalidExperiment(
                "experiment circuit count exceeds max_circuits".to_owned(),
            ));
        }

        validate_circuit_ordinals(&experiment.circuits)?;
        validate_unique_circuit_ids(&experiment.circuits)?;
        validate_workload_references(
            &experiment.circuits,
            experiment.workload.ids(),
        )?;

        let total_shots = experiment.total_shots()?;

        if total_shots > experiment.execution.max_total_shots {
            return Err(BenchmarkError::InvalidExperiment(
                "experiment total shots exceed max_total_shots".to_owned(),
            ));
        }

        Ok(experiment)
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_identifier(
    value: &str,
    field: &str,
    maximum_length: usize,
) -> Result<(), BenchmarkError> {
    if value.trim().is_empty() {
        return Err(BenchmarkError::InvalidExperiment(format!(
            "{} cannot be empty",
            field
        )));
    }

    if value.len() > maximum_length {
        return Err(BenchmarkError::InvalidExperiment(format!(
            "{} exceeds maximum length of {} bytes",
            field, maximum_length
        )));
    }

    /*
     * Reject control characters because identifiers are routinely propagated
     * into:
     *
     * - logs;
     * - JSON;
     * - CSV;
     * - filesystem names;
     * - CI output;
     * - registry keys.
     *
     * Unicode identifiers remain permitted.
     */
    if value.chars().any(char::is_control) {
        return Err(BenchmarkError::InvalidExperiment(format!(
            "{} contains control characters",
            field
        )));
    }

    Ok(())
}

fn insert_bounded_map_entry(
    map: &mut BTreeMap<String, String>,
    key: String,
    value: String,
    maximum_entries: usize,
    maximum_key_length: usize,
    maximum_value_length: usize,
    field: &str,
) -> Result<(), BenchmarkError> {
    if key.trim().is_empty() {
        return Err(BenchmarkError::InvalidExperiment(format!(
            "{} key cannot be empty",
            field
        )));
    }

    if key.len() > maximum_key_length {
        return Err(BenchmarkError::InvalidExperiment(format!(
            "{} key exceeds maximum length",
            field
        )));
    }

    if value.len() > maximum_value_length {
        return Err(BenchmarkError::InvalidExperiment(format!(
            "{} value exceeds maximum length",
            field
        )));
    }

    if key.chars().any(char::is_control) {
        return Err(BenchmarkError::InvalidExperiment(format!(
            "{} key contains control characters",
            field
        )));
    }

    if value.chars().any(char::is_control) {
        return Err(BenchmarkError::InvalidExperiment(format!(
            "{} value contains control characters",
            field
        )));
    }

    if !map.contains_key(&key) && map.len() >= maximum_entries {
        return Err(BenchmarkError::InvalidExperiment(format!(
            "{} entry limit exceeded",
            field
        )));
    }

    map.insert(key, value);

    Ok(())
}

fn validate_bounded_map(
    map: &BTreeMap<String, String>,
    maximum_entries: usize,
    maximum_key_length: usize,
    maximum_value_length: usize,
    field: &str,
) -> Result<(), BenchmarkError> {
    if map.len() > maximum_entries {
        return Err(BenchmarkError::InvalidExperiment(format!(
            "{} entry limit exceeded",
            field
        )));
    }

    for (key, value) in map {
        if key.trim().is_empty() {
            return Err(BenchmarkError::InvalidExperiment(format!(
                "{} key cannot be empty",
                field
            )));
        }

        if key.len() > maximum_key_length {
            return Err(BenchmarkError::InvalidExperiment(format!(
                "{} key exceeds maximum length",
                field
            )));
        }

        if value.len() > maximum_value_length {
            return Err(BenchmarkError::InvalidExperiment(format!(
                "{} value exceeds maximum length",
                field
            )));
        }

        if key.chars().any(char::is_control)
            || value.chars().any(char::is_control)
        {
            return Err(BenchmarkError::InvalidExperiment(format!(
                "{} contains control characters",
                field
            )));
        }
    }

    Ok(())
}

fn validate_circuit_ordinals(
    circuits: &[ExperimentCircuit],
) -> Result<(), BenchmarkError> {
    let mut expected = 1usize;

    for circuit in circuits {
        if circuit.ordinal != expected {
            return Err(BenchmarkError::InvalidExperiment(format!(
                "experiment circuit ordinals must be contiguous starting at 1; \
                 expected {}, found {}",
                expected, circuit.ordinal
            )));
        }

        expected = expected.checked_add(1).ok_or_else(|| {
            BenchmarkError::InvalidExperiment(
                "circuit ordinal overflow".to_owned(),
            )
        })?;
    }

    Ok(())
}

fn validate_unique_circuit_ids(
    circuits: &[ExperimentCircuit],
) -> Result<(), BenchmarkError> {
    let mut ids = BTreeMap::<String, usize>::new();

    for circuit in circuits {
        /*
         * `BenchmarkCircuit::id()` is part of the core/circuit contract.
         *
         * Storing the ordinal makes a duplicate-circuit diagnostic
         * actionable without requiring circuit formatting.
         */
        let id = circuit.circuit.id().to_string();

        if let Some(previous_ordinal) = ids.insert(id.clone(), circuit.ordinal)
        {
            return Err(BenchmarkError::InvalidExperiment(format!(
                "duplicate benchmark circuit ID '{}' at ordinals {} and {}",
                id, previous_ordinal, circuit.ordinal
            )));
        }
    }

    Ok(())
}

fn validate_workload_references(
    circuits: &[ExperimentCircuit],
    workload_ids: &[WorkloadId],
) -> Result<(), BenchmarkError> {
    let mut known = BTreeMap::<&str, ()>::new();

    for id in workload_ids {
        known.insert(id.as_str(), ());
    }

    for circuit in circuits {
        if !known.contains_key(circuit.workload_id.as_str()) {
            return Err(BenchmarkError::InvalidExperiment(format!(
                "circuit at ordinal {} references unknown workload '{}'",
                circuit.ordinal, circuit.workload_id
            )));
        }
    }

    Ok(())
}

fn validate_against_limits(
    experiment: &Experiment,
    limits: &BenchmarkLimits,
    total_shots: usize,
) -> Result<(), BenchmarkError> {
    /*
     * These method names are intentionally the canonical contract for
     * BenchmarkLimits. The limits module owns policy; experiment.rs only
     * delegates to it.
     */

    if !limits.allows_circuits(experiment.circuits.len()) {
        return Err(BenchmarkError::InvalidExperiment(
            "experiment exceeds configured benchmark circuit limit"
                .to_owned(),
        ));
    }

    if !limits.allows_shots(total_shots) {
        return Err(BenchmarkError::InvalidExperiment(
            "experiment exceeds configured benchmark shot limit".to_owned(),
        ));
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /*
     * These helper constructors deliberately live only in tests.
     *
     * Production code must use the canonical implementations supplied by
     * core::config, core::workload, and core::circuit.
     */

    #[test]
    fn experiment_id_rejects_empty_values() {
        let result = ExperimentId::new("");

        assert!(result.is_err());
    }

    #[test]
    fn experiment_id_rejects_control_characters() {
        let result = ExperimentId::new("experiment\n1");

        assert!(result.is_err());
    }

    #[test]
    fn seed_is_deterministic() {
        let a = ExperimentSeed::new(42);
        let b = ExperimentSeed::new(42);

        assert_eq!(a, b);
        assert_eq!(a.value(), 42);
    }

    #[test]
    fn execution_policy_rejects_zero_shots() {
        let result = ExperimentExecutionPolicy::new(0);

        assert!(result.is_err());
    }

    #[test]
    fn execution_policy_rejects_zero_parallelism() {
        let policy = ExperimentExecutionPolicy::new(100)
            .expect("valid initial policy");

        let result = policy.with_max_parallelism(0);

        assert!(result.is_err());
    }

    #[test]
    fn execution_policy_rejects_total_shots_below_per_circuit_shots() {
        let policy = ExperimentExecutionPolicy::new(100)
            .expect("valid initial policy")
            .with_max_total_shots(50);

        assert!(policy.is_err());
    }

    #[test]
    fn execution_modes_have_correct_execution_semantics() {
        assert!(!ExecutionMode::GenerateOnly.requires_execution());
        assert!(ExecutionMode::Simulator.requires_execution());
        assert!(ExecutionMode::Hardware.requires_hardware());
        assert!(!ExecutionMode::Simulator.requires_hardware());
    }

    #[test]
    fn failure_policy_default_is_fail_fast() {
        assert_eq!(
            FailurePolicy::default(),
            FailurePolicy::FailFast
        );
    }

    #[test]
    fn metadata_rejects_empty_name() {
        let result = ExperimentMetadata::new().with_name("");

        assert!(result.is_err());
    }

    #[test]
    fn metadata_accepts_valid_tags() {
        let metadata = ExperimentMetadata::new()
            .with_name("Quantum Volume experiment")
            .expect("valid name")
            .with_tag("backend", "simulator")
            .expect("valid tag");

        assert_eq!(
            metadata.tags.get("backend"),
            Some(&"simulator".to_owned())
        );
    }

    #[test]
    fn metadata_rejects_control_characters() {
        let result = ExperimentMetadata::new()
            .with_name("experiment")
            .expect("valid name")
            .with_tag("backend\n", "simulator");

        assert!(result.is_err());
    }

    #[test]
    fn experiment_state_start_rules_are_explicit() {
        assert!(ExperimentState::Defined.can_start());
        assert!(ExperimentState::Validated.can_start());

        assert!(!ExperimentState::Running.can_start());
        assert!(!ExperimentState::Completed.can_start());
        assert!(!ExperimentState::Failed.can_start());
    }

    #[test]
    fn terminal_states_are_explicit() {
        assert!(ExperimentState::Completed.is_terminal());
        assert!(ExperimentState::Cancelled.is_terminal());
        assert!(ExperimentState::Failed.is_terminal());
        assert!(ExperimentState::PartiallyCompleted.is_terminal());

        assert!(!ExperimentState::Running.is_terminal());
    }

    #[test]
    fn identifier_display_is_stable() {
        let id =
            ExperimentId::new("qv-20q-seed42").expect("valid identifier");

        assert_eq!(id.to_string(), "qv-20q-seed42");
        assert_eq!(id.as_str(), "qv-20q-seed42");
    }
}