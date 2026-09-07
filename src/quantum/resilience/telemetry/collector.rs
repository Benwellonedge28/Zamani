//! Zamani Quantum Resilience — Telemetry Collector
//!
//! Path:
//!     src/quantum/resilience/telemetry/collector.rs
//!
//! # Purpose
//!
//! This module defines the provider-neutral collection boundary for
//! `quantum::resilience::telemetry`.
//!
//! The collector accepts already-observed telemetry from authoritative
//! producers and converts it into the canonical telemetry contracts used by
//! the resilience subsystem.
//!
//! The collector is deliberately an ingestion/orchestration boundary.
//! It does not own hardware access, runtime execution, QEC algorithms,
//! anomaly detection, diagnosis, recovery, persistence, or exporting.
//!
//! Architectural flow:
//!
//! ```text
//! hardware / runtime / compiler / routing / scheduling / optimization
//! / QEC / mitigation / simulation / benchmarking / application
//!                         │
//!                         ▼
//!                 CollectorSource
//!                         │
//!                         ▼
//!                 CollectorInput
//!                         │
//!                         ▼
//!                TelemetryCollector
//!                         │
//!              ┌──────────┴──────────┐
//!              ▼                     ▼
//!       TelemetryEvent             Metric
//!              │                     │
//!              └──────────┬──────────┘
//!                         ▼
//!              telemetry consumers
//!              detection / history / exporter
//! ```
//!
//! # Architectural rules
//!
//! The collector MUST:
//!
//! - accept explicit observations;
//! - preserve producer identity;
//! - preserve source kind;
//! - preserve timestamps supplied by producers;
//! - preserve producer sequence information;
//! - preserve causal/correlation/trace information;
//! - preserve canonical quantum identities;
//! - avoid inventing hardware limits;
//! - avoid inventing event or metric semantics;
//! - remain deterministic for deterministic input;
//! - avoid global mutable state;
//! - support bounded and streaming collection;
//! - permit deployment-specific backpressure;
//! - expose explicit collection statistics;
//! - distinguish accepted, rejected, duplicated, and overflowed input;
//! - make overflow policy explicit;
//! - remain independent of exporter implementations;
//! - remain independent of persistence implementations;
//! - remain independent of detector implementations;
//! - remain independent of hardware providers.
//!
//! The collector MUST NOT:
//!
//! - read the system clock;
//! - generate timestamps;
//! - generate UUIDs;
//! - generate random identifiers;
//! - access hardware directly;
//! - access the network;
//! - access the filesystem;
//! - execute quantum operations;
//! - mutate quantum state;
//! - perform QEC;
//! - perform error mitigation;
//! - route circuits;
//! - schedule circuits;
//! - optimize circuits;
//! - diagnose faults;
//! - select recovery strategies;
//! - execute recovery;
//! - silently discard telemetry;
//! - silently truncate telemetry;
//! - introduce provider-specific semantics into the canonical model;
//! - introduce a resilience-local qubit identifier.
//!
//! # Write once, scale everywhere
//!
//! There is intentionally no universal:
//!
//! - maximum number of events;
//! - maximum number of metrics;
//! - maximum number of sources;
//! - maximum number of qubits;
//! - maximum number of devices;
//! - maximum batch size;
//! - maximum queue size;
//! - maximum telemetry rate.
//!
//! Concrete limits belong to the caller/deployment through
//! `quantum::resilience::limits`, memory policy, security policy,
//! storage policy, exporter capacity, or runtime configuration.
//!
//! A collector instance may therefore be used for one observation or for a
//! very large streaming workload, subject only to the finite resources of the
//! deployment.
//!
//! "Infinity" in the Zamani architectural requirement means that no artificial
//! machine-size constant is embedded in this module. Every concrete process
//! remains naturally bounded by available memory, CPU, storage, transport and
//! configured policy.
//!
//! # Canonical quantum identities
//!
//! This module does not define quantum IDs.
//!
//! If an input needs to identify a logical or physical qubit, the input must
//! use the canonical types from:
//!
//!     crate::quantum::ir::qubit
//!
//! In particular:
//!
//!     QubitId
//!     PhysicalQubitId
//!
//! Mapping between those identities remains owned by
//! `quantum::routing` and the hardware abstraction.
//!
//! # Determinism
//!
//! The collector performs no implicit environmental observation.
//!
//! Given identical:
//!
//! - collector configuration;
//! - input sequence;
//! - source identity;
//! - timestamps;
//! - producer sequence numbers;
//! - event/metric identities;
//! - collection state;
//!
//! collection produces identical decisions and statistics.
//!
//! This is required for deterministic replay and resilience verification.
//!
//! # Ordering
//!
//! The collector does not impose a global chronological ordering on
//! independently produced telemetry.
//!
//! Timestamps alone are insufficient to establish total ordering.
//!
//! Producer sequence numbers, causal relationships, correlation identifiers,
//! and trace identifiers remain part of the canonical telemetry contracts.
//!
//! The collector therefore preserves ordering information rather than
//! inventing ordering semantics.
//!
//! # Backpressure
//!
//! Backpressure is explicit.
//!
//! This module provides a bounded in-memory collector but does not prescribe
//! the deployment's transport/storage policy.
//!
//! When capacity is reached, the configured overflow policy determines whether
//! an input is:
//!
//! - rejected;
//! - reported as overflow;
//! - replaced according to an explicit policy.
//!
//! The default policy is rejection because silent telemetry loss can cause
//! incorrect resilience decisions.
//!
//! # Thread safety
//!
//! The collector uses ordinary Rust ownership and can be wrapped in
//! synchronization primitives by callers that need shared concurrent access.
//!
//! No global singleton is created here.
//!
//! `TelemetryCollector` itself is intentionally not internally synchronized.
//! This keeps ownership explicit and avoids hidden contention.
//!
//! # Integration
//!
//! `telemetry/mod.rs` should expose:
//!
//! ```text
//! pub mod collector;
//! ```
//!
//! The collector consumes the canonical types from:
//!
//! ```text
//! crate::quantum::resilience::telemetry::event
//! crate::quantum::resilience::telemetry::metric
//! ```
//!
//! Downstream consumers should consume the resulting immutable records.
//!
//! Detection consumes telemetry as evidence.
//!
//! Diagnosis interprets evidence.
//!
//! History decides retention.
//!
//! Exporters decide external representation.
//!
//! The collector does not know those implementations.
//!
//! # Rust contract
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021 edition
//! - stable Rust
//! - no nightly features
//! - no unsafe code
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]
#![deny(rust_2018_idioms)]

