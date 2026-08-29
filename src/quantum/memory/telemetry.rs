//! Zamani Quantum Memory — Production Telemetry
//!
//! Provider-neutral, bounded, deterministic and thread-safe telemetry for
//! `crate::quantum::memory`.
//!
//! # Purpose
//!
//! `telemetry.rs` is the machine-oriented observability boundary of the
//! quantum-memory subsystem.
//!
//! It records:
//!
//! - allocation/deallocation activity;
//! - current and peak memory usage;
//! - reserved memory;
//! - memory by storage location;
//! - memory by allocation class;
//! - state-representation usage;
//! - provider/backend/QPU resource observations;
//! - CPU/GPU/accelerator memory observations;
//! - distributed-memory observations;
//! - migration activity;
//! - synchronization activity;
//! - compaction activity;
//! - cache activity;
//! - allocation failures;
//! - budget/limit violations;
//! - operation durations;
//! - allocation sizes;
//! - migration sizes;
//! - synchronization sizes;
//! - generic hardware-resource measurements;
//! - deterministic snapshots suitable for exporters;
//! - bounded metric dimensions.
//!
//! # Architectural boundary
//!
//! This module DOES NOT own:
//!
//! - allocation;
//! - deallocation;
//! - memory budgets;
//! - memory limits;
//! - state-vector mathematics;
//! - density-matrix mathematics;
//! - stabilizer mathematics;
//! - tensor-network mathematics;
//! - GPU kernels;
//! - distributed communication;
//! - QPU APIs;
//! - routing;
//! - scheduling;
//! - benchmarking protocols;
//! - compiler semantics;
//! - quantum IR;
//! - authentication;
//! - credentials;
//! - provider SDKs.
//!
//! Other modules report events to this module.
//!
//! Telemetry must never influence quantum semantics.
//!
//! # Provider neutrality
//!
//! No vendor, QPU family, accelerator, simulator, transport, or hardware
//! provider is hard-coded into this module.
//!
//! The same API can represent:
//!
//! - superconducting QPUs;
//! - trapped-ion QPUs;
//! - neutral-atom QPUs;
//! - photonic processors;
//! - spin/semiconductor devices;
//! - topological devices;
//! - annealing hardware;
//! - analog quantum processors;
//! - quantum simulators;
//! - CPU simulators;
//! - GPU simulators;
//! - distributed simulators;
//! - remote execution services;
//! - future quantum hardware.
//!
//! A QPU may expose no classical memory addressable by Zamani. In that case,
//! telemetry records only the resource observations explicitly supplied by the
//! hardware adapter. It MUST NOT fabricate state-vector memory statistics.
//!
//! # Dependency direction
//!
//! ```text
//! quantum::memory::allocator.rs ───────┐
//! quantum::memory::pool.rs ────────────┤
//! quantum::memory::state.rs ───────────┤
//! quantum::memory::state_vector.rs ────┤
//! quantum::memory::density_matrix.rs ──┤
//! quantum::memory::stabilizer.rs ──────┤
//! quantum::memory::sparse.rs ──────────┤
//! quantum::memory::tensor_network.rs ──┤
//! quantum::memory::gpu.rs ─────────────┤
//! quantum::memory::distributed.rs ─────┤
//! quantum::memory::migration.rs ───────┤
//! quantum::memory::compaction.rs ──────┤
//! quantum::memory::cache.rs ────────────┤
//! quantum::memory::synchronization.rs ─┤
//!                                       ▼
//!                              quantum::memory::telemetry
//!                                       │
//!                    ┌──────────────────┼──────────────────┐
//!                    ▼                  ▼                  ▼
//!                diagnostics       runtime            exporters
//!                                      │
//!                         ┌────────────┼────────────┐
//!                         ▼            ▼            ▼
//!                     OpenTelemetry Prometheus   custom sink
//! ```
//!
//! `telemetry.rs` is therefore a consumer-facing observability contract.
//!
//! # Important rule
//!
//! Telemetry recording MUST be best-effort and must never cause a quantum
//! operation to fail merely because telemetry accounting is saturated,
//! contended, or otherwise unable to retain an optional dimension.
//!
//! Core counters are retained without dynamic allocation.
//!
//! Dynamic dimensions are bounded.
//!
//! # Security
//!
//! Telemetry MUST NOT contain:
//!
//! - API keys;
//! - access tokens;
//! - passwords;
//! - private keys;
//! - authentication headers;
//! - raw pointers;
//! - raw memory addresses;
//! - QPU credentials;
//! - secret circuit source;
//! - secret program contents;
//! - raw quantum-state amplitudes;
//! - measurement results unless explicitly recorded by another subsystem;
//! - arbitrary unbounded user strings.
//!
//! Labels are bounded and sanitized.
//!
//! # No unsafe
//!
//! This module contains no unsafe Rust.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features.
//!
//! # External telemetry integration
//!
//! The public snapshot types intentionally resemble the semantic categories
//! used by modern telemetry systems:
//!
//! - counters for cumulative events;
//! - gauges for current resource state;
//! - histograms for distributions.
//!
//! The module does not require OpenTelemetry or Prometheus. An adapter can map
//! `TelemetrySnapshot` to those systems without changing the memory subsystem.
//!
//! Current telemetry standards distinguish counters, gauges/up-down counters,
//! and histograms. Zamani keeps that semantic distinction internally while
//! remaining independent from a particular exporter.
//!
//! # Integration contract
//!
//! The foundational modules already provide:
//!
//! - `ByteCount`;
//! - `QubitCount`;
//! - provider-neutral state contracts;
//! - provider-neutral memory abstractions.
//!
//! This file intentionally does not import higher memory modules. This avoids
//! dependency cycles and allows this file to be completed independently.
//!
//! Later modules can integrate by calling methods such as:
//!
//! ```text
//! allocator.rs
//!     record_allocation()
//!     record_release()
//!
//! pool.rs
//!     record_pool_acquire()
//!     record_pool_release()
//!     record_pool_miss()
//!
//! state*.rs
//!     record_state_memory()
//!
//! gpu.rs / cpu.rs / distributed.rs
//!     record_resource_sample()
//!
//! migration.rs
//!     record_migration()
//!
//! synchronization.rs
//!     record_synchronization()
//!
//! compaction.rs
//!     record_compaction()
//!
//! cache.rs
//!     record_cache_hit()
//!     record_cache_miss()
//!
//! diagnostics.rs
//!     export or consume TelemetrySnapshot
//!
//! benchmarking
//!     consume TelemetrySnapshot
//! ```
//!
//! None of those integrations require changing this file merely because a
//! new state representation, QPU, accelerator, transport, or vendor is added.
//!
//! # Determinism
//!
//! - snapshots use `BTreeMap`;
//! - counters are integer based;
//! - dynamic dimensions have deterministic eviction behavior;
//! - no wall-clock timestamp is required for correctness;
//! - monotonic elapsed nanoseconds are used for local observation time;
//! - no hidden RNG exists.
//!
//! # Overflow policy
//!
//! Telemetry must never panic because a counter reaches its numeric maximum.
//!
//! Counters and byte quantities saturate at their maximum representable value.
//!
//! # Performance
//!
//! Hot-path core counters use atomics.
//!
//! Dynamic labelled dimensions use bounded mutex-protected maps.
//!
//! Callers performing extremely high-frequency operations should use the
//! unlabelled record methods on hot paths and periodically attach dimensions
//! through higher-level adapters.
//!
//! # Schema
//!
//! `TELEMETRY_SCHEMA_ID` and `TELEMETRY_SCHEMA_VERSION` identify the stable
//! machine-readable snapshot contract.
//!
//! Consumers MUST NOT parse `Debug` or `Display` output as a data format.

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

