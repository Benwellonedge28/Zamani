//! Zamani Quantum Memory — Diagnostics and Observability
//!
//! Production-grade, provider-neutral diagnostics for
//! `crate::quantum::memory`.
//!
//! # Responsibility
//!
//! This module owns:
//!
//! - point-in-time memory diagnostics;
//! - allocation/resource accounting snapshots;
//! - current and peak memory usage;
//! - allocation/release counters;
//! - memory usage grouped by storage location;
//! - memory usage grouped by allocation class;
//! - quantum-state representation accounting;
//! - provider/backend-neutral resource accounting;
//! - migration/copy/synchronization counters;
//! - cache statistics;
//! - memory pressure classification;
//! - diagnostic health/invariant reporting;
//! - bounded diagnostic labels;
//! - thread-safe diagnostics collection;
//! - deterministic snapshot generation;
//! - machine-readable diagnostic structures;
//! - integration points for allocator, state, GPU, distributed,
//!   migration, compaction, cache, synchronization and telemetry modules.
//!
//! # Architectural boundary
//!
//! `diagnostics.rs` is an observability layer.
//!
//! It does NOT own:
//!
//! - allocation;
//! - deallocation;
//! - memory limits;
//! - memory budgets;
//! - quantum-state mathematics;
//! - state-vector storage;
//! - density-matrix storage;
//! - stabilizer/tableau storage;
//! - tensor-network algorithms;
//! - GPU kernels;
//! - CUDA;
//! - HIP;
//! - Metal;
//! - Vulkan;
//! - SYCL;
//! - MPI;
//! - UCX;
//! - RDMA;
//! - QPU APIs;
//! - provider authentication;
//! - routing;
//! - scheduling;
//! - benchmarking protocols;
//! - compiler semantics;
//! - quantum IR.
//!
//! Those responsibilities remain in their respective subsystems.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::memory
//!                              │
//!          ┌───────────────────┼───────────────────┐
//!          │                   │                   │
//!          ▼                   ▼                   ▼
//!      allocator             state          synchronization
//!          │                   │                   │
//!          └───────────────────┼───────────────────┘
//!                              ▼
//!                         diagnostics
//!                              │
//!          ┌───────────────────┼───────────────────┐
//!          ▼                   ▼                   ▼
//!       telemetry         benchmarking          runtime
//! ```
//!
//! Diagnostics is therefore a consumer of subsystem events/statistics.
//!
//! It must not make allocation or execution decisions.
//!
//! # Provider/QPU neutrality
//!
//! This module deliberately does not enumerate quantum vendors or hardware
//! technologies.
//!
//! A provider is represented by an opaque, bounded textual identifier.
//!
//! Therefore the same diagnostics API can account for:
//!
//! - CPU RAM;
//! - NUMA memory;
//! - pinned host memory;
//! - NVIDIA GPU memory;
//! - AMD GPU memory;
//! - Apple accelerator memory;
//! - Vulkan/SYCL/other accelerator memory;
//! - unified memory;
//! - distributed memory;
//! - remote simulator memory;
//! - backend-native memory;
//! - superconducting QPU execution resources;
//! - trapped-ion QPU resources;
//! - neutral-atom resources;
//! - photonic resources;
//! - spin/semiconductor resources;
//! - annealing hardware;
//! - future quantum hardware;
//! - future accelerator technologies.
//!
//! Diagnostics does not assume that a physical QPU exposes a state vector or
//! any other internal quantum-memory representation. QPU-side diagnostics may
//! therefore record opaque provider/resource metrics without pretending that
//! physical-device state is locally inspectable.
//!
//! # Security boundary
//!
//! Diagnostics must never be used to store:
//!
//! - credentials;
//! - API keys;
//! - access tokens;
//! - passwords;
//! - private keys;
//! - authorization headers;
//! - raw device pointers;
//! - raw memory addresses;
//! - QPU session secrets;
//! - secret circuit/program contents;
//! - measurement data unless the caller explicitly intends it.
//!
//! Provider labels, representation labels, operation labels and resource labels
//! are bounded before storage.
//!
//! This module does not attempt to infer whether an arbitrary label is secret;
//! callers must supply non-secret diagnostic labels.
//!
//! # No unsafe
//!
//! This module contains no unsafe Rust.
//!
//! ```text
//! #![deny(unsafe_code)]
//! #![deny(unsafe_op_in_unsafe_fn)]
//! ```
//!
//! # Concurrency
//!
//! `MemoryDiagnostics` is thread-safe and cloneable.
//!
//! Clones refer to the same diagnostics collector.
//!
//! This permits independent subsystems to record events without requiring
//! global mutable state.
//!
//! No global diagnostics singleton is created here.
//!
//! The runtime/application decides which collector instance is shared.
//!
//! # Determinism
//!
//! Diagnostic counters are deterministic for a deterministic sequence of
//! recorded events.
//!
//! Snapshot maps are emitted through `BTreeMap`, providing deterministic key
//! ordering.
//!
//! Timing is represented using monotonic elapsed nanoseconds from the
//! collector's creation. Wall-clock time is intentionally not required.
//!
//! # Rust compatibility
//!
//! This module targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features.
//!
//! # Integration contract
//!
//! This module intentionally depends only on foundational memory types:
//!
//! ```text
//! memory::types
//!     ├── ByteCount
//!     └── QubitCount
//! ```
//!
//! It does not import allocator.rs, state.rs, GPU modules, distributed.rs,
//! migration.rs, telemetry.rs or benchmarking modules.
//!
//! This avoids dependency cycles and means this file can be completed before
//! those modules are finalized.
//!
//! Later modules integrate by recording events through this API:
//!
//! ```text
//! allocator.rs
//!     ├── record_allocation()
//!     └── record_release()
//!
//! pool.rs
//!     ├── record_pool_acquire()
//!     ├── record_pool_release()
//!     └── record_pool_miss()
//!
//! state.rs / state_vector.rs / density_matrix.rs / stabilizer.rs
//!     └── record_state_memory()
//!
//! gpu.rs
//!     └── record_provider_memory()
//!
//! distributed.rs
//!     └── record_provider_memory()
//!
//! migration.rs
//!     └── record_migration()
//!
//! compaction.rs
//!     └── record_compaction()
//!
//! synchronization.rs
//!     └── record_synchronization()
//!
//! cache.rs
//!     └── record_cache_hit()/record_cache_miss()
//!
//! telemetry.rs
//!     └── consume MemoryDiagnosticsSnapshot
//!
//! benchmarking
//!     └── consume MemoryDiagnosticsSnapshot
//! ```
//!
//! The collector never requires those modules to be modified merely because
//! another provider or state representation is added.
//!
//! # Important semantic rule
//!
//! Diagnostics are observational.
//!
//! Recording a diagnostic event must never mutate quantum state, memory
//! ownership, allocation policy, layout, or execution semantics.
//!
//! Diagnostics failures must therefore not be allowed to corrupt a quantum
//! computation.
//!
//! This implementation uses bounded, infallible counter updates wherever
//! possible. Counter overflow saturates rather than panicking.
//!
//! # Schema stability
//!
//! `MEMORY_DIAGNOSTICS_SCHEMA_ID` and
//! `MEMORY_DIAGNOSTICS_SCHEMA_VERSION` identify the machine-readable
//! diagnostics contract.
//!
//! Consumers should use explicit fields and schema versions rather than
//! parsing `Debug` output.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::fmt;
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc, Mutex,
};
use std::time::Instant;

