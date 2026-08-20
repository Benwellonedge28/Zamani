//! Zamani Quantum Error Correction — bounded syndrome streaming.
//!
//! This module is the execution-layer streaming boundary for the QEC
//! subsystem. Mathematical syndrome processing remains in `syndrome.rs`;
//! cooperative cancellation remains in `cancellation.rs`; resource policy
//! remains in `limits.rs`; canonical failures remain in `errors.rs`.
//!
//! Architecture:
//!
//! ```text
//!                  UNTRUSTED SYNDROME SOURCE
//!                             │
//!                             ▼
//!                    ┌──────────────────┐
//!                    │ QecLimits        │
//!                    │ Validation       │
//!                    └────────┬─────────┘
//!                             │
//!                             ▼
//!                    ┌──────────────────┐
//!                    │ Bounded Queue    │
//!                    │ Backpressure     │
//!                    │ Ordering         │
//!                    └────────┬─────────┘
//!                             │
//!                             ▼
//!                    ┌──────────────────┐
//!                    │ SyndromeProcessor│
//!                    │                  │
//!                    │ XOR rounds       │
//!                    │ cancellation     │
//!                    │ resource checks  │
//!                    └────────┬─────────┘
//!                             │
//!                             ▼
//!                    Detection Events
//!                             │
//!              ┌──────────────┼──────────────┐
//!              ▼              ▼              ▼
//!           Decoder       Graph          Checkpoint
//!
//! ```
//!
//! ## Important invariants
//!
//! * No unbounded input queue.
//! * No unbounded output accumulation.
//! * No duplicate cancellation abstraction.
//! * No silent syndrome loss in lossless mode.
//! * No out-of-order measurement rounds.
//! * No sequence-number wrapping.
//! * No unchecked resource arithmetic.
//! * No decoding-specific logic.
//! * No thread creation or async-runtime ownership.
//! * Cancellation is checked before every externally visible operation and
//!   before every potentially expensive processing step.
//!
//! Streaming means that the complete syndrome history does not need to remain
//! in memory. It does **not** mean unlimited memory, unlimited execution time,
//! or unlimited event production.

use std::collections::VecDeque;
use std::fmt;

use super::cancellation::CancellationToken;
use super::errors::{QecError, QecResult, ResourceKind};
use super::limits::QecLimits;
use super::syndrome::{
    DetectionEvent,
    MeasurementRound,
    Syndrome,
    SyndromeProcessor,
    SyndromeSource,
};

// ============================================================================
// Constants
// ============================================================================

/// Current logical streaming-state schema.
pub const STREAM_STATE_VERSION: u16 = 2;

/// Conservative default input-buffer size.
pub const DEFAULT_MAX_BUFFERED_SYNDROMES: usize = 1_024;

/// Maximum number of rounds returned by one polling operation.
pub const DEFAULT_MAX_POLL_ROUNDS: usize = 1_024;

/// Stream-local sequence values reserve `u64::MAX` for overflow detection.
pub const MAX_STREAM_SEQUENCE: u64 = u64::MAX - 1;

// ============================================================================
// Backpressure
// ============================================================================

/// Policy used when the bounded input buffer is full.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BackpressurePolicy {
    /// Reject the new syndrome.
    ///
    /// This is the only policy permitted for lossless QEC decoding.
    Reject,

    /// Reject the newest syndrome while retaining buffered data.
    ///
    /// This is lossy and is only appropriate for explicitly lossy monitoring.
    DropNewest,

    /// Remove the oldest buffered syndrome before accepting the new one.
    ///
    /// This is lossy and must never silently be used for mathematical QEC
    /// decoding.
    DropOldest,
}

impl BackpressurePolicy {
    /// Returns whether the policy preserves every submitted syndrome.
    #[must_use]
    pub const fn is_lossless(self) -> bool {
        matches!(self, Self::Reject)
    }
}

// ============================================================================
// Stream mode
// ============================================================================

/// Controls how much processed syndrome history the stream retains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StreamMode {
    /// Retain only the state required for consecutive-round processing.
    Minimal,

    /// Retain a bounded history for replay/inspection.
    Windowed {
        /// Number of complete syndrome rounds retained.
        rounds: usize,
    },
}

impl StreamMode {
    /// Returns the effective history size.
    #[must_use]
    pub const fn rounds(self) -> usize {
        match self {
            Self::Minimal => 2,
            Self::Windowed { rounds } if rounds < 2 => 2,
            Self::Windowed { rounds } => rounds,
        }
    }
}

// ============================================================================
// Lifecycle
// ============================================================================

/// Lifecycle state of a syndrome stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StreamStatus {
    /// Accepting new input.
    Open,

    /// Input has been closed but buffered work remains.
    Closing,

    /// Input is closed and all buffered work has been processed.
    Closed,

    /// Cancellation was observed.
    Cancelled,

    /// A terminal processing failure occurred.
    Failed,
}

impl StreamStatus {
    /// Returns whether input can be submitted.
    #[must_use]
    pub const fn accepts_input(self) -> bool {
        matches!(self, Self::Open)
    }