// =============================================================================
// Stable schema
// =============================================================================

/// Stable identifier for Zamani quantum-memory telemetry.
pub const TELEMETRY_SCHEMA_ID: &str = "zamani.quantum.memory.telemetry";

/// Semantic version of the telemetry schema.
pub const TELEMETRY_SCHEMA_VERSION: u16 = 1;

/// Default maximum number of dynamic dimensions retained.
pub const DEFAULT_MAX_DIMENSIONS: usize = 256;

/// Maximum length of a telemetry label in Unicode scalar values.
pub const MAX_LABEL_LENGTH: usize = 128;

/// Maximum number of buckets retained by one histogram.
pub const MAX_HISTOGRAM_BUCKETS: usize = 64;

// =============================================================================
// Core enums
// =============================================================================

/// Provider-neutral storage category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum StorageKind {
    /// Ordinary host RAM.
    Host,

    /// Page-locked/pinned host memory.
    PinnedHost,

    /// Accelerator/device memory.
    Device,

    /// Unified/shared host-device memory.
    Unified,

    /// Distributed memory spanning multiple nodes.
    Distributed,

    /// Backend-owned or provider-native memory.
    BackendNative,

    /// Remote/opaque resource for which no local memory representation exists.
    Remote,

    /// Future or provider-defined storage.
    Other,
}

impl StorageKind {
    /// Stable machine-readable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Host => "host",
            Self::PinnedHost => "pinned_host",
            Self::Device => "device",
            Self::Unified => "unified",
            Self::Distributed => "distributed",
            Self::BackendNative => "backend_native",
            Self::Remote => "remote",
            Self::Other => "other",
        }
    }
}

impl fmt::Display for StorageKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Semantic allocation category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum AllocationClass {
    /// Short-lived scratch allocation.
    Temporary,

    /// Long-lived application allocation.
    Persistent,

    /// Quantum-state storage.
    State,

    /// Snapshot/checkpoint storage.
    Checkpoint,

    /// Metadata.
    Metadata,

    /// Future/provider-defined class.
    Other,
}

impl AllocationClass {
    /// Stable machine-readable name.
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

impl fmt::Display for AllocationClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Bounded labels
// =============================================================================

/// Bounded provider/backend identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct ProviderLabel(String);

impl ProviderLabel {
    /// Creates a bounded provider label.
    pub fn new(value: impl AsRef<str>) -> Self {
        Self(sanitize_label(value.as_ref()))
    }

    /// Returns the label.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProviderLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Bounded backend/device identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct DeviceLabel(String);

impl DeviceLabel {
    /// Creates a bounded device label.
    pub fn new(value: impl AsRef<str>) -> Self {
        Self(sanitize_label(value.as_ref()))
    }

    /// Returns the label.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DeviceLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Bounded state-representation identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct RepresentationLabel(String);

impl RepresentationLabel {
    /// Creates a bounded representation label.
    pub fn new(value: impl AsRef<str>) -> Self {
        Self(sanitize_label(value.as_ref()))
    }

    /// Returns the label.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RepresentationLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Bounded hardware metric name.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct MetricLabel(String);

impl MetricLabel {
    /// Creates a bounded metric name.
    pub fn new(value: impl AsRef<str>) -> Self {
        Self(sanitize_label(value.as_ref()))
    }

    /// Returns the metric name.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for MetricLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Sanitizes a potentially untrusted telemetry label.
///
/// Telemetry labels are intentionally conservative:
///
/// - empty values become `"unknown"`;
/// - leading/trailing whitespace is removed;
/// - control characters become `_`;
/// - the label is bounded;
/// - no arbitrary memory growth is permitted.
fn sanitize_label(value: &str) -> String {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        return String::from("unknown");
    }

    let mut result = String::new();

    for ch in trimmed.chars().take(MAX_LABEL_LENGTH) {
        if ch.is_control() {
            result.push('_');
        } else {
            result.push(ch);
        }
    }

    if result.is_empty() {
        String::from("unknown")
    } else {
        result
    }
}

// =============================================================================
// Saturating atomic counters
// =============================================================================

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

    fn get(&self) -> u64 {
        self.0.load(Ordering::Relaxed)
    }
}

#[derive(Debug)]
struct Gauge(AtomicU64);

impl Gauge {
    const fn new() -> Self {
        Self(AtomicU64::new(0))
    }

    fn set(&self, value: u64) {
        self.0.store(value, Ordering::Relaxed);
    }

    fn get(&self) -> u64 {
        self.0.load(Ordering::Relaxed)
    }
}

#[derive(Debug)]
struct Peak(AtomicU64);

impl Peak {
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

    fn get(&self) -> u64 {
        self.0.load(Ordering::Relaxed)
    }
}

// =============================================================================
// Histogram
// =============================================================================

/// Snapshot of a bounded histogram.
#[derive(Debug, Clone, PartialEq)]
pub struct HistogramSnapshot {
    /// Number of observations.
    pub count: u64,

    /// Sum of observations.
    pub sum: f64,

    /// Minimum observation.
    pub min: Option<f64>,

    /// Maximum observation.
    pub max: Option<f64>,

    /// Arithmetic mean.
    pub mean: Option<f64>,

