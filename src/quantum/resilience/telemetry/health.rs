//! Zamani Quantum Resilience — Health Telemetry
//!
//! Path:
//!     src/quantum/resilience/telemetry/health.rs
//!
//! Purpose:
//!     Provides the observability-facing representation of resource health.
//!
//! Architectural ownership:
//!
//!     quantum::resilience::model::health
//!         |
//!         | canonical health semantics
//!         v
//!     quantum::resilience::telemetry::health
//!         |
//!         +--> telemetry::collector
//!         +--> telemetry::event
//!         +--> telemetry::metric
//!         +--> telemetry::trace
//!         +--> detection
//!         +--> diagnosis
//!         +--> history
//!         +--> verification::provenance
//!         +--> telemetry::exporter
//!
//! This module records health observations. It does NOT:
//!
//! - determine causal faults;
//! - diagnose incidents;
//! - authorize recovery;
//! - perform recovery;
//! - select hardware;
//! - perform routing;
//! - perform scheduling;
//! - perform QEC;
//! - perform mitigation;
//! - execute quantum programs;
//! - communicate with providers;
//! - perform I/O;
//! - own persistence;
//! - own serialization;
//! - define a competing quantum identity model.
//!
//! # Canonical identity
//!
//! Resource identity comes from:
//!
//!     crate::quantum::resilience::model::resource::ResourceIdentity
//!
//! and that model already uses the canonical IR identities:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! Consequently this file MUST NOT introduce:
//!
//!     TelemetryQubitId
//!     HealthQubitId
//!     ObservationQubitId
//!     ResilienceQubitId
//!
//! or any equivalent competing identity.
//!
//! # Health semantic ownership
//!
//! The canonical health state is:
//!
//!     crate::quantum::resilience::model::health::HealthState
//!
//! The canonical condition category is:
//!
//!     crate::quantum::resilience::model::health::HealthConditionKind
//!
//! This module only observes and exposes those values.
//!
//! # Write once, scale everywhere
//!
//! This module contains no fixed limits for:
//!
//! - qubits;
//! - physical qubits;
//! - logical qubits;
//! - devices;
//! - backends;
//! - resources;
//! - observations;
//! - hierarchy depth;
//! - event count.
//!
//! There is deliberately no:
//!
//!     MAX_QUBITS
//!     MAX_RESOURCES
//!     MAX_HEALTH_OBSERVATIONS
//!     MAX_DEVICES
//!
//! Concrete memory, retention, batching, sampling, and transport limits belong
//! to the collector, policy, storage, exporter, or deployment configuration.
//!
//! The health representation itself is constant-size per observation.
//!
//! # Determinism
//!
//! This module:
//!
//! - does not read the system clock;
//! - does not generate timestamps;
//! - does not generate random identifiers;
//! - does not use global mutable state;
//! - does not inspect environment variables;
//! - does not access hardware;
//! - does not access the network;
//! - does not access the filesystem;
//! - does not depend on asynchronous completion order.
//!
//! All temporal and source information is supplied explicitly.
//!
//! Snapshot ordering is deterministic.
//!
//! # Security
//!
//! Health telemetry is evidence, not authority.
//!
//! A `Healthy` observation MUST NOT grant execution permission.
//!
//! A `Quarantined` observation MUST NOT itself perform quarantine.
//!
//! A `Retired` observation MUST NOT itself modify hardware state.
//!
//! Authentication, source trust, authorization, provenance integrity, and
//! admission policy remain owned by higher-level boundaries.
//!
//! # Unknown values
//!
//! Telemetry must distinguish:
//!
//! - known health;
//! - unknown health;
//! - unavailable observation;
//! - stale observation;
//! - invalid observation.
//!
//! This module therefore does not use `Option<HealthState>` as the only
//! representation of observation validity. `Option` cannot distinguish all of
//! those semantic conditions.
//!
//! # Observability principles
//!
//! The repository's observability architecture requires health telemetry to
//! support:
//!
//! - physical resources;
//! - logical resources;
//! - backend resources;
//! - device resources;
//! - regions;
//! - couplings;
//! - control channels;
//! - execution resources;
//! - future resource kinds.
//!
//! Resource identity and scope therefore remain data-driven.
//!
//! # Integration
//!
//! `telemetry::collector` constructs observations.
//!
//! `telemetry::event` may carry references or copies of observation metadata.
//!
//! `telemetry::metric` may derive aggregate health metrics.
//!
//! `telemetry::trace` may correlate health observations with executions.
//!
//! `detection` consumes observations as evidence.
//!
//! `diagnosis` consumes observations as evidence.
//!
//! `history` may persist immutable observations according to its retention
//! policy.
//!
//! `verification::provenance` may reference an immutable health snapshot.
//!
//! `telemetry::exporter` converts observations to an external telemetry
//! representation.
//!
//! Serialization belongs to:
//!
//!     quantum::resilience::serialization
//!
//! This module intentionally does not depend on a serialization format.
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
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]
#![deny(rust_2018_idioms)]

