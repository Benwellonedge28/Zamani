//! Zamani Quantum Benchmarking — Production Benchmark Registry
//!
//! This module is the authoritative registry for benchmark implementations.
//!
//! # Responsibilities
//!
//! The registry is responsible for:
//!
//! - registering benchmark descriptors;
//! - enforcing globally unique benchmark IDs;
//! - supporting stable aliases;
//! - looking up benchmark metadata without constructing a benchmark;
//! - constructing benchmarks on demand;
//! - deterministic enumeration;
//! - capability/category filtering;
//! - compatibility checks;
//! - preventing accidental duplicate registration;
//! - validating registry invariants;
//! - providing a thread-safe-by-ownership design;
//! - avoiding process-global mutable state;
//! - keeping protocol implementations independent from registry internals.
//!
//! The registry deliberately does NOT:
//!
//! - execute benchmarks;
//! - generate circuits;
//! - communicate with hardware;
//! - perform statistical analysis;
//! - serialize benchmark results;
//! - own Quantum IR;
//! - contain protocol mathematics;
//! - dynamically load arbitrary code.
//!
//! Those responsibilities remain in their owning modules.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani language
//!                               │
//!                               ▼
//!                    stdlib::quantum / frontend
//!                               │
//!                               ▼
//!                    quantum::benchmarking
//!                               │
//!                    ┌──────────┴──────────┐
//!                    ▼                     ▼
//!               BenchmarkRegistry      Benchmark
//!                    │                     │
//!                    │              protocol implementation
//!                    │                     │
//!                    └──────────┬──────────┘
//!                               ▼
//!                    execution / analysis
//! ```
//!
//! # Registration model
//!
//! A benchmark is registered using a [`BenchmarkDescriptor`].
//!
//! The descriptor contains immutable metadata and a constructor function:
//!
//! ```text
//! BenchmarkDescriptor
//! ├── metadata
//! ├── factory
//! ├── aliases
//! ├── capabilities
//! └── execution targets
//! ```
//!
//! The registry stores descriptors by canonical benchmark ID.
//!
//! # Important production property
//!
//! Registration does not instantiate the benchmark.
//!
//! This means:
//!
//! - registration is cheap;
//! - registration has no execution side effects;
//! - registration does not allocate protocol state;
//! - metadata can be inspected without constructing a benchmark;
//! - startup remains deterministic;
//! - invalid duplicate IDs are rejected immediately.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//!
//! No nightly features are required.
//!
//! # Integration contract
//!
//! This file integrates with:
//!
//! - `crate::quantum::benchmarking::core::benchmark::Benchmark`
//! - `crate::quantum::benchmarking::core::benchmark::BenchmarkMetadata`
//! - `crate::quantum::benchmarking::core::benchmark::BenchmarkCategory`
//! - `registry/builtin.rs` for built-in registrations;
//! - `registry/compatibility.rs` for richer backend compatibility policy.
//!
//! The registry itself intentionally does not import concrete protocol files.
//! `builtin.rs` owns that dependency direction.
//!
//! # Dependency direction
//!
//! ```text
//! protocols/* ────────┐
//! applications/* ────┤
//! qec/* ─────────────┤
//!                       ▼
//!                  registry/builtin.rs
//!                       │
//!                       ▼
//!                 BenchmarkRegistry
//! ```
//!
//! The reverse dependency must never be introduced:
//!
//! ```text
//! protocol → registry → protocol
//! ```
//!
//! Such a cycle would make independent benchmark development impossible.
//!
//! # Determinism
//!
//! The registry uses `BTreeMap` rather than `HashMap` so enumeration is stable.
//! This is important for:
//!
//! - reproducible reports;
//! - CLI output;
//! - language tooling;
//! - documentation generation;
//! - deterministic tests;
//! - benchmark manifests;
//! - registry fingerprints.
//!
//! # Security/resource properties
//!
//! The registry:
//!
//! - rejects empty identifiers;
//! - rejects invalid identifiers;
//! - rejects duplicate canonical IDs;
//! - rejects alias collisions;
//! - rejects aliases that collide with canonical IDs;
//! - prevents an identifier from being registered twice through aliases;
//! - does not execute constructors during lookup;
//! - does not use global mutable state;
//! - does not dynamically load untrusted libraries.
//!
//! User-provided Zamani benchmark definitions must be compiled/validated by
//! the frontend and supplied through an explicit trusted registration boundary.
//! This registry is not a plugin loader and must not be treated as one.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::sync::Arc;

use super::super::core::benchmark::{
    Benchmark, BenchmarkCategory, BenchmarkId, BenchmarkMetadata, BenchmarkVersion,
};

// =============================================================================
// Public constants
// =============================================================================

/// Stable schema version for this registry contract.
///
/// This is independent from individual benchmark protocol versions.
pub const REGISTRY_SCHEMA_VERSION: &str = "1.0.0";

/// Stable component identifier.
pub const REGISTRY_COMPONENT_ID: &str = "zamani.quantum.benchmark.registry";

/// Maximum number of aliases allowed for one benchmark.
///
/// This is intentionally bounded so malformed registration data cannot create
/// unbounded registry state.
pub const MAX_ALIASES_PER_BENCHMARK: usize = 32;

/// Maximum length of an identifier.
pub const MAX_IDENTIFIER_LENGTH: usize = 128;

/// Maximum length of a human-readable benchmark description retained by the
/// registry.
pub const MAX_DESCRIPTION_LENGTH: usize = 16 * 1024;

/// Maximum number of descriptors in one registry.
///
/// This is deliberately generous for a language/runtime registry while still
/// protecting against accidental unbounded registration.
pub const DEFAULT_MAX_BENCHMARKS: usize = 4096;

// =============================================================================
// Factory
// =============================================================================

/// Constructor for a benchmark implementation.
///
/// The registry stores the constructor without executing it.
///
/// The constructor must return a fresh benchmark instance. It must not return
/// shared mutable global state.
///
/// Function pointers are used instead of closures so descriptors remain:
///
/// - cheap to clone;
/// - `Send`;
/// - `Sync`;
/// - deterministic;
/// - free from hidden captured state.
///
/// A protocol that requires configuration should obtain that configuration
/// from the benchmark execution/configuration layer rather than capturing
/// mutable state in this factory.
pub type BenchmarkFactory = fn() -> Box<dyn Benchmark>;

// =============================================================================
// Capability model
// =============================================================================

