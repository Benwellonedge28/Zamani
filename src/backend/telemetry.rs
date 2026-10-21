
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Privacy-conscious, vendor-neutral telemetry for Zamani backend operations.
//!
//! Path: `src/backend/telemetry.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Collect aggregate measurements and optional bounded event history about
//! backend operations. Telemetry is observational: recording, sampling,
//! snapshotting, or exporting telemetry must not change compilation results,
//! target selection, artifact validity, or program semantics.
//!
//! # Ownership
//!
//! This module owns:
//! - Backend-stage and outcome classifications.
//! - Aggregate counters and durations.
//! - Configurable event sampling and retention.
//! - Privacy-conscious snapshots.
//! - Export policy checks and a vendor-neutral sink interface.
//!
//! This module does not own:
//! - Compiler diagnostics, error classification, or recovery policy.
//! - Source-language or IR semantics.
//! - Target capabilities, hardware allocation, or execution scheduling.
//! - External network clients, logging frameworks, or telemetry vendors.
//! - Global state, persistence, or telemetry service lifecycle.
//!
//! # Privacy contract
//!
//! Records contain typed stage/outcome/failure classifications and numeric
//! measurements only. They do not accept source text, IR text, arbitrary
//! labels, paths, circuit data, raw measurements, credentials, device
//! identifiers, or proprietary topology.
//!
//! A caller must not encode sensitive information into a typed classification
//! or misuse numeric fields as covert payloads. Remote export contains
//! aggregate data only, never retained event history.
//!
//! # Resource contract
//!
//! Event retention and deterministic sampling are configurable. The retention
//! capacity is an operational memory budget, not a hardware or language limit.
//! A zero capacity retains no events while aggregate counters continue.
//! Aggregate counters saturate rather than overflow.
//!
//! # Integration
//!
//! Declare `pub mod telemetry;` in the canonical `src/backend/mod.rs`.
//! The pipeline may call `Telemetry::record` at stage boundaries and obtain
//! snapshots for reporting. Existing compiler diagnostics remain responsible
//! for errors; `src/compiler/safety_guard.rs` remains responsible for security
//! inspection. Neither responsibility is duplicated here.
//!
//! `ExplicitRemote` is an explicit policy choice, not an authentication system.
//! The integration layer must authorize remote export using its security and
//! capability policy before issuing a `RemoteExportPermit`.
//!
//! # Side effects
//!
//! This module performs no I/O and starts no background tasks. Export occurs
//! only when a caller explicitly supplies a `TelemetrySink`.

use std::collections::{BTreeMap, VecDeque};
use std::fmt;
use std::sync::{Mutex, MutexGuard};
use std::time::Duration;

/// Default retained-event capacity.
///
/// This is a configurable operational default, not a universal limit.
/// Set it to zero to disable event retention.
pub const DEFAULT_EVENT_CAPACITY: usize = 1_024;

/// Default sampling interval: retain every sampled event.
pub const DEFAULT_SAMPLE_EVERY: u64 = 1;

/// Controls whether telemetry is collected and how it may be exported.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TelemetryPolicy {
    /// Do not record measurements or events.
    #[default]
    Disabled,

    /// Collect measurements and permit local snapshots only.
    ///
    /// This module does not persist the snapshot. The caller is responsible
    /// for keeping any snapshot local.
    LocalOnly,

    /// Collect measurements and permit aggregate-only sink export.
    ///
    /// Retained event history is excluded from exported snapshots.
    Aggregated,

    /// Permit aggregate-only export after explicit authorization.
    ///
    /// This setting does not itself authenticate a destination or establish
    /// that a remote endpoint is trustworthy.
    ExplicitRemote,
}

/// Configuration for telemetry collection.
///
/// Configuration is owned by the caller and may be constructed independently
/// for each compiler instance or compilation request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TelemetryConfig {
    /// Collection and export policy.
    pub policy: TelemetryPolicy,

    /// Maximum number of events retained in memory.
    ///
    /// This affects event history only, not aggregate counters.
    pub event_capacity: usize,

    /// Retain one event for every `sample_every` observations.
    ///
    /// The selection is deterministic from the observation sequence number.
    /// A value of zero is invalid.
    pub sample_every: u64,
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            policy: TelemetryPolicy::LocalOnly,
            event_capacity: DEFAULT_EVENT_CAPACITY,
            sample_every: DEFAULT_SAMPLE_EVERY,
        }
    }
}