use core::fmt;
use std::sync::Arc;

use crate::quantum::resilience::model::health::{
    HealthConditionKind,
    HealthState,
};
use crate::quantum::resilience::model::resource::{
    ResourceIdentity,
    ResourceKind,
    ResourceScope,
};

// ============================================================================
// Schema identity
// ============================================================================

/// Stable semantic schema identifier for resilience health telemetry.
pub const HEALTH_TELEMETRY_SCHEMA_ID: &str =
    "zamani.quantum.resilience.telemetry.health";

/// Current semantic schema version.
///
/// This is a representation/schema version, not a hardware or resource limit.
pub const HEALTH_TELEMETRY_SCHEMA_VERSION: u32 = 1;

// ============================================================================
// Health observation identity
// ============================================================================

/// Stable producer-supplied identifier for one health observation.
///
/// The telemetry layer does not prescribe UUID, ULID, hash, database sequence,
/// or provider-specific identifier formats.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HealthObservationId(Arc<str>);

impl HealthObservationId {
    /// Creates an observation identifier.
    ///
    /// Empty and whitespace-only identifiers are rejected because they cannot
    /// provide useful correlation.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, HealthTelemetryError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(HealthTelemetryError::EmptyObservationId);
        }

        Ok(Self(value))
    }

    /// Returns the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for HealthObservationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Source identity
// ============================================================================

/// Explicit identity of the component that produced the health observation.
///
/// This is intentionally opaque. Provider-specific identity remains outside
/// the provider-neutral health model.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HealthSourceId(Arc<str>);

impl HealthSourceId {
    /// Creates a source identity.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, HealthTelemetryError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(HealthTelemetryError::EmptySourceId);
        }

        Ok(Self(value))
    }

    /// Returns the source identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for HealthSourceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Time
// ============================================================================

/// Explicit health-observation timestamp.
///
/// The health telemetry layer does not read a clock. Producers provide the
/// timestamp.
///
/// Nanoseconds are normalized to `[0, 1_000_000_000)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HealthTimestamp {
    unix_seconds: i64,
    nanoseconds: u32,
}

impl HealthTimestamp {
    /// Number of nanoseconds in one second.
    pub const NANOS_PER_SECOND: u32 = 1_000_000_000;

    /// Creates an explicit timestamp.
    pub const fn new(
        unix_seconds: i64,
        nanoseconds: u32,
    ) -> Result<Self, HealthTelemetryError> {
        if nanoseconds >= Self::NANOS_PER_SECOND {
            return Err(HealthTelemetryError::InvalidNanoseconds);
        }

        Ok(Self {
            unix_seconds,
            nanoseconds,
        })
    }

    /// Creates a timestamp with no fractional second.
    #[must_use]
    pub const fn from_unix_seconds(unix_seconds: i64) -> Self {
        Self {
            unix_seconds,
            nanoseconds: 0,
        }
    }

    /// Returns Unix seconds.
    #[must_use]
    pub const fn unix_seconds(self) -> i64 {
        self.unix_seconds
    }

    /// Returns fractional nanoseconds.
    #[must_use]
    pub const fn nanoseconds(self) -> u32 {
        self.nanoseconds
    }
}

impl fmt::Display for HealthTimestamp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{:09}",
            self.unix_seconds,
            self.nanoseconds
        )
    }
}

// ============================================================================
// Observation validity
// ============================================================================

/// Semantic validity of a health observation.
///
/// This deliberately separates observation validity from `HealthState`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HealthObservationValidity {
    /// Observation is structurally and semantically usable as evidence.
    Valid,

    /// Observation was not available from the source.
    Unavailable,

    /// Observation exists but its health state cannot currently be established.
    Unknown,

    /// Observation is older than the applicable freshness policy.
    Stale,

    /// Observation failed structural validation.
    Invalid,
}

