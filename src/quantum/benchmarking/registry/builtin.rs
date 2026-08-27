//! Zamani Quantum Benchmarking — Built-in Benchmark Registry
//!
//! This module defines the authoritative built-in benchmark catalog for
//! Zamani's quantum benchmarking subsystem.
//!
//! # Architectural responsibility
//!
//! `builtin.rs` is responsible for:
//!
//! - defining stable built-in benchmark identities;
//! - defining built-in benchmark metadata;
//! - defining built-in aliases;
//! - defining built-in capability declarations;
//! - defining built-in execution-target declarations;
//! - constructing the built-in registry;
//! - validating the built-in registry;
//! - preventing accidental capability overclaiming;
//! - keeping concrete protocol dependencies in one integration boundary;
//! - providing deterministic registry construction;
//! - providing a single place where a concrete protocol becomes a registered
//!   `core::Benchmark` implementation.
//!
//! This module does NOT:
//!
//! - execute circuits;
//! - communicate with hardware;
//! - generate quantum circuits itself;
//! - implement statistical algorithms;
//! - implement Quantum IR;
//! - implement protocol mathematics;
//! - serialize benchmark results;
//! - own runtime/backend state;
//! - dynamically load plugins.
//!
//! Those responsibilities belong to their owning modules.
//!
//! # Critical production rule
//!
//! A source file existing under `protocols/`, `applications/`, or `qec/` is
//! NOT sufficient reason to register it as an executable `Benchmark`.
//!
//! A benchmark may only be registered here when an adapter/factory exists that
//! satisfies the authoritative `core::benchmark::Benchmark` contract.
//!
//! This prevents a dangerous class of bugs where the registry advertises a
//! benchmark as executable even though the protocol has not yet been connected
//! to the universal benchmark lifecycle.
//!
//! # Current repository integration
//!
//! The repository currently contains a substantial protocol implementation
//! hierarchy, including:
//!
//! - Quantum Volume;
//! - randomized benchmarking;
//! - interleaved RB;
//! - simultaneous/purity/leakage RB;
//! - cycle benchmarking;
//! - layer fidelity;
//! - RCS/XEB-related infrastructure;
//! - mirror circuits;
//! - SPAM;
//! - coherence;
//! - gate/process fidelity;
//! - crosstalk;
//! - drift;
//! - application benchmarks;
//! - QEC benchmarks.
//!
//! The registry must not assume that the existence of those files means that
//! each one already implements `core::benchmark::Benchmark`.
//!
//! The concrete protocol is integrated here only through a factory whose return
//! type is:
//!
//! ```text
//! Box<dyn Benchmark>
//! ```
//!
//! # Dependency direction
//!
//! ```text
//! protocols/* ───────────────┐
//! applications/* ────────────┤
//! qec/* ─────────────────────┤
//!                             ▼
//!                    registry/builtin.rs
//!                             │
//!                             ▼
//!                    registry/registry.rs
//!                             │
//!                             ▼
//!                       BenchmarkRegistry
//! ```
//!
//! Never introduce:
//!
//! ```text
//! registry -> protocol -> registry
//! ```
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
//! # Production properties
//!
//! Built-in registration is:
//!
//! - deterministic;
//! - bounded;
//! - explicit;
//! - statically linked;
//! - free of global mutable state;
//! - free of dynamic loading;
//! - free of network access;
//! - free of benchmark execution side effects;
//! - safe to construct repeatedly;
//! - safe to share after construction.
//!
//! # Integration contract
//!
//! This module depends on:
//!
//! - `registry::registry` for the generic registry;
//! - `core::benchmark` for the universal benchmark contract.
//!
//! Concrete protocol modules are intentionally imported only when their
//! implementations actually satisfy the universal benchmark contract.
//!
//! When a protocol is upgraded to implement `Benchmark`, its adapter belongs
//! here and nowhere else in the registry implementation.
//!
//! # Important distinction
//!
//! The built-in catalog contains stable definitions for the benchmark families
//! Zamani intends to support. The executable registry contains only benchmarks
//! whose `Benchmark` integration is complete.
//!
//! This distinction is intentional and production-critical.
//!
//! ```text
//! built-in catalog
//!       │
//!       ├── implemented adapter ──► executable registry
//!       │
//!       └── not yet integrated ──► catalog only
//! ```
//!
//! This prevents the language/runtime from discovering a benchmark and then
//! failing later because the registry falsely advertised an implementation.

use super::registry::{
    descriptor,
    BenchmarkCapability,
    BenchmarkDescriptor,
    BenchmarkExecutionTarget,
    BenchmarkFactory,
    BenchmarkRegistry,
    RegistryError,
    DEFAULT_MAX_BENCHMARKS,
};

use super::super::core::benchmark::{
    Benchmark,
    BenchmarkCategory,
    BenchmarkId,
    BenchmarkMetadata,
    BenchmarkVersion,
};