/// Execution capability required or supported by a benchmark.
///
/// Capabilities are deliberately protocol-neutral and backend-neutral.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BenchmarkCapability {
    /// Ordinary gate-model circuits.
    GateModel,

    /// Dynamic circuits / mid-circuit operations.
    DynamicCircuits,

    /// Parameterized circuits.
    ParameterizedCircuits,

    /// Randomized workload generation.
    RandomizedWorkload,

    /// Deterministic workload generation.
    DeterministicWorkload,

    /// Classical sampling/count measurements.
    Sampling,

    /// Expectation-value measurements.
    ExpectationValues,

    /// Access to an ideal/reference probability distribution.
    IdealReferenceDistribution,

    /// State-vector access.
    StateVector,

    /// Density-matrix access.
    DensityMatrix,

    /// Pulse-level execution.
    PulseLevel,

    /// Analog quantum execution.
    Analog,

    /// Quantum annealing.
    Annealing,

    /// Physical error-correction execution.
    PhysicalQec,

    /// Logical-qubit execution.
    LogicalQubits,

    /// Syndrome extraction.
    SyndromeExtraction,

    /// Decoder execution.
    Decoder,

    /// Calibration metadata.
    CalibrationMetadata,

    /// Hardware timing metadata.
    TimingMetadata,

    /// Hardware topology metadata.
    TopologyMetadata,

    /// Readout characterization.
    ReadoutCharacterization,

    /// Coherence characterization.
    CoherenceCharacterization,

    /// Crosstalk characterization.
    CrosstalkCharacterization,

    /// Long-duration drift characterization.
    DriftCharacterization,

    /// Classical optimization loop.
    ClassicalOptimization,

    /// Offline analysis of already captured observations.
    OfflineAnalysis,
}

impl BenchmarkCapability {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GateModel => "gate_model",
            Self::DynamicCircuits => "dynamic_circuits",
            Self::ParameterizedCircuits => "parameterized_circuits",
            Self::RandomizedWorkload => "randomized_workload",
            Self::DeterministicWorkload => "deterministic_workload",
            Self::Sampling => "sampling",
            Self::ExpectationValues => "expectation_values",
            Self::IdealReferenceDistribution => "ideal_reference_distribution",
            Self::StateVector => "state_vector",
            Self::DensityMatrix => "density_matrix",
            Self::PulseLevel => "pulse_level",
            Self::Analog => "analog",
            Self::Annealing => "annealing",
            Self::PhysicalQec => "physical_qec",
            Self::LogicalQubits => "logical_qubits",
            Self::SyndromeExtraction => "syndrome_extraction",
            Self::Decoder => "decoder",
            Self::CalibrationMetadata => "calibration_metadata",
            Self::TimingMetadata => "timing_metadata",
            Self::TopologyMetadata => "topology_metadata",
            Self::ReadoutCharacterization => "readout_characterization",
            Self::CoherenceCharacterization => "coherence_characterization",
            Self::CrosstalkCharacterization => "crosstalk_characterization",
            Self::DriftCharacterization => "drift_characterization",
            Self::ClassicalOptimization => "classical_optimization",
            Self::OfflineAnalysis => "offline_analysis",
        }
    }
}

impl fmt::Display for BenchmarkCapability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Execution target
// =============================================================================

/// Broad execution target supported by a benchmark.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BenchmarkExecutionTarget {
    /// CPU simulator.
    CpuSimulator,

    /// GPU simulator.
    GpuSimulator,

    /// State-vector simulator.
    StateVectorSimulator,

    /// Density-matrix simulator.
    DensityMatrixSimulator,

    /// Tensor-network simulator.
    TensorNetworkSimulator,

    /// Stabilizer simulator.
    StabilizerSimulator,

    /// Physical gate-model quantum hardware.
    GateModelHardware,

    /// Analog quantum hardware.
    AnalogHardware,

    /// Quantum annealing hardware.
    AnnealingHardware,

    /// Logical-qubit/fault-tolerant backend.
    LogicalQuantumHardware,

    /// External/custom execution provider.
    External,
}

impl BenchmarkExecutionTarget {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CpuSimulator => "cpu_simulator",
            Self::GpuSimulator => "gpu_simulator",
            Self::StateVectorSimulator => "state_vector_simulator",
            Self::DensityMatrixSimulator => "density_matrix_simulator",
            Self::TensorNetworkSimulator => "tensor_network_simulator",
            Self::StabilizerSimulator => "stabilizer_simulator",
            Self::GateModelHardware => "gate_model_hardware",
            Self::AnalogHardware => "analog_hardware",
            Self::AnnealingHardware => "annealing_hardware",
            Self::LogicalQuantumHardware => "logical_quantum_hardware",
            Self::External => "external",
        }
    }
}

impl fmt::Display for BenchmarkExecutionTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Registry error
// =============================================================================

/// Errors produced by benchmark registry operations.
///
/// These errors deliberately live in this file rather than requiring
/// `core/errors.rs` to know about registry-specific invariants.
///
/// This keeps the registry independently completable and prevents a future
/// registry change from forcing unrelated changes to the global benchmark
/// error hierarchy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryError {
    /// Registry has reached its configured capacity.
    CapacityExceeded {
        /// Maximum number of benchmarks allowed.
        maximum: usize,
    },

    /// Canonical benchmark ID is invalid.
    InvalidBenchmarkId {
        /// Invalid value.
        value: String,

        /// Human-readable reason.
        reason: String,
    },

    /// Alias is invalid.
    InvalidAlias {
        /// Invalid alias.
        value: String,

        /// Human-readable reason.
        reason: String,
    },

    /// Canonical benchmark ID already exists.
    DuplicateBenchmarkId {
        /// Existing identifier.
        id: String,
    },

    /// Alias already exists.
    DuplicateAlias {
        /// Existing alias.
        alias: String,
    },

    /// Alias conflicts with a canonical benchmark ID.
    AliasConflictsWithBenchmark {
        /// Alias.
        alias: String,

        /// Conflicting canonical ID.
        benchmark_id: String,
    },

    /// Benchmark has too many aliases.
    TooManyAliases {
        /// Benchmark ID.
        benchmark_id: String,

        /// Number supplied.
        count: usize,

        /// Maximum allowed.
        maximum: usize,
    },

    /// Metadata does not match the descriptor ID.
    MetadataIdMismatch {
        /// Descriptor ID.
        descriptor_id: String,

        /// Metadata ID.
        metadata_id: String,
    },

    /// Metadata contains an invalid field.
    InvalidMetadata {
        /// Field name.
        field: String,

        /// Reason.
        reason: String,
    },

    /// A requested benchmark does not exist.
    BenchmarkNotFound {
        /// Requested identifier.
        id: String,
    },

    /// A requested alias does not resolve.
    AliasNotFound {
        /// Requested alias.
        alias: String,
    },

    /// Factory construction failed at the registry boundary.