    /// Fixed histogram buckets.
    ///
    /// Each entry is `(upper_bound, count)`.
    pub buckets: Vec<(f64, u64)>,
}

#[derive(Debug)]
struct Histogram {
    observations: Counter,
    sum_bits: AtomicU64,
    min_bits: AtomicU64,
    max_bits: AtomicU64,
    buckets: Vec<f64>,
    bucket_counts: Vec<Counter>,
}

impl Histogram {
    fn new(mut buckets: Vec<f64>) -> Self {
        buckets.retain(|value| value.is_finite() && *value >= 0.0);
        buckets.sort_by(|a, b| a.total_cmp(b));
        buckets.dedup_by(|a, b| a.total_cmp(b).is_eq());
        buckets.truncate(MAX_HISTOGRAM_BUCKETS);

        let bucket_counts = (0..buckets.len())
            .map(|_| Counter::new())
            .collect::<Vec<_>>();

        Self {
            observations: Counter::new(),
            sum_bits: AtomicU64::new(0.0f64.to_bits()),
            min_bits: AtomicU64::new(f64::INFINITY.to_bits()),
            max_bits: AtomicU64::new(f64::NEG_INFINITY.to_bits()),
            buckets,
            bucket_counts,
        }
    }

    fn record(&self, value: f64) {
        if !value.is_finite() || value < 0.0 {
            return;
        }

        self.observations.increment();

        // The histogram sum is an observational value. Compare/exchange is
        // used so concurrent recording does not lose updates.
        let mut current = self.sum_bits.load(Ordering::Relaxed);

        loop {
            let current_value = f64::from_bits(current);
            let next_value = current_value + value;

            if !next_value.is_finite() {
                return;
            }

            match self.sum_bits.compare_exchange_weak(
                current,
                next_value.to_bits(),
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(observed) => current = observed,
            }
        }

        update_min(&self.min_bits, value);
        update_max(&self.max_bits, value);

        for (index, upper_bound) in self.buckets.iter().enumerate() {
            if value <= *upper_bound {
                self.bucket_counts[index].increment();
            }
        }
    }

    fn snapshot(&self) -> HistogramSnapshot {
        let count = self.observations.get();
        let sum = f64::from_bits(self.sum_bits.load(Ordering::Relaxed));

        let min = f64::from_bits(self.min_bits.load(Ordering::Relaxed));
        let max = f64::from_bits(self.max_bits.load(Ordering::Relaxed));

        let min = if min.is_finite() { Some(min) } else { None };
        let max = if max.is_finite() { Some(max) } else { None };

        let mean = if count == 0 {
            None
        } else {
            Some(sum / count as f64)
        };

        let buckets = self
            .buckets
            .iter()
            .enumerate()
            .map(|(index, upper_bound)| {
                (*upper_bound, self.bucket_counts[index].get())
            })
            .collect();

        HistogramSnapshot {
            count,
            sum,
            min,
            max,
            mean,
            buckets,
        }
    }
}

fn update_min(target: &AtomicU64, value: f64) {
    let mut current = target.load(Ordering::Relaxed);

    loop {
        let current_value = f64::from_bits(current);

        if value >= current_value {
            return;
        }

        match target.compare_exchange_weak(
            current,
            value.to_bits(),
            Ordering::Relaxed,
            Ordering::Relaxed,
        ) {
            Ok(_) => return,
            Err(observed) => current = observed,
        }
    }
}

fn update_max(target: &AtomicU64, value: f64) {
    let mut current = target.load(Ordering::Relaxed);

    loop {
        let current_value = f64::from_bits(current);

        if value <= current_value {
            return;
        }

        match target.compare_exchange_weak(
            current,
            value.to_bits(),
            Ordering::Relaxed,
            Ordering::Relaxed,
        ) {
            Ok(_) => return,
            Err(observed) => current = observed,
        }
    }
}

// =============================================================================
// Dynamic dimension
// =============================================================================

#[derive(Debug)]
struct DimensionCounter {
    values: Mutex<BTreeMap<String, u64>>,
    maximum: usize,
}

impl DimensionCounter {
    fn new(maximum: usize) -> Self {
        Self {
            values: Mutex::new(BTreeMap::new()),
            maximum: maximum.max(1),
        }
    }

    fn increment(&self, label: &str, amount: u64) {
        let label = sanitize_label(label);

        let mut values = match self.values.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        if !values.contains_key(&label) && values.len() >= self.maximum {
            let fallback = String::from("other");
            let entry = values.entry(fallback).or_insert(0);
            *entry = entry.saturating_add(amount);
            return;
        }

        let entry = values.entry(label).or_insert(0);
        *entry = entry.saturating_add(amount);
    }

    fn snapshot(&self) -> BTreeMap<String, u64> {
        match self.values.lock() {
            Ok(guard) => guard.clone(),
            Err(poisoned) => poisoned.into_inner().clone(),
        }
    }
}

// =============================================================================
// Configuration
// =============================================================================

/// Telemetry configuration.
#[derive(Debug, Clone)]
pub struct TelemetryConfig {
    /// Maximum number of provider labels.
    pub max_providers: usize,

    /// Maximum number of device/backend labels.
    pub max_devices: usize,

    /// Maximum number of representation labels.
    pub max_representations: usize,

    /// Maximum number of custom hardware metrics.
    pub max_hardware_metrics: usize,

    /// Whether event counters are enabled.
    pub record_events: bool,

    /// Whether duration histograms are enabled.
    pub record_histograms: bool,

    /// Histogram bucket boundaries for durations in nanoseconds.
    pub duration_buckets_ns: Vec<f64>,

    /// Histogram bucket boundaries for byte quantities.
    pub byte_buckets: Vec<f64>,

    /// Histogram bucket boundaries for qubit counts.
    pub qubit_buckets: Vec<f64>,
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            max_providers: DEFAULT_MAX_DIMENSIONS,
            max_devices: DEFAULT_MAX_DIMENSIONS,
            max_representations: DEFAULT_MAX_DIMENSIONS,
            max_hardware_metrics: DEFAULT_MAX_DIMENSIONS,
            record_events: true,
            record_histograms: true,
            duration_buckets_ns: vec![
                1_000.0,
                10_000.0,
                100_000.0,
                1_000_000.0,
                10_000_000.0,
                100_000_000.0,
                1_000_000_000.0,
                10_000_000_000.0,
            ],
            byte_buckets: vec![
                1_024.0,
                4_096.0,
                16_384.0,
                65_536.0,
                1_048_576.0,
                16_777_216.0,
                268_435_456.0,
                4_294_967_296.0,
                68_719_476_736.0,
                1_099_511_627_776.0,
            ],
            qubit_buckets: vec![
                1.0, 2.0, 4.0, 8.0, 16.0, 32.0, 64.0, 128.0, 256.0, 512.0, 1_024.0,
                4_096.0, 16_384.0, 65_536.0,
            ],
        }
    }
}

// =============================================================================
// Hardware observation
// =============================================================================

/// Generic hardware/resource observation.
///
/// This is intentionally not limited to a fixed vendor-specific metric set.
///
/// Examples:
///
/// - memory_utilization_ratio;
/// - temperature_celsius;
/// - power_watts;
/// - queue_depth;
/// - available_memory_bytes;
/// - memory_bandwidth_bytes_per_second;
/// - device_utilization_ratio;
/// - calibration_age_seconds.
///
/// The metric is accepted only if the value is finite.
#[derive(Debug, Clone)]
pub struct HardwareObservation {
    /// Provider/backend label.
    pub provider: ProviderLabel,