// =============================================================================
// Registry constants
// =============================================================================

/// Stable built-in catalog schema version.
///
/// This is independent from individual benchmark protocol versions.
pub const BUILTIN_CATALOG_VERSION: &str = "1.0.0";

/// Stable component identifier.
pub const BUILTIN_CATALOG_ID: &str =
    "zamani.quantum.benchmark.registry.builtin";

/// Maximum number of built-in benchmark descriptors.
///
/// This is intentionally lower than the generic registry's maximum so that a
/// malformed future edit cannot silently create an unexpectedly large built-in
/// registry.
pub const BUILTIN_MAX_BENCHMARKS: usize = 512;

// =============================================================================
// Stable benchmark identifiers
// =============================================================================
//
// These constants are deliberately centralized.
//
// Never use ad-hoc string literals throughout the language/runtime integration.
// The same identifiers eventually become:
//
// - registry IDs;
// - Zamani-language benchmark names;
// - report IDs;
// - baseline keys;
// - CI benchmark IDs;
// - documentation anchors;
// - reproducibility identifiers.
// =============================================================================

/// Quantum Volume.
pub const ID_QUANTUM_VOLUME: &str = "quantum_volume";

/// Standard randomized benchmarking.
pub const ID_RANDOMIZED_BENCHMARKING: &str = "randomized_benchmarking";

/// Interleaved randomized benchmarking.
pub const ID_INTERLEAVED_RB: &str = "interleaved_rb";

/// Simultaneous randomized benchmarking.
pub const ID_SIMULTANEOUS_RB: &str = "simultaneous_rb";

/// Purity randomized benchmarking.
pub const ID_PURITY_RB: &str = "purity_rb";

/// Leakage randomized benchmarking.
pub const ID_LEAKAGE_RB: &str = "leakage_rb";

/// Cycle benchmarking.
pub const ID_CYCLE_BENCHMARKING: &str = "cycle_benchmarking";

/// Layer fidelity.
pub const ID_LAYER_FIDELITY: &str = "layer_fidelity";

/// Cross-entropy benchmarking.
pub const ID_XEB: &str = "xeb";

/// Random circuit sampling.
pub const ID_RANDOM_CIRCUIT_SAMPLING: &str = "random_circuit_sampling";

/// Mirror circuits.
pub const ID_MIRROR: &str = "mirror";

/// SPAM characterization.
pub const ID_SPAM: &str = "spam";

/// Gate fidelity characterization.
pub const ID_GATE_FIDELITY: &str = "gate_fidelity";

/// Process fidelity characterization.
pub const ID_PROCESS_FIDELITY: &str = "process_fidelity";

/// Coherence characterization.
pub const ID_COHERENCE: &str = "coherence";

/// Crosstalk characterization.
pub const ID_CROSSTALK: &str = "crosstalk";

/// Drift characterization.
pub const ID_DRIFT: &str = "drift";

/// State/process tomography.
pub const ID_TOMOGRAPHY: &str = "tomography";

// -----------------------------------------------------------------------------
// Application benchmarks
// -----------------------------------------------------------------------------

pub const ID_DEUTSCH_JOZSA: &str = "deutsch_jozsa";
pub const ID_BERNSTEIN_VAZIRANI: &str = "bernstein_vazirani";
pub const ID_HIDDEN_SHIFT: &str = "hidden_shift";
pub const ID_QFT: &str = "qft";
pub const ID_GROVER: &str = "grover";
pub const ID_PHASE_ESTIMATION: &str = "phase_estimation";
pub const ID_AMPLITUDE_ESTIMATION: &str = "amplitude_estimation";
pub const ID_VQE: &str = "vqe";
pub const ID_QAOA: &str = "qaoa";
pub const ID_MAXCUT: &str = "maxcut";
pub const ID_HHL: &str = "hhl";
pub const ID_MONTE_CARLO: &str = "monte_carlo";
pub const ID_HAMILTONIAN_SIMULATION: &str = "hamiltonian_simulation";
pub const ID_SHOR: &str = "shor";
pub const ID_CUSTOM_APPLICATION: &str = "custom";

// -----------------------------------------------------------------------------
// QEC benchmarks
// -----------------------------------------------------------------------------

pub const ID_PHYSICAL_ERROR_RATE: &str = "physical_error_rate";
pub const ID_LOGICAL_ERROR_RATE: &str = "logical_error_rate";
pub const ID_QEC_THRESHOLD: &str = "qec_threshold";
pub const ID_DECODER: &str = "decoder";
pub const ID_SYNDROME: &str = "syndrome";
pub const ID_SURFACE_CODE: &str = "surface_code";
pub const ID_QEC_RESOURCE_OVERHEAD: &str = "qec_resource_overhead";

// =============================================================================
// Benchmark family
// =============================================================================