use super::types::{ByteCount, QubitCount};

// =============================================================================
// Schema
// =============================================================================

/// Stable identifier for the diagnostics schema.
pub const MEMORY_DIAGNOSTICS_SCHEMA_ID: &str =
    "zamani.quantum.memory.diagnostics";

/// Semantic version of the diagnostics schema.
pub const MEMORY_DIAGNOSTICS_SCHEMA_VERSION: u16 = 1;

/// Maximum number of distinct dynamically labelled entries retained by one
/// diagnostics dimension.
///
/// This prevents a malicious or accidental stream of unique provider/resource
/// labels from becoming an unbounded memory leak.
pub const DEFAULT_MAX_DIMENSION_ENTRIES: usize = 1_024;

/// Maximum diagnostic-label length in Unicode scalar values.
pub const MAX_LABEL_LENGTH: usize = 256;

/// Maximum number of state-representation entries retained.
pub const MAX_REPRESENTATION_ENTRIES: usize = 256;

/// Maximum number of provider entries retained.
pub const MAX_PROVIDER_ENTRIES: usize = 256;

/// Maximum number of storage-location entries retained.
pub const MAX_LOCATION_ENTRIES: usize = 64;

/// Maximum number of allocation-class entries retained.
pub const MAX_ALLOCATION_CLASS_ENTRIES: usize = 64;

// =============================================================================
// Stable category names
// =============================================================================

/// Provider-neutral storage category.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DiagnosticStorageKind {
    /// Ordinary host memory.
    Host,

    /// Pinned host memory.
    PinnedHost,

    /// Accelerator/device memory.
    Device,

    /// Unified/shared memory.
    Unified,

    /// Distributed memory.
    Distributed,

    /// Backend/provider-native memory.
    BackendNative,

    /// Unknown/future storage class.
    Other,
}

impl DiagnosticStorageKind {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Host => "host",
            Self::PinnedHost => "pinned_host",
            Self::Device => "device",
            Self::Unified => "unified",
            Self::Distributed => "distributed",
            Self::BackendNative => "backend_native",
            Self::Other => "other",
        }
    }
}

impl fmt::Display for DiagnosticStorageKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Semantic allocation category.
///
/// These values intentionally mirror the allocator's conceptual allocation
/// classes without importing allocator.rs. This avoids a dependency cycle
/// while preserving a stable diagnostics vocabulary.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DiagnosticAllocationClass {
    /// Short-lived scratch memory.
    Temporary,

    /// Long-lived application memory.
    Persistent,

    /// Quantum-state storage.
    State,

    /// Snapshot/checkpoint storage.
    Checkpoint,

    /// Metadata and diagnostics memory.
    Metadata,

    /// Unknown/future allocation class.
    Other,
}

impl DiagnosticAllocationClass {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Temporary => "temporary",
            Self::Persistent => "persistent",
            Self::State => "state",
            Self::Checkpoint => "checkpoint",
            Self::Metadata => "metadata",
            Self::Other => "other",
        }
    }
}

impl fmt::Display for DiagnosticAllocationClass {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// State representation category.
///
/// This is intentionally an extensible semantic label rather than a Rust
/// enum owned by state.rs. A future representation therefore does not require
/// this diagnostics file to be reopened.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct StateRepresentationLabel(String);

impl StateRepresentationLabel {
    /// Creates a bounded state-representation label.
    pub fn new(value: impl AsRef<str>) -> Self {
        Self(sanitize_label(value.as_ref()))
    }

    /// Returns the machine-readable label.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for StateRepresentationLabel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Provider identifier.
///
/// This is deliberately opaque. It can identify any accelerator, QPU,
/// simulator, distributed transport or backend without requiring diagnostics
/// to know that provider's API.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ProviderLabel(String);

impl ProviderLabel {
    /// Creates a bounded provider label.
    pub fn new(value: impl AsRef<str>) -> Self {
        Self(sanitize_label(value.as_ref()))
    }

    /// Returns the provider label.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProviderLabel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Counter
// =============================================================================

/// Saturating thread-safe unsigned counter.
///
/// Saturation is intentional: diagnostics must never panic because an
/// observability counter reached its representational maximum.
#[derive(Debug)]
struct Counter(AtomicU64);

impl Counter {
    const fn new() -> Self {
        Self(AtomicU64::new(0))
    }

    fn increment(&self) {
        self.add(1);
    }

    fn add(&self, amount: u64) {
        let mut current = self.0.load(Ordering::Relaxed);

        loop {
            let next = current.saturating_add(amount);

            match self.0.compare_exchange_weak(
                current,
                next,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => return,
                Err(observed) => current = observed,
            }
        }
    }

    fn load(&self) -> u64 {
        self.0.load(Ordering::Relaxed)
    }
}

// =============================================================================
// Atomic byte accounting
// =============================================================================

/// Thread-safe saturating byte counter.
#[derive(Debug)]
struct ByteCounter(AtomicU64);

impl ByteCounter {
    const fn new() -> Self {
        Self(AtomicU64::new(0))
    }

    fn add(&self, amount: u64) {
        let mut current = self.0.load(Ordering::Relaxed);

        loop {
            let next = current.saturating_add(amount);

            match self.0.compare_exchange_weak(
                current,
                next,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => return,
                Err(observed) => current = observed,
            }
        }
    }

    fn subtract_saturating(&self, amount: u64) {
        let mut current = self.0.load(Ordering::Relaxed);

        loop {
            let next = current.saturating_sub(amount);

            match self.0.compare_exchange_weak(
                current,
                next,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => return,
                Err(observed) => current = observed,
            }
        }
    }

    fn load(&self) -> u64 {
        self.0.load(Ordering::Relaxed)
    }
}

// =============================================================================
// Peak accounting
// =============================================================================

/// Thread-safe maximum-value tracker.
#[derive(Debug)]
struct PeakCounter(AtomicU64);

impl PeakCounter {
    const fn new() -> Self {
        Self(AtomicU64::new(0))
    }

