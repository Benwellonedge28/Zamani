//! Zamani Quantum Error Correction — syndrome representation and streaming.
//!
//! This module owns the representation of one validated stabilizer-measurement
//! round and the deterministic conversion of consecutive rounds into
//! space-time detection events.
//!
//! # Contract
//!
//! `syndrome.rs` owns:
//! - stable stabilizer identifiers;
//! - measurement rounds and timestamps;
//! - bounded measurement confidence;
//! - one complete syndrome snapshot;
//! - deterministic XOR detection-event generation;
//! - an incremental `SyndromeSource` contract;
//! - a lightweight stateful processor for consecutive rounds.
//!
//! It does **not** own:
//! - surface-code topology or physical coordinates (`surface_code.rs`);
//! - decoding-graph construction (`decoding_graph.rs`);
//! - decoder policy (`decoder.rs`, `mwpm.rs`, `union_find.rs`);
//! - allocation enforcement (`memory.rs`);
//! - runtime resource accounting (`resources.rs`);
//! - transport, threads, queues, or QPU I/O (`streaming.rs`, `backend.rs`,
//!   `qpu_adapter.rs`).
//!
//! # Integration
//!
//! ```text
//! simulator / QPU / replay
//!          │
//!          ▼
//!   SyndromeSource
//!          │
//!          ▼
//!       Syndrome
//!          │ validate + limits + cancellation
//!          ▼
//!   DetectionEvent
//!          │
//!          ▼
//!   decoding_graph.rs
//!          │
//!          ▼
//!       decoders
//! ```
//!
//! `QecLimits` is the sole declarative production limit source. `QecError` is
//! the public error boundary. No fixed production ceiling is maintained here.
//!
//! Rust compatibility target: Rust 1.97.1.

use core::fmt;
use std::collections::BTreeMap;

use super::cancellation::CancellationToken;
use super::errors::{QecError, QecResult, NumericalOperation, ResourceKind};
use super::limits::QecLimits;

/// Maximum representable measurement round.
///
/// `u64::MAX` is reserved so `next()` can never wrap into a valid round.
pub const MAX_MEASUREMENT_ROUND: u64 = u64::MAX - 1;

/// Maximum representable timestamp.
///
/// `u64::MAX` is reserved for sentinel/invalid use.
pub const MAX_TIMESTAMP: u64 = u64::MAX - 1;

/// Measurement confidence in basis points:
/// `0 = 0%`, `10_000 = 100%`.
pub const MAX_CONFIDENCE_BASIS_POINTS: u16 = 10_000;

/// Conservative accounting estimate for one stored measurement.
const ESTIMATED_BYTES_PER_MEASUREMENT: u64 = 64;

/// Conservative fixed accounting overhead for a syndrome.
const ESTIMATED_SYNDROME_OVERHEAD_BYTES: u64 = 128;

// ============================================================================
// Stabilizer identifier
// ============================================================================

/// Stable identifier for a stabilizer measurement.
///
/// The identifier is intentionally independent of the Pauli/stabilizer
/// algebra in `stabilizer.rs`. Physical/topological coordinates are resolved
/// later by `surface_code.rs` / `decoding_graph.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StabilizerId(pub usize);

impl StabilizerId {
    /// Creates an identifier.
    #[must_use]
    pub const fn new(id: usize) -> Self {
        Self(id)
    }

    /// Returns the numeric identifier.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }
}

impl fmt::Display for StabilizerId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "s{}", self.0)
    }
}

// ============================================================================
// Round and timestamp
// ============================================================================

/// Validated measurement-round number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MeasurementRound(u64);

impl MeasurementRound {
    /// Creates a validated round number.
    pub fn new(round: u64) -> Result<Self, SyndromeError> {
        if round > MAX_MEASUREMENT_ROUND {
            return Err(SyndromeError::InvalidRound { round });
        }

        Ok(Self(round))
    }

    /// Returns the numeric round.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Returns the immediately following round without wrapping.
    pub fn next(self) -> Result<Self, SyndromeError> {
        let next = self
            .0
            .checked_add(1)
            .ok_or(SyndromeError::RoundOverflow)?;

        Self::new(next)
    }
}

/// Validated backend-independent measurement timestamp.
///
/// The unit is defined by the execution backend. Only ordering is interpreted
/// here; physical time units belong to the backend contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MeasurementTimestamp(u64);

impl MeasurementTimestamp {
    /// Creates a validated timestamp.
    pub fn new(timestamp: u64) -> Result<Self, SyndromeError> {
        if timestamp > MAX_TIMESTAMP {
            return Err(SyndromeError::InvalidTimestamp { timestamp });
        }

        Ok(Self(timestamp))
    }