/// Stable built-in benchmark family.
///
/// This is a catalog-level classification and is intentionally independent of
/// `BenchmarkCategory`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BuiltinBenchmarkFamily {
    /// Device characterization.
    Device,

    /// Gate-model computational benchmark.
    Computation,

    /// System-level benchmark.
    System,

    /// Scaling/volumetric benchmark.
    Scaling,

    /// Application benchmark.
    Application,

    /// Hybrid quantum/classical application.
    Hybrid,

    /// Fault-tolerant/QEC benchmark.
    FaultTolerance,

    /// Analog benchmark.
    Analog,

    /// Annealing benchmark.
    Annealing,

    /// Sampling benchmark.
    Sampling,
}

impl BuiltinBenchmarkFamily {
    /// Returns a stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Device => "device",
            Self::Computation => "computation",
            Self::System => "system",
            Self::Scaling => "scaling",
            Self::Application => "application",
            Self::Hybrid => "hybrid",
            Self::FaultTolerance => "fault_tolerance",
            Self::Analog => "analog",
            Self::Annealing => "annealing",
            Self::Sampling => "sampling",
        }
    }
}

// =============================================================================
// Built-in catalog entry
// =============================================================================

/// Immutable catalog information for one built-in benchmark.
///
/// `BuiltinCatalogEntry` is deliberately separate from
/// `BenchmarkDescriptor`.
///
/// This permits Zamani to know which benchmark families exist without lying
/// about whether their universal `Benchmark` adapter has already been
/// integrated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuiltinCatalogEntry {
    /// Stable benchmark identifier.
    pub id: &'static str,

    /// Human-readable name.
    pub name: &'static str,

    /// Stable protocol version.
    pub version: BenchmarkVersion,

    /// High-level family.
    pub family: BuiltinBenchmarkFamily,

    /// Core registry category.
    pub category: BenchmarkCategory,

    /// Whether the universal `Benchmark` adapter is currently registered.
    pub executable: bool,
}

impl BuiltinCatalogEntry {
    /// Creates a catalog entry.
    pub const fn new(
        id: &'static str,
        name: &'static str,
        version: BenchmarkVersion,
        family: BuiltinBenchmarkFamily,
        category: BenchmarkCategory,
        executable: bool,
    ) -> Self {
        Self {
            id,
            name,
            version,
            family,
            category,
            executable,
        }
    }
}

// =============================================================================
// Built-in catalog
// =============================================================================

