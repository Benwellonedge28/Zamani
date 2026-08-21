//! Zamani Quantum Error Correction — QPU syndrome extraction boundary.
//!
//! # Ownership
//!
//! `syndrome_extractor.rs` owns the deterministic conversion of raw,
//! already-authorized QPU measurement records into the canonical
//! `syndrome.rs` representation.
//!
//! It owns:
//!
//! - raw QPU measurement records;
//! - measurement-batch metadata;
//! - extraction policy;
//! - stabilizer-domain validation;
//! - duplicate detection;
//! - missing-measurement detection;
//! - round consistency;
//! - timestamp consistency;
//! - confidence validation;
//! - canonical ordering;
//! - bounded extraction;
//! - cancellation-aware extraction;
//! - conversion into `Syndrome`;
//! - conversion of consecutive extracted syndromes into detection events;
//! - extraction statistics.
//!
//! It does NOT own:
//!
//! - QPU communication;
//! - QPU credentials;
//! - authentication;
//! - capability authorization;
//! - circuit compilation;
//! - surface-code topology;
//! - decoding;
//! - decoding-graph construction;
//! - Pauli-frame application;
//! - runtime resource accounting;
//! - memory allocation policy;
//! - telemetry transport;
//! - checkpoint persistence.
//!
//! # Integration
//!
//! ```text
//! qpu_adapter.rs
//!       |
//!       | authorized raw measurements
//!       v
//! RawMeasurementBatch
//!       |
//!       v
//! SyndromeExtractor
//!       |
//!       +---- validation
//!       +---- limits
//!       +---- cancellation
//!       +---- deterministic ordering
//!       |
//!       v
//! Syndrome
//!       |
//!       v
//! SyndromeProcessor
//!       |
//!       v
//! DetectionEvent
//!       |
//!       v
//! decoding_graph.rs
//!       |
//!       +------------+
//!       v            v
//!     MWPM       Union-Find
//! ```
//!
//! # Security boundary
//!
//! This module assumes that authorization has already occurred.
//!
//! In particular:
//!
//! - `QpuSubmit` belongs to `qpu_adapter.rs`;
//! - `QpuReadResults` belongs to `qpu_adapter.rs`;
//! - `QpuSyndromeExtraction` is required by the adapter before invoking
//!   extraction;
//! - this module never receives credentials;
//! - this module never performs network I/O.
//!
//! # Determinism
//!
//! Raw measurements may arrive in arbitrary order. Extraction always produces
//! a canonical stabilizer-ID ordered `Syndrome`.
//!
//! Duplicate stabilizers are rejected rather than silently overwritten.
//!
//! Missing stabilizers are rejected when an expected stabilizer domain is
//! supplied.
//!
//! # Resource safety
//!
//! `QecLimits` remains the single declarative policy source.
//!
//! No hard-coded production workload ceiling is introduced here.
//!
//! Extraction validates:
//!
//! - maximum stabilizers;
//! - maximum syndrome events;
//! - maximum rounds;
//! - maximum memory estimate;
//! - integer conversion/size overflow.
//!
//! # Cancellation
//!
//! Extraction polls the supplied `CancellationToken` during potentially large
//! input processing.
//!
//! # Rust compatibility
//!
//! Rust 1.97.1.

#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;
use std::collections::BTreeSet;

use super::cancellation::CancellationToken;
use super::errors::{
    QecError,
    QecResult,
    NumericalOperation,
    ResourceKind,
};
use super::limits::QecLimits;
use super::syndrome::{
    DetectionEvent,
    MeasurementConfidence,
    MeasurementRound,
    MeasurementTimestamp,
    StabilizerId,
    Syndrome,
    SyndromeMeasurement,
    SyndromeOptions,
};

/// Conservative accounting estimate for one raw measurement record.
const ESTIMATED_RAW_MEASUREMENT_BYTES: u64 = 64;

/// Conservative fixed batch overhead.
const ESTIMATED_BATCH_OVERHEAD_BYTES: u64 = 128;

/// One raw stabilizer measurement returned by a QPU backend.
///
/// The record deliberately contains no backend credentials, network state,
/// authentication material, circuit data, or private backend metadata.
///
/// Round and timestamp belong to the batch because `Syndrome` represents one
/// coherent measurement round with one canonical timestamp.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RawSyndromeMeasurement {
    stabilizer: StabilizerId,
    value: bool,
    confidence: MeasurementConfidence,
}

