//! Zamani Quantum Error Correction — Syndrome Representation.
//!
//! This module owns the validated, deterministic representation of repeated
//! stabilizer measurements and their conversion into space-time detection
//! events.
//!
//! Architecture:
//!
//! ```text
//! QPU / simulator / decoder input
//!             │
//!             ▼
//!       SyndromeSource
//!             │
//!             ▼
//!       SyndromeSnapshot
//!             │
//!       validation + limits
//!             │
//!             ▼
//!          Syndrome
//!             │
//!     consecutive-round XOR
//!             │
//!             ▼
//!      DetectionEvent
//!             │
//!             ▼
//!     decoding_graph.rs
//! ```
//!
//! Design principles:
//!
//! - deterministic ordering;
//! - centralized QecLimits;
//! - canonical QecError boundary;
//! - checked integer arithmetic;
//! - explicit measurement rounds;
//! - monotonic timestamps;
//! - duplicate-measurement rejection;
//! - strict stabilizer-set validation;
//! - bounded syndrome/event resources;
//! - cooperative cancellation;
//! - incremental source processing;
//! - no decoder-specific assumptions;
//! - no panic-based handling of untrusted data;
//! - deterministic replay semantics;
//! - compatibility with streaming.rs;
//! - suitable for simulator and QPU measurement pipelines.
//!
//! A syndrome is a measurement snapshot.
//!
//! A detection event is the XOR difference between two consecutive snapshots:
//!
//! ```text
//! D(t) = S(t) XOR S(t-1)
//! ```
//!
//! Missing stabilizers are never silently interpreted as zero. A malformed or
//! incomplete round must be rejected explicitly because silently changing the
//! stabilizer domain changes the mathematical meaning of the syndrome.

use core::fmt;
use std::collections::BTreeMap;

use super::cancellation::CancellationToken;
use super::errors::{
    QecError,
    QecResult,
    ResourceKind,
};
use super::limits::QecLimits;

// ============================================================================
// Constants
// ============================================================================

/// Representation-level hard ceiling.
///
/// The authoritative production limit is `QecLimits::max_stabilizers`.
/// This constant prevents a caller from constructing an absurdly large
/// representation even before a QecLimits object is supplied.
pub const MAX_STABILIZERS_PER_SYNDROME: usize = 1_000_000_000;

/// Maximum representable measurement round.
///
/// `u64::MAX` is reserved so that arithmetic overflow remains detectable.
pub const MAX_MEASUREMENT_ROUND: u64 = u64::MAX - 1;

/// Maximum representable timestamp.
///
/// `u64::MAX` is reserved for sentinel/invalid use.
pub const MAX_TIMESTAMP: u64 = u64::MAX - 1;

/// Confidence is represented in basis points.
///
/// ```text
/// 0      =   0%
/// 5_000  =  50%
/// 10_000 = 100%
/// ```
pub const MAX_CONFIDENCE_BASIS_POINTS: u16 = 10_000;

/// Conservative representation estimate used for resource preflight.
///
/// This is intentionally an upper-bound estimate rather than a claim about
/// allocator internals.
const ESTIMATED_BYTES_PER_MEASUREMENT: u64 = 64;

/// Conservative fixed overhead estimate for a syndrome.
const ESTIMATED_SYNDROME_OVERHEAD_BYTES: u64 = 128;

// ============================================================================
// Stabilizer identifier
// ============================================================================

/// Stable identifier for a stabilizer measurement.
///
/// This identifier is intentionally independent of the Pauli representation
/// in `stabilizer.rs`.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct StabilizerId(pub usize);

impl StabilizerId {
    /// Creates a stabilizer identifier.
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
// Measurement round
// ============================================================================

/// Validated measurement-round number.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct MeasurementRound(u64);

impl MeasurementRound {
    /// Creates a validated measurement round.
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

    /// Returns the next round without wrapping.
    pub fn next(self) -> Result<Self, SyndromeError> {
        let next = self
            .0
            .checked_add(1)
            .ok_or(SyndromeError::RoundOverflow)?;

        Self::new(next)
    }
}

// ============================================================================
// Measurement timestamp
// ============================================================================

/// Validated measurement timestamp.
///
/// The unit is deliberately backend-independent. It can represent hardware
/// ticks, nanoseconds, simulator time, cycle counts, or another integer time
/// domain defined by the execution backend.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct MeasurementTimestamp(u64);

impl MeasurementTimestamp {
    /// Creates a validated timestamp.
    pub fn new(timestamp: u64) -> Result<Self, SyndromeError> {
        if timestamp > MAX_TIMESTAMP {
            return Err(SyndromeError::InvalidTimestamp { timestamp });
        }

        Ok(Self(timestamp))
    }