/// Complete built-in benchmark catalog.
///
/// This list is intentionally explicit instead of being generated through
/// macros or reflection. Explicit registration is easier to audit, deterministic
/// and compatible with Rust's static linking model.
pub const BUILTIN_CATALOG: &[BuiltinCatalogEntry] = &[
    // Device.
    BuiltinCatalogEntry::new(
        ID_RANDOMIZED_BENCHMARKING,
        "Randomized Benchmarking",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::Device,
        BenchmarkCategory::Device,
        false,
    ),
    BuiltinCatalogEntry::new(
        ID_INTERLEAVED_RB,
        "Interleaved Randomized Benchmarking",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::Device,
        BenchmarkCategory::Device,
        false,
    ),
    BuiltinCatalogEntry::new(
        ID_SIMULTANEOUS_RB,
        "Simultaneous Randomized Benchmarking",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::Device,
        BenchmarkCategory::Device,
        false,
    ),
    BuiltinCatalogEntry::new(
        ID_PURITY_RB,
        "Purity Randomized Benchmarking",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::Device,
        BenchmarkCategory::Device,
        false,
    ),
    BuiltinCatalogEntry::new(
        ID_LEAKAGE_RB,
        "Leakage Randomized Benchmarking",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::Device,
        BenchmarkCategory::Device,
        false,
    ),
    BuiltinCatalogEntry::new(
        ID_CYCLE_BENCHMARKING,
        "Cycle Benchmarking",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::Device,
        BenchmarkCategory::Device,
        false,
    ),
    BuiltinCatalogEntry::new(
        ID_LAYER_FIDELITY,
        "Layer Fidelity",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::Device,
        BenchmarkCategory::Device,
        false,
    ),
    BuiltinCatalogEntry::new(
        ID_SPAM,
        "SPAM Characterization",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::Device,
        BenchmarkCategory::Device,
        false,
    ),
    BuiltinCatalogEntry::new(
        ID_GATE_FIDELITY,
        "Gate Fidelity",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::Device,
        BenchmarkCategory::Device,
        false,
    ),
    BuiltinCatalogEntry::new(
        ID_PROCESS_FIDELITY,
        "Process Fidelity",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::Device,
        BenchmarkCategory::Device,
        false,
    ),
    BuiltinCatalogEntry::new(
        ID_COHERENCE,
        "Coherence Characterization",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::Device,
        BenchmarkCategory::Device,
        false,
    ),
    BuiltinCatalogEntry::new(
        ID_CROSSTALK,
        "Crosstalk Characterization",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::Device,
        BenchmarkCategory::Device,
        false,
    ),
    BuiltinCatalogEntry::new(
        ID_DRIFT,
        "Drift Characterization",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::Device,
        BenchmarkCategory::Device,
        false,
    ),
    BuiltinCatalogEntry::new(
        ID_TOMOGRAPHY,
        "Tomography",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::Device,
        BenchmarkCategory::Device,
        false,
    ),

    // Scaling / sampling.
    BuiltinCatalogEntry::new(
        ID_QUANTUM_VOLUME,
        "Quantum Volume",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::Scaling,
        BenchmarkCategory::Scaling,
        false,
    ),
    BuiltinCatalogEntry::new(
        ID_XEB,
        "Cross-Entropy Benchmarking",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::Scaling,
        BenchmarkCategory::Scaling,
        false,
    ),
    BuiltinCatalogEntry::new(
        ID_RANDOM_CIRCUIT_SAMPLING,
        "Random Circuit Sampling",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::Sampling,
        BenchmarkCategory::Sampling,
        false,
    ),
    BuiltinCatalogEntry::new(
        ID_MIRROR,
        "Mirror Circuit Benchmarking",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::Scaling,
        BenchmarkCategory::Scaling,
        false,
    ),

    // Applications.
    BuiltinCatalogEntry::new(
        ID_DEUTSCH_JOZSA,
        "Deutsch-Jozsa",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::Application,
        BenchmarkCategory::Application,
        false,
    ),
    BuiltinCatalogEntry::new(
        ID_BERNSTEIN_VAZIRANI,
        "Bernstein-Vazirani",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::Application,
        BenchmarkCategory::Application,
        false,
    ),
    BuiltinCatalogEntry::new(
        ID_HIDDEN_SHIFT,
        "Hidden Shift",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::Application,
        BenchmarkCategory::Application,
        false,
    ),
    BuiltinCatalogEntry::new(
        ID_QFT,
        "Quantum Fourier Transform",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::Application,
        BenchmarkCategory::Application,
        false,
    ),
    BuiltinCatalogEntry::new(
        ID_GROVER,
        "Grover Search",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::Application,
        BenchmarkCategory::Application,
        false,
    ),
    BuiltinCatalogEntry::new(
        ID_PHASE_ESTIMATION,
        "Quantum Phase Estimation",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::Application,
        BenchmarkCategory::Application,
        false,
    ),
    BuiltinCatalogEntry::new(
        ID_AMPLITUDE_ESTIMATION,
        "Quantum Amplitude Estimation",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::Application,
        BenchmarkCategory::Application,
        false,
    ),
    BuiltinCatalogEntry::new(
        ID_VQE,
        "Variational Quantum Eigensolver",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::Hybrid,
        BenchmarkCategory::Hybrid,
        false,
    ),
    BuiltinCatalogEntry::new(
        ID_QAOA,
        "Quantum Approximate Optimization Algorithm",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::Hybrid,
        BenchmarkCategory::Hybrid,
        false,
    ),
    BuiltinCatalogEntry::new(
        ID_MAXCUT,
        "MaxCut",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::Application,
        BenchmarkCategory::Application,
        false,
    ),
    BuiltinCatalogEntry::new(
        ID_HHL,
        "HHL",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::Application,
        BenchmarkCategory::Application,
        false,
    ),
    BuiltinCatalogEntry::new(
        ID_MONTE_CARLO,
        "Quantum Monte Carlo",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::Application,
        BenchmarkCategory::Application,
        false,
    ),
    BuiltinCatalogEntry::new(
        ID_HAMILTONIAN_SIMULATION,
        "Hamiltonian Simulation",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::Application,
        BenchmarkCategory::Application,
        false,
    ),
    BuiltinCatalogEntry::new(
        ID_SHOR,
        "Shor",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::Application,
        BenchmarkCategory::Application,
        false,
    ),
    BuiltinCatalogEntry::new(
        ID_CUSTOM_APPLICATION,
        "Custom Zamani Application Benchmark",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::Application,
        BenchmarkCategory::Custom,
        false,
    ),

    // QEC.
    BuiltinCatalogEntry::new(
        ID_PHYSICAL_ERROR_RATE,
        "Physical Error Rate",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::FaultTolerance,
        BenchmarkCategory::FaultTolerance,
        false,
    ),
    BuiltinCatalogEntry::new(
        ID_LOGICAL_ERROR_RATE,
        "Logical Error Rate",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::FaultTolerance,
        BenchmarkCategory::FaultTolerance,
        false,
    ),
    BuiltinCatalogEntry::new(
        ID_QEC_THRESHOLD,
        "QEC Threshold",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::FaultTolerance,
        BenchmarkCategory::FaultTolerance,
        false,
    ),
    BuiltinCatalogEntry::new(
        ID_DECODER,
        "Decoder Benchmark",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::FaultTolerance,
        BenchmarkCategory::FaultTolerance,
        false,
    ),
    BuiltinCatalogEntry::new(
        ID_SYNDROME,
        "Syndrome Extraction",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::FaultTolerance,
        BenchmarkCategory::FaultTolerance,
        false,
    ),
    BuiltinCatalogEntry::new(
        ID_SURFACE_CODE,
        "Surface Code",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::FaultTolerance,
        BenchmarkCategory::FaultTolerance,
        false,
    ),
    BuiltinCatalogEntry::new(
        ID_QEC_RESOURCE_OVERHEAD,
        "QEC Resource Overhead",
        BenchmarkVersion::new(1, 0, 0),
        BuiltinBenchmarkFamily::FaultTolerance,
        BenchmarkCategory::FaultTolerance,
        false,
    ),
];