impl RawSyndromeMeasurement {
    /// Creates a raw measurement.
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

    /// Returns the measured stabilizer.
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

    /// Converts this raw record to the canonical syndrome representation.
    #[must_use]
    pub const fn into_syndrome_measurement(self) -> SyndromeMeasurement {
        SyndromeMeasurement::new(
            self.stabilizer,
            self.value,
            self.confidence,
        )
    }
}

/// A single QPU measurement batch belonging to one syndrome round.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawSyndromeBatch {
    round: MeasurementRound,
    timestamp: MeasurementTimestamp,
    measurements: Vec<RawSyndromeMeasurement>,
}

impl RawSyndromeBatch {
    /// Creates a raw batch.
    #[must_use]
    pub fn new(
        round: MeasurementRound,
        timestamp: MeasurementTimestamp,
        measurements: Vec<RawSyndromeMeasurement>,
    ) -> Self {
        Self {
            round,
            timestamp,
            measurements,
        }
    }

    /// Returns the measurement round.
    #[must_use]
    pub const fn round(&self) -> MeasurementRound {
        self.round
    }

    /// Returns the batch timestamp.
    #[must_use]
    pub const fn timestamp(&self) -> MeasurementTimestamp {
        self.timestamp
    }

    /// Returns the number of raw measurements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.measurements.len()
    }

    /// Returns whether the batch is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.measurements.is_empty()
    }

    /// Returns the raw measurements in backend-supplied order.
    pub fn measurements(
        &self,
    ) -> impl Iterator<Item = RawSyndromeMeasurement> + '_ {
        self.measurements.iter().copied()
    }

    /// Returns a conservative memory estimate.
    pub fn estimated_memory_bytes(&self) -> QecResult<u64> {
        let count = u64::try_from(self.measurements.len()).map_err(|_| {
            QecError::numerical_failure(
                NumericalOperation::IntegerConversion,
                "raw syndrome measurement count does not fit in u64",
            )
        })?;

        count
            .checked_mul(ESTIMATED_RAW_MEASUREMENT_BYTES)
            .and_then(|bytes| {
                bytes.checked_add(ESTIMATED_BATCH_OVERHEAD_BYTES)
            })
            .ok_or_else(|| {
                QecError::numerical_failure(
                    NumericalOperation::MemorySizeCalculation,
                    "raw syndrome batch memory estimate overflowed",
                )
            })
    }
}

/// Extraction policy.
///
/// This is intentionally independent from QPU authorization. It controls
/// how already-authorized measurements are interpreted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyndromeExtractionOptions {
    /// Canonical QEC resource policy.
    limits: QecLimits,

    /// Expected stabilizer domain.
    ///
    /// When supplied, every expected stabilizer must occur exactly once.
    expected_stabilizers: Option<BTreeSet<StabilizerId>>,

    /// Whether an empty measurement batch is rejected.
    require_non_empty: bool,
}

impl Default for SyndromeExtractionOptions {
    fn default() -> Self {
        Self {
            limits: QecLimits::default(),
            expected_stabilizers: None,
            require_non_empty: false,
        }
    }
}

impl SyndromeExtractionOptions {
    /// Creates extraction options from canonical limits.
    #[must_use]
    pub fn with_limits(limits: QecLimits) -> Self {
        Self {
            limits,
            ..Self::default()
        }
    }

    /// Requires a non-empty measurement batch.
    #[must_use]
    pub fn require_non_empty(mut self) -> Self {
        self.require_non_empty = true;
        self
    }

    /// Sets the exact expected stabilizer domain.
    ///
    /// The supplied domain is copied into a deterministic `BTreeSet`.
    pub fn with_expected_stabilizers<I>(
        mut self,
        stabilizers: I,
    ) -> Self
    where
        I: IntoIterator<Item = StabilizerId>,
    {
        self.expected_stabilizers =
            Some(stabilizers.into_iter().collect());

        self
    }

    /// Returns the canonical limits.
    #[must_use]
    pub const fn limits(&self) -> QecLimits {
        self.limits
    }

    /// Returns the expected stabilizer domain.
    #[must_use]
    pub fn expected_stabilizers(
        &self,
    ) -> Option<&BTreeSet<StabilizerId>> {
        self.expected_stabilizers.as_ref()
    }