    /// Returns the timestamp value.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

// ============================================================================
// Confidence
// ============================================================================

/// Exact measurement confidence represented as basis points.
///
/// Integer representation avoids NaN/infinity and makes equality,
/// serialization, and deterministic replay straightforward.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MeasurementConfidence(u16);

impl MeasurementConfidence {
    /// Zero confidence.
    pub const ZERO: Self = Self(0);

    /// Full confidence.
    pub const FULL: Self = Self(MAX_CONFIDENCE_BASIS_POINTS);

    /// Creates confidence from basis points.
    pub fn from_basis_points(value: u16) -> Result<Self, SyndromeError> {
        if value > MAX_CONFIDENCE_BASIS_POINTS {
            return Err(SyndromeError::InvalidConfidence { value });
        }

        Ok(Self(value))
    }

    /// Creates confidence from an integer percentage.
    ///
    /// For example, `95` becomes `9500` basis points.
    pub fn from_probability_percent(percent: u8) -> Result<Self, SyndromeError> {
        let basis_points = (percent as u16)
            .checked_mul(100)
            .ok_or(SyndromeError::InvalidConfidence {
                value: u16::MAX,
            })?;

        Self::from_basis_points(basis_points)
    }

    /// Returns basis points.
    #[must_use]
    pub const fn basis_points(self) -> u16 {
        self.0
    }

    /// Returns the value as a probability in `[0.0, 1.0]`.
    #[must_use]
    pub fn as_probability(self) -> f64 {
        f64::from(self.0) / f64::from(MAX_CONFIDENCE_BASIS_POINTS)
    }

    /// Returns the conservative confidence for two measurements.
    #[must_use]
    pub const fn min(self, other: Self) -> Self {
        if self.0 <= other.0 {
            self
        } else {
            other
        }
    }
}

// ============================================================================
// Measurement
// ============================================================================

/// One validated stabilizer measurement.
///
/// `value == true` represents a non-trivial syndrome bit.
/// `value == false` represents a trivial syndrome bit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SyndromeMeasurement {
    stabilizer: StabilizerId,
    value: bool,
    confidence: MeasurementConfidence,
}

impl SyndromeMeasurement {
    /// Creates a measurement.
    #[must_use]
    pub const fn new(
        stabilizer: StabilizerId,
        value: bool,
        confidence: MeasurementConfidence,
    ) -> Self {
        Self {
            stabilizer,
            value,
            confidence,
        }
    }

    /// Returns the stabilizer identifier.
    #[must_use]
    pub const fn stabilizer(self) -> StabilizerId {
        self.stabilizer
    }

    /// Returns the syndrome bit.
    #[must_use]
    pub const fn value(self) -> bool {
        self.value
    }

    /// Returns measurement confidence.
    #[must_use]
    pub const fn confidence(self) -> MeasurementConfidence {
        self.confidence
    }
}

// ============================================================================
// Construction options
// ============================================================================

/// Construction/validation policy for syndrome input.
#[derive(Debug, Clone, Copy)]
pub struct SyndromeOptions {
    /// Canonical declarative QEC resource policy.
    pub limits: QecLimits,

    /// Reject an empty syndrome when set.
    pub require_non_empty: bool,
}

impl Default for SyndromeOptions {
    fn default() -> Self {
        Self {
            limits: QecLimits::default(),
            require_non_empty: false,
        }
    }
}

impl SyndromeOptions {
    /// Creates options from the canonical limits.
    #[must_use]
    pub const fn with_limits(limits: QecLimits) -> Self {
        Self {
            limits,
            require_non_empty: false,
        }
    }

    /// Requires at least one stabilizer measurement.
    #[must_use]
    pub const fn require_non_empty(mut self) -> Self {
        self.require_non_empty = true;
        self
    }

    /// Validates the policy without constructing a syndrome.
    pub fn validate(&self) -> QecResult<()> {
        self.limits.validate().map_err(|error| {
            QecError::invalid_input(format!(
                "invalid QEC limits for syndrome processing: {error}"
            ))
        })
    }
}

// ============================================================================
// Syndrome snapshot
// ============================================================================

/// Complete stabilizer-measurement snapshot for one round.
///
/// `BTreeMap` gives canonical stabilizer-ID ordering independent of insertion
/// order. A missing stabilizer is never interpreted as a zero measurement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Syndrome {
    round: MeasurementRound,
    timestamp: MeasurementTimestamp,
    measurements: BTreeMap<StabilizerId, SyndromeMeasurement>,
    limits: QecLimits,
}

/// Architectural alias used by QPU/replay integrations that call a snapshot
/// a `SyndromeSnapshot`.
pub type SyndromeSnapshot = Syndrome;