///
/// The factory type itself is infallible, but the constructed benchmark can
/// still violate the descriptor invariant. This error is therefore used when
/// validation detects that condition.
    FactoryContractViolation {
        /// Benchmark ID.
        benchmark_id: String,

        /// Reason.
        reason: String,
    },

    /// Registry invariant violation.
    InvariantViolation {
        /// Human-readable reason.
        reason: String,
    },
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CapacityExceeded { maximum } => {
                write!(f, "benchmark registry capacity exceeded (maximum {maximum})")
            }

            Self::InvalidBenchmarkId { value, reason } => {
                write!(f, "invalid benchmark ID `{value}`: {reason}")
            }

            Self::InvalidAlias { value, reason } => {
                write!(f, "invalid benchmark alias `{value}`: {reason}")
            }

            Self::DuplicateBenchmarkId { id } => {
                write!(f, "benchmark ID `{id}` is already registered")
            }

            Self::DuplicateAlias { alias } => {
                write!(f, "benchmark alias `{alias}` is already registered")
            }

            Self::AliasConflictsWithBenchmark {
                alias,
                benchmark_id,
            } => {
                write!(
                    f,
                    "benchmark alias `{alias}` conflicts with benchmark ID `{benchmark_id}`"
                )
            }

            Self::TooManyAliases {
                benchmark_id,
                count,
                maximum,
            } => {
                write!(
                    f,
                    "benchmark `{benchmark_id}` declares {count} aliases; maximum is {maximum}"
                )
            }

            Self::MetadataIdMismatch {
                descriptor_id,
                metadata_id,
            } => {
                write!(
                    f,
                    "descriptor ID `{descriptor_id}` does not match metadata ID `{metadata_id}`"
                )
            }

            Self::InvalidMetadata { field, reason } => {
                write!(f, "invalid benchmark metadata `{field}`: {reason}")
            }

            Self::BenchmarkNotFound { id } => {
                write!(f, "benchmark `{id}` is not registered")
            }

            Self::AliasNotFound { alias } => {
                write!(f, "benchmark alias `{alias}` is not registered")
            }

            Self::FactoryContractViolation {
                benchmark_id,
                reason,
            } => {
                write!(
                    f,
                    "benchmark factory for `{benchmark_id}` violated registry contract: {reason}"
                )
            }

            Self::InvariantViolation { reason } => {
                write!(f, "benchmark registry invariant violation: {reason}")
            }
        }
    }
}

impl std::error::Error for RegistryError {}

// =============================================================================
// Benchmark descriptor
// =============================================================================

/// Immutable registration information for one benchmark implementation.
///
/// A descriptor is intentionally independent of the benchmark's runtime
/// configuration. It describes *what the benchmark is*, not *how a particular
/// experiment should run*.
#[derive(Clone)]
pub struct BenchmarkDescriptor {
    /// Canonical benchmark ID.
    id: String,

    /// Immutable benchmark metadata.
    metadata: BenchmarkMetadata,

    /// Constructor for the concrete implementation.
    factory: BenchmarkFactory,

    /// Stable aliases.
    aliases: Vec<String>,

    /// Declared capabilities.
    capabilities: BTreeSet<BenchmarkCapability>,

    /// Declared execution targets.
    execution_targets: BTreeSet<BenchmarkExecutionTarget>,
}

impl fmt::Debug for BenchmarkDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BenchmarkDescriptor")
            .field("id", &self.id)
            .field("metadata", &self.metadata)
            .field("aliases", &self.aliases)
            .field("capabilities", &self.capabilities)
            .field("execution_targets", &self.execution_targets)
            .finish_non_exhaustive()
    }
}

impl BenchmarkDescriptor {
    /// Creates a descriptor.
    ///
    /// The constructor performs all descriptor-local validation so invalid
    /// descriptors cannot enter a registry.
    pub fn new(
        id: impl Into<String>,
        metadata: BenchmarkMetadata,
        factory: BenchmarkFactory,
    ) -> Result<Self, RegistryError> {
        let id = normalize_identifier(id.into());

        validate_identifier(&id, "benchmark ID")?;

        let metadata_id = metadata.id.as_str();

        if metadata_id != id {
            return Err(RegistryError::MetadataIdMismatch {
                descriptor_id: id,
                metadata_id: metadata_id.to_owned(),
            });
        }

        validate_metadata(&metadata)?;

        Ok(Self {
            id,
            metadata,
            factory,
            aliases: Vec::new(),
            capabilities: BTreeSet::new(),
            execution_targets: BTreeSet::new(),
        })
    }

    /// Adds one alias.
    ///
    /// Alias validation occurs immediately. Registry-level collision checking
    /// occurs when the descriptor is registered.
    #[must_use]
    pub fn with_alias(mut self, alias: impl Into<String>) -> Result<Self, RegistryError> {
        let alias = normalize_identifier(alias.into());

        validate_identifier(&alias, "benchmark alias")?;

        if alias == self.id {
            return Err(RegistryError::AliasConflictsWithBenchmark {
                alias,
                benchmark_id: self.id.clone(),
            });
        }

        if self.aliases.iter().any(|existing| existing == &alias) {
            return Err(RegistryError::DuplicateAlias { alias });
        }

        if self.aliases.len() >= MAX_ALIASES_PER_BENCHMARK {
            return Err(RegistryError::TooManyAliases {
                benchmark_id: self.id.clone(),
                count: self.aliases.len() + 1,
                maximum: MAX_ALIASES_PER_BENCHMARK,
            });
        }

        self.aliases.push(alias);
        self.aliases.sort();

        Ok(self)
    }

    /// Adds several aliases.
    #[must_use]
    pub fn with_aliases<I, S>(mut self, aliases: I) -> Result<Self, RegistryError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for alias in aliases {
            self = self.with_alias(alias)?;
        }

