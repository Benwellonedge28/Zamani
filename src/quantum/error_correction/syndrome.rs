//! Zamani Quantum Error Correction — Repeated-Round Syndrome Representation.
//!
//! This module owns the measurement-side representation of a syndrome stream.
//!
//! The architecture is:
//!
//! ```text
//! measurement round t-1 ──┐
//!                         ├─ XOR ──> DetectionEvent
//! measurement round t   ──┘
//! ```
//!
//! A [`Syndrome`] represents one complete measurement snapshot.
//! A [`DetectionEvent`] represents a change in one stabilizer measurement
//! between two consecutive rounds.
//!
//! Design goals:
//! - deterministic representation;
//! - explicit measurement rounds;
//! - explicit timestamps;
//! - bounded confidence representation;
//! - duplicate-measurement rejection;
//! - bounded syndrome size;
//! - checked round arithmetic;
//! - no panics on malformed external input;
//! - no dependency on noise generation;
//! - no dependency on any particular decoder.
//!
//! A decoder can therefore consume:
//!
//! ```text
//! Syndrome
//!     ↓
//! DetectionEvent
//!     ↓
//! decoding_graph.rs
//! ```
//!
//! without needing to know how the physical errors were generated.

use std::collections::BTreeMap;
use std::fmt;

// ============================================================================
// Safety limits
// ============================================================================

/// Maximum number of stabilizer measurements accepted by one syndrome.
///
/// This is a representation-level safety boundary. Larger codes should be
/// processed in bounded batches rather than allowing untrusted input to
/// request an unbounded allocation.
pub const MAX_STABILIZERS_PER_SYNDROME: usize = 1_000_000;

/// Maximum representable measurement round.
///
/// `u64::MAX` is reserved so that `next()` can always detect overflow.
pub const MAX_MEASUREMENT_ROUND: u64 = u64::MAX - 1;

/// Maximum representable timestamp.
///
/// `u64::MAX` is reserved for invalid/sentinel use.
pub const MAX_TIMESTAMP: u64 = u64::MAX - 1;

/// Confidence is represented in basis points.
///
/// ```text
/// 0      =   0%
/// 5_000  =  50%
/// 10_000 = 100%
/// ```
pub const MAX_CONFIDENCE_BASIS_POINTS: u16 = 10_000;

// ============================================================================
// Stabilizer identifier
// ============================================================================

/// Stable identifier for a stabilizer measurement.
///
/// The identifier is intentionally independent of the stabilizer's internal
/// Pauli representation. The stabilizer algebra belongs to `stabilizer.rs`;
/// this module only identifies its measurement result.
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
    pub const fn new(id: usize) -> Self {
        Self(id)
    }

    /// Returns the underlying numeric identifier.
    pub const fn index(self) -> usize {
        self.0
    }
}

impl fmt::Display for StabilizerId {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "s{}", self.0)
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
    pub fn new(
        round: u64,
    ) -> Result<Self, SyndromeError> {
        if round > MAX_MEASUREMENT_ROUND {
            return Err(
                SyndromeError::InvalidRound {
                    round,
                },
            );
        }

        Ok(Self(round))
    }

    /// Returns the round number.
    pub const fn value(
        self,
    ) -> u64 {
        self.0
    }

    /// Returns the next measurement round.
    ///
    /// Overflow is explicitly rejected rather than wrapping.
    pub fn next(
        self,
    ) -> Result<Self, SyndromeError> {
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

/// Validated timestamp associated with a syndrome measurement round.
///
/// The exact physical unit is intentionally left to the caller. It may be:
///
/// - nanoseconds;
/// - hardware ticks;
/// - simulation time;
/// - cycle count;
/// - backend-specific logical time.
///
/// The representation remains deterministic because it uses an integer.
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
    pub fn new(
        timestamp: u64,
    ) -> Result<Self, SyndromeError> {
        if timestamp > MAX_TIMESTAMP {
            return Err(
                SyndromeError::InvalidTimestamp {
                    timestamp,
                },
            );
        }

        Ok(Self(timestamp))
    }

    /// Returns the timestamp.
    pub const fn value(
        self,
    ) -> u64 {
        self.0
    }
}