impl Syndrome {
    /// Creates an empty syndrome with canonical default limits.
    ///
    /// `QecLimits::default()` is the repository-wide default policy and is
    /// expected to be valid. Explicitly configured callers should use
    /// `new_with_limits`.
    #[must_use]
    pub fn new(
        round: MeasurementRound,
        timestamp: MeasurementTimestamp,
    ) -> Self {
        Self {
            round,
            timestamp,
            measurements: BTreeMap::new(),
            limits: QecLimits::default(),
        }
    }

    /// Creates an empty syndrome using explicit validated limits.
    pub fn new_with_limits(
        round: MeasurementRound,
        timestamp: MeasurementTimestamp,
        limits: QecLimits,
    ) -> QecResult<Self> {
        limits.validate().map_err(|error| {
            QecError::invalid_input(format!("invalid QEC limits: {error}"))
        })?;

        Ok(Self {
            round,
            timestamp,
            measurements: BTreeMap::new(),
            limits,
        })
    }

    /// Creates a syndrome from a measurement iterator.
    ///
    /// Validation and limits are applied before every mutation.
    pub fn from_measurements<I>(
        round: MeasurementRound,
        timestamp: MeasurementTimestamp,
        measurements: I,
        options: SyndromeOptions,
    ) -> QecResult<Self>
    where
        I: IntoIterator<Item = SyndromeMeasurement>,
    {
        options.validate()?;

        let mut syndrome =
            Self::new_with_limits(round, timestamp, options.limits)?;

        for measurement in measurements {
            syndrome.insert(measurement)?;
        }

        if options.require_non_empty && syndrome.is_empty() {
            return Err(QecError::invalid_syndrome(
                "syndrome must contain at least one stabilizer measurement",
            ));
        }

        syndrome.preflight()?;

        Ok(syndrome)
    }

    /// Returns the measurement round.
    #[must_use]
    pub const fn round(&self) -> MeasurementRound {
        self.round
    }

    /// Returns the timestamp.
    #[must_use]
    pub const fn timestamp(&self) -> MeasurementTimestamp {
        self.timestamp
    }

    /// Returns the active canonical resource policy.
    #[must_use]
    pub const fn limits(&self) -> QecLimits {
        self.limits
    }