    /// Device/backend label.
    pub device: DeviceLabel,

    /// Metric name.
    pub metric: MetricLabel,

    /// Numeric value.
    pub value: f64,

    /// Unit identifier.
    pub unit: String,

    /// Monotonic elapsed time since collector creation.
    pub elapsed_ns: u64,
}

impl HardwareObservation {
    /// Creates an observation.
    ///
    /// Non-finite values are represented as zero and are therefore not
    /// propagated into telemetry.
    pub fn new(
        provider: impl AsRef<str>,
        device: impl AsRef<str>,
        metric: impl AsRef<str>,
        value: f64,
        unit: impl AsRef<str>,
        elapsed_ns: u64,
    ) -> Self {
        Self {
            provider: ProviderLabel::new(provider),
            device: DeviceLabel::new(device),
            metric: MetricLabel::new(metric),
            value: if value.is_finite() { value } else { 0.0 },
            unit: sanitize_label(unit.as_ref()),
            elapsed_ns,
        }
    }
}

// =============================================================================
// Snapshot structures
// =============================================================================

/// Point-in-time memory usage.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MemoryUsageSnapshot {
    /// Current allocated bytes.
    pub allocated_bytes: u64,

    /// Current reserved bytes.
    pub reserved_bytes: u64,

    /// Peak allocated bytes.
    pub peak_allocated_bytes: u64,

    /// Peak reserved bytes.
    pub peak_reserved_bytes: u64,
}

/// Cumulative event counters.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EventSnapshot {
    /// Number of allocations.
    pub allocations: u64,

    /// Number of releases.
    pub releases: u64,

    /// Allocation failures.
    pub allocation_failures: u64,

    /// Budget violations.
    pub budget_violations: u64,

    /// Limit violations.
    pub limit_violations: u64,

    /// Pool acquisitions.
    pub pool_acquires: u64,

    /// Pool releases.
    pub pool_releases: u64,

    /// Pool misses.
    pub pool_misses: u64,

    /// Migrations.
    pub migrations: u64,

    /// Synchronizations.
    pub synchronizations: u64,

    /// Compactions.
    pub compactions: u64,

    /// Cache hits.
    pub cache_hits: u64,

    /// Cache misses.
    pub cache_misses: u64,
}

/// Memory bytes grouped by storage kind.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StorageSnapshot {
    /// Bytes by storage kind.
    pub bytes: BTreeMap<String, u64>,
}

/// Memory bytes grouped by allocation class.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AllocationSnapshot {
    /// Bytes by allocation class.
    pub bytes: BTreeMap<String, u64>,
}

/// State-representation usage.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RepresentationSnapshot {
    /// Current bytes grouped by state representation.
    pub bytes: BTreeMap<String, u64>,

    /// Number of observed state instances grouped by representation.
    pub instances: BTreeMap<String, u64>,

    /// Qubit counts observed by representation.
    pub qubits: BTreeMap<String, u64>,
}

/// Provider/device resource usage.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProviderSnapshot {
    /// Bytes observed by provider.
    pub bytes: BTreeMap<String, u64>,

    /// Event counts by provider.
    pub events: BTreeMap<String, u64>,
}

/// Generic hardware metrics.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HardwareSnapshot {
    /// Latest observation for each provider/device/metric tuple.
    pub values: BTreeMap<String, f64>,

    /// Units associated with the metrics.
    pub units: BTreeMap<String, String>,
}

/// Histogram snapshots.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HistogramCollectionSnapshot {
    /// Allocation-size histogram.
    pub allocation_bytes: Option<HistogramSnapshot>,

    /// Release-size histogram.
    pub release_bytes: Option<HistogramSnapshot>,

    /// Migration-size histogram.
    pub migration_bytes: Option<HistogramSnapshot>,

    /// Synchronization-size histogram.
    pub synchronization_bytes: Option<HistogramSnapshot>,

    /// Operation-duration histogram.
    pub operation_duration_ns: Option<HistogramSnapshot>,

    /// Qubit-count histogram.
    pub state_qubits: Option<HistogramSnapshot>,
}

/// Complete immutable telemetry snapshot.
///
/// This is the primary integration boundary for diagnostics, benchmarking,
/// runtime and external exporters.
#[derive(Debug, Clone, PartialEq)]
pub struct TelemetrySnapshot {
    /// Stable schema identifier.
    pub schema_id: String,

    /// Stable schema version.
    pub schema_version: u16,

    /// Monotonic elapsed time since collector creation.
    pub elapsed_ns: u64,

    /// Whether collection is currently enabled.
    pub enabled: bool,

    /// Core memory usage.
    pub memory: MemoryUsageSnapshot,

    /// Cumulative events.
    pub events: EventSnapshot,

    /// Storage breakdown.
    pub storage: StorageSnapshot,

    /// Allocation-class breakdown.
    pub allocations: AllocationSnapshot,

    /// State-representation breakdown.
    pub representations: RepresentationSnapshot,

    /// Provider/backend breakdown.
    pub providers: ProviderSnapshot,

    /// Hardware observations.
    pub hardware: HardwareSnapshot,

    /// Histograms.
    pub histograms: HistogramCollectionSnapshot,
}

// =============================================================================
// Collector internals
// =============================================================================

#[derive(Debug)]
struct TelemetryInner {
    config: TelemetryConfig,

    enabled: AtomicBool,

    start: Instant,

    allocated_bytes: Gauge,
    reserved_bytes: Gauge,

    peak_allocated_bytes: Peak,
    peak_reserved_bytes: Peak,

    allocations: Counter,
    releases: Counter,
    allocation_failures: Counter,
    budget_violations: Counter,
    limit_violations: Counter,

    pool_acquires: Counter,
    pool_releases: Counter,
    pool_misses: Counter,

    migrations: Counter,
    synchronizations: Counter,
    compactions: Counter,

    cache_hits: Counter,
    cache_misses: Counter,

    storage_bytes: DimensionCounter,
    allocation_bytes: DimensionCounter,

    representation_bytes: DimensionCounter,
    representation_instances: DimensionCounter,
    representation_qubits: DimensionCounter,

    provider_bytes: DimensionCounter,
    provider_events: DimensionCounter,

    hardware_values: Mutex<BTreeMap<String, (f64, String)>>,