        Ok(self)
    }

    /// Adds a capability.
    #[must_use]
    pub fn with_capability(mut self, capability: BenchmarkCapability) -> Self {
        self.capabilities.insert(capability);
        self
    }

    /// Adds several capabilities.
    #[must_use]
    pub fn with_capabilities<I>(mut self, capabilities: I) -> Self
    where
        I: IntoIterator<Item = BenchmarkCapability>,
    {
        self.capabilities.extend(capabilities);
        self
    }

    /// Adds an execution target.
    #[must_use]
    pub fn with_execution_target(mut self, target: BenchmarkExecutionTarget) -> Self {
        self.execution_targets.insert(target);
        self
    }

    /// Adds several execution targets.
    #[must_use]
    pub fn with_execution_targets<I>(mut self, targets: I) -> Self
    where
        I: IntoIterator<Item = BenchmarkExecutionTarget>,
    {
        self.execution_targets.extend(targets);
        self
    }

    /// Returns the canonical benchmark ID.
    #[inline]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns benchmark metadata.
    #[inline]
    pub fn metadata(&self) -> &BenchmarkMetadata {
        &self.metadata
    }

    /// Returns benchmark aliases.
    #[inline]
    pub fn aliases(&self) -> &[String] {
        &self.aliases
    }

    /// Returns declared capabilities.
    #[inline]
    pub fn capabilities(&self) -> &BTreeSet<BenchmarkCapability> {
        &self.capabilities
    }

    /// Returns declared execution targets.
    #[inline]
    pub fn execution_targets(&self) -> &BTreeSet<BenchmarkExecutionTarget> {
        &self.execution_targets
    }

    /// Returns true if the descriptor declares a capability.
    #[inline]
    pub fn supports(&self, capability: BenchmarkCapability) -> bool {
        self.capabilities.contains(&capability)
    }

    /// Returns true if the descriptor supports the target.
    #[inline]
    pub fn supports_target(&self, target: BenchmarkExecutionTarget) -> bool {
        self.execution_targets.contains(&target)
    }

    /// Constructs a new benchmark instance.
    ///
    /// Construction happens only here and never during lookup or registration.
    pub fn create(&self) -> Result<Box<dyn Benchmark>, RegistryError> {
        let benchmark = (self.factory)();

        validate_factory_identity(&self.id, benchmark.as_ref())?;

        Ok(benchmark)
    }
}

// =============================================================================
// Registry query
// =============================================================================

/// Query used to filter registered benchmarks.
///
/// Empty optional filters mean "no restriction".
#[derive(Debug, Clone, Default)]
pub struct BenchmarkRegistryQuery {
    /// Optional category filter.
    pub category: Option<BenchmarkCategory>,

    /// Required capabilities.
    pub required_capabilities: BTreeSet<BenchmarkCapability>,

    /// Required execution target.
    pub execution_target: Option<BenchmarkExecutionTarget>,

    /// Require deterministic generation.
    pub deterministic_generation: Option<bool>,

    /// Require offline analysis.
    pub offline_analysis: Option<bool>,

    /// Require simulator support.
    pub simulator_supported: Option<bool>,

    /// Require hardware support.
    pub hardware_supported: Option<bool>,
}

impl BenchmarkRegistryQuery {
    /// Creates an empty query.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Filters by category.
    #[must_use]
    pub fn category(mut self, category: BenchmarkCategory) -> Self {
        self.category = Some(category);
        self
    }

    /// Requires a capability.
    #[must_use]
    pub fn require_capability(mut self, capability: BenchmarkCapability) -> Self {
        self.required_capabilities.insert(capability);
        self
    }

    /// Requires an execution target.
    #[must_use]
    pub fn execution_target(mut self, target: BenchmarkExecutionTarget) -> Self {
        self.execution_target = Some(target);
        self
    }

    /// Filters deterministic generation support.
    #[must_use]
    pub fn deterministic_generation(mut self, value: bool) -> Self {
        self.deterministic_generation = Some(value);
        self
    }

    /// Filters offline analysis support.
    #[must_use]
    pub fn offline_analysis(mut self, value: bool) -> Self {
        self.offline_analysis = Some(value);
        self
    }

    /// Filters simulator support.
    #[must_use]
    pub fn simulator_supported(mut self, value: bool) -> Self {
        self.simulator_supported = Some(value);
        self
    }

    /// Filters hardware support.
    #[must_use]
    pub fn hardware_supported(mut self, value: bool) -> Self {
        self.hardware_supported = Some(value);
        self
    }

    fn matches(&self, descriptor: &BenchmarkDescriptor) -> bool {
        let metadata = descriptor.metadata();

        if let Some(category) = self.category {
            if metadata.category != category {
                return false;
            }
        }

        if !self
            .required_capabilities
            .iter()
            .all(|capability| descriptor.supports(*capability))
        {
            return false;
        }

        if let Some(target) = self.execution_target {
            if !descriptor.supports_target(target) {
                return false;
            }
        }

        if let Some(value) = self.deterministic_generation {
            if metadata.deterministic_generation != value {
                return false;
            }
        }

        if let Some(value) = self.offline_analysis {
            if metadata.offline_analysis != value {
                return false;
            }
        }

        if let Some(value) = self.simulator_supported {
            if metadata.simulator_supported != value {
                return false;
            }
        }

        if let Some(value) = self.hardware_supported {
            if metadata.hardware_supported != value {
                return false;
            }
        }

        true
    }
}

// =============================================================================
// Registry entry
// =============================================================================

#[derive(Clone)]
struct RegistryEntry {
    descriptor: Arc<BenchmarkDescriptor>,
}

// =============================================================================
// Benchmark registry
// =============================================================================

/// Production benchmark registry.
///
/// The registry owns descriptors but not benchmark instances.
///
/// It is intentionally an ordinary value rather than a process-global
/// singleton. Applications that need a shared registry should own one in the
/// appropriate runtime/service context and pass references explicitly.
///
/// Because all stored state is immutable after registration and uses
/// deterministic ordered collections, a completed registry is safe to share
/// through `Arc`.
#[derive(Debug)]
pub struct BenchmarkRegistry {
    entries: BTreeMap<String, RegistryEntry>,
    aliases: BTreeMap<String, String>,
    max_benchmarks: usize,
}