    /// Returns whether the stream is terminal.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Closed | Self::Cancelled | Self::Failed
        )
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Streaming-specific diagnostic information.
///
/// Public methods convert these conditions to `QecError`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StreamingError {
    /// Input is not currently accepted.
    NotAcceptingInput,

    /// Physical measurement rounds are not consecutive.
    OutOfOrderRound {
        expected: u64,
        received: u64,
    },

    /// Stream sequence number would wrap.
    SequenceOverflow,

    /// Input buffer reached its configured capacity.
    BufferFull {
        capacity: usize,
    },

    /// Requested window is invalid.
    InvalidWindowSize {
        requested: usize,
    },

    /// Requested polling size is invalid.
    InvalidPollSize {
        requested: usize,
    },

    /// Lossy operation was requested without explicit permission.
    LossyBackpressureNotAllowed,

    /// Snapshot belongs to another schema.
    UnsupportedStateVersion {
        version: u16,
    },

    /// Snapshot violates a stream invariant.
    InvalidState {
        message: String,
    },
}

impl fmt::Display for StreamingError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::NotAcceptingInput => {
                write!(
                    formatter,
                    "syndrome stream is not accepting input"
                )
            }

            Self::OutOfOrderRound {
                expected,
                received,
            } => {
                write!(
                    formatter,
                    "out-of-order syndrome round: expected {}, received {}",
                    expected,
                    received
                )
            }

            Self::SequenceOverflow => {
                write!(
                    formatter,
                    "syndrome stream sequence number overflow"
                )
            }

            Self::BufferFull { capacity } => {
                write!(
                    formatter,
                    "syndrome stream buffer is full (capacity {})",
                    capacity
                )
            }

            Self::InvalidWindowSize { requested } => {
                write!(
                    formatter,
                    "invalid syndrome window size {}; minimum is 2",
                    requested
                )
            }

            Self::InvalidPollSize { requested } => {
                write!(
                    formatter,
                    "invalid poll size {}; minimum is 1",
                    requested
                )
            }

            Self::LossyBackpressureNotAllowed => {
                write!(
                    formatter,
                    "lossy backpressure is not permitted for lossless QEC"
                )
            }

            Self::UnsupportedStateVersion { version } => {
                write!(
                    formatter,
                    "unsupported streaming state version {}",
                    version
                )
            }

            Self::InvalidState { message } => {
                write!(
                    formatter,
                    "invalid streaming state: {}",
                    message
                )
            }
        }
    }
}

impl std::error::Error for StreamingError {}

// ============================================================================
// Metrics
// ============================================================================

/// Streaming metrics.
///
/// These counters are intentionally independent from global telemetry. A
/// caller may feed this snapshot into `metrics.rs` without making streaming
/// itself responsible for telemetry policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct StreamMetrics {
    /// Accepted syndrome rounds.
    pub syndromes_accepted: u64,

    /// Rejected syndrome submissions.
    pub syndromes_rejected: u64,

    /// Detection events generated.
    pub detection_events_generated: u64,

    /// Detection events delivered to a consumer.
    pub detection_events_delivered: u64,

    /// Current input-buffer occupancy.
    pub buffered_syndromes: u64,

    /// Peak input-buffer occupancy.
    pub peak_buffered_syndromes: u64,

    /// Number of cancellation checks.
    pub cancellation_checks: u64,

    /// Number of backpressure incidents.
    pub backpressure_events: u64,

    /// Number of successfully processed rounds.
    pub rounds_processed: u64,

    /// Number of polling operations.
    pub polls: u64,

    /// Number of flush operations.
    pub flushes: u64,

    /// Number of intentionally dropped syndromes.
    ///
    /// Non-zero values indicate a lossy stream and therefore invalidate
    /// lossless-QEC assumptions.
    pub syndromes_dropped: u64,
}

// ============================================================================
// Stream item
// ============================================================================

/// A syndrome submitted to the stream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamItem {
    sequence: u64,
    syndrome: Syndrome,
}

impl StreamItem {
    /// Returns the stream-local sequence number.
    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    /// Returns the syndrome.
    #[must_use]
    pub const fn syndrome(&self) -> &Syndrome {
        &self.syndrome
    }
}

// ============================================================================
// Stream output
// ============================================================================

/// Result of processing one syndrome round.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamOutput {
    /// Stream-local sequence number.
    pub sequence: u64,

    /// Physical measurement round.
    pub round: MeasurementRound,

    /// Detection events generated against the previous round.
    pub events: Vec<DetectionEvent>,
}

impl StreamOutput {
    /// Returns whether this output contains no detection events.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Returns the number of detection events.
    #[must_use]
    pub fn len(&self) -> usize {
        self.events.len()
    }
}

// ============================================================================
// Snapshot
// ============================================================================

/// In-memory snapshot of stream execution state.
///
/// This is deliberately not a wire-format checkpoint. `checkpoint.rs` owns
/// serialization, compatibility validation, integrity protection and durable
/// storage.
///
/// The snapshot contains enough state to reconstruct deterministic stream
/// processing without serializing the cancellation primitive itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamSnapshot {
    /// Snapshot schema.
    pub version: u16,

    /// Stream lifecycle.
    pub status: StreamStatus,

    /// Next sequence number.
    pub next_sequence: u64,

    /// Stream mode.
    pub mode: StreamMode,

    /// Backpressure policy.
    pub backpressure: BackpressurePolicy,

    /// Whether lossy processing has been explicitly authorized.
    pub lossy_allowed: bool,

    /// Configured buffer capacity.
    pub buffer_capacity: usize,

    /// Buffered input.
    pub buffered_input: Vec<StreamItem>,

    /// Processed syndrome history.
    pub history: Vec<Syndrome>,

    /// Metrics at snapshot time.
    pub metrics: StreamMetrics,
}