impl TelemetryConfig {
    /// Validate this configuration without allocating telemetry state.
    pub fn validate(&self) -> Result<(), TelemetryConfigError> {
        if self.sample_every == 0 {
            return Err(TelemetryConfigError::ZeroSamplingInterval);
        }

        Ok(())
    }
}

/// Configuration errors detected before telemetry collection begins.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TelemetryConfigError {
    /// Sampling interval must be greater than zero.
    ZeroSamplingInterval,
}

impl fmt::Display for TelemetryConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroSamplingInterval => {
                formatter.write_str("telemetry sampling interval must be greater than zero")
            }
        }
    }
}

impl std::error::Error for TelemetryConfigError {}

/// Stable, low-cardinality backend stage classification.
///
/// Add variants when the pipeline establishes a new stable stage contract.
/// Do not put target names, file paths, source identifiers, or user-supplied
/// strings into this type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum TelemetryStage {
    /// Input and request validation.
    Validation,
    /// Target and capability compatibility checking.
    Compatibility,
    /// Lowering into a target representation.
    Lowering,
    /// Target-specific code or artifact generation.
    Emission,
    /// Validation of a generated artifact.
    ArtifactValidation,
    /// Linking or artifact composition.
    Linking,
    /// Packaging or manifest generation.
    Packaging,
    /// An external toolchain operation.
    Toolchain,
    /// A backend-defined stage not yet assigned a dedicated category.
    Other,
}

/// Result category for an observed backend operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum TelemetryOutcome {
    /// Operation completed successfully.
    Succeeded,
    /// Operation failed.
    Failed,
    /// Operation was cancelled.
    Cancelled,
}

/// Sanitized failure categories suitable for aggregate reporting.
///
/// These are intentionally not free-form error messages. Detailed errors
/// belong to the backend error and diagnostic systems.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum TelemetryFailureKind {
    /// Invalid input or configuration.
    InvalidInput,
    /// Required target capability is unavailable.
    UnsupportedCapability,
    /// Verification failed.
    Verification,
    /// Required toolchain was unavailable or failed.
    Toolchain,
    /// An I/O operation failed.
    Io,
    /// An operational resource budget was exhausted.
    ResourceExhausted,
    /// An artifact was invalid or incomplete.
    InvalidArtifact,
    /// An internal backend invariant failed.
    Internal,
    /// Failure does not fit another stable category.
    Other,
}

/// Measurements for one completed backend operation.
///
/// The type deliberately has no string, path, source, IR, or device fields.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TelemetryObservation {
    /// Pipeline stage where the operation occurred.
    pub stage: TelemetryStage,

    /// Final outcome of the operation.
    pub outcome: TelemetryOutcome,

    /// Sanitized failure category, if applicable.
    pub failure: Option<TelemetryFailureKind>,

    /// Elapsed time for this operation.
    pub duration: Duration,

    /// Input bytes processed, when meaningful.
    pub input_bytes: u64,

    /// Output bytes produced, when meaningful.
    pub output_bytes: u64,
}

impl TelemetryObservation {
    /// Construct an observation with no byte counts and no failure category.
    #[must_use]
    pub const fn new(stage: TelemetryStage, outcome: TelemetryOutcome) -> Self {
        Self {
            stage,
            outcome,
            failure: None,
            duration: Duration::ZERO,
            input_bytes: 0,
            output_bytes: 0,
        }
    }

    /// Set the operation duration.
    #[must_use]
    pub const fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Set input and output byte counts.
    #[must_use]
    pub const fn with_byte_counts(mut self, input_bytes: u64, output_bytes: u64) -> Self {
        self.input_bytes = input_bytes;
        self.output_bytes = output_bytes;
        self
    }