    fn observe(&self, value: u64) {
        let mut current = self.0.load(Ordering::Relaxed);

        while value > current {
            match self.0.compare_exchange_weak(
                current,
                value,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => return,
                Err(observed) => current = observed,
            }
        }
    }

    fn load(&self) -> u64 {
        self.0.load(Ordering::Relaxed)
    }
}

// =============================================================================
// Dimension statistics
// =============================================================================

/// Statistics for one dynamically labelled dimension.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DimensionStatistics {
    /// Current bytes attributed to this dimension.
    pub current_bytes: u64,

    /// Peak bytes attributed to this dimension.
    pub peak_bytes: u64,

    /// Total allocation bytes attributed to this dimension.
    pub allocated_bytes_total: u64,

    /// Total released bytes attributed to this dimension.
    pub released_bytes_total: u64,

    /// Number of allocation events.
    pub allocation_count: u64,

    /// Number of release events.
    pub release_count: u64,
}

#[derive(Debug, Default)]
struct MutableDimensionStatistics {
    current_bytes: u64,
    peak_bytes: u64,
    allocated_bytes_total: u64,
    released_bytes_total: u64,
    allocation_count: u64,
    release_count: u64,
}

impl MutableDimensionStatistics {
    fn allocate(&mut self, bytes: u64) {
        self.current_bytes = self.current_bytes.saturating_add(bytes);

        self.peak_bytes = self.peak_bytes.max(self.current_bytes);

        self.allocated_bytes_total =
            self.allocated_bytes_total.saturating_add(bytes);

        self.allocation_count =
            self.allocation_count.saturating_add(1);
    }

    fn release(&mut self, bytes: u64) {
        self.current_bytes = self.current_bytes.saturating_sub(bytes);

        self.released_bytes_total =
            self.released_bytes_total.saturating_add(bytes);

        self.release_count =
            self.release_count.saturating_add(1);
    }

    fn snapshot(&self) -> DimensionStatistics {
        DimensionStatistics {
            current_bytes: self.current_bytes,
            peak_bytes: self.peak_bytes,
            allocated_bytes_total: self.allocated_bytes_total,
            released_bytes_total: self.released_bytes_total,
            allocation_count: self.allocation_count,
            release_count: self.release_count,
        }
    }
}

// =============================================================================
// Resource pressure
// =============================================================================

/// Coarse memory-pressure classification.
///
/// This classification is based only on caller-supplied utilization ratios.
/// Diagnostics does not inspect operating-system memory or provider internals.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MemoryPressure {
    /// No configured capacity information is available.
    Unknown,

    /// Utilization is below 50%.
    Normal,

    /// Utilization is at least 50% but below 75%.
    Elevated,

    /// Utilization is at least 75% but below 90%.
    High,

    /// Utilization is at least 90%.
    Critical,
}

impl MemoryPressure {
    /// Classifies a utilization ratio represented in the range `[0, 1]`.
    ///
    /// Values outside that range are clamped rather than rejected because
    /// diagnostics must remain non-failing.
    pub fn from_ratio(ratio: f64) -> Self {
        if !ratio.is_finite() {
            return Self::Unknown;
        }

        let ratio = ratio.clamp(0.0, 1.0);

        if ratio >= 0.90 {
            Self::Critical
        } else if ratio >= 0.75 {
            Self::High
        } else if ratio >= 0.50 {
            Self::Elevated
        } else {
            Self::Normal
        }
    }

    /// Returns a stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Normal => "normal",
            Self::Elevated => "elevated",
            Self::High => "high",
            Self::Critical => "critical",
        }
    }
}

impl fmt::Display for MemoryPressure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Health
// =============================================================================

/// Diagnostic health state.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DiagnosticsHealth {
    /// Diagnostics is operating normally.
    Healthy,

    /// Diagnostics has observed a recoverable accounting inconsistency.
    Degraded,

    /// Diagnostics collection has been disabled.
    Disabled,
}

impl DiagnosticsHealth {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Degraded => "degraded",
            Self::Disabled => "disabled",
        }
    }
}

impl fmt::Display for DiagnosticsHealth {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Global counters snapshot
// =============================================================================

/// Point-in-time global diagnostic counters.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DiagnosticsCounters {
    /// Number of allocation events.
    pub allocations: u64,

    /// Number of release events.
    pub releases: u64,

    /// Number of failed allocation attempts reported to diagnostics.
    pub allocation_failures: u64,

    /// Number of allocation attempts rejected by policy.
    pub allocation_rejections: u64,

    /// Number of state-memory observations.
    pub state_observations: u64,

    /// Number of migration events.
    pub migrations: u64,

    /// Number of compaction events.
    pub compactions: u64,

    /// Number of synchronization events.
    pub synchronizations: u64,

    /// Number of synchronization failures.
    pub synchronization_failures: u64,

    /// Number of cache hits.
    pub cache_hits: u64,

    /// Number of cache misses.
    pub cache_misses: u64,

    /// Number of provider-memory observations.
    pub provider_observations: u64,

    /// Number of invariant violations reported.
    pub invariant_violations: u64,

    /// Number of diagnostic events dropped because a bounded dimension was
    /// full.
    pub dropped_dimension_events: u64,
}

// =============================================================================
// Provider snapshot
// =============================================================================

/// Diagnostic information for one provider.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProviderDiagnostic {
    /// Provider identifier.
    pub provider: String,

    /// Current attributed bytes.
    pub current_bytes: u64,

    /// Peak attributed bytes.
    pub peak_bytes: u64,

    /// Total observed allocation bytes.
    pub allocated_bytes_total: u64,

    /// Total observed release bytes.
    pub released_bytes_total: u64,

    /// Number of allocation events.
    pub allocation_count: u64,

    /// Number of release events.
    pub release_count: u64,
}

// =============================================================================
// Full diagnostic snapshot
// =============================================================================