// ============================================================================
// Syndrome stream
// ============================================================================

/// Bounded, deterministic syndrome streaming infrastructure.
///
/// `SyndromeStream` is intentionally synchronous. Scheduling, threading,
/// distributed transport and QPU transport belong to their respective
/// infrastructure modules.
pub struct SyndromeStream {
    limits: QecLimits,
    cancellation: CancellationToken,

    mode: StreamMode,
    backpressure: BackpressurePolicy,
    lossy_allowed: bool,

    buffer_capacity: usize,
    buffer: VecDeque<StreamItem>,

    history: VecDeque<Syndrome>,

    processor: SyndromeProcessor,

    next_sequence: u64,
    status: StreamStatus,

    metrics: StreamMetrics,
}

impl std::fmt::Debug for SyndromeStream {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("SyndromeStream")
            .field("limits", &self.limits)
            .field("mode", &self.mode)
            .field("backpressure", &self.backpressure)
            .field("lossy_allowed", &self.lossy_allowed)
            .field("buffer_capacity", &self.buffer_capacity)
            .field("buffered_input", &self.buffer.len())
            .field("history_len", &self.history.len())
            .field("next_sequence", &self.next_sequence)
            .field("status", &self.status)
            .field("metrics", &self.metrics)
            .finish()
    }
}

impl SyndromeStream {
    /// Creates a stream with the canonical default resource policy.
    pub fn new() -> QecResult<Self> {
        Self::with_limits_and_cancellation(
            QecLimits::default(),
            CancellationToken::new(),
        )
    }

    /// Creates a stream with explicit resource limits.
    pub fn with_limits(
        limits: QecLimits,
    ) -> QecResult<Self> {
        Self::with_limits_and_cancellation(
            limits,
            CancellationToken::new(),
        )
    }

    /// Creates a stream with explicit limits and the canonical cancellation
    /// token.
    pub fn with_limits_and_cancellation(
        limits: QecLimits,
        cancellation: CancellationToken,
    ) -> QecResult<Self> {
        limits.validate().map_err(|error| {
            QecError::invalid_input(format!(
                "invalid QEC streaming limits: {}",
                error
            ))
        })?;

        let capacity = DEFAULT_MAX_BUFFERED_SYNDROMES
            .min(limits.max_stream_buffer_events)
            .min(limits.max_syndrome_events);

        if capacity == 0 {
            return Err(QecError::resource_limit(
                ResourceKind::StreamBuffer,
                1,
                0,
                "stream input buffer capacity must be greater than zero",
            ));
        }

        let processor =
            SyndromeProcessor::with_limits(
                limits,
                cancellation.clone(),
            )?;

        Ok(Self {
            limits,
            cancellation,

            mode: StreamMode::Minimal,
            backpressure: BackpressurePolicy::Reject,
            lossy_allowed: false,

            buffer_capacity: capacity,
            buffer: VecDeque::with_capacity(capacity),

            history: VecDeque::new(),

            processor,

            next_sequence: 0,
            status: StreamStatus::Open,

            metrics: StreamMetrics::default(),
        })
    }

    // ------------------------------------------------------------------------
    // Accessors
    // ------------------------------------------------------------------------

    /// Returns the configured QEC limits.
    #[must_use]
    pub const fn limits(&self) -> QecLimits {
        self.limits
    }

    /// Returns the stream status.
    #[must_use]
    pub const fn status(&self) -> StreamStatus {
        self.status
    }

    /// Returns the current stream mode.
    #[must_use]
    pub const fn mode(&self) -> StreamMode {
        self.mode
    }

    /// Returns the configured backpressure policy.
    #[must_use]
    pub const fn backpressure_policy(
        &self,
    ) -> BackpressurePolicy {
        self.backpressure
    }

    /// Returns whether lossy processing has explicitly been enabled.
    #[must_use]
    pub const fn lossy_allowed(&self) -> bool {
        self.lossy_allowed
    }

    /// Returns stream metrics.
    #[must_use]
    pub const fn metrics(&self) -> StreamMetrics {
        self.metrics
    }

    /// Returns input-buffer occupancy.
    #[must_use]
    pub fn buffered_len(&self) -> usize {
        self.buffer.len()
    }

    /// Returns configured input-buffer capacity.
    #[must_use]
    pub const fn capacity(&self) -> usize {
        self.buffer_capacity
    }

    /// Returns whether no input is buffered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Returns retained syndrome history.
    #[must_use]
    pub fn history(&self) -> impl Iterator<Item = &Syndrome> {
        self.history.iter()
    }

    // ------------------------------------------------------------------------
    // Configuration
    // ------------------------------------------------------------------------

    /// Configures minimal-history operation.
    pub fn set_minimal_mode(&mut self) {
        self.mode = StreamMode::Minimal;

        self.history.clear();
    }