    /// Attach a sanitized failure category.
    #[must_use]
    pub const fn with_failure(mut self, failure: TelemetryFailureKind) -> Self {
        self.failure = Some(failure);
        self
    }
}

/// Result of attempting to record an observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecordDisposition {
    /// Collection is disabled; no state was changed.
    Disabled,
    /// Aggregate counters were updated and the event was retained.
    Retained,
    /// Aggregate counters were updated, but deterministic sampling skipped
    /// retaining the event.
    SampledOut,
    /// Aggregate counters were updated, but event retention is disabled.
    RetentionDisabled,
    /// Aggregate counters were updated; an older event was evicted to retain
    /// the new event.
    RetainedAfterEviction,
}

/// Aggregate counters for a single pipeline stage.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StageMetrics {
    /// Number of observations for this stage.
    pub observations: u64,
    /// Number of successful observations.
    pub succeeded: u64,
    /// Number of failed observations.
    pub failed: u64,
    /// Number of cancelled observations.
    pub cancelled: u64,
    /// Saturating sum of operation durations in nanoseconds.
    pub total_duration_nanos: u64,
    /// Saturating sum of input bytes.
    pub input_bytes: u64,
    /// Saturating sum of output bytes.
    pub output_bytes: u64,
}

impl StageMetrics {
    fn record(&mut self, observation: TelemetryObservation) {
        self.observations = self.observations.saturating_add(1);

        match observation.outcome {
            TelemetryOutcome::Succeeded => {
                self.succeeded = self.succeeded.saturating_add(1);
            }
            TelemetryOutcome::Failed => {
                self.failed = self.failed.saturating_add(1);
            }
            TelemetryOutcome::Cancelled => {
                self.cancelled = self.cancelled.saturating_add(1);
            }
        }

        self.total_duration_nanos = self
            .total_duration_nanos
            .saturating_add(duration_nanos_saturating(observation.duration));
        self.input_bytes = self.input_bytes.saturating_add(observation.input_bytes);
        self.output_bytes = self.output_bytes.saturating_add(observation.output_bytes);
    }
}

/// A retained, privacy-conscious event.
///
/// It contains no wall-clock timestamp so repeated builds can produce stable
/// snapshots for the same observation sequence and inputs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TelemetryEvent {
    /// Monotonically increasing observation sequence within this instance.
    pub sequence: u64,
    /// Pipeline stage.
    pub stage: TelemetryStage,
    /// Outcome.
    pub outcome: TelemetryOutcome,
    /// Sanitized failure category, if present.
    pub failure: Option<TelemetryFailureKind>,
    /// Operation duration in nanoseconds, saturated to `u64`.
    pub duration_nanos: u64,
    /// Input bytes processed.
    pub input_bytes: u64,
    /// Output bytes produced.
    pub output_bytes: u64,
}

/// Snapshot of aggregate backend telemetry.
///
/// `events` is populated only for local snapshots under `LocalOnly` policy.
/// Aggregate and remote-export snapshots omit it.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TelemetrySnapshot {
    /// Number of observations processed by this telemetry instance.
    pub observations: u64,
    /// Total successful operations.
    pub succeeded: u64,
    /// Total failed operations.
    pub failed: u64,
    /// Total cancelled operations.
    pub cancelled: u64,
    /// Saturating total duration in nanoseconds.
    pub total_duration_nanos: u64,
    /// Saturating total input bytes.
    pub input_bytes: u64,
    /// Saturating total output bytes.
    pub output_bytes: u64,
    /// Number of observations excluded by deterministic sampling.
    pub sampled_out: u64,
    /// Number of retained events evicted to make room for newer events.
    pub evicted_events: u64,
    /// Number of sampled events not retained because capacity was zero.
    pub unretained_events: u64,
    /// Aggregate metrics by stable pipeline stage.
    pub stages: BTreeMap<TelemetryStage, StageMetrics>,
    /// Optional retained history; empty for aggregate-only snapshots.
    pub events: Vec<TelemetryEvent>,
}

impl TelemetrySnapshot {
    /// Produce a snapshot with no event history.
    ///
    /// This is the format used for aggregate and remote export.
    fn aggregate_only(mut self) -> Self {
        self.events.clear();
        self
    }
}