/// Immutable point-in-time memory diagnostics.
///
/// This structure is suitable for:
///
/// - telemetry;
/// - benchmarking;
/// - logs;
/// - runtime diagnostics;
/// - dashboards;
/// - test assertions;
/// - Danga diagnostics commands;
/// - future serialization.
///
/// It contains no live synchronization primitives.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MemoryDiagnosticsSnapshot {
    /// Diagnostics schema identifier.
    pub schema_id: &'static str,

    /// Diagnostics schema version.
    pub schema_version: u16,

    /// Nanoseconds elapsed since the collector was created.
    pub elapsed_nanos: u128,

    /// Whether diagnostics collection is enabled.
    pub enabled: bool,

    /// Diagnostic health state.
    pub health: DiagnosticsHealth,

    /// Total current bytes.
    pub current_bytes: u64,

    /// Total peak bytes.
    pub peak_bytes: u64,

    /// Total bytes ever allocated through recorded events.
    pub allocated_bytes_total: u64,

    /// Total bytes ever released through recorded events.
    pub released_bytes_total: u64,

    /// Current number of logically live allocation events.
    pub live_allocations: u64,

    /// Current observed quantum-state bytes.
    pub state_current_bytes: u64,

    /// Peak observed quantum-state bytes.
    pub state_peak_bytes: u64,

    /// Current observed provider/device bytes.
    pub provider_current_bytes: u64,

    /// Peak observed provider/device bytes.
    pub provider_peak_bytes: u64,

    /// Current observed temporary bytes.
    pub temporary_current_bytes: u64,

    /// Peak observed temporary bytes.
    pub temporary_peak_bytes: u64,

    /// Current observed persistent bytes.
    pub persistent_current_bytes: u64,

    /// Peak observed persistent bytes.
    pub persistent_peak_bytes: u64,

    /// Current observed checkpoint bytes.
    pub checkpoint_current_bytes: u64,

    /// Peak observed checkpoint bytes.
    pub checkpoint_peak_bytes: u64,

    /// Global event counters.
    pub counters: DiagnosticsCounters,

    /// Storage-location statistics.
    pub by_storage: BTreeMap<String, DimensionStatistics>,

    /// Allocation-class statistics.
    pub by_allocation_class: BTreeMap<String, DimensionStatistics>,

    /// State-representation statistics.
    pub by_representation: BTreeMap<String, DimensionStatistics>,

    /// Provider statistics.
    pub by_provider: BTreeMap<String, DimensionStatistics>,

    /// User/caller-defined bounded diagnostic dimensions.
    pub custom_dimensions: BTreeMap<String, DimensionStatistics>,
}

// =============================================================================
// Internal state
// =============================================================================

#[derive(Debug)]
struct DiagnosticsInner {
    total_current: ByteCounter,
    total_peak: PeakCounter,
    total_allocated: ByteCounter,
    total_released: ByteCounter,

    live_allocations: Counter,

    state_current: ByteCounter,
    state_peak: PeakCounter,

    provider_current: ByteCounter,
    provider_peak: PeakCounter,

    temporary_current: ByteCounter,
    temporary_peak: PeakCounter,

    persistent_current: ByteCounter,
    persistent_peak: PeakCounter,

    checkpoint_current: ByteCounter,
    checkpoint_peak: PeakCounter,

    allocations: Counter,
    releases: Counter,
    allocation_failures: Counter,
    allocation_rejections: Counter,

    state_observations: Counter,
    migrations: Counter,
    compactions: Counter,

    synchronizations: Counter,
    synchronization_failures: Counter,

    cache_hits: Counter,
    cache_misses: Counter,

    provider_observations: Counter,
    invariant_violations: Counter,

    dropped_dimension_events: Counter,

    by_storage: Mutex<BTreeMap<String, MutableDimensionStatistics>>,
    by_allocation_class: Mutex<BTreeMap<String, MutableDimensionStatistics>>,
    by_representation: Mutex<BTreeMap<String, MutableDimensionStatistics>>,
    by_provider: Mutex<BTreeMap<String, MutableDimensionStatistics>>,
    custom_dimensions: Mutex<BTreeMap<String, MutableDimensionStatistics>>,

    health_degraded: AtomicBool,
    enabled: AtomicBool,
}

// =============================================================================
// Collector
// =============================================================================

/// Thread-safe production memory diagnostics collector.
///
/// Cloning this object creates another handle to the same collector.
///
/// The collector is deliberately not a global singleton. The runtime may
/// create one collector per process, execution session, simulator, QPU
/// session, test, or application context as appropriate.
#[derive(Clone, Debug)]
pub struct MemoryDiagnostics {
    inner: Arc<DiagnosticsInner>,
    created_at: Instant,
    max_dimension_entries: usize,
}

impl Default for MemoryDiagnostics {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryDiagnostics {
    /// Creates a diagnostics collector with default bounded dimensions.
    pub fn new() -> Self {
        Self::with_max_dimension_entries(DEFAULT_MAX_DIMENSION_ENTRIES)
    }

    /// Creates a collector with an explicit maximum number of entries in each
    /// dynamic dimension.
    ///
    /// A zero capacity is accepted and means dynamic dimensions are disabled.
    pub fn with_max_dimension_entries(max_dimension_entries: usize) -> Self {
        Self {
            inner: Arc::new(DiagnosticsInner {
                total_current: ByteCounter::new(),
                total_peak: PeakCounter::new(),
                total_allocated: ByteCounter::new(),
                total_released: ByteCounter::new(),

                live_allocations: Counter::new(),

                state_current: ByteCounter::new(),
                state_peak: PeakCounter::new(),

                provider_current: ByteCounter::new(),
                provider_peak: PeakCounter::new(),

                temporary_current: ByteCounter::new(),
                temporary_peak: PeakCounter::new(),

                persistent_current: ByteCounter::new(),
                persistent_peak: PeakCounter::new(),

                checkpoint_current: ByteCounter::new(),
                checkpoint_peak: PeakCounter::new(),

                allocations: Counter::new(),
                releases: Counter::new(),
                allocation_failures: Counter::new(),
                allocation_rejections: Counter::new(),

                state_observations: Counter::new(),
                migrations: Counter::new(),
                compactions: Counter::new(),

                synchronizations: Counter::new(),
                synchronization_failures: Counter::new(),

                cache_hits: Counter::new(),
                cache_misses: Counter::new(),

                provider_observations: Counter::new(),
                invariant_violations: Counter::new(),

                dropped_dimension_events: Counter::new(),

                by_storage: Mutex::new(BTreeMap::new()),
                by_allocation_class: Mutex::new(BTreeMap::new()),
                by_representation: Mutex::new(BTreeMap::new()),
                by_provider: Mutex::new(BTreeMap::new()),
                custom_dimensions: Mutex::new(BTreeMap::new()),

                health_degraded: AtomicBool::new(false),
                enabled: AtomicBool::new(true),
            }),
            created_at: Instant::now(),
            max_dimension_entries,
        }
    }

    /// Enables diagnostics collection.
    pub fn enable(&self) {
        self.inner.enabled.store(true, Ordering::Release);
    }

    /// Disables diagnostics collection.
    ///
    /// Disabling collection does not erase existing counters.
    pub fn disable(&self) {
        self.inner.enabled.store(false, Ordering::Release);
    }