    allocation_histogram: Option<Histogram>,
    release_histogram: Option<Histogram>,
    migration_histogram: Option<Histogram>,
    synchronization_histogram: Option<Histogram>,
    operation_duration_histogram: Option<Histogram>,
    state_qubit_histogram: Option<Histogram>,
}

// =============================================================================
// Public collector
// =============================================================================

/// Thread-safe quantum-memory telemetry collector.
///
/// Cloning a collector creates another handle to the same underlying
/// telemetry state.
///
/// No global singleton is created.
#[derive(Clone, Debug)]
pub struct MemoryTelemetry {
    inner: Arc<TelemetryInner>,
}

impl MemoryTelemetry {
    /// Creates a collector with the supplied configuration.
    pub fn new(config: TelemetryConfig) -> Self {
        let record_histograms = config.record_histograms;

        let allocation_histogram = if record_histograms {
            Some(Histogram::new(config.byte_buckets.clone()))
        } else {
            None
        };

        let release_histogram = if record_histograms {
            Some(Histogram::new(config.byte_buckets.clone()))
        } else {
            None
        };

        let migration_histogram = if record_histograms {
            Some(Histogram::new(config.byte_buckets.clone()))
        } else {
            None
        };

        let synchronization_histogram = if record_histograms {
            Some(Histogram::new(config.byte_buckets.clone()))
        } else {
            None
        };

        let operation_duration_histogram = if record_histograms {
            Some(Histogram::new(config.duration_buckets_ns.clone()))
        } else {
            None
        };

        let state_qubit_histogram = if record_histograms {
            Some(Histogram::new(config.qubit_buckets.clone()))
        } else {
            None
        };

        let max_providers = config.max_providers.max(1);
        let max_devices = config.max_devices.max(1);
        let max_representations = config.max_representations.max(1);
        let max_hardware_metrics = config.max_hardware_metrics.max(1);

        Self {
            inner: Arc::new(TelemetryInner {
                config,

                enabled: AtomicBool::new(true),

                start: Instant::now(),

                allocated_bytes: Gauge::new(),
                reserved_bytes: Gauge::new(),

                peak_allocated_bytes: Peak::new(),
                peak_reserved_bytes: Peak::new(),

                allocations: Counter::new(),
                releases: Counter::new(),
                allocation_failures: Counter::new(),
                budget_violations: Counter::new(),
                limit_violations: Counter::new(),

                pool_acquires: Counter::new(),
                pool_releases: Counter::new(),
                pool_misses: Counter::new(),

                migrations: Counter::new(),
                synchronizations: Counter::new(),
                compactions: Counter::new(),

                cache_hits: Counter::new(),
                cache_misses: Counter::new(),

                storage_bytes: DimensionCounter::new(max_devices),
                allocation_bytes: DimensionCounter::new(max_devices),

                representation_bytes: DimensionCounter::new(max_representations),
                representation_instances: DimensionCounter::new(max_representations),
                representation_qubits: DimensionCounter::new(max_representations),

                provider_bytes: DimensionCounter::new(max_providers),
                provider_events: DimensionCounter::new(max_providers),

                hardware_values: Mutex::new(BTreeMap::new()),

                allocation_histogram,
                release_histogram,
                migration_histogram,
                synchronization_histogram,
                operation_duration_histogram,
                state_qubit_histogram,

                // `max_hardware_metrics` is represented through the bounded
                // hardware map at runtime.
            }),
        }
    }

    /// Creates a collector with production defaults.
    pub fn production() -> Self {
        Self::new(TelemetryConfig::default())
    }

    /// Returns whether collection is enabled.
    pub fn is_enabled(&self) -> bool {
        self.inner.enabled.load(Ordering::Relaxed)
    }

    /// Enables telemetry collection.
    pub fn enable(&self) {
        self.inner.enabled.store(true, Ordering::Relaxed);
    }

    /// Disables telemetry collection.
    ///
    /// Existing data remains available in the current collector.
    pub fn disable(&self) {
        self.inner.enabled.store(false, Ordering::Relaxed);
    }

    /// Returns monotonic elapsed time since collector creation.
    pub fn elapsed_ns(&self) -> u64 {
        self.inner
            .start
            .elapsed()
            .as_nanos()
            .min(u64::MAX as u128) as u64
    }

    // =========================================================================
    // Memory accounting
    // =========================================================================

    /// Records a successful allocation.
    pub fn record_allocation(
        &self,
        bytes: u64,
        storage: StorageKind,
        class: AllocationClass,
    ) {
        if !self.is_enabled() {
            return;
        }

        self.inner.allocations.increment();

        self.inner
            .allocated_bytes
            .set(self.inner.allocated_bytes.get().saturating_add(bytes));

        self.inner
            .peak_allocated_bytes
            .observe(self.inner.allocated_bytes.get());

        self.inner
            .storage_bytes
            .increment(storage.as_str(), bytes);

        self.inner
            .allocation_bytes
            .increment(class.as_str(), bytes);

        if let Some(histogram) = &self.inner.allocation_histogram {
            histogram.record(bytes as f64);
        }
    }

    /// Records a release.
    ///
    /// The current allocation gauge is saturating and therefore cannot become
    /// negative even if telemetry receives an inconsistent release event.
    pub fn record_release(
        &self,
        bytes: u64,
        storage: StorageKind,
        class: AllocationClass,
    ) {
        if !self.is_enabled() {
            return;
        }

        self.inner.releases.increment();

        let current = self.inner.allocated_bytes.get();
        self.inner
            .allocated_bytes
            .set(current.saturating_sub(bytes));

        self.inner
            .storage_bytes
            .increment(storage.as_str(), 0);

        self.inner
            .allocation_bytes
            .increment(class.as_str(), 0);

        if let Some(histogram) = &self.inner.release_histogram {
            histogram.record(bytes as f64);
        }
    }

    /// Records reserved memory.
    pub fn record_reservation(&self, bytes: u64) {
        if !self.is_enabled() {
            return;
        }

        self.inner
            .reserved_bytes
            .set(self.inner.reserved_bytes.get().saturating_add(bytes));

        self.inner
            .peak_reserved_bytes
            .observe(self.inner.reserved_bytes.get());
    }

    /// Records released reservation.
    pub fn record_reservation_release(&self, bytes: u64) {
        if !self.is_enabled() {
            return;
        }

        let current = self.inner.reserved_bytes.get();

        self.inner
            .reserved_bytes
            .set(current.saturating_sub(bytes));
    }

    /// Records an allocation failure.
    pub fn record_allocation_failure(&self) {
        if self.is_enabled() {
            self.inner.allocation_failures.increment();
        }
    }