    /// Returns the timestamp.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

// ============================================================================
// Measurement confidence
// ============================================================================

/// Exact confidence represented as basis points.
///
/// Integer representation avoids NaN, infinity, floating-point equality
/// problems, and nondeterministic serialization.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
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
    /// `95` becomes `9500` basis points.
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

    /// Returns the confidence as `[0.0, 1.0]`.
    #[must_use]
    pub fn as_probability(self) -> f64 {
        f64::from(self.0) / f64::from(MAX_CONFIDENCE_BASIS_POINTS)
    }

    /// Returns the lower of two confidence values.
    ///
    /// This is useful for detection events because an event depends on two
    /// measurements. The resulting confidence cannot be stronger than its
    /// least-confident constituent measurement.
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
// Syndrome measurement
// ============================================================================

/// One stabilizer measurement.
///
/// `value == true` means a non-trivial syndrome bit.
///
/// `value == false` means a trivial syndrome bit.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
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
// Syndrome construction policy
// ============================================================================

/// Construction options for a syndrome.
///
/// This provides an explicit preflight boundary so callers can validate
/// resources before inserting potentially large measurement sets.
#[derive(Debug, Clone, Copy)]
pub struct SyndromeOptions {
    /// Resource policy.
    pub limits: QecLimits,

    /// Whether the syndrome must contain at least one measurement.
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
    /// Creates options from a QEC resource policy.
    #[must_use]
    pub const fn with_limits(limits: QecLimits) -> Self {
        Self {
            limits,
            require_non_empty: false,
        }
    }

    /// Requires a non-empty syndrome.
    #[must_use]
    pub const fn require_non_empty(mut self) -> Self {
        self.require_non_empty = true;
        self
    }

    /// Validates the configuration.
    pub fn validate(&self) -> QecResult<()> {
        self.limits.validate().map_err(|error| {
            QecError::invalid_input(format!(
                "invalid QEC limits for syndrome processing: {}",
                error
            ))
        })
    }
}

// ============================================================================
// Syndrome
// ============================================================================

/// A complete syndrome snapshot from one measurement round.
///
/// Measurements are stored in a `BTreeMap`, giving deterministic ordering
/// regardless of insertion order.
///
/// The map deliberately stores the complete stabilizer measurement domain.
/// This prevents a missing stabilizer from being silently interpreted as a
/// zero-valued measurement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Syndrome {
    round: MeasurementRound,
    timestamp: MeasurementTimestamp,

    measurements: BTreeMap<
        StabilizerId,
        SyndromeMeasurement,
    >,

    /// Resource policy used when the snapshot was constructed.
    limits: QecLimits,
}

impl Syndrome {
    /// Creates an empty syndrome using default limits.
    pub fn new(
        round: MeasurementRound,
        timestamp: MeasurementTimestamp,
    ) -> Self {
        Self::new_with_limits(
            round,
            timestamp,
            QecLimits::default(),
        )
        .expect("default QEC limits are valid")
    }

    /// Creates an empty syndrome with explicit resource limits.
    ///
    /// The default constructor above is retained for compatibility, while
    /// production callers should prefer this method when processing untrusted
    /// or large inputs.
    pub fn new_with_limits(
        round: MeasurementRound,
        timestamp: MeasurementTimestamp,
        limits: QecLimits,
    ) -> QecResult<Self> {
        limits.validate().map_err(|error| {
            QecError::invalid_input(format!(
                "invalid QEC limits: {}",
                error
            ))
        })?;

        Ok(Self {
            round,
            timestamp,
            measurements: BTreeMap::new(),
            limits,
        })
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

    /// Returns the active resource policy.
    #[must_use]
    pub const fn limits(&self) -> QecLimits {
        self.limits
    }

    /// Returns the number of stabilizer measurements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.measurements.len()
    }

    /// Returns whether no measurements are present.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.measurements.is_empty()
    }