    /// Configures bounded windowed history.
    pub fn set_windowed_mode(
        &mut self,
        rounds: usize,
    ) -> QecResult<()> {
        if rounds < 2 {
            return Err(
                QecError::invalid_input(
                    StreamingError::InvalidWindowSize {
                        requested: rounds,
                    }
                    .to_string(),
                ),
            );
        }

        if rounds > self.limits.max_rounds {
            return Err(QecError::resource_limit(
                ResourceKind::MeasurementRounds,
                rounds as u128,
                self.limits.max_rounds as u128,
                "streaming history window exceeds QEC round limit",
            ));
        }

        self.mode = StreamMode::Windowed { rounds };

        while self.history.len() > rounds {
            self.history.pop_front();
        }

        Ok(())
    }

    /// Configures the backpressure policy.
    ///
    /// Lossy policies require explicit opt-in through
    /// [`Self::allow_lossy_processing`].
    pub fn set_backpressure_policy(
        &mut self,
        policy: BackpressurePolicy,
    ) -> QecResult<()> {
        if !policy.is_lossless() && !self.lossy_allowed {
            return Err(
                QecError::unsupported(
                    "lossy_streaming",
                    StreamingError::LossyBackpressureNotAllowed
                        .to_string(),
                ),
            );
        }

        self.backpressure = policy;

        Ok(())
    }

    /// Explicitly authorizes lossy monitoring/telemetry behavior.
    ///
    /// This must never be enabled by a decoder automatically.
    pub fn allow_lossy_processing(
        &mut self,
        allowed: bool,
    ) {
        self.lossy_allowed = allowed;

        if !allowed && !self.backpressure.is_lossless() {
            self.backpressure =
                BackpressurePolicy::Reject;
        }
    }

    // ------------------------------------------------------------------------
    // Cancellation
    // ------------------------------------------------------------------------

    #[inline]
    fn check_cancellation(&mut self) -> QecResult<()> {
        self.metrics.cancellation_checks =
            self.metrics
                .cancellation_checks
                .checked_add(1)
                .ok_or_else(|| {
                    QecError::internal_invariant(
                        "stream cancellation metric overflow",
                        "cancellation check counter overflowed",
                    )
                })?;

        match self.cancellation.check() {
            Ok(()) => Ok(()),
            Err(error) => {
                self.status = StreamStatus::Cancelled;
                Err(error)
            }
        }
    }

    // ------------------------------------------------------------------------
    // Sequence numbers
    // ------------------------------------------------------------------------

    fn allocate_sequence(&mut self) -> QecResult<u64> {
        if self.next_sequence > MAX_STREAM_SEQUENCE {
            return Err(
                QecError::internal_invariant(
                    "stream sequence does not wrap",
                    StreamingError::SequenceOverflow
                        .to_string(),
                ),
            );
        }

        let sequence = self.next_sequence;

        self.next_sequence =
            self.next_sequence
                .checked_add(1)
                .ok_or_else(|| {
                    QecError::internal_invariant(
                        "stream sequence arithmetic is checked",
                        StreamingError::SequenceOverflow
                            .to_string(),
                    )
                })?;

        Ok(sequence)
    }

    // ------------------------------------------------------------------------
    // Input
    // ------------------------------------------------------------------------

    /// Submits one syndrome to the bounded input queue.
    ///
    /// Submission itself does not perform decoding. This creates a genuine
    /// producer/consumer boundary and therefore permits scheduler-driven
    /// backpressure.
    pub fn submit(
        &mut self,
        syndrome: Syndrome,
    ) -> QecResult<u64> {
        self.check_cancellation()?;

        if !self.status.accepts_input() {
            self.metrics.syndromes_rejected =
                self.metrics
                    .syndromes_rejected
                    .checked_add(1)
                    .ok_or_else(|| {
                        QecError::internal_invariant(
                            "syndrome rejection metric is monotonic",
                            "rejection counter overflowed",
                        )
                    })?;

            return Err(
                QecError::invalid_syndrome(
                    StreamingError::NotAcceptingInput
                        .to_string(),
                ),
            );
        }

        /*
         * Validate the syndrome before putting it into the queue.
         * This prevents malformed input from consuming buffer capacity.
         */
        syndrome.preflight()?;

        /*
         * Check the configured round ceiling before allocation.
         *
         * `SyndromeProcessor` performs the authoritative consecutive-round
         * validation when processing begins.
         */
        if self.metrics.syndromes_accepted
            >= u64::try_from(self.limits.max_rounds)
                .map_err(|_| {
                    QecError::numerical_failure(
                        super::errors::NumericalOperation::IntegerConversion,
                        "maximum round limit does not fit in u64",
                    )
                })?
        {
            return Err(QecError::resource_limit(
                ResourceKind::MeasurementRounds,
                self.metrics.syndromes_accepted as u128 + 1,
                self.limits.max_rounds as u128,
                "maximum syndrome-round limit exceeded",
            ));
        }

        if self.buffer.len() >= self.buffer_capacity {
            self.metrics.backpressure_events =
                self.metrics
                    .backpressure_events
                    .checked_add(1)
                    .ok_or_else(|| {
                        QecError::internal_invariant(
                            "backpressure metric is monotonic",
                            "backpressure counter overflowed",
                        )
                    })?;

            match self.backpressure {
                BackpressurePolicy::Reject => {
                    self.metrics.syndromes_rejected =
                        self.metrics
                            .syndromes_rejected
                            .checked_add(1)
                            .ok_or_else(|| {
                                QecError::internal_invariant(
                                    "syndrome rejection metric is monotonic",
                                    "rejection counter overflowed",
                                )
                            })?;

                    return Err(
                        QecError::resource_limit(
                            ResourceKind::StreamBuffer,
                            self.buffer.len() as u128 + 1,
                            self.buffer_capacity as u128,
                            StreamingError::BufferFull {
                                capacity: self.buffer_capacity,
                            }
                            .to_string(),
                        ),
                    );
                }

                BackpressurePolicy::DropNewest => {
                    self.metrics.syndromes_rejected =
                        self.metrics
                            .syndromes_rejected
                            .checked_add(1)
                            .ok_or_else(|| {
                                QecError::internal_invariant(
                                    "syndrome rejection metric is monotonic",
                                    "rejection counter overflowed",
                                )
                            })?;

                    self.metrics.syndromes_dropped =
                        self.metrics
                            .syndromes_dropped
                            .checked_add(1)
                            .ok_or_else(|| {
                                QecError::internal_invariant(
                                    "drop metric is monotonic",
                                    "drop counter overflowed",
                                )
                            })?;

                    return Ok(self.next_sequence);
                }

                BackpressurePolicy::DropOldest => {
                    let _ = self.buffer.pop_front();

                    self.metrics.syndromes_dropped =
                        self.metrics
                            .syndromes_dropped
                            .checked_add(1)
                            .ok_or_else(|| {
                                QecError::internal_invariant(
                                    "drop metric is monotonic",
                                    "drop counter overflowed",
                                )
                            })?;
                }
            }
        }

        let sequence = self.allocate_sequence()?;

        self.buffer.push_back(StreamItem {
            sequence,
            syndrome,
        });

        self.metrics.syndromes_accepted =
            self.metrics
                .syndromes_accepted
                .checked_add(1)
                .ok_or_else(|| {
                    QecError::internal_invariant(
                        "syndrome acceptance metric is monotonic",
                        "acceptance counter overflowed",
                    )
                })?;

        self.metrics.buffered_syndromes =
            u64::try_from(self.buffer.len())
                .map_err(|_| {
                    QecError::numerical_failure(
                        super::errors::NumericalOperation::IntegerConversion,
                        "stream buffer length does not fit in u64",
                    )
                })?;

        self.metrics.peak_buffered_syndromes =
            self.metrics
                .peak_buffered_syndromes
                .max(self.metrics.buffered_syndromes);

        Ok(sequence)
    }