    /// Records a budget violation.
    pub fn record_budget_violation(&self) {
        if self.is_enabled() {
            self.inner.budget_violations.increment();
        }
    }

    /// Records a memory-limit violation.
    pub fn record_limit_violation(&self) {
        if self.is_enabled() {
            self.inner.limit_violations.increment();
        }
    }

    // =========================================================================
    // Pool
    // =========================================================================

    /// Records a pool acquisition.
    pub fn record_pool_acquire(&self) {
        if self.is_enabled() {
            self.inner.pool_acquires.increment();
        }
    }

    /// Records a pool release.
    pub fn record_pool_release(&self) {
        if self.is_enabled() {
            self.inner.pool_releases.increment();
        }
    }

    /// Records a pool miss.
    pub fn record_pool_miss(&self) {
        if self.is_enabled() {
            self.inner.pool_misses.increment();
        }
    }

    // =========================================================================
    // State representation
    // =========================================================================

    /// Records memory associated with a quantum-state representation.
    ///
    /// `representation` may be any current or future representation:
    ///
    /// - state_vector;
    /// - density_matrix;
    /// - stabilizer;
    /// - sparse;
    /// - mps;
    /// - backend_native;
    /// - photonic;
    /// - continuous_variable;
    /// - annealing;
    /// - custom.
    pub fn record_state_memory(
        &self,
        representation: impl AsRef<str>,
        bytes: u64,
        qubits: u64,
    ) {
        if !self.is_enabled() {
            return;
        }

        let representation = RepresentationLabel::new(representation);

        self.inner
            .representation_bytes
            .increment(representation.as_str(), bytes);

        self.inner
            .representation_instances
            .increment(representation.as_str(), 1);

        self.inner
            .representation_qubits
            .increment(representation.as_str(), qubits);

        if let Some(histogram) = &self.inner.state_qubit_histogram {
            histogram.record(qubits as f64);
        }
    }

    /// Records a provider/backend memory observation.
    ///
    /// This does not imply that the provider is a simulator. For a real QPU,
    /// the backend adapter may report provider-managed memory or queue/resource
    /// metrics without exposing quantum state.
    pub fn record_provider_memory(
        &self,
        provider: impl AsRef<str>,
        bytes: u64,
    ) {
        if !self.is_enabled() {
            return;
        }

        let provider = ProviderLabel::new(provider);

        self.inner
            .provider_bytes
            .increment(provider.as_str(), bytes);

        self.inner
            .provider_events
            .increment(provider.as_str(), 1);
    }

    /// Records a provider/backend event without fabricating memory usage.
    pub fn record_provider_event(&self, provider: impl AsRef<str>) {
        if self.is_enabled() {
            let provider = ProviderLabel::new(provider);

            self.inner
                .provider_events
                .increment(provider.as_str(), 1);
        }
    }

    // =========================================================================
    // Migration
    // =========================================================================

    /// Records state/memory migration.
    pub fn record_migration(&self, bytes: u64) {
        if !self.is_enabled() {
            return;
        }

        self.inner.migrations.increment();

        if let Some(histogram) = &self.inner.migration_histogram {
            histogram.record(bytes as f64);
        }
    }

    // =========================================================================
    // Synchronization
    // =========================================================================

    /// Records synchronization between memory domains.
    pub fn record_synchronization(&self, bytes: u64) {
        if !self.is_enabled() {
            return;
        }

        self.inner.synchronizations.increment();

        if let Some(histogram) = &self.inner.synchronization_histogram {
            histogram.record(bytes as f64);
        }
    }

    // =========================================================================
    // Compaction
    // =========================================================================