use std::collections::{BTreeMap, VecDeque};
use std::fmt;
use std::sync::Arc;

use super::event::TelemetryEvent;
use super::metric::Metric;

/// Provider-neutral collection overflow behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OverflowPolicy {
    /// Reject the incoming observation and preserve existing buffered data.
    Reject,

    /// Remove the oldest buffered observation before accepting the new one.
    ///
    /// This is appropriate only when the deployment explicitly accepts
    /// loss of oldest buffered telemetry.
    DropOldest,

    /// Remove the newest buffered observation before accepting the new one.
    ///
    /// This is appropriate only for workloads whose newest observations are
    /// intentionally lower priority than the existing buffer.
    DropNewest,
}

impl Default for OverflowPolicy {
    fn default() -> Self {
        Self::Reject
    }
}

impl fmt::Display for OverflowPolicy {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reject => formatter.write_str("reject"),
            Self::DropOldest => formatter.write_str("drop_oldest"),
            Self::DropNewest => formatter.write_str("drop_newest"),
        }
    }
}

/// Collector capacity configuration.
///
/// Capacity is an operational limit, not a universal Zamani hardware limit.
///
/// `None` means the collector does not impose an in-memory queue limit.
/// Deployments using `None` must still ensure that their surrounding runtime
/// and transport provide an appropriate resource policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CollectorCapacity {
    events: Option<usize>,
    metrics: Option<usize>,
}

impl CollectorCapacity {
    /// Creates a capacity configuration.
    ///
    /// A zero capacity is valid and means that the corresponding queue cannot
    /// buffer observations.
    #[must_use]
    pub const fn new(events: Option<usize>, metrics: Option<usize>) -> Self {
        Self { events, metrics }
    }