    /// Compatibility alias for producer-oriented callers.
    pub fn push(
        &mut self,
        syndrome: Syndrome,
    ) -> QecResult<u64> {
        self.submit(syndrome)
    }

    // ------------------------------------------------------------------------
    // Processing
    // ------------------------------------------------------------------------

    /// Processes the next buffered syndrome.
    ///
    /// Returns `None` when the input queue is empty.
    pub fn process_next(
        &mut self,
    ) -> QecResult<Option<StreamOutput>> {
        self.check_cancellation()?;

        let item = match self.buffer.pop_front() {
            Some(item) => item,
            None => {
                self.maybe_close_after_drain();
                return Ok(None);
            }
        };

        self.metrics.buffered_syndromes =
            u64::try_from(self.buffer.len())
                .map_err(|_| {
                    QecError::numerical_failure(
                        super::errors::NumericalOperation::IntegerConversion,
                        "stream buffer length does not fit in u64",
                    )
                })?;

        /*
         * The mathematical processing path is centralized in
         * `SyndromeProcessor`. This prevents streaming.rs from maintaining a
         * second XOR implementation with subtly different semantics.
         */
        let events = match self.processor.push(
            item.syndrome.clone(),
        ) {
            Ok(events) => events,
            Err(error) => {
                self.status = if matches!(
                    error,
                    QecError::CancellationRequested { .. }
                ) {
                    StreamStatus::Cancelled
                } else {
                    StreamStatus::Failed
                };

                return Err(error);
            }
        };

        self.metrics.rounds_processed =
            self.metrics
                .rounds_processed
                .checked_add(1)
                .ok_or_else(|| {
                    QecError::internal_invariant(
                        "processed-round counter is monotonic",
                        "processed-round counter overflowed",
                    )
                })?;

        let generated =
            u64::try_from(events.len())
                .map_err(|_| {
                    QecError::numerical_failure(
                        super::errors::NumericalOperation::IntegerConversion,
                        "event count does not fit in u64",
                    )
                })?;

        self.metrics.detection_events_generated =
            self.metrics
                .detection_events_generated
                .checked_add(generated)
                .ok_or_else(|| {
                    QecError::internal_invariant(
                        "detection-event metric is monotonic",
                        "detection-event counter overflowed",
                    )
                })?;

        self.record_history(item.syndrome.clone());

        self.maybe_close_after_drain();

        Ok(Some(StreamOutput {
            sequence: item.sequence,
            round: item.syndrome.round(),
            events,
        }))
    }

    fn record_history(
        &mut self,
        syndrome: Syndrome,
    ) {
        match self.mode {
            StreamMode::Minimal => {
                /*
                 * The processor owns the mathematical previous-round state.
                 * Minimal mode therefore intentionally stores no duplicate
                 * history here.
                 */
            }

            StreamMode::Windowed { rounds } => {
                self.history.push_back(syndrome);

                while self.history.len() > rounds {
                    self.history.pop_front();
                }
            }
        }
    }