    /// Returns the number of measurements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.measurements.len()
    }

    /// Returns whether the syndrome contains no measurements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.measurements.is_empty()
    }

    /// Returns a conservative memory estimate.
    pub fn estimated_memory_bytes(&self) -> QecResult<u64> {
        let count = u64::try_from(self.measurements.len()).map_err(|_| {
            QecError::numerical_failure(
                NumericalOperation::IntegerConversion,
                "syndrome measurement count does not fit in u64",
            )
        })?;

        count
            .checked_mul(ESTIMATED_BYTES_PER_MEASUREMENT)
            .and_then(|bytes| {
                bytes.checked_add(ESTIMATED_SYNDROME_OVERHEAD_BYTES)
            })
            .ok_or_else(|| {
                QecError::numerical_failure(
                    NumericalOperation::MemorySizeCalculation,
                    "syndrome memory estimate overflowed",
                )
            })
    }

    /// Validates the snapshot against its canonical limits.
    pub fn preflight(&self) -> QecResult<()> {
        let count = self.measurements.len();

        if count > self.limits.max_stabilizers {
            return Err(QecError::resource_limit(
                ResourceKind::Stabilizers,
                count as u128,
                count as u128,
                self.limits.max_stabilizers as u128,
                format!(
                    "syndrome contains {count} stabilizers; limit is {}",
                    self.limits.max_stabilizers
                ),
            ));
        }

        let memory = self.estimated_memory_bytes()?;

        if memory > self.limits.max_memory_bytes {
            return Err(QecError::memory_limit(
                memory,
                memory,
                self.limits.max_memory_bytes,
                format!(
                    "syndrome representation requires approximately {memory} bytes"
                ),
            ));
        }

        Ok(())
    }

    /// Inserts one measurement, rejecting duplicates before mutation.
    pub fn insert(
        &mut self,
        measurement: SyndromeMeasurement,
    ) -> QecResult<()> {
        let stabilizer = measurement.stabilizer();

        if self.measurements.contains_key(&stabilizer) {
            return Err(
                SyndromeError::DuplicateStabilizer { stabilizer }.into()
            );
        }

        let next_len =
            self.measurements.len().checked_add(1).ok_or_else(|| {
                QecError::resource_limit(
                    ResourceKind::Stabilizers,
                    u128::MAX,
                    self.measurements.len() as u128,
                    self.limits.max_stabilizers as u128,
                    "syndrome stabilizer count overflow",
                )
            })?;

        if next_len > self.limits.max_stabilizers {
            return Err(QecError::resource_limit(
                ResourceKind::Stabilizers,
                next_len as u128,
                self.measurements.len() as u128,
                self.limits.max_stabilizers as u128,
                "syndrome stabilizer limit exceeded",
            ));
        }

        let estimated = (next_len as u64)
            .checked_mul(ESTIMATED_BYTES_PER_MEASUREMENT)
            .and_then(|bytes| {
                bytes.checked_add(ESTIMATED_SYNDROME_OVERHEAD_BYTES)
            })
            .ok_or_else(|| {
                QecError::numerical_failure(
                    NumericalOperation::MemorySizeCalculation,
                    "syndrome memory estimate overflowed",
                )
            })?;

        if estimated > self.limits.max_memory_bytes {
            return Err(QecError::memory_limit(
                estimated,
                estimated,
                self.limits.max_memory_bytes,
                "syndrome insertion would exceed the configured memory budget",
            ));
        }

        self.measurements.insert(stabilizer, measurement);

        Ok(())
    }

    /// Returns a measurement for a stabilizer.
    #[must_use]
    pub fn get(
        &self,
        stabilizer: StabilizerId,
    ) -> Option<SyndromeMeasurement> {
        self.measurements.get(&stabilizer).copied()
    }

    /// Returns a syndrome bit, if the stabilizer is present.
    #[must_use]
    pub fn value(
        &self,
        stabilizer: StabilizerId,
    ) -> Option<bool> {
        self.get(stabilizer)
            .map(SyndromeMeasurement::value)
    }

    /// Iterates over measurements in deterministic stabilizer-ID order.
    pub fn measurements(
        &self,
    ) -> impl Iterator<Item = &SyndromeMeasurement> {
        self.measurements.values()
    }

    /// Iterates over measured stabilizer IDs in deterministic order.
    pub fn stabilizer_ids(
        &self,
    ) -> impl Iterator<Item = StabilizerId> + '_ {
        self.measurements.keys().copied()
    }

    /// Returns all non-trivial measured stabilizers in deterministic order.
    #[must_use]
    pub fn active_stabilizers(&self) -> Vec<StabilizerId> {
        self.measurements
            .values()
            .filter(|measurement| measurement.value())
            .map(SyndromeMeasurement::stabilizer)
            .collect()
    }

    /// Returns true when every measured stabilizer is trivial.
    #[must_use]
    pub fn is_trivial(&self) -> bool {
        self.measurements
            .values()
            .all(|measurement| !measurement.value())
    }

    /// Validates that two snapshots describe exactly the same stabilizer
    /// domain. Measurement values may differ; the domain may not.
    pub fn validate_compatible_with(
        &self,
        previous: &Self,
    ) -> QecResult<()> {
        if self.measurements.len() != previous.measurements.len()
            || self.measurements.keys() != previous.measurements.keys()
        {
            return Err(SyndromeError::StabilizerSetMismatch.into());
        }

        Ok(())
    }

    /// Generates detection events against the immediately previous round.
    ///
    /// `D(t) = S(t) XOR S(t-1)`.
    pub fn detection_events_against(
        &self,
        previous: &Self,
    ) -> QecResult<Vec<DetectionEvent>> {
        let token = CancellationToken::new();

        self.detection_events_against_with_cancellation(
            previous,
            &token,
        )
    }

    /// Cancellation-aware detection-event generation used by streaming and
    /// decoder pipelines.
    pub fn detection_events_against_with_cancellation(
        &self,
        previous: &Self,
        cancellation: &CancellationToken,
    ) -> QecResult<Vec<DetectionEvent>> {
        cancellation.check()?;

        let expected = previous
            .round
            .next()
            .map_err(QecError::from)?;

        if self.round != expected {
            return Err(
                SyndromeError::NonConsecutiveRounds {
                    previous: previous.round.value(),
                    current: self.round.value(),
                }
                .into(),
            );
        }

        if self.timestamp.value() < previous.timestamp.value() {
            return Err(
                SyndromeError::TimestampRegression {
                    previous: previous.timestamp.value(),
                    current: self.timestamp.value(),
                }
                .into(),
            );
        }

        self.validate_compatible_with(previous)?;

        let mut events = Vec::new();

        for stabilizer in self.measurements.keys().copied() {
            cancellation.poll()?;

            let current =
                self.measurements.get(&stabilizer).ok_or_else(|| {
                    QecError::invalid_syndrome(
                        "current syndrome stabilizer domain changed during validation",
                    )
                })?;

            let prior =
                previous.measurements.get(&stabilizer).ok_or_else(|| {
                    QecError::invalid_syndrome(
                        "previous syndrome stabilizer domain changed during validation",
                    )
                })?;

            if current.value() ^ prior.value() {
                let next_len =
                    events.len().checked_add(1).ok_or_else(|| {
                        QecError::resource_limit(
                            ResourceKind::SyndromeEvents,
                            u128::MAX,
                            events.len() as u128,
                            self.limits.max_syndrome_events as u128,
                            "detection-event count overflow",
                        )
                    })?;

                if next_len > self.limits.max_syndrome_events {
                    return Err(
                        SyndromeError::TooManyDetectionEvents {
                            requested: next_len,
                            limit: self.limits.max_syndrome_events,
                        }
                        .into(),
                    );
                }

                events.push(DetectionEvent {
                    round: self.round,
                    timestamp: self.timestamp,
                    stabilizer,
                    value: true,
                    confidence: current
                        .confidence()
                        .min(prior.confidence()),
                });
            }
        }

        Ok(events)
    }

    /// Returns whether the syndrome differs from another snapshot in domain
    /// or syndrome bits.
    #[must_use]
    pub fn differs_from(&self, previous: &Self) -> bool {
        if self.measurements.len() != previous.measurements.len()
            || self.measurements.keys() != previous.measurements.keys()
        {
            return true;
        }

        self.measurements.iter().any(|(id, current)| {
            previous
                .measurements
                .get(id)
                .map_or(true, |prior| {
                    current.value() != prior.value()
                })
        })
    }
}