    /// Returns an estimated memory footprint.
    ///
    /// This is intentionally a conservative accounting estimate, not an
    /// allocator-specific measurement.
    pub fn estimated_memory_bytes(&self) -> QecResult<u64> {
        let count = u64::try_from(self.measurements.len())
            .map_err(|_| {
                QecError::numerical_failure(
                    super::errors::NumericalOperation::IntegerConversion,
                    "syndrome measurement count does not fit in u64",
                )
            })?;

        let payload = count
            .checked_mul(ESTIMATED_BYTES_PER_MEASUREMENT)
            .and_then(|value| {
                value.checked_add(ESTIMATED_SYNDROME_OVERHEAD_BYTES)
            })
            .ok_or_else(|| {
                QecError::numerical_failure(
                    super::errors::NumericalOperation::Accumulation,
                    "syndrome memory estimate overflowed",
                )
            })?;

        Ok(payload)
    }

    /// Performs a resource preflight.
    pub fn preflight(&self) -> QecResult<()> {
        let count = self.measurements.len();

        if count > self.limits.max_stabilizers {
            return Err(QecError::resource_limit(
                ResourceKind::Stabilizers,
                count as u128,
                self.limits.max_stabilizers as u128,
                format!(
                    "syndrome contains {} stabilizers; configured limit is {}",
                    count,
                    self.limits.max_stabilizers
                ),
            ));
        }

        let memory = self.estimated_memory_bytes()?;

        if memory > self.limits.max_memory_bytes {
            return Err(QecError::memory_limit(
                memory,
                self.limits.max_memory_bytes,
                format!(
                    "syndrome representation requires approximately {} bytes",
                    memory
                ),
            ));
        }

        Ok(())
    }

    /// Iterates over all measurements in deterministic stabilizer-ID order.
    pub fn measurements(
        &self,
    ) -> impl Iterator<Item = &SyndromeMeasurement> {
        self.measurements.values()
    }

    /// Returns a measurement for a stabilizer.
    #[must_use]
    pub fn get(
        &self,
        stabilizer: StabilizerId,
    ) -> Option<SyndromeMeasurement> {
        self.measurements.get(&stabilizer).copied()
    }

    /// Returns the syndrome bit for a stabilizer.
    #[must_use]
    pub fn value(
        &self,
        stabilizer: StabilizerId,
    ) -> Option<bool> {
        self.get(stabilizer)
            .map(SyndromeMeasurement::value)
    }

    /// Inserts a measurement.
    ///
    /// Duplicate stabilizer identifiers are rejected.
    ///
    /// Resource limits are checked before the map is mutated.
    pub fn insert(
        &mut self,
        measurement: SyndromeMeasurement,
    ) -> QecResult<()> {
        let stabilizer = measurement.stabilizer();

        if self.measurements.contains_key(&stabilizer) {
            return Err(SyndromeError::DuplicateStabilizer {
                stabilizer,
            }
            .into());
        }

        let next_len = self
            .measurements
            .len()
            .checked_add(1)
            .ok_or_else(|| {
                QecError::resource_limit(
                    ResourceKind::Stabilizers,
                    u128::MAX,
                    self.limits.max_stabilizers as u128,
                    "syndrome stabilizer count overflow",
                )
            })?;

        if next_len > self.limits.max_stabilizers
            || next_len > MAX_STABILIZERS_PER_SYNDROME
        {
            return Err(QecError::resource_limit(
                ResourceKind::Stabilizers,
                next_len as u128,
                (self.limits.max_stabilizers)
                    .min(MAX_STABILIZERS_PER_SYNDROME)
                    as u128,
                "syndrome stabilizer limit exceeded",
            ));
        }

        self.measurements.insert(
            stabilizer,
            measurement,
        );

        Ok(())
    }

    /// Returns whether every measured stabilizer is trivial.
    #[must_use]
    pub fn is_trivial(&self) -> bool {
        self.measurements
            .values()
            .all(|measurement| !measurement.value())
    }

    /// Returns all active stabilizers in deterministic order.
    #[must_use]
    pub fn active_stabilizers(&self) -> Vec<StabilizerId> {
        self.measurements
            .values()
            .filter(|measurement| measurement.value())
            .map(SyndromeMeasurement::stabilizer)
            .collect()
    }