impl BenchmarkRegistry {
    /// Creates an empty registry with the default capacity.
    #[must_use]
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_MAX_BENCHMARKS)
    }

    /// Creates an empty registry with an explicit capacity.
    ///
    /// Capacity must be greater than zero.
    #[must_use]
    pub fn with_capacity(max_benchmarks: usize) -> Self {
        assert!(
            max_benchmarks > 0,
            "benchmark registry capacity must be greater than zero"
        );

        Self {
            entries: BTreeMap::new(),
            aliases: BTreeMap::new(),
            max_benchmarks,
        }
    }

    /// Returns the configured maximum number of benchmarks.
    #[inline]
    pub const fn max_benchmarks(&self) -> usize {
        self.max_benchmarks
    }

    /// Returns the number of registered canonical benchmarks.
    #[inline]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true when no benchmarks are registered.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the number of aliases.
    #[inline]
    pub fn alias_count(&self) -> usize {
        self.aliases.len()
    }

    /// Registers a benchmark descriptor.
    ///
    /// Registration is atomic from the caller's perspective: all validation
    /// is completed before any registry mutation occurs.
    pub fn register(&mut self, descriptor: BenchmarkDescriptor) -> Result<(), RegistryError> {
        descriptor_validate(&descriptor)?;

        if self.entries.len() >= self.max_benchmarks {
            return Err(RegistryError::CapacityExceeded {
                maximum: self.max_benchmarks,
            });
        }

        let id = descriptor.id().to_owned();

        if self.entries.contains_key(&id) {
            return Err(RegistryError::DuplicateBenchmarkId { id });
        }

        for alias in descriptor.aliases() {
            if self.entries.contains_key(alias) {
                return Err(RegistryError::AliasConflictsWithBenchmark {
                    alias: alias.clone(),
                    benchmark_id: alias.clone(),
                });
            }

            if self.aliases.contains_key(alias) {
                return Err(RegistryError::DuplicateAlias {
                    alias: alias.clone(),
                });
            }

            if alias == &id {
                return Err(RegistryError::AliasConflictsWithBenchmark {
                    alias: alias.clone(),
                    benchmark_id: id.clone(),
                });
            }
        }

        let descriptor = Arc::new(descriptor);

        self.entries.insert(
            id.clone(),
            RegistryEntry {
                descriptor: Arc::clone(&descriptor),
            },
        );

        for alias in descriptor.aliases() {
            self.aliases.insert(alias.clone(), id.clone());
        }

        debug_assert!(self.validate().is_ok());

        Ok(())
    }

    /// Registers several descriptors.
    ///
    /// Registration is transactional with respect to registry contents:
    /// descriptors are validated against a temporary registry first. If any
    /// descriptor fails, the original registry remains unchanged.
    pub fn register_all<I>(&mut self, descriptors: I) -> Result<(), RegistryError>
    where
        I: IntoIterator<Item = BenchmarkDescriptor>,
    {
        let mut candidate = self.clone();

        for descriptor in descriptors {
            candidate.register(descriptor)?;
        }

        *self = candidate;

        Ok(())
    }

    /// Removes a benchmark and all of its aliases.
    ///
    /// This is primarily intended for controlled test/plugin-management
    /// environments. Production applications should generally construct a
    /// registry once and treat it as immutable afterwards.
    pub fn unregister(&mut self, id_or_alias: &str) -> Result<BenchmarkDescriptor, RegistryError> {
        let canonical_id = self.resolve(id_or_alias)?;

        let entry = self
            .entries
            .remove(canonical_id)
            .ok_or_else(|| RegistryError::BenchmarkNotFound {
                id: canonical_id.to_owned(),
            })?;

        let aliases_to_remove = entry.descriptor.aliases().to_vec();

        for alias in aliases_to_remove {
            self.aliases.remove(&alias);
        }

        Arc::try_unwrap(entry.descriptor).map_err(|_| RegistryError::InvariantViolation {
            reason: "benchmark descriptor is still referenced internally".to_owned(),
        })
    }

    /// Resolves a canonical ID or alias to its canonical benchmark ID.
    pub fn resolve(&self, id_or_alias: &str) -> Result<&str, RegistryError> {
        let normalized = normalize_identifier(id_or_alias.to_owned());

        if self.entries.contains_key(&normalized) {
            return Ok(self
                .entries
                .get_key_value(&normalized)
                .map(|(key, _)| key.as_str())
                .expect("entry was checked immediately above"));
        }

        if let Some(canonical) = self.aliases.get(&normalized) {
            return Ok(canonical.as_str());
        }

        if self.aliases.contains_key(&normalized) {
            Err(RegistryError::AliasNotFound {
                alias: normalized,
            })
        } else {
            Err(RegistryError::BenchmarkNotFound {
                id: normalized,
            })
        }
    }

    /// Returns a descriptor by canonical ID or alias.
    pub fn get(&self, id_or_alias: &str) -> Result<&BenchmarkDescriptor, RegistryError> {
        let canonical = self.resolve(id_or_alias)?;

        self.entries
            .get(canonical)
            .map(|entry| entry.descriptor.as_ref())
            .ok_or_else(|| RegistryError::InvariantViolation {
                reason: format!(
                    "resolved benchmark `{canonical}` does not have a registry entry"
                ),
            })
    }

    /// Constructs a benchmark by canonical ID or alias.
    pub fn create(&self, id_or_alias: &str) -> Result<Box<dyn Benchmark>, RegistryError> {
        self.get(id_or_alias)?.create()
    }

    /// Returns true when a benchmark is registered.
    pub fn contains(&self, id_or_alias: &str) -> bool {
        self.resolve(id_or_alias).is_ok()
    }

    /// Returns all canonical benchmark IDs in deterministic order.
    pub fn ids(&self) -> Vec<&str> {
        self.entries.keys().map(String::as_str).collect()
    }

    /// Returns all descriptors in deterministic canonical-ID order.
    pub fn descriptors(&self) -> Vec<&BenchmarkDescriptor> {
        self.entries
            .values()
            .map(|entry| entry.descriptor.as_ref())
            .collect()
    }

    /// Returns all aliases and their canonical IDs in deterministic order.
    pub fn aliases(&self) -> Vec<(&str, &str)> {
        self.aliases
            .iter()
            .map(|(alias, canonical)| (alias.as_str(), canonical.as_str()))
            .collect()
    }

    /// Finds benchmarks matching a query.
    ///
    /// Results are returned in deterministic canonical-ID order.
    pub fn query(&self, query: &BenchmarkRegistryQuery) -> Vec<&BenchmarkDescriptor> {
        self.entries
            .values()
            .filter(|entry| query.matches(entry.descriptor.as_ref()))
            .map(|entry| entry.descriptor.as_ref())
            .collect()
    }

    /// Finds all benchmarks in a category.
    pub fn by_category(
        &self,
        category: BenchmarkCategory,
    ) -> Vec<&BenchmarkDescriptor> {
        self.query(&BenchmarkRegistryQuery::new().category(category))
    }

    /// Finds all benchmarks requiring a capability.
    pub fn requiring(
        &self,
        capability: BenchmarkCapability,
    ) -> Vec<&BenchmarkDescriptor> {
        self.query(
            &BenchmarkRegistryQuery::new().require_capability(capability),
        )
    }

    /// Finds all benchmarks supporting an execution target.
    pub fn supporting_target(
        &self,
        target: BenchmarkExecutionTarget,
    ) -> Vec<&BenchmarkDescriptor> {
        self.query(
            &BenchmarkRegistryQuery::new().execution_target(target),
        )
    }

    /// Validates every registry invariant.
    ///
    /// This should be called:
    ///
    /// - after building a built-in registry in debug/test builds;
    /// - after loading a persisted registry;
    /// - before exposing an externally constructed registry.
    pub fn validate(&self) -> Result<(), RegistryError> {
        if self.entries.len() > self.max_benchmarks {
            return Err(RegistryError::InvariantViolation {
                reason: format!(
                    "registry contains {} benchmarks but maximum is {}",
                    self.entries.len(),
                    self.max_benchmarks
                ),
            });
        }

        for (id, entry) in &self.entries {
            descriptor_validate(entry.descriptor.as_ref())?;

            if entry.descriptor.id() != id {
                return Err(RegistryError::InvariantViolation {
                    reason: format!(
                        "entry key `{id}` does not match descriptor ID `{}`",
                        entry.descriptor.id()
                    ),
                });
            }

            for alias in entry.descriptor.aliases() {
                match self.aliases.get(alias) {
                    Some(canonical) if canonical == id => {}
                    Some(canonical) => {
                        return Err(RegistryError::InvariantViolation {
                            reason: format!(
                                "alias `{alias}` points to `{canonical}` but belongs to `{id}`"
                            ),
                        });
                    }
                    None => {
                        return Err(RegistryError::InvariantViolation {
                            reason: format!(
                                "descriptor `{id}` declares alias `{alias}` but registry does not index it"
                            ),
                        });
                    }
                }
            }
        }

        for (alias, canonical) in &self.aliases {
            if alias == canonical {
                return Err(RegistryError::InvariantViolation {
                    reason: format!(
                        "alias `{alias}` resolves to itself as a canonical benchmark"
                    ),
                });
            }

            if !self.entries.contains_key(canonical) {
                return Err(RegistryError::InvariantViolation {
                    reason: format!(
                        "alias `{alias}` points to missing benchmark `{canonical}`"
                    ),
                });
            }

            if self.entries.contains_key(alias) {
                return Err(RegistryError::InvariantViolation {
                    reason: format!(
                        "alias `{alias}` conflicts with a canonical benchmark ID"
                    ),
                });
            }
        }

        Ok(())
    }

    /// Creates an immutable snapshot suitable for sharing.
    ///
    /// The returned `Arc` owns the complete registry. Callers can cheaply clone
    /// the `Arc` and safely share it among threads because registry descriptors
    /// contain immutable metadata and function pointers.
    #[must_use]
    pub fn into_shared(self) -> Arc<Self> {
        Arc::new(self)
    }
}