    /// Returns whether collection is enabled.
    pub fn is_enabled(&self) -> bool {
        self.inner.enabled.load(Ordering::Acquire)
    }

    /// Clears the degraded-health flag.
    ///
    /// This does not repair underlying application/resource state. It only
    /// acknowledges the diagnostic condition.
    pub fn acknowledge_health(&self) {
        self.inner
            .health_degraded
            .store(false, Ordering::Release);
    }

    /// Returns the current diagnostics health.
    pub fn health(&self) -> DiagnosticsHealth {
        if !self.is_enabled() {
            DiagnosticsHealth::Disabled
        } else if self
            .inner
            .health_degraded
            .load(Ordering::Acquire)
        {
            DiagnosticsHealth::Degraded
        } else {
            DiagnosticsHealth::Healthy
        }
    }

    // =========================================================================
    // Allocation events
    // =========================================================================

    /// Records a successful allocation.
    ///
    /// `bytes` is the amount charged to diagnostics. The allocator remains
    /// authoritative for actual allocation/accounting.
    pub fn record_allocation(
        &self,
        bytes: ByteCount,
        storage: DiagnosticStorageKind,
        class: DiagnosticAllocationClass,
    ) {
        if !self.is_enabled() {
            return;
        }

        let bytes = bytes.get();

        self.inner.total_current.add(bytes);
        self.inner.total_peak.observe(self.inner.total_current.load());

        self.inner.total_allocated.add(bytes);
        self.inner.live_allocations.increment();
        self.inner.allocations.increment();

        match class {
            DiagnosticAllocationClass::State => {
                self.inner.state_current.add(bytes);
                self.inner
                    .state_peak
                    .observe(self.inner.state_current.load());
            }
            DiagnosticAllocationClass::Temporary => {
                self.inner.temporary_current.add(bytes);
                self.inner
                    .temporary_peak
                    .observe(self.inner.temporary_current.load());
            }
            DiagnosticAllocationClass::Persistent => {
                self.inner.persistent_current.add(bytes);
                self.inner
                    .persistent_peak
                    .observe(self.inner.persistent_current.load());
            }
            DiagnosticAllocationClass::Checkpoint => {
                self.inner.checkpoint_current.add(bytes);
                self.inner
                    .checkpoint_peak
                    .observe(self.inner.checkpoint_current.load());
            }
            DiagnosticAllocationClass::Metadata
            | DiagnosticAllocationClass::Other => {}
        }

        if storage == DiagnosticStorageKind::Device
            || storage == DiagnosticStorageKind::Unified
            || storage == DiagnosticStorageKind::Distributed
            || storage == DiagnosticStorageKind::BackendNative
        {
            self.inner.provider_current.add(bytes);
            self.inner
                .provider_peak
                .observe(self.inner.provider_current.load());
        }

        self.record_dimension_allocate(
            &self.inner.by_storage,
            storage.as_str(),
            bytes,
            MAX_LOCATION_ENTRIES,
        );

        self.record_dimension_allocate(
            &self.inner.by_allocation_class,
            class.as_str(),
            bytes,
            MAX_ALLOCATION_CLASS_ENTRIES,
        );
    }

    /// Records a successful release.
    ///
    /// Subtraction saturates to zero. If a caller reports more released bytes
    /// than currently observed, diagnostics marks itself degraded rather than
    /// panicking or producing an impossible negative value.
    pub fn record_release(
        &self,
        bytes: ByteCount,
        storage: DiagnosticStorageKind,
        class: DiagnosticAllocationClass,
    ) {
        if !self.is_enabled() {
            return;
        }

        let bytes = bytes.get();

        let current_before = self.inner.total_current.load();

        if bytes > current_before {
            self.inner
                .health_degraded
                .store(true, Ordering::Release);
        }

        self.inner.total_current.subtract_saturating(bytes);
        self.inner.total_released.add(bytes);

        self.inner.live_allocations.subtract_saturating(1);
        self.inner.releases.increment();

        match class {
            DiagnosticAllocationClass::State => {
                let current = self.inner.state_current.load();

                if bytes > current {
                    self.inner
                        .health_degraded
                        .store(true, Ordering::Release);
                }

                self.inner.state_current.subtract_saturating(bytes);
            }
            DiagnosticAllocationClass::Temporary => {
                self.inner.temporary_current.subtract_saturating(bytes);
            }
            DiagnosticAllocationClass::Persistent => {
                self.inner.persistent_current.subtract_saturating(bytes);
            }
            DiagnosticAllocationClass::Checkpoint => {
                self.inner.checkpoint_current.subtract_saturating(bytes);
            }
            DiagnosticAllocationClass::Metadata
            | DiagnosticAllocationClass::Other => {}
        }

        if storage == DiagnosticStorageKind::Device
            || storage == DiagnosticStorageKind::Unified
            || storage == DiagnosticStorageKind::Distributed
            || storage == DiagnosticStorageKind::BackendNative
        {
            self.inner.provider_current.subtract_saturating(bytes);
        }

        self.record_dimension_release(
            &self.inner.by_storage,
            storage.as_str(),
            bytes,
        );

        self.record_dimension_release(
            &self.inner.by_allocation_class,
            class.as_str(),
            bytes,
        );
    }

    /// Records an allocation failure.
    pub fn record_allocation_failure(&self) {
        if self.is_enabled() {
            self.inner.allocation_failures.increment();
        }
    }

    /// Records a policy rejection such as a memory-limit or budget rejection.
    pub fn record_allocation_rejection(&self) {
        if self.is_enabled() {
            self.inner.allocation_rejections.increment();
        }
    }

    // =========================================================================
    // State representation
    // =========================================================================

    /// Records the currently attributed bytes of a quantum-state
    /// representation.
    ///
    /// This method is useful when the state implementation has more precise
    /// information than individual allocation events.
    ///
    /// The observation replaces the current value attributed to the supplied
    /// representation only if the caller uses the `replace_*` APIs below;
    /// this method is an additive accounting event.
    pub fn record_state_memory(
        &self,
        representation: impl AsRef<str>,
        bytes: ByteCount,
        qubits: QubitCount,
    ) {
        if !self.is_enabled() {
            return;
        }

        let label = sanitize_label(representation.as_ref());
        let bytes = bytes.get();

        self.inner.state_observations.increment();

        self.inner.state_current.add(bytes);
        self.inner
            .state_peak
            .observe(self.inner.state_current.load());

        self.record_dimension_allocate(
            &self.inner.by_representation,
            &label,
            bytes,
            MAX_REPRESENTATION_ENTRIES,
        );

        let _ = qubits;
    }