impl HealthObservationValidity {
    /// Returns a stable machine-readable representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Valid => "valid",
            Self::Unavailable => "unavailable",
            Self::Unknown => "unknown",
            Self::Stale => "stale",
            Self::Invalid => "invalid",
        }
    }

    /// Returns whether the observation can be consumed as positive evidence.
    #[must_use]
    pub const fn is_usable(self) -> bool {
        matches!(self, Self::Valid)
    }

    /// Returns whether the observation requires interpretation before use.
    #[must_use]
    pub const fn requires_attention(self) -> bool {
        !matches!(self, Self::Valid)
    }
}

impl fmt::Display for HealthObservationValidity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Observation source class
// ============================================================================

/// Broad source class for a health observation.
///
/// Provider-specific source details remain represented by `HealthSourceId`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HealthSourceKind {
    /// Quantum hardware abstraction layer.
    Hardware,

    /// Quantum runtime or execution engine.
    Runtime,

    /// Calibration subsystem.
    Calibration,

    /// QEC subsystem.
    Qec,

    /// Simulator.
    Simulation,

    /// Emulator.
    Emulator,

    /// Benchmarking subsystem.
    Benchmarking,

    /// Resilience detector.
    Detection,

    /// Resilience diagnosis.
    Diagnosis,

    /// Application-level source.
    Application,

    /// External observer.
    External,

    /// Unknown source category.
    Unknown,
}

impl HealthSourceKind {
    /// Returns a stable machine-readable representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Hardware => "hardware",
            Self::Runtime => "runtime",
            Self::Calibration => "calibration",
            Self::Qec => "qec",
            Self::Simulation => "simulation",
            Self::Emulator => "emulator",
            Self::Benchmarking => "benchmarking",
            Self::Detection => "detection",
            Self::Diagnosis => "diagnosis",
            Self::Application => "application",
            Self::External => "external",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for HealthSourceKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Health observation
// ============================================================================

/// One immutable observation of a resource's health.
///
/// This is the primary type exported by this module.
///
/// It intentionally contains no recovery action and no authority.
///
/// A health observation is evidence:
///
///     observation
///         ↓
///     detection / diagnosis / policy
///         ↓
///     possible decision
///
/// not:
///
///     observation
///         ↓
///     automatic action
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HealthObservation {
    observation_id: HealthObservationId,
    resource: ResourceIdentity,
    resource_kind: ResourceKind,
    scope: ResourceScope,
    state: HealthState,
    condition: HealthConditionKind,
    validity: HealthObservationValidity,
    source_kind: HealthSourceKind,
    source_id: HealthSourceId,
    observed_at: HealthTimestamp,
    sequence: Option<u64>,
    generation: Option<u64>,
    detail: Option<Arc<str>>,
}

impl HealthObservation {
    /// Creates a validated health observation.
    ///
    /// The constructor is deliberately explicit about all identity and
    /// temporal fields so deterministic replay does not depend on hidden
    /// runtime state.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        observation_id: HealthObservationId,
        resource: ResourceIdentity,
        resource_kind: ResourceKind,
        scope: ResourceScope,
        state: HealthState,
        condition: HealthConditionKind,
        validity: HealthObservationValidity,
        source_kind: HealthSourceKind,
        source_id: HealthSourceId,
        observed_at: HealthTimestamp,
        sequence: Option<u64>,
        generation: Option<u64>,
        detail: Option<Arc<str>>,
    ) -> Result<Self, HealthTelemetryError> {
        if detail
            .as_deref()
            .is_some_and(|value| value.trim().is_empty())
        {
            return Err(HealthTelemetryError::EmptyDetail);
        }

        let expected_scope = ResourceScope::for_identity(resource);

        if scope != expected_scope {
            return Err(HealthTelemetryError::ResourceScopeMismatch {
                expected: expected_scope,
                actual: scope,
            });
        }

        if matches!(
            validity,
            HealthObservationValidity::Invalid
        ) {
            return Err(HealthTelemetryError::InvalidObservationValidity);
        }

        Ok(Self {
            observation_id,
            resource,
            resource_kind,
            scope,
            state,
            condition,
            validity,
            source_kind,
            source_id,
            observed_at,
            sequence,
            generation,
            detail,
        })
    }

    /// Returns the observation identity.
    #[must_use]
    pub fn observation_id(&self) -> &HealthObservationId {
        &self.observation_id
    }

    /// Returns the canonical resource identity.
    #[must_use]
    pub const fn resource(&self) -> ResourceIdentity {
        self.resource
    }

    /// Returns the semantic resource kind.
    #[must_use]
    pub &ResourceKind {
        &self.resource_kind
    }

    /// Returns the resource scope.
    #[must_use]
    pub const fn scope(&self) -> ResourceScope {
        self.scope
    }