    /// Returns all measured stabilizer identifiers in deterministic order.
    pub fn stabilizer_ids(
        &self,
    ) -> impl Iterator<Item = StabilizerId> + '_ {
        self.measurements.keys().copied()
    }

    /// Validates that two syndromes have exactly the same stabilizer domain.
    pub fn validate_compatible_with(
        &self,
        previous: &Self,
    ) -> QecResult<()> {
        if self.measurements.len() != previous.measurements.len() {
            return Err(SyndromeError::StabilizerSetMismatch.into());
        }

        if self.measurements.keys() != previous.measurements.keys() {
            return Err(SyndromeError::StabilizerSetMismatch.into());
        }

        Ok(())
    }

    /// Generates detection events against the immediately preceding round.
    ///
    /// ```text
    /// D(t) = S(t) XOR S(t-1)
    /// ```
    ///
    /// Detection-event confidence is the minimum confidence of the two
    /// measurements participating in the XOR.
    pub fn detection_events_against(
        &self,
        previous: &Self,
    ) -> QecResult<Vec<DetectionEvent>> {
        if previous.round.value().checked_add(1)
            != Some(self.round.value())
        {
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
            let current = self
                .measurements
                .get(&stabilizer)
                .ok_or_else(|| {
                    SyndromeError::StabilizerSetMismatch
                })?;

            let prior = previous
                .measurements
                .get(&stabilizer)
                .ok_or_else(|| {
                    SyndromeError::StabilizerSetMismatch
                })?;

            if current.value() ^ prior.value() {
                let confidence = current
                    .confidence()
                    .min(prior.confidence());

                events.push(DetectionEvent {
                    round: self.round,
                    timestamp: self.timestamp,
                    stabilizer,
                    value: true,
                    confidence,
                });
            }
        }

        if events.len() > self.limits.max_syndrome_events {
            return Err(QecError::resource_limit(
                ResourceKind::SyndromeEvents,
                events.len() as u128,
                self.limits.max_syndrome_events as u128,
                "detection-event limit exceeded",
            ));
        }

        Ok(events)
    }

    /// Returns a deterministic XOR difference without requiring event
    /// allocation.
    ///
    /// This is useful for callers that want to inspect whether two rounds
    /// differ before constructing the full event vector.
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
                .map(|prior| current.value() != prior.value())
                .unwrap_or(true)
        })
    }
}

// ============================================================================
// Detection event
// ============================================================================

/// A change in one stabilizer parity between two consecutive measurement
/// rounds.
///
/// This is the natural input to `decoding_graph.rs`.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct DetectionEvent {
    round: MeasurementRound,
    timestamp: MeasurementTimestamp,
    stabilizer: StabilizerId,
    value: bool,
    confidence: MeasurementConfidence,
}

impl DetectionEvent {
    /// Returns the detection round.
    #[must_use]
    pub const fn round(self) -> MeasurementRound {
        self.round
    }

    /// Returns the event timestamp.
    #[must_use]
    pub const fn timestamp(self) -> MeasurementTimestamp {
        self.timestamp
    }

    /// Returns the stabilizer that changed.
    #[must_use]
    pub const fn stabilizer(self) -> StabilizerId {
        self.stabilizer
    }

    /// Returns the detection bit.
    ///
    /// For a valid `DetectionEvent`, this is always `true`.
    #[must_use]
    pub const fn value(self) -> bool {
        self.value
    }

    /// Returns event confidence.
    #[must_use]
    pub const fn confidence(self) -> MeasurementConfidence {
        self.confidence
    }

    /// Returns the space-time coordinate represented by the event.
    ///
    /// The coordinate uses:
    ///
    /// ```text
    /// x = stabilizer id
    /// y = measurement round
    /// ```
    ///
    /// Actual physical/topological coordinates remain the responsibility of
    /// `surface_code.rs` and `decoding_graph.rs`.
    #[must_use]
    pub const fn coordinate(self) -> (usize, u64) {
        (self.stabilizer.index(), self.round.value())
    }
}

// ============================================================================
// Syndrome source
// ============================================================================

/// Incremental source of validated or raw syndrome snapshots.
///
/// This trait deliberately does not define transport, threads, async
/// runtimes, or QPU communication. Those responsibilities belong to the
/// backend/streaming layers.
///
/// A source must return `None` to indicate end-of-stream.
pub trait SyndromeSource {
    /// Retrieves the next syndrome snapshot.
    fn next_syndrome(&mut self) -> QecResult<Option<Syndrome>>;
}

// ============================================================================
// Syndrome stream processor
// ============================================================================