    /// Records a provider/device memory observation.
    ///
    /// The provider name remains opaque and can represent any hardware,
    /// accelerator, simulator or QPU provider.
    pub fn record_provider_memory(
        &self,
        provider: impl AsRef<str>,
        storage: DiagnosticStorageKind,
        bytes: ByteCount,
    ) {
        if !self.is_enabled() {
            return;
        }

        let provider = sanitize_label(provider.as_ref());
        let bytes = bytes.get();

        self.inner.provider_observations.increment();

        self.inner.provider_current.add(bytes);
        self.inner
            .provider_peak
            .observe(self.inner.provider_current.load());

        self.record_dimension_allocate(
            &self.inner.by_provider,
            &provider,
            bytes,
            MAX_PROVIDER_ENTRIES,
        );

        self.record_dimension_allocate(
            &self.inner.by_storage,
            storage.as_str(),
            bytes,
            MAX_LOCATION_ENTRIES,
        );
    }

    // =========================================================================
    // Migration / compaction
    // =========================================================================

    /// Records a representation or storage migration.
    pub fn record_migration(&self, bytes_moved: ByteCount) {
        if !self.is_enabled() {
            return;
        }

        self.inner.migrations.increment();

        let _ = bytes_moved;
    }

    /// Records a memory compaction operation.
    pub fn record_compaction(&self, bytes_moved: ByteCount) {
        if !self.is_enabled() {
            return;
        }

        self.inner.compactions.increment();

        let _ = bytes_moved;
    }

    // =========================================================================
    // Synchronization
    // =========================================================================

    /// Records a successful synchronization event.
    pub fn record_synchronization(&self, bytes: ByteCount) {
        if !self.is_enabled() {
            return;
        }

        self.inner.synchronizations.increment();

        let _ = bytes;
    }

    /// Records a synchronization failure.
    pub fn record_synchronization_failure(&self) {
        if self.is_enabled() {
            self.inner.synchronization_failures.increment();
        }
    }

    // =========================================================================
    // Cache
    // =========================================================================

    /// Records a cache hit.
    pub fn record_cache_hit(&self) {
        if self.is_enabled() {
            self.inner.cache_hits.increment();
        }
    }

    /// Records a cache miss.
    pub fn record_cache_miss(&self) {
        if self.is_enabled() {
            self.inner.cache_misses.increment();
        }
    }

    // =========================================================================
    // Invariants
    // =========================================================================

    /// Records an invariant violation observed by another memory subsystem.
    ///
    /// Diagnostics does not decide whether the violation is fatal. The owning
    /// subsystem remains responsible for enforcing its invariant.
    pub fn record_invariant_violation(&self) {
        if !self.is_enabled() {
            return;
        }

        self.inner.invariant_violations.increment();

        self.inner
            .health_degraded
            .store(true, Ordering::Release);
    }

    // =========================================================================
    // Custom dimensions
    // =========================================================================

    /// Records allocation accounting against an application-defined bounded
    /// diagnostic dimension.
    ///
    /// Example dimensions:
    ///
    /// - `"execution_session"`;
    /// - `"circuit_type"`;
    /// - `"tenant"`;
    /// - `"workload"`;
    /// - `"memory_domain"`.
    ///
    /// The dimension name and value are both bounded.
    pub fn record_custom_allocation(
        &self,
        dimension: impl AsRef<str>,
        value: impl AsRef<str>,
        bytes: ByteCount,
    ) {
        if !self.is_enabled() {
            return;
        }

        let dimension = sanitize_label(dimension.as_ref());
        let value = sanitize_label(value.as_ref());

        let key = format!("{dimension}={value}");

        self.record_dimension_allocate(
            &self.inner.custom_dimensions,
            &key,
            bytes.get(),
            self.max_dimension_entries,
        );
    }

    /// Records a custom-dimension release.
    pub fn record_custom_release(
        &self,
        dimension: impl AsRef<str>,
        value: impl AsRef<str>,
        bytes: ByteCount,
    ) {
        if !self.is_enabled() {
            return;
        }

        let dimension = sanitize_label(dimension.as_ref());
        let value = sanitize_label(value.as_ref());

        let key = format!("{dimension}={value}");

        self.record_dimension_release(
            &self.inner.custom_dimensions,
            &key,
            bytes.get(),
        );
    }

    // =========================================================================
    // Counters
    // =========================================================================

    /// Returns a point-in-time counter snapshot.
    pub fn counters(&self) -> DiagnosticsCounters {
        DiagnosticsCounters {
            allocations: self.inner.allocations.load(),
            releases: self.inner.releases.load(),
            allocation_failures: self.inner.allocation_failures.load(),
            allocation_rejections: self.inner.allocation_rejections.load(),
            state_observations: self.inner.state_observations.load(),
            migrations: self.inner.migrations.load(),
            compactions: self.inner.compactions.load(),
            synchronizations: self.inner.synchronizations.load(),
            synchronization_failures: self.inner.synchronization_failures.load(),
            cache_hits: self.inner.cache_hits.load(),
            cache_misses: self.inner.cache_misses.load(),
            provider_observations: self.inner.provider_observations.load(),
            invariant_violations: self.inner.invariant_violations.load(),
            dropped_dimension_events: self.inner.dropped_dimension_events.load(),
        }
    }

    /// Returns the total current memory attributed to diagnostics.
    pub fn current_bytes(&self) -> ByteCount {
        ByteCount::new(self.inner.total_current.load())
    }

    /// Returns the peak total memory attributed to diagnostics.
    pub fn peak_bytes(&self) -> ByteCount {
        ByteCount::new(self.inner.total_peak.load())
    }

    /// Returns current quantum-state memory.
    pub fn state_current_bytes(&self) -> ByteCount {
        ByteCount::new(self.inner.state_current.load())
    }

    /// Returns peak quantum-state memory.
    pub fn state_peak_bytes(&self) -> ByteCount {
        ByteCount::new(self.inner.state_peak.load())
    }

    /// Returns the current provider/device memory attributed to diagnostics.
    pub fn provider_current_bytes(&self) -> ByteCount {
        ByteCount::new(self.inner.provider_current.load())
    }

    /// Returns peak provider/device memory.
    pub fn provider_peak_bytes(&self) -> ByteCount {
        ByteCount::new(self.inner.provider_peak.load())
    }

    /// Returns the current number of logically live allocation events.
    pub fn live_allocations(&self) -> u64 {
        self.inner.live_allocations.load()
    }

    /// Returns the cache hit ratio.
    ///
    /// If no cache accesses have been recorded, returns `None`.
    pub fn cache_hit_ratio(&self) -> Option<f64> {
        let hits = self.inner.cache_hits.load();
        let misses = self.inner.cache_misses.load();
        let total = hits.saturating_add(misses);

        if total == 0 {
            None
        } else {
            Some(hits as f64 / total as f64)
        }
    }