    /// Creates an unbounded-in-architecture capacity configuration.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            events: None,
            metrics: None,
        }
    }

    /// Returns the event capacity.
    #[must_use]
    pub const fn events(self) -> Option<usize> {
        self.events
    }

    /// Returns the metric capacity.
    #[must_use]
    pub const fn metrics(self) -> Option<usize> {
        self.metrics
    }
}

impl Default for CollectorCapacity {
    fn default() -> Self {
        Self::unbounded()
    }
}

/// Collector behavior configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CollectorConfig {
    capacity: CollectorCapacity,
    overflow_policy: OverflowPolicy,
    reject_duplicates: bool,
}

impl CollectorConfig {
    /// Creates an explicit collector configuration.
    #[must_use]
    pub const fn new(
        capacity: CollectorCapacity,
        overflow_policy: OverflowPolicy,
        reject_duplicates: bool,
    ) -> Self {
        Self {
            capacity,
            overflow_policy,
            reject_duplicates,
        }
    }

    /// Returns the configured capacity.
    #[must_use]
    pub const fn capacity(self) -> CollectorCapacity {
        self.capacity
    }

    /// Returns the overflow policy.
    #[must_use]
    pub const fn overflow_policy(self) -> OverflowPolicy {
        self.overflow_policy
    }

    /// Returns whether duplicate identities are rejected.
    #[must_use]
    pub const fn reject_duplicates(self) -> bool {
        self.reject_duplicates
    }
}

impl Default for CollectorConfig {
    fn default() -> Self {
        Self {
            capacity: CollectorCapacity::unbounded(),
            overflow_policy: OverflowPolicy::Reject,
            reject_duplicates: true,
        }
    }
}

/// Identifies the type of telemetry accepted by the collector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TelemetryKind {
    /// A lifecycle or observation event.
    Event,

    /// A quantitative metric.
    Metric,
}

impl fmt::Display for TelemetryKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Event => formatter.write_str("event"),
            Self::Metric => formatter.write_str("metric"),
        }
    }
}

/// A collected telemetry record.
///
/// This enum intentionally contains the canonical event and metric types
/// rather than duplicate representations.
#[derive(Debug, Clone)]
pub enum CollectedTelemetry {
    /// Canonical resilience telemetry event.
    Event(TelemetryEvent),

    /// Canonical resilience telemetry metric.
    Metric(Metric),
}

impl CollectedTelemetry {
    /// Returns the telemetry kind.
    #[must_use]
    pub const fn kind(&self) -> TelemetryKind {
        match self {
            Self::Event(_) => TelemetryKind::Event,
            Self::Metric(_) => TelemetryKind::Metric,
        }
    }
}

/// Input supplied to a collector.
///
/// The producer constructs the canonical event/metric object and hands it to
/// the collection boundary. The collector never manufactures semantic
/// telemetry.
#[derive(Debug, Clone)]
pub enum CollectorInput {
    /// An event to collect.
    Event(TelemetryEvent),

    /// A metric to collect.
    Metric(Metric),
}

impl CollectorInput {
    /// Returns the telemetry kind.
    #[must_use]
    pub const fn kind(&self) -> TelemetryKind {
        match self {
            Self::Event(_) => TelemetryKind::Event,
            Self::Metric(_) => TelemetryKind::Metric,
        }
    }

    /// Converts the input into an owned collected record.
    #[must_use]
    pub fn into_collected(self) -> CollectedTelemetry {
        match self {
            Self::Event(event) => CollectedTelemetry::Event(event),
            Self::Metric(metric) => CollectedTelemetry::Metric(metric),
        }
    }
}

/// Result of a collection operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CollectionOutcome {
    /// The observation was accepted.
    Accepted,

    /// The observation was rejected because an equivalent identity was
    /// already present and duplicate rejection is enabled.
    DuplicateRejected,

    /// The observation was rejected because the corresponding buffer was full.
    CapacityRejected,

    /// The observation was accepted after dropping the oldest buffered
    /// observation according to explicit policy.
    AcceptedAfterDropOldest,

    /// The incoming observation was intentionally discarded by
    /// `DropNewest`.
    DroppedNewest,
}