/// Incremental syndrome processor.
///
/// This is the lightweight mathematical processing layer beneath the more
/// feature-rich `streaming.rs` infrastructure.
///
/// It provides:
///
/// - consecutive-round validation;
/// - cancellation;
/// - resource limits;
/// - incremental detection-event generation;
/// - deterministic ordering;
/// - bounded event accounting.
///
/// It intentionally does not own a queue or spawn workers.
#[derive(Debug, Clone)]
pub struct SyndromeProcessor {
    limits: QecLimits,
    cancellation: CancellationToken,
    previous: Option<Syndrome>,
    rounds_processed: usize,
    events_generated: usize,
}

impl SyndromeProcessor {
    /// Creates a processor using default limits and a fresh cancellation
    /// token.
    pub fn new() -> QecResult<Self> {
        Self::with_limits(
            QecLimits::default(),
            CancellationToken::new(),
        )
    }

    /// Creates a processor with explicit limits and cancellation.
    pub fn with_limits(
        limits: QecLimits,
        cancellation: CancellationToken,
    ) -> QecResult<Self> {
        limits.validate().map_err(|error| {
            QecError::invalid_input(format!(
                "invalid QEC limits: {}",
                error
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

    /// Returns the configured limits.
    #[must_use]
    pub const fn limits(&self) -> QecLimits {
        self.limits
    }

    /// Returns the number of processed rounds.
    #[must_use]
    pub const fn rounds_processed(&self) -> usize {
        self.rounds_processed
    }

    /// Returns the number of generated detection events.
    #[must_use]
    pub const fn events_generated(&self) -> usize {
        self.events_generated
    }

    /// Returns the previous syndrome, if any.
    #[must_use]
    pub fn previous(&self) -> Option<&Syndrome> {
        self.previous.as_ref()
    }

    /// Processes one syndrome snapshot.
    ///
    /// The first round establishes the baseline and therefore produces no
    /// detection events.
    pub fn push(
        &mut self,
        syndrome: Syndrome,
    ) -> QecResult<Vec<DetectionEvent>> {
        self.cancellation.check()?;

        if self.rounds_processed >= self.limits.max_rounds {
            return Err(QecError::resource_limit(
                ResourceKind::MeasurementRounds,
                self.rounds_processed as u128 + 1,
                self.limits.max_rounds as u128,
                "maximum measurement-round limit exceeded",
            ));
        }

        syndrome.preflight()?;

        if let Some(previous) = &self.previous {
            if syndrome.round().value()
                != previous.round().value().saturating_add(1)
            {
                return Err(
                    SyndromeError::NonConsecutiveRounds {
                        previous: previous.round().value(),
                        current: syndrome.round().value(),
                    }
                    .into(),
                );
            }

            let events =
                syndrome.detection_events_against(previous)?;

            let new_total = self
                .events_generated
                .checked_add(events.len())
                .ok_or_else(|| {
                    QecError::resource_limit(
                        ResourceKind::SyndromeEvents,
                        u128::MAX,
                        self.limits.max_syndrome_events as u128,
                        "detection-event counter overflow",
                    )
                })?;

            if new_total > self.limits.max_syndrome_events {
                return Err(QecError::resource_limit(
                    ResourceKind::SyndromeEvents,
                    new_total as u128,
                    self.limits.max_syndrome_events as u128,
                    "maximum detection-event limit exceeded",
                ));
            }

            self.events_generated = new_total;
            self.previous = Some(syndrome);

            self.rounds_processed = self
                .rounds_processed
                .checked_add(1)
                .ok_or_else(|| {
                    QecError::resource_limit(
                        ResourceKind::MeasurementRounds,
                        u128::MAX,
                        self.limits.max_rounds as u128,
                        "measurement-round counter overflow",
                    )
                })?;

            return Ok(events);
        }

        self.previous = Some(syndrome);

        self.rounds_processed = self
            .rounds_processed
            .checked_add(1)
            .ok_or_else(|| {
                QecError::resource_limit(
                    ResourceKind::MeasurementRounds,
                    u128::MAX,
                    self.limits.max_rounds as u128,
                    "measurement-round counter overflow",
                )
            })?;

        Ok(Vec::new())
    }

    /// Processes an entire source incrementally.
    ///
    /// No complete syndrome history is retained.
    pub fn process_source<S>(
        &mut self,
        source: &mut S,
    ) -> QecResult<Vec<DetectionEvent>>
    where
        S: SyndromeSource,
    {
        let mut all_events = Vec::new();

        loop {
            self.cancellation.check()?;

            let next = source.next_syndrome()?;

            let Some(syndrome) = next else {
                break;
            };

            let events = self.push(syndrome)?;

            if all_events
                .len()
                .checked_add(events.len())
                .is_none()
            {
                return Err(QecError::resource_limit(
                    ResourceKind::SyndromeEvents,
                    u128::MAX,
                    self.limits.max_syndrome_events as u128,
                    "event collection size overflow",
                ));
            }

            all_events.extend(events);
        }

        Ok(all_events)
    }

    /// Clears the current baseline.
    ///
    /// This should only be used when a new independent measurement stream
    /// begins. It deliberately does not reset resource counters.
    pub fn reset_baseline(&mut self) {
        self.previous = None;
    }
}

// ============================================================================
// Syndrome errors
// ============================================================================

/// Syndrome-specific validation errors.
///
/// These remain useful internally and are converted into canonical
/// `QecError::InvalidSyndrome` errors at the subsystem boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyndromeError {
    /// Measurement round is outside the representable domain.
    InvalidRound {
        round: u64,
    },

    /// Measurement timestamp is outside the representable domain.
    InvalidTimestamp {
        timestamp: u64,
    },

    /// Round increment overflowed.
    RoundOverflow,

    /// Confidence exceeded 100%.
    InvalidConfidence {
        value: u16,
    },

    /// The same stabilizer was measured twice in one snapshot.
    DuplicateStabilizer {
        stabilizer: StabilizerId,
    },

    /// Two snapshots do not contain the same stabilizer domain.
    StabilizerSetMismatch,

    /// Two snapshots are not consecutive measurement rounds.
    NonConsecutiveRounds {
        previous: u64,
        current: u64,
    },

    /// Timestamp moved backwards.
    TimestampRegression {
        previous: u64,
        current: u64,
    },

    /// Detection-event construction would exceed a configured limit.
    TooManyDetectionEvents {
        requested: usize,
        limit: usize,
    },

    /// A source attempted to provide an invalid syndrome.
    InvalidSource {
        message: String,
    },
}

impl fmt::Display for SyndromeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRound { round } => {
                write!(
                    formatter,
                    "invalid measurement round {}",
                    round
                )
            }

            Self::InvalidTimestamp { timestamp } => {
                write!(
                    formatter,
                    "invalid measurement timestamp {}",
                    timestamp
                )
            }

            Self::RoundOverflow => {
                write!(
                    formatter,
                    "measurement round overflow"
                )
            }

            Self::InvalidConfidence { value } => {
                write!(
                    formatter,
                    "invalid measurement confidence {} basis points",
                    value
                )
            }

            Self::DuplicateStabilizer { stabilizer } => {
                write!(
                    formatter,
                    "duplicate stabilizer measurement for {}",
                    stabilizer
                )
            }

            Self::StabilizerSetMismatch => {
                write!(
                    formatter,
                    "syndrome stabilizer sets do not match"
                )
            }

            Self::NonConsecutiveRounds {
                previous,
                current,
            } => {
                write!(
                    formatter,
                    "non-consecutive syndrome rounds: previous={}, current={}",
                    previous,
                    current
                )
            }

            Self::TimestampRegression {
                previous,
                current,
            } => {
                write!(
                    formatter,
                    "syndrome timestamp regressed: previous={}, current={}",
                    previous,
                    current
                )
            }

            Self::TooManyDetectionEvents {
                requested,
                limit,
            } => {
                write!(
                    formatter,
                    "detection-event count {} exceeds limit {}",
                    requested,
                    limit
                )
            }

            Self::InvalidSource { message } => {
                write!(
                    formatter,
                    "invalid syndrome source: {}",
                    message
                )
            }
        }
    }
}

impl std::error::Error for SyndromeError {}

// ============================================================================
// Canonical error integration
// ============================================================================

impl From<SyndromeError> for QecError {
    fn from(error: SyndromeError) -> Self {
        QecError::invalid_syndrome(error.to_string())
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
            MeasurementRound::new(round).unwrap(),
            MeasurementTimestamp::new(timestamp).unwrap(),
        );