// ============================================================================
// Measurement confidence
// ============================================================================

/// Exact measurement confidence represented in basis points.
///
/// Using an integer instead of `f32`/`f64` avoids:
///
/// - NaN;
/// - infinity;
/// - floating-point equality problems;
/// - nondeterministic serialization concerns.
///
/// Range:
///
/// ```text
/// 0 ..= 10_000
/// ```
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
    pub const FULL: Self =
        Self(MAX_CONFIDENCE_BASIS_POINTS);

    /// Creates confidence from basis points.
    pub fn from_basis_points(
        value: u16,
    ) -> Result<Self, SyndromeError> {
        if value > MAX_CONFIDENCE_BASIS_POINTS {
            return Err(
                SyndromeError::InvalidConfidence {
                    value,
                },
            );
        }

        Ok(Self(value))
    }

    /// Creates confidence from an integer percentage.
    ///
    /// For example:
    ///
    /// ```text
    /// 95 -> 9_500 basis points
    /// ```
    pub fn from_probability_percent(
        percent: u8,
    ) -> Result<Self, SyndromeError> {
        Self::from_basis_points(
            (percent as u16) * 100,
        )
    }

    /// Returns confidence in basis points.
    pub const fn basis_points(
        self,
    ) -> u16 {
        self.0
    }

    /// Returns confidence as a probability in `[0.0, 1.0]`.
    pub fn as_probability(
        self,
    ) -> f64 {
        f64::from(self.0)
            / f64::from(
                MAX_CONFIDENCE_BASIS_POINTS,
            )
    }
}

// ============================================================================
// Syndrome measurement
// ============================================================================

/// One stabilizer measurement.
///
/// `value == true` represents a non-trivial syndrome bit.
///
/// `value == false` represents a trivial syndrome bit.
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
    /// Creates a syndrome measurement.
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
    pub const fn stabilizer(
        self,
    ) -> StabilizerId {
        self.stabilizer
    }

    /// Returns the syndrome bit.
    pub const fn value(
        self,
    ) -> bool {
        self.value
    }

    /// Returns measurement confidence.
    pub const fn confidence(
        self,
    ) -> MeasurementConfidence {
        self.confidence
    }
}

// ============================================================================
// Syndrome
// ============================================================================

/// A complete syndrome snapshot from one measurement round.
///
/// Internally, measurements are stored in a `BTreeMap` so iteration order is
/// deterministic regardless of insertion order.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct Syndrome {
    round: MeasurementRound,
    timestamp: MeasurementTimestamp,

    measurements:
        BTreeMap<
            StabilizerId,
            SyndromeMeasurement,
        >,
}

impl Syndrome {
    /// Creates an empty syndrome snapshot.
    pub fn new(
        round: MeasurementRound,
        timestamp: MeasurementTimestamp,
    ) -> Self {
        Self {
            round,
            timestamp,
            measurements:
                BTreeMap::new(),
        }
    }

    /// Returns the measurement round.
    pub const fn round(
        &self,
    ) -> MeasurementRound {
        self.round
    }

    /// Returns the measurement timestamp.
    pub const fn timestamp(
        &self,
    ) -> MeasurementTimestamp {
        self.timestamp
    }

    /// Returns the number of stabilizers measured.
    pub fn len(
        &self,
    ) -> usize {
        self.measurements.len()
    }

    /// Returns whether the syndrome contains no measurements.
    pub fn is_empty(
        &self,
    ) -> bool {
        self.measurements.is_empty()
    }

    /// Iterates over measurements in deterministic stabilizer-ID order.
    pub fn measurements(
        &self,
    ) -> impl Iterator<
        Item = &SyndromeMeasurement,
    > {
        self.measurements.values()
    }

    /// Returns a measurement for a stabilizer.
    pub fn get(
        &self,
        stabilizer: StabilizerId,
    ) -> Option<SyndromeMeasurement> {
        self.measurements
            .get(&stabilizer)
            .copied()
    }

    /// Returns the syndrome bit for a stabilizer.
    pub fn value(
        &self,
        stabilizer: StabilizerId,
    ) -> Option<bool> {
        self.get(stabilizer)
            .map(
                SyndromeMeasurement::value,
            )
    }