    /// Returns the observed health state.
    #[must_use]
    pub const fn state(&self) -> HealthState {
        self.state
    }

    /// Returns the health-condition category.
    #[must_use]
    pub const fn condition(&self) -> HealthConditionKind {
        self.condition
    }

    /// Returns observation validity.
    #[must_use]
    pub const fn validity(&self) -> HealthObservationValidity {
        self.validity
    }

    /// Returns the source class.
    #[must_use]
    pub const fn source_kind(&self) -> HealthSourceKind {
        self.source_kind
    }

    /// Returns the explicit source identity.
    #[must_use]
    pub &HealthSourceId {
        &self.source_id
    }

    /// Returns the observation timestamp.
    #[must_use]
    pub const fn observed_at(&self) -> HealthTimestamp {
        self.observed_at
    }

    /// Returns the producer sequence number when supplied.
    #[must_use]
    pub const fn sequence(&self) -> Option<u64> {
        self.sequence
    }

    /// Returns the source generation when supplied.
    ///
    /// A generation may identify a calibration/device/configuration generation
    /// without embedding provider-specific semantics in this module.
    #[must_use]
    pub const fn generation(&self) -> Option<u64> {
        self.generation
    }

    /// Returns optional human-readable diagnostic detail.
    #[must_use]
    pub fn detail(&self) -> Option<&str> {
        self.detail.as_deref()
    }

    /// Returns whether this observation is usable evidence.
    #[must_use]
    pub const fn is_usable(&self) -> bool {
        self.validity.is_usable()
    }

    /// Returns whether the observed resource requires health attention.
    #[must_use]
    pub const fn requires_attention(&self) -> bool {
        self.state.requires_attention()
    }

    /// Returns a deterministic ordering key.
    ///
    /// Observation identifiers are intentionally included after resource and
    /// time fields so equal-time observations remain deterministically ordered.
    #[must_use]
    pub fn ordering_key(
        &self,
    ) -> (
        ResourceIdentity,
        HealthTimestamp,
        Option<u64>,
        &HealthObservationId,
    ) {
        (
            self.resource,
            self.observed_at,
            self.sequence,
            &self.observation_id,
        )
    }
}

// ============================================================================
// Health snapshot
// ============================================================================

/// Immutable collection of health observations representing one telemetry
/// snapshot.
///
/// A snapshot does not imply that every resource in the universe is present.
/// Absence means "not represented in this snapshot", not "healthy".
///
/// This distinction is essential for distributed systems and partial telemetry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HealthSnapshot {
    schema_id: &'static str,
    schema_version: u32,
    observations: Vec<HealthObservation>,
}