// =============================================================================
// Catalog queries
// =============================================================================

/// Returns the complete immutable built-in catalog.
#[must_use]
pub const fn builtin_catalog() -> &'static [BuiltinCatalogEntry] {
    BUILTIN_CATALOG
}

/// Finds a built-in catalog entry by its canonical ID.
///
/// This function does not construct a benchmark.
#[must_use]
pub fn catalog_entry(id: &str) -> Option<&'static BuiltinCatalogEntry> {
    let mut index = 0usize;

    while index < BUILTIN_CATALOG.len() {
        let entry = &BUILTIN_CATALOG[index];

        if entry.id == id {
            return Some(entry);
        }

        index += 1;
    }

    None
}

/// Returns true when an identifier belongs to Zamani's built-in catalog.
#[must_use]
pub fn is_builtin(id: &str) -> bool {
    catalog_entry(id).is_some()
}

/// Returns true when a built-in benchmark has a universal executable adapter.
#[must_use]
pub fn is_builtin_executable(id: &str) -> bool {
    match catalog_entry(id) {
        Some(entry) => entry.executable,
        None => false,
    }
}

// =============================================================================
// Descriptor helpers
// =============================================================================

/// Constructs a benchmark descriptor using the canonical metadata contract.
///
/// This helper is the only place built-in registration code should need to
/// repeat the registry's metadata construction pattern.
fn make_descriptor(
    id: &'static str,
    version: BenchmarkVersion,
    category: BenchmarkCategory,
    name: &'static str,
    description: &'static str,
    factory: BenchmarkFactory,
    aliases: &'static [&'static str],
    capabilities: &'static [BenchmarkCapability],
    targets: &'static [BenchmarkExecutionTarget],
) -> Result<BenchmarkDescriptor, RegistryError> {
    let benchmark_id = BenchmarkId::new(id).map_err(|error| {
        RegistryError::InvalidBenchmarkId {
            value: id.to_owned(),
            reason: error.to_string(),
        }
    })?;

    let metadata = BenchmarkMetadata::new(
        benchmark_id.clone(),
        version,
        category,
        name,
        description,
    )
    .map_err(|error| RegistryError::InvalidMetadata {
        field: "metadata".to_owned(),
        reason: error.to_string(),
    })?;

    let mut descriptor = descriptor(
        benchmark_id,
        version,
        category,
        name,
        description,
        factory,
    )?;

    for alias in aliases {
        descriptor = descriptor.with_alias(*alias)?;
    }

    descriptor = descriptor.with_capabilities(capabilities.iter().copied());
    descriptor = descriptor.with_execution_targets(targets.iter().copied());

    // Keep the metadata construction above explicit. It serves as an
    // integration-time invariant check and prevents accidental divergence
    // between this helper and the core metadata contract.
    debug_assert_eq!(descriptor.metadata(), &metadata);

    Ok(descriptor)
}

// =============================================================================
// Universal adapter boundary
// =============================================================================

/// An explicit built-in adapter placeholder.
///
/// This type exists solely to make the integration boundary explicit.
///
/// It is intentionally NOT registered as an executable benchmark.
///
/// A concrete protocol must implement the universal `Benchmark` trait before
/// it can be passed to `register_executable_benchmark`.
///
/// This prevents `builtin.rs` from creating fake executable implementations.
#[derive(Debug)]
struct NotIntegratedBenchmark {
    metadata: BenchmarkMetadata,
}

impl NotIntegratedBenchmark {
    fn new(
        id: &'static str,
        name: &'static str,
        version: BenchmarkVersion,
        category: BenchmarkCategory,
    ) -> Self {
        let benchmark_id =
            BenchmarkId::new(id).expect("built-in benchmark IDs are compile-time constants");

        let metadata = BenchmarkMetadata::new(
            benchmark_id,
            version,
            category,
            name,
            "Built-in benchmark catalog entry without a universal Benchmark adapter.",
        )
        .expect("built-in benchmark metadata must be valid");

        Self { metadata }
    }
}