    /// Inserts a measurement.
    ///
    /// Duplicate stabilizer identifiers are rejected rather than silently
    /// replacing an existing measurement.
    pub fn insert(
        &mut self,
        measurement: SyndromeMeasurement,
    ) -> Result<(), SyndromeError> {
        let stabilizer =
            measurement.stabilizer();

        if self.measurements.len()
            >= MAX_STABILIZERS_PER_SYNDROME
            && !self
                .measurements
                .contains_key(&stabilizer)
        {
            return Err(
                SyndromeError::TooManyStabilizers {
                    limit:
                        MAX_STABILIZERS_PER_SYNDROME,
                },
            );
        }

        if self
            .measurements
            .contains_key(&stabilizer)
        {
            return Err(
                SyndromeError::DuplicateStabilizer {
                    stabilizer,
                },
            );
        }

        self.measurements
            .insert(
                stabilizer,
                measurement,
            );

        Ok(())
    }

    /// Returns true if every measured stabilizer has a trivial syndrome bit.
    pub fn is_trivial(
        &self,
    ) -> bool {
        self.measurements
            .values()
            .all(
                |measurement| {
                    !measurement.value()
                },
            )
    }

    /// Returns the stabilizers whose syndrome bit is active.
    pub fn active_stabilizers(
        &self,
    ) -> Vec<StabilizerId> {
        self.measurements
            .values()
            .filter(
                |measurement| {
                    measurement.value()
                },
            )
            .map(
                SyndromeMeasurement::stabilizer,
            )
            .collect()
    }

    /// Returns the identifiers of every measured stabilizer.
    pub fn stabilizer_ids(
        &self,
    ) -> impl Iterator<
        Item = StabilizerId,
    > + '_ {
        self.measurements.keys().copied()
    }

    /// Calculates detection events between two consecutive rounds.
    ///
    /// Mathematically:
    ///
    /// ```text
    /// D(t) = S(t) XOR S(t-1)
    /// ```
    ///
    /// A missing stabilizer is **not** interpreted as zero. The two syndrome
    /// snapshots must contain exactly the same stabilizer set.
    pub fn detection_events_against(
        &self,
        previous: &Self,
    ) -> Result<
        Vec<DetectionEvent>,
        SyndromeError,
    > {
        if previous
            .round
            .value()
            .checked_add(1)
            != Some(self.round.value())
        {
            return Err(
                SyndromeError::NonConsecutiveRounds {
                    previous:
                        previous.round.value(),
                    current:
                        self.round.value(),
                },
            );
        }

        if self.measurements.len()
            != previous.measurements.len()
            || self.measurements.keys()
                != previous.measurements.keys()
        {
            return Err(
                SyndromeError::StabilizerSetMismatch,
            );
        }

        let mut events =
            Vec::new();

        for stabilizer in
            self.measurements.keys().copied()
        {
            let current =
                self.measurements
                    .get(&stabilizer)
                    .map(
                        SyndromeMeasurement::value,
                    )
                    .ok_or(
                        SyndromeError::StabilizerSetMismatch,
                    )?;

            let previous_value =
                previous
                    .measurements
                    .get(&stabilizer)
                    .map(
                        SyndromeMeasurement::value,
                    )
                    .ok_or(
                        SyndromeError::StabilizerSetMismatch,
                    )?;

            if current ^ previous_value {
                let measurement =
                    self.measurements
                        .get(&stabilizer)
                        .ok_or(
                            SyndromeError::StabilizerSetMismatch,
                        )?;

                events.push(
                    DetectionEvent {
                        round: self.round,
                        timestamp:
                            self.timestamp,
                        stabilizer,
                        value: true,
                        confidence:
                            measurement
                                .confidence(),
                    },
                );
            }
        }

        Ok(events)
    }
}

// ============================================================================
// Detection event
// ============================================================================