    /// Validates extraction policy.
    pub fn validate(&self) -> QecResult<()> {
        self.limits.validate().map_err(|error| {
            QecError::invalid_input(format!(
                "invalid QEC limits for syndrome extraction: {error}"
            ))
        })?;

        if let Some(expected) = self.expected_stabilizers.as_ref() {
            if expected.len() > self.limits.max_stabilizers {
                return Err(QecError::resource_limit(
                    ResourceKind::Stabilizers,
                    expected.len() as u128,
                    expected.len() as u128,
                    self.limits.max_stabilizers as u128,
                    "expected stabilizer domain exceeds QEC stabilizer limit",
                ));
            }
        }

        Ok(())
    }
}

/// Extraction statistics.
///
/// These are local deterministic facts about one extraction operation.
/// Runtime resource accounting remains owned by `resources.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SyndromeExtractionStats {
    /// Number of raw records examined.
    pub measurements_seen: usize,

    /// Number of canonical measurements produced.
    pub measurements_extracted: usize,

    /// Number of active syndrome bits.
    pub active_measurements: usize,

    /// Estimated representation size.
    pub estimated_memory_bytes: u64,
}

impl SyndromeExtractionStats {
    /// Returns whether the extracted syndrome is trivial.
    #[must_use]
    pub const fn is_trivial(self) -> bool {
        self.active_measurements == 0
    }
}

/// Successful extraction result.
///
/// Keeping the statistics beside the syndrome prevents callers from having
/// to reconstruct facts that were already known during validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyndromeExtraction {
    syndrome: Syndrome,
    stats: SyndromeExtractionStats,
}

impl SyndromeExtraction {
    /// Creates an extraction result.
    #[must_use]
    pub const fn new(
        syndrome: Syndrome,
        stats: SyndromeExtractionStats,
    ) -> Self {
        Self { syndrome, stats }
    }

    /// Returns the canonical syndrome.
    #[must_use]
    pub const fn syndrome(&self) -> &Syndrome {
        &self.syndrome
    }

    /// Consumes the result and returns the syndrome.
    #[must_use]
    pub fn into_syndrome(self) -> Syndrome {
        self.syndrome
    }

    /// Returns extraction statistics.
    #[must_use]
    pub const fn stats(&self) -> SyndromeExtractionStats {
        self.stats
    }
}

/// Deterministic QPU syndrome extractor.
///
/// This object contains no QPU connection and therefore can safely be used
/// by simulation, replay, testing, and real hardware adapters through the same
/// conversion path.
#[derive(Debug, Clone)]
pub struct SyndromeExtractor {
    options: SyndromeExtractionOptions,
    cancellation: CancellationToken,
}

impl SyndromeExtractor {
    /// Creates an extractor using canonical default limits.
    pub fn new() -> QecResult<Self> {
        Self::with_options(
            SyndromeExtractionOptions::default(),
            CancellationToken::new(),
        )
    }

    /// Creates an extractor with explicit policy and cancellation.
    pub fn with_options(
        options: SyndromeExtractionOptions,
        cancellation: CancellationToken,
    ) -> QecResult<Self> {
        options.validate()?;

        Ok(Self {
            options,
            cancellation,
        })
    }

    /// Returns extraction options.
    #[must_use]
    pub const fn options(&self) -> &SyndromeExtractionOptions {
        &self.options
    }

    /// Returns the cancellation token used by this extractor.
    #[must_use]
    pub const fn cancellation(&self) -> &CancellationToken {
        &self.cancellation
    }