impl CollectionOutcome {
    /// Returns whether the input became buffered.
    #[must_use]
    pub const fn accepted(self) -> bool {
        matches!(
            self,
            Self::Accepted | Self::AcceptedAfterDropOldest
        )
    }

    /// Returns whether the outcome discarded telemetry.
    #[must_use]
    pub const fn discarded(self) -> bool {
        matches!(
            self,
            Self::DuplicateRejected
                | Self::CapacityRejected
                | Self::DroppedNewest
        )
    }
}

/// Stable collector statistics.
///
/// Counters are monotonically increasing for the lifetime of one collector.
/// They are local to the collector and are not global system counters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CollectorStatistics {
    accepted_events: u64,
    accepted_metrics: u64,
    rejected_duplicates: u64,
    rejected_capacity: u64,
    dropped_oldest: u64,
    dropped_newest: u64,
}

impl CollectorStatistics {
    /// Number of accepted events.
    #[must_use]
    pub const fn accepted_events(self) -> u64 {
        self.accepted_events
    }

    /// Number of accepted metrics.
    #[must_use]
    pub const fn accepted_metrics(self) -> u64 {
        self.accepted_metrics
    }

    /// Number of duplicate observations rejected.
    #[must_use]
    pub const fn rejected_duplicates(self) -> u64 {
        self.rejected_duplicates
    }

    /// Number of capacity rejections.
    #[must_use]
    pub const fn rejected_capacity(self) -> u64 {
        self.rejected_capacity
    }

    /// Number of observations removed from the front of a buffer.
    #[must_use]
    pub const fn dropped_oldest(self) -> u64 {
        self.dropped_oldest
    }

    /// Number of newest observations intentionally discarded.
    #[must_use]
    pub const fn dropped_newest(self) -> u64 {
        self.dropped_newest
    }

    /// Total accepted records.
    #[must_use]
    pub const fn total_accepted(self) -> u64 {
        self.accepted_events
            .saturating_add(self.accepted_metrics)
    }

    /// Total records rejected without being buffered.
    #[must_use]
    pub const fn total_rejected(self) -> u64 {
        self.rejected_duplicates
            .saturating_add(self.rejected_capacity)
    }

    /// Total records explicitly discarded.
    #[must_use]
    pub const fn total_discarded(self) -> u64 {
        self.rejected_duplicates
            .saturating_add(self.rejected_capacity)
            .saturating_add(self.dropped_newest)
    }

    /// Returns whether any telemetry has been lost since creation.
    #[must_use]
    pub const fn has_loss(self) -> bool {
        self.total_discarded() > 0 || self.dropped_oldest > 0
    }
}

/// Collector errors.
///
/// These errors describe collection-boundary failures only. They deliberately
/// do not reproduce the complete resilience error hierarchy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollectorError {
    /// The configured event capacity cannot accept the input.
    EventCapacityExceeded,

    /// The configured metric capacity cannot accept the input.
    MetricCapacityExceeded,

    /// An equivalent event identity already exists in the collector.
    DuplicateEvent,

    /// An equivalent metric identity already exists in the collector.
    DuplicateMetric,

    /// The supplied operation is incompatible with collector state.
    InvalidOperation(Arc<str>),
}

impl fmt::Display for CollectorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EventCapacityExceeded => {
                formatter.write_str("telemetry event collector capacity exceeded")
            }
            Self::MetricCapacityExceeded => {
                formatter.write_str("telemetry metric collector capacity exceeded")
            }
            Self::DuplicateEvent => {
                formatter.write_str("duplicate telemetry event")
            }
            Self::DuplicateMetric => {
                formatter.write_str("duplicate telemetry metric")
            }
            Self::InvalidOperation(message) => {
                write!(formatter, "invalid telemetry collector operation: {message}")
            }
        }
    }
}

impl std::error::Error for CollectorError {}

/// Provider-neutral telemetry collection boundary.
///
/// The collector stores canonical events and metrics without interpreting
/// their quantum semantics.
#[derive(Debug)]
pub struct TelemetryCollector {
    config: CollectorConfig,
    events: VecDeque<TelemetryEvent>,
    metrics: VecDeque<Metric>,
    event_ids: BTreeMap<String, ()>,
    metric_ids: BTreeMap<String, ()>,
    statistics: CollectorStatistics,
}

