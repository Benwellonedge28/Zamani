//! Zamani Quantum Error Correction — Production Syndrome Streaming.
//!
//! This module provides bounded, deterministic, cancellation-aware streaming
//! of syndrome measurements into detection events.
//!
//! # Architecture
//!
//! ```text
//!                    External syndrome source
//!                              │
//!                              ▼
//!                    ┌───────────────────┐
//!                    │   Validation      │
//!                    └─────────┬─────────┘
//!                              │
//!                              ▼
//!                    ┌───────────────────┐
//!                    │ SyndromeStream    │
//!                    │                   │
//!                    │ bounded buffer    │
//!                    │ ordering          │
//!                    │ backpressure      │
//!                    │ cancellation      │
//!                    └─────────┬─────────┘
//!                              │
//!                              ▼
//!                    ┌───────────────────┐
//!                    │ Detection Events  │
//!                    └─────────┬─────────┘
//!                              │
//!             ┌────────────────┼────────────────┐
//!             ▼                ▼                ▼
//!           Decoder        Decoding Graph    Checkpoint
//!
//! ```
//!
//! # Production properties
//!
//! - bounded memory;
//! - configurable resource limits;
//! - deterministic processing;
//! - strict measurement-round ordering;
//! - explicit duplicate/out-of-order protection;
//! - streaming detection-event generation;
//! - configurable windowing;
//! - backpressure;
//! - cancellation;
//! - graceful shutdown;
//! - overflow-safe sequence numbers;
//! - no panic-based handling of external input;
//! - no unbounded allocation;
//! - metrics suitable for telemetry;
//! - snapshot/restore of stream state;
//! - explicit end-of-stream semantics;
//! - decoder-independent API;
//! - suitable for synchronous, threaded, and future distributed backends.
//!
//! The module deliberately does not spawn threads or create an async runtime.
//! Scheduling belongs to `scheduler.rs`; transport belongs to future backend
//! implementations. This keeps the core streaming state deterministic and
//! portable.
//!
//! # Important scalability rule
//!
//! Streaming does not make computation literally infinite. It removes the
//! requirement to retain an entire syndrome history in memory. The stream is
//! still governed by explicit resource limits and bounded buffers.

use std::collections::VecDeque;
use std::fmt;

use super::errors::{QecError, QecResult, ResourceKind};
use super::limits::QecLimits;
use super::syndrome::{
    DetectionEvent,
    MeasurementRound,
    Syndrome,
};

// ============================================================================
// Constants
// ============================================================================

/// Current streaming-state schema version.
///
/// This must be incremented whenever the serialized checkpoint/snapshot
/// representation changes incompatibly.
pub const STREAM_STATE_VERSION: u16 = 1;

/// Maximum number of events that may be returned by one polling operation.
///
/// This prevents a caller from accidentally requesting an enormous temporary
/// allocation through a single `poll_events` call.
pub const DEFAULT_MAX_POLL_EVENTS: usize = 65_536;

/// Maximum number of syndromes retained by the stream by default.
///
/// The actual value is always capped by `QecLimits::max_stream_buffer_events`.
pub const DEFAULT_MAX_BUFFERED_SYNDROMES: usize = 1_024;

/// Default number of consecutive syndrome rounds retained for a sliding window.
pub const DEFAULT_WINDOW_ROUNDS: usize = 2;

/// Maximum stream sequence number.
///
/// `u64::MAX` is reserved so that overflow can be detected before increment.
pub const MAX_STREAM_SEQUENCE: u64 = u64::MAX - 1;

// ============================================================================
// Cancellation
// ============================================================================

/// Cancellation state observed by a syndrome stream.
///
/// The trait intentionally has only one method so it can be implemented by:
///
/// - a local atomic token;
/// - a scheduler;
/// - a distributed coordinator;
/// - a GUI cancellation mechanism;
/// - a future async runtime.
///
/// The stream itself does not own the cancellation mechanism.
pub trait CancellationToken {
    /// Returns `true` when the current operation should stop.
    fn is_cancelled(&self) -> bool;
}

/// A permanently active cancellation token.
///
/// Useful for tests and callers that want to construct a stream without
/// supplying a scheduler-owned token.
#[derive(Debug, Clone, Copy, Default)]
pub struct NeverCancel;

impl CancellationToken for NeverCancel {
    fn is_cancelled(&self) -> bool {
        false
    }
}

// ============================================================================
// Backpressure
// ============================================================================

/// Policy used when the stream's bounded buffer is full.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BackpressurePolicy {
    /// Reject the new item and leave stream state unchanged.
    Reject,

    /// Retain the oldest buffered item and reject the new item.
    ///
    /// This policy is intended for monitoring/telemetry scenarios only.
    /// It must not be used where every syndrome round is required for
    /// mathematically correct decoding.
    DropNewest,

    /// Remove the oldest item to make room for the new item.
    ///
    /// This is explicitly lossy and is therefore unsafe for lossless QEC
    /// decoding.
    DropOldest,
}