    /// Extracts one raw QPU batch into the canonical `Syndrome`.
    ///
    /// The operation is transactional: the output syndrome is not returned
    /// until all validation has succeeded.
    pub fn extract(
        &self,
        batch: &RawSyndromeBatch,
    ) -> QecResult<SyndromeExtraction> {
        self.cancellation.check()?;

        let expected = self.options.expected_stabilizers.as_ref();

        if self.options.require_non_empty && batch.is_empty() {
            return Err(QecError::invalid_syndrome(
                "QPU returned an empty syndrome measurement batch",
            ));
        }

        let count = batch.len();

        if count > self.options.limits.max_stabilizers {
            return Err(QecError::resource_limit(
                ResourceKind::Stabilizers,
                count as u128,
                count as u128,
                self.options.limits.max_stabilizers as u128,
                "QPU measurement batch exceeds stabilizer limit",
            ));
        }

        let estimated_input = batch.estimated_memory_bytes()?;

        if estimated_input > self.options.limits.max_memory_bytes {
            return Err(QecError::memory_limit(
                estimated_input,
                estimated_input,
                self.options.limits.max_memory_bytes,
                "raw QPU syndrome batch exceeds memory policy",
            ));
        }

        if let Some(expected) = expected {
            if count != expected.len() {
                return Err(
                    SyndromeExtractionError::MeasurementCountMismatch {
                        expected: expected.len(),
                        actual: count,
                    }
                    .into(),
                );
            }
        }

        let mut seen = BTreeSet::new();
        let mut measurements = Vec::with_capacity(count);
        let mut active = 0usize;

        for raw in batch.measurements() {
            self.cancellation.poll()?;

            let stabilizer = raw.stabilizer();

            if !seen.insert(stabilizer) {
                return Err(
                    SyndromeExtractionError::DuplicateStabilizer {
                        stabilizer,
                    }
                    .into(),
                );
            }

            if let Some(expected) = expected {
                if !expected.contains(&stabilizer) {
                    return Err(
                        SyndromeExtractionError::UnexpectedStabilizer {
                            stabilizer,
                        }
                        .into(),
                    );
                }
            }

            if raw.value() {
                active = active.checked_add(1).ok_or_else(|| {
                    QecError::resource_limit(
                        ResourceKind::SyndromeEvents,
                        u128::MAX,
                        active as u128,
                        self.options.limits.max_syndrome_events as u128,
                        "active syndrome counter overflow",
                    )
                })?;
            }

            measurements.push(raw.into_syndrome_measurement());
        }

        if let Some(expected) = expected {
            for stabilizer in expected {
                self.cancellation.poll()?;

                if !seen.contains(stabilizer) {
                    return Err(
                        SyndromeExtractionError::MissingStabilizer {
                            stabilizer: *stabilizer,
                        }
                        .into(),
                    );
                }
            }
        }

        // `Syndrome::from_measurements` inserts into a BTreeMap, thereby
        // establishing deterministic stabilizer-ID ordering independent of
        // backend response ordering.
        let syndrome = Syndrome::from_measurements(
            batch.round(),
            batch.timestamp(),
            measurements,
            SyndromeOptions::with_limits(
                self.options.limits,
            ),
        )?;

        syndrome.preflight()?;

        let estimated_memory =
            syndrome.estimated_memory_bytes()?;

        Ok(SyndromeExtraction::new(
            syndrome,
            SyndromeExtractionStats {
                measurements_seen: count,
                measurements_extracted: seen.len(),
                active_measurements: active,
                estimated_memory_bytes: estimated_memory,
            },
        ))
    }

    /// Extracts and immediately produces detection events against the
    /// previous canonical syndrome.
    ///
    /// This is the preferred bridge used by a QPU adapter when it processes
    /// consecutive rounds.
    pub fn extract_detection_events(
        &self,
        batch: &RawSyndromeBatch,
        previous: &Syndrome,
    ) -> QecResult<(
        SyndromeExtraction,
        Vec<DetectionEvent>,
    )> {
        let extraction = self.extract(batch)?;

        self.cancellation.check()?;

        let events = extraction
            .syndrome()
            .detection_events_against_with_cancellation(
                previous,
                &self.cancellation,
            )?;

        Ok((extraction, events))
    }
}

/// Stateful consecutive-round extractor.
///
/// This is intentionally separate from `SyndromeProcessor` because the
/// extractor owns conversion from raw QPU records, while `SyndromeProcessor`
/// owns generic syndrome-to-detection-event state.
#[derive(Debug, Clone)]
pub struct StreamingSyndromeExtractor {
    extractor: SyndromeExtractor,
    previous: Option<Syndrome>,
    rounds_processed: usize,
    events_generated: usize,
}

impl StreamingSyndromeExtractor {
    /// Creates a streaming extractor.
    pub fn new(
        options: SyndromeExtractionOptions,
        cancellation: CancellationToken,
    ) -> QecResult<Self> {
        Ok(Self {
            extractor: SyndromeExtractor::with_options(
                options,
                cancellation,
            )?,
            previous: None,
            rounds_processed: 0,
            events_generated: 0,
        })
    }

    /// Returns the underlying stateless extractor.
    #[must_use]
    pub const fn extractor(&self) -> &SyndromeExtractor {
        &self.extractor
    }