impl Benchmark for NotIntegratedBenchmark {
    fn metadata(&self) -> &BenchmarkMetadata {
        &self.metadata
    }

    fn validate(
        &self,
        _config: &super::super::core::config::BenchmarkConfig,
    ) -> Result<(), super::super::core::errors::BenchmarkError> {
        Err(
            super::super::core::errors::BenchmarkError::UnsupportedOperation {
                operation: "benchmark execution".to_owned(),
                benchmark: self.metadata.id.as_str().to_owned(),
            },
        )
    }

    fn generate(
        &self,
        _config: &super::super::core::config::BenchmarkConfig,
    ) -> Result<
        super::super::core::experiment::BenchmarkExperiment,
        super::super::core::errors::BenchmarkError,
    > {
        Err(
            super::super::core::errors::BenchmarkError::UnsupportedOperation {
                operation: "benchmark experiment generation".to_owned(),
                benchmark: self.metadata.id.as_str().to_owned(),
            },
        )
    }

    fn analyze(
        &self,
        _config: &super::super::core::config::BenchmarkConfig,
        _experiment: &super::super::core::experiment::BenchmarkExperiment,
        _observations: &super::super::core::observation::BenchmarkObservationSet,
    ) -> Result<
        super::super::core::result::BenchmarkResult,
        super::super::core::errors::BenchmarkError,
    > {
        Err(
            super::super::core::errors::BenchmarkError::UnsupportedOperation {
                operation: "benchmark analysis".to_owned(),
                benchmark: self.metadata.id.as_str().to_owned(),
            },
        )
    }
}

/// Factory for the explicit non-integrated adapter.
///
/// This is intentionally private and is not inserted into the executable
/// registry.
fn not_integrated_factory() -> Box<dyn Benchmark> {
    Box::new(NotIntegratedBenchmark::new(
        "builtin.not_integrated",
        "Non-integrated benchmark",
        BenchmarkVersion::new(1, 0, 0),
        BenchmarkCategory::Custom,
    ))
}

// =============================================================================
// Executable registration API
// =============================================================================

/// Registers one fully integrated benchmark implementation.
///
/// This function is the permanent integration point for concrete protocol
/// adapters.
///
/// The caller is responsible for supplying metadata/capabilities that exactly
/// match the concrete implementation.
///
/// The factory is never executed during registration.
pub fn register_executable_benchmark(
    registry: &mut BenchmarkRegistry,
    id: &'static str,
    version: BenchmarkVersion,
    category: BenchmarkCategory,
    name: &'static str,
    description: &'static str,
    factory: BenchmarkFactory,
    aliases: &'static [&'static str],
    capabilities: &'static [BenchmarkCapability],
    targets: &'static [BenchmarkExecutionTarget],
) -> Result<(), RegistryError> {
    if !is_builtin(id) {
        return Err(RegistryError::InvariantViolation {
            reason: format!(
                "attempted to register non-built-in benchmark `{id}`"
            ),
        });
    }

    let catalog = catalog_entry(id).ok_or_else(|| {
        RegistryError::InvariantViolation {
            reason: format!(
                "built-in catalog entry `{id}` disappeared during registration"
            ),
        }
    })?;

    if catalog.version != version {
        return Err(RegistryError::InvariantViolation {
            reason: format!(
                "benchmark `{id}` registration version {version} does not match \
                 catalog version {}",
                catalog.version
            ),
        });
    }

    if catalog.category != category {
        return Err(RegistryError::InvariantViolation {
            reason: format!(
                "benchmark `{id}` registration category `{category}` does not \
                 match catalog category `{}`",
                catalog.category
            ),
        });
    }

    let descriptor = make_descriptor(
        id,
        version,
        category,
        name,
        description,
        factory,
        aliases,
        capabilities,
        targets,
    )?;

    registry.register(descriptor)
}

// =============================================================================
// Built-in registry construction
// =============================================================================

/// Constructs the authoritative executable built-in registry.
///
/// IMPORTANT:
///
/// The current repository contains protocol implementations, but they are not
/// all yet connected to `core::benchmark::Benchmark`. Therefore this function
/// deliberately registers only adapters explicitly supplied by the integration
/// layer.
///
/// At the current repository state, that means the function returns an empty
/// executable registry rather than falsely advertising incomplete protocols as
/// runnable.
///
/// Once a protocol receives its permanent `Benchmark` adapter, its registration
/// is added here exactly once.
///
/// This design means no later protocol implementation can silently alter the
/// semantics of an already-completed registry file.
pub fn build_builtin_registry() -> Result<BenchmarkRegistry, RegistryError> {
    let mut registry = BenchmarkRegistry::with_capacity(
        DEFAULT_MAX_BENCHMARKS.min(BUILTIN_MAX_BENCHMARKS),
    );

    register_builtin_adapters(&mut registry)?;

    registry.validate()?;

    Ok(registry)
}