impl TelemetryCollector {
    /// Creates an empty collector with explicit configuration.
    #[must_use]
    pub fn new(config: CollectorConfig) -> Self {
        Self {
            config,
            events: VecDeque::new(),
            metrics: VecDeque::new(),
            event_ids: BTreeMap::new(),
            metric_ids: BTreeMap::new(),
            statistics: CollectorStatistics::default(),
        }
    }

    /// Creates an empty collector with the default configuration.
    #[must_use]
    pub fn with_defaults() -> Self {
        Self::new(CollectorConfig::default())
    }

    /// Returns the immutable configuration.
    #[must_use]
    pub const fn config(&self) -> CollectorConfig {
        self.config
    }

    /// Returns current collector statistics.
    #[must_use]
    pub const fn statistics(&self) -> CollectorStatistics {
        self.statistics
    }

    /// Returns the number of buffered events.
    #[must_use]
    pub fn event_len(&self) -> usize {
        self.events.len()
    }

    /// Returns the number of buffered metrics.
    #[must_use]
    pub fn metric_len(&self) -> usize {
        self.metrics.len()
    }

    /// Returns whether no telemetry is currently buffered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.events.is_empty() && self.metrics.is_empty()
    }

    /// Collects one canonical telemetry input.
    ///
    /// This is the primary producer-facing operation.
    ///
    /// The producer remains responsible for constructing a valid
    /// `TelemetryEvent` or `Metric`, including timestamp and identity.
    pub fn collect(
        &mut self,
        input: CollectorInput,
    ) -> Result<CollectionOutcome, CollectorError> {
        match input {
            CollectorInput::Event(event) => self.collect_event(event),
            CollectorInput::Metric(metric) => self.collect_metric(metric),
        }
    }

    /// Collects one canonical event.
    pub fn collect_event(
        &mut self,
        event: TelemetryEvent,
    ) -> Result<CollectionOutcome, CollectorError> {
        let identity = event_identity(&event);

        if self.config.reject_duplicates && self.event_ids.contains_key(identity.as_ref()) {
            self.statistics.rejected_duplicates =
                self.statistics.rejected_duplicates.saturating_add(1);
            return Err(CollectorError::DuplicateEvent);
        }

        if let Some(capacity) = self.config.capacity.events() {
            if self.events.len() >= capacity {
                match self.config.overflow_policy {
                    OverflowPolicy::Reject => {
                        self.statistics.rejected_capacity =
                            self.statistics.rejected_capacity.saturating_add(1);
                        return Err(CollectorError::EventCapacityExceeded);
                    }
                    OverflowPolicy::DropOldest => {
                        if let Some(oldest) = self.events.pop_front() {
                            let old_identity = event_identity(&oldest);
                            self.event_ids.remove(old_identity.as_ref());
                            self.statistics.dropped_oldest =
                                self.statistics.dropped_oldest.saturating_add(1);
                        }
                    }
                    OverflowPolicy::DropNewest => {
                        self.statistics.dropped_newest =
                            self.statistics.dropped_newest.saturating_add(1);
                        return Ok(CollectionOutcome::DroppedNewest);
                    }
                }
            }
        }

        self.event_ids.insert(identity.to_string(), ());
        self.events.push_back(event);
        self.statistics.accepted_events =
            self.statistics.accepted_events.saturating_add(1);

        if self.config.overflow_policy == OverflowPolicy::DropOldest
            && self
                .config
                .capacity
                .events()
                .is_some_and(|capacity| self.events.len() > capacity)
        {
            return Err(CollectorError::InvalidOperation(Arc::from(
                "event buffer exceeded configured capacity after collection",
            )));
        }

        Ok(CollectionOutcome::Accepted)
    }

    /// Collects one canonical metric.
    pub fn collect_metric(
        &mut self,
        metric: Metric,
    ) -> Result<CollectionOutcome, CollectorError> {
        let identity = metric_identity(&metric);

        if self.config.reject_duplicates && self.metric_ids.contains_key(identity.as_ref()) {
            self.statistics.rejected_duplicates =
                self.statistics.rejected_duplicates.saturating_add(1);
            return Err(CollectorError::DuplicateMetric);
        }

        if let Some(capacity) = self.config.capacity.metrics() {
            if self.metrics.len() >= capacity {
                match self.config.overflow_policy {
                    OverflowPolicy::Reject => {
                        self.statistics.rejected_capacity =
                            self.statistics.rejected_capacity.saturating_add(1);
                        return Err(CollectorError::MetricCapacityExceeded);
                    }
                    OverflowPolicy::DropOldest => {
                        if let Some(oldest) = self.metrics.pop_front() {
                            let old_identity = metric_identity(&oldest);
                            self.metric_ids.remove(old_identity.as_ref());
                            self.statistics.dropped_oldest =
                                self.statistics.dropped_oldest.saturating_add(1);
                        }
                    }
                    OverflowPolicy::DropNewest => {
                        self.statistics.dropped_newest =
                            self.statistics.dropped_newest.saturating_add(1);
                        return Ok(CollectionOutcome::DroppedNewest);
                    }
                }
            }
        }

        self.metric_ids.insert(identity.to_string(), ());
        self.metrics.push_back(metric);
        self.statistics.accepted_metrics =
            self.statistics.accepted_metrics.saturating_add(1);

        if self.config.overflow_policy == OverflowPolicy::DropOldest
            && self
                .config
                .capacity
                .metrics()
                .is_some_and(|capacity| self.metrics.len() > capacity)
        {
            return Err(CollectorError::InvalidOperation(Arc::from(
                "metric buffer exceeded configured capacity after collection",
            )));
        }

        Ok(CollectionOutcome::Accepted)
    }

    /// Collects a batch of telemetry.
    ///
    /// Inputs are processed in the exact supplied order.
    ///
    /// Collection continues after individual rejected records. The returned
    /// vector contains one result for each input.
    pub fn collect_batch(
        &mut self,
        inputs: impl IntoIterator<Item = CollectorInput>,
    ) -> Vec<Result<CollectionOutcome, CollectorError>> {
        inputs
            .into_iter()
            .map(|input| self.collect(input))
            .collect()
    }

    /// Removes and returns the oldest buffered event.
    pub fn pop_event(&mut self) -> Option<TelemetryEvent> {
        let event = self.events.pop_front()?;
        let identity = event_identity(&event);
        self.event_ids.remove(identity.as_ref());
        Some(event)
    }

    /// Removes and returns the oldest buffered metric.
    pub fn pop_metric(&mut self) -> Option<Metric> {
        let metric = self.metrics.pop_front()?;
        let identity = metric_identity(&metric);
        self.metric_ids.remove(identity.as_ref());
        Some(metric)
    }

    /// Removes and returns the oldest buffered event or metric according to
    /// explicit queue selection.
    pub fn pop(&mut self, kind: TelemetryKind) -> Option<CollectedTelemetry> {
        match kind {
            TelemetryKind::Event => self.pop_event().map(CollectedTelemetry::Event),
            TelemetryKind::Metric => self.pop_metric().map(CollectedTelemetry::Metric),
        }
    }

    /// Drains up to `limit` events.
    ///
    /// `limit` is a caller-provided operational value and is not a universal
    /// telemetry limit.
    pub fn drain_events(&mut self, limit: usize) -> Vec<TelemetryEvent> {
        let count = limit.min(self.events.len());
        let mut result = Vec::with_capacity(count);

        for _ in 0..count {
            if let Some(event) = self.pop_event() {
                result.push(event);
            }
        }

        result
    }

    /// Drains up to `limit` metrics.
    pub fn drain_metrics(&mut self, limit: usize) -> Vec<Metric> {
        let count = limit.min(self.metrics.len());
        let mut result = Vec::with_capacity(count);

        for _ in 0..count {
            if let Some(metric) = self.pop_metric() {
                result.push(metric);
            }
        }

        result
    }

    /// Removes all buffered telemetry while preserving lifetime statistics.
    pub fn clear(&mut self) {
        self.events.clear();
        self.metrics.clear();
        self.event_ids.clear();
        self.metric_ids.clear();
    }

    /// Returns an iterator over buffered events.
    pub fn events(&self) -> impl Iterator<Item = &TelemetryEvent> {
        self.events.iter()
    }

    /// Returns an iterator over buffered metrics.
    pub fn metrics(&self) -> impl Iterator<Item = &Metric> {
        self.metrics.iter()
    }
}