/// A detection event produced by the XOR of two consecutive syndrome rounds.
///
/// A detection event means the measured parity associated with a stabilizer
/// changed between rounds.
///
/// This is the natural input to a future space-time decoding graph.
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
    /// Returns the round in which the event was detected.
    pub const fn round(
        self,
    ) -> MeasurementRound {
        self.round
    }

    /// Returns the timestamp of the current syndrome round.
    pub const fn timestamp(
        self,
    ) -> MeasurementTimestamp {
        self.timestamp
    }

    /// Returns the affected stabilizer.
    pub const fn stabilizer(
        self,
    ) -> StabilizerId {
        self.stabilizer
    }

    /// Returns the event value.
    ///
    /// For an emitted event this is currently always `true`. Keeping the
    /// value explicit makes the representation extensible to richer event
    /// streams later.
    pub const fn value(
        self,
    ) -> bool {
        self.value
    }

    /// Returns confidence associated with the current measurement.
    pub const fn confidence(
        self,
    ) -> MeasurementConfidence {
        self.confidence
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by validated syndrome construction and comparison.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub enum SyndromeError {
    /// Measurement round is outside the supported range.
    InvalidRound {
        round: u64,
    },

    /// Incrementing a measurement round would overflow.
    RoundOverflow,

    /// Timestamp is outside the supported range.
    InvalidTimestamp {
        timestamp: u64,
    },

    /// Measurement confidence exceeds 100%.
    InvalidConfidence {
        value: u16,
    },

    /// Syndrome contains more stabilizers than the safety limit permits.
    TooManyStabilizers {
        limit: usize,
    },

    /// The same stabilizer was measured more than once in one round.
    DuplicateStabilizer {
        stabilizer: StabilizerId,
    },

    /// Two syndrome snapshots are not consecutive.
    NonConsecutiveRounds {
        previous: u64,
        current: u64,
    },

    /// Two syndrome snapshots contain different stabilizer sets.
    StabilizerSetMismatch,
}

impl fmt::Display for SyndromeError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidRound {
                round,
            } => {
                write!(
                    f,
                    "invalid measurement round: {round}"
                )
            }

            Self::RoundOverflow => {
                write!(
                    f,
                    "measurement round overflow"
                )
            }

            Self::InvalidTimestamp {
                timestamp,
            } => {
                write!(
                    f,
                    "invalid measurement timestamp: {timestamp}"
                )
            }

            Self::InvalidConfidence {
                value,
            } => {
                write!(
                    f,
                    "measurement confidence {value} exceeds 10000 basis points"
                )
            }

            Self::TooManyStabilizers {
                limit,
            } => {
                write!(
                    f,
                    "syndrome exceeds stabilizer limit of {limit}"
                )
            }

            Self::DuplicateStabilizer {
                stabilizer,
            } => {
                write!(
                    f,
                    "duplicate measurement for {stabilizer}"
                )
            }

            Self::NonConsecutiveRounds {
                previous,
                current,
            } => {
                write!(
                    f,
                    "non-consecutive syndrome rounds: {previous} -> {current}"
                )
            }

            Self::StabilizerSetMismatch => {
                write!(
                    f,
                    "syndrome rounds contain different stabilizer sets"
                )
            }
        }
    }
}