    /// Records compaction.
    pub fn record_compaction(&self) {
        if self.is_enabled() {
            self.inner.compactions.increment();
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
    // Operation timing
    // =========================================================================

    /// Records an operation duration in nanoseconds.
    pub fn record_operation_duration_ns(&self, duration_ns: u64) {
        if !self.is_enabled() || !self.inner.config.record_histograms {
            return;
        }

        if let Some(histogram) = &self.inner.operation_duration_histogram {
            histogram.record(duration_ns as f64);
        }
    }

    /// Starts a monotonic operation timer.
    ///
    /// Dropping the returned timer does not automatically record anything.
    /// Call `finish()` explicitly so recording is deterministic.
    pub fn start_timer(&self) -> TelemetryTimer {
        TelemetryTimer {
            telemetry: self.clone(),
            started: Instant::now(),
        }
    }

    // =========================================================================
    // Generic hardware
    // =========================================================================

    /// Records a generic hardware/provider observation.
    ///
    /// This is the primary extension point for future QPU families and
    /// accelerators.
    ///
    /// Examples:
    ///
    /// ```text
    /// temperature_celsius
    /// power_watts
    /// queue_depth
    /// memory_utilization_ratio
    /// device_utilization_ratio
    /// available_memory_bytes
    /// memory_bandwidth_bytes_per_second
    /// calibration_age_seconds
    /// ```
    ///
    /// The telemetry layer does not interpret vendor semantics.
    pub fn record_hardware_observation(
        &self,
        provider: impl AsRef<str>,
        device: impl AsRef<str>,
        metric: impl AsRef<str>,
        value: f64,
        unit: impl AsRef<str>,
    ) {
        if !self.is_enabled() || !value.is_finite() {
            return;
        }

        let provider = ProviderLabel::new(provider);
        let device = DeviceLabel::new(device);
        let metric = MetricLabel::new(metric);

        let key = format!(
            "{}|{}|{}",
            provider.as_str(),
            device.as_str(),
            metric.as_str()
        );

        let mut values = match self.inner.hardware_values.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };

        if !values.contains_key(&key)
            && values.len() >= self.inner.config.max_hardware_metrics.max(1)
        {
            return;
        }

        values.insert(
            key,
            (
                value,
                sanitize_label(unit.as_ref()),
            ),
        );
    }

    /// Records a complete generic hardware observation object.
    pub fn record_hardware(&self, observation: HardwareObservation) {
        self.record_hardware_observation(
            observation.provider.as_str(),
            observation.device.as_str(),
            observation.metric.as_str(),
            observation.value,
            observation.unit,
        );
    }

    // =========================================================================
    // Snapshot
    // =========================================================================

    /// Produces a deterministic immutable telemetry snapshot.
    pub fn snapshot(&self) -> TelemetrySnapshot {
        let hardware_values = match self.inner.hardware_values.lock() {
            Ok(guard) => guard.clone(),
            Err(poisoned) => poisoned.into_inner().clone(),
        };

        let mut hardware = HardwareSnapshot::default();

        for (key, (value, unit)) in hardware_values {
            hardware.values.insert(key.clone(), value);
            hardware.units.insert(key, unit);
        }

        TelemetrySnapshot {
            schema_id: String::from(TELEMETRY_SCHEMA_ID),
            schema_version: TELEMETRY_SCHEMA_VERSION,
            elapsed_ns: self.elapsed_ns(),
            enabled: self.is_enabled(),

            memory: MemoryUsageSnapshot {
                allocated_bytes: self.inner.allocated_bytes.get(),
                reserved_bytes: self.inner.reserved_bytes.get(),
                peak_allocated_bytes: self.inner.peak_allocated_bytes.get(),
                peak_reserved_bytes: self.inner.peak_reserved_bytes.get(),
            },

            events: EventSnapshot {
                allocations: self.inner.allocations.get(),
                releases: self.inner.releases.get(),
                allocation_failures: self.inner.allocation_failures.get(),
                budget_violations: self.inner.budget_violations.get(),
                limit_violations: self.inner.limit_violations.get(),
                pool_acquires: self.inner.pool_acquires.get(),
                pool_releases: self.inner.pool_releases.get(),
                pool_misses: self.inner.pool_misses.get(),
                migrations: self.inner.migrations.get(),
                synchronizations: self.inner.synchronizations.get(),
                compactions: self.inner.compactions.get(),
                cache_hits: self.inner.cache_hits.get(),
                cache_misses: self.inner.cache_misses.get(),
            },

            storage: StorageSnapshot {
                bytes: self.inner.storage_bytes.snapshot(),
            },

            allocations: AllocationSnapshot {
                bytes: self.inner.allocation_bytes.snapshot(),
            },

            representations: RepresentationSnapshot {
                bytes: self.inner.representation_bytes.snapshot(),
                instances: self.inner.representation_instances.snapshot(),
                qubits: self.inner.representation_qubits.snapshot(),
            },

            providers: ProviderSnapshot {
                bytes: self.inner.provider_bytes.snapshot(),
                events: self.inner.provider_events.snapshot(),
            },

            hardware,

            histograms: HistogramCollectionSnapshot {
                allocation_bytes: self
                    .inner
                    .allocation_histogram
                    .as_ref()
                    .map(Histogram::snapshot),

                release_bytes: self
                    .inner
                    .release_histogram
                    .as_ref()
                    .map(Histogram::snapshot),

                migration_bytes: self
                    .inner
                    .migration_histogram
                    .as_ref()
                    .map(Histogram::snapshot),

                synchronization_bytes: self
                    .inner
                    .synchronization_histogram
                    .as_ref()
                    .map(Histogram::snapshot),

                operation_duration_ns: self
                    .inner
                    .operation_duration_histogram
                    .as_ref()
                    .map(Histogram::snapshot),

                state_qubits: self
                    .inner
                    .state_qubit_histogram
                    .as_ref()
                    .map(Histogram::snapshot),
            },
        }
    }

    /// Returns current allocated bytes.
    pub fn allocated_bytes(&self) -> u64 {
        self.inner.allocated_bytes.get()
    }

    /// Returns current reserved bytes.
    pub fn reserved_bytes(&self) -> u64 {
        self.inner.reserved_bytes.get()
    }

    /// Returns peak allocated bytes.
    pub fn peak_allocated_bytes(&self) -> u64 {
        self.inner.peak_allocated_bytes.get()
    }

    /// Returns peak reserved bytes.
    pub fn peak_reserved_bytes(&self) -> u64 {
        self.inner.peak_reserved_bytes.get()
    }

    /// Resets the collector by replacing all accumulated state with a fresh
    /// collector.
    ///
    /// The method returns the replacement collector rather than mutating the
    /// existing shared collector. This avoids races between readers and
    /// writers and keeps snapshots internally coherent.
    pub fn replacement(&self) -> Self {
        Self::new(self.inner.config.clone())
    }
}

// =============================================================================
// Timer
// =============================================================================

/// Explicit operation timer.
#[derive(Debug)]
pub struct TelemetryTimer {
    telemetry: MemoryTelemetry,
    started: Instant,
}

impl TelemetryTimer {
    /// Completes the timer and records its duration.
    pub fn finish(self) -> u64 {
        let duration_ns = self
            .started
            .elapsed()
            .as_nanos()
            .min(u64::MAX as u128) as u64;

        self.telemetry.record_operation_duration_ns(duration_ns);

        duration_ns
    }
}

// =============================================================================
// Export sink
// =============================================================================

/// Export boundary for telemetry.
///
/// This keeps `memory/telemetry.rs` independent from:
///
/// - OpenTelemetry;
/// - Prometheus;
/// - OTLP;
/// - logging;
/// - files;
/// - network exporters.
///
/// An application can implement this trait in another module without changing
/// the telemetry collector.
pub trait TelemetrySink: Send + Sync {
    /// Consumes one immutable snapshot.
    fn record(&self, snapshot: &TelemetrySnapshot);
}

impl MemoryTelemetry {
    /// Exports one immutable snapshot to a caller-provided sink.
    ///
    /// The sink is outside the quantum-memory subsystem and therefore owns
    /// transport, retries, authentication and persistence.
    pub fn export<S>(&self, sink: &S)
    where
        S: TelemetrySink,
    {
        let snapshot = self.snapshot();
        sink.record(&snapshot);
    }
}

// =============================================================================
// Stable metric names
// =============================================================================

/// Stable metric-name constants for external exporters.
///
/// These are semantic names, not exporter-specific names.
pub mod metric_names {
    /// Current allocated bytes.
    pub const MEMORY_ALLOCATED_BYTES: &str =
        "zamani.quantum.memory.allocated_bytes";

    /// Current reserved bytes.
    pub const MEMORY_RESERVED_BYTES: &str =
        "zamani.quantum.memory.reserved_bytes";

    /// Peak allocated bytes.
    pub const MEMORY_PEAK_ALLOCATED_BYTES: &str =
        "zamani.quantum.memory.peak_allocated_bytes";

    /// Peak reserved bytes.
    pub const MEMORY_PEAK_RESERVED_BYTES: &str =
        "zamani.quantum.memory.peak_reserved_bytes";

    /// Allocation count.
    pub const ALLOCATIONS_TOTAL: &str =
        "zamani.quantum.memory.allocations_total";

    /// Release count.
    pub const RELEASES_TOTAL: &str =
        "zamani.quantum.memory.releases_total";

    /// Allocation failure count.
    pub const ALLOCATION_FAILURES_TOTAL: &str =
        "zamani.quantum.memory.allocation_failures_total";

    /// Budget violation count.
    pub const BUDGET_VIOLATIONS_TOTAL: &str =
        "zamani.quantum.memory.budget_violations_total";

    /// Limit violation count.
    pub const LIMIT_VIOLATIONS_TOTAL: &str =
        "zamani.quantum.memory.limit_violations_total";