#[derive(Debug, Default)]
struct TelemetryState {
    sequence: u64,
    observations: u64,
    succeeded: u64,
    failed: u64,
    cancelled: u64,
    total_duration_nanos: u64,
    input_bytes: u64,
    output_bytes: u64,
    sampled_out: u64,
    evicted_events: u64,
    unretained_events: u64,
    stages: BTreeMap<TelemetryStage, StageMetrics>,
    events: VecDeque<TelemetryEvent>,
}

impl TelemetryState {
    fn record(
        &mut self,
        observation: TelemetryObservation,
        config: &TelemetryConfig,
    ) -> RecordDisposition {
        self.sequence = self.sequence.saturating_add(1);
        self.observations = self.observations.saturating_add(1);

        match observation.outcome {
            TelemetryOutcome::Succeeded => {
                self.succeeded = self.succeeded.saturating_add(1);
            }
            TelemetryOutcome::Failed => {
                self.failed = self.failed.saturating_add(1);
            }
            TelemetryOutcome::Cancelled => {
                self.cancelled = self.cancelled.saturating_add(1);
            }
        }

        self.total_duration_nanos = self
            .total_duration_nanos
            .saturating_add(duration_nanos_saturating(observation.duration));
        self.input_bytes = self.input_bytes.saturating_add(observation.input_bytes);
        self.output_bytes = self.output_bytes.saturating_add(observation.output_bytes);
        self.stages
            .entry(observation.stage)
            .or_default()
            .record(observation);

        if self.sequence % config.sample_every != 0 {
            self.sampled_out = self.sampled_out.saturating_add(1);
            return RecordDisposition::SampledOut;
        }

        if config.event_capacity == 0 {
            self.unretained_events = self.unretained_events.saturating_add(1);
            return RecordDisposition::RetentionDisabled;
        }

        let event = TelemetryEvent {
            sequence: self.sequence,
            stage: observation.stage,
            outcome: observation.outcome,
            failure: observation.failure,
            duration_nanos: duration_nanos_saturating(observation.duration),
            input_bytes: observation.input_bytes,
            output_bytes: observation.output_bytes,
        };

        let evicted = if self.events.len() >= config.event_capacity {
            let _ = self.events.pop_front();
            self.evicted_events = self.evicted_events.saturating_add(1);
            true
        } else {
            false
        };

        self.events.push_back(event);

        if evicted {
            RecordDisposition::RetainedAfterEviction
        } else {
            RecordDisposition::Retained
        }
    }

    fn snapshot(&self, include_events: bool) -> TelemetrySnapshot {
        TelemetrySnapshot {
            observations: self.observations,
            succeeded: self.succeeded,
            failed: self.failed,
            cancelled: self.cancelled,
            total_duration_nanos: self.total_duration_nanos,
            input_bytes: self.input_bytes,
            output_bytes: self.output_bytes,
            sampled_out: self.sampled_out,
            evicted_events: self.evicted_events,
            unretained_events: self.unretained_events,
            stages: self.stages.clone(),
            events: if include_events {
                self.events.iter().copied().collect()
            } else {
                Vec::new()
            },
        }
    }
}

/// In-process telemetry collector.
///
/// Each collector owns its own state. There is no global singleton and no
/// background worker. A collector may be shared between threads because its
/// state is protected by a mutex. Lock poisoning is recovered because
/// telemetry is observational and must not turn a prior panic into a new
/// compilation panic.
#[derive(Debug)]
pub struct Telemetry {
    config: TelemetryConfig,
    state: Mutex<TelemetryState>,
}

impl Telemetry {
    /// Create a collector with validated configuration.
    pub fn new(config: TelemetryConfig) -> Result<Self, TelemetryConfigError> {
        config.validate()?;
        Ok(Self {
            config,
            state: Mutex::new(TelemetryState::default()),
        })
    }

    /// Return the immutable collection/export policy.
    #[must_use]
    pub const fn config(&self) -> &TelemetryConfig {
        &self.config
    }