impl HealthSnapshot {
    /// Creates an empty health snapshot.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            schema_id: HEALTH_TELEMETRY_SCHEMA_ID,
            schema_version: HEALTH_TELEMETRY_SCHEMA_VERSION,
            observations: Vec::new(),
        }
    }

    /// Creates a snapshot from observations.
    ///
    /// Observations are sorted deterministically.
    ///
    /// No arbitrary capacity limit is imposed. The actual allocation is
    /// naturally bounded by available memory and caller policy.
    pub fn from_observations(
        mut observations: Vec<HealthObservation>,
    ) -> Result<Self, HealthTelemetryError> {
        for observation in &observations {
            if !observation.is_usable() {
                continue;
            }

            if observation.state().is_retired()
                && observation.validity() != HealthObservationValidity::Valid
            {
                return Err(HealthTelemetryError::RetiredObservationNotValid);
            }
        }

        observations.sort_by(|left, right| {
            left.ordering_key().cmp(&right.ordering_key())
        });

        Ok(Self {
            schema_id: HEALTH_TELEMETRY_SCHEMA_ID,
            schema_version: HEALTH_TELEMETRY_SCHEMA_VERSION,
            observations,
        })
    }

    /// Returns the schema identifier.
    #[must_use]
    pub const fn schema_id(&self) -> &'static str {
        self.schema_id
    }

    /// Returns the schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// Returns the number of observations in the snapshot.
    #[must_use]
    pub fn len(&self) -> usize {
        self.observations.len()
    }

    /// Returns whether the snapshot contains no observations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.observations.is_empty()
    }

    /// Returns observations in deterministic order.
    #[must_use]
    pub fn observations(&self) -> &[HealthObservation] {
        &self.observations
    }

    /// Returns an iterator over observations.
    pub fn iter(&self) -> impl Iterator<Item = &HealthObservation> {
        self.observations.iter()
    }

    /// Finds all observations for one canonical resource.
    ///
    /// The result is an iterator to avoid allocating another collection.
    pub fn for_resource(
        &self,
        resource: ResourceIdentity,
    ) -> impl Iterator<Item = &HealthObservation> {
        self.observations
            .iter()
            .filter(move |observation| observation.resource() == resource)
    }

    /// Returns the most recent observation for a resource.
    ///
    /// "Most recent" is determined by explicit observation timestamp and then
    /// producer sequence. Arrival order is never used.
    #[must_use]
    pub fn latest_for_resource(
        &self,
        resource: ResourceIdentity,
    ) -> Option<&HealthObservation> {
        self.for_resource(resource)
            .max_by(|left, right| {
                (
                    left.observed_at(),
                    left.sequence(),
                    left.observation_id(),
                )
                    .cmp(&(
                        right.observed_at(),
                        right.sequence(),
                        right.observation_id(),
                    ))
            })
    }

    /// Computes a conservative aggregate health state across usable
    /// observations.
    ///
    /// Important:
    ///
    ///     UNKNOWN != UNAVAILABLE
    ///
    /// Therefore unknown observations are not silently converted to
    /// unavailable.
    ///
    /// The aggregation is intended for observability summaries only. It is
    /// not a recovery decision.
    #[must_use]
    pub fn aggregate_state(&self) -> HealthState {
        let mut aggregate = HealthState::Healthy;
        let mut observed_any = false;
        let mut unknown_present = false;

        for observation in &self.observations {
            if !observation.is_usable() {
                unknown_present = true;
                continue;
            }

            observed_any = true;
            aggregate = aggregate.max(observation.state());
        }

        if !observed_any {
            return HealthState::Unknown;
        }

        if unknown_present && aggregate.is_healthy() {
            return HealthState::Unknown;
        }

        aggregate
    }

    /// Counts usable observations for a given health state.
    #[must_use]
    pub fn count_state(&self, state: HealthState) -> usize {
        self.observations
            .iter()
            .filter(|observation| {
                observation.is_usable() && observation.state() == state
            })
            .count()
    }

    /// Counts observations requiring attention.
    #[must_use]
    pub fn attention_count(&self) -> usize {
        self.observations
            .iter()
            .filter(|observation| {
                observation.is_usable() && observation.requires_attention()
            })
            .count()
    }

    /// Counts observations that are not currently usable.
    #[must_use]
    pub fn unusable_count(&self) -> usize {
        self.observations
            .iter()
            .filter(|observation| !observation.is_usable())
            .count()
    }
}

impl Default for HealthSnapshot {
    fn default() -> Self {
        Self::empty()
    }
}

// ============================================================================
// Incremental health telemetry accumulator
// ============================================================================

/// Deterministic in-memory accumulator for health observations.
///
/// This is intentionally a simple value type rather than a globally shared
/// singleton.
///
/// The collector or runtime owns its lifecycle and decides when to publish a
/// snapshot.
///
/// It does not impose a retention limit. Production deployments may wrap it
/// with a bounded retention policy, streaming sink, or persistence layer.
///
/// This separation is important because a universal quantum runtime must not
/// impose one telemetry retention policy on every deployment.
#[derive(Debug, Default)]
pub struct HealthTelemetry {
    observations: Vec<HealthObservation>,
}

impl HealthTelemetry {
    /// Creates an empty accumulator.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            observations: Vec::new(),
        }
    }

    /// Returns the number of observations currently retained.
    #[must_use]
    pub fn len(&self) -> usize {
        self.observations.len()
    }

    /// Returns whether no observations are retained.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.observations.is_empty()
    }

    /// Adds one immutable observation.
    ///
    /// No existing observation is mutated.
    pub fn record(
        &mut self,
        observation: HealthObservation,
    ) -> Result<(), HealthTelemetryError> {
        if self
            .observations
            .iter()
            .any(|existing| existing.observation_id() == observation.observation_id())
        {
            return Err(HealthTelemetryError::DuplicateObservationId);
        }

        self.observations.push(observation);
        Ok(())
    }

    /// Records many observations.
    ///
    /// The operation is transactional with respect to duplicate identifiers:
    /// if a duplicate is found, no observation from the supplied batch is
    /// appended.
    pub fn record_batch<I>(
        &mut self,
        observations: I,
    ) -> Result<(), HealthTelemetryError>
    where
        I: IntoIterator<Item = HealthObservation>,
    {
        let incoming: Vec<HealthObservation> = observations.into_iter().collect();

        for candidate in &incoming {
            if self
                .observations
                .iter()
                .any(|existing| existing.observation_id() == candidate.observation_id())
            {
                return Err(HealthTelemetryError::DuplicateObservationId);
            }
        }

        for left in 0..incoming.len() {
            for right in (left + 1)..incoming.len() {
                if incoming[left].observation_id() == incoming[right].observation_id() {
                    return Err(HealthTelemetryError::DuplicateObservationId);
                }
            }
        }

        self.observations.extend(incoming);
        Ok(())
    }

    /// Produces an immutable deterministic snapshot.
    pub fn snapshot(&self) -> Result<HealthSnapshot, HealthTelemetryError> {
        HealthSnapshot::from_observations(self.observations.clone())
    }

    /// Clears retained observations and returns them in deterministic order.
    ///
    /// This method does not delete durable history because this type owns only
    /// its local in-memory accumulation.
    pub fn drain_snapshot(&mut self) -> Result<HealthSnapshot, HealthTelemetryError> {
        let observations = core::mem::take(&mut self.observations);
        HealthSnapshot::from_observations(observations)
    }

    /// Removes all retained observations.
    ///
    /// This affects only this accumulator. It is not a persistence operation.
    pub fn clear(&mut self) {
        self.observations.clear();
    }
}