/// Extracts a stable event identity without interpreting event semantics.
///
/// The canonical event contract owns the actual identity representation.
/// This helper deliberately obtains only the identity required for duplicate
/// protection.
fn event_identity(event: &TelemetryEvent) -> Arc<str> {
    /*
     * Integration note:
     *
     * `TelemetryEvent` already defines the canonical event identity.
     * The collector must use that accessor rather than inventing a second
     * identifier.
     *
     * If the current event implementation exposes a differently named
     * accessor, this helper is the ONLY adapter point that needs adjustment.
     */
    Arc::from(event.id().as_str())
}

/// Extracts a stable metric identity without interpreting metric semantics.
///
/// The canonical metric contract owns the actual metric identity.
fn metric_identity(metric: &Metric) -> Arc<str> {
    /*
     * Integration note:
     *
     * `Metric` already defines the canonical metric identity.
     * The collector must use that accessor rather than inventing a second
     * identifier.
     *
     * If the current metric implementation exposes a differently named
     * accessor, this helper is the ONLY adapter point that needs adjustment.
     */
    Arc::from(metric.id().as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_configuration_is_unbounded_and_rejects_duplicates() {
        let config = CollectorConfig::default();

        assert_eq!(config.capacity(), CollectorCapacity::unbounded());
        assert_eq!(config.overflow_policy(), OverflowPolicy::Reject);
        assert!(config.reject_duplicates());
    }

    #[test]
    fn capacity_configuration_is_preserved() {
        let capacity = CollectorCapacity::new(Some(4), Some(8));
        let config = CollectorConfig::new(
            capacity,
            OverflowPolicy::DropOldest,
            false,
        );

        assert_eq!(config.capacity().events(), Some(4));
        assert_eq!(config.capacity().metrics(), Some(8));
        assert_eq!(config.overflow_policy(), OverflowPolicy::DropOldest);
        assert!(!config.reject_duplicates());
    }

    #[test]
    fn statistics_start_empty() {
        let collector = TelemetryCollector::with_defaults();

        assert_eq!(collector.statistics(), CollectorStatistics::default());
        assert!(collector.is_empty());
        assert_eq!(collector.event_len(), 0);
        assert_eq!(collector.metric_len(), 0);
    }

    #[test]
    fn collection_kind_is_stable() {
        assert_eq!(
            TelemetryKind::Event.to_string(),
            "event"
        );
        assert_eq!(
            TelemetryKind::Metric.to_string(),
            "metric"
        );
    }

    #[test]
    fn overflow_policy_display_is_stable() {
        assert_eq!(
            OverflowPolicy::Reject.to_string(),
            "reject"
        );
        assert_eq!(
            OverflowPolicy::DropOldest.to_string(),
            "drop_oldest"
        );
        assert_eq!(
            OverflowPolicy::DropNewest.to_string(),
            "drop_newest"
        );
    }

    #[test]
    fn collection_outcome_classification_is_stable() {
        assert!(CollectionOutcome::Accepted.accepted());
        assert!(CollectionOutcome::AcceptedAfterDropOldest.accepted());

        assert!(CollectionOutcome::DuplicateRejected.discarded());
        assert!(CollectionOutcome::CapacityRejected.discarded());
        assert!(CollectionOutcome::DroppedNewest.discarded());
    }

    #[test]
    fn statistics_saturate_instead_of_wrapping() {
        let statistics = CollectorStatistics {
            accepted_events: u64::MAX,
            accepted_metrics: u64::MAX,
            rejected_duplicates: u64::MAX,
            rejected_capacity: u64::MAX,
            dropped_oldest: u64::MAX,
            dropped_newest: u64::MAX,
        };

        assert_eq!(statistics.total_accepted(), u64::MAX);
        assert_eq!(statistics.total_rejected(), u64::MAX);
        assert_eq!(statistics.total_discarded(), u64::MAX);
        assert!(statistics.has_loss());
    }
}