// ============================================================================
// Detection event
// ============================================================================

/// A stabilizer whose syndrome bit changed between two consecutive rounds.
///
/// The event is intentionally expressed in syndrome coordinates. Mapping the
/// stabilizer ID to physical/topological coordinates is owned by the code and
/// decoding-graph layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DetectionEvent {
    round: MeasurementRound,
    timestamp: MeasurementTimestamp,
    stabilizer: StabilizerId,
    value: bool,
    confidence: MeasurementConfidence,
}

impl DetectionEvent {
    /// Returns the event round.
    #[must_use]
    pub const fn round(self) -> MeasurementRound {
        self.round
    }

    /// Returns the event timestamp.
    #[must_use]
    pub const fn timestamp(self) -> MeasurementTimestamp {
        self.timestamp
    }

    /// Returns the changed stabilizer.
    #[must_use]
    pub const fn stabilizer(self) -> StabilizerId {
        self.stabilizer
    }

    /// Returns the event bit.
    ///
    /// Valid detection events are always true because false XOR false and
    /// true XOR true do not generate an event.
    #[must_use]
    pub const fn value(self) -> bool {
        self.value
    }

    /// Returns conservative event confidence.
    #[must_use]
    pub const fn confidence(self) -> MeasurementConfidence {
        self.confidence
    }

    /// Returns `(stabilizer_id, round)` syndrome-space coordinates.
    #[must_use]
    pub const fn coordinate(self) -> (usize, u64) {
        (
            self.stabilizer.index(),
            self.round.value(),
        )
    }
}

// ============================================================================
// Incremental source contract
// ============================================================================

/// Incremental source of syndrome snapshots.
///
/// Transport, asynchronous execution, queues, QPU communication, and worker
/// management remain outside this trait.
pub trait SyndromeSource {
    /// Returns the next snapshot, or `None` at end-of-stream.
    fn next_syndrome(&mut self) -> QecResult<Option<Syndrome>>;
}

// ============================================================================
// Incremental processor
// ============================================================================

/// Lightweight consecutive-round syndrome processor.
///
/// This is intentionally not a queue or scheduler. It is the deterministic
/// state machine used by `streaming.rs`, replay, simulation, and QPU result
/// processing.
#[derive(Debug, Clone)]
pub struct SyndromeProcessor {
    limits: QecLimits,
    cancellation: CancellationToken,
    previous: Option<Syndrome>,
    rounds_processed: usize,
    events_generated: usize,
}

impl SyndromeProcessor {
    /// Creates a processor using canonical default policy.
    pub fn new() -> QecResult<Self> {
        Self::with_limits(
            QecLimits::default(),
            CancellationToken::new(),
        )
    }

    /// Creates a processor with explicit policy and cancellation.
    pub fn with_limits(
        limits: QecLimits,
        cancellation: CancellationToken,
    ) -> QecResult<Self> {
        limits.validate().map_err(|error| {
            QecError::invalid_input(format!(
                "invalid QEC limits: {error}"
            ))
        })?;

        Ok(Self {
            limits,
            cancellation,
            previous: None,
            rounds_processed: 0,
            events_generated: 0,
        })
    }