impl Clone for BenchmarkRegistry {
    fn clone(&self) -> Self {
        Self {
            entries: self.entries.clone(),
            aliases: self.aliases.clone(),
            max_benchmarks: self.max_benchmarks,
        }
    }
}

impl Default for BenchmarkRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Descriptor validation
// =============================================================================

fn descriptor_validate(descriptor: &BenchmarkDescriptor) -> Result<(), RegistryError> {
    validate_identifier(descriptor.id(), "benchmark ID")?;
    validate_metadata(descriptor.metadata())?;

    if descriptor.metadata().id.as_str() != descriptor.id() {
        return Err(RegistryError::MetadataIdMismatch {
            descriptor_id: descriptor.id().to_owned(),
            metadata_id: descriptor.metadata().id.as_str().to_owned(),
        });
    }

    if descriptor.aliases().len() > MAX_ALIASES_PER_BENCHMARK {
        return Err(RegistryError::TooManyAliases {
            benchmark_id: descriptor.id().to_owned(),
            count: descriptor.aliases().len(),
            maximum: MAX_ALIASES_PER_BENCHMARK,
        });
    }

    let mut aliases = BTreeSet::new();

    for alias in descriptor.aliases() {
        validate_identifier(alias, "benchmark alias")?;

        if alias == descriptor.id() {
            return Err(RegistryError::AliasConflictsWithBenchmark {
                alias: alias.clone(),
                benchmark_id: descriptor.id().to_owned(),
            });
        }

        if !aliases.insert(alias.clone()) {
            return Err(RegistryError::DuplicateAlias {
                alias: alias.clone(),
            });
        }
    }

    Ok(())
}

fn validate_metadata(metadata: &BenchmarkMetadata) -> Result<(), RegistryError> {
    if metadata.name.trim().is_empty() {
        return Err(RegistryError::InvalidMetadata {
            field: "name".to_owned(),
            reason: "benchmark name must not be empty".to_owned(),
        });
    }

    if metadata.description.trim().is_empty() {
        return Err(RegistryError::InvalidMetadata {
            field: "description".to_owned(),
            reason: "benchmark description must not be empty".to_owned(),
        });
    }

    if metadata.description.len() > MAX_DESCRIPTION_LENGTH {
        return Err(RegistryError::InvalidMetadata {
            field: "description".to_owned(),
            reason: format!(
                "description exceeds maximum length of {} bytes",
                MAX_DESCRIPTION_LENGTH
            ),
        });
    }

    if metadata.version.major == u16::MAX
        && metadata.version.minor == u16::MAX
        && metadata.version.patch == u16::MAX
    {
        return Err(RegistryError::InvalidMetadata {
            field: "version".to_owned(),
            reason: "reserved semantic-version value is not permitted".to_owned(),
        });
    }

    Ok(())
}