// ============================================================================
// Health summary
// ============================================================================

/// Constant-size aggregate summary of a health snapshot.
///
/// This type is useful for metrics/exporters because it avoids requiring an
/// exporter to emit one high-cardinality metric for every observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HealthSummary {
    unknown: usize,
    healthy: usize,
    degraded: usize,
    unstable: usize,
    unavailable: usize,
    recovering: usize,
    quarantined: usize,
    retired: usize,
    unusable: usize,
}

impl HealthSummary {
    /// Computes a summary from a snapshot.
    #[must_use]
    pub fn from_snapshot(snapshot: &HealthSnapshot) -> Self {
        Self {
            unknown: snapshot.count_state(HealthState::Unknown),
            healthy: snapshot.count_state(HealthState::Healthy),
            degraded: snapshot.count_state(HealthState::Degraded),
            unstable: snapshot.count_state(HealthState::Unstable),
            unavailable: snapshot.count_state(HealthState::Unavailable),
            recovering: snapshot.count_state(HealthState::Recovering),
            quarantined: snapshot.count_state(HealthState::Quarantined),
            retired: snapshot.count_state(HealthState::Retired),
            unusable: snapshot.unusable_count(),
        }
    }

    /// Number of unknown observations.
    #[must_use]
    pub const fn unknown(self) -> usize {
        self.unknown
    }

    /// Number of healthy observations.
    #[must_use]
    pub const fn healthy(self) -> usize {
        self.healthy
    }

    /// Number of degraded observations.
    #[must_use]
    pub const fn degraded(self) -> usize {
        self.degraded
    }

    /// Number of unstable observations.
    #[must_use]
    pub const fn unstable(self) -> usize {
        self.unstable
    }

    /// Number of unavailable observations.
    #[must_use]
    pub const fn unavailable(self) -> usize {
        self.unavailable
    }

    /// Number of recovering observations.
    #[must_use]
    pub const fn recovering(self) -> usize {
        self.recovering
    }

    /// Number of quarantined observations.
    #[must_use]
    pub const fn quarantined(self) -> usize {
        self.quarantined
    }

    /// Number of retired observations.
    #[must_use]
    pub const fn retired(self) -> usize {
        self.retired
    }

    /// Number of observations whose validity is not usable.
    #[must_use]
    pub const fn unusable(self) -> usize {
        self.unusable
    }