    /// Whether telemetry collection is enabled.
    #[must_use]
    pub const fn is_enabled(&self) -> bool {
        !matches!(self.config.policy, TelemetryPolicy::Disabled)
    }

    /// Record one completed operation.
    ///
    /// This method does not return a compilation error. Its disposition tells
    /// callers whether the event was retained, sampled out, or disabled.
    pub fn record(&self, observation: TelemetryObservation) -> RecordDisposition {
        if !self.is_enabled() {
            return RecordDisposition::Disabled;
        }

        self.lock_state().record(observation, &self.config)
    }

    /// Obtain a snapshot appropriate for the configured policy.
    ///
    /// `LocalOnly` includes bounded event history. Aggregate and remote modes
    /// expose counters only. `Disabled` returns an empty snapshot.
    #[must_use]
    pub fn snapshot(&self) -> TelemetrySnapshot {
        match self.config.policy {
            TelemetryPolicy::Disabled => TelemetrySnapshot::default(),
            TelemetryPolicy::LocalOnly => self.lock_state().snapshot(true),
            TelemetryPolicy::Aggregated | TelemetryPolicy::ExplicitRemote => {
                self.lock_state().snapshot(false)
            }
        }
    }

    /// Export an aggregate-only snapshot through a caller-supplied sink.
    ///
    /// This method does not implement networking or persistence. For remote
    /// export, the caller must provide a permit obtained after the repository's
    /// security/capability layer has authorized the destination.
    pub fn export_to(
        &self,
        sink: &dyn TelemetrySink,
        remote_permit: Option<&RemoteExportPermit>,
    ) -> Result<(), TelemetryExportError> {
        match self.config.policy {
            TelemetryPolicy::Disabled => {
                return Err(TelemetryExportError::ExportDisabled);
            }
            TelemetryPolicy::LocalOnly => {
                return Err(TelemetryExportError::LocalOnlyPolicy);
            }
            TelemetryPolicy::Aggregated => {}
            TelemetryPolicy::ExplicitRemote if remote_permit.is_some() => {}
            TelemetryPolicy::ExplicitRemote => {
                return Err(TelemetryExportError::RemoteAuthorizationRequired);
            }
        }

        let snapshot = self.lock_state().snapshot(false).aggregate_only();
        sink.export(&snapshot)
    }

    fn lock_state(&self) -> MutexGuard<'_, TelemetryState> {
        self.state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}

fn duration_nanos_saturating(duration: Duration) -> u64 {
    u64::try_from(duration.as_nanos()).unwrap_or(u64::MAX)
}

/// Explicit marker that the caller has authorized remote telemetry export.
///
/// Construct this only after checking the applicable authorization and
/// capability policy. It is a coordination marker, not a cryptographic token.
#[derive(Clone, Copy, Debug)]
pub struct RemoteExportPermit {
    _private: (),
}

impl RemoteExportPermit {
    /// Record an explicit caller opt-in to remote telemetry export.
    ///
    /// The integration layer must perform destination authorization before
    /// calling this function. This method intentionally does not discover,
    /// authenticate, or connect to a remote destination.
    #[must_use]
    pub const fn explicit_opt_in() -> Self {
        Self { _private: () }
    }
}

/// Sink for aggregate telemetry snapshots.
///
/// Implementations may persist or export snapshots. The implementation is
/// responsible for its own destination security, timeouts, retry limits,
/// transport privacy, and error handling. A sink must not mutate compiler
/// state or reinterpret telemetry as program semantics.
pub trait TelemetrySink: Send + Sync {
    /// Export one aggregate-only snapshot.
    fn export(&self, snapshot: &TelemetrySnapshot) -> Result<(), TelemetryExportError>;
}

/// Errors produced by explicit telemetry export.
///
/// Export errors are separate from backend compilation errors. Callers should
/// report or suppress them according to observability policy; they must not
/// retroactively turn a successful compilation into a failed compilation.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum TelemetryExportError {
    /// Collection/export policy disables export.
    ExportDisabled,
    /// The collector is configured for local snapshots only.
    LocalOnlyPolicy,
    /// Remote export was requested without an explicit permit.
    RemoteAuthorizationRequired,
    /// The sink rejected or failed to export the snapshot.
    SinkFailure(String),
}