    // =========================================================================
    // Memory pressure
    // =========================================================================

    /// Classifies memory pressure using a caller-provided capacity.
    ///
    /// This method never queries OS, GPU or QPU APIs.
    pub fn memory_pressure(&self, capacity: Option<ByteCount>) -> MemoryPressure {
        match capacity {
            Some(capacity) if capacity.get() > 0 => {
                let current = self.current_bytes().get();

                MemoryPressure::from_ratio(
                    current as f64 / capacity.get() as f64,
                )
            }
            _ => MemoryPressure::Unknown,
        }
    }

    // =========================================================================
    // Snapshot
    // =========================================================================

    /// Creates an immutable point-in-time diagnostics snapshot.
    ///
    /// Dynamic maps are copied under short-lived mutex guards. No lock is held
    /// while constructing the complete snapshot.
    pub fn snapshot(&self) -> MemoryDiagnosticsSnapshot {
        let by_storage = snapshot_dimension(&self.inner.by_storage);
        let by_allocation_class =
            snapshot_dimension(&self.inner.by_allocation_class);
        let by_representation =
            snapshot_dimension(&self.inner.by_representation);
        let by_provider = snapshot_dimension(&self.inner.by_provider);
        let custom_dimensions =
            snapshot_dimension(&self.inner.custom_dimensions);

        MemoryDiagnosticsSnapshot {
            schema_id: MEMORY_DIAGNOSTICS_SCHEMA_ID,
            schema_version: MEMORY_DIAGNOSTICS_SCHEMA_VERSION,

            elapsed_nanos: self.created_at.elapsed().as_nanos(),

            enabled: self.is_enabled(),
            health: self.health(),

            current_bytes: self.inner.total_current.load(),
            peak_bytes: self.inner.total_peak.load(),
            allocated_bytes_total: self.inner.total_allocated.load(),
            released_bytes_total: self.inner.total_released.load(),

            live_allocations: self.inner.live_allocations.load(),

            state_current_bytes: self.inner.state_current.load(),
            state_peak_bytes: self.inner.state_peak.load(),

            provider_current_bytes: self.inner.provider_current.load(),
            provider_peak_bytes: self.inner.provider_peak.load(),

            temporary_current_bytes: self.inner.temporary_current.load(),
            temporary_peak_bytes: self.inner.temporary_peak.load(),

            persistent_current_bytes: self.inner.persistent_current.load(),
            persistent_peak_bytes: self.inner.persistent_peak.load(),

            checkpoint_current_bytes: self.inner.checkpoint_current.load(),
            checkpoint_peak_bytes: self.inner.checkpoint_peak.load(),

            counters: self.counters(),

            by_storage,
            by_allocation_class,
            by_representation,
            by_provider,
            custom_dimensions,
        }
    }

    // =========================================================================
    // Internal dimension operations
    // =========================================================================

    fn record_dimension_allocate(
        &self,
        map: &Mutex<BTreeMap<String, MutableDimensionStatistics>>,
        key: &str,
        bytes: u64,
        capacity: usize,
    ) {
        let Ok(mut map) = map.lock() else {
            self.inner
                .health_degraded
                .store(true, Ordering::Release);
            return;
        };

        if let Some(statistics) = map.get_mut(key) {
            statistics.allocate(bytes);
            return;
        }

        if map.len() >= capacity {
            self.inner.dropped_dimension_events.increment();
            return;
        }

        let mut statistics = MutableDimensionStatistics::default();
        statistics.allocate(bytes);

        map.insert(key.to_owned(), statistics);
    }