    /// Migration count.
    pub const MIGRATIONS_TOTAL: &str =
        "zamani.quantum.memory.migrations_total";

    /// Synchronization count.
    pub const SYNCHRONIZATIONS_TOTAL: &str =
        "zamani.quantum.memory.synchronizations_total";

    /// Compaction count.
    pub const COMPACTIONS_TOTAL: &str =
        "zamani.quantum.memory.compactions_total";

    /// Cache hit count.
    pub const CACHE_HITS_TOTAL: &str =
        "zamani.quantum.memory.cache_hits_total";

    /// Cache miss count.
    pub const CACHE_MISSES_TOTAL: &str =
        "zamani.quantum.memory.cache_misses_total";

    /// Allocation size histogram.
    pub const ALLOCATION_SIZE_BYTES: &str =
        "zamani.quantum.memory.allocation_size_bytes";

    /// Operation duration histogram.
    pub const OPERATION_DURATION_NS: &str =
        "zamani.quantum.memory.operation_duration_ns";

    /// State qubit-count histogram.
    pub const STATE_QUBITS: &str =
        "zamani.quantum.memory.state_qubits";
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_collector_starts_empty() {
        let telemetry = MemoryTelemetry::production();

        assert_eq!(telemetry.allocated_bytes(), 0);
        assert_eq!(telemetry.reserved_bytes(), 0);
        assert_eq!(telemetry.peak_allocated_bytes(), 0);
        assert_eq!(telemetry.peak_reserved_bytes(), 0);
    }

    #[test]
    fn allocation_and_release_are_accounted() {
        let telemetry = MemoryTelemetry::production();

        telemetry.record_allocation(
            1_024,
            StorageKind::Host,
            AllocationClass::State,
        );

        assert_eq!(telemetry.allocated_bytes(), 1_024);
        assert_eq!(telemetry.peak_allocated_bytes(), 1_024);

        telemetry.record_release(
            512,
            StorageKind::Host,
            AllocationClass::State,
        );

        assert_eq!(telemetry.allocated_bytes(), 512);
        assert_eq!(telemetry.peak_allocated_bytes(), 1_024);
    }

    #[test]
    fn reservation_peak_is_retained() {
        let telemetry = MemoryTelemetry::production();

        telemetry.record_reservation(4_096);
        telemetry.record_reservation_release(2_048);

        assert_eq!(telemetry.reserved_bytes(), 2_048);
        assert_eq!(telemetry.peak_reserved_bytes(), 4_096);
    }

    #[test]
    fn state_representation_is_extensible() {
        let telemetry = MemoryTelemetry::production();

        telemetry.record_state_memory(
            "future_quantum_representation",
            8_192,
            32,
        );

        let snapshot = telemetry.snapshot();

        assert_eq!(
            snapshot
                .representations
                .bytes
                .get("future_quantum_representation"),
            Some(&8_192)
        );
    }

    #[test]
    fn provider_names_are_generic() {
        let telemetry = MemoryTelemetry::production();

        telemetry.record_provider_memory(
            "future-qpu-provider",
            4_096,
        );

        let snapshot = telemetry.snapshot();

        assert_eq!(
            snapshot.providers.bytes.get("future-qpu-provider"),
            Some(&4_096)
        );
    }

    #[test]
    fn hardware_observation_is_recorded() {
        let telemetry = MemoryTelemetry::production();

        telemetry.record_hardware_observation(
            "provider-a",
            "device-a",
            "temperature_celsius",
            42.5,
            "degC",
        );

        let snapshot = telemetry.snapshot();

        assert_eq!(
            snapshot
                .hardware
                .values
                .get("provider-a|device-a|temperature_celsius"),
            Some(&42.5)
        );
    }

    #[test]
    fn non_finite_hardware_values_are_ignored() {
        let telemetry = MemoryTelemetry::production();

        telemetry.record_hardware_observation(
            "provider-a",
            "device-a",
            "invalid",
            f64::NAN,
            "unknown",
        );

        let snapshot = telemetry.snapshot();

        assert!(snapshot.hardware.values.is_empty());
    }

    #[test]
    fn timer_records_duration() {
        let telemetry = MemoryTelemetry::production();

        let timer = telemetry.start_timer();
        let duration = timer.finish();

        assert!(duration < u64::MAX);

        let snapshot = telemetry.snapshot();

        let histogram = snapshot
            .histograms
            .operation_duration_ns
            .expect("duration histogram enabled");

        assert_eq!(histogram.count, 1);
    }

    #[test]
    fn histogram_ignores_invalid_values() {
        let histogram = Histogram::new(vec![1.0, 10.0, 100.0]);

        histogram.record(5.0);
        histogram.record(f64::NAN);
        histogram.record(f64::INFINITY);
        histogram.record(-1.0);

        let snapshot = histogram.snapshot();

        assert_eq!(snapshot.count, 1);
        assert_eq!(snapshot.min, Some(5.0));
        assert_eq!(snapshot.max, Some(5.0));
    }

    #[test]
    fn snapshots_are_deterministic_in_key_order() {
        let telemetry = MemoryTelemetry::production();

        telemetry.record_state_memory("z", 100, 2);
        telemetry.record_state_memory("a", 200, 4);

        let snapshot = telemetry.snapshot();

        let keys = snapshot
            .representations
            .bytes
            .keys()
            .cloned()
            .collect::<Vec<_>>();

        assert_eq!(
            keys,
            vec![
                String::from("a"),
                String::from("z")
            ]
        );
    }

    #[test]
    fn disabled_telemetry_does_not_change_counters() {
        let telemetry = MemoryTelemetry::production();

        telemetry.disable();

        telemetry.record_allocation(
            1_024,
            StorageKind::Host,
            AllocationClass::State,
        );

        assert_eq!(telemetry.allocated_bytes(), 0);
    }

    #[test]
    fn telemetry_can_be_cloned() {
        let telemetry = MemoryTelemetry::production();
        let clone = telemetry.clone();

        clone.record_allocation(
            512,
            StorageKind::Host,
            AllocationClass::Temporary,
        );

        assert_eq!(telemetry.allocated_bytes(), 512);
    }

    #[test]
    fn release_cannot_underflow() {
        let telemetry = MemoryTelemetry::production();

        telemetry.record_release(
            10_000,
            StorageKind::Host,
            AllocationClass::Temporary,
        );

        assert_eq!(telemetry.allocated_bytes(), 0);
    }

    #[test]
    fn labels_are_bounded_and_sanitized() {
        let label = sanitize_label("  hello\nworld  ");

        assert_eq!(label, "hello_world");
        assert!(label.len() <= MAX_LABEL_LENGTH);
    }
}