    /// Total number of observations represented by this summary.
    #[must_use]
    pub const fn total(self) -> usize {
        self.unknown
            + self.healthy
            + self.degraded
            + self.unstable
            + self.unavailable
            + self.recovering
            + self.quarantined
            + self.retired
            + self.unusable
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Errors specific to the health telemetry boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HealthTelemetryError {
    /// Observation identifier was empty.
    EmptyObservationId,

    /// Source identifier was empty.
    EmptySourceId,

    /// Detail was supplied but contained only whitespace.
    EmptyDetail,

    /// Timestamp fractional component was invalid.
    InvalidNanoseconds,

    /// Resource identity and declared scope disagree.
    ResourceScopeMismatch {
        /// Scope implied by the canonical resource identity.
        expected: ResourceScope,

        /// Scope supplied by the observation producer.
        actual: ResourceScope,
    },

    /// An invalid observation validity value was explicitly submitted.
    InvalidObservationValidity,

    /// A retired resource was represented by an observation that was not
    /// valid evidence.
    RetiredObservationNotValid,

    /// An observation identifier already exists in an accumulator.
    DuplicateObservationId,
}

impl fmt::Display for HealthTelemetryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyObservationId => {
                formatter.write_str("health observation identifier is empty")
            }

            Self::EmptySourceId => {
                formatter.write_str("health telemetry source identifier is empty")
            }

            Self::EmptyDetail => {
                formatter.write_str("health observation detail is empty")
            }

            Self::InvalidNanoseconds => {
                formatter.write_str(
                    "health observation timestamp nanoseconds must be less than one billion",
                )
            }

            Self::ResourceScopeMismatch { expected, actual } => {
                write!(
                    formatter,
                    "health observation resource scope mismatch: expected {expected:?}, got {actual:?}"
                )
            }

            Self::InvalidObservationValidity => {
                formatter.write_str(
                    "an explicitly invalid health observation cannot be recorded",
                )
            }

            Self::RetiredObservationNotValid => {
                formatter.write_str(
                    "a retired-resource health observation must be valid evidence",
                )
            }

            Self::DuplicateObservationId => {
                formatter.write_str("duplicate health observation identifier")
            }
        }
    }
}