impl BackpressurePolicy {
    /// Returns whether the policy can safely be used for lossless QEC.
    pub const fn is_lossless(self) -> bool {
        matches!(self, Self::Reject)
    }
}

// ============================================================================
// Stream mode
// ============================================================================

/// Determines how the stream retains syndrome history.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StreamMode {
    /// Retain only the previous syndrome needed to calculate the next
    /// detection-event set.
    Minimal,

    /// Retain a bounded sliding window of recent rounds.
    Windowed {
        /// Number of syndrome rounds retained.
        rounds: usize,
    },
}

impl StreamMode {
    /// Returns the configured number of retained rounds.
    pub const fn rounds(self) -> usize {
        match self {
            Self::Minimal => 2,
            Self::Windowed { rounds } => {
                if rounds < 2 {
                    2
                } else {
                    rounds
                }
            }
        }
    }
}

// ============================================================================
// Stream status
// ============================================================================

/// Current lifecycle state of a syndrome stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StreamStatus {
    /// Stream accepts new syndrome rounds.
    Open,

    /// Stream has received a graceful end-of-input request.
    Closing,

    /// All buffered data has been drained and the stream is complete.
    Closed,

    /// Stream was cancelled.
    Cancelled,

    /// Stream encountered a terminal error.
    Failed,
}

impl StreamStatus {
    /// Returns whether new syndromes may be submitted.
    pub const fn accepts_input(self) -> bool {
        matches!(self, Self::Open)
    }

    /// Returns whether the stream has reached a terminal state.
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Closed | Self::Cancelled | Self::Failed
        )
    }
}

// ============================================================================
// Stream errors
// ============================================================================

/// Streaming-specific error conditions.
///
/// These are converted into the canonical [`QecError`] boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StreamingError {
    /// The stream is no longer accepting input.
    NotAcceptingInput,

    /// A syndrome round was submitted out of order.
    OutOfOrderRound {
        expected: u64,
        received: u64,
    },

    /// A stream sequence number would overflow.
    SequenceOverflow,

    /// The bounded stream buffer is full.
    BufferFull {
        capacity: usize,
    },

    /// A window size is invalid.
    InvalidWindowSize {
        requested: usize,
    },

    /// The requested polling size is invalid.
    InvalidPollSize {
        requested: usize,
    },

    /// A stream state snapshot belongs to an incompatible version.
    UnsupportedStateVersion {
        version: u16,
    },

    /// A snapshot contains internally inconsistent state.
    InvalidState {
        message: String,
    },

    /// The stream has already been drained.
    AlreadyDrained,

    /// Lossy backpressure was requested for a lossless operation.
    LossyBackpressureNotAllowed,
}

impl fmt::Display for StreamingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotAcceptingInput => {
                write!(f, "syndrome stream is no longer accepting input")
            }

            Self::OutOfOrderRound {
                expected,
                received,
            } => {
                write!(
                    f,
                    "out-of-order syndrome round: expected {}, received {}",
                    expected, received
                )
            }

            Self::SequenceOverflow => {
                write!(f, "syndrome stream sequence number overflow")
            }

            Self::BufferFull { capacity } => {
                write!(
                    f,
                    "syndrome stream buffer is full (capacity {})",
                    capacity
                )
            }

            Self::InvalidWindowSize { requested } => {
                write!(
                    f,
                    "invalid syndrome window size {}; at least 2 rounds are required",
                    requested
                )
            }

            Self::InvalidPollSize { requested } => {
                write!(
                    f,
                    "invalid poll size {}; poll size must be greater than zero",
                    requested
                )
            }

            Self::UnsupportedStateVersion { version } => {
                write!(
                    f,
                    "unsupported syndrome stream state version {}",
                    version
                )
            }

            Self::InvalidState { message } => {
                write!(f, "invalid syndrome stream state: {}", message)
            }

            Self::AlreadyDrained => {
                write!(f, "syndrome stream has already been drained")
            }

            Self::LossyBackpressureNotAllowed => {
                write!(
                    f,
                    "lossy backpressure is not permitted for lossless QEC decoding"
                )
            }
        }
    }
}

// ============================================================================
// Metrics
// ============================================================================

/// Immutable snapshot of streaming metrics.
///
/// All counters are monotonically increasing within one stream instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct StreamMetrics {
    /// Number of syndrome rounds successfully accepted.
    pub syndromes_accepted: u64,

    /// Number of syndrome submissions rejected.
    pub syndromes_rejected: u64,

    /// Number of detection events generated.
    pub detection_events_generated: u64,

    /// Number of events delivered to consumers.
    pub detection_events_delivered: u64,

    /// Number of currently buffered syndrome rounds.
    pub buffered_syndromes: u64,

    /// Peak number of buffered syndrome rounds.
    pub peak_buffered_syndromes: u64,

    /// Number of cancellation observations.
    pub cancellation_observations: u64,

    /// Number of backpressure events.
    pub backpressure_events: u64,

    /// Number of completed windows.
    pub windows_completed: u64,

    /// Number of stream sequence values assigned.
    pub sequence_numbers_assigned: u64,

    /// Number of successful flush operations.
    pub flushes: u64,
}