impl std::error::Error for SyndromeError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn round(
        value: u64,
    ) -> MeasurementRound {
        MeasurementRound::new(value)
            .expect("test round must be valid")
    }

    fn timestamp(
        value: u64,
    ) -> MeasurementTimestamp {
        MeasurementTimestamp::new(value)
            .expect("test timestamp must be valid")
    }

    fn measurement(
        id: usize,
        value: bool,
    ) -> SyndromeMeasurement {
        SyndromeMeasurement::new(
            StabilizerId::new(id),
            value,
            MeasurementConfidence::FULL,
        )
    }

    #[test]
    fn syndrome_stores_round_and_timestamp() {
        let syndrome =
            Syndrome::new(
                round(3),
                timestamp(300),
            );

        assert_eq!(
            syndrome.round().value(),
            3
        );

        assert_eq!(
            syndrome.timestamp().value(),
            300
        );

        assert!(syndrome.is_empty());
    }

    #[test]
    fn measurements_are_stored_deterministically() {
        let mut syndrome =
            Syndrome::new(
                round(3),
                timestamp(300),
            );

        syndrome
            .insert(measurement(2, true))
            .expect("insert should succeed");

        syndrome
            .insert(measurement(0, false))
            .expect("insert should succeed");

        syndrome
            .insert(measurement(1, true))
            .expect("insert should succeed");

        let ids: Vec<_> = syndrome
            .stabilizer_ids()
            .collect();

        assert_eq!(
            ids,
            vec![
                StabilizerId::new(0),
                StabilizerId::new(1),
                StabilizerId::new(2),
            ]
        );
    }

    #[test]
    fn duplicate_stabilizer_is_rejected() {
        let mut syndrome =
            Syndrome::new(
                round(0),
                timestamp(0),
            );

        let value =
            measurement(0, true);

        syndrome
            .insert(value)
            .expect("first insertion should succeed");

        assert_eq!(
            syndrome.insert(value),
            Err(
                SyndromeError::DuplicateStabilizer {
                    stabilizer:
                        StabilizerId::new(0),
                }
            )
        );
    }

    #[test]
    fn active_stabilizers_are_reported() {
        let mut syndrome =
            Syndrome::new(
                round(0),
                timestamp(0),
            );

        syndrome
            .insert(measurement(0, false))
            .unwrap();

        syndrome
            .insert(measurement(1, true))
            .unwrap();

        syndrome
            .insert(measurement(2, false))
            .unwrap();

        syndrome
            .insert(measurement(3, true))
            .unwrap();

        assert_eq!(
            syndrome.active_stabilizers(),
            vec![
                StabilizerId::new(1),
                StabilizerId::new(3),
            ]
        );

        assert!(!syndrome.is_trivial());
    }

    #[test]
    fn trivial_syndrome_has_no_active_stabilizers() {
        let mut syndrome =
            Syndrome::new(
                round(0),
                timestamp(0),
            );

        syndrome
            .insert(measurement(0, false))
            .unwrap();

        syndrome
            .insert(measurement(1, false))
            .unwrap();

        assert!(syndrome.is_trivial());

        assert!(
            syndrome
                .active_stabilizers()
                .is_empty()
        );
    }

    #[test]
    fn detection_event_is_xor_of_consecutive_rounds() {
        let mut previous =
            Syndrome::new(
                round(0),
                timestamp(10),
            );

        let mut current =
            Syndrome::new(
                round(1),
                timestamp(20),
            );

        previous
            .insert(measurement(0, false))
            .unwrap();

        previous
            .insert(measurement(1, true))
            .unwrap();

        current
            .insert(measurement(0, true))
            .unwrap();

        current
            .insert(measurement(1, true))
            .unwrap();

        let events =
            current
                .detection_events_against(
                    &previous,
                )
                .expect(
                    "rounds should be compatible",
                );

        assert_eq!(
            events.len(),
            1
        );

        assert_eq!(
            events[0].stabilizer(),
            StabilizerId::new(0)
        );

        assert_eq!(
            events[0].round().value(),
            1
        );

        assert_eq!(
            events[0].timestamp().value(),
            20
        );

        assert!(events[0].value());
    }

    #[test]
    fn unchanged_syndrome_produces_no_detection_events() {
        let mut previous =
            Syndrome::new(
                round(0),
                timestamp(10),
            );

        let mut current =
            Syndrome::new(
                round(1),
                timestamp(20),
            );

        previous
            .insert(measurement(0, true))
            .unwrap();

        previous
            .insert(measurement(1, false))
            .unwrap();

        current
            .insert(measurement(0, true))
            .unwrap();

        current
            .insert(measurement(1, false))
            .unwrap();

        let events =
            current
                .detection_events_against(
                    &previous,
                )
                .unwrap();

        assert!(events.is_empty());
    }

    #[test]
    fn detection_requires_consecutive_rounds() {
        let previous =
            Syndrome::new(
                round(0),
                timestamp(0),
            );

        let current =
            Syndrome::new(
                round(2),
                timestamp(2),
            );

        assert_eq!(
            current.detection_events_against(
                &previous,
            ),
            Err(
                SyndromeError::NonConsecutiveRounds {
                    previous: 0,
                    current: 2,
                }
            )
        );
    }

    #[test]
    fn detection_requires_matching_stabilizer_sets() {
        let mut previous =
            Syndrome::new(
                round(0),
                timestamp(0),
            );

        let mut current =
            Syndrome::new(
                round(1),
                timestamp(1),
            );

        previous
            .insert(measurement(0, false))
            .unwrap();

        current
            .insert(measurement(1, true))
            .unwrap();

        assert_eq!(
            current.detection_events_against(
                &previous,
            ),
            Err(
                SyndromeError::StabilizerSetMismatch
            )
        );
    }

    #[test]
    fn measurement_round_next_is_checked() {
        let round =
            MeasurementRound::new(
                MAX_MEASUREMENT_ROUND,
            )
            .expect("maximum valid round");

        assert_eq!(
            round.next(),
            Err(
                SyndromeError::RoundOverflow
            )
        );
    }

    #[test]
    fn invalid_round_is_rejected() {
        assert_eq!(
            MeasurementRound::new(
                u64::MAX,
            ),
            Err(
                SyndromeError::InvalidRound {
                    round: u64::MAX,
                }
            )
        );
    }

    #[test]
    fn invalid_timestamp_is_rejected() {
        assert_eq!(
            MeasurementTimestamp::new(
                u64::MAX,
            ),
            Err(
                SyndromeError::InvalidTimestamp {
                    timestamp: u64::MAX,
                }
            )
        );
    }

    #[test]
    fn confidence_is_exact_and_bounded() {
        assert_eq!(
            MeasurementConfidence::ZERO
                .as_probability(),
            0.0
        );

        assert_eq!(
            MeasurementConfidence::FULL
                .as_probability(),
            1.0
        );

        let confidence =
            MeasurementConfidence::from_basis_points(
                9_500,
            )
            .expect(
                "9500 basis points is valid",
            );

        assert_eq!(
            confidence.basis_points(),
            9_500
        );

        assert_eq!(
            confidence.as_probability(),
            0.95
        );

        assert_eq!(
            MeasurementConfidence::from_basis_points(
                10_001,
            ),
            Err(
                SyndromeError::InvalidConfidence {
                    value: 10_001,
                }
            )
        );
    }

    #[test]
    fn confidence_percent_conversion_is_correct() {
        let confidence =
            MeasurementConfidence::from_probability_percent(
                95,
            )
            .expect(
                "95 percent is valid",
            );

        assert_eq!(
            confidence.basis_points(),
            9_500
        );

        assert_eq!(
            confidence.as_probability(),
            0.95
        );
    }

    #[test]
    fn detection_event_preserves_measurement_confidence() {
        let confidence =
            MeasurementConfidence::from_basis_points(
                8_750,
            )
            .unwrap();

        let mut previous =
            Syndrome::new(
                round(0),
                timestamp(0),
            );

        let mut current =
            Syndrome::new(
                round(1),
                timestamp(1),
            );

        previous
            .insert(
                SyndromeMeasurement::new(
                    StabilizerId::new(0),
                    false,
                    MeasurementConfidence::FULL,
                ),
            )
            .unwrap();

        current
            .insert(
                SyndromeMeasurement::new(
                    StabilizerId::new(0),
                    true,
                    confidence,
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
            events.len(),
            1
        );

        assert_eq!(
            events[0]
                .confidence()
                .basis_points(),
            8_750
        );
    }

    #[test]
    fn detection_events_are_deterministically_ordered() {
        let mut previous =
            Syndrome::new(
                round(0),
                timestamp(0),
            );

        let mut current =
            Syndrome::new(
                round(1),
                timestamp(1),
            );

        for id in 0..10 {
            previous
                .insert(
                    measurement(
                        id,
                        false,
                    ),
                )
                .unwrap();

            current
                .insert(
                    measurement(
                        id,
                        true,
                    ),
                )
                .unwrap();
        }

        let events =
            current
                .detection_events_against(
                    &previous,
                )
                .unwrap();

        let ids: Vec<_> = events
            .iter()
            .map(
                DetectionEvent::stabilizer,
            )
            .collect();

        let expected: Vec<_> =
            (0..10)
                .map(StabilizerId::new)
                .collect();

        assert_eq!(
            ids,
            expected
        );
    }
}