        for &(id, value) in values {
            result
                .insert(measurement(id, value))
                .unwrap();
        }

        result
    }

    #[test]
    fn round_overflow_is_rejected() {
        let round =
            MeasurementRound::new(MAX_MEASUREMENT_ROUND)
                .unwrap();

        assert!(matches!(
            round.next(),
            Err(SyndromeError::InvalidRound { .. })
                | Err(SyndromeError::RoundOverflow)
        ));
    }

    #[test]
    fn duplicate_measurements_are_rejected() {
        let mut syndrome = syndrome(
            0,
            0,
            &[(0, false)],
        );

        let result =
            syndrome.insert(measurement(0, true));

        assert!(matches!(
            result,
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

        let events =
            current
                .detection_events_against(
                    &previous,
                )
                .unwrap();

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
    fn missing_stabilizer_is_not_treated_as_zero() {
        let previous = syndrome(
            0,
            0,
            &[
                (0, false),
                (1, false),
            ],
        );

        let current = syndrome(
            1,
            1,
            &[(0, false)],
        );

        let result =
            current
                .detection_events_against(
                    &previous,
                );

        assert!(matches!(
            result,
            Err(QecError::InvalidSyndrome { .. })
        ));
    }

    #[test]
    fn timestamp_regression_is_rejected() {
        let previous = syndrome(
            0,
            100,
            &[(0, false)],
        );

        let current = syndrome(
            1,
            99,
            &[(0, true)],
        );

        let result =
            current
                .detection_events_against(
                    &previous,
                );

        assert!(matches!(
            result,
            Err(QecError::InvalidSyndrome { .. })
        ));
    }

    #[test]
    fn confidence_is_minimum_of_two_measurements() {
        let previous_round =
            MeasurementRound::new(0).unwrap();

        let current_round =
            MeasurementRound::new(1).unwrap();

        let timestamp0 =
            MeasurementTimestamp::new(0).unwrap();

        let timestamp1 =
            MeasurementTimestamp::new(1).unwrap();

        let mut previous =
            Syndrome::new(
                previous_round,
                timestamp0,
            );

        previous
            .insert(
                SyndromeMeasurement::new(
                    StabilizerId::new(0),
                    false,
                    MeasurementConfidence::from_basis_points(
                        7_000,
                    )
                    .unwrap(),
                ),
            )
            .unwrap();

        let mut current =
            Syndrome::new(
                current_round,
                timestamp1,
            );

        current
            .insert(
                SyndromeMeasurement::new(
                    StabilizerId::new(0),
                    true,
                    MeasurementConfidence::from_basis_points(
                        9_000,
                    )
                    .unwrap(),
                ),
            )
            .unwrap();

        let events =
            current
                .detection_events_against(
                    &previous,
                )
                .unwrap();

        assert_eq!(
            events[0]
                .confidence()
                .basis_points(),
            7_000
        );
    }

    #[test]
    fn processor_establishes_baseline_without_event() {
        let mut processor =
            SyndromeProcessor::new()
                .unwrap();

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
    fn processor_generates_incremental_events() {
        let mut processor =
            SyndromeProcessor::new()
                .unwrap();

        processor
            .push(syndrome(
                0,
                0,
                &[(0, false)],
            ))
            .unwrap();

        let events = processor
            .push(syndrome(
                1,
                1,
                &[(0, true)],
            ))
            .unwrap();

        assert_eq!(events.len(), 1);
        assert_eq!(
            events[0].stabilizer(),
            StabilizerId::new(0)
        );
    }

    #[test]
    fn processor_rejects_out_of_order_rounds() {
        let mut processor =
            SyndromeProcessor::new()
                .unwrap();

        processor
            .push(syndrome(
                0,
                0,
                &[(0, false)],
            ))
            .unwrap();

        let result = processor
            .push(syndrome(
                2,
                2,
                &[(0, true)],
            ));

        assert!(matches!(
            result,
            Err(QecError::InvalidSyndrome { .. })
        ));
    }

    #[test]
    fn deterministic_order_is_preserved() {
        let syndrome = syndrome(
            0,
            0,
            &[
                (10, true),
                (2, true),
                (7, true),
            ],
        );

        let ids: Vec<_> =
            syndrome
                .stabilizer_ids()
                .collect();

        assert_eq!(
            ids,
            vec![
                StabilizerId::new(2),
                StabilizerId::new(7),
                StabilizerId::new(10),
            ]
        );
    }

    #[test]
    fn active_stabilizers_are_sorted() {
        let syndrome = syndrome(
            0,
            0,
            &[
                (5, true),
                (1, true),
                (3, false),
                (2, true),
            ],
        );

        assert_eq!(
            syndrome.active_stabilizers(),
            vec![
                StabilizerId::new(1),
                StabilizerId::new(2),
                StabilizerId::new(5),
            ]
        );
    }
}