fn validate_factory_identity(
    registered_id: &str,
    benchmark: &dyn Benchmark,
) -> Result<(), RegistryError> {
    let metadata = benchmark.metadata();

    if metadata.id.as_str() != registered_id {
        return Err(RegistryError::FactoryContractViolation {
            benchmark_id: registered_id.to_owned(),
            reason: format!(
                "factory returned benchmark `{}`",
                metadata.id.as_str()
            ),
        });
    }

    Ok(())
}

// =============================================================================
// Identifier validation
// =============================================================================

/// Normalizes an externally supplied identifier.
///
/// Benchmark IDs are deliberately case-sensitive in their canonical form, but
/// surrounding whitespace is never meaningful. We therefore trim whitespace
/// while preserving the actual identifier bytes otherwise.
fn normalize_identifier(value: String) -> String {
    value.trim().to_owned()
}

/// Validates benchmark IDs and aliases.
///
/// Allowed grammar:
///
/// ```text
/// [a-zA-Z0-9][a-zA-Z0-9._:-]{0,127}
/// ```
///
/// Underscore is intentionally supported because Zamani's built-in benchmark
/// identifiers use names such as `quantum_volume`.
fn validate_identifier(value: &str, kind: &str) -> Result<(), RegistryError> {
    if value.is_empty() {
        return Err(if kind == "benchmark alias" {
            RegistryError::InvalidAlias {
                value: value.to_owned(),
                reason: "identifier must not be empty".to_owned(),
            }
        } else {
            RegistryError::InvalidBenchmarkId {
                value: value.to_owned(),
                reason: "identifier must not be empty".to_owned(),
            }
        });
    }

    if value.len() > MAX_IDENTIFIER_LENGTH {
        let reason = format!(
            "identifier exceeds maximum length of {} bytes",
            MAX_IDENTIFIER_LENGTH
        );

        return Err(if kind == "benchmark alias" {
            RegistryError::InvalidAlias {
                value: value.to_owned(),
                reason,
            }
        } else {
            RegistryError::InvalidBenchmarkId {
                value: value.to_owned(),
                reason,
            }
        });
    }

    let bytes = value.as_bytes();

    if !is_identifier_start(bytes[0]) {
        let reason =
            "first character must be ASCII alphanumeric".to_owned();

        return Err(if kind == "benchmark alias" {
            RegistryError::InvalidAlias {
                value: value.to_owned(),
                reason,
            }
        } else {
            RegistryError::InvalidBenchmarkId {
                value: value.to_owned(),
                reason,
            }
        });
    }

    for &byte in &bytes[1..] {
        if !is_identifier_continue(byte) {
            let reason = format!(
                "character `{}` is not permitted; allowed characters are \
                 ASCII letters, digits, `_`, `.`, `:`, and `-`",
                byte as char
            );

            return Err(if kind == "benchmark alias" {
                RegistryError::InvalidAlias {
                    value: value.to_owned(),
                    reason,
                }
            } else {
                RegistryError::InvalidBenchmarkId {
                    value: value.to_owned(),
                    reason,
                }
            });
        }
    }

    Ok(())
}

#[inline]
const fn is_identifier_start(byte: u8) -> bool {
    byte.is_ascii_alphanumeric()
}

#[inline]
const fn is_identifier_continue(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'.' | b':' | b'-')
}

// =============================================================================
// Convenience constructor
// =============================================================================