impl std::error::Error for HealthTelemetryError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::core::identity::ResourceId;

    fn resource() -> ResourceIdentity {
        ResourceIdentity::ir(ResourceId::new())
    }

    fn resource_kind() -> ResourceKind {
        ResourceKind::new("test.resource")
            .expect("test resource kind must be valid")
    }

    fn source() -> HealthSourceId {
        HealthSourceId::new("test-source")
            .expect("test source must be valid")
    }

    fn observation_id(value: &str) -> HealthObservationId {
        HealthObservationId::new(value)
            .expect("test observation identifier must be valid")
    }

    fn observation(
        id: &str,
        state: HealthState,
        timestamp: i64,
    ) -> HealthObservation {
        let identity = resource();

        HealthObservation::new(
            observation_id(id),
            identity,
            resource_kind(),
            ResourceScope::Generic,
            state,
            HealthConditionKind::Operational,
            HealthObservationValidity::Valid,
            HealthSourceKind::Hardware,
            source(),
            HealthTimestamp::from_unix_seconds(timestamp),
            None,
            None,
            None,
        )
        .expect("test observation must be valid")
    }

    #[test]
    fn health_timestamp_rejects_invalid_nanoseconds() {
        let result = HealthTimestamp::new(
            0,
            HealthTimestamp::NANOS_PER_SECOND,
        );

        assert!(matches!(
            result,
            Err(HealthTelemetryError::InvalidNanoseconds)
        ));
    }

    #[test]
    fn health_observation_rejects_scope_mismatch() {
        let identity = resource();

        let result = HealthObservation::new(
            observation_id("scope-mismatch"),
            identity,
            resource_kind(),
            ResourceScope::Physical,
            HealthState::Healthy,
            HealthConditionKind::Operational,
            HealthObservationValidity::Valid,
            HealthSourceKind::Hardware,
            source(),
            HealthTimestamp::from_unix_seconds(0),
            None,
            None,
            None,
        );

        assert!(matches!(
            result,
            Err(HealthTelemetryError::ResourceScopeMismatch {
                expected: ResourceScope::Generic,
                actual: ResourceScope::Physical,
            })
        ));
    }

    #[test]
    fn snapshot_orders_observations_deterministically() {
        let first = observation("b", HealthState::Healthy, 2);
        let second = observation("a", HealthState::Healthy, 1);

        let snapshot = HealthSnapshot::from_observations(vec![first, second])
            .expect("snapshot must be valid");

        assert_eq!(snapshot.len(), 2);
        assert_eq!(
            snapshot.observations()[0]
                .observation_id()
                .as_str(),
            "a"
        );
        assert_eq!(
            snapshot.observations()[1]
                .observation_id()
                .as_str(),
            "b"
        );
    }

    #[test]
    fn snapshot_does_not_treat_missing_resources_as_healthy() {
        let snapshot = HealthSnapshot::empty();

        assert_eq!(snapshot.aggregate_state(), HealthState::Unknown);
    }

    #[test]
    fn healthy_and_unknown_do_not_become_healthy_aggregate() {
        let healthy = observation("healthy", HealthState::Healthy, 1);

        let mut unknown = observation("unknown", HealthState::Unknown, 2);

        /*
         * Unknown is a valid health state, so it remains valid evidence.
         * The aggregate must preserve uncertainty when no degraded condition
         * otherwise dominates it.
         */
        unknown = HealthObservation::new(
            observation_id("unknown"),
            unknown.resource(),
            resource_kind(),
            ResourceScope::Generic,
            HealthState::Unknown,
            HealthConditionKind::Operational,
            HealthObservationValidity::Valid,
            HealthSourceKind::Hardware,
            source(),
            unknown.observed_at(),
            None,
            None,
            None,
        )
        .expect("unknown observation must be valid");

        let snapshot = HealthSnapshot::from_observations(vec![healthy, unknown])
            .expect("snapshot must be valid");

        assert_eq!(snapshot.aggregate_state(), HealthState::Unknown);
    }

    #[test]
    fn degraded_dominates_healthy_observation() {
        let degraded = observation("degraded", HealthState::Degraded, 2);

        let healthy = HealthObservation::new(
            observation_id("healthy"),
            degraded.resource(),
            resource_kind(),
            ResourceScope::Generic,
            HealthState::Healthy,
            HealthConditionKind::Operational,
            HealthObservationValidity::Valid,
            HealthSourceKind::Hardware,
            source(),
            HealthTimestamp::from_unix_seconds(1),
            None,
            None,
            None,
        )
        .expect("healthy observation must be valid");

        let snapshot = HealthSnapshot::from_observations(vec![healthy, degraded])
            .expect("snapshot must be valid");

        assert_eq!(snapshot.aggregate_state(), HealthState::Degraded);
    }

    #[test]
    fn accumulator_rejects_duplicate_observation_ids() {
        let mut telemetry = HealthTelemetry::new();

        telemetry
            .record(observation("same", HealthState::Healthy, 1))
            .expect("first observation must succeed");

        let result =
            telemetry.record(observation("same", HealthState::Degraded, 2));

        assert!(matches!(
            result,
            Err(HealthTelemetryError::DuplicateObservationId)
        ));

        assert_eq!(telemetry.len(), 1);
    }

    #[test]
    fn batch_record_is_transactional_for_duplicates() {
        let mut telemetry = HealthTelemetry::new();

        let first = observation("first", HealthState::Healthy, 1);
        telemetry
            .record(first)
            .expect("first observation must succeed");

        let second = observation("second", HealthState::Healthy, 2);
        let duplicate = observation("second", HealthState::Degraded, 3);

        let result = telemetry.record_batch(vec![second, duplicate]);

        assert!(matches!(
            result,
            Err(HealthTelemetryError::DuplicateObservationId)
        ));

        assert_eq!(telemetry.len(), 1);
    }

    #[test]
    fn drain_snapshot_transfers_ownership() {
        let mut telemetry = HealthTelemetry::new();

        telemetry
            .record(observation("one", HealthState::Healthy, 1))
            .expect("observation must succeed");

        let snapshot = telemetry
            .drain_snapshot()
            .expect("snapshot must succeed");

        assert_eq!(snapshot.len(), 1);
        assert!(telemetry.is_empty());
    }

    #[test]
    fn summary_is_constant_size() {
        let healthy = observation("healthy", HealthState::Healthy, 1);
        let degraded = HealthObservation::new(
            observation_id("degraded"),
            healthy.resource(),
            resource_kind(),
            ResourceScope::Generic,
            HealthState::Degraded,
            HealthConditionKind::Operational,
            HealthObservationValidity::Valid,
            HealthSourceKind::Hardware,
            source(),
            HealthTimestamp::from_unix_seconds(2),
            None,
            None,
            None,
        )
        .expect("degraded observation must be valid");

        let snapshot =
            HealthSnapshot::from_observations(vec![healthy, degraded])
                .expect("snapshot must be valid");

        let summary = HealthSummary::from_snapshot(&snapshot);

        assert_eq!(summary.healthy(), 1);
        assert_eq!(summary.degraded(), 1);
        assert_eq!(summary.total(), 2);
    }

    #[test]
    fn retired_is_terminal_in_observation_semantics() {
        let retired = observation("retired", HealthState::Retired, 1);

        assert!(retired.state().is_terminal());
        assert!(retired.requires_attention());
    }

    #[test]
    fn source_and_observation_ids_are_opaque() {
        let source = HealthSourceId::new("provider/device/source")
            .expect("source must be valid");

        let observation =
            HealthObservationId::new("provider-specific-observation")
                .expect("observation id must be valid");

        assert_eq!(source.as_str(), "provider/device/source");
        assert_eq!(
            observation.as_str(),
            "provider-specific-observation"
        );
    }
}