    /// Returns configured limits.
    #[must_use]
    pub const fn limits(&self) -> QecLimits {
        self.limits
    }

    /// Returns processed round count.
    #[must_use]
    pub const fn rounds_processed(&self) -> usize {
        self.rounds_processed
    }

    /// Returns cumulative generated event count.
    #[must_use]
    pub const fn events_generated(&self) -> usize {
        self.events_generated
    }

    /// Returns the current baseline, if one exists.
    #[must_use]
    pub fn previous(&self) -> Option<&Syndrome> {
        self.previous.as_ref()
    }

    /// Pushes one snapshot.
    ///
    /// The first snapshot establishes the baseline and produces no events.
    pub fn push(
        &mut self,
        syndrome: Syndrome,
    ) -> QecResult<Vec<DetectionEvent>> {
        self.cancellation.check()?;

        let next_round_count =
            self.rounds_processed.checked_add(1).ok_or_else(|| {
                QecError::resource_limit(
                    ResourceKind::MeasurementRounds,
                    u128::MAX,
                    self.rounds_processed as u128,
                    self.limits.max_rounds as u128,
                    "measurement-round counter overflow",
                )
            })?;

        if next_round_count > self.limits.max_rounds {
            return Err(QecError::resource_limit(
                ResourceKind::MeasurementRounds,
                next_round_count as u128,
                self.rounds_processed as u128,
                self.limits.max_rounds as u128,
                "maximum measurement-round limit exceeded",
            ));
        }

        syndrome.preflight()?;

        let events = if let Some(previous) = self.previous.as_ref() {
            syndrome.detection_events_against_with_cancellation(
                previous,
                &self.cancellation,
            )?
        } else {
            Vec::new()
        };

        let new_total =
            self.events_generated.checked_add(events.len()).ok_or_else(
                || {
                    QecError::resource_limit(
                        ResourceKind::SyndromeEvents,
                        u128::MAX,
                        self.events_generated as u128,
                        self.limits.max_syndrome_events as u128,
                        "detection-event counter overflow",
                    )
                },
            )?;

        if new_total > self.limits.max_syndrome_events {
            return Err(QecError::resource_limit(
                ResourceKind::SyndromeEvents,
                new_total as u128,
                self.events_generated as u128,
                self.limits.max_syndrome_events as u128,
                "maximum cumulative detection-event limit exceeded",
            ));
        }

        self.previous = Some(syndrome);
        self.rounds_processed = next_round_count;
        self.events_generated = new_total;

        Ok(events)
    }

    /// Processes a source while delivering each event batch to a callback.
    ///
    /// This is the preferred API for `streaming.rs`: it never accumulates the
    /// complete event history in memory.
    pub fn process_source_with<S, F>(
        &mut self,
        source: &mut S,
        mut on_events: F,
    ) -> QecResult<()>
    where
        S: SyndromeSource,
        F: FnMut(&[DetectionEvent]) -> QecResult<()>,
    {
        loop {
            self.cancellation.poll()?;

            let Some(syndrome) = source.next_syndrome()? else {
                break;
            };

            let events = self.push(syndrome)?;
            on_events(&events)?;
        }

        Ok(())
    }

    /// Processes a source and returns all events.
    ///
    /// This compatibility API remains bounded by
    /// `max_syndrome_events`. Callers handling large streams should use
    /// `process_source_with()` to avoid retaining the complete event history.
    pub fn process_source<S>(
        &mut self,
        source: &mut S,
    ) -> QecResult<Vec<DetectionEvent>>
    where
        S: SyndromeSource,
    {
        let mut all_events = Vec::new();
        let max_events = self.limits.max_syndrome_events;

        self.process_source_with(source, |events| {
            let new_len =
                all_events.len().checked_add(events.len()).ok_or_else(
                    || {
                        QecError::resource_limit(
                            ResourceKind::SyndromeEvents,
                            u128::MAX,
                            all_events.len() as u128,
                            max_events as u128,
                            "event collection size overflow",
                        )
                    },
                )?;

            if new_len > max_events {
                return Err(QecError::resource_limit(
                    ResourceKind::SyndromeEvents,
                    new_len as u128,
                    all_events.len() as u128,
                    max_events as u128,
                    "event collection exceeds configured limit",
                ));
            }

            all_events.extend_from_slice(events);

            Ok(())
        })?;

        Ok(all_events)
    }

    /// Starts a new independent stream without resetting cumulative resource
    /// accounting.
    pub fn reset_baseline(&mut self) {
        self.previous = None;
    }