    /// Processes up to `max_rounds` buffered syndrome rounds.
    pub fn poll(
        &mut self,
        max_rounds: usize,
    ) -> QecResult<Vec<StreamOutput>> {
        self.check_cancellation()?;

        if max_rounds == 0 {
            return Err(
                QecError::invalid_input(
                    StreamingError::InvalidPollSize {
                        requested: max_rounds,
                    }
                    .to_string(),
                ),
            );
        }

        let max_rounds =
            max_rounds.min(DEFAULT_MAX_POLL_ROUNDS);

        let mut outputs =
            Vec::with_capacity(max_rounds.min(self.buffer.len()));

        for _ in 0..max_rounds {
            self.check_cancellation()?;

            match self.process_next()? {
                Some(output) => {
                    let event_count =
                        u64::try_from(output.events.len())
                            .map_err(|_| {
                                QecError::numerical_failure(
                                    super::errors::NumericalOperation::IntegerConversion,
                                    "event count does not fit in u64",
                                )
                            })?;

                    self.metrics.detection_events_delivered =
                        self.metrics
                            .detection_events_delivered
                            .checked_add(event_count)
                            .ok_or_else(|| {
                                QecError::internal_invariant(
                                    "delivered-event metric is monotonic",
                                    "delivered-event counter overflowed",
                                )
                            })?;

                    outputs.push(output);
                }

                None => break,
            }
        }

        self.metrics.polls =
            self.metrics
                .polls
                .checked_add(1)
                .ok_or_else(|| {
                    QecError::internal_invariant(
                        "poll counter is monotonic",
                        "poll counter overflowed",
                    )
                })?;

        Ok(outputs)
    }

    /// Drains all currently buffered input.
    ///
    /// The queue itself is bounded, so the returned vector is bounded by the
    /// configured stream capacity.
    pub fn flush(
        &mut self,
    ) -> QecResult<Vec<StreamOutput>> {
        self.check_cancellation()?;

        let mut outputs =
            Vec::with_capacity(self.buffer.len());

        while !self.buffer.is_empty() {
            self.check_cancellation()?;

            if let Some(output) = self.process_next()? {
                let event_count =
                    u64::try_from(output.events.len())
                        .map_err(|_| {
                            QecError::numerical_failure(
                                super::errors::NumericalOperation::IntegerConversion,
                                "event count does not fit in u64",
                            )
                        })?;

                self.metrics.detection_events_delivered =
                    self.metrics
                        .detection_events_delivered
                        .checked_add(event_count)
                        .ok_or_else(|| {
                            QecError::internal_invariant(
                                "delivered-event metric is monotonic",
                                "delivered-event counter overflowed",
                            )
                        })?;

                outputs.push(output);
            }
        }

        self.metrics.flushes =
            self.metrics
                .flushes
                .checked_add(1)
                .ok_or_else(|| {
                    QecError::internal_invariant(
                        "flush counter is monotonic",
                        "flush counter overflowed",
                    )
                })?;

        self.maybe_close_after_drain();

        Ok(outputs)
    }

    // ------------------------------------------------------------------------
    // Source integration
    // ------------------------------------------------------------------------

    /// Pulls at most `max_items` syndromes from an incremental source.
    ///
    /// Source transport remains outside this module.
    pub fn ingest_source<S>(
        &mut self,
        source: &mut S,
        max_items: usize,
    ) -> QecResult<usize>
    where
        S: SyndromeSource,
    {
        self.check_cancellation()?;

        if max_items == 0 {
            return Err(
                QecError::invalid_input(
                    StreamingError::InvalidPollSize {
                        requested: max_items,
                    }
                    .to_string(),
                ),
            );
        }

        let max_items =
            max_items.min(self.buffer_capacity);

        let mut accepted = 0usize;

        while accepted < max_items {
            self.check_cancellation()?;

            if self.buffer.len() >= self.buffer_capacity {
                break;
            }

            match source.next_syndrome()? {
                Some(syndrome) => {
                    self.submit(syndrome)?;
                    accepted = accepted
                        .checked_add(1)
                        .ok_or_else(|| {
                            QecError::internal_invariant(
                                "ingested-source counter is monotonic",
                                "source counter overflowed",
                            )
                        })?;
                }

                None => {
                    self.close_input()?;
                    break;
                }
            }
        }

        Ok(accepted)
    }

    /// Repeatedly ingests and processes a source without retaining the entire
    /// source history.
    ///
    /// This method returns outputs in deterministic stream order. The returned
    /// vector is bounded by `max_outputs`; callers needing unbounded operation
    /// should repeatedly call `ingest_source` and `poll`.
    pub fn process_source<S>(
        &mut self,
        source: &mut S,
        max_outputs: usize,
    ) -> QecResult<Vec<StreamOutput>>
    where
        S: SyndromeSource,
    {
        self.check_cancellation()?;

        if max_outputs == 0 {
            return Err(
                QecError::invalid_input(
                    StreamingError::InvalidPollSize {
                        requested: max_outputs,
                    }
                    .to_string(),
                ),
            );
        }

        let max_outputs =
            max_outputs.min(DEFAULT_MAX_POLL_ROUNDS);

        let mut outputs =
            Vec::with_capacity(max_outputs);

        while outputs.len() < max_outputs {
            self.check_cancellation()?;

            if self.buffer.is_empty()
                && self.status.accepts_input()
            {
                let ingested =
                    self.ingest_source(source, 1)?;

                if ingested == 0 {
                    break;
                }
            }

            match self.process_next()? {
                Some(output) => {
                    let event_count =
                        u64::try_from(output.events.len())
                            .map_err(|_| {
                                QecError::numerical_failure(
                                    super::errors::NumericalOperation::IntegerConversion,
                                    "event count does not fit in u64",
                                )
                            })?;

                    self.metrics.detection_events_delivered =
                        self.metrics
                            .detection_events_delivered
                            .checked_add(event_count)
                            .ok_or_else(|| {
                                QecError::internal_invariant(
                                    "delivered-event metric is monotonic",
                                    "delivered-event counter overflowed",
                                )
                            })?;

                    outputs.push(output);
                }

                None => {
                    if self.status.is_terminal() {
                        break;
                    }

                    break;
                }
            }
        }

        Ok(outputs)
    }