impl StreamMetrics {
    fn record_buffer_size(&mut self, size: usize) {
        self.buffered_syndromes = size as u64;

        let size = size as u64;

        if size > self.peak_buffered_syndromes {
            self.peak_buffered_syndromes = size;
        }
    }
}

// ============================================================================
// Stream item
// ============================================================================

/// A syndrome round accepted by the stream.
///
/// The sequence number is a stream-local ordering identifier and must not be
/// confused with the physical measurement round.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamItem {
    sequence: u64,
    syndrome: Syndrome,
}

impl StreamItem {
    /// Returns the stream sequence number.
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    /// Returns the underlying syndrome.
    pub const fn syndrome(&self) -> &Syndrome {
        &self.syndrome
    }
}

// ============================================================================
// Stream output
// ============================================================================

/// Detection events emitted by one processing step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamOutput {
    /// Sequence number of the current syndrome.
    pub sequence: u64,

    /// Measurement round that produced the events.
    pub round: MeasurementRound,

    /// Generated detection events.
    pub events: Vec<DetectionEvent>,
}

impl StreamOutput {
    /// Returns whether this output contains no detection events.
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Returns the number of generated events.
    pub fn len(&self) -> usize {
        self.events.len()
    }
}

// ============================================================================
// Snapshot state
// ============================================================================

/// Serializable-independent logical state of a syndrome stream.
///
/// This type deliberately contains no transport handles, locks, threads, or
/// external resources. It can therefore be serialized by a future
/// `checkpoint.rs` implementation without serializing execution machinery.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamState {
    /// State schema version.
    pub version: u16,

    /// Current lifecycle state.
    pub status: StreamStatus,

    /// Next stream sequence number.
    pub next_sequence: u64,

    /// Number of accepted syndrome rounds.
    pub accepted_syndromes: u64,

    /// Last accepted physical measurement round.
    pub last_round: Option<u64>,

    /// Number of buffered items.
    pub buffered_items: usize,

    /// Configured stream mode.
    pub mode: StreamMode,

    /// Configured backpressure policy.
    pub backpressure: BackpressurePolicy,

    /// Configured maximum buffer size.
    pub buffer_capacity: usize,

    /// Stream metrics.
    pub metrics: StreamMetrics,
}

// ============================================================================
// Syndrome stream
// ============================================================================

/// Production-grade bounded syndrome stream.
///
/// The stream performs three important jobs:
///
/// 1. controls resource usage;
/// 2. enforces deterministic measurement ordering;
/// 3. converts consecutive syndrome snapshots into detection events.
///
/// It does **not** perform decoding itself.
pub struct SyndromeStream<C = NeverCancel>
where
    C: CancellationToken,
{
    limits: QecLimits,
    cancellation: C,

    mode: StreamMode,
    backpressure: BackpressurePolicy,

    buffer: VecDeque<StreamItem>,

    previous: Option<StreamItem>,

    next_sequence: u64,

    status: StreamStatus,

    metrics: StreamMetrics,
}

impl SyndromeStream<NeverCancel> {
    /// Creates a stream using the default QEC limits.
    pub fn new() -> QecResult<Self> {
        Self::with_limits(QecLimits::default())
    }

    /// Creates a stream with explicit QEC limits.
    pub fn with_limits(limits: QecLimits) -> QecResult<Self> {
        Self::with_limits_and_cancellation(
            limits,
            NeverCancel,
        )
    }
}