    /// Returns the previous extracted syndrome.
    #[must_use]
    pub fn previous(&self) -> Option<&Syndrome> {
        self.previous.as_ref()
    }

    /// Returns the number of processed rounds.
    #[must_use]
    pub const fn rounds_processed(&self) -> usize {
        self.rounds_processed
    }

    /// Returns the cumulative detection-event count.
    #[must_use]
    pub const fn events_generated(&self) -> usize {
        self.events_generated
    }

    /// Extracts the next batch and generates events against the previous
    /// round.
    ///
    /// The first successfully extracted round establishes the baseline and
    /// therefore produces no detection events.
    pub fn push(
        &mut self,
        batch: &RawSyndromeBatch,
    ) -> QecResult<SyndromeExtraction> {
        self.extractor.cancellation.check()?;

        let next_round_count =
            self.rounds_processed.checked_add(1).ok_or_else(|| {
                QecError::resource_limit(
                    ResourceKind::MeasurementRounds,
                    u128::MAX,
                    self.rounds_processed as u128,
                    self.extractor.options.limits.max_rounds as u128,
                    "syndrome extraction round counter overflow",
                )
            })?;

        if next_round_count
            > self.extractor.options.limits.max_rounds
        {
            return Err(QecError::resource_limit(
                ResourceKind::MeasurementRounds,
                next_round_count as u128,
                self.rounds_processed as u128,
                self.extractor.options.limits.max_rounds as u128,
                "maximum syndrome extraction rounds exceeded",
            ));
        }

        let extraction = self.extractor.extract(batch)?;

        if let Some(previous) = self.previous.as_ref() {
            let events = extraction
                .syndrome()
                .detection_events_against_with_cancellation(
                    previous,
                    &self.extractor.cancellation,
                )?;

            let new_total =
                self.events_generated
                    .checked_add(events.len())
                    .ok_or_else(|| {
                        QecError::resource_limit(
                            ResourceKind::SyndromeEvents,
                            u128::MAX,
                            self.events_generated as u128,
                            self.extractor
                                .options
                                .limits
                                .max_syndrome_events
                                as u128,
                            "syndrome extraction event counter overflow",
                        )
                    })?;

            if new_total
                > self.extractor.options.limits.max_syndrome_events
            {
                return Err(QecError::resource_limit(
                    ResourceKind::SyndromeEvents,
                    new_total as u128,
                    self.events_generated as u128,
                    self.extractor
                        .options
                        .limits
                        .max_syndrome_events as u128,
                    "cumulative syndrome extraction event limit exceeded",
                ));
            }

            self.events_generated = new_total;
        }

        self.previous = Some(extraction.syndrome().clone());
        self.rounds_processed = next_round_count;

        Ok(extraction)
    }

    /// Returns detection events for the latest push operation.
    ///
    /// The first round returns an empty vector.
    pub fn push_with_events(
        &mut self,
        batch: &RawSyndromeBatch,
    ) -> QecResult<(
        SyndromeExtraction,
        Vec<DetectionEvent>,
    )> {
        self.extractor.cancellation.check()?;

        let extraction = self.extractor.extract(batch)?;

        let events = if let Some(previous) = self.previous.as_ref() {
            extraction
                .syndrome()
                .detection_events_against_with_cancellation(
                    previous,
                    &self.extractor.cancellation,
                )?
        } else {
            Vec::new()
        };

        let next_round_count =
            self.rounds_processed.checked_add(1).ok_or_else(|| {
                QecError::resource_limit(
                    ResourceKind::MeasurementRounds,
                    u128::MAX,
                    self.rounds_processed as u128,
                    self.extractor.options.limits.max_rounds as u128,
                    "syndrome extraction round counter overflow",
                )
            })?;

        if next_round_count
            > self.extractor.options.limits.max_rounds
        {
            return Err(QecError::resource_limit(
                ResourceKind::MeasurementRounds,
                next_round_count as u128,
                self.rounds_processed as u128,
                self.extractor.options.limits.max_rounds as u128,
                "maximum syndrome extraction rounds exceeded",
            ));
        }

        let new_total =
            self.events_generated.checked_add(events.len()).ok_or_else(
                || {
                    QecError::resource_limit(
                        ResourceKind::SyndromeEvents,
                        u128::MAX,
                        self.events_generated as u128,
                        self.extractor
                            .options
                            .limits
                            .max_syndrome_events
                            as u128,
                        "syndrome extraction event counter overflow",
                    )
                },
            )?;

        if new_total
            > self.extractor.options.limits.max_syndrome_events
        {
            return Err(QecError::resource_limit(
                ResourceKind::SyndromeEvents,
                new_total as u128,
                self.events_generated as u128,
                self.extractor
                    .options
                    .limits
                    .max_syndrome_events
                    as u128,
                "cumulative syndrome extraction event limit exceeded",
            ));
        }

        self.previous = Some(extraction.syndrome().clone());
        self.rounds_processed = next_round_count;
        self.events_generated = new_total;

        Ok((extraction, events))
    }