/// Registers all currently approved executable built-in adapters.
///
/// This function is intentionally separate from `build_builtin_registry` so
/// that integration work can add adapters without changing registry ownership
/// or validation semantics.
///
/// The current repository state does not yet contain universal `Benchmark`
/// implementations for the concrete protocol types. Consequently there are
/// no executable registrations here yet.
///
/// This is deliberate: registering an unsupported protocol would be a
/// production correctness bug.
fn register_builtin_adapters(
    _registry: &mut BenchmarkRegistry,
) -> Result<(), RegistryError> {
    Ok(())
}

// =============================================================================
// Catalog validation
// =============================================================================

/// Validates the static built-in catalog.
///
/// This checks structural invariants that are independent from concrete
/// protocol implementations.
pub fn validate_builtin_catalog() -> Result<(), RegistryError> {
    if BUILTIN_CATALOG.is_empty() {
        return Err(RegistryError::InvariantViolation {
            reason: "built-in benchmark catalog must not be empty".to_owned(),
        });
    }

    let mut ids = std::collections::BTreeSet::new();

    for entry in BUILTIN_CATALOG {
        if entry.id.trim().is_empty() {
            return Err(RegistryError::InvariantViolation {
                reason: "built-in benchmark ID must not be empty".to_owned(),
            });
        }

        if !ids.insert(entry.id) {
            return Err(RegistryError::InvariantViolation {
                reason: format!(
                    "duplicate built-in benchmark ID `{}`",
                    entry.id
                ),
            });
        }

        if entry.name.trim().is_empty() {
            return Err(RegistryError::InvariantViolation {
                reason: format!(
                    "built-in benchmark `{}` has an empty name",
                    entry.id
                ),
            });
        }

        if entry.version.major == u16::MAX
            && entry.version.minor == u16::MAX
            && entry.version.patch == u16::MAX
        {
            return Err(RegistryError::InvariantViolation {
                reason: format!(
                    "built-in benchmark `{}` uses reserved version",
                    entry.id
                ),
            });
        }
    }

    Ok(())
}

// =============================================================================
// Registry fingerprint
// =============================================================================

/// Computes a deterministic, dependency-free fingerprint of the built-in
/// catalog.
///
/// This is intentionally not a cryptographic hash.
///
/// It is a stable lightweight change detector suitable for:
///
/// - tests;
/// - diagnostics;
/// - benchmark manifests;
/// - registry compatibility checks.
///
/// Cryptographic provenance hashing belongs in the provenance subsystem.
#[must_use]
pub fn builtin_catalog_fingerprint() -> u64 {
    let mut hash = FNV_OFFSET_BASIS;

    for entry in BUILTIN_CATALOG {
        hash = fnv1a_update(hash, entry.id.as_bytes());
        hash = fnv1a_update(hash, &[0]);
        hash = fnv1a_update(hash, entry.name.as_bytes());
        hash = fnv1a_update(hash, &[0]);

        hash = fnv1a_update(hash, &entry.version.major.to_le_bytes());
        hash = fnv1a_update(hash, &entry.version.minor.to_le_bytes());
        hash = fnv1a_update(hash, &entry.version.patch.to_le_bytes());

        hash = fnv1a_update(hash, &[0]);

        hash = fnv1a_update(hash, entry.family.as_str().as_bytes());
        hash = fnv1a_update(hash, &[0]);
        hash = fnv1a_update(hash, entry.category.as_str().as_bytes());
        hash = fnv1a_update(hash, &[0]);
        hash = fnv1a_update(hash, &[entry.executable as u8]);
    }

    hash
}

const FNV_OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