impl<C> SyndromeStream<C>
where
    C: CancellationToken,
{
    /// Creates a stream with explicit resource limits and a cancellation
    /// token.
    pub fn with_limits_and_cancellation(
        limits: QecLimits,
        cancellation: C,
    ) -> QecResult<Self> {
        limits.validate().map_err(|error| {
            QecError::invalid_input(format!(
                "invalid QEC streaming limits: {}",
                error
            ))
        })?;

        let capacity =
            DEFAULT_MAX_BUFFERED_SYNDROMES
                .min(limits.max_stream_buffer_events);

        if capacity == 0 {
            return Err(QecError::resource_limit(
                ResourceKind::SyndromeEvents,
                1,
                0,
                "stream buffer capacity must be greater than zero",
            ));
        }

        Ok(Self {
            limits,
            cancellation,
            mode: StreamMode::Minimal,
            backpressure: BackpressurePolicy::Reject,
            buffer: VecDeque::with_capacity(capacity),
            previous: None,
            next_sequence: 0,
            status: StreamStatus::Open,
            metrics: StreamMetrics::default(),
        })
    }

    /// Returns the current stream status.
    pub const fn status(&self) -> StreamStatus {
        self.status
    }

    /// Returns the configured resource policy.
    pub const fn limits(&self) -> QecLimits {
        self.limits
    }

    /// Returns the configured stream mode.
    pub const fn mode(&self) -> StreamMode {
        self.mode
    }

    /// Returns the configured backpressure policy.
    pub const fn backpressure_policy(
        &self,
    ) -> BackpressurePolicy {
        self.backpressure
    }

    /// Returns a copy of the current metrics.
    pub const fn metrics(&self) -> StreamMetrics {
        self.metrics
    }

    /// Returns the current number of buffered syndromes.
    pub fn buffered_len(&self) -> usize {
        self.buffer.len()
    }

    /// Returns whether no syndromes are buffered.
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Returns the configured stream capacity.
    pub fn capacity(&self) -> usize {
        self.buffer.capacity()
    }

    /// Configures minimal history mode.
    ///
    /// This is the lowest-memory lossless configuration.
    pub fn set_minimal_mode(&mut self) {
        self.mode = StreamMode::Minimal;
    }

    /// Configures a bounded sliding-window mode.
    pub fn set_windowed_mode(
        &mut self,
        rounds: usize,
    ) -> QecResult<()> {
        if rounds < 2 {
            return Err(QecError::invalid_input(
                StreamingError::InvalidWindowSize {
                    requested: rounds,
                }
                .to_string(),
            ));
        }

        if rounds > self.limits.max_rounds {
            return Err(QecError::resource_limit(
                ResourceKind::MeasurementRounds,
                rounds as u128,
                self.limits.max_rounds as u128,
                "requested streaming window exceeds configured measurement-round limit",
            ));
        }

        self.mode =
            StreamMode::Windowed { rounds };

        Ok(())
    }

    /// Configures the backpressure policy.
    ///
    /// Lossy policies are rejected by default because silently dropping
    /// syndrome rounds can invalidate QEC decoding.
    pub fn set_backpressure_policy(
        &mut self,
        policy: BackpressurePolicy,
    ) -> QecResult<()> {
        if !policy.is_lossless() {
            return Err(QecError::unsupported(
                "lossy_qec_streaming",
                StreamingError::LossyBackpressureNotAllowed
                    .to_string(),
            ));
        }

        self.backpressure = policy;

        Ok(())
    }

    /// Returns a logical stream state suitable for checkpointing.
    pub fn state(&self) -> StreamState {
        StreamState {
            version: STREAM_STATE_VERSION,
            status: self.status,
            next_sequence: self.next_sequence,
            accepted_syndromes:
                self.metrics.syndromes_accepted,
            last_round: self
                .previous
                .as_ref()
                .map(|item| {
                    item.syndrome.round().value()
                }),
            buffered_items: self.buffer.len(),
            mode: self.mode,
            backpressure: self.backpressure,
            buffer_capacity: self.capacity(),
            metrics: self.metrics,
        }
    }

    /// Validates a previously produced stream state.
    ///
    /// This does not restore the actual syndrome contents; a future
    /// checkpoint implementation should store the buffered syndrome data
    /// separately and validate it before resuming execution.
    pub fn validate_state(
        state: &StreamState,
        limits: QecLimits,
    ) -> QecResult<()> {
        if state.version != STREAM_STATE_VERSION {
            return Err(QecError::unsupported(
                "stream_state_version",
                StreamingError::UnsupportedStateVersion {
                    version: state.version,
                }
                .to_string(),
            ));
        }

        limits.validate().map_err(|error| {
            QecError::invalid_input(format!(
                "invalid QEC limits for stream state: {}",
                error
            ))
        })?;

        if state.buffer_capacity == 0 {
            return Err(QecError::invalid_input(
                StreamingError::InvalidState {
                    message:
                        "stream buffer capacity is zero"
                            .to_owned(),
                }
                .to_string(),
            ));
        }

        if state.buffer_capacity
            > limits.max_stream_buffer_events
        {
            return Err(QecError::resource_limit(
                ResourceKind::SyndromeEvents,
                state.buffer_capacity as u128,
                limits.max_stream_buffer_events as u128,
                "checkpointed stream capacity exceeds current resource policy",
            ));
        }

        if state.buffered_items
            > state.buffer_capacity
        {
            return Err(QecError::invalid_input(
                StreamingError::InvalidState {
                    message:
                        "buffered item count exceeds buffer capacity"
                            .to_owned(),
                }
                .to_string(),
            ));
        }

        if state.next_sequence
            > MAX_STREAM_SEQUENCE + 1
        {
            return Err(QecError::invalid_input(
                StreamingError::InvalidState {
                    message:
                        "next sequence number exceeds representable stream range"
                            .to_owned(),
                }
                .to_string(),
            ));
        }

        if state.metrics.buffered_syndromes
            != state.buffered_items as u64
        {
            return Err(QecError::invalid_input(
                StreamingError::InvalidState {
                    message:
                        "buffered-syndrome metric does not match buffered item count"
                            .to_owned(),
                }
                .to_string(),
            ));
        }

        if let StreamMode::Windowed { rounds } =
            state.mode
        {
            if rounds < 2 {
                return Err(QecError::invalid_input(
                    StreamingError::InvalidWindowSize {
                        requested: rounds,
                    }
                    .to_string(),
                ));
            }
        }

        Ok(())
    }

    /// Checks cancellation without mutating stream state.
    pub fn check_cancellation(
        &mut self,
    ) -> QecResult<()> {
        if self.cancellation.is_cancelled() {
            self.metrics
                .cancellation_observations = self
                .metrics
                .cancellation_observations
                .saturating_add(1);

            self.status =
                StreamStatus::Cancelled;

            return Err(
                QecError::cancelled(
                    "QEC syndrome streaming was cancelled",
                ),
            );
        }

        Ok(())
    }

    /// Assigns the next stream sequence number without overflow.
    fn allocate_sequence(
        &mut self,
    ) -> QecResult<u64> {
        if self.next_sequence
            > MAX_STREAM_SEQUENCE
        {
            return Err(
                QecError::resource_limit(
                    ResourceKind::SyndromeEvents,
                    u128::from(u64::MAX),
                    u128::from(MAX_STREAM_SEQUENCE),
                    StreamingError::SequenceOverflow
                        .to_string(),
                ),
            );
        }

        let sequence =
            self.next_sequence;

        self.next_sequence =
            self.next_sequence
                .checked_add(1)
                .ok_or_else(|| {
                    QecError::invalid_input(
                        StreamingError::SequenceOverflow
                            .to_string(),
                    )
                })?;

        self.metrics
            .sequence_numbers_assigned =
            self.metrics
                .sequence_numbers_assigned
                .saturating_add(1);

        Ok(sequence)
    }

    /// Validates that a syndrome can follow the previous syndrome.
    fn validate_round_order(
        &self,
        syndrome: &Syndrome,
    ) -> QecResult<()> {
        let Some(previous) =
            self.previous.as_ref()
        else {
            return Ok(());
        };

        let expected =
            previous.syndrome.round()
                .value()
                .checked_add(1)
                .ok_or_else(|| {
                    QecError::invalid_syndrome(
                        "measurement-round arithmetic overflow",
                    )
                })?;

        let received =
            syndrome.round().value();

        if received != expected {
            return Err(
                QecError::invalid_syndrome(
                    StreamingError::OutOfOrderRound {
                        expected,
                        received,
                    }
                    .to_string(),
                ),
            );
        }

        Ok(())
    }

    /// Accepts one syndrome round into the bounded stream.
    ///
    /// This operation performs no unbounded allocation and never silently
    /// replaces an existing buffered syndrome.
    pub fn push(
        &mut self,
        syndrome: Syndrome,
    ) -> QecResult<u64> {
        self.check_cancellation()?;

        if !self.status.accepts_input() {
            self.metrics
                .syndromes_rejected = self
                .metrics
                .syndromes_rejected
                .saturating_add(1);

            return Err(
                QecError::invalid_input(
                    StreamingError::NotAcceptingInput
                        .to_string(),
                ),
            );
        }

        self.validate_round_order(
            &syndrome,
        )?;

        if self.buffer.len()
            >= self.limits.max_stream_buffer_events
        {
            self.metrics
                .syndromes_rejected = self
                .metrics
                .syndromes_rejected
                .saturating_add(1);

            self.metrics
                .backpressure_events = self
                .metrics
                .backpressure_events
                .saturating_add(1);

            return Err(
                QecError::resource_limit(
                    ResourceKind::SyndromeEvents,
                    (self.buffer.len() + 1)
                        as u128,
                    self.limits
                        .max_stream_buffer_events
                        as u128,
                    StreamingError::BufferFull {
                        capacity: self
                            .limits
                            .max_stream_buffer_events,
                    }
                    .to_string(),
                ),
            );
        }

        let sequence =
            self.allocate_sequence()?;

        let item =
            StreamItem {
                sequence,
                syndrome,
            };

        self.buffer.push_back(item);

        self.metrics
            .syndromes_accepted = self
            .metrics
            .syndromes_accepted
            .saturating_add(1);

        self.metrics
            .record_buffer_size(
                self.buffer.len(),
            );

        Ok(sequence)
    }

    /// Processes one buffered syndrome and returns its detection events.
    ///
    /// The first syndrome establishes the baseline and therefore produces no
    /// detection events. Every subsequent consecutive syndrome produces the
    /// XOR transition against its predecessor.
    pub fn process_next(
        &mut self,
    ) -> QecResult<Option<StreamOutput>> {
        self.check_cancellation()?;

        let item =
            match self.buffer.pop_front() {
                Some(item) => item,
                None => return Ok(None),
            };

        self.metrics
            .record_buffer_size(
                self.buffer.len(),
            );

        let output =
            if let Some(previous) =
                self.previous.as_ref()
            {
                let events =
                    item.syndrome
                        .detection_events_against(
                            &previous.syndrome,
                        )
                        .map_err(|error| {
                            QecError::invalid_syndrome(
                                error.to_string(),
                            )
                        })?;

                self.metrics
                    .detection_events_generated =
                    self.metrics
                        .detection_events_generated
                        .saturating_add(
                            events.len() as u64,
                        );

                Some(StreamOutput {
                    sequence:
                        item.sequence,
                    round:
                        item.syndrome.round(),
                    events,
                })
            } else {
                None
            };

        self.previous =
            Some(item);

        if let Some(output) =
            output.as_ref()
        {
            self.metrics
                .detection_events_delivered =
                self.metrics
                    .detection_events_delivered
                    .saturating_add(
                        output.events.len()
                            as u64,
                    );
        }

        if self.status
            == StreamStatus::Closing
            && self.buffer.is_empty()
        {
            self.status =
                StreamStatus::Closed;
        }

        Ok(output)
    }

    /// Processes up to `max_items` buffered syndrome rounds.
    pub fn poll(
        &mut self,
        max_items: usize,
    ) -> QecResult<Vec<StreamOutput>> {
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

        let bounded =
            max_items.min(
                DEFAULT_MAX_POLL_EVENTS,
            );

        let mut outputs =
            Vec::with_capacity(
                bounded.min(
                    self.buffer.len(),
                ),
            );

        for _ in 0..bounded {
            self.check_cancellation()?;

            match self.process_next()? {
                Some(output) => {
                    outputs.push(output)
                }
                None => break,
            }
        }

        Ok(outputs)
    }

    /// Processes every currently buffered syndrome.
    ///
    /// This is still bounded by the stream's configured buffer, so it cannot
    /// consume an unbounded history.
    pub fn drain(
        &mut self,
    ) -> QecResult<Vec<StreamOutput>> {
        let mut outputs =
            Vec::new();

        while !self.buffer.is_empty() {
            self.check_cancellation()?;

            let output =
                self.process_next()?;

            if let Some(output) =
                output
            {
                outputs.push(output);
            }
        }

        Ok(outputs)
    }

    /// Requests graceful stream shutdown.
    ///
    /// Buffered syndromes remain available and are processed normally.
    pub fn close(&mut self) -> QecResult<()> {
        self.check_cancellation()?;

        if self.status
            == StreamStatus::Closed
        {
            return Ok(());
        }

        if self.status
            == StreamStatus::Cancelled
            || self.status
                == StreamStatus::Failed
        {
            return Err(
                QecError::invalid_input(
                    StreamingError::NotAcceptingInput
                        .to_string(),
                ),
            );
        }

        self.status =
            StreamStatus::Closing;

        if self.buffer.is_empty() {
            self.status =
                StreamStatus::Closed;
        }

        Ok(())
    }

    /// Flushes all currently buffered syndromes.
    ///
    /// This is equivalent to `drain()` but records an explicit flush metric.
    pub fn flush(
        &mut self,
    ) -> QecResult<Vec<StreamOutput>> {
        self.check_cancellation()?;

        let outputs =
            self.drain()?;

        self.metrics.flushes =
            self.metrics.flushes
                .saturating_add(1);

        if self.status
            == StreamStatus::Closing
            && self.buffer.is_empty()
        {
            self.status =
                StreamStatus::Closed;
        }

        Ok(outputs)
    }

    /// Cancels the stream immediately.
    ///
    /// Buffered data is deliberately not processed after cancellation.
    pub fn cancel(&mut self) {
        self.status =
            StreamStatus::Cancelled;
    }

    /// Marks the stream as failed.
    ///
    /// This is intended for higher-level components that encounter a terminal
    /// transport, validation, or decoder error.
    pub fn fail(&mut self) {
        self.status =
            StreamStatus::Failed;
    }

    /// Returns the previous accepted syndrome.
    ///
    /// This is useful to a checkpointing or diagnostic subsystem but does not
    /// expose mutable internal state.
    pub fn previous_syndrome(
        &self,
    ) -> Option<&Syndrome> {
        self.previous
            .as_ref()
            .map(StreamItem::syndrome)
    }

    /// Returns the next expected physical measurement round.
    pub fn next_expected_round(
        &self,
    ) -> Option<QecResult<u64>> {
        let previous =
            self.previous.as_ref()?;

        Some(
            previous
                .syndrome
                .round()
                .value()
                .checked_add(1)
                .ok_or_else(|| {
                    QecError::invalid_syndrome(
                        "next measurement round would overflow",
                    )
                }),
        )
    }

    /// Returns the next stream-local sequence number.
    pub const fn next_sequence(
        &self,
    ) -> u64 {
        self.next_sequence
    }

    /// Returns whether the stream can accept another syndrome without
    /// immediately exceeding its configured buffer.
    pub fn has_capacity(&self) -> bool {
        self.buffer.len()
            < self.limits.max_stream_buffer_events
    }

    /// Returns the number of additional syndrome rounds that can currently
    /// be buffered.
    pub fn available_capacity(
        &self,
    ) -> usize {
        self.limits
            .max_stream_buffer_events
            .saturating_sub(
                self.buffer.len(),
            )
    }

    /// Returns a deterministic iterator over currently buffered sequence
    /// numbers.
    pub fn buffered_sequences(
        &self,
    ) -> impl Iterator<Item = u64> + '_ {
        self.buffer
            .iter()
            .map(StreamItem::sequence)
    }
}