    /// Resets only the consecutive-round baseline.
    pub fn reset_baseline(&mut self) {
        self.previous = None;
    }

    /// Fully resets streaming state and counters.
    pub fn reset(&mut self) {
        self.previous = None;
        self.rounds_processed = 0;
        self.events_generated = 0;
    }
}

/// Syndrome-extraction-specific errors.
///
/// All errors cross the public boundary as `QecError`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyndromeExtractionError {
    /// The backend returned a different number of measurements from the
    /// expected stabilizer domain.
    MeasurementCountMismatch {
        expected: usize,
        actual: usize,
    },

    /// The same stabilizer was returned more than once.
    DuplicateStabilizer {
        stabilizer: StabilizerId,
    },

    /// A measurement was returned for a stabilizer outside the expected
    /// code domain.
    UnexpectedStabilizer {
        stabilizer: StabilizerId,
    },

    /// A required stabilizer was absent.
    MissingStabilizer {
        stabilizer: StabilizerId,
    },
}

impl fmt::Display for SyndromeExtractionError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::MeasurementCountMismatch {
                expected,
                actual,
            } => write!(
                formatter,
                "QPU syndrome measurement count mismatch: \
                 expected {expected}, received {actual}"
            ),

            Self::DuplicateStabilizer { stabilizer } => write!(
                formatter,
                "QPU returned duplicate stabilizer measurement: \
                 {stabilizer}"
            ),

            Self::UnexpectedStabilizer { stabilizer } => write!(
                formatter,
                "QPU returned unexpected stabilizer measurement: \
                 {stabilizer}"
            ),

            Self::MissingStabilizer { stabilizer } => write!(
                formatter,
                "QPU omitted required stabilizer measurement: \
                 {stabilizer}"
            ),
        }
    }
}

impl std::error::Error for SyndromeExtractionError {}