#[inline]
fn fnv1a_update(mut hash: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }

    hash
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_catalog_is_not_empty() {
        validate_builtin_catalog()
            .expect("built-in catalog must satisfy structural invariants");

        assert!(!builtin_catalog().is_empty());
    }

    #[test]
    fn builtin_ids_are_unique() {
        let mut ids = std::collections::BTreeSet::new();

        for entry in builtin_catalog() {
            assert!(
                ids.insert(entry.id),
                "duplicate built-in benchmark ID: {}",
                entry.id
            );
        }
    }

    #[test]
    fn quantum_volume_is_in_catalog() {
        let entry = catalog_entry(ID_QUANTUM_VOLUME)
            .expect("Quantum Volume must be present in catalog");

        assert_eq!(entry.id, ID_QUANTUM_VOLUME);
        assert_eq!(entry.family, BuiltinBenchmarkFamily::Scaling);
        assert_eq!(entry.category, BenchmarkCategory::Scaling);

        // The current repository protocol has not yet been connected to the
        // universal Benchmark trait, so this must remain false until the
        // adapter is actually implemented.
        assert!(!entry.executable);
    }

    #[test]
    fn planned_protocols_are_cataloged_without_false_execution_claims() {
        let planned = [
            ID_RANDOMIZED_BENCHMARKING,
            ID_INTERLEAVED_RB,
            ID_SIMULTANEOUS_RB,
            ID_PURITY_RB,
            ID_LEAKAGE_RB,
            ID_CYCLE_BENCHMARKING,
            ID_LAYER_FIDELITY,
            ID_XEB,
            ID_RANDOM_CIRCUIT_SAMPLING,
            ID_MIRROR,
            ID_SPAM,
            ID_GATE_FIDELITY,
            ID_PROCESS_FIDELITY,
            ID_COHERENCE,
            ID_CROSSTALK,
            ID_DRIFT,
            ID_TOMOGRAPHY,
        ];

        for id in planned {
            let entry =
                catalog_entry(id).expect("planned benchmark must be cataloged");

            assert!(
                !entry.executable,
                "benchmark `{id}` must not claim executable integration \
                 before a core::Benchmark adapter exists"
            );
        }
    }

    #[test]
    fn application_benchmarks_are_cataloged() {
        let ids = [
            ID_DEUTSCH_JOZSA,
            ID_BERNSTEIN_VAZIRANI,
            ID_HIDDEN_SHIFT,
            ID_QFT,
            ID_GROVER,
            ID_PHASE_ESTIMATION,
            ID_AMPLITUDE_ESTIMATION,
            ID_VQE,
            ID_QAOA,
            ID_MAXCUT,
            ID_HHL,
            ID_MONTE_CARLO,
            ID_HAMILTONIAN_SIMULATION,
            ID_SHOR,
            ID_CUSTOM_APPLICATION,
        ];

        for id in ids {
            let entry =
                catalog_entry(id).expect("application benchmark must be cataloged");

            assert!(
                matches!(
                    entry.category,
                    BenchmarkCategory::Application
                        | BenchmarkCategory::Hybrid
                        | BenchmarkCategory::Custom
                ),
                "unexpected category for `{id}`"
            );
        }
    }

    #[test]
    fn qec_benchmarks_are_cataloged() {
        let ids = [
            ID_PHYSICAL_ERROR_RATE,
            ID_LOGICAL_ERROR_RATE,
            ID_QEC_THRESHOLD,
            ID_DECODER,
            ID_SYNDROME,
            ID_SURFACE_CODE,
            ID_QEC_RESOURCE_OVERHEAD,
        ];

        for id in ids {
            let entry =
                catalog_entry(id).expect("QEC benchmark must be cataloged");

            assert_eq!(
                entry.category,
                BenchmarkCategory::FaultTolerance
            );
            assert_eq!(
                entry.family,
                BuiltinBenchmarkFamily::FaultTolerance
            );
            assert!(!entry.executable);
        }
    }

    #[test]
    fn fingerprint_is_deterministic() {
        let first = builtin_catalog_fingerprint();
        let second = builtin_catalog_fingerprint();

        assert_eq!(first, second);
    }

    #[test]
    fn registry_is_empty_until_real_adapters_are_registered() {
        let registry =
            build_builtin_registry().expect("built-in registry must build");

        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
        assert_eq!(registry.alias_count(), 0);
        assert_eq!(
            registry.max_benchmarks(),
            DEFAULT_MAX_BENCHMARKS.min(BUILTIN_MAX_BENCHMARKS)
        );
    }

    #[test]
    fn non_builtin_registration_is_rejected() {
        let mut registry = BenchmarkRegistry::new();

        let result = register_executable_benchmark(
            &mut registry,
            "not_a_builtin",
            BenchmarkVersion::new(1, 0, 0),
            BenchmarkCategory::Custom,
            "Not Builtin",
            "Invalid test registration.",
            not_integrated_factory,
            &[],
            &[],
            &[],
        );

        assert!(matches!(
            result,
            Err(RegistryError::InvariantViolation { .. })
        ));
    }

    #[test]
    fn catalog_contains_no_duplicate_identifiers() {
        validate_builtin_catalog()
            .expect("catalog validation must succeed");

        let mut ids = std::collections::BTreeSet::new();

        for entry in BUILTIN_CATALOG {
            assert!(ids.insert(entry.id));
        }
    }

    #[test]
    fn every_catalog_identifier_is_valid_registry_identifier() {
        for entry in BUILTIN_CATALOG {
            let id = BenchmarkId::new(entry.id)
                .expect("every built-in ID must satisfy BenchmarkId rules");

            assert_eq!(id.as_str(), entry.id);
        }
    }

    #[test]
    fn catalog_version_is_stable() {
        assert_eq!(BUILTIN_CATALOG_VERSION, "1.0.0");
    }

    #[test]
    fn catalog_id_is_stable() {
        assert_eq!(
            BUILTIN_CATALOG_ID,
            "zamani.quantum.benchmark.registry.builtin"
        );
    }
}