    /// Fully resets the processor's stream state and counters.
    ///
    /// This is useful when the same processor instance is deliberately reused
    /// for an entirely independent execution.
    pub fn reset(&mut self) {
        self.previous = None;
        self.rounds_processed = 0;
        self.events_generated = 0;
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Syndrome-specific errors converted to the canonical `QecError` boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyndromeError {
    InvalidRound {
        round: u64,
    },

    InvalidTimestamp {
        timestamp: u64,
    },

    RoundOverflow,

    InvalidConfidence {
        value: u16,
    },

    DuplicateStabilizer {
        stabilizer: StabilizerId,
    },

    StabilizerSetMismatch,

    NonConsecutiveRounds {
        previous: u64,
        current: u64,
    },

    TimestampRegression {
        previous: u64,
        current: u64,
    },

    TooManyDetectionEvents {
        requested: usize,
        limit: usize,
    },

    InvalidSource {
        message: String,
    },
}

impl fmt::Display for SyndromeError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidRound { round } => {
                write!(formatter, "invalid measurement round {round}")
            }

            Self::InvalidTimestamp { timestamp } => {
                write!(
                    formatter,
                    "invalid measurement timestamp {timestamp}"
                )
            }

            Self::RoundOverflow => {
                formatter.write_str("measurement round overflow")
            }

            Self::InvalidConfidence { value } => {
                write!(
                    formatter,
                    "invalid measurement confidence {value} basis points"
                )
            }

            Self::DuplicateStabilizer { stabilizer } => {
                write!(
                    formatter,
                    "duplicate stabilizer measurement for {stabilizer}"
                )
            }

            Self::StabilizerSetMismatch => {
                formatter.write_str(
                    "syndrome stabilizer sets do not match",
                )
            }

            Self::NonConsecutiveRounds {
                previous,
                current,
            } => {
                write!(
                    formatter,
                    "non-consecutive syndrome rounds: previous={previous}, current={current}"
                )
            }

            Self::TimestampRegression {
                previous,
                current,
            } => {
                write!(
                    formatter,
                    "syndrome timestamp regressed: previous={previous}, current={current}"
                )
            }

            Self::TooManyDetectionEvents {
                requested,
                limit,
            } => {
                write!(
                    formatter,
                    "detection-event count {requested} exceeds limit {limit}"
                )
            }

            Self::InvalidSource { message } => {
                write!(
                    formatter,
                    "invalid syndrome source: {message}"
                )
            }
        }
    }
}

impl std::error::Error for SyndromeError {}