    // ------------------------------------------------------------------------
    // Lifecycle
    // ------------------------------------------------------------------------

    /// Stops accepting new syndrome rounds.
    ///
    /// Existing buffered input remains valid and can still be drained.
    pub fn close_input(&mut self) -> QecResult<()> {
        self.check_cancellation()?;

        if self.status == StreamStatus::Open {
            self.status = StreamStatus::Closing;
        }

        self.maybe_close_after_drain();

        Ok(())
    }

    fn maybe_close_after_drain(&mut self) {
        if self.status == StreamStatus::Closing
            && self.buffer.is_empty()
        {
            self.status = StreamStatus::Closed;
        }
    }

    /// Explicitly marks the stream cancelled.
    ///
    /// The underlying cancellation token remains owned by its source.
    pub fn cancel(&mut self) -> QecResult<()> {
        let _ = self.cancellation.request();

        self.status = StreamStatus::Cancelled;

        self.cancellation.check()
    }

    // ------------------------------------------------------------------------
    // Snapshot
    // ------------------------------------------------------------------------

    /// Creates a deterministic in-memory snapshot.
    ///
    /// Durable serialization and cryptographic integrity belong to
    /// `checkpoint.rs`.
    pub fn snapshot(&self) -> StreamSnapshot {
        StreamSnapshot {
            version: STREAM_STATE_VERSION,

            status: self.status,

            next_sequence: self.next_sequence,

            mode: self.mode,

            backpressure: self.backpressure,

            lossy_allowed: self.lossy_allowed,

            buffer_capacity: self.buffer_capacity,

            buffered_input: self.buffer.iter().cloned().collect(),

            history: self.history.iter().cloned().collect(),

            metrics: self.metrics,
        }
    }

    /// Validates a snapshot against the active resource policy.
    pub fn validate_snapshot(
        &self,
        snapshot: &StreamSnapshot,
    ) -> QecResult<()> {
        if snapshot.version != STREAM_STATE_VERSION {
            return Err(
                QecError::unsupported(
                    "stream_state_version",
                    StreamingError::UnsupportedStateVersion {
                        version: snapshot.version,
                    }
                    .to_string(),
                ),
            );
        }

        if snapshot.buffer_capacity == 0 {
            return Err(
                QecError::invalid_input(
                    StreamingError::InvalidState {
                        message:
                            "snapshot buffer capacity is zero"
                                .to_owned(),
                    }
                    .to_string(),
                ),
            );
        }

        if snapshot.buffer_capacity
            > self.limits.max_stream_buffer_events
        {
            return Err(QecError::resource_limit(
                ResourceKind::StreamBuffer,
                snapshot.buffer_capacity as u128,
                self.limits.max_stream_buffer_events
                    as u128,
                "snapshot buffer exceeds active QEC stream limit",
            ));
        }

        if snapshot.buffered_input.len()
            > snapshot.buffer_capacity
        {
            return Err(
                QecError::invalid_input(
                    StreamingError::InvalidState {
                        message:
                            "snapshot contains more buffered syndromes than capacity"
                                .to_owned(),
                    }
                    .to_string(),
                ),
            );
        }

        if snapshot.next_sequence
            > MAX_STREAM_SEQUENCE
        {
            return Err(
                QecError::invalid_input(
                    StreamingError::InvalidState {
                        message:
                            "snapshot sequence number is out of range"
                                .to_owned(),
                    }
                    .to_string(),
                ),
            );
        }

        if !snapshot.backpressure.is_lossless()
            && !snapshot.lossy_allowed
        {
            return Err(
                QecError::invalid_input(
                    StreamingError::InvalidState {
                        message:
                            "snapshot enables lossy backpressure without explicit authorization"
                                .to_owned(),
                    }
                    .to_string(),
                ),
            );
        }

        Ok(())
    }

    // ------------------------------------------------------------------------
    // Statistics
    // ------------------------------------------------------------------------

    /// Returns the total number of processed syndrome rounds.
    #[must_use]
    pub const fn rounds_processed(&self) -> u64 {
        self.metrics.rounds_processed
    }