    fn record_dimension_release(
        &self,
        map: &Mutex<BTreeMap<String, MutableDimensionStatistics>>,
        key: &str,
        bytes: u64,
    ) {
        let Ok(mut map) = map.lock() else {
            self.inner
                .health_degraded
                .store(true, Ordering::Release);
            return;
        };

        if let Some(statistics) = map.get_mut(key) {
            statistics.release(bytes);
        } else {
            self.inner
                .health_degraded
                .store(true, Ordering::Release);
        }
    }
}

// =============================================================================
// Public helper functions
// =============================================================================

/// Sanitizes a diagnostic label.
///
/// The function is intentionally conservative:
///
/// - removes control characters;
/// - replaces them with `_`;
/// - limits Unicode scalar count;
/// - preserves valid UTF-8.
///
/// This is not a secret detector. Callers remain responsible for not supplying
/// credentials or sensitive data.
pub fn sanitize_label(value: &str) -> String {
    value
        .chars()
        .map(|character| {
            if character.is_control() {
                '_'
            } else {
                character
            }
        })
        .take(MAX_LABEL_LENGTH)
        .collect()
}

/// Returns a deterministic snapshot of a diagnostics dimension.
fn snapshot_dimension(
    map: &Mutex<BTreeMap<String, MutableDimensionStatistics>>,
) -> BTreeMap<String, DimensionStatistics> {
    let Ok(map) = map.lock() else {
        return BTreeMap::new();
    };

    map.iter()
        .map(|(key, value)| (key.clone(), value.snapshot()))
        .collect()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_collector_is_empty() {
        let diagnostics = MemoryDiagnostics::new();

        assert_eq!(diagnostics.current_bytes().get(), 0);
        assert_eq!(diagnostics.peak_bytes().get(), 0);
        assert_eq!(diagnostics.live_allocations(), 0);
        assert_eq!(diagnostics.health(), DiagnosticsHealth::Healthy);
    }

    #[test]
    fn allocation_and_release_are_accounted() {
        let diagnostics = MemoryDiagnostics::new();

        diagnostics.record_allocation(
            ByteCount::new(1024),
            DiagnosticStorageKind::Host,
            DiagnosticAllocationClass::State,
        );

        assert_eq!(diagnostics.current_bytes().get(), 1024);
        assert_eq!(diagnostics.state_current_bytes().get(), 1024);
        assert_eq!(diagnostics.peak_bytes().get(), 1024);
        assert_eq!(diagnostics.live_allocations(), 1);

        diagnostics.record_release(
            ByteCount::new(1024),
            DiagnosticStorageKind::Host,
            DiagnosticAllocationClass::State,
        );

        assert_eq!(diagnostics.current_bytes().get(), 0);
        assert_eq!(diagnostics.state_current_bytes().get(), 0);
        assert_eq!(diagnostics.peak_bytes().get(), 1024);
        assert_eq!(diagnostics.live_allocations(), 0);
    }

    #[test]
    fn peak_is_not_reduced_after_release() {
        let diagnostics = MemoryDiagnostics::new();

        diagnostics.record_allocation(
            ByteCount::new(4096),
            DiagnosticStorageKind::Host,
            DiagnosticAllocationClass::Temporary,
        );

        diagnostics.record_release(
            ByteCount::new(4096),
            DiagnosticStorageKind::Host,
            DiagnosticAllocationClass::Temporary,
        );

        diagnostics.record_allocation(
            ByteCount::new(1024),
            DiagnosticStorageKind::Host,
            DiagnosticAllocationClass::Temporary,
        );

        assert_eq!(diagnostics.current_bytes().get(), 1024);
        assert_eq!(diagnostics.peak_bytes().get(), 4096);
    }

    #[test]
    fn provider_memory_is_separately_accounted() {
        let diagnostics = MemoryDiagnostics::new();

        diagnostics.record_provider_memory(
            "future-accelerator",
            DiagnosticStorageKind::Device,
            ByteCount::new(8192),
        );

        assert_eq!(diagnostics.provider_current_bytes().get(), 8192);
        assert_eq!(diagnostics.provider_peak_bytes().get(), 8192);
    }

    #[test]
    fn representation_labels_are_bounded() {
        let diagnostics = MemoryDiagnostics::new();

        diagnostics.record_state_memory(
            "state_vector",
            ByteCount::new(128),
            QubitCount::new(3),
        );

        let snapshot = diagnostics.snapshot();

        assert_eq!(
            snapshot
                .by_representation
                .get("state_vector")
                .map(|value| value.current_bytes),
            Some(128)
        );
    }

    #[test]
    fn custom_dimension_is_bounded() {
        let diagnostics = MemoryDiagnostics::with_max_dimension_entries(1);

        diagnostics.record_custom_allocation(
            "workload",
            "first",
            ByteCount::new(10),
        );

        diagnostics.record_custom_allocation(
            "workload",
            "second",
            ByteCount::new(20),
        );

        let snapshot = diagnostics.snapshot();

        assert_eq!(snapshot.custom_dimensions.len(), 1);
        assert_eq!(snapshot.counters.dropped_dimension_events, 1);
    }

    #[test]
    fn cache_ratio_is_correct() {
        let diagnostics = MemoryDiagnostics::new();

        diagnostics.record_cache_hit();
        diagnostics.record_cache_hit();
        diagnostics.record_cache_miss();

        let ratio = diagnostics
            .cache_hit_ratio()
            .expect("cache accesses were recorded");

        assert!((ratio - (2.0 / 3.0)).abs() < f64::EPSILON);
    }

    #[test]
    fn cache_ratio_is_none_when_unused() {
        let diagnostics = MemoryDiagnostics::new();

        assert_eq!(diagnostics.cache_hit_ratio(), None);
    }

    #[test]
    fn pressure_thresholds_are_stable() {
        assert_eq!(
            MemoryPressure::from_ratio(0.10),
            MemoryPressure::Normal
        );

        assert_eq!(
            MemoryPressure::from_ratio(0.50),
            MemoryPressure::Elevated
        );

        assert_eq!(
            MemoryPressure::from_ratio(0.75),
            MemoryPressure::High
        );

        assert_eq!(
            MemoryPressure::from_ratio(0.90),
            MemoryPressure::Critical
        );

        assert_eq!(
            MemoryPressure::from_ratio(f64::NAN),
            MemoryPressure::Unknown
        );
    }

    #[test]
    fn over_release_degrades_health_without_panicking() {
        let diagnostics = MemoryDiagnostics::new();

        diagnostics.record_release(
            ByteCount::new(1024),
            DiagnosticStorageKind::Host,
            DiagnosticAllocationClass::Temporary,
        );

        assert_eq!(
            diagnostics.health(),
            DiagnosticsHealth::Degraded
        );

        assert_eq!(diagnostics.current_bytes().get(), 0);
    }

    #[test]
    fn invariant_violation_degrades_health() {
        let diagnostics = MemoryDiagnostics::new();

        diagnostics.record_invariant_violation();

        assert_eq!(
            diagnostics.health(),
            DiagnosticsHealth::Degraded
        );

        assert_eq!(
            diagnostics.counters().invariant_violations,
            1
        );
    }

    #[test]
    fn disabled_diagnostics_do_not_record_events() {
        let diagnostics = MemoryDiagnostics::new();

        diagnostics.disable();

        diagnostics.record_allocation(
            ByteCount::new(1024),
            DiagnosticStorageKind::Host,
            DiagnosticAllocationClass::State,
        );

        assert_eq!(diagnostics.current_bytes().get(), 0);
        assert_eq!(diagnostics.counters().allocations, 0);
        assert_eq!(
            diagnostics.health(),
            DiagnosticsHealth::Disabled
        );
    }

    #[test]
    fn cloned_collectors_share_state() {
        let first = MemoryDiagnostics::new();
        let second = first.clone();

        first.record_allocation(
            ByteCount::new(512),
            DiagnosticStorageKind::Host,
            DiagnosticAllocationClass::Persistent,
        );

        assert_eq!(second.current_bytes().get(), 512);
        assert_eq!(second.live_allocations(), 1);
    }

    #[test]
    fn snapshot_is_deterministically_ordered() {
        let diagnostics = MemoryDiagnostics::new();

        diagnostics.record_allocation(
            ByteCount::new(1),
            DiagnosticStorageKind::Host,
            DiagnosticAllocationClass::Temporary,
        );

        diagnostics.record_allocation(
            ByteCount::new(1),
            DiagnosticStorageKind::Device,
            DiagnosticAllocationClass::State,
        );

        let snapshot = diagnostics.snapshot();

        let keys: Vec<&String> = snapshot.by_storage.keys().collect();

        let mut sorted = keys.clone();
        sorted.sort();

        assert_eq!(keys, sorted);
    }

    #[test]
    fn labels_remove_control_characters() {
        let value = sanitize_label("hello\nworld\t");

        assert_eq!(value, "hello_world_");
    }

    #[test]
    fn labels_are_bounded() {
        let value = "x".repeat(MAX_LABEL_LENGTH + 100);
        let sanitized = sanitize_label(&value);

        assert_eq!(sanitized.chars().count(), MAX_LABEL_LENGTH);
    }

    #[test]
    fn schema_is_stable() {
        let diagnostics = MemoryDiagnostics::new();
        let snapshot = diagnostics.snapshot();

        assert_eq!(
            snapshot.schema_id,
            MEMORY_DIAGNOSTICS_SCHEMA_ID
        );

        assert_eq!(
            snapshot.schema_version,
            MEMORY_DIAGNOSTICS_SCHEMA_VERSION
        );
    }
}