/// Creates a [`BenchmarkDescriptor`] using an existing [`BenchmarkId`].
///
/// This convenience function exists so built-in registration code does not
/// need to duplicate the ID-to-string conversion logic.
pub fn descriptor(
    id: BenchmarkId,
    version: BenchmarkVersion,
    category: BenchmarkCategory,
    name: impl Into<String>,
    description: impl Into<String>,
    factory: BenchmarkFactory,
) -> Result<BenchmarkDescriptor, RegistryError> {
    let metadata = BenchmarkMetadata::new(
        id.clone(),
        version,
        category,
        name,
        description,
    )
    .map_err(|error| RegistryError::InvalidMetadata {
        field: "metadata".to_owned(),
        reason: error.to_string(),
    })?;

    BenchmarkDescriptor::new(id.as_str(), metadata, factory)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Minimal test benchmark used only for registry tests.
    struct TestBenchmark {
        metadata: BenchmarkMetadata,
    }

    impl TestBenchmark {
        fn new() -> Self {
            let id = BenchmarkId::new("test_benchmark")
                .expect("test benchmark ID must be valid");

            let metadata = BenchmarkMetadata::new(
                id,
                BenchmarkVersion::new(1, 0, 0),
                BenchmarkCategory::Custom,
                "Test Benchmark",
                "Benchmark used by registry unit tests.",
            )
            .expect("test metadata must be valid");

            Self { metadata }
        }
    }

    impl Benchmark for TestBenchmark {
        fn metadata(&self) -> &BenchmarkMetadata {
            &self.metadata
        }

        // NOTE:
        // The remaining lifecycle methods are intentionally not implemented
        // here because this test benchmark is only useful when the current
        // Benchmark trait's default lifecycle behavior permits it.
        //
        // Concrete protocol implementations provide the complete lifecycle.
    }

    fn test_factory() -> Box<dyn Benchmark> {
        Box::new(TestBenchmark::new())
    }

    fn test_descriptor() -> BenchmarkDescriptor {
        let id = BenchmarkId::new("test_benchmark")
            .expect("test benchmark ID must be valid");

        let metadata = BenchmarkMetadata::new(
            id,
            BenchmarkVersion::new(1, 0, 0),
            BenchmarkCategory::Custom,
            "Test Benchmark",
            "Benchmark used by registry unit tests.",
        )
        .expect("test metadata must be valid");

        BenchmarkDescriptor::new(
            "test_benchmark",
            metadata,
            test_factory,
        )
        .expect("descriptor must be valid")
        .with_alias("test")
        .expect("alias must be valid")
        .with_capabilities([
            BenchmarkCapability::DeterministicWorkload,
            BenchmarkCapability::OfflineAnalysis,
        ])
        .with_execution_targets([
            BenchmarkExecutionTarget::CpuSimulator,
            BenchmarkExecutionTarget::External,
        ])
    }

    #[test]
    fn validates_identifiers() {
        assert!(validate_identifier("quantum_volume", "benchmark ID").is_ok());
        assert!(validate_identifier("rb.v1", "benchmark ID").is_ok());
        assert!(validate_identifier("cycle:benchmark", "benchmark ID").is_ok());
        assert!(validate_identifier("xeb-2", "benchmark ID").is_ok());

        assert!(validate_identifier("", "benchmark ID").is_err());
        assert!(validate_identifier("_invalid", "benchmark ID").is_err());
        assert!(validate_identifier("-invalid", "benchmark ID").is_err());
        assert!(validate_identifier("contains space", "benchmark ID").is_err());
    }

    #[test]
    fn registers_and_resolves_aliases() {
        let mut registry = BenchmarkRegistry::new();

        registry
            .register(test_descriptor())
            .expect("registration must succeed");

        assert_eq!(registry.len(), 1);
        assert_eq!(registry.alias_count(), 1);

        assert!(registry.contains("test_benchmark"));
        assert!(registry.contains("test"));

        assert_eq!(
            registry
                .resolve("test_benchmark")
                .expect("canonical ID must resolve"),
            "test_benchmark"
        );

        assert_eq!(
            registry
                .resolve("test")
                .expect("alias must resolve"),
            "test_benchmark"
        );
    }

    #[test]
    fn duplicate_ids_are_rejected() {
        let mut registry = BenchmarkRegistry::new();

        registry
            .register(test_descriptor())
            .expect("first registration must succeed");

        let duplicate = test_descriptor();

        let error = registry
            .register(duplicate)
            .expect_err("duplicate registration must fail");

        assert!(matches!(
            error,
            RegistryError::DuplicateBenchmarkId { .. }
        ));
    }

    #[test]
    fn alias_collision_is_rejected() {
        let mut registry = BenchmarkRegistry::new();

        registry
            .register(test_descriptor())
            .expect("first registration must succeed");

        let id = BenchmarkId::new("other_benchmark")
            .expect("ID must be valid");

        let metadata = BenchmarkMetadata::new(
            id,
            BenchmarkVersion::new(1, 0, 0),
            BenchmarkCategory::Custom,
            "Other Benchmark",
            "Second benchmark.",
        )
        .expect("metadata must be valid");

        let descriptor = BenchmarkDescriptor::new(
            "other_benchmark",
            metadata,
            test_factory,
        )
        .expect("descriptor must be valid")
        .with_alias("test")
        .expect("descriptor-local alias must be valid");

        let error = registry
            .register(descriptor)
            .expect_err("alias collision must fail");

        assert!(matches!(
            error,
            RegistryError::DuplicateAlias { .. }
        ));
    }

    #[test]
    fn query_is_deterministic() {
        let mut registry = BenchmarkRegistry::new();

        let first = test_descriptor();

        let second_id = BenchmarkId::new("another_benchmark")
            .expect("ID must be valid");

        let second_metadata = BenchmarkMetadata::new(
            second_id,
            BenchmarkVersion::new(1, 0, 0),
            BenchmarkCategory::Device,
            "Another Benchmark",
            "Another benchmark.",
        )
        .expect("metadata must be valid");

        let second = BenchmarkDescriptor::new(
            "another_benchmark",
            second_metadata,
            test_factory,
        )
        .expect("descriptor must be valid")
        .with_capability(BenchmarkCapability::Sampling);

        registry
            .register(second)
            .expect("second registration must succeed");

        registry
            .register(first)
            .expect("first registration must succeed");

        assert_eq!(
            registry.ids(),
            vec!["another_benchmark", "test_benchmark"]
        );

        let custom = registry.by_category(BenchmarkCategory::Custom);

        assert_eq!(custom.len(), 1);
        assert_eq!(custom[0].id(), "test_benchmark");
    }

    #[test]
    fn capability_query_works() {
        let mut registry = BenchmarkRegistry::new();

        registry
            .register(test_descriptor())
            .expect("registration must succeed");

        let matches = registry.requiring(
            BenchmarkCapability::DeterministicWorkload,
        );

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].id(), "test_benchmark");
    }

    #[test]
    fn target_query_works() {
        let mut registry = BenchmarkRegistry::new();

        registry
            .register(test_descriptor())
            .expect("registration must succeed");

        let matches = registry.supporting_target(
            BenchmarkExecutionTarget::CpuSimulator,
        );

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].id(), "test_benchmark");
    }

    #[test]
    fn registration_is_transactional() {
        let mut registry = BenchmarkRegistry::new();

        registry
            .register(test_descriptor())
            .expect("initial registration must succeed");

        let invalid = BenchmarkDescriptor::new(
            "test_benchmark",
            BenchmarkMetadata::new(
                BenchmarkId::new("test_benchmark")
                    .expect("ID must be valid"),
                BenchmarkVersion::new(1, 0, 0),
                BenchmarkCategory::Custom,
                "Duplicate",
                "Duplicate benchmark.",
            )
            .expect("metadata must be valid"),
            test_factory,
        )
        .expect("descriptor itself is valid");

        let error = registry
            .register_all([invalid])
            .expect_err("transaction should fail");

        assert!(matches!(
            error,
            RegistryError::DuplicateBenchmarkId { .. }
        ));

        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn registry_invariants_hold() {
        let mut registry = BenchmarkRegistry::new();

        registry
            .register(test_descriptor())
            .expect("registration must succeed");

        registry
            .validate()
            .expect("registry invariants must hold");
    }

    #[test]
    fn aliases_are_sorted() {
        let id = BenchmarkId::new("sorted_benchmark")
            .expect("ID must be valid");

        let metadata = BenchmarkMetadata::new(
            id,
            BenchmarkVersion::new(1, 0, 0),
            BenchmarkCategory::Custom,
            "Sorted Benchmark",
            "Benchmark with multiple aliases.",
        )
        .expect("metadata must be valid");

        let descriptor = BenchmarkDescriptor::new(
            "sorted_benchmark",
            metadata,
            test_factory,
        )
        .expect("descriptor must be valid")
        .with_aliases(["z_alias", "a_alias", "m_alias"])
        .expect("aliases must be valid");

        assert_eq!(
            descriptor.aliases(),
            &[
                "a_alias".to_owned(),
                "m_alias".to_owned(),
                "z_alias".to_owned()
            ]
        );
    }

    #[test]
    fn factory_is_lazy() {
        // Registration must not construct the benchmark. This is indirectly
        // guaranteed by the fact that a descriptor can be registered and
        // queried without calling create().
        let mut registry = BenchmarkRegistry::new();

        registry
            .register(test_descriptor())
            .expect("registration must succeed");

        assert_eq!(registry.len(), 1);
    }
}