impl fmt::Display for TelemetryExportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExportDisabled => formatter.write_str("telemetry export is disabled"),
            Self::LocalOnlyPolicy => {
                formatter.write_str("telemetry policy permits local snapshots only")
            }
            Self::RemoteAuthorizationRequired => {
                formatter.write_str("remote telemetry export requires explicit authorization")
            }
            Self::SinkFailure(message) => write!(formatter, "telemetry sink failed: {message}"),
        }
    }
}

impl std::error::Error for TelemetryExportError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn observation(stage: TelemetryStage, outcome: TelemetryOutcome) -> TelemetryObservation {
        TelemetryObservation::new(stage, outcome)
            .with_duration(Duration::from_nanos(17))
            .with_byte_counts(3, 5)
    }

    #[derive(Default)]
    struct TestSink {
        snapshots: Mutex<Vec<TelemetrySnapshot>>,
    }

    impl TelemetrySink for TestSink {
        fn export(&self, snapshot: &TelemetrySnapshot) -> Result<(), TelemetryExportError> {
            self.snapshots
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .push(snapshot.clone());
            Ok(())
        }
    }

    #[test]
    fn rejects_zero_sampling_interval() {
        let config = TelemetryConfig {
            sample_every: 0,
            ..TelemetryConfig::default()
        };

        assert_eq!(
            Telemetry::new(config).err(),
            Some(TelemetryConfigError::ZeroSamplingInterval)
        );
    }

    #[test]
    fn disabled_policy_does_not_record() {
        let telemetry = Telemetry::new(TelemetryConfig {
            policy: TelemetryPolicy::Disabled,
            ..TelemetryConfig::default()
        })
        .expect("valid configuration");

        assert_eq!(
            telemetry.record(observation(
                TelemetryStage::Emission,
                TelemetryOutcome::Succeeded
            )),
            RecordDisposition::Disabled
        );
        assert_eq!(telemetry.snapshot(), TelemetrySnapshot::default());
    }

    #[test]
    fn aggregate_metrics_update_when_event_retention_is_zero() {
        let telemetry = Telemetry::new(TelemetryConfig {
            event_capacity: 0,
            ..TelemetryConfig::default()
        })
        .expect("valid configuration");

        assert_eq!(
            telemetry.record(observation(
                TelemetryStage::Emission,
                TelemetryOutcome::Succeeded
            )),
            RecordDisposition::RetentionDisabled
        );

        let snapshot = telemetry.snapshot();
        assert_eq!(snapshot.observations, 1);
        assert_eq!(snapshot.succeeded, 1);
        assert_eq!(snapshot.total_duration_nanos, 17);
        assert_eq!(snapshot.input_bytes, 3);
        assert_eq!(snapshot.output_bytes, 5);
        assert_eq!(snapshot.unretained_events, 1);
        assert!(snapshot.events.is_empty());
    }

    #[test]
    fn event_history_is_bounded_and_oldest_event_is_evicted() {
        let telemetry = Telemetry::new(TelemetryConfig {
            event_capacity: 2,
            ..TelemetryConfig::default()
        })
        .expect("valid configuration");

        for _ in 0..3 {
            telemetry.record(observation(
                TelemetryStage::Validation,
                TelemetryOutcome::Succeeded,
            ));
        }

        let snapshot = telemetry.snapshot();
        assert_eq!(snapshot.observations, 3);
        assert_eq!(snapshot.events.len(), 2);
        assert_eq!(snapshot.events[0].sequence, 2);
        assert_eq!(snapshot.events[1].sequence, 3);
        assert_eq!(snapshot.evicted_events, 1);
    }

    #[test]
    fn sampling_is_deterministic_and_does_not_skip_aggregates() {
        let telemetry = Telemetry::new(TelemetryConfig {
            sample_every: 2,
            event_capacity: 10,
            ..TelemetryConfig::default()
        })
        .expect("valid configuration");

        let first = telemetry.record(observation(
            TelemetryStage::Lowering,
            TelemetryOutcome::Succeeded,
        ));
        let second = telemetry.record(observation(
            TelemetryStage::Lowering,
            TelemetryOutcome::Failed,
        ));

        assert_eq!(first, RecordDisposition::SampledOut);
        assert_eq!(second, RecordDisposition::Retained);

        let snapshot = telemetry.snapshot();
        assert_eq!(snapshot.observations, 2);
        assert_eq!(snapshot.succeeded, 1);
        assert_eq!(snapshot.failed, 1);
        assert_eq!(snapshot.sampled_out, 1);
        assert_eq!(snapshot.events.len(), 1);
        assert_eq!(snapshot.events[0].sequence, 2);
    }

    #[test]
    fn aggregate_policy_never_exposes_event_history() {
        let telemetry = Telemetry::new(TelemetryConfig {
            policy: TelemetryPolicy::Aggregated,
            ..TelemetryConfig::default()
        })
        .expect("valid configuration");

        telemetry.record(observation(
            TelemetryStage::Linking,
            TelemetryOutcome::Succeeded,
        ));

        assert!(telemetry.snapshot().events.is_empty());
    }

    #[test]
    fn aggregate_export_requires_an_exportable_policy() {
        let telemetry = Telemetry::new(TelemetryConfig {
            policy: TelemetryPolicy::LocalOnly,
            ..TelemetryConfig::default()
        })
        .expect("valid configuration");
        let sink = TestSink::default();

        assert_eq!(
            telemetry.export_to(&sink, None),
            Err(TelemetryExportError::LocalOnlyPolicy)
        );
        assert!(sink
            .snapshots
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .is_empty());
    }

    #[test]
    fn remote_export_requires_explicit_permit_and_exports_aggregates_only() {
        let telemetry = Telemetry::new(TelemetryConfig {
            policy: TelemetryPolicy::ExplicitRemote,
            ..TelemetryConfig::default()
        })
        .expect("valid configuration");
        let sink = TestSink::default();

        telemetry.record(observation(
            TelemetryStage::Toolchain,
            TelemetryOutcome::Failed,
        ));

        assert_eq!(
            telemetry.export_to(&sink, None),
            Err(TelemetryExportError::RemoteAuthorizationRequired)
        );

        telemetry
            .export_to(&sink, Some(&RemoteExportPermit::explicit_opt_in()))
            .expect("explicitly authorized export");

        let snapshots = sink
            .snapshots
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        assert_eq!(snapshots.len(), 1);
        assert_eq!(snapshots[0].observations, 1);
        assert!(snapshots[0].events.is_empty());
    }

    #[test]
    fn failure_categories_are_typed_and_do_not_store_error_text() {
        let telemetry = Telemetry::new(TelemetryConfig::default()).expect("valid configuration");

        telemetry.record(
            observation(TelemetryStage::ArtifactValidation, TelemetryOutcome::Failed)
                .with_failure(TelemetryFailureKind::InvalidArtifact),
        );

        let snapshot = telemetry.snapshot();
        assert_eq!(snapshot.failed, 1);
        assert_eq!(
            snapshot.events[0].failure,
            Some(TelemetryFailureKind::InvalidArtifact)
        );
    }

    #[test]
    fn counters_saturate_instead_of_wrapping() {
        let mut metrics = StageMetrics {
            observations: u64::MAX,
            succeeded: u64::MAX,
            total_duration_nanos: u64::MAX,
            input_bytes: u64::MAX,
            output_bytes: u64::MAX,
            ..StageMetrics::default()
        };

        metrics.record(
            TelemetryObservation::new(
                TelemetryStage::Emission,
                TelemetryOutcome::Succeeded,
            )
            .with_duration(Duration::from_nanos(1))
            .with_byte_counts(1, 1),
        );

        assert_eq!(metrics.observations, u64::MAX);
        assert_eq!(metrics.succeeded, u64::MAX);
        assert_eq!(metrics.total_duration_nanos, u64::MAX);
        assert_eq!(metrics.input_bytes, u64::MAX);
        assert_eq!(metrics.output_bytes, u64::MAX);
    }
}