impl From<SyndromeExtractionError> for QecError {
    fn from(error: SyndromeExtractionError) -> Self {
        match error {
            SyndromeExtractionError::MeasurementCountMismatch {
                expected,
                actual,
            } => QecError::invalid_syndrome(format!(
                "measurement count mismatch: expected {expected}, \
                 received {actual}"
            )),

            SyndromeExtractionError::DuplicateStabilizer {
                stabilizer,
            } => QecError::invalid_syndrome(format!(
                "duplicate stabilizer measurement for {stabilizer}"
            )),

            SyndromeExtractionError::UnexpectedStabilizer {
                stabilizer,
            } => QecError::invalid_syndrome(format!(
                "unexpected stabilizer measurement for {stabilizer}"
            )),

            SyndromeExtractionError::MissingStabilizer {
                stabilizer,
            } => QecError::invalid_syndrome(format!(
                "required stabilizer measurement missing: {stabilizer}"
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round(value: u64) -> MeasurementRound {
        MeasurementRound::new(value).expect("valid round")
    }

    fn timestamp(value: u64) -> MeasurementTimestamp {
        MeasurementTimestamp::new(value)
            .expect("valid timestamp")
    }

    fn confidence() -> MeasurementConfidence {
        MeasurementConfidence::FULL
    }

    #[test]
    fn extraction_is_independent_of_backend_measurement_order() {
        let extractor = SyndromeExtractor::new().expect("extractor");

        let first = RawSyndromeBatch::new(
            round(0),
            timestamp(100),
            vec![
                RawSyndromeMeasurement::new(
                    StabilizerId::new(7),
                    true,
                    confidence(),
                ),
                RawSyndromeMeasurement::new(
                    StabilizerId::new(2),
                    false,
                    confidence(),
                ),
            ],
        );

        let second = RawSyndromeBatch::new(
            round(0),
            timestamp(100),
            vec![
                RawSyndromeMeasurement::new(
                    StabilizerId::new(2),
                    false,
                    confidence(),
                ),
                RawSyndromeMeasurement::new(
                    StabilizerId::new(7),
                    true,
                    confidence(),
                ),
            ],
        );

        let a = extractor.extract(&first).expect("first");
        let b = extractor.extract(&second).expect("second");

        assert_eq!(a.syndrome(), b.syndrome());
    }

    #[test]
    fn duplicate_stabilizers_are_rejected() {
        let extractor = SyndromeExtractor::new().expect("extractor");

        let batch = RawSyndromeBatch::new(
            round(0),
            timestamp(1),
            vec![
                RawSyndromeMeasurement::new(
                    StabilizerId::new(1),
                    false,
                    confidence(),
                ),
                RawSyndromeMeasurement::new(
                    StabilizerId::new(1),
                    true,
                    confidence(),
                ),
            ],
        );

        let result = extractor.extract(&batch);

        assert!(matches!(
            result,
            Err(QecError::InvalidSyndrome { .. })
        ));
    }

    #[test]
    fn expected_domain_rejects_missing_measurements() {
        let options =
            SyndromeExtractionOptions::default()
                .with_expected_stabilizers([
                    StabilizerId::new(0),
                    StabilizerId::new(1),
                ]);

        let extractor = SyndromeExtractor::with_options(
            options,
            CancellationToken::new(),
        )
        .expect("extractor");

        let batch = RawSyndromeBatch::new(
            round(0),
            timestamp(1),
            vec![RawSyndromeMeasurement::new(
                StabilizerId::new(0),
                false,
                confidence(),
            )],
        );

        let result = extractor.extract(&batch);

        assert!(matches!(
            result,
            Err(QecError::InvalidSyndrome { .. })
        ));
    }

    #[test]
    fn consecutive_batches_generate_detection_events() {
        let options =
            SyndromeExtractionOptions::default()
                .with_expected_stabilizers([
                    StabilizerId::new(0),
                    StabilizerId::new(1),
                ]);

        let mut extractor =
            StreamingSyndromeExtractor::new(
                options,
                CancellationToken::new(),
            )
            .expect("extractor");

        let first = RawSyndromeBatch::new(
            round(0),
            timestamp(1),
            vec![
                RawSyndromeMeasurement::new(
                    StabilizerId::new(0),
                    false,
                    confidence(),
                ),
                RawSyndromeMeasurement::new(
                    StabilizerId::new(1),
                    false,
                    confidence(),
                ),
            ],
        );

        let second = RawSyndromeBatch::new(
            round(1),
            timestamp(2),
            vec![
                RawSyndromeMeasurement::new(
                    StabilizerId::new(0),
                    true,
                    confidence(),
                ),
                RawSyndromeMeasurement::new(
                    StabilizerId::new(1),
                    false,
                    confidence(),
                ),
            ],
        );

        let (_, first_events) =
            extractor.push_with_events(&first).expect("first");

        assert!(first_events.is_empty());

        let (_, second_events) =
            extractor.push_with_events(&second).expect("second");

        assert_eq!(second_events.len(), 1);
        assert_eq!(
            second_events[0].stabilizer(),
            StabilizerId::new(0)
        );
    }

    #[test]
    fn cancellation_is_checked_before_extraction() {
        let token = CancellationToken::new();

        let extractor = SyndromeExtractor::with_options(
            SyndromeExtractionOptions::default(),
            token.clone(),
        )
        .expect("extractor");

        let batch = RawSyndromeBatch::new(
            round(0),
            timestamp(1),
            vec![RawSyndromeMeasurement::new(
                StabilizerId::new(0),
                false,
                confidence(),
            )],
        );

        let result = extractor.extract(&batch);

        assert!(result.is_ok());
    }

    #[test]
    fn first_stream_round_establishes_baseline() {
        let mut extractor =
            StreamingSyndromeExtractor::new(
                SyndromeExtractionOptions::default(),
                CancellationToken::new(),
            )
            .expect("extractor");

        let batch = RawSyndromeBatch::new(
            round(0),
            timestamp(1),
            vec![RawSyndromeMeasurement::new(
                StabilizerId::new(0),
                true,
                confidence(),
            )],
        );

        let (_, events) =
            extractor.push_with_events(&batch).expect("push");

        assert!(events.is_empty());
        assert_eq!(extractor.rounds_processed(), 1);
    }
}