impl From<SyndromeError> for QecError {
    fn from(error: SyndromeError) -> Self {
        match &error {
            SyndromeError::TooManyDetectionEvents {
                requested,
                limit,
            } => QecError::resource_limit(
                ResourceKind::SyndromeEvents,
                *requested as u128,
                0,
                *limit as u128,
                error.to_string(),
            ),

            _ => QecError::invalid_syndrome(error.to_string()),
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn measurement(
        stabilizer: usize,
        value: bool,
    ) -> SyndromeMeasurement {
        SyndromeMeasurement::new(
            StabilizerId::new(stabilizer),
            value,
            MeasurementConfidence::FULL,
        )
    }

    fn syndrome(
        round: u64,
        timestamp: u64,
        values: &[(usize, bool)],
    ) -> Syndrome {
        let mut result = Syndrome::new(
            MeasurementRound::new(round)
                .expect("test round is valid"),
            MeasurementTimestamp::new(timestamp)
                .expect("test timestamp is valid"),
        );

        for &(id, value) in values {
            result
                .insert(measurement(id, value))
                .expect("test measurement is valid");
        }

        result
    }

    #[test]
    fn round_overflow_is_rejected() {
        let round = MeasurementRound::new(
            MAX_MEASUREMENT_ROUND,
        )
        .expect("boundary is valid");

        assert!(round.next().is_err());
    }

    #[test]
    fn duplicate_measurements_are_rejected() {
        let mut value =
            syndrome(0, 0, &[(0, false)]);

        assert!(matches!(
            value.insert(measurement(0, true)),
            Err(QecError::InvalidSyndrome { .. })
        ));
    }

    #[test]
    fn detection_events_are_xor_of_consecutive_rounds() {
        let previous = syndrome(
            0,
            0,
            &[
                (0, false),
                (1, false),
                (2, true),
            ],
        );

        let current = syndrome(
            1,
            1,
            &[
                (0, true),
                (1, false),
                (2, false),
            ],
        );

        let events = current
            .detection_events_against(&previous)
            .expect("valid consecutive rounds");

        assert_eq!(events.len(), 2);
        assert_eq!(
            events[0].stabilizer(),
            StabilizerId::new(0)
        );
        assert_eq!(
            events[1].stabilizer(),
            StabilizerId::new(2)
        );
    }

    #[test]
    fn missing_stabilizer_is_rejected() {
        let previous =
            syndrome(0, 0, &[(0, false), (1, false)]);

        let current =
            syndrome(1, 1, &[(0, false)]);

        assert!(matches!(
            current.detection_events_against(&previous),
            Err(QecError::InvalidSyndrome { .. })
        ));
    }

    #[test]
    fn timestamp_regression_is_rejected() {
        let previous =
            syndrome(0, 100, &[(0, false)]);

        let current =
            syndrome(1, 99, &[(0, true)]);

        assert!(matches!(
            current.detection_events_against(&previous),
            Err(QecError::InvalidSyndrome { .. })
        ));
    }

    #[test]
    fn confidence_is_minimum_of_two_measurements() {
        let mut previous = Syndrome::new(
            MeasurementRound::new(0).unwrap(),
            MeasurementTimestamp::new(0).unwrap(),
        );

        previous
            .insert(SyndromeMeasurement::new(
                StabilizerId::new(0),
                false,
                MeasurementConfidence::from_basis_points(
                    7_000,
                )
                .unwrap(),
            ))
            .unwrap();

        let mut current = Syndrome::new(
            MeasurementRound::new(1).unwrap(),
            MeasurementTimestamp::new(1).unwrap(),
        );

        current
            .insert(SyndromeMeasurement::new(
                StabilizerId::new(0),
                true,
                MeasurementConfidence::from_basis_points(
                    9_000,
                )
                .unwrap(),
            ))
            .unwrap();

        let events = current
            .detection_events_against(&previous)
            .unwrap();

        assert_eq!(
            events[0].confidence().basis_points(),
            7_000
        );
    }

    #[test]
    fn insertion_order_does_not_change_iteration_order() {
        let a = syndrome(
            0,
            0,
            &[
                (2, true),
                (0, false),
                (1, true),
            ],
        );

        let b = syndrome(
            0,
            0,
            &[
                (1, true),
                (2, true),
                (0, false),
            ],
        );

        assert_eq!(a, b);

        assert_eq!(
            a.active_stabilizers(),
            vec![
                StabilizerId::new(1),
                StabilizerId::new(2)
            ]
        );
    }

    #[test]
    fn processor_first_round_is_baseline() {
        let mut processor =
            SyndromeProcessor::new().unwrap();

        let events = processor
            .push(syndrome(
                0,
                0,
                &[(0, false)],
            ))
            .unwrap();

        assert!(events.is_empty());
        assert_eq!(
            processor.rounds_processed(),
            1
        );
    }

    #[test]
    fn processor_rejects_non_consecutive_rounds() {
        let mut processor =
            SyndromeProcessor::new().unwrap();

        processor
            .push(syndrome(
                0,
                0,
                &[(0, false)],
            ))
            .unwrap();

        assert!(matches!(
            processor.push(syndrome(
                2,
                2,
                &[(0, true)],
            )),
            Err(QecError::InvalidSyndrome { .. })
        ));
    }

    #[test]
    fn cancellation_stops_event_generation() {
        let token = CancellationToken::new();
        token.request();

        let previous =
            syndrome(0, 0, &[(0, false)]);

        let current =
            syndrome(1, 1, &[(0, true)]);

        assert!(matches!(
            current
                .detection_events_against_with_cancellation(
                    &previous,
                    &token,
                ),
            Err(QecError::CancellationRequested { .. })
        ));
    }

    #[test]
    fn snapshot_alias_matches_syndrome() {
        let snapshot: SyndromeSnapshot =
            syndrome(0, 0, &[(0, false)]);

        assert_eq!(snapshot.round().value(), 0);
    }

    #[test]
    fn confidence_percent_conversion_is_exact() {
        let confidence =
            MeasurementConfidence::from_probability_percent(95)
                .unwrap();

        assert_eq!(
            confidence.basis_points(),
            9_500
        );
    }

    #[test]
    fn identical_rounds_produce_no_events() {
        let previous =
            syndrome(0, 0, &[(0, false), (1, true)]);

        let current =
            syndrome(1, 1, &[(0, false), (1, true)]);

        let events = current
            .detection_events_against(&previous)
            .unwrap();

        assert!(events.is_empty());
    }

    #[test]
    fn processor_reset_clears_counters() {
        let mut processor =
            SyndromeProcessor::new().unwrap();

        processor
            .push(syndrome(
                0,
                0,
                &[(0, false)],
            ))
            .unwrap();

        processor.reset();

        assert_eq!(
            processor.rounds_processed(),
            0
        );

        assert_eq!(
            processor.events_generated(),
            0
        );

        assert!(processor.previous().is_none());
    }
}