// ============================================================================
// Batch ingestion
// ============================================================================

/// Production helper for ingesting a bounded iterator of syndromes.
///
/// The iterator itself remains owned by the caller. This function does not
/// collect the entire source into memory.
pub fn push_iter<I, C>(
    stream: &mut SyndromeStream<C>,
    input: I,
) -> QecResult<usize>
where
    I: IntoIterator<Item = Syndrome>,
    C: CancellationToken,
{
    let mut accepted = 0usize;

    for syndrome in input {
        stream.check_cancellation()?;

        stream.push(syndrome)?;

        accepted = accepted
            .checked_add(1)
            .ok_or_else(|| {
                QecError::numerical_failure(
                    super::errors::NumericalOperation::IntegerConversion,
                    "stream ingestion counter overflow",
                )
            })?;
    }

    Ok(accepted)
}

// ============================================================================
// Lossless producer/consumer interface
// ============================================================================

/// Result of attempting to submit a syndrome while respecting backpressure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PushStatus {
    /// Syndrome was accepted.
    Accepted {
        sequence: u64,
    },

    /// Caller should wait for the consumer to drain the stream.
    Backpressured {
        capacity: usize,
    },

    /// Stream is no longer accepting data.
    Closed,
}

/// Attempts a lossless push without treating a full buffer as an exceptional
/// programming failure.
///
/// This is useful for future scheduler/transport integration.
pub fn try_push<C>(
    stream: &mut SyndromeStream<C>,
    syndrome: Syndrome,
) -> QecResult<PushStatus>
where
    C: CancellationToken,
{
    stream.check_cancellation()?;

    if !stream.status.accepts_input() {
        return Ok(PushStatus::Closed);
    }

    if !stream.has_capacity() {
        stream.metrics.backpressure_events =
            stream.metrics.backpressure_events
                .saturating_add(1);

        return Ok(
            PushStatus::Backpressured {
                capacity: stream.capacity(),
            },
        );
    }

    let sequence =
        stream.push(syndrome)?;

    Ok(PushStatus::Accepted {
        sequence,
    })
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::syndrome::{
        MeasurementConfidence,
        MeasurementRound,
        MeasurementTimestamp,
        StabilizerId,
        SyndromeMeasurement,
    };

    fn syndrome(
        round: u64,
        value: bool,
    ) -> Syndrome {
        let mut syndrome =
            Syndrome::new(
                MeasurementRound::new(round)
                    .expect("valid test round"),
                MeasurementTimestamp::new(round)
                    .expect("valid test timestamp"),
            );

        syndrome
            .insert(
                SyndromeMeasurement::new(
                    StabilizerId::new(0),
                    value,
                    MeasurementConfidence::FULL,
                ),
            )
            .expect("unique test stabilizer");

        syndrome
    }

    fn test_limits() -> QecLimits {
        let mut limits =
            QecLimits::default();

        limits.max_stream_buffer_events = 8;
        limits.max_rounds = 128;

        limits
    }

    #[test]
    fn first_syndrome_establishes_baseline() {
        let mut stream =
            SyndromeStream::with_limits(
                test_limits(),
            )
            .expect("valid limits");

        stream
            .push(syndrome(0, false))
            .expect("push succeeds");

        let output =
            stream
                .process_next()
                .expect("processing succeeds");

        assert!(output.is_none());
    }

    #[test]
    fn consecutive_rounds_generate_detection_event() {
        let mut stream =
            SyndromeStream::with_limits(
                test_limits(),
            )
            .expect("valid limits");

        stream
            .push(syndrome(0, false))
            .expect("push succeeds");

        stream
            .push(syndrome(1, true))
            .expect("push succeeds");

        assert!(
            stream
                .process_next()
                .expect("processing succeeds")
                .is_none()
        );

        let output =
            stream
                .process_next()
                .expect("processing succeeds")
                .expect("second round produces output");

        assert_eq!(output.events.len(), 1);
        assert_eq!(
            stream.metrics().detection_events_generated,
            1
        );
    }

    #[test]
    fn unchanged_syndrome_generates_no_event() {
        let mut stream =
            SyndromeStream::with_limits(
                test_limits(),
            )
            .expect("valid limits");

        stream
            .push(syndrome(0, false))
            .expect("push succeeds");

        stream
            .push(syndrome(1, false))
            .expect("push succeeds");

        stream
            .process_next()
            .expect("processing succeeds");

        let output =
            stream
                .process_next()
                .expect("processing succeeds")
                .expect("second output exists");

        assert!(output.events.is_empty());
    }

    #[test]
    fn out_of_order_round_is_rejected() {
        let mut stream =
            SyndromeStream::with_limits(
                test_limits(),
            )
            .expect("valid limits");

        stream
            .push(syndrome(10, false))
            .expect("first push succeeds");

        let result =
            stream.push(syndrome(12, true));

        assert!(result.is_err());
    }

    #[test]
    fn buffer_is_bounded() {
        let mut limits =
            test_limits();

        limits.max_stream_buffer_events = 2;

        let mut stream =
            SyndromeStream::with_limits(
                limits,
            )
            .expect("valid limits");

        stream
            .push(syndrome(0, false))
            .expect("first push succeeds");

        stream
            .push(syndrome(1, false))
            .expect("second push succeeds");

        let result =
            stream.push(syndrome(2, false));

        assert!(result.is_err());
        assert_eq!(
            stream.buffered_len(),
            2
        );
    }

    #[test]
    fn backpressure_is_lossless() {
        let mut limits =
            test_limits();

        limits.max_stream_buffer_events = 1;

        let mut stream =
            SyndromeStream::with_limits(
                limits,
            )
            .expect("valid limits");

        stream
            .push(syndrome(0, false))
            .expect("push succeeds");

        let result =
            try_push(
                &mut stream,
                syndrome(1, true),
            )
            .expect("backpressure is not an error");

        assert_eq!(
            result,
            PushStatus::Backpressured {
                capacity: 1,
            }
        );

        assert_eq!(
            stream.buffered_len(),
            1
        );
    }

    #[test]
    fn lossy_backpressure_is_rejected() {
        let mut stream =
            SyndromeStream::with_limits(
                test_limits(),
            )
            .expect("valid limits");

        let result =
            stream.set_backpressure_policy(
                BackpressurePolicy::DropOldest,
            );

        assert!(result.is_err());
    }

    #[test]
    fn graceful_close_drains_buffer() {
        let mut stream =
            SyndromeStream::with_limits(
                test_limits(),
            )
            .expect("valid limits");

        stream
            .push(syndrome(0, false))
            .expect("push succeeds");

        stream.close()
            .expect("close succeeds");

        assert_eq!(
            stream.status(),
            StreamStatus::Closing
        );

        stream
            .flush()
            .expect("flush succeeds");

        assert_eq!(
            stream.status(),
            StreamStatus::Closed
        );
    }

    #[test]
    fn cancelled_stream_rejects_processing() {
        let mut stream =
            SyndromeStream::with_limits(
                test_limits(),
            )
            .expect("valid limits");

        stream.cancel();

        let result =
            stream.push(syndrome(0, false));

        assert!(result.is_err());
        assert_eq!(
            stream.status(),
            StreamStatus::Cancelled
        );
    }

    #[test]
    fn sequence_numbers_are_monotonic() {
        let mut stream =
            SyndromeStream::with_limits(
                test_limits(),
            )
            .expect("valid limits");

        let a =
            stream
                .push(syndrome(0, false))
                .expect("push succeeds");

        let b =
            stream
                .push(syndrome(1, false))
                .expect("push succeeds");

        assert_eq!(a, 0);
        assert_eq!(b, 1);
    }

    #[test]
    fn metrics_track_buffer_peak() {
        let mut stream =
            SyndromeStream::with_limits(
                test_limits(),
            )
            .expect("valid limits");

        stream
            .push(syndrome(0, false))
            .expect("push succeeds");

        stream
            .push(syndrome(1, false))
            .expect("push succeeds");

        assert_eq!(
            stream
                .metrics()
                .peak_buffered_syndromes,
            2
        );

        stream
            .process_next()
            .expect("processing succeeds");

        assert_eq!(
            stream
                .metrics()
                .buffered_syndromes,
            1
        );
    }

    #[test]
    fn polling_is_bounded() {
        let mut stream =
            SyndromeStream::with_limits(
                test_limits(),
            )
            .expect("valid limits");

        for round in 0..5 {
            stream
                .push(syndrome(round, false))
                .expect("push succeeds");
        }

        let outputs =
            stream
                .poll(2)
                .expect("poll succeeds");

        assert_eq!(outputs.len(), 2);
        assert_eq!(
            stream.buffered_len(),
            3
        );
    }

    #[test]
    fn windowed_mode_requires_two_rounds() {
        let mut stream =
            SyndromeStream::with_limits(
                test_limits(),
            )
            .expect("valid limits");

        assert!(
            stream
                .set_windowed_mode(1)
                .is_err()
        );

        stream
            .set_windowed_mode(4)
            .expect("valid window");

        assert_eq!(
            stream.mode(),
            StreamMode::Windowed {
                rounds: 4
            }
        );
    }

    #[test]
    fn state_is_self_consistent() {
        let mut stream =
            SyndromeStream::with_limits(
                test_limits(),
            )
            .expect("valid limits");

        stream
            .push(syndrome(0, false))
            .expect("push succeeds");

        let state =
            stream.state();

        SyndromeStream::<NeverCancel>::validate_state(
            &state,
            test_limits(),
        )
        .expect("state should validate");
    }

    #[test]
    fn empty_stream_reports_no_capacity_error() {
        let mut stream =
            SyndromeStream::with_limits(
                test_limits(),
            )
            .expect("valid limits");

        assert!(stream.is_empty());
        assert!(
            stream.has_capacity()
        );
    }

    #[test]
    fn iterator_ingestion_is_streaming() {
        let mut stream =
            SyndromeStream::with_limits(
                test_limits(),
            )
            .expect("valid limits");

        let input = (0..3)
            .map(|round| {
                syndrome(round, false)
            });

        let accepted =
            push_iter(
                &mut stream,
                input,
            )
            .expect("ingestion succeeds");

        assert_eq!(accepted, 3);
        assert_eq!(
            stream.buffered_len(),
            3
        );
    }
}