    /// Returns the total number of generated detection events.
    #[must_use]
    pub const fn detection_events_generated(
        &self,
    ) -> u64 {
        self.metrics.detection_events_generated
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn limits() -> QecLimits {
        let mut limits = QecLimits::default();

        limits.max_stream_buffer_events = 4;
        limits.max_syndrome_events = 64;
        limits.max_rounds = 16;

        limits
    }

    fn syndrome(round: u64, value: bool) -> Syndrome {
        use super::super::syndrome::{
            MeasurementConfidence,
            MeasurementRound,
            MeasurementTimestamp,
            StabilizerId,
            SyndromeMeasurement,
        };

        let mut syndrome = Syndrome::new_with_limits(
            MeasurementRound::new(round).unwrap(),
            MeasurementTimestamp::new(round).unwrap(),
            limits(),
        )
        .unwrap();

        syndrome
            .insert(SyndromeMeasurement::new(
                StabilizerId::new(0),
                value,
                MeasurementConfidence::FULL,
            ))
            .unwrap();

        syndrome
    }

    #[test]
    fn default_stream_is_lossless() {
        let stream =
            SyndromeStream::with_limits(limits())
                .unwrap();

        assert_eq!(
            stream.backpressure_policy(),
            BackpressurePolicy::Reject
        );

        assert!(!stream.lossy_allowed());
        assert_eq!(
            stream.status(),
            StreamStatus::Open
        );
    }

    #[test]
    fn first_round_establishes_baseline() {
        let mut stream =
            SyndromeStream::with_limits(limits())
                .unwrap();

        stream.submit(syndrome(0, false)).unwrap();

        let output =
            stream.process_next().unwrap().unwrap();

        assert!(output.events.is_empty());
        assert_eq!(output.round.value(), 0);
    }

    #[test]
    fn consecutive_rounds_generate_detection_events() {
        let mut stream =
            SyndromeStream::with_limits(limits())
                .unwrap();

        stream.submit(syndrome(0, false)).unwrap();
        stream.submit(syndrome(1, true)).unwrap();

        let first =
            stream.process_next().unwrap().unwrap();

        assert!(first.events.is_empty());

        let second =
            stream.process_next().unwrap().unwrap();

        assert_eq!(second.events.len(), 1);
        assert_eq!(
            second.events[0].stabilizer().index(),
            0
        );
        assert!(second.events[0].value());
    }

    #[test]
    fn out_of_order_rounds_are_rejected() {
        let mut stream =
            SyndromeStream::with_limits(limits())
                .unwrap();

        stream.submit(syndrome(0, false)).unwrap();
        stream.submit(syndrome(2, true)).unwrap();

        stream.process_next().unwrap().unwrap();

        let result = stream.process_next();

        assert!(result.is_err());
        assert_eq!(
            stream.status(),
            StreamStatus::Failed
        );
    }

    #[test]
    fn buffer_is_bounded() {
        let mut stream =
            SyndromeStream::with_limits(limits())
                .unwrap();

        for round in 0..4 {
            stream
                .submit(syndrome(round, false))
                .unwrap();
        }

        let result =
            stream.submit(syndrome(4, false));

        assert!(result.is_err());
        assert_eq!(stream.buffered_len(), 4);
    }

    #[test]
    fn lossy_backpressure_requires_explicit_opt_in() {
        let mut stream =
            SyndromeStream::with_limits(limits())
                .unwrap();

        let result =
            stream.set_backpressure_policy(
                BackpressurePolicy::DropOldest,
            );

        assert!(result.is_err());

        stream.allow_lossy_processing(true);

        stream
            .set_backpressure_policy(
                BackpressurePolicy::DropOldest,
            )
            .unwrap();

        assert_eq!(
            stream.backpressure_policy(),
            BackpressurePolicy::DropOldest
        );
    }

    #[test]
    fn closing_drains_existing_input() {
        let mut stream =
            SyndromeStream::with_limits(limits())
                .unwrap();

        stream.submit(syndrome(0, false)).unwrap();
        stream.close_input().unwrap();

        assert_eq!(
            stream.status(),
            StreamStatus::Closing
        );

        stream.process_next().unwrap().unwrap();

        assert_eq!(
            stream.status(),
            StreamStatus::Closed
        );

        assert!(stream.submit(syndrome(1, false)).is_err());
    }

    #[test]
    fn snapshot_is_bounded_and_versioned() {
        let mut stream =
            SyndromeStream::with_limits(limits())
                .unwrap();

        stream.submit(syndrome(0, false)).unwrap();

        let snapshot = stream.snapshot();

        assert_eq!(
            snapshot.version,
            STREAM_STATE_VERSION
        );

        assert_eq!(
            snapshot.buffered_input.len(),
            1
        );

        assert!(
            stream.validate_snapshot(&snapshot).is_ok()
        );
    }

    #[test]
    fn cancellation_is_terminal() {
        let source = super::super::cancellation::CancellationSource::new();
        let token = source.token();

        let mut stream =
            SyndromeStream::with_limits_and_cancellation(
                limits(),
                token,
            )
            .unwrap();

        source.cancel();

        let result =
            stream.submit(syndrome(0, false));

        assert!(result.is_err());
        assert_eq!(
            stream.status(),
            StreamStatus::Cancelled
        );
    }

    #[test]
    fn deterministic_sequence_numbers() {
        let mut stream =
            SyndromeStream::with_limits(limits())
                .unwrap();

        let a =
            stream.submit(syndrome(0, false)).unwrap();

        let b =
            stream.submit(syndrome(1, false)).unwrap();

        assert_eq!(a, 0);
        assert_eq!